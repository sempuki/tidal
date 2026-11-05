# Q-0349: In constant evaluation, can a member of `std::allocator<T>` convert the result of `::operator new` to a pointer to a type other than `T`?

**Status:** provisional
**Affects:** CE-4, LSUP-11, `tests/foundations/constant-evaluation/allocator-result-other-type.cpp`
**Waits for:** a review (silent)

## Context

LSUP-11 lets a program declare `std::allocator<T>` with members that
call `::operator new` and `::operator delete` by name, and lets constant
evaluation call them there. CE-4 allows the conversion of the result to
`T*` inside such a member. The tests of LSUP-11 write
`static_cast<T*>(::operator new(n * sizeof(T)))` in `allocate`, and
every compiler accepts them.

When `allocate` of `std::allocator<int>` converts the result to `long*`
instead, the compilers split:

| Compiler | Result |
|---|---|
| Clang 22 | rejects: a cast from `void*` is not allowed in a constant expression |
| GCC 16 | accepts |
| EDG 7.0 | accepts |

Every compiler accepts the conversion to `T*`, also through a local
variable of type `void*`. Every compiler rejects the call of
`::operator new` in a member that returns the `void*` with no
conversion.

## Evidence

`tests/foundations/constant-evaluation/allocator-result-other-type.cpp`,
with overrides for GCC and EDG. Probes of the clean-room writer with
Clang 22, GCC 16, and EDG 7.0 on the self-declared allocator of
`tests/foundations/language-support/allocator-self-declared-constant.cpp`.

## Options

1. Only the conversion to `T*` is valid. The storage is for an array of
   `T` (STG-28), and the conversion gives a pointer to its first
   element (Clang).
2. A conversion to any object type is valid inside the member (GCC,
   EDG).

## Provisional answer

No; only the conversion to `T*` is valid (option 1). The exception in
CE-4 exists so that `allocate` can return a pointer to the array of `T`
that STG-28 describes, and a pointer of another type does not point to
an element of that array.

Kept open on 2026-11-03 under decision 0054, because the review is silent.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as silent between the options. During constant evaluation the reference does not look at the bodies of the members of the library's allocator, so a body that the program writes has no standing. The library's `allocate` returns a pointer to the first element of an array of the element type, which fits option 1.
