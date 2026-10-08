# 0001. No network access, verified automatically

Date: 2026-10-07
Status: Accepted

## Context

Blume Finder reads personal documents and stores their text in an index. The value of the
project is that this data never leaves the machine. A promise of privacy is only worth something
if anyone can check it in the code.

## Decision

The application has no network access at all.

- The engine (`core`) depends on no networking crate, no async runtime and not even HTTP types.
- The application shell embeds no HTTP client and requests no network permission. Its content
  security policy only allows the application's own origin and Tauri's IPC.
- No telemetry, no update check, no remote API of any kind.

`scripts/check-no-network.sh` enforces this on the dependency trees of the four desktop targets,
on the Rust and front-end sources, on the content security policy, on the Tauri capabilities and
on the npm packages. It runs in the pre-commit hook and in CI, and its own behavior is covered by
`scripts/test-check-no-network.sh`.

## Consequences

- No automatic updates: new versions are downloaded by the user from GitHub Releases.
- No crash reports or usage statistics; problems are learned from issues and email.
- Any feature that needs the network is out of scope by design.
- Mobile targets are not checked, because Tauri pulls in an HTTP client there. Shipping on mobile
  would require revisiting this decision.
