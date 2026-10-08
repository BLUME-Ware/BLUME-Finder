# 0011. Exceptions to the dependency audit

Date: 2026-10-08
Status: Accepted

## Context

`cargo-deny` stops the checks on any known advisory. Some advisories only report that a crate is
unmaintained, with no known vulnerability and no fixed version, for crates the project cannot
replace without giving up something more important.

## Decision

- An advisory is ignored in `deny.toml` only with a written reason.
- RUSTSEC-2026-0192 (`ttf-parser`, unmaintained) is ignored. `lopdf` 0.42 requires it only to
  embed fonts when creating a PDF, which the engine never does, and `lopdf` 0.42 is required for
  the RUSTSEC-2026-0187 fix. This exception is re-examined at each new version of `pdf-extract`:
  is `ttf-parser` still in the dependency tree, and can reading a PDF now reach it?
- RUSTSEC-2024-0370 (`proc-macro-error`, unmaintained) is ignored. It is a build-time macro crate
  pulled in only by the Linux GTK stack of Tauri.

## Consequences

- An update of `pdf-extract`, including one proposed by Dependabot, is not merged before the
  `ttf-parser` exception has been re-examined.
- Any new advisory stops the checks until a decision is taken.
