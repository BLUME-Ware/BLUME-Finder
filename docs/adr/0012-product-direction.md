# 0012. Product direction: a smarter, local search for the whole computer

Date: 2026-10-08
Status: Accepted

## Context

The first release finds files by their content, in folders chosen by the user. The product is
meant to go much further. The order of the steps has to be explicit, so that the first release
stays small and ships on time.

## Decision

- In the long term, Blume Finder searches every file, folder and application on the computer,
  as Spotlight does on macOS, but smarter: search by meaning and better ranking. Everything runs
  on the machine (ADR 0001).
- Version 0.1.0, on 3 November 2026, keeps its scope: finding a file by its content.
- Version 0.2.0 adds folders as results, and applications, discovered and launched on macOS and
  Windows.
- Checkpoint on Monday 26 October 2026: if every task of the plan is done and green, a decision
  is taken on whether part of 0.2.0 joins 0.1.0. Otherwise it stays in 0.2.0.
- Managing files and folders (moving, organizing, deleting) comes much later. Since it breaks the
  read-only rule, it needs a new record that supersedes ADR 0002; the other safety rules still
  apply. Until then, the application stays read-only.
- The other sources of Spotlight (mail, messages, contacts, calendar, web) are not planned.

## Consequences

- The index will hold other kinds of results than files. The index schema version, planned with
  the English translation of the engine, keeps such changes cheap.
- Searching the whole computer will require updating the index from file system events instead
  of walking every folder at launch, and asking for the operating system's privacy permissions.
- Launching applications means starting other programs. It gets its own security review in 0.2.0.
