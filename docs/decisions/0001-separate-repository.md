# 0001. Tidal lives in its own git repository

Date: 2026-10-30
Status: Accepted

## Context

The project was started inside a working copy of the author's WG21 papers
repository. That repository contains drafts of WG21 papers (for example
P2034), and those papers quote ISO standard wording as part of their proposed
changes.

Tidal's git history is meant to be its provenance record. If Tidal commits
were mixed into the WG21 history, publishing Tidal would mean publishing ISO
wording alongside it, and an observer could not easily tell which text came
from where.

## Decision

`tidal/` is a separate git repository (`git init` inside the directory). The
parent WG21 repository does not track it.

## Consequences

- Tidal's history contains only Tidal work and can be published as is.
- The parent repository will show `tidal/` as untracked. The author may add it
  to the parent's `.gitignore`.
- Placing the repository on the same disk as WG21 material is still a weak
  point. It is acceptable for a proof of concept and is listed in
  `docs/clean-room.md` under known limitations.
