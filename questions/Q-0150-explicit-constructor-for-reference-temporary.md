# Q-0150: Can a braced list for a reference use an explicit constructor to make the temporary?

**Status:** settled
**Affects:** REF-11, REF-14, `tests/declarations/initialization/references/braced-list-explicit-constructor.cpp`
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences

## Context

A braced list for a reference to a class makes a temporary of the class
(REF-14). The compilers differ when the only constructor is `explicit`
and the list has braces with no `=`, as in `const Meters& length{3}`:

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | accepts |
| GCC 14, GCC 16 | rejects ("would use explicit constructor") |
| EDG 7.0 | rejects ("copy-list-initialization cannot use a constructor marked `explicit`") |

With `=`, as in `const Meters& length = {3}`, all compilers reject the
list.

## Evidence

`braced-list-explicit-constructor.cpp`.

## Options

1. The temporary is copy-initialized from the list, so an `explicit`
   constructor is not valid (GCC, EDG).
2. The temporary is initialized as the braces of the reference are, so
   braces without `=` can use an `explicit` constructor (Clang).

## Answer

No; GCC and EDG agree, Clang differs (option 1). Two of the three compilers reject the list, and the result
then does not depend on the presence of `=`.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) supports this answer with no doubt, and REF-P3 gives the same answer: the temporary is copy-initialized from the list, so it cannot use an `explicit` constructor. Clang 22 still differs. Tidal records Clang 22's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for Clang 22.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. The temporary for a braced list bound to a reference is made by copy list initialization, with or without `=`. Clang does not follow this reading.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
