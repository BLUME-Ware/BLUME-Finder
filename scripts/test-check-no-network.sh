#!/usr/bin/env bash
# Runs check-no-network.sh against copies of the repository with one injected violation each.
set -euo pipefail

repo=$(cd "$(dirname "$0")/.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
export CARGO_TARGET_DIR="$work/target"

failures=0

fresh_copy() {
  local dir="$work/case"
  rm -rf "$dir"
  mkdir -p "$dir"
  # --cached also lists files deleted from disk but still staged; those are skipped.
  (
    cd "$repo"
    git ls-files --cached --others --exclude-standard \
      | while IFS= read -r file; do if [[ -e "$file" ]]; then echo "$file"; fi; done \
      | tar -cf - -T -
  ) | tar -xf - -C "$dir"
  echo "$dir"
}

# Usage: expect <name> <expected output, empty for success> <shell snippet run in the copy>
expect() {
  local name=$1 expected=$2 inject=$3 dir output status=0
  dir=$(fresh_copy)
  (cd "$dir" && eval "$inject")
  output=$("$dir/scripts/check-no-network.sh" 2>&1) || status=$?

  if [[ -z "$expected" && $status -eq 0 ]] || [[ -n "$expected" && $status -ne 0 && "$output" == *"$expected"* ]]; then
    echo "ok   - ${name}"
  else
    echo "FAIL - ${name}"
    echo "$output"
    failures=$((failures + 1))
  fi
}

expect "clean repository passes" "" ":"

fake_crate() {
  mkdir -p "fake/$1/src"
  touch "fake/$1/src/lib.rs"
  printf '[package]\nname = "%s"\nversion = "0.1.0"\nedition = "2021"\n' "$1" > "fake/$1/Cargo.toml"
}

# The app depends on the engine by path, so both lockfiles have to follow.
add_engine_dependency() {
  fake_crate "$1"
  printf '%s = { path = "../fake/%s" }\n' "$1" "$1" >> core/Cargo.toml
  cargo update --offline --quiet --package blume-finder-core
  if [[ -f app/src-tauri/Cargo.toml ]]; then
    cargo update --offline --quiet --package blume-finder-core --manifest-path app/src-tauri/Cargo.toml
  fi
}

expect "networking crate in the engine is rejected" "Cargo.toml: networking crate in dependency tree: reqwest" '
  add_engine_dependency reqwest'

expect "async runtime in the engine is rejected" "the engine must not depend on tokio" '
  add_engine_dependency tokio'

expect "networking crate in the app is rejected" "app/src-tauri/Cargo.toml: networking crate in dependency tree: hyper" '
  fake_crate hyper
  mkdir -p app/src-tauri/src && echo "fn main() {}" > app/src-tauri/src/main.rs
  printf "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nhyper = { path = \"../../fake/hyper\" }\n" > app/src-tauri/Cargo.toml
  cargo generate-lockfile --offline --quiet --manifest-path app/src-tauri/Cargo.toml'

expect "socket API in Rust is rejected" "socket API used in Rust sources" '
  echo "use std::net::TcpStream;" >> core/src/lib.rs'

expect "network API in front end is rejected" "network API used in front-end sources" '
  mkdir -p app/src && echo "fetch(\"/x\")" > app/src/main.ts'

expect "remote URL in front end is rejected" "remote URL in front-end sources" '
  mkdir -p app/src && echo "const u = \"https://example.com\"" > app/src/main.ts'

expect "SVG namespace is allowed" "" '
  mkdir -p app/src && echo "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>" > app/src/Logo.svelte'

expect "networking package is rejected" "networking package in app/package.json: axios" '
  mkdir -p app && echo "{\"dependencies\": {\"axios\": \"1.0.0\"}}" > app/package.json'

expect "missing CSP is rejected" "no CSP set" '
  mkdir -p app/src-tauri && echo "{}" > app/src-tauri/tauri.conf.json'

expect "remote origin in CSP is rejected" "CSP allows a remote origin" '
  mkdir -p app/src-tauri
  echo "{\"csp\": \"default-src '"'"'self'"'"'; connect-src https://example.com\"}" > app/src-tauri/tauri.conf.json'

expect "Tauri IPC origin in CSP is allowed" "" '
  mkdir -p app/src-tauri
  echo "{\"csp\": \"default-src '"'"'self'"'"'; connect-src ipc: http://ipc.localhost\"}" > app/src-tauri/tauri.conf.json'

expect "network capability is rejected" "network permission granted" '
  mkdir -p app/src-tauri/capabilities
  echo "{\"permissions\": [\"http:default\"]}" > app/src-tauri/capabilities/default.json'

if ((failures > 0)); then
  echo "test-check-no-network: ${failures} case(s) failed" >&2
  exit 1
fi
echo "test-check-no-network: OK"
