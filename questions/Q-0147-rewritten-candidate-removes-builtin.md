# Q-0147: Does a rewritten candidate remove the built-in candidate with the same parameter list?

**Status:** provisional
**Affects:** OVL-10, `tests/foundations/overload-resolution/builtin-removed-by-rewritten.cpp`

## Context

An unscoped enumeration `Color` has a non-member `operator<=>(Color,
Color)`. The expression `red < blue` has the built-in candidate for `<`
with the parameters `(Color, Color)`, and the rewritten candidate
`(red <=> blue) < 0` with the same parameters.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | the rewritten candidate |
| EDG 7.0 | the rewritten candidate |
| GCC 14, GCC 16 | the built-in candidate |

If both candidates stay, the built-in one wins, because a candidate
that is not rewritten beats a rewritten one (OVL-32). A non-member
`operator==(Color, Color)` that is not rewritten removes the built-in
`==` on every compiler (`builtin-same-shape-removed.cpp`).

## Evidence

`tests/foundations/overload-resolution/builtin-removed-by-rewritten.cpp`.
Item BI-5 of the overload resolution inventory names rewritten
candidates among those that remove a built-in candidate.

## Options

1. The rewritten candidate removes the built-in candidate (Clang, EDG).
2. Only a candidate that is not rewritten removes it (GCC).

## Provisional answer

Option 1. Two of three implementations agree, and the inventory
describes the removal this way.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
