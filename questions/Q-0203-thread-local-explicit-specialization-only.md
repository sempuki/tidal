# Q-0203: Can an explicit specialization of a variable template be thread_local when the template is not?

**Status:** provisional
**Affects:** TSP-24, TSP-25

## Context

`template <class T> int counter = 0;` and
`template <> thread_local int counter<char> = 1;`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts and runs |
| GCC 14, 16, host | accepts and runs |
| EDG 7.0 | rejects: a thread-local declaration follows one that is not |

All compilers accept the program when the template is `thread_local`
too, and all reject other storage class specifiers on an explicit
specialization.

## Evidence

`tests/templates/specialization/explicit-thread-local-only.cpp`, with
an override for EDG, and `explicit-thread-local.cpp`.

## Options

1. The explicit specialization takes its properties from its own
   declaration, so the difference is valid (Clang, GCC).
2. The explicit specialization must agree with the template on
   `thread_local` (EDG).

## Provisional answer

Yes; Clang and GCC accept, EDG rejects (option 1). Clang and GCC agree, and TSP-25 gives an explicit
specialization its `inline` and `constexpr` from its own declaration.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
