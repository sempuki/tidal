# Q-0415: In a friend declaration with a qualified name, is a name after the declarator searched in the scope that the qualifier names?

**Status:** provisional
**Affects:** NAM-21, `tests/foundations/names/friend-qualified-declarator-lookup.cpp`
**Source:** the gap review `reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`, section NAM-21, from the probe that a clean-room writer ran for it.
**Waits for:** a first review

## Context

Namespace `N` declares `using T = int;` and `void f(T);`. Class `S` declares `friend void N::f(T);`. The name `T` after the declarator names `N::T` only if the search starts in `N`.

| Implementation | Result |
|---|---|
| Clang 22 | rejects: `T` is not found |
| GCC 16 | accepts |
| EDG 7.0 | accepts |

## Evidence

The test in **Affects**.

## Options

1. The name is searched first in `N`, then by ordinary lookup (GCC, EDG, the gap review of 2026-11-05).
2. The name gets only ordinary lookup (Clang).

## Provisional answer

Searched first in `N` (option 1). Two of three families agree, and the gap review of the text phase 10 wrote reads it so.

## Opinions

### Gap review, 2026-11-05 (phase 10 text)

The source-reader gap review of the text phase 10 wrote (`reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`) reads the case as the question's option 1. See its section on NAM-21.
