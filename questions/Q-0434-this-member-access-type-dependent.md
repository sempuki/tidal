# Q-0434: Is `this->n` type-dependent when the type of `n` does not depend on the parameters?

**Status:** provisional
**Affects:** DEP-2, `tests/templates/dependent-names/this-member-independent-type-bound-early.cpp`, `tests/templates/dependent-names/this-in-local-class-not-type-dependent.cpp`
**Source:** the I6 report (`reports/clean-room/I6/report.md`, I6-5 and I6-20), the round 11 writer probe `reports/clean-room/roadmap-11/writer-probes/i6-call-binding.json`.
**Waits for:** a first review

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
