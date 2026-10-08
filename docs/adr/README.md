# Architecture decision records

Each significant decision is recorded in a short numbered file: its context, the decision itself,
and its consequences. A record is never rewritten once accepted; a new record supersedes it.

| No. | Decision | Status |
|-----|----------|--------|
| [0001](0001-no-network-access.md) | No network access, verified automatically | Accepted |
| [0002](0002-read-only-file-access.md) | Read-only access to user files | Accepted |
| [0003](0003-engine-and-shell-separation.md) | Engine separate from the application shell | Accepted |
| [0004](0004-local-sqlite-fts5-index.md) | Local SQLite FTS5 index | Accepted |
| [0005](0005-permanent-application-identifier.md) | Permanent application identifier | Accepted |
| [0006](0006-svelte-typescript-vite-front-end.md) | Svelte, TypeScript and Vite for the interface | Accepted |
| [0007](0007-english-source-localized-interface.md) | English source, localized interface | Accepted |
| [0008](0008-free-distribution-and-portability.md) | Free distribution, portable code | Accepted |
| [0009](0009-versioning.md) | Semantic versioning, first release 0.1.0 | Accepted |
| [0010](0010-quality-gates.md) | Single set of quality gates | Accepted |
| [0011](0011-dependency-audit-exceptions.md) | Exceptions to the dependency audit | Accepted |

New records follow the same structure:

```
# NNNN. Title

Date: YYYY-MM-DD
Status: Proposed | Accepted | Superseded by NNNN

## Context
## Decision
## Consequences
```
