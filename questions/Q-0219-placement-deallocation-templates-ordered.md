# Q-0219: Does partial ordering choose among placement deallocation function templates?

**Status:** provisional
**Affects:** DED-39, `tests/templates/deduction/placement-deallocation-ordered.cpp`, `tests/templates/deduction/placement-deallocation-ordered-instantiation.cpp`

## Context

A class has the placement allocation function
`operator new(size, Arena*)` and two placement deallocation function
templates, `operator delete(void*, T*)` and `operator delete(void*, T)`.
Both templates can match the allocation function, with `T = Arena` and
`T = Arena*`. The constructor throws.

| Compiler | Result |
|---|---|
| GCC 14, 16, host | calls the template with `T*` |
| EDG 7.0 | instantiates the template with `T*` |
| Clang 18, 22, host | calls no deallocation function, and warns that two match |

EDG has no code generation. In
`placement-deallocation-ordered-instantiation.cpp`, the template with
`T` holds a failing `static_assert`, and EDG accepts the program. A
probe with the `static_assert` in the template with `T*` is rejected by
EDG and GCC and accepted by Clang.

## Evidence

`tests/templates/deduction/placement-deallocation-ordered.cpp`, with an
override for Clang, and
`tests/templates/deduction/placement-deallocation-ordered-instantiation.cpp`. Items M7 and M10 of the gap review of 2026-10-31
(`reports/review/2026-10-31-gap-review-deduction-and-ordering.md`)
describe partial ordering in this context.

## Options

1. Partial ordering chooses the more specialized template (GCC, EDG).
2. Two matching templates give no deallocation function (Clang).

## Provisional answer

Yes; GCC and EDG agree, Clang calls none (option 1). GCC and EDG agree, and the gap review describes the same use
of partial ordering.

The question stays open under decision 0051 (2026-11-03). GCC 16 calls the template with `T*`, EDG 7.0 instantiates the same template (`placement-deallocation-ordered-instantiation.cpp`), and Clang 22 calls no deallocation function, so two families disagree. ORD-P1 supports the provisional answer. Its table lists the placement deallocation match as a context of partial ordering. Items M7 and M10 of the gap review describe the same use.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
