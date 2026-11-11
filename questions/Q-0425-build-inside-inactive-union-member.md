# Q-0425: Can a program build an object inside a union member that is not active, as an element of an array member or a member of a member union?

**Status:** provisional
**Affects:** UNI-14, UNI-6, CE-6, STG-10, `tests/classes/unions/construct-in-member-of-inactive-member-union-constant.cpp`, `tests/classes/unions/construct-in-member-of-inactive-member-union-run.cpp`, `tests/classes/unions/construct-at-array-member-element-constant.cpp`, `tests/classes/unions/array-member-element-address-no-active-member.cpp`, `tests/classes/unions/array-member-element-steps-no-active-member-constant.cpp`
**Source:** W20-2 (`reports/clean-room/W20/report.md`) with the writer probe `reports/clean-room/roadmap-10/writer-probes/w20-2-union-array-member.json`, and the round 10 programmer report (`reports/clean-room/roadmap-10/programmer.md`, sections 1.4, 1.5, and 6), reduction `construct_in_member_of_inactive_union_member.cpp`.
**Waits for:** a first review; newest families that agree

## Context

UNI-14 makes a union member active when placement `new` or
`std::construct_at` creates an object of the member's type at the
member's address. No rule covers an object of another type inside the
member: an element of an array member, or a member of a member that is
itself a union. Two forms reach the case.

A union `Outer` has a member `inner` of union type, and no member of
`Outer` is active. In constant evaluation, `std::construct_at` builds a
`double` at the address of `inner.d`:

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 | Tidal front end |
|---|---|---|---|---|
| the constant | rejects | rejects | accepts | rejects (UNI-6) |
| the same at run time | runs, also with the sanitizers | runs, also with the sanitizers | accepts | stops (UNI-6) |

A union `Store` has a member `cells` of type `int[3]`, and no member is
active:

| Form | Clang 18 and 22 | GCC 14 | GCC 16 | EDG 7.0 | Tidal front end |
|---|---|---|---|---|---|
| `&s.cells[0]` and `&s.cells[1]` at run time, with no read | runs | runs | runs | accepts | stops ("unsupported") |
| `std::construct_at` of an element in constant evaluation | rejects | rejects | accepts | accepts | not probed |

Clang's note for the constant forms says that the construction of a
subobject of a member of a union with no active member is not allowed
in a constant expression.

## Evidence

The tests in **Affects**:

- The constant test with a member union expects GCC's and Clang's
  rejection and records EDG as an override.
- The run test with a member union expects the matrix compilers to
  build the program, and the front end's interpreter to stop (UNI-6).
- The constant test of `std::construct_at` on an element of an array
  member expects the rejection of Clang and GCC 14, and records GCC 16
  and EDG as overrides.
- The run test forms pointers to two elements of an array member that
  is not active.
- The constant test steps between such pointers, subtracts them, and
  compares them. Every compiler accepts it, also with another member
  active (`reports/clean-room/roadmap-10/writer-probes/f3b-w20-2-step-in-inactive-array.json`).

The writer probe `reports/clean-room/roadmap-10/writer-probes/w20-2-union-array-member.json`
gives the forms of the array member.

## Options

1. Building an object inside a member that is not active does not make
   the member active. Constant evaluation rejects the construction. At
   run time, a pointer to an element or to a member of such a member
   can be formed (STG-10), and a program that only forms and compares
   such pointers runs.
2. Building the object makes the enclosing member active, as UNI-14
   does for an object of the member's type. Constant evaluation accepts
   the construction (EDG, and GCC 16 for an array element).

## Provisional answer

No; the construction does not make the member active, and constant
evaluation rejects it (option 1). Clang and GCC reject the form with a
member union, and Clang and GCC 14 reject it with an array member. A
program that only forms pointers into the member runs on every
compiler.

## Opinions
