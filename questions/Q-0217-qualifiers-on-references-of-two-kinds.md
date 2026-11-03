# Q-0217: Does the qualifier tie-breaker of partial ordering apply to an lvalue reference against an rvalue reference?

**Status:** provisional
**Affects:** ORD-11, `tests/templates/ordering/reference-kind-and-qualifiers.cpp`

## Context

A class has two conversion function templates, `operator T&()` and
`operator const T&&()`. The initialization `int x = Source{};` gives
both the same sequences, and partial ordering compares the results
`T&` and `const T&&`. Deduction succeeds both ways.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | ambiguous |
| GCC 14, 16, host | `operator T&()` |
| EDG 7.0 | `operator T&()` |

ORD-10 makes `const T&&` not at least as specialized, because the other
type is an lvalue reference. Clang also applies ORD-11 to the pair,
which makes `T&` not at least as specialized, because `const T&&` has
more qualifiers. Neither template is then more specialized. GCC and EDG
apply ORD-11 only to two references of one kind.

The other combinations of reference kind and `const` give the same
result on every compiler (`reference-kinds-conversion.cpp`).

## Evidence

`tests/templates/ordering/reference-kind-and-qualifiers.cpp`, with
overrides for GCC and EDG. Item O5 of the gap review of 2026-10-31
(`reports/review/2026-10-31-gap-review-deduction-and-ordering.md`)
describes option 1.

## Options

1. ORD-11 applies to any two references, and the pair gives no order
   (Clang).
2. ORD-11 applies only to two references of one kind, and the lvalue
   reference wins (GCC, EDG).

## Provisional answer

Yes; Clang differs from GCC and EDG (option 1). The gap review describes it, and Clang implements it. With
option 1, ORD-10 and ORD-11 each remove one direction of a pair on
their own, with no condition on each other.

The question stays open under decision 0051 (2026-11-03). Clang 22 finds no order, and GCC 16 and EDG 7.0 choose `operator T&()`, so two families disagree. The provisional answer follows the one family. ORD-P2 supports the provisional answer. Steps 5a and 5b each act inside a pair on their own, and option 2 adds a condition that ties step 5b to the kinds of the two references. Item O5 of the gap review describes option 1.

Kept on 2026-11-03. The source-reader gap review of question cluster 3 agrees with option 1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 3 (`reports/review/2026-11-03-gap-review-question-cluster-3.md`) reads the reference as supporting option 1. The more qualified of the two referenced types is recorded whenever both types are references, of either kind. The check on reference kinds and the check on cv-qualifiers each remove the same direction, so the condition that links them never changes a result, and neither template wins.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
