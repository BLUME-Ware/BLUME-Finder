#!/usr/bin/env bash
# Every check a change must pass. Run locally, by the pre-commit hook and by CI.
set -euo pipefail

cd "$(dirname "$0")/.."

for tool in cargo cargo-deny shellcheck; do
  if ! command -v "$tool" >/dev/null; then
    echo "missing tool: ${tool} (see README.md, Development)" >&2
    exit 1
  fi
done

step() {
  echo "== $*"
  "$@"
}

step cargo fmt --all --check
step cargo clippy --workspace --all-targets --locked -- -D warnings
step cargo test --workspace --locked
step cargo deny --locked check

# The app is a separate Cargo project, outside the workspace.
app=app/src-tauri/Cargo.toml
step cargo fmt --manifest-path "$app" --check
step cargo clippy --manifest-path "$app" --all-targets --locked -- -D warnings
step cargo deny --manifest-path "$app" --locked check

step shellcheck scripts/*.sh .githooks/*
step scripts/check-no-network.sh
step scripts/test-check-no-network.sh

echo "All checks passed."
