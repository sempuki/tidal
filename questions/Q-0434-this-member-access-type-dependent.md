# Q-0434: Is `this->n` type-dependent when the type of `n` does not depend on the parameters?

**Status:** provisional
**Affects:** DEP-2, `tests/templates/dependent-names/this-member-independent-type-bound-early.cpp`, `tests/templates/dependent-names/this-in-local-class-not-type-dependent.cpp`
**Source:** the I6 report (`reports/clean-room/I6/report.md`, I6-5 and I6-20), the round 11 writer probe `reports/clean-room/roadmap-11/writer-probes/i6-call-binding.json`.
**Waits for:** a principle (DEP-P2)

## Context

A member of a class template calls `pick(this->n, 0)`, where the member
`n` has a class type that does not depend on the parameters. A better
`pick` is declared in the namespace of that class after the template.
If `this->n` is type-dependent, the call is dependent, and
argument-dependent lookup at the point of instantiation finds the
better function. If not, the call binds at the definition.

| Form | Clang 18 | Clang 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|---|
| `pick(this->n, 0)` in a member of a class template | the later function | the earlier one | the earlier one | the earlier one |
| `pick(n, 0)` in a member of a class template | the earlier one | the earlier one | the earlier one | the earlier one |
| `pick(this->n, 0)` in a member of a local class of a function template | the later function | the earlier one | the earlier one | the earlier one |

## Evidence

The tests in **Affects**. Each asserts the result in a `static_assert`.

## Options

1. A member access through `this` is type-dependent only when the
   member's type is dependent, or when lookup finds nothing and the
   class has a dependent base. `this` in a local class of a function
   template is not type-dependent (Clang 22, GCC, EDG).
2. `this` is type-dependent in every member of a templated class, so
   every member access through it is type-dependent (Clang 18).

## Provisional answer

A member access through `this` is type-dependent only for a dependent
member type or a dependent base (option 1). Every newest compiler agrees, and Clang 18 alone differs.
DEP-2 states the limit in its bullets on `this` and on a member access.

Kept open on 2026-11-11 under decision 0054 (item RV7). The review supports the answer with three doubts, and only Clang 18 differs.

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with no doubt. Every newest family agrees, and Clang 18 alone differs. No principle gives the answer. DEP-P2 lists `this` in a class template as a type-dependent expression, and it does not say when a member access through `this` is type-dependent. Option 2 also fits the model, so DEP-P2 fails step 3 of the principle test.

## Opinions

### Gap review, 2026-11-11 (phase 14 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 14 wrote on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md`) reads the question: supports with a doubt (RV7-B-11). `this` is type-dependent when the class around it is a dependent type, and the reference's list of dependent types names no local class. So `this` in a member of a local class of a function template is not type-dependent (option 1). The doubts are three. The same reading covers a local class inside a member function of a class template, which DEP-2 does not name. A member access is also type-dependent when its name is dependent for another reason, such as a conversion function to a dependent type. And the reference states the member-access case as a sufficient condition, which reads as a full condition only as the exception to the rule on subexpressions. Every newest compiler reads it as DEP-2 does.

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports, and the doubts of item RV7 are resolved for the tests. A local class of a function template is not among the dependent types, so `this` in its member function is not type-dependent. The general rule that a dependent part makes the whole expression dependent yields to the specific rule for a member access, which applies here. The other two doubts concern cases that the tests do not hold.
