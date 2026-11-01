# Q-0146: Which conditions must two functions meet before their constraints decide between them?

**Status:** provisional
**Affects:** ORD-13, OVL-30, OVL-41, `tests/foundations/overload-resolution/tie-constraints-different-parameters.cpp`, `tests/foundations/overload-resolution/tie-constraints-default-parameter.cpp`, `tests/foundations/overload-resolution/tie-constraints-ellipsis.cpp`, `tests/templates/ordering/constraints-default-parameter.cpp`, `tests/templates/ordering/constraints-different-heads.cpp`, `tests/templates/ordering/constraints-conversion-results-differ.cpp`

## Context

ORD-13 states when constraints decide between two functions after every
other step ties. OVL-30 (plain functions), OVL-41 (template
specializations), and OVL-19 (the address of an overload set) use the
conditions of ORD-13. Each row below meets every condition except one.
With all conditions met, every compiler lets the constraint decide
(`tie-more-constrained.cpp`, `tie-more-specialized-constraints.cpp`,
`constraints-partial.cpp`, `constraints-conversion.cpp`).

| Case | Clang 22 | Clang 18 | GCC 14, 16 | EDG 7.0 |
|---|---|---|---|---|
| Two templates, `T` against `const T&` (an `int` lvalue gives both the identity) | ambiguous | ambiguous | ambiguous | the constrained template |
| Two templates, one with a defaulted second parameter | ambiguous | ambiguous | ambiguous | the constrained template |
| Two templates, heads `template <class T>` and `template <class T, class U = int>` | ambiguous | ambiguous | ambiguous | the constrained template |
| Two conversion function templates, results `T*` and `T* const`, target `int*` | ambiguous | ambiguous | ambiguous | the constrained template |
| Two plain functions, `f(int)` and `f(int, int = 0)` | ambiguous | ambiguous | ambiguous | the constrained function |
| Two plain functions, `f(int)` and `f(int, ...)` | ambiguous | the constrained function | ambiguous | the constrained function |
| Two plain functions, `f(int)` and `f(const int&)` | ambiguous | ambiguous | ambiguous | the constrained function |

Two templates that differ only in an ellipsis split the compilers in
another way. Q-0293 holds that case.

Q-0162 (plain functions) and Q-0210 (heads and conversion results)
asked the same question for other conditions. Both are duplicates of
this question.

## Evidence

The tests in **Affects**, each with an override for EDG, and for Clang
18 on the ellipsis. The columns "template const lref / constrained
template" and "template forwarding / constrained template" of the
templates grid of the overload resolution page. Gap G-15 of the
overload resolution inventory. The sanitized Clang source
(clang-sema-deduction) compares constraints only when the template
parameter lists are equivalent, the function parameter types are the
same, and, for conversion functions, the result types are the same.
Items M2 to M4 of the deduction inventory describe these conditions.
Item O-2 of the gap review of overload resolution
(`reports/review/2026-10-31-gap-review-overload-resolution.md`)
describes the two plain-function calls as ambiguous.

## Options

1. Constraints decide only between functions that meet every condition
   of ORD-13: the same parameter list, including defaulted parameters,
   the same parameter types, the same conversion result, equivalent
   template heads, and, for plain functions, the same ellipsis (Clang 22,
   GCC).
2. Constraints decide whenever the other steps tie (EDG).

## Provisional answer

Option 1. Clang 22 and GCC agree on every row, and the sanitized Clang
source and the gap review describe the same conditions.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
