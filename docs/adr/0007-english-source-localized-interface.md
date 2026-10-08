# 0007. English source, localized interface

Date: 2026-10-07
Status: Accepted

## Context

The project is open source and meant to outlive its first developer. Users, on the other hand,
should see the application in the language of their system.

## Decision

- Code, comments, commit messages and technical documentation are written in English.
- Every user-facing string lives in a translation catalog, never inline in components. English is
  the reference catalog; each locale has its own catalog bundled with the application. English and
  French ship first.
- The locale is chosen at startup from the system language, with English as the fallback.
  Translations are written ahead of time; nothing is ever translated online.

## Consequences

- Adding a language means adding a catalog, without touching components.
- Public documents (`README.md`, `SECURITY.md`) are written in English and French.
