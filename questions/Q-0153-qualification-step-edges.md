# Q-0153: Does the qualification step compare array bounds, and does it compare a reference that binds a temporary?

**Status:** provisional
**Affects:** ICS-31, ICS-P2, `tests/foundations/conversion-sequences/qualification-array-bound.cpp`, `tests/foundations/conversion-sequences/qualification-reference-temporary.cpp`

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

Yes for bounds, yes for temporaries; ICS-31 prefers the known bound (option 2), and compares a reference that binds a temporary (option 1).

Changed on 2026-11-03 for the array bound. The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) reads the reference as supporting option 2 for the bound. A qualification adjustment can drop an array bound (CONV-16), and the result that converts to the other is better. The review matches Clang 22, item E3 of the earlier gap review, and ICS-46 (Q-0188), which prefers the known bound for references. ICS-31 now gives option 2 for the bound. The test `qualification-array-bound.cpp` now compares the two pointers alone and expects the known bound, with overrides for GCC and EDG.

Kept on 2026-11-03 for the reference that binds a temporary. The same review agrees with option 1 there, and ICS-P5 compares standard sequences however their references bind.

The question stays open under decision 0051 (2026-11-03). For the array bound, Clang 22 prefers the known bound, and GCC 16 and EDG 7.0 find a tie. For the reference that binds a temporary, EDG 7.0 finds a tie, and Clang 22 and GCC 16 prefer the reference. So two families disagree on each case.

Kept open on 2026-11-03 under decision 0054, because two of the newest families differ from the answer in at least one form, and the rule records one differing family.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) reads the reference as supporting option 2 for the array bound and option 1 for the reference that binds a temporary. A pointer to an array of known bound converts to a pointer to an array of unknown bound by a qualification conversion, and not the reverse. For the temporary, the reference's sequence is the conversion to the referenced type, so only the qualification conversions differ.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
