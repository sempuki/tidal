# Q-0146: Which conditions must two functions meet before their constraints decide between them?

**Status:** settled
**Affects:** ORD-13, OVL-30, OVL-41, `tests/overloading/overload-resolution/tie-constraints-different-parameters.cpp`, `tests/overloading/overload-resolution/tie-constraints-default-parameter.cpp`, `tests/overloading/overload-resolution/tie-constraints-ellipsis.cpp`, `tests/templates/ordering/constraints-default-parameter.cpp`, `tests/templates/ordering/constraints-different-heads.cpp`, `tests/templates/ordering/constraints-conversion-results-differ.cpp`

## Context

ORD-13 states when constraints decide between two functions after every
other step ties. OVL-30 (plain functions), OVL-41 (template
specializations), and OVL-19 (the address of an overload set) use the
conditions of ORD-13. Each row below meets every condition except one.
With all conditions met, every compiler lets the constraint decide
(`tie-more-constrained.cpp`, `tie-more-specialized-constraints.cpp`,
`constraints-partial.cpp`, `constraints-conversion.cpp`).

| Case | Clang 22 | Clang 18 | GCC 14, 16 | EDG 7.0 |
|---|---|---|---|---|
| Two templates, `T` against `const T&` (an `int` lvalue gives both the identity) | ambiguous | ambiguous | ambiguous | the constrained template |
| Two templates, one with a defaulted second parameter | ambiguous | ambiguous | ambiguous | the constrained template |
| Two templates, heads `template <class T>` and `template <class T, class U = int>` | ambiguous | ambiguous | ambiguous | the constrained template |
| Two conversion function templates, results `T*` and `T* const`, target `int*` | ambiguous | ambiguous | ambiguous | the constrained template |
| Two plain functions, `f(int)` and `f(int, int = 0)` | ambiguous | ambiguous | ambiguous | the constrained function |
| Two plain functions, `f(int)` and `f(int, ...)` | ambiguous | the constrained function | ambiguous | the constrained function |
| Two plain functions, `f(int)` and `f(const int&)` | ambiguous | ambiguous | ambiguous | the constrained function |

Two templates that differ only in an ellipsis split the compilers in
another way. Q-0293 holds that case.

Q-0162 (plain functions) and Q-0210 (heads and conversion results)
asked the same question for other conditions. Both are duplicates of
this question.

## Evidence

The tests in **Affects**, each with an override for EDG, and for Clang
18 on the ellipsis. The columns "template const lref / constrained
template" and "template forwarding / constrained template" of the
templates grid of the overload resolution page. Gap G-15 of the
overload resolution inventory. The sanitized Clang source
(clang-sema-deduction) compares constraints only when the template
parameter lists are equivalent, the function parameter types are the
same, and, for conversion functions, the result types are the same.
Items M2 to M4 of the deduction inventory describe these conditions.
Item O-2 of the gap review of overload resolution
(`reports/review/2026-10-31-gap-review-overload-resolution.md`)
describes the two plain-function calls as ambiguous.

## Options

1. Constraints decide only between functions that meet every condition
   of ORD-13: the same parameter list, including defaulted parameters,
   the same parameter types, the same conversion result, equivalent
   template heads, and, for plain functions, the same ellipsis (Clang 22,
   GCC).
2. Constraints decide whenever the other steps tie (EDG).

## Answer

The conditions of ORD-13: same parameter lists, heads, and conversion results; Clang 22 and GCC agree, EDG differs (option 1). Clang 22 and GCC agree on every row, and the sanitized Clang
source and the gap review describe the same conditions.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 find every row of the table ambiguous, and EDG 7.0 lets the constraint decide in every row, so two families disagree. Probes of `T` against `const T&`, `f(int)` against `f(const int&)`, and `f(int)` against `f(int, int = 0)` give the same split. ORD-P2 and CNS-P4 support the provisional answer. Step 0 of CNS-P4 requires the function parameters of two templates to correspond before their constraints are compared, and ORD-P2 lets constraints decide only when the heads and the parameter lists are the same. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) supports this answer with no doubt, and ORD-P2 and CNS-P4 give the same answer: constraints decide only when the heads and the parameter lists correspond. EDG 7.0 still differs. Tidal records EDG's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. For functions that are not templates, constraints decide only when the parameter type lists are the same, and a default argument or an ellipsis makes the lists differ. For templates, a different length of either list, a pair of parameters of different types, or different conversion results stops the comparison. Each row of the table falls under one of these cases.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-10-31

The source-reader gap review of overload resolution (`reports/review/2026-10-31-gap-review-overload-resolution.md`) names the question as the place for the constraint ordering of templates with equal parameters, and asks for no probe beyond the existing tests. It gives no reading of its own.

### Inventory, 2026-10-31

The source-reader inventory of concepts and constraints (`reports/review/2026-10-31-inventory-concepts-and-constraints.md`) names the question in its list of compiler differences that Tidal already records, and gives no reading.

### Gap review of cluster 3, 2026-11-03

The source-reader gap review of question cluster 3 (`reports/review/2026-11-03-gap-review-question-cluster-3.md`) reads the constraint tie-breaker for two plain functions as needing the same list of parameter types, a list that records an ellipsis. The ellipsis then counts, which the review says fits this question. The review reads the condition for two templates differently, in line with Clang 22.
