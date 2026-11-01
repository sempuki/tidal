# Q-0149: What type does a braced list give a reference to an array of unknown bound?

**Status:** provisional
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

## Provisional answer

Option 1. A reference to an array of unknown bound can bind to an array
of known bound (REF-1) without a change to the reference's type. Two of
the three compilers keep the declared type. An array of zero elements
is not valid elsewhere, and Q-0136 gives empty braces no sequence to
such a reference.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
