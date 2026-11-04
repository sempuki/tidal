# Q-0324: Does a cast of a byte array's address reach an object that the array created implicitly?

**Status:** settled
**Affects:** STG-8, STG-16, STG-23, STG-29, `tests/foundations/storage/byte-array-cast-reaches-byte.cpp`, `tests/foundations/storage/byte-array-placement-reaches-object.cpp`

## Context

A byte array creates objects of implicit-lifetime types when its
lifetime starts (STG-8). A program casts the array's address to `int*`
with `reinterpret_cast` and writes through the result.

STG-16 says that a cast between objects at one address that are not
pointer-interconvertible keeps the original object. An element of a
byte array and an object in the array are not pointer-interconvertible.
So by STG-16 the result points to a byte, and the write reaches an
`unsigned char` object through an `int` glvalue (STG-23). The
language-support page says that a program with no header has
no launder operation and keeps the pointer that placement `new`
returns.

| Configuration | Result |
|---|---|
| Clang and GCC, `-O0`, `-O2`, address and undefined sanitizers | the program runs, with no report |
| Clang 22, `-fsanitize=type` | no report for this program |
| constant evaluation | cannot form the cast (CAST-8) |

## Evidence

The two tests above. No configuration can tell the two answers apart.

## Options

1. The cast does not reach the object. The program reaches it through
   the pointer that placement `new` returns, or through the launder
   operation. The write in the test is undefined (STG-23).
2. The cast reaches the object, because the implementation picks the
   set of implicit objects that gives the program defined behavior.

## Answer

No; the cast points to a byte (option 1). Option 1 follows STG-16 as
written, and the implicit set of STG-8 decides which objects exist, and
not where a pointer points. A program that follows option 1 is valid
under both options. `byte-array-cast-reaches-byte.cpp` checks only
that the program compiles.

Settled on 2026-11-03 under decision 0054. The review supports this answer with no doubt, and STG-P2 gives the same answer: a cast of the array's address still points to a byte. No family differs, because the test checks only that the program compiles.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as supporting option 1, the provisional answer, with no doubt. An element of a byte array is not pointer-interconvertible with an object that the array hosts. So the cast still points to the element, and the reference names the launder operation for this case.
