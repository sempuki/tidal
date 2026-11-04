# Q-0149: What type does a braced list give a reference to an array of unknown bound?

**Status:** settled
**Affects:** REF-14, REF-11, `tests/declarations/initialization/references/braced-list-unknown-bound.cpp`, `tests/declarations/initialization/references/braced-list-unknown-bound-empty.cpp`
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences

## Context

A braced list for a reference to an array makes a temporary array
(REF-14). For a reference to an array of unknown bound, such as
`int (&&values)[]{1, 2, 3}`, the list gives the temporary three
elements. The compilers differ on the type of the reference:

| Compiler | `decltype(values)` | Empty braces `{}` |
|---|---|---|
| Clang 18, Clang 22 | `int (&&)[3]` | rejects ("zero size arrays are an extension") |
| GCC 14, GCC 16 | `int (&&)[]` | accepts |
| EDG 7.0 | `int (&&)[]` | rejects ("an empty initializer is invalid for an array with unspecified bound") |

All compilers read and write the three elements at run time. GCC and
EDG do not index the array in constant evaluation, because the
reference's type has no bound.

## Evidence

- `braced-list-unknown-bound.cpp` checks the declared type.
- `braced-list-unknown-bound-values.cpp` runs on every compiler.
- `braced-list-unknown-bound-empty.cpp` checks empty braces.

## Options

1. The reference keeps its declared type, and the temporary has one
   element for each value. Empty braces make no valid array (GCC and EDG
   for the type, Clang and EDG for empty braces).
2. The reference takes the bound of the list, as a variable of array
   type of unknown bound does (Clang).

## Answer

The reference keeps its declared type, and empty braces are rejected; Clang differs on the type, GCC on empty braces (option 1). A reference to an array of unknown bound can bind to an array
of known bound (REF-1) without a change to the reference's type. Two of
the three compilers keep the declared type. An array of zero elements
is not valid elsewhere, and Q-0136 gives empty braces no sequence to
such a reference.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) supports this answer with no doubt, and REF-P3 gives the same answer: several values for `E[]` make a temporary of one element for each value, which the reference binds to, and `{}` for `E[]` gives none. Clang 22 still differs on the type, and GCC 16 on empty braces. Tidal records each result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. The temporary has the type that a variable of array type with unknown bound would get from the same list, so it has one element for each value. Nothing changes the reference's declared type. A variable of unknown bound may not be initialized from empty braces, so the temporary for empty braces is not valid either.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
