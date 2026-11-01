# Q-0245: Can the keyword template come before a destructor name?

**Status:** provisional
**Affects:** DEP-34, `tests/templates/dependent-names/template-keyword-destructor.cpp`

## Context

`p->template ~T()` in a function template, where `p` has the type
`T*`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: the keyword is not permitted in a destructor name |
| GCC 14, 16, host | rejects, with the same message |
| EDG 7.0 | accepts |

## Evidence

The test above. Item 2 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says
that the keyword must not come before a destructor name. EDG also
accepts `p->T::template ~T()`, and Clang and GCC reject it.

## Options

1. The keyword is not valid before a destructor name (Clang, GCC).
2. The keyword is accepted and has no effect (EDG).

## Provisional answer

No; Clang and GCC agree, EDG differs (option 1). Two of three implementations and the gap review agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
