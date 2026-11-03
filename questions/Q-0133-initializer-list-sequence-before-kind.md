# Q-0133: Does a list sequence to `std::initializer_list` beat a standard sequence when its own sequence is user-defined?

**Status:** provisional
**Affects:** ICS-21, ICS-22, `tests/foundations/conversion-sequences/initializer-list-over-kind.cpp`

## Context

A braced list `{1}` is the argument. One function takes
`std::initializer_list<Num>`, where `Num` has a converting constructor
from `int`. The other takes `int`. The sequence to the initializer list
is user-defined, and the sequence to `int` is the identity.

ICS-22 prefers a list sequence to an initializer list. ICS-21 prefers a
standard sequence to a user-defined one. The two rules disagree here.
When both sequences are standard, every compiler applies ICS-22, also
against a better rank (`initializer-list-over-rank.cpp`).

| Compiler | Result |
|---|---|
| GCC 14, GCC 16 | chooses `std::initializer_list<Num>` |
| EDG 7.0 | chooses `std::initializer_list<Num>` |
| Clang 18, Clang 22 | chooses `int` |

The split is the same against `long`, a conversion, in place of `int`.
When the list sequence to the initializer list is the identity, as for
`std::initializer_list<int>` against a class `Box` with a converting
constructor, every compiler chooses the initializer list.

## Evidence

`tests/foundations/conversion-sequences/initializer-list-over-kind.cpp`.
The sanitized EDG source (`edg-overload`) checks for an initializer-list
target before it compares the kinds of the two sequences. A source reader's
gap review (item O3 of
`reports/review/2026-10-31-gap-review-conversion-sequences.md`) reports
that the source compares the kinds first, which is Clang's result.

## Options

1. ICS-22 comes before the kind comparison. The initializer list wins
   (GCC, EDG).
2. The kind comparison comes first. ICS-22 decides only between
   sequences of the same kind (Clang).

## Provisional answer

No; the kind comparison comes first, and ICS-22 decides only between sequences of the same kind (option 2). Clang chooses `take(int)`.

Changed on 2026-11-03. The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) reads the reference as supporting option 2. The review matches Clang 22, item O3 of the earlier gap review, and ICS-P5, where the steps prefer the sequence with no user code. ICS-21 and ICS-22 now give option 2, and the test expects `take(int)`, with overrides for GCC and EDG.

The question stays open under decision 0051 (2026-11-03). Clang 22 chooses `take(int)`, and GCC 16 and EDG 7.0 choose the initializer list, so two families disagree.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) reads the reference as supporting option 2. The kinds of the two sequences compare first, and the initializer-list preference acts only between sequences of the same kind. A list sequence whose worst element conversion is user-defined is a user-defined sequence, so the identity to `int` wins.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
