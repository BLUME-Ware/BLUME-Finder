# Development and release process

How a change reaches users, and how a version is prepared, published and maintained. Versioning
follows ADR 0009, the checks ADR 0010, the dependency exceptions ADR 0011.

## Every change

1. A need, written down: a GitHub issue, or a line in `docs/IDEAS.md` when it is outside the
   current version.
2. A decision record in `docs/adr/` when the change decides something lasting (a permission, a
   dependency, a behavior users rely on).
3. One branch per change: `feat/…`, `fix/…`, `docs/…`, `chore/…`.
4. Tests: an automatic test for any engine logic, written first for a bug fix; a measured check
   and a real run of the app for the interface.
5. `scripts/check-all.sh` passes, as in the pre-commit hook and CI.
6. `README.md` describes any visible change, `CHANGELOG.md` has an entry under "Unreleased",
   `docs/ROADMAP.md` is updated when the plan or the status changes.
7. A pull request, merged only once all four CI jobs have passed. `main` stays green.

## Versions

- Semantic Versioning: `0.1.1` fixes defects, `0.2.0` adds features, `1.0.0` comes once the
  software has run well on macOS and Windows.
- `main` carries the next feature version. A published version lives on its own branch,
  `release/0.1` for the 0.1 series.

## Preparing a version

1. **Development.** Features and adjustments are merged into `main` until the date set for the
   candidate.
2. **Release candidate.** On that date, a branch `release/X.Y` is created from `main`. From then
   on it only receives fixes for serious defects, each through a pull request. `main` moves on to
   the next version.
3. **Candidate build.** The disk image is built from the release branch, uploaded to a draft
   GitHub Release, and its SHA-256 written in the release notes.
4. **Candidate test.** On a downloaded copy, from a clean state: installation into Applications,
   first launch, macOS permissions, indexing of real folders, searches, relaunch. The macOS version
   and processor tested are recorded in the notes, as well as what was not tested.
5. **Release notes**, in English and French: features, installation, SHA-256, tested
   configuration, known limitations.
6. **Publication.** The `CHANGELOG.md` heading becomes `[X.Y.Z] - date`, then the draft is
   published so the tag `vX.Y.Z` lands on the release branch. A rebuilt disk image means a new
   SHA-256 and a new test.

## After a release

- Feedback arrives through GitHub issues and is sorted by severity: a security issue or data at
  risk first (see `SECURITY.md`), then a defect that blocks use, then the rest.
- A serious defect is fixed on `release/X.Y`, released as a patch version, then the fix is carried
  over to `main`.
- Dependency updates proposed by Dependabot are reviewed on `main`, outside release candidates. An
  update touching an exception of `deny.toml` re-examines that exception first (ADR 0011).
