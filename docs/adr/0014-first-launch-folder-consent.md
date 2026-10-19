# 0014. Offering the usual folders on first launch

Date: 2026-10-18
Status: Accepted

## Context

Until now an empty index waited for the user to find a small "Add a folder" link and to pick
folders one by one. On first contact, nothing showed that Blume Finder works with the files
already on the computer, and many people would stop there.

## Decision

- When the index has no folder and the offer was never declined, one sentence under the search
  field offers to read the Documents, Desktop and Downloads folders, with "Allow" and "Not now".
- "Allow" indexes those three folders, as if the user had added them. macOS then asks for access to
  each of them with its own dialog; the text of that dialog is set in `Info.plist`, in English and
  French.
- "Not now" keeps the earlier behavior and the offer is not shown again. The refusal is stored in
  the web view's local storage; if that storage is lost, the offer only comes back.
- No other folder is offered. The home folder as a whole is excluded: its `Library` holds mail,
  messages and the data of other applications.
- Everything else in ADR 0002 holds: read-only access, secret and hidden files never indexed.

## Consequences

- The first launch shows the brand, the search field and this offer only.
- With an ad-hoc signature, macOS may ask again for folder access after an update, since the
  permission is tied to the code signature. To be checked on each release.
- A folder the user refuses in the macOS dialog cannot be read; the error is shown like any
  unreadable folder.
