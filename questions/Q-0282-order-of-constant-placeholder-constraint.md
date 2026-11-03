# Q-0282: Is the constraint of a constrained constant parameter checked in the order of the template parameters?

**Status:** provisional
**Affects:** CNS-1, `tests/templates/constraints/order-constant-placeholder-after-type.cpp`

## Context

A function template has the head `<Trapped T, Wide auto N>`. Checking
`Trapped<int>` instantiates a class whose static assertion fails. The
call gives `N` the value `'a'`, which fails `Wide`. A second template
without constraints can take the call.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: the static assertion fails |
| GCC 16, host | rejects: the static assertion fails |
| GCC 14 | accepts: chooses the second template |
| EDG 7.0 | accepts: chooses the second template |

## Evidence

The test above. When the constant parameter comes first, every
compiler chooses the second template and never instantiates the class
(`order-constant-placeholder.cpp`). Every compiler checks the
constraint of `N` before a requires clause after the head
(`order-constant-placeholder-before-clause.cpp`).

## Options

1. The constraints of the head are checked in parameter order, type
   and constant parameters together (Clang, GCC 16).
2. The constraint of a constant placeholder is checked first, before
   the type constraints of the head (GCC 14, EDG).

## Provisional answer

Yes; Clang and GCC 16 agree, GCC 14 and EDG differ (option 1). The newest Clang and GCC agree, and the order follows the
written order of the head, as CNS-1 does for the other parts.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 check `Trapped<int>` first and reject the program, and EDG 7.0 checks the constant parameter first and chooses the other template, so two families disagree. GCC 14 agrees with EDG. CNS-P1 supports the provisional answer. Step 1a collects the type constraints of type and constant parameters in the order of the template head (CNS-1). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 2, with doubt, which matches EDG 7.0 and GCC 14. Tidal's rules and the matrix keep option 1. CPT-21 makes the type constraint of a constant placeholder a constraint of the template, and CNS-P1 collects the constraints of the head in parameter order. No Tidal rule checks a placeholder constraint while it matches an explicit argument, before the associated constraints. The review's doubt is on the same point, since the constraint also appears among the associated constraints. GCC moved from option 2 in GCC 14 to option 1 in GCC 16, and Clang 22 and GCC 16 now agree on option 1.

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 2, with doubt. With explicit template arguments, the reference first asks whether the resulting template-id is valid, and deduction fails if it is not. The type of a constant parameter with a placeholder comes from an invented variable declaration with the argument as its initializer, and a constrained placeholder there requires the constraint to hold. The constraint fails, so the argument does not match, and the associated constraints are never checked. So the class with the failing assertion is never instantiated, and the second template is chosen, as EDG and GCC 14 do. The doubt is that the reference does not say plainly that a failed placeholder constraint makes the argument not match, instead of making the program not valid. The constraint also appears among the associated constraints, where Clang and GCC 16 check it.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
