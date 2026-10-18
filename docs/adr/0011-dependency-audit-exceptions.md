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
- RUSTSEC-2026-0367, RUSTSEC-2026-0368 and RUSTSEC-2026-0369 (`zip` 2, added on 18 October 2026)
  are ignored. They concern extracting an archive to disk, where symbolic links or SUID bits can
  escape the destination. The engine never extracts: it reads the entries of Office and
  OpenDocument files into memory and writes nothing. The fix needs `zip` 9, a major upgrade that
  is deferred until after 0.1.0, when these exceptions are removed.

## Consequences

- An update of `pdf-extract`, including one proposed by Dependabot, is not merged before the
  `ttf-parser` exception has been re-examined.
- Any new advisory stops the checks until a decision is taken.
