# Q-0188: When two references bind directly to one array, is the reference to the known bound better?

**Status:** provisional
**Affects:** ICS-46, `tests/foundations/conversion-sequences/array-known-bound-direct.cpp`, `tests/foundations/conversion-sequences/list-string-literal-bound.cpp`, `tests/foundations/conversion-sequences/array-known-bound-direct-cv.cpp`
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences

## Context

An array lvalue `int row[2]` is passed to `f(int (&)[2])` and
`f(int (&)[])`. Both references bind directly to `row`, with the
identity. No rule of the argument comparison tells the two apart.

| Compiler | `f(row)` | `f({row})` | an xvalue, with `&&` references | braced string literal |
|---|---|---|---|---|
| Clang 18, Clang 22 | tie | tie | tie | tie |
| GCC 14, GCC 16 | known bound | known bound | known bound | known bound |
| EDG 7.0 | known bound | known bound | known bound | known bound |

ICS-24 prefers the known bound for two list sequences that make
temporaries. Q-0153 covers pointers to arrays, where Clang prefers the
known bound and GCC and EDG find a tie.

## Evidence

The two tests in **Affects**, and probes with `const int` elements, which
give the same split.

## Options

1. The known bound is better for two direct bindings (GCC, EDG).
2. The two bindings tie (Clang).

## Provisional answer

Yes; GCC and EDG agree, Clang finds a tie (option 1). ICS-46 prefers the reference whose referenced type converts to the other's by a qualification adjustment, and dropping a bound is such an adjustment.

Kept on 2026-11-03. The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) reads the C++23 text as giving a tie for plain direct bindings (option 2), and the known bound for the braced string literal (option 1). The review reports that a later change to the language adds a preference of this kind for references, and that GCC 16 and EDG 7.0 may follow it. A probe of 2026-11-03 tests that report:

| Probe | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| `int* const&` against `const int* const&`, for an `int*` lvalue | `int* const&` | `int* const&` | `int* const&` |
| `const int* const&` against `const volatile int* const&` | the first | the first | the first |
| `int (&)[2]` against `const int (&)[]`, for an `int[2]` lvalue | tie | `int (&)[2]` | `int (&)[2]` |
| both references bind directly in a constant evaluation | yes | yes | no result |

Every compiler prefers the reference with fewer cv-qualifiers below the top level. The review's reading of the last reference step, which needs the same referenced type apart from top-level cv-qualifiers, gives a tie there too. So every compiler follows the later change for pointers, and GCC and EDG also apply it to the bound. ICS-46 states that preference for both cases, and the known bound then follows from Q-0153, where ICS-31 lets a qualification adjustment drop a bound. Pointers and references to arrays then give one answer. The answer depends on the later change, as Q-0181 does, and Q-0007 holds the maintainer's choice of language version. New tests: `reference-direct-fewer-qualifiers.cpp`, which every compiler accepts, and `array-known-bound-direct-cv.cpp`.

The question stays open under decision 0051 (2026-11-03). Clang 22 finds a tie, and GCC 16 and EDG 7.0 prefer the known bound, so two families disagree. A probe of 2026-11-03 gives the same split for a braced list and for an xvalue.

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) reads the C++23 text as giving option 2 for plain lvalue and xvalue arguments, because no step compares two direct bindings to array types with different bounds. For the braced string literal, the list rule prefers the known bound, which is option 1. The review is unclear on a braced list that holds the array lvalue. The review reports a later change to reference binding between similar types that gives a preference of this kind, and asks the writer to confirm it by probe.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
