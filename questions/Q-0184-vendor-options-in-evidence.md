# Q-0184: Should Evidence record vendor options that change behavior?

**Status:** provisional
**Affects:** `documentation/specification-format.md` (Evidence), INIT-1 and other rules of undefined or indeterminate values
**Waits for:** a review (silent)

## Context

Some compiler options change what a program with undefined behavior
does, for example an option that fills automatic variables with zero.
The matrix runs each compiler with fixed flags (`tools/compilers.json`).
A reader of proposal X3 asked whether Evidence should list such
options.

## Evidence

Finding X3-10 in `reports/clean-room/X3/report.md`.

## Options

1. Evidence records the matrix configurations only.
2. Evidence can name an option and its effect when a test runs the
   compiler with that option.

## Provisional answer

Yes, when a test runs the compiler with the option (option 2). Each configuration is an implementation (decision 0047), so a
configuration that a test runs is evidence. An option no test runs is
not.

Kept open on 2026-11-03 under decision 0054, because the review finds the reference silent.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) finds the reference silent. The question concerns Tidal's Evidence.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
