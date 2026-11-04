# Q-0269: Does a function with a placeholder parameter redeclare the function template with the same invented head?

**Status:** provisional
**Affects:** CNS-27, CNS-40, `tests/templates/constraints/redeclare-placeholder.cpp`, `tests/templates/constraints/redeclare-placeholder-unconstrained.cpp`, the spellings grid

## Context

`template <A T> int f(T);` and then `int f(A auto) { return 1; }`.
The same holds without the constraint.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts: the second declaration defines the first |
| GCC 14, 16, host | rejects: the call is ambiguous |
| EDG 7.0 | rejects: the call is ambiguous |

## Evidence

The test above, and the columns `A T / A auto` and `AB T / AB auto` of the spellings grid, where Clang reports a redefinition.

In those two grid columns every compiler rejects, for different reasons. Clang reports a redefinition, and GCC and EDG find the call ambiguous.

## Options

1. One template (Clang). DED-34 makes the placeholder an
   invented type parameter, so the two heads are equivalent by
   TPL-20.
2. Two templates (GCC, EDG). A program that declares both is not
   valid, and no diagnostic is required.

## Provisional answer

No, two templates, and no diagnostic is required; GCC and EDG diagnose the call as ambiguous, Clang accepts (option 2). A placeholder parameter makes a function template of its own (DED-P1), and constraints in other places declare another template (CNS-P5). CNS-27 and CNS-40 state the answer.

Changed on 2026-11-03 to option 2, with no diagnostic required. The review reads option 2, GCC 16 and EDG 7.0 follow it, and two principles give the same answer: DED-P1 says that a placeholder parameter makes a function template of its own, and CNS-P5 says that a declaration with constraints in other places declares another template. CNS-27 now says that a placeholder parameter does not match a template parameter of another declaration. The new `no-diagnostic` rule CNS-40 covers a program with both templates. `redeclare-placeholder.cpp` expects a rejection with an override for Clang, and the new test `redeclare-placeholder-unconstrained.cpp` covers the form without constraints.

Kept open on 2026-11-03 under decision 0054, because CNS-27 changed and CNS-40 is new on the same day, and a question settles only after a reader or a review has used the changed rule (decision 0055). The review also reads with doubt.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) reads option 2, with doubt, and the program needs no diagnostic. Each placeholder parameter becomes an invented type parameter, and an example treats the declaration with placeholders and the template written out by hand as two declarations that accept the same arguments and are distinct. A program that depends on two such constructs being the same is not valid, with no diagnostic required, and so is a program with two such function templates of one name and signature. So GCC's and EDG's ambiguity and Clang's acceptance all conform. The doubt is that only an example states that the two forms differ. The unconstrained form follows the same reasoning.

### Gap review, 2026-11-04

The source-reader gap review of the rules that phase 7 added (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) agrees with CNS-40 and reads its condition as narrower than the reference. The reference makes a program not valid, with no diagnostic required, when two function templates that are not the same template have one name and one signature, would declare one entity, and accept and are satisfied by the same template argument lists. So two templates whose constraints differ in form and have one meaning also fall under the rule. CNS-40 now states this condition. Editorial notes in the draft ask what two of its terms mean, so the wording is not settled text.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-10-31

The source-reader gap review of concepts and constraints (`reports/review/2026-10-31-gap-review-concepts-and-constraints.md`) raises the case as an open point. Two constant template parameters compare equal when their types match apart from a type constraint on the placeholder, and the comparison of template heads does not settle whether a constrained and an unconstrained placeholder parameter declare the same template. The review expects compilers to reject, perhaps for different reasons, and gives no answer to the question.
