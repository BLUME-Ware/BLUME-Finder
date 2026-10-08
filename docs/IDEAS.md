# Ideas after V1

Requests and ideas that are out of scope for the first public release, one line each, set aside
until after 3 November 2026. Nothing here is a commitment.

## Product

- Search by meaning with a local, multilingual embedding model.
- OCR for scans and images.
- Tuning of the name versus content weight in ranking.
- Silencing the font warnings printed by `pdf-extract`.
- UI languages beyond English and French.
- Windows and Linux releases. The code is kept portable and tested on both in CI; only packaging and release are deferred.
- macOS signing and notarization, if a free route opens (for example Apple's fee waiver for registered non-profit organizations).

## Engineering practices

- Strict lints: clippy pedantic, no `unwrap`, `expect`, `panic` or `print` outside tests, unsafe code forbidden. On 7 October 2026 they raised 55 mechanical findings on the engine.
- Text extraction in a separate process with time and memory limits, so a malformed file cannot stop the application.
- Fuzzing of the file readers (`cargo-fuzz`, then OSS-Fuzz).
- Reference corpus of small documents with their expected text, tested on every change.
- Accessibility pass: full keyboard navigation, VoiceOver labels, contrast in both themes.
- Branch protection on `main`: merge only with green CI.
- Release checklist in `docs/`.
- SHA-256 checksums and GitHub build provenance attestations on releases.
- Software bill of materials (SBOM) published with each release.
- Performance budgets: startup time, indexing throughput, memory use.
- OpenSSF Scorecard and OpenSSF Best Practices badge.
- Deprecation policy: no breaking change without one release of notice.
- Public `ARCHITECTURE.md`.
- Short blameless notes after serious bugs: what happened, why, which test now prevents it.
