# 0008. Free distribution, portable code

Date: 2026-10-07
Status: Accepted

## Context

The project is non-profit and must not depend on any paid service or account. Its first release
targets macOS, but it is meant to run on every desktop system.

## Decision

- No paid service or account is required to build, test or ship the software.
- The first release is for macOS, unsigned, distributed through GitHub Releases.
- The code stays portable: paths go through `std::path`, nothing is hard-coded per platform, and
  platform-specific code is isolated behind `cfg`. CI builds and tests the engine on Linux and
  Windows as well.

## Consequences

- On first launch, macOS blocks the unsigned application until the user chooses "Open Anyway" in
  System Settings, Privacy & Security. The download page has to explain this step.
- Windows and Linux releases only need packaging work, not code changes.
