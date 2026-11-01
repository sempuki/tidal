# Q-0185: How does Tidal mark a change to a rule and the language version a rule holds for?

**Status:** provisional
**Affects:** `documentation/specification-format.md`, Q-0007, RFOR-4

## Context

A proposal changes rules, and a reader needs to see which rules change
and how. The language itself also changes between versions: RFOR-4
records a change in when the temporaries of a range-based `for` end.
Tidal has no notation for a rule change and no field for the version a
rule holds for. Grammar productions have no IDs that a change can cite.

## Evidence

Findings X2-13 and X4-12 in `reports/clean-room/X2/report.md` and
`reports/clean-room/X4/report.md`. The roadmap's parallel lane for a
change notation.

## Options

1. A change table for proposals (rule ID, kind of change, old class,
   new class, tests affected), and no version field.
2. The change table, plus a `:since:` field on rules that differ from
   an earlier version, and anchors for productions.

## Provisional answer

Option 1 now, because proposals need it and Tidal specifies one
version. Option 2 when Tidal specifies a second version.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
