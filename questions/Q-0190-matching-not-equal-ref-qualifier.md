# Q-0190: Do ref-qualifiers take part in the match of `!=` and `==`?

**Status:** provisional
**Affects:** DCMP-13, `tests/classes/comparisons/ne-matching-ref-qualifier.cpp`

## Context

A class has `bool operator==(int) const&` and
`bool operator!=(int) const&&`. If the `!=` matches the `==`, the
reversed `==` is not a candidate, and `1 == m` has no viable function.

| Compiler | `const&` and `const&&` | no qualifier and `const&` | `const` and `const volatile` |
|---|---|---|---|
| Clang 18, Clang 22 | match | match | match |
| GCC 14, GCC 16 | no match | match | match |
| EDG 7.0 | match | match | match |

## Evidence

`tests/classes/comparisons/ne-matching-ref-qualifier.cpp` and
`ne-matching-other-result.cpp`.

## Options

1. The cv-qualifiers and ref-qualifiers of members take no part (Clang,
   EDG).
2. Two ref-qualifiers of different kinds make the functions differ
   (GCC).

## Provisional answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the rule then ignores
every qualifier of the object parameter.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
