# Q-0153: Does the qualification step compare array bounds, and does it compare a reference that binds a temporary?

**Status:** provisional
**Affects:** ICS-31, `tests/foundations/conversion-sequences/qualification-array-bound.cpp`, `tests/foundations/conversion-sequences/qualification-reference-temporary.cpp`

## Context

ICS-31 prefers the sequence that adds fewer cv-qualifiers when two
sequences differ only in a qualification adjustment. Two cases go past
cv-qualifiers.

A pointer to an array of three `int` goes to a pointer to an array of
three `const int`, or to a pointer to an array of unknown bound of
`const int`. Both add `const`. The second also drops the bound.

An `int*` goes to `const int* const&`, which binds a temporary, or to a
`const volatile int*` value.

| Compiler | Array bound | Reference to a temporary |
|---|---|---|
| Clang 18, Clang 22 | the known bound | `const int* const&` |
| GCC 14, GCC 16 | tie | `const int* const&` |
| EDG 7.0 | tie | tie |

## Evidence

The two tests in **Affects**. A source reader's gap review (item E3 of
`reports/review/2026-10-31-gap-review-conversion-sequences.md`) reports
that the source lets the qualification step drop an array bound and
prefers the result that converts to the other.

## Options

1. ICS-31 counts only cv-qualifiers, so a bound does not decide. ICS-31
   also compares a reference that binds a temporary (GCC, and Clang on
   the second case).
2. ICS-31 also prefers the known bound (Clang).
3. ICS-31 does not compare a reference that binds a temporary (EDG).

## Provisional answer

Option 1. Two of three implementations agree on each case. The rule
then keeps its one measure, the cv-qualifiers it adds.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
