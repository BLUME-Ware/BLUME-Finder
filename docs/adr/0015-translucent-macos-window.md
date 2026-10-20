# 0015. Translucent window on macOS

Date: 2026-10-20
Status: Accepted

## Context

The maintainer wants the window background translucent and blurred, as in Finder and other Apple
applications. CSS cannot blur what lies behind the window: only the system can, through a
transparent window and a visual effect material. Tauri exposes this with the `macOSPrivateApi`
option, which relies on an undocumented macOS interface.

## Decision

- `macOSPrivateApi` is enabled in `tauri.conf.json`, with the matching `macos-private-api` feature
  of the `tauri` crate. It adds no crate to the dependency tree; it only matters on macOS.
- The transparent window and the `underWindowBackground` material, which follows the window's
  active state, are set in `tauri.macos.conf.json`, read on macOS only. Windows and Linux keep an
  opaque window.
- On macOS the page background is transparent; the search field, folder chips and result cards
  keep a semi-opaque backing so that text stays readable on any wallpaper.

## Consequences

- An undocumented interface can change with a macOS update. If the blur breaks or hurts
  readability, the fallback is to remove `tauri.macos.conf.json` and the option, which restores the
  opaque window.
- The application cannot be distributed through the Mac App Store, which the project does not use.
- Readability is checked on a light and a dark wallpaper at each release.
