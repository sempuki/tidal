# Q-0204: Is a private type in the function type of an explicit specialization access-checked?

**Status:** settled
**Affects:** TSP-29

## Context

`template <> int size(Owner::Hidden)`, where `Hidden` is a private
member of `Owner`, at namespace scope.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | rejects: `Owner::Hidden` is private |
| EDG 7.0 | accepts |

All compilers accept a private type in the argument list of an explicit
or partial specialization of a class template, and all reject one in
the member list.

## Evidence

`tests/templates/specialization/explicit-function-private-argument.cpp`,
with an override for GCC, `explicit-private-argument.cpp`, and
`partial-private-argument.cpp`.

## Options

1. The declaration of an explicit specialization is not access-checked,
   for function templates as for class templates (Clang, EDG).
2. The function type is access-checked (GCC).

## Answer

No; Clang and EDG accept, GCC rejects (option 1). Clang and EDG agree, and the answer treats function and class
templates alike.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and TSP-P4 gives the same answer: the implementation does not check access to the names in the declaration of a specialization. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. The usual access checks are off for names in the declaration of an explicit specialization or explicit instantiation, except in a function body, a default argument, a base list, a member list, an enumerator list, and an initializer. Parameter types are not among the exceptions. GCC's rejection is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
