# Q-0421: In constant evaluation, can a program assign to an element of storage from `std::allocator` before `std::construct_at` builds it?

**Status:** provisional
**Affects:** LSUP-11, CE-9, `tests/foundations/storage/allocation-assign-before-construction-int-element.cpp`, `tests/foundations/storage/allocation-assign-before-construction-pod-member.cpp`
**Source:** Q-0323, which held this form until decision 0059 item QR moved it here, with the probe grid `reports/clean-room/roadmap-9/writer-probes/q-0323-assign-before-construction.json` that a clean-room writer ran for it.
**Waits for:** newest families that agree; a first review

## Context

A `constexpr` function gets storage for two `int` elements from a
`std::allocator` that the program declares (LSUP-11). It assigns to an
element before `std::construct_at` builds one there, reads the element
back, and frees the storage. At run time, the allocation creates `int`
objects implicitly (STG-8). During constant evaluation, the compilers
split:

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| assign element 0, then read it | rejects | accepts | accepts |
| assign element 1, then read it | rejects | accepts | accepts |
| assign through `*p` for one element | rejects | accepts | accepts |
| assign a member of a simple class element | rejects | accepts | accepts |
| `std::construct_at` element 0, then read it | accepts | accepts | accepts |

Clang notes an assignment to an object outside its lifetime. Every
compiler rejects a read of an element that nothing has written or built
(LSUP-11).

## Evidence

The two tests in **Affects**. Each expects Clang's rejection and
records GCC and EDG as overrides.

## Options

1. Constant evaluation creates no implicit objects in the storage, so
   the assignment writes outside any lifetime and is rejected (Clang).
2. The assignment begins the lifetime of an `int` element, as an
   implicitly created object would at run time, and constant evaluation
   accepts it (GCC, EDG).

## Provisional answer

The assignment is rejected (option 1). The gap review that read Q-0323
on 2026-11-03 reads the reference as Clang does for this form, and the
tests build each element with `std::construct_at` before they use it.

## Opinions
