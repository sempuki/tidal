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

Option 1. GCC diagnoses the case, the inventory and the gap review agree
that the call is not valid, and no compiler rejects equal default
arguments.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
