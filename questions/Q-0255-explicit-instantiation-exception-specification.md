# Q-0255: Must the exception specification of an explicit instantiation match the template?

**Status:** provisional
**Affects:** INST-22, `tests/templates/instantiation/explicit-exception-specification-differs.cpp`

## Context

`template <class T> void run(T) {}` is explicitly instantiated as
`template void run(int) noexcept;`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: the exception specification does not match |
| EDG 7.0 | rejects: the exception specification is incompatible |
| GCC 14, 16, host | accepts |

Every compiler accepts a matching `noexcept`, and an explicit
instantiation with no exception specification.

## Evidence

The test above and `explicit-exception-specification-matches.cpp`.
Item 35 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says that a stated
specification must match.

## Options

1. A stated exception specification must match (Clang, EDG).
2. The exception specification of an explicit instantiation is ignored (GCC).

## Provisional answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations and the gap review agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
