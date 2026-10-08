# 0009. Semantic versioning, first release 0.1.0

Date: 2026-10-07
Status: Accepted

## Context

Users need to know what changed between versions and how stable a version is.

## Decision

- Versions follow Semantic Versioning.
- The first public release, on 3 November 2026, is `0.1.0`: it is still lightly tested.
- `1.0.0` comes once the software has run well on macOS and Windows.
- Notable changes are recorded in `CHANGELOG.md`, following the Keep a Changelog format.

## Consequences

- Every user-visible change adds an entry under "Unreleased" in the changelog.
- While the version is `0.x`, breaking changes remain possible and are called out in the changelog.
