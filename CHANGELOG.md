# Changelog

All notable changes to Blume Finder are recorded in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- First public release, for macOS 13 or later on Apple Silicon and Intel, as a disk image with an
  ad-hoc signature, not notarized.
- Search of files by the words they contain and by their name, tolerating plurals, accents and
  partial words, with the matching passage highlighted.
- Text extraction from plain text, Markdown, CSV, HTML, PDF with a text layer, Word, PowerPoint,
  Excel and OpenDocument files. Other files, such as scans and images, are found by name only.
- A local SQLite index, re-read at launch for changed files only, with removed text erased from
  the index file.
- Folder management: adding and removing indexed folders, without overlap between a folder and
  its subfolders.
- Opening a result, or showing it in its folder.
- Read-only access to files, no network access, and hidden and secret files never indexed.
- A single-view interface in light and dark themes, in English or French following the system
  language.
- The `blume` command-line tool, to index and search from a terminal.
