# Q-0398: Does the object parameter of a `const` member keep the `const` in partial ordering?

**Status:** settled
**Affects:** ORD-6, `tests/templates/ordering/member-object-const.cpp`
**Source:** a reduction of finding I2-15 of `reports/clean-room/I2/report.md` (`reductions/r3b-member-not-swapped.cpp`), and the probes that a clean-room writer ran for it.

## Context

ORD-6 gives a member template a first parameter of type reference to
its class, with the member's qualifiers. The test orders a `const`
member `operator+(T*)` against a template `operator+(const Grid&, T)`
for `grid + p`, with `grid` a `const Grid`. The object parameter is
`const Grid&`, so the first pair ties, and `T*` is more specialized
than `T`.

| Implementation | Result |
|---|---|
| Clang 22 | chooses the member |
| GCC 16 | chooses the member |
| EDG 7.0 | rejects: more than one operator matches |

EDG gives the same answer for the reduction of the report, where the
member is `operator==(T* const&) const` and the other template is
`operator==(const X&, const T&)`. EDG chooses the member when neither
the member nor the other template's first parameter is `const`
(`member-object.cpp`). So EDG seems to drop the `const` of the member
from the object parameter, and the first pair then prefers the other
template by ORD-11.

## Evidence

The test in **Affects**, with an override for EDG. The probes of the
writer also ran the reduction `r3b-member-not-swapped.cpp` and a form
with by-value parameters, with the same results.

## Options

1. The object parameter has the member's `const` and `volatile`, and
   the member wins (Clang, GCC).
2. The object parameter has no `const`, and the call is ambiguous
   (EDG).

## Answer

The object parameter keeps the member's `const` (option 1). Two of
three families agree. The object parameter stands for the way the
member binds its object in a call (ORD-P1), and a `const` member binds
its object through a `const` reference.

The question stays open under decision 0051, because EDG 7.0 differs
from Clang 22 and GCC 16.

Settled on 2026-11-05 under decision 0054. The gap review of the text phase 9 wrote (item RV3) supports the answer with no doubt. At most one newest family differs in each form, and each test keeps its override.

## Opinions

### Gap review, 2026-11-05 (expressions and templates)

The source-reader gap review of the text phase 9 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) reads the question: supports with no doubt. The object parameter is a reference to the class with the member's qualifiers.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
