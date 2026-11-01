# Q-0148: Does the function reference kind come before the template tie-breakers?

**Status:** provisional
**Affects:** OVL-27, OVL-28, OVL-29, `tests/foundations/overload-resolution/tie-reference-kind-before-template.cpp`, `tests/foundations/overload-resolution/tie-reference-kind-before-template-lvalue.cpp`, `tests/foundations/overload-resolution/tie-reference-kind-before-more-specialized.cpp`

## Context

A class has two conversion functions to a function type `F`, and a
reference to `F` binds directly to the result of either one with the
identity. The kind of reference (OVL-29) prefers one function. A
template tie-breaker (OVL-27 or OVL-28) prefers the other.

| Conversion functions | Target | Kind prefers | Clang 18, 22 | EDG 7.0 | GCC 14, 16 |
|---|---|---|---|---|---|
| plain `F&`, template `T&&` | `F&&` | the template | plain | plain | the template |
| plain `F&&`, template `T&` | `F&` | the template | the template | plain | the template |
| template `T&`, template `T&&` | `F&&` | `T&&` | `T&` | `T&` | `T&&` |

Clang does not apply the kind of reference (Q-0138). For an lvalue
reference, Clang has only the `T&` candidate. For the two templates,
Clang chooses `T&`, the more specialized template. So Clang's results
do not show an order. EDG applies both template tie-breakers first, and
GCC applies the kind of reference first.

## Evidence

`tests/foundations/overload-resolution/tie-reference-kind-before-template.cpp`,
`tie-reference-kind-before-template-lvalue.cpp`, and
`tie-reference-kind-before-more-specialized.cpp`. The probe grid
`grids/reference-kind.json`. Items RK-4 and RK-5 of the overload
resolution inventory. Item O-1 of the gap review of overload resolution
(`reports/review/2026-10-31-gap-review-overload-resolution.md`) places
the kind of reference before the plain function over a template.

## Options

1. The template tie-breakers decide first (EDG).
2. The kind of reference decides first (GCC).

## Provisional answer

Option 2. Of the two implementations that apply both tie-breakers, one
takes each order. The gap review places the kind of reference first.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
