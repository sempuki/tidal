# Q-0036: May the size in an array `new` have floating-point type?

**Status:** provisional
**Affects:** NEW-2, `tests/expressions/new-delete/array-non-integral-size.cpp`, `tests/expressions/new-delete/array-floating-size-exact.cpp`

## Context

`new int[1.5]` is accepted by Clang 22 and 18, with a `-Wliteral-conversion` warning, and by EDG 7.0. Both convert the size to 1. GCC 16 and 14 reject it ("must have integral or enumeration type").

## Options

1. Accepted: the size converts to the type of `sizeof`, as any value converts.
2. Rejected: only integer and enumeration sizes are allowed.

## Provisional answer

Yes; converted to the type of `sizeof` (option 1). Two of three implementations accept the floating-point size.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. NEW-P1 says that an array can have a size known only at run time, and it does not say which types the size can have. GCC 16 rejects the floating-point size.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The first array size in a `new` expression converts implicitly to the type of `sizeof`. A floating-point value converts by the ordinary floating-integral conversion, so the form is valid. An older rule limited the size to integral and enumeration types, and GCC still applies that rule.
