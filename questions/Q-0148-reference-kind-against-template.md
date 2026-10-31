# Q-0148: Does the plain-over-template tie-breaker come before the function reference kind?

**Status:** provisional
**Affects:** OVL-27, OVL-29, `tests/foundations/overload-resolution/tie-template-before-reference-kind.cpp`

## Context

A class has a plain `operator F&()` and a conversion function template
`operator T&&()`, where `F` is a function type. A reference of type
`F&&` binds directly to the result of either one, with the identity.
The kind of reference prefers the template (OVL-29). The plain function
over the template prefers `operator F&` (OVL-27).

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | `operator F&` |
| EDG 7.0 | `operator F&` |
| GCC 14, GCC 16 | the template |

Clang does not apply the kind of reference at all (Q-0138), so only EDG
shows the order of the two tie-breakers.

## Evidence

`tests/foundations/overload-resolution/tie-template-before-reference-kind.cpp`.
Items RK-4 and RK-5 of the overload resolution inventory.

## Options

1. A plain function beats a template first (EDG, and the result of
   Clang).
2. The kind of reference decides first (GCC).

## Provisional answer

Option 1. Two of three implementations choose the plain function.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
