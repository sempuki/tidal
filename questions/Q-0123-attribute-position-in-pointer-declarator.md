# Q-0123: Does an attribute in a pointer declarator come before or after the cv-qualifiers?

**Status:** provisional
**Affects:** ATTR-18, the `ptr_part` production, `tests/declarations/attributes/pointer-attribute-after-cv.cpp`, `tests/declarations/attributes/pointer-attribute-before-cv.cpp`

## Context

An attribute specifier can follow the `*` of a pointer declarator. With
a cv-qualifier there too, the compilers disagree on the order. GCC 14 and 16 and
EDG 7.0 accept `* const [[]]` and reject `* [[]] const`. Clang 18 and 22
accept `* [[]] const` and reject `* const [[]]`. All three accept `* [[]]`
with no qualifier.

## Evidence

`tests/RESULTS.md`, rows `pointer-attribute-after-cv.cpp` and
`pointer-attribute-before-cv.cpp`.

## Options

1. The attribute comes after the cv-qualifiers. Clang differs.
2. The attribute comes before the cv-qualifiers. GCC and EDG differ.

## Provisional answer

Option 1. Two of three implementations take this order.

## Opinions
