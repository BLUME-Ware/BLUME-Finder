# 0002. Read-only access to user files

Date: 2026-10-07
Status: Accepted

## Context

Users point the application at their own documents. Any write, move or deletion, even by mistake,
would be a severe breach of trust, and some files must never be read at all.

## Decision

- The code never moves, renames, modifies or deletes a user file. The only file written is the
  index, in the application data directory.
- Removing a folder only removes its entries from the index.
- Secret files (keys, `.env` files, certificates, keychains, password databases) are never read
  nor indexed, not even by name. The list of secret patterns can grow, never shrink.
- Hidden folders, applications and dependency folders are skipped. Symbolic links are not followed.

## Consequences

- Features that organize files (renaming, tagging, moving) are out of scope.
- Opening a file or showing it in its folder is delegated to the operating system, and only for
  files present in the index.
