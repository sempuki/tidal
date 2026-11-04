# Q-0323: Does a `T*` from an allocation function's result point to element 0 of an array of `T` whose elements are not built yet?

**Status:** provisional
**Affects:** STG-8, STG-10, STG-28, LSUP-11, `tests/foundations/storage/allocation-array-class-elements.cpp`, `tests/foundations/storage/allocation-array-class-elements-constant.cpp`, `tests/foundations/storage/allocation-array-unbuilt-element-constant.cpp`

## Context

A growable array gets storage with `::operator new(n * sizeof(T))`,
converts the result to `T*`, and builds element `i` with placement
`new` at `p + i`. `T` has a constructor or a destructor that the
program writes, so `T` is not an implicit-lifetime type.

STG-8 says that the call creates objects of implicit-lifetime types.
An array of `T` is such a type, whatever `T` is (STG-7). If the
elements of that array were not created, `p` would have no element 0
to point to, and `p + i` would fall under the arithmetic of a single
object.

| Configuration | Result |
|---|---|
| Clang and GCC, `-O0`, `-O2`, address and undefined sanitizers | the program runs, with no report |
| Clang, GCC, EDG, constant evaluation with a declared `std::allocator` | accept `p + 3` and placement `new` there before any element exists |
| Clang, GCC, EDG, constant evaluation | reject a read of an element that the program has not built |
| Clang, constant evaluation | rejects an assignment to an `int` element before `std::construct_at` builds it |
| GCC, EDG, constant evaluation | accept that assignment |

## Evidence

The three tests above, and `tests/foundations/language-support/allocator-self-declared-constant.cpp`.
Clang also rejects `p + 3` during constant evaluation for an array of 2
elements, with a note that names the bound of 2. GCC accepts that
pointer.

## Options

1. The result points to element 0 of an array of `T`. The elements
   exist before their lifetimes begin, so arithmetic over them is
   valid, and placement `new` at `p + i` begins the lifetime of element
   `i`.
2. The result points to the array, or to no element. Arithmetic past
   `p` is undefined until the elements exist, and a program needs
   another way to build a growable array.

## Provisional answer

Yes; `p + i` is valid before element `i` exists (option 1). Every
configuration in the matrix behaves this way, and constant evaluation,
which rejects undefined behavior, accepts the arithmetic and rejects a
read of an element that is not built. Option 2 would make every
growable container that the program writes undefined. The divergence
on an assignment to an `int` element in constant evaluation stays open
under this question. The tests build each element with
`std::construct_at`.

Kept open on 2026-11-03 under decision 0054, because two newest families differ in one form. The review supports the answer for the element pointer and the arithmetic. In the form that assigns to an `int` element before its construction during constant evaluation, the review reads the reference as Clang 22 does, and GCC 16 and EDG 7.0 accept the assignment.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as supporting option 1, the provisional answer. An allocation by name creates an array of the element type, which is an implicit-lifetime type for any element type. Its elements exist, and their lifetimes do not start when the element type is not an implicit-lifetime type. The limits on a pointer outside a lifetime do not include arithmetic, so a step over elements that are not built is valid. The result of the allocation points to one of the created objects that starts at the first byte, and element 0 qualifies. The reference says this through its intent, and the library's allocator says that it returns a pointer to the first element. In constant evaluation, `allocate` of the library's allocator starts the lifetime of the array and of none of its elements. So an assignment to an `int` element before its construction writes outside the element's lifetime, and Clang's rejection agrees with the reference.
