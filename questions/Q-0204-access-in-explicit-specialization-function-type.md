# Q-0204: Is a private type in the function type of an explicit specialization access-checked?

**Status:** provisional
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

## Provisional answer

No; Clang and EDG accept, GCC rejects (option 1). Clang and EDG agree, and the answer treats function and class
templates alike.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
