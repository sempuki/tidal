# Q-0304: Can a pointer to a character type step through the bytes of an object that is not an array of bytes?

**Status:** provisional
**Affects:** STG-13, ADD-6, ADD-10, `tests/foundations/storage/byte-pointer-walks-object.cpp`, `tests/foundations/storage/byte-round-trip.cpp`, `tests/foundations/storage/object-bytes-cover-members.cpp`, `tests/foundations/storage/past-end-address.cpp`, `tests/foundations/storage/padding-bytes-not-in-value.cpp`

## Context

ADD-6 and ADD-10 define pointer arithmetic over the elements of an
array. A pointer to `unsigned char` that a cast gives from a pointer to
a `long` points to the first byte of the `long`. No array of
`unsigned char` exists there, so ADD-6 gives no meaning to `p + 1`.
Byte copies (STG-12) and the inspection of an object representation
(STG-11) need such steps.

| Implementation | Result |
|---|---|
| Clang 18, 22, host 22, at `-O0` and `-O2` | each step gives the next byte, up to one past the last byte |
| GCC 14, 16, host 16, at `-O0` and `-O2` | the same |
| `-fsanitize=address,undefined` on GCC and Clang | no report |
| constant evaluation | cannot form the pointer, because a constant expression has no `reinterpret_cast` (CAST-8) and no cast from `void*` in C++23 |

## Evidence

The tests above run on every configuration and give the expected
bytes. No implementation in the matrix reports an error.

## Options

1. The bytes of any object act as an array of `sizeof` elements of
   `unsigned char` for pointer arithmetic and comparison, through a
   pointer to `std::byte`, `unsigned char`, or `char`. Byte copies and
   byte inspection have defined behavior.
2. Only a byte array has elements, and a step through another object's
   bytes is undefined. Byte copies of other objects then have no
   defined form without a library function.

## Provisional answer

Yes; the bytes act as an array of `sizeof` elements (STG-13, option 1). Every implementation behaves this way, and the
byte-copy rules (STG-12, CPY-19) need the steps.

Kept open on 2026-11-03 under decision 0054, because the review reads the reference with doubt.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as unclear, with option 1 supported by its intent. By the letter, a cast of a pointer to an object to a pointer to `unsigned char` still points to the object, because no byte object is pointer-interconvertible with it. Arithmetic on that pointer is then undefined, because the type of the pointer and the type of the element are not similar. The reference still calls the object representation a sequence of `unsigned char` objects, and it promises byte copies, which need the step. The review knows of a proposal to fix access to object representations that C++23 did not adopt. Option 1 matches the intent and every implementation.
