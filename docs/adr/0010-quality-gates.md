# 0010. Single set of quality gates

Date: 2026-10-07
Status: Accepted

## Context

The project has one developer and much of its code is written with an AI assistant. Mistakes
have to be caught by tools, the same way every time, before they reach `main`.

## Decision

`scripts/check-all.sh` is the single entry point for every check:

- formatting (rustfmt) and clippy with its default lints, warnings treated as errors;
- tests;
- dependency audit with `cargo-deny`: known vulnerabilities, licenses, sources;
- ShellCheck on the scripts;
- the no-network check and its own tests.

The same script runs in the pre-commit hook and in CI on macOS; CI also builds and tests the
engine on Linux and Windows. The Rust toolchain and the CI actions are pinned to exact versions.
A change is done only when this script passes and its output has been shown.

## Consequences

- Commits take a few seconds longer, and more once the application is checked as well.
- A lint can only be silenced locally, with a stated reason.
- Stricter lints are deferred until after the first release (`docs/IDEAS.md`).
