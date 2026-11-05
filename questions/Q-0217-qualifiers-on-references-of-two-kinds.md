# Q-0217: Does the qualifier tie-breaker of partial ordering apply to an lvalue reference against an rvalue reference?

**Status:** provisional
**Affects:** ORD-11, `tests/templates/ordering/reference-kind-and-qualifiers.cpp`
**Waits for:** a use of ORD-P2; newest families that agree

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

Kept open on 2026-11-03 under decision 0054, because two of the newest families differ from the answer in at least one form, and the rule records one differing family.

Kept open on 2026-11-04 under decision 0055, because ORD-P2 changed in this commit: steps 1, 3, and 6 now give the type of an invented value, the pairing of packs, and the agreement of the values from all pairs. Steps 5a and 5b did not change. The question waits for the next use of ORD-P2.

Used on 2026-11-04 by the principle-use batch PU2 (`reports/clean-room/PU2/`), under decision 0056. A clean-room reader applied ORD-P1 and ORD-P2, without the rule text, to the program of `reference-kind-and-qualifiers.cpp`. The answer names both principles and says that the outcome depends on the compiler: Clang finds no order and rejects, and GCC and EDG choose `operator T&()`. The test allows both outcomes, and the batch score counts the use as support. The answer takes the split from the example table of ORD-P1 and does not say which option the model gives.

Kept open on 2026-11-04 under decision 0054, because two newest families, GCC 16 and EDG 7.0, differ from the answer (`documentation/decisions/pending.md`, row 1). The answer in batch PU2 also does not choose between the options.

Kept open on 2026-11-04 under decision 0055, because ORD-P2 changed in this commit: steps 6c and 6d now say how an ellipsis counts and that 6d compares the declared types. Steps 5a and 5b did not change. The question waits for the next use of ORD-P2.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 3 (`reports/review/2026-11-03-gap-review-question-cluster-3.md`) reads the reference as supporting option 1. The more qualified of the two referenced types is recorded whenever both types are references, of either kind. The check on reference kinds and the check on cv-qualifiers each remove the same direction, so the condition that links them never changes a result, and neither template wins.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
