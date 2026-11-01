# 0001. Tidal lives in its own git repository

Date: 2026-10-30
Status: Accepted

## Context

WG21 papers quote ISO standard wording as part of their proposed changes.
Tidal's git history is its provenance record, so it must not share a
repository with such papers.

## Decision

Tidal is a standalone git repository, stored and published separately
from any WG21 papers repository.

## Consequences

- Tidal's history contains only Tidal work and can be published as is.
- The public repository is https://github.com/sempuki/tidal.
- The writer works only inside this repository and its `inputs/` directory.
