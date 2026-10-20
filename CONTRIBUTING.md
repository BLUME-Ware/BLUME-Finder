# Contributing

Blume Finder only reads files and never accesses the network. Every contribution keeps it that way.

To propose a change, open an issue that describes the need, then make one focused change on its
own branch, with a test for any engine logic, and open a pull request. It must pass
`scripts/check-all.sh`, which CI also runs.

Changes that add a dependency, a permission or any network access are declined by default. The
reasons behind the project rules are recorded in [`docs/adr/`](docs/adr/). Security issues are
reported privately, as described in [`SECURITY.md`](SECURITY.md). Where the project goes and where
it stands is in [`docs/ROADMAP.md`](docs/ROADMAP.md).

## Project rules

1. **Read-only.** The code never moves, renames, modifies or deletes a user file. The only file
   written is the index, in the application data folder (ADR 0002).
2. **No network, verifiably.** The engine has no networking dependency and no async runtime; the
   app requests no network permission and ships no HTTP client. No telemetry, no automatic update,
   no remote AI service. `scripts/check-no-network.sh` enforces it (ADR 0001).
3. **Secret files are never indexed**: keys, `.env` files, certificates, keychains, password
   databases, credential exports, recovery codes. The list can grow, never shrink.
4. **The interface never uses `innerHTML`** for text coming from files. The content security
   policy allows no external resource, and the interface keeps the `core:default` permissions.
5. **No new dependency without a written reason** given before it is added.
6. **No secret, token or personal data** in the repository, tests or examples.
7. **No paid service or account** to build, test or ship.
8. **Portable by construction**: `std::path`, no hard-coded separators or locations,
   platform-specific code behind `cfg`. CI builds and tests on macOS, Windows and Linux (ADR 0008).

Everything is written in English: identifiers, comments, commit messages and documentation.
Every user-facing string lives in the translation catalogs (`app/src/lib/locales`), English being
the reference (ADR 0007).

## Definition of done

A change is done when `scripts/check-all.sh` passes; when it is tested, with an automatic test for
any engine logic and a real run of the app for the interface; when `README.md` describes any
visible change and `CHANGELOG.md` has an entry under "Unreleased"; and when a significant decision
has its ADR. Every advisory ignored in `deny.toml` has a written reason (ADR 0011). `main` stays
green: a pull request is merged only once all four CI jobs have passed.
