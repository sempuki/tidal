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

## Options

1. One template (Clang). DED-34 makes the placeholder an
   invented type parameter, so the two heads are equivalent by
   TPL-20.
2. Two templates (GCC, EDG). A program that declares both is not
   valid, and no diagnostic is required.

## Provisional answer

No, two templates, and no diagnostic is required; GCC and EDG diagnose the call as ambiguous, Clang accepts (option 2). A placeholder parameter makes a function template of its own (DED-P1), and constraints in other places declare another template (CNS-P5). CNS-27 and CNS-40 state the answer.

Changed on 2026-11-03 to option 2, with no diagnostic required. The review reads option 2, GCC 16 and EDG 7.0 follow it, and two principles give the same answer: DED-P1 says that a placeholder parameter makes a function template of its own, and CNS-P5 says that a declaration with constraints in other places declares another template. CNS-27 now says that a placeholder parameter does not match a template parameter of another declaration. The new `no-diagnostic` rule CNS-40 covers a program with both templates. `redeclare-placeholder.cpp` expects a rejection with an override for Clang, and the new test `redeclare-placeholder-unconstrained.cpp` covers the form without constraints. The question stays open.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) reads option 2, with doubt, and the program needs no diagnostic. Each placeholder parameter becomes an invented type parameter, and an example treats the declaration with placeholders and the template written out by hand as two declarations that accept the same arguments and are distinct. A program that depends on two such constructs being the same is not valid, with no diagnostic required, and so is a program with two such function templates of one name and signature. So GCC's and EDG's ambiguity and Clang's acceptance all conform. The doubt is that only an example states that the two forms differ. The unconstrained form follows the same reasoning.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
