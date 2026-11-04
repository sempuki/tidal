# Q-0154: Does the nearest-base tie-breaker apply to a braced list with one element of a derived class?

**Status:** provisional
**Affects:** ICS-19, ICS-36, `tests/foundations/conversion-sequences/list-class-nearest-base.cpp`
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences

## Context

`Derived` derives from `Middle`, and `Middle` from `Base`. Each class has
a user-declared default constructor, so none is an aggregate. The
argument is `{d}`, where `d` is a `Derived` lvalue. One function takes
`Middle` and the other takes `Base`, both by value. ICS-19 gives each
list the element's sequence, a derived-to-base conversion.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | `Middle` |
| GCC 14, GCC 16 | `Middle` |
| EDG 7.0 | ambiguous |

With aggregate classes (ICS-18), every compiler chooses `Middle`
(`list-aggregate-nearest-base.cpp`).

## Evidence

`tests/foundations/conversion-sequences/list-class-nearest-base.cpp`,
`tests/foundations/conversion-sequences/list-aggregate-nearest-base.cpp`,
and the row "braced {d}" of the classes grid on the page. A source
reader's gap review (item M8) reports that the source gives the
sequence conversion rank and does not say whether the tie-breaker
applies.

## Options

1. The element's sequence is the sequence of the element as an
   argument, so ICS-36 applies (Clang, GCC).
2. The list sequence keeps only its rank, so the two functions tie
   (EDG).

## Provisional answer

Yes; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and every compiler
applies ICS-36 to the same list for an aggregate.

Kept open on 2026-11-03 under decision 0054, because the review finds the question unclear.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) finds the question unclear. For a class that is not an aggregate and one element of a derived class, the sequence has conversion rank, and nothing calls it a derived-to-base conversion. The nearest-base tie-breaker is stated for standard sequences, and nothing says whether a list sequence described only by its rank counts. For an aggregate, the element's own sequence applies, and the tie-breaker applies there. The uniform answer is a reasonable reading.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
