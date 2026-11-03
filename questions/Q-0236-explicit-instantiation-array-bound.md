# Q-0236: Can an explicit instantiation of a variable leave out the array bound?

**Status:** settled
**Affects:** INST-22, `tests/templates/instantiation/explicit-array-bound-left-out.cpp`

## Context

`template <class T> int table[] = {1, 2, 3};` and `template int
table<int>[];`. The specialization has the type `int[3]`.

| Compiler | Result |
|---|---|
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |
| Clang 18, 22, host | rejects: `int[]` does not match `int[3]` |

Every compiler accepts `template int table<int>[3];`.

## Evidence

The test above and `explicit-deduced-arguments.cpp`. Inventory item
G17 says that an array bound can be left out.

## Options

1. The bound can be left out (GCC, EDG).
2. The type must match exactly (Clang).

## Answer

Yes; GCC and EDG agree, Clang differs (option 1). Two of three implementations and the inventory agree.

The question stays open under decision 0051 (2026-11-03). Clang 22 rejects the explicit instantiation definition without the bound, and GCC 16 and EDG 7.0 accept it, so two families disagree. Probes of 2026-11-03 found that every compiler, Clang 22 too, accepts the explicit instantiation declaration `extern template int table<int>[];`. Clang 22 also rejects the definition without the bound when the template is declared `int table[3]`, and GCC 16 and EDG 7.0 accept it. A bound that differs from the specialization's is Q-0346. INST-P3 supports the provisional answer through INST-22. The template arguments name the specialization, and the type without a bound leaves no doubt about which specialization the program names.

Kept on 2026-11-03. The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) agrees with option 1. INST-22 keeps the allowance for a missing bound.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) supports this answer with no doubt, and INST-P3 gives the same answer: the template arguments name the specialization, and a type without the bound names it too. Clang 22 still differs. Tidal records Clang's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for Clang.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) reads the reference as supporting option 1. The explicit instantiation and the declaration made from the template declare one entity and must have the same type. Declarations of one array object may differ in whether the first bound is present. The rule applies to both kinds of explicit instantiation, whether or not the template states the bound, and a note of the reference accepts the form.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
