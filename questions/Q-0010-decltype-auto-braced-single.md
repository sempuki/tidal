# Q-0010: Is `decltype(auto) x{e};` valid, deducing from `e`?

**Status:** settled
**Affects:** AUTO-11, `tests/declarations/auto/decltype-auto-braced.cpp`

## Context

`decltype(auto)` is rejected with `= { ... }` (AUTO-11), and `auto` with a
single-element braced initializer deduces from the element (AUTO-6). For
`decltype(auto)` with a single-element braced initializer and no `=`,
rejection is an easy wrong prediction. All seven compilers accept it and
deduce from the element, including its value category: `decltype(auto)
b{(i)};` gives `int&`.

## Evidence

`tests/RESULTS.md`, rows `decltype-auto-braced.cpp` and
`decltype-auto-braced-multiple.cpp`. Clang 22.1.8 and 18.1.3, GCC 16.2.1,
16.2.0, and 14.2.0, and EDG 7.0 all accept the single-element form and
reject the multiple-element form.

## Options

1. Valid, deducing from the single element as if the braces were absent.
   Matches every implementation.
2. Rejected. Programs using it today would break on a conforming
   implementation.

## Answer

Yes, as all implementations do (option 1). Tidal describes what implementations do, and they agree.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. No older version differs.

## Opinions

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) supports the answer. For a braced initializer with one element and no equals sign, the deduction uses the element as the operand of `decltype`, and a list of several elements is an error.
