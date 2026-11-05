# Q-0137: Do the pointer tie-breakers apply after an array-to-pointer step?

**Status:** provisional
**Affects:** ICS-26, ICS-33, ICS-36, `tests/overloading/conversion-sequences/array-pointer-tie-breaker.cpp`
**Waits for:** a review (doubt)

## Context

The argument is an array of `Derived`, where `Derived` derives from
`Middle` and `Middle` from `Base`. The array becomes a pointer, and the
pointer converts to `void*`, `Base*`, or `Middle*`. For a pointer
argument, every compiler applies ICS-33 (a base pointer beats `void*`)
and ICS-36 (the nearer base wins).

| Compiler | `void*` against `Base*` | `Base*` against `Middle*` |
|---|---|---|
| Clang 18, Clang 22 | `Base*` | `Middle*` |
| EDG 7.0 | `Base*` | `Middle*` |
| GCC 14, GCC 16 | ambiguous | ambiguous |

## Evidence

`tests/overloading/conversion-sequences/array-pointer-tie-breaker.cpp`,
and the row "Derived[2]" of the pointers grid in `tests/overloading/conversion-sequences/grids/tables.md`.

## Options

1. The tie-breakers apply. Sequences compare without their lvalue
   transformations (ICS-26), so an array argument ranks as a pointer
   argument does (Clang, EDG).
2. The tie-breakers do not apply after an array-to-pointer step (GCC).

## Provisional answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the answer keeps
lvalue transformations out of every comparison.

The question stays open under decision 0051 (2026-11-03). GCC 16 finds both calls ambiguous, and Clang 22 and EDG 7.0 apply the tie-breakers, so two families disagree. The provisional answer follows ICS-26, which compares two standard sequences without their lvalue transformations. An array argument then ranks as a pointer argument does, and every compiler applies ICS-33 and ICS-36 to a pointer argument.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) agrees with option 1. An array-to-pointer step is an lvalue transformation, and the comparison of two sequences leaves lvalue transformations out. The rest of each sequence is a pointer conversion from a pointer to the derived class, so the tie-breakers for a base over `void` and for the nearer base apply. The review's one doubt is that the reference states these tie-breakers for pointer sources and says nothing of array arguments.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
