# Q-0216: Does a parameter with a default argument count against a trailing pack in partial ordering?

**Status:** provisional
**Affects:** ORD-12, `tests/templates/ordering/pack-default-parameter.cpp`

## Context

`f(1)` has two candidates:

- `template <class T> void f(T, T = T())`,
- `template <class T, class... U> void f(T, U...)`.

Only the first parameter has a call argument, and each template is at
least as specialized as the other for that parameter. ORD-12 prefers a
template with no trailing pack. The first template has a parameter with
a default argument at the position of the pack.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | ambiguous |
| GCC 14, 16, host | the first template |
| EDG 7.0 | the first template |

Clang counts the defaulted parameter as a parameter against the pack,
so ORD-12 does not apply. GCC and EDG count only the parameters with
call arguments.

## Evidence

`tests/templates/ordering/pack-default-parameter.cpp`, with overrides
for GCC and EDG. Item O6 of the gap review of 2026-10-31
(`reports/review/2026-10-31-gap-review-deduction-and-ordering.md`)
describes option 1.

## Options

1. A parameter with a default argument counts, and the call is
   ambiguous (Clang).
2. Only parameters with call arguments count, and the template with no
   pack wins (GCC, EDG).

## Provisional answer

Yes; Clang differs from GCC and EDG (option 1). The gap review describes it, and Clang implements it. The
same view of the whole parameter list applies to the constraint
tie-breaker, where Clang and GCC agree (ORD-13, Q-0146).

The question stays open under decision 0051 (2026-11-03). Clang 22 finds the call ambiguous, and GCC 16 and EDG 7.0 choose the template with no pack, so two families disagree. The provisional answer follows the one family. Item O6 of the gap review gives option 1, and ORD-13 counts a parameter with a default argument in the length of a parameter list (Q-0146). ORD-5 fits option 2, because in a call a parameter that uses its default argument takes no part. The gap review of this cluster should weigh a change to option 2.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
