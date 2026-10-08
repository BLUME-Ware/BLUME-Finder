# 0003. Engine separate from the application shell

Date: 2026-10-07
Status: Accepted

## Context

The search logic must be testable without a graphical interface, and the interface technology
may change over time without touching that logic.

## Decision

- `core` is a Rust library (`blume_finder_core`) with a command-line tool (`blume`). It holds all
  the logic: crawling, text extraction, stemming, indexing and search. It does not depend on Tauri.
- `app/src-tauri` is a thin Tauri v2 shell that maps interface commands to `core` calls.
- The shell is a separate Cargo project, excluded from the workspace, so checking the engine does
  not require building the graphical stack.

## Consequences

- Engine logic is unit-tested in `core`; the shell stays small enough to review by reading.
- The command-line tool allows testing the engine on real folders without the interface.
- There are two `Cargo.lock` files to keep up to date.
