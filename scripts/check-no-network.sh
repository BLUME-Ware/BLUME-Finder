#!/usr/bin/env bash
# Fails if anything in the repository could give the application network access.
set -euo pipefail

cd "$(dirname "$0")/.."

failures=0
fail() {
  echo "FAIL: $1" >&2
  failures=$((failures + 1))
}

# Desktop platforms only: on Android and iOS, Tauri itself pulls in an HTTP client,
# and the project does not ship there.
targets=(
  --target aarch64-apple-darwin
  --target x86_64-apple-darwin
  --target x86_64-pc-windows-msvc
  --target x86_64-unknown-linux-gnu
)

# Crates whose only purpose is network I/O. Matched against the full resolved tree,
# so a transitive pull-in fails the check as well.
banned_crates=(
  reqwest hyper hyper-util h2 h3 ureq attohttpc isahc surf curl curl-sys
  quinn tungstenite tokio-tungstenite async-tungstenite websocket tonic
  native-tls openssl openssl-sys rustls tokio-rustls hickory-resolver trust-dns-resolver
  tauri-plugin-http tauri-plugin-websocket tauri-plugin-updater tauri-plugin-upload
)
# The engine is held to a stricter rule: no async runtime, no sockets, not even HTTP types.
engine_banned_crates=(tokio async-std mio socket2 http url)
banned_packages=(
  axios ky got node-fetch undici ws socket.io-client
  @tauri-apps/plugin-http @tauri-apps/plugin-websocket
  @tauri-apps/plugin-updater @tauri-apps/plugin-upload
)

# Usage: dependency_tree <manifest> <edges> [cargo tree options...]
dependency_tree() {
  local manifest=$1 edges=$2
  shift 2
  cargo tree --manifest-path "$manifest" --locked "${targets[@]}" \
    --edges "$edges" --prefix none --format '{p}' "$@"
}

check_rust_dependencies() {
  local manifest tree features crate
  for manifest in Cargo.toml app/src-tauri/Cargo.toml; do
    [[ -f "$manifest" ]] || continue
    tree=$(dependency_tree "$manifest" normal,build --workspace)
    for crate in "${banned_crates[@]}"; do
      if grep -qE "^${crate} v" <<<"$tree"; then
        fail "${manifest}: networking crate in dependency tree: ${crate}"
      fi
    done

    features=$(dependency_tree "$manifest" features --workspace)
    if grep -qE '^(tokio|mio) feature "net"' <<<"$features"; then
      fail "${manifest}: tokio or mio is built with the \"net\" feature"
    fi
  done

  [[ -f core/Cargo.toml ]] || return 0
  tree=$(dependency_tree Cargo.toml normal,build --package blume-finder-core)
  for crate in "${engine_banned_crates[@]}"; do
    if grep -qE "^${crate} v" <<<"$tree"; then
      fail "the engine must not depend on ${crate}"
    fi
  done
}

check_rust_sources() {
  local dir
  for dir in core app/src-tauri/src; do
    [[ -d "$dir" ]] || continue
    if grep -rnE --include='*.rs' 'std::net|TcpStream|TcpListener|UdpSocket' "$dir"; then
      fail "socket API used in Rust sources under ${dir}"
    fi
  done
}

check_frontend_sources() {
  [[ -d app/src ]] || return 0
  local urls
  if grep -rnE '\b(fetch|XMLHttpRequest|WebSocket|EventSource|sendBeacon)\b' app/src; then
    fail "network API used in front-end sources"
  fi
  # The SVG namespace is an identifier, not a request.
  urls=$(grep -rnEs 'https?://' app/src app/index.html | grep -v 'http://www.w3.org/' || true)
  if [[ -n "$urls" ]]; then
    echo "$urls"
    fail "remote URL in front-end sources"
  fi
}

check_frontend_packages() {
  [[ -f app/package.json ]] || return 0
  local package
  for package in "${banned_packages[@]}"; do
    if grep -qF "\"${package}\"" app/package.json; then
      fail "networking package in app/package.json: ${package}"
    fi
  done
}

check_tauri_config() {
  local conf=app/src-tauri/tauri.conf.json csp
  if [[ -f "$conf" ]]; then
    csp=$(grep -oE '"csp"[[:space:]]*:[[:space:]]*"[^"]*"' "$conf" || true)
    if [[ -z "$csp" ]]; then
      fail "no CSP set in ${conf}"
    elif ! grep -q "default-src 'self'" <<<"$csp"; then
      fail "CSP does not restrict default-src to 'self'"
    elif grep -oE 'https?://[^ ;"]+' <<<"$csp" | grep -qvE '^https?://(ipc|asset)\.localhost$'; then
      fail "CSP allows a remote origin"
    fi
  fi

  if [[ -d app/src-tauri/capabilities ]] \
    && grep -rnE '"(http|websocket|updater|upload):' app/src-tauri/capabilities; then
    fail "network permission granted in Tauri capabilities"
  fi
}

check_rust_dependencies
check_rust_sources
check_frontend_sources
check_frontend_packages
check_tauri_config

if ((failures > 0)); then
  echo "check-no-network: ${failures} problem(s) found" >&2
  exit 1
fi
echo "check-no-network: OK"
