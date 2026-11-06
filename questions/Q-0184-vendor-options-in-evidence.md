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

Kept open on 2026-11-05 under decision 0054, because the review reads the reference as silent (gap review of the text phase 9 wrote, item RV3).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) finds the reference silent. The question concerns Tidal's Evidence.

### Gap review, 2026-11-05 (declarations, statements, and foundations)

The source-reader gap review of the text phase 9 wrote on declarations, statements, and foundations (`reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`) reads the question: silent. The question is about what Tidal's Evidence records. The reference says nothing on vendor options.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
