# 0013. Search by meaning as version 0.2.0

Date: 2026-10-12
Status: Accepted

## Context

Version 0.1.0 finds a file by the words it contains. Spotlight, Windows Search and Everything
already find files by their words or their name, so word search alone does not make Blume Finder
worth installing. What sets it apart is finding a document from an idea, in French as in English
("document armée" finding everything related to military service), showing the exact passage,
and behaving the same on macOS, Windows and Linux. ADR 0012 placed search by meaning in a later,
undated step, after folders and applications. This record changes that order. The rest of
ADR 0012 still applies.

## Decision

- Version 0.1.0 keeps its scope and its date, 3 November 2026.
- Version 0.2.0 is search by meaning: a local multilingual model turns passages into vectors,
  and results combine closeness in meaning with the existing word index, so exact matches and
  the displayed passage are kept. It runs on the machine only (ADR 0001).
- The model is bundled with the application. The application never downloads it, and gains no
  network access.
- Indexing by meaning runs in the background; word search stays available meanwhile.
- 0.2.0 ships only if, on a set of real searches compared with Spotlight, it does better on
  searches by idea without losing ground on searches by exact words. The searches and the files
  stay out of the repository; only aggregated scores may be recorded.
- Version 0.3.0 is the Windows release.
- Folders as results, and applications discovered and launched, come after 0.3.0.
- Managing files and folders still comes much later, and still requires a new record
  superseding ADR 0002.

## Consequences

- The choice of model and runtime gets its own record, after measurement on the real searches.
- The model file is not stored in the git history. Its SHA-256 hash is recorded in the
  repository and checked when the application is built.
- The application grows by the size of the model, from about a hundred megabytes upward.
- The index gains a table of vectors, with a new schema version and a migration; secure deletion
  applies to it as it does to the text.
- The checkpoint of 26 October 2026 cannot move search by meaning into 0.1.0: it is too large.
