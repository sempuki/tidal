# Q-0219: Does partial ordering choose among placement deallocation function templates?

**Status:** provisional
**Affects:** DED-39, `tests/templates/deduction/placement-deallocation-ordered.cpp`

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

EDG has no code generation. A probe with a failing `static_assert` in
the body of each template shows which one EDG instantiates.

## Evidence

`tests/templates/deduction/placement-deallocation-ordered.cpp`, with an
override for Clang. Items M7 and M10 of the gap review of 2026-10-31
(`reports/review/2026-10-31-gap-review-deduction-and-ordering.md`)
describe partial ordering in this context.

## Options

1. Partial ordering chooses the more specialized template (GCC, EDG).
2. Two matching templates give no deallocation function (Clang).

## Provisional answer

Yes; GCC and EDG agree, Clang calls none (option 1). GCC and EDG agree, and the gap review describes the same use
of partial ordering.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
