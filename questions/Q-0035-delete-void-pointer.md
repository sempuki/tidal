# Q-0035: Is `delete` of a `void*` rejected?

**Status:** settled
**Affects:** NEW-8, `tests/expressions/new-delete/delete-void-pointer.cpp`

## Context

GCC 16 and 14 accept `delete p;` where `p` is a `void*`, with a warning that the deletion is undefined. Clang 22 and 18 reject the deletion under `-pedantic-errors` (`-Wdelete-incomplete`). EDG 7.0 rejects it.

## Options

1. Rejected: `delete` needs a pointer to a complete object type.
2. Accepted, with undefined behavior at run time, as GCC treats it.

## Answer

Yes (option 1). Two of three implementations reject the deletion, and GCC's own warning says the deletion can't work.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and NEW-P2 gives the same answer: the operand of `delete` needs a pointer to an object type, so that the implementation knows which destructor to run. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The operand of `delete` must be a pointer to an object type, or a class that converts to one. A `void*` is not a pointer to an object type, so a diagnostic is required. GCC's warning is a diagnostic, and its acceptance after the warning is an extension.
