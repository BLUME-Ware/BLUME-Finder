# Ideas after V1

Requests and ideas that are out of scope for the first public release, one line each, set aside
until after 3 November 2026. Nothing here is a commitment.

## Product

- OCR for scans and images.
- Tuning of the name versus content weight in ranking.
- Silencing the font warnings printed by `pdf-extract`.
- `blume stats` undercounts the text size when a document contains a NUL character (SQLite `LENGTH` stops there), and counts files over the size limit as unreadable while indexing reports them as findable by name only.
- Memory use of `pdf-extract` on large PDFs: 230 MB peak for a 5 MB, 186-page report; a PDF near the 150 MB limit is untested.
- UI languages beyond English and French.
- The other sources of Spotlight: mail, messages, contacts, calendar, web. Not planned (ADR 0012).
- Honoring the Windows hidden attribute; hidden files and folders are recognized by their leading dot only.
- Linux release. The code is kept portable and tested in CI; only packaging and release are deferred. The Windows release is version 0.3.0 (ADR 0013).
- macOS signing and notarization, if a free route opens (for example Apple's fee waiver for registered non-profit organizations).
- Search field fixed at the top while results scroll beneath it, with a translucent, blurred backdrop (CSS only).
- Watching the indexed folders while the app is open, so a new file is found at once; part of background indexing in 0.2.0 (ADR 0013).
- Mail attachments saved by the user are found; attachments left inside the mail application are not, since its data is never read. A clear hint about it, or a guided way to save them.
- Upgrade `zip` to 9 and remove the RUSTSEC-2026-0367 to 0369 exceptions (ADR 0011).
- An easier update path without any network code in the app: a BLUME-Ware Homebrew tap (`brew upgrade`), or a link that opens the releases page in the browser. An in-app updater is excluded by rule 2.

## Engineering practices

- Strict lints: clippy pedantic, no `unwrap`, `expect`, `panic` or `print` outside tests, unsafe code forbidden. On 7 October 2026 they raised 55 mechanical findings on the engine.
- Text extraction in a separate process with time and memory limits, so a malformed file cannot stop the application.
- Fuzzing of the file readers (`cargo-fuzz`, then OSS-Fuzz).
- Reference corpus of small documents with their expected text, tested on every change.
- Accessibility pass: full keyboard navigation, VoiceOver labels, contrast in both themes.
- Branch protection on `main`: merge only with green CI.
- SHA-256 checksums and GitHub build provenance attestations on releases.
- Software bill of materials (SBOM) published with each release.
- Performance budgets: startup time, indexing throughput, memory use.
- OpenSSF Scorecard and OpenSSF Best Practices badge.
- Deprecation policy: no breaking change without one release of notice.
- Public `ARCHITECTURE.md`.
- Short blameless notes after serious bugs: what happened, why, which test now prevents it.
