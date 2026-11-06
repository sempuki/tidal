# Q-0417: Does a new expression for an over-aligned class fall back to the unaligned form when the class's aligned form is deleted or private?

**Status:** provisional
**Affects:** NEW-29, NEW-25, `tests/expressions/new-delete/aligned-deleted-form-beside-unaligned.cpp`, `tests/expressions/new-delete/aligned-private-form-beside-unaligned.cpp`
**Source:** finding X23-2 of `reports/clean-room/X23/report.md`, from the probes `d2.cpp` and `d3.cpp` beside it.
**Waits for:** a review (doubt)

## Context

An over-aligned class declares a public `operator new(std::size_t)` and
an `operator new(std::size_t, std::align_val_t)` that is deleted, or
private. A `new` expression for the class first passes the alignment
(NEW-28). NEW-29 tries again without it "if no allocation function
accepts the arguments".

| Implementation | Result |
|---|---|
| Clang 22 | rejects: the selected aligned form is deleted or private |
| GCC 16 | calls the unaligned form |
| EDG 7.0 | rejects |

## Evidence

The tests in **Affects**, each with an override for GCC.

## Options

1. The selected aligned form accepts the arguments, so the expression
   is not valid and does not try again (Clang, EDG).
2. A deleted or private form does not accept them, and the expression
   tries again without the alignment (GCC).

## Provisional answer

Not valid (option 1). Two of three families agree.

Kept open on 2026-11-05 under decision 0054, because the review reads the answer with a doubt (item RV5).

## Opinions

### Gap review, 2026-11-05 (phase 12 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 12 wrote on expressions, overloading, and templates (`reports/review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md`) reads the question: supports with a doubt (RV5-B-13). The second search happens only when overload resolution finds no function. A deleted or inaccessible aligned form takes part in overload resolution and is selected, and the checks for deletion and access come after the selection, so the search ends and the program is not valid. The doubt is that the reference does not define what passes the test for the second search.
