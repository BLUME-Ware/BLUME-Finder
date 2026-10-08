# 0005. Permanent application identifier

Date: 2026-10-07
Status: Accepted

## Context

The operating system uses the application identifier to recognize the application and to decide
where its data, including the index, is stored. Changing it after release would leave existing
users with an orphaned index and settings.

## Decision

The identifier is `fr.blumeware.finder`, based on the `blumeware.fr` domain. It never changes.

## Consequences

- The identifier does not depend on the code hosting service.
- Renaming the product or moving the repository does not affect it.
