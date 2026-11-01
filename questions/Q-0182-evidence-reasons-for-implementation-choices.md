# Q-0182: May Evidence give the reason for an implementation's choice?

**Status:** provisional
**Affects:** `documentation/specification-format.md` (Evidence), Evidence segments that describe orders and layouts

## Context

Evidence segments record what the compilers do, for example that GCC
evaluates the arguments of a call from right to left. A reader of
proposal X1 asked for the reason, such as a calling convention. Tidal
has no clean input that states such reasons. The compilers show the
choice and not the reason.

## Evidence

Finding X1-14 in `reports/clean-room/X1/report.md`. The order tables in
the Evidence of `specification/foundations/evaluation-order.md`.

## Options

1. Evidence records observed behavior only.
2. Evidence can give a reason when a clean input states it, such as a
   sanitized source or a compiler's documented option.
3. Evidence can give a reason from general knowledge, marked as such.

## Provisional answer

Option 2. A reason is useful to a reader, and a clean input keeps the
clean-room rules. A reason with no clean input stays out.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
