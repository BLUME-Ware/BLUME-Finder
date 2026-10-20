# Roadmap

What Blume Finder is for, where it goes, and where it stands. Kept current at every version; the
reasons behind each decision are in [`docs/adr/`](adr/), and the user-visible changes in
[`CHANGELOG.md`](../CHANGELOG.md).

## Purpose

Find a file by what it contains, on the user's own computer, without anything leaving it, with
code anyone can check. Blume Finder is the first BLUME Ware product built on that principle of
security and software sovereignty.

## Versions

| Version | Content | Status |
| --- | --- | --- |
| 0.1.0 | Search by words in file contents and names, macOS, unsigned | Finishing, release on 3 November 2026 |
| 0.2.0 | Search by meaning with a model running on the computer, hybrid with the word index, background indexing (ADR 0013) | Next |
| 0.3.0 | Windows release | Planned |
| Later | Folders as results, applications found and launched, then the whole computer, always local (ADR 0012). OCR between 0.2.0 and 0.3.0, to be decided | Direction |
| 1.0.0 | Once the software has run well on macOS and Windows | Goal |

Not planned: mail, messages, contacts, calendar, web. Managing files (moving, renaming, tidying)
needs a new ADR superseding ADR 0002, which keeps the app read-only.

## What later versions must guarantee

Recorded on 20 October 2026, after testing 0.1.0 on a real computer.

- **One consent, once.** On first launch, a single small Blume window explains in a few words what
  is read and asks for an explicit "I have read and accept". No series of windows, no long text.
- **Everything the user owns.** Once accepted, Blume reads all of the user's files, not only three
  folders, without exposing what must stay private: mail, messages, other applications' data,
  system files and secrets stay excluded. Where macOS requires Full Disk Access, the app guides the
  user through it in one step.
- **Search by understanding.** A natural phrase such as "electricity bill March" finds the right
  document even when its words differ, which is the purpose of 0.2.0.
- **No visible search error.** No misleading "no results", no list where the right file is lost:
  the best results first, and a clear message when nothing matches. In 0.1.0, a search whose words
  are not all found falls back to files containing some of them, with a notice; this is a stopgap,
  and a real answer to it is expected from search by understanding.
- **Official-grade experience.** Download, install and launch without detours, and a result in the
  first seconds, at the level of software from a large publisher.

Each guarantee becomes a task with its own test and, where it changes a decision, an ADR.

## Long-term BLUME direction

Beyond search, two ideas build on the same local index:

- **A local context graph**, the foundation: it remembers where each file or dataset comes from,
  what derives from it and what relates to it, and can be questioned. It extends Blume Finder from
  finding to understanding, and other BLUME projects can build on it.
- **A local data workbench**, the showcase: files are dropped in, a question is asked in plain
  language, and a model running on the computer writes and runs the processing locally, each step
  readable and replayable.

Both depend on local understanding of content (0.2.0) first, use only models running on the
computer (no remote AI service), and run any generated code in an isolated environment. They are
directions, not commitments.

## Where the project stands

On 20 October 2026: every task of the 0.1.0 plan is done and its checks are green. Since the scope
freeze, the maintainer approved interface and first-launch additions, all merged and covered by
the last full install test. Remaining before release: the first-launch offer shown again when no
folder is left, content matches ranked before name-only matches, no fallback to "any of the words",
a last disk image and install test, then the release itself.
