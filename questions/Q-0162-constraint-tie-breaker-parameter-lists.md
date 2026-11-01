# Q-0162: Does the constraint tie-breaker apply when the parameter lists differ beyond the parameters that the call uses?

**Status:** provisional
**Affects:** OVL-30, `tests/foundations/overload-resolution/tie-constraints-default-parameter.cpp`, `tests/foundations/overload-resolution/tie-constraints-ellipsis.cpp`

## Context

A class template has two plain static members. The first is `f(int)`.
The second is `f(int, int = 0)` or `f(int, ...)`, and its constraint
subsumes the first one's. A call passes one `int`, so every sequence
ties.

| Compiler | Defaulted parameter | Ellipsis | `f(int)` against `f(const int&)` |
|---|---|---|---|
| Clang 22 | ambiguous | ambiguous | ambiguous |
| Clang 18 | ambiguous | the constrained function | ambiguous |
| GCC 14, GCC 16 | ambiguous | ambiguous | ambiguous |
| EDG 7.0 | the constrained function | the constrained function | the constrained function |

With the same parameter list, every compiler chooses the more
constrained function (`tie-more-constrained.cpp`).

## Evidence

`tests/foundations/overload-resolution/tie-constraints-default-parameter.cpp`
and `tie-constraints-ellipsis.cpp`. Item O-2 of the gap review of
overload resolution
(`reports/review/2026-10-31-gap-review-overload-resolution.md`)
describes both calls as ambiguous. Q-0146 records the same lenience of
EDG for two templates.

## Options

1. The two functions need the same full parameter list, including
   defaulted parameters and the ellipsis (Clang 22, GCC).
2. Only the parameters that the call uses must match (EDG).

## Provisional answer

Option 1. Two implementations agree in their newest versions, and the
gap review describes it.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
