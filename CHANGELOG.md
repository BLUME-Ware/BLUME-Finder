# Changelog

All notable changes to Blume Finder are recorded in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Hidden files and plain-text secret exports, such as `credentials.csv`, are no longer indexed.
- Removing a folder while another one is being indexed no longer fails with "database is locked".
- An indexed folder and its subfolders no longer overlap: adding a subfolder of an indexed folder
  is refused, and adding a parent folder takes over the indexed folders it contains.
- Text removed from the index, when a folder is removed or a file changes or disappears, is now
  erased from the index file and its journal instead of only being marked as deleted.
