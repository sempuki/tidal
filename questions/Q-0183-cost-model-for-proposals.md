# Q-0183: Should Tidal give proposals a way to state implementation cost?

**Status:** provisional
**Affects:** `documentation/exercises.md` (Propose), designer finding D32

## Context

A proposal states what a change costs implementations and programs.
Tidal has no model for cost. The readers of proposals X3 and X4 could
say only that a change looks cheap, with no measure behind it. One
measure Tidal could produce is a count from the clean-room front end,
for example how many default-initialized scalars each function has in
a body of programs.

## Evidence

Findings X3-9 and X4-13 in `reports/clean-room/X3/report.md` and
`reports/clean-room/X4/report.md`.

## Options

1. No cost model. A proposal states cost in words.
2. The front end reports counts of constructs that a proposal names,
   over the test suite and any program corpus that Tidal adds.

## Provisional answer

No cost model for now; a proposal states cost in words (option 1). The test suite is small fragments, so counts over it
would mislead. Option 2 becomes useful once Tidal has a corpus of whole
programs.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
