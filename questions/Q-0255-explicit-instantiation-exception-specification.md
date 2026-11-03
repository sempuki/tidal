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

The question stays open under decision 0051 (2026-11-03). GCC 16 accepts the `noexcept` that differs, and Clang 22 and EDG 7.0 reject it, so two families disagree. Probes of 2026-11-03 give the same split for `noexcept(false)` on a template declared `noexcept`, for a class template's member function, and for an explicit instantiation declaration. INST-P3 supports the provisional answer through INST-22. An explicit instantiation names a specialization whose type the template already gives, and the exception specification is part of that type.

Kept on 2026-11-03. The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) agrees with option 1. INST-22 keeps the check of a stated exception specification.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) reads the reference as supporting option 1. An explicit instantiation can leave out the exception specification. A stated one must be the same as on every other declaration of the function. A diagnostic is required when the mismatch is in one translation unit, so GCC's acceptance is a defect. The rule covers both kinds of explicit instantiation and members of class templates.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
