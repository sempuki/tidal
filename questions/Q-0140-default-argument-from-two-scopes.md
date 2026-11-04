# Q-0140: Is a call valid when it uses a default argument that two scopes give?

**Status:** provisional
**Affects:** OVL-38, `tests/foundations/overload-resolution/default-from-two-scopes.cpp`, `tests/foundations/overload-resolution/default-from-two-scopes-same.cpp`

## Context

One `extern "C"` function is declared in two namespaces. Each
declaration gives a default argument for the same parameter.
Using-declarations or using-directives bring both into one scope, and a
call relies on the default argument.

| Compiler | Different default arguments | Equal default arguments |
|---|---|---|
| GCC 14, GCC 16 | rejects | accepts |
| Clang 18, Clang 22 | accepts | accepts |
| EDG 7.0 | accepts | accepts |

The results are the same with using-declarations and with
using-directives. A call that passes the argument is accepted by all
compilers. A redeclaration in a block scope that repeats the default
argument is also accepted by all compilers.

## Evidence

`tests/foundations/overload-resolution/default-from-two-scopes.cpp` and
`default-from-two-scopes-same.cpp`. GCC reports "default argument
mismatch in overload resolution". Item AF-4 of the overload resolution
inventory describes the call as not valid whenever two scopes give the
default argument, and gap G-8 says that compilers may not diagnose it.
Item E-4 of the gap review of overload resolution
(`reports/review/2026-10-31-gap-review-overload-resolution.md`) also
describes the case of equal arguments as not valid. No compiler rejects
that case.

## Options

1. The call is not valid if the two default arguments differ (GCC).
2. The call is not valid whenever two scopes give the default argument
   (the inventory and the gap review). No compiler rejects the case of
   equal arguments.
3. The call is valid (Clang, EDG).

## Provisional answer

No, if the defaults differ; GCC agrees, Clang and EDG differ (option 1). GCC diagnoses the case, the inventory and the gap review agree
that the call is not valid, and no compiler rejects equal default
arguments.

Kept on 2026-11-03. The review reads option 2, and every compiler accepts the call with equal default arguments, which option 2 rejects. Tidal keeps option 1, which GCC follows, until the maintainer decides. The form with equal values is in row 4 of `documentation/decisions/pending.md`. The question stays open under decision 0054, because the review reads against the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) reads option 2. The call is not valid whenever two declarations in different scopes each supply the default argument that the call needs, with no comparison of the two values. A diagnostic is required. GCC rejects only different values, and no compiler in the table rejects equal values.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-10-31

The source-reader gap review of overload resolution (`reports/review/2026-10-31-gap-review-overload-resolution.md`) reads default arguments from two scopes as making the call ill-formed even when the two values are equal. Only GCC matches that reading. The reading goes against the test for equal values, which expects acceptance, as the later review of the open questions on overloading also finds.
