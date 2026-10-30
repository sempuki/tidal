# 0011. The similarity check is automated and may be run by anyone

Date: 2026-10-30
Status: Accepted

## Context

Decision 0003 gives the similarity check to a checker who has read the ISO
text. The check itself is mechanical: `tools/similarity.py` reads a
reference corpus and prints only Tidal text and scores. No person needs to
read the corpus for the check to work.

## Decision

- The checker role is filled by `tools/similarity.py`. Anyone may run it,
  including the writer.
- The reference corpus is WG21 document N4950 (the C++23 working draft),
  taken from tag `n4950` of the `cplusplus/draft` repository into a
  directory outside the Tidal repository. Nobody opens it, and it is never
  committed.
- Reports are committed to `reports/similarity/` with the reference tag and
  the Tidal commit checked.

## Consequences

- The check can run in continuous integration on every change.
- The writer sees only Tidal text in reports, as before.
- Thresholds are an open question (Q-0008).
