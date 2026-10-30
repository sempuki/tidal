# 0001. Tidal lives in its own git repository

Date: 2026-10-30
Status: Accepted

## Context

The directory containing `tidal/` is a WG21 papers repository. It holds
drafts of WG21 papers, and those papers quote ISO standard wording as part
of their proposed changes.

Tidal's git history is its provenance record. Mixing Tidal commits into
the WG21 history would mean publishing ISO wording alongside Tidal, and an
observer could not easily tell which text came from where.

## Decision

`tidal/` is a separate git repository (`git init` inside the directory). The
parent WG21 repository does not track it.

## Consequences

- Tidal's history contains only Tidal work and can be published as is.
- The parent repository shows `tidal/` as untracked. Adding it to the
  parent's `.gitignore` hides it.
- Tidal needs its own public remote before contributors can join.
- The writer works only inside this repository and its `inputs/` directory.
