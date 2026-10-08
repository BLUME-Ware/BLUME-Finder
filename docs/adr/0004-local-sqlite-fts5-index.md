# 0004. Local SQLite FTS5 index

Date: 2026-10-07
Status: Accepted

## Context

Search must be fast on a personal computer, without a server, a background service or any
system dependency.

## Decision

- A single SQLite database, bundled with the application through `rusqlite`, stored in the
  application data directory.
- Full-text search with FTS5, using the `unicode61` tokenizer with diacritics removed. File names
  and stemmed contents are indexed; the extracted text is kept to build result passages.
- Snowball stemming for French, so that plurals and word forms match.
- A file is read again only when its size or modification time changes.

## Consequences

- The index contains the text of indexed files. It is created with owner-only permissions on
  macOS and Linux, and deleting it removes everything the application knows.
- Bundling SQLite removes a system dependency; SQLite updates arrive with `rusqlite` updates.
- The index is derived data that can always be rebuilt from the files, which keeps schema changes
  cheap.
