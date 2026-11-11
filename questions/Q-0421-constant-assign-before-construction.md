# Q-0421: In constant evaluation, can a program assign to an element of storage from `std::allocator` before `std::construct_at` builds it?

**Status:** provisional
**Affects:** LSUP-11, CE-9, `tests/foundations/storage/allocation-assign-before-construction-int-element.cpp`, `tests/foundations/storage/allocation-assign-before-construction-pod-member.cpp`
**Source:** Q-0323, which held this form until decision 0059 item QR moved it here, with the probe grid `reports/clean-room/roadmap-9/writer-probes/q-0323-assign-before-construction.json` that a clean-room writer ran for it.
**Waits for:** newest families that agree; a review (doubt)

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

Kept open on 2026-11-10 under decision 0054. The review's doubt is that the tests declare their own `std::allocator`. With `<memory>`'s own `std::allocator` and `std::construct_at`, Clang 22 rejects the assignment to an `int` element and to a member of a class element, and GCC 16 accepts both (`reports/clean-room/roadmap-10/writer-probes/q-0421-library-allocator.cpp`, host compilers only, because EDG has no `<memory>`). The split is the same as with the declared allocator. The probe grid `reports/clean-room/roadmap-10/writer-probes/g156-assign-before-construction-types.json` shows the same split for `double`, a pointer, an enumeration, `char`, a whole element of a simple class, and a class with its own assignment operator. Whether the text exempts a declared template stays for the review.

Kept open on 2026-11-05 under decision 0054, because two newest families differ from the answer and the review reads it with a doubt (item RV5).

## Opinions

### Gap review, 2026-11-05 (phase 12 text, classes, declarations, foundations, statements, and exceptions)

The source-reader gap review of the text phase 12 wrote on classes, declarations, foundations, statements, and exceptions (`reports/review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md`) reads the question: supports with a doubt (RV5-A-17). The allocate member starts the life of the array and of no element, so an assignment to an element changes an object outside its lifetime, and constant evaluation stops. The doubt is that the reference's exemption for the allocator members names the library's own template, and the tests declare their own.
