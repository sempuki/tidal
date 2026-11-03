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

No for bounds, yes for temporaries; Clang differs on bounds, EDG on temporaries (option 1). Two of three implementations agree on each case. The rule
then keeps its one measure, the cv-qualifiers it adds.

The question stays open under decision 0051 (2026-11-03). For the array bound, Clang 22 prefers the known bound, and GCC 16 and EDG 7.0 find a tie. For the reference that binds a temporary, EDG 7.0 finds a tie, and Clang 22 and GCC 16 prefer the reference. So two families disagree on each case. For the temporary, the provisional answer follows ICS-P5. The reference's sequence is a standard sequence, and step 4f compares standard sequences however their references bind. For the bound, Tidal's own pages point the other way. ICS-24 and ICS-46 (Q-0188) prefer a known bound, and item E3 of the gap review describes a qualification step that prefers it too. The gap review of this cluster should weigh option 2 for the bound.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
