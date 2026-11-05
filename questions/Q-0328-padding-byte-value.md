# Q-0328: What does a read of a padding byte give, and can a program copy it through `char`?

**Status:** provisional
**Affects:** STG-11, STG-12, INIT-1, `tests/foundations/storage/byte-round-trip.cpp`, `tests/foundations/storage/padding-bytes-not-in-value.cpp`
**Waits for:** a review (against)

## Context

STG-11 says that the bits outside the value representation are padding,
and that a change to padding does not change the value. No rule said
what a read of a padding byte gives. STG-12 let a program copy the
bytes of an object into an array of `char`. INIT-1 lets a program copy
a value that does not exist only into `unsigned char` or `std::byte`
(Q-0039). So a byte copy of an object with padding through `char`
reads a byte with no value through `char`, and INIT-1 makes that read
undefined.

A front end that tracks which bytes have values needs an answer for a
padding byte that the program has not written.

## Evidence

`byte-round-trip.cpp` copies the bytes of a class with padding through
`unsigned char` and back. Clang and GCC run the program with the
address and undefined sanitizers and report nothing.
`padding-bytes-not-in-value.cpp` writes each padding byte through
`unsigned char`, and the value of the object stays the same. No
sanitizer in the matrix tracks bytes with no value, so no configuration
can tell the options apart. A copy by a library function reads no byte
through a `char` glvalue, so STG-12 sets no such condition on it.

## Options

1. A padding byte has no value until the program writes the byte
   through a byte pointer. A byte copy through `unsigned char` or
   `std::byte` copies the missing value (INIT-1). A copy through `char`
   is undefined for a byte with no value.
2. As option 1, and INIT-1 also lets a program copy a value that does
   not exist into `char`. Every byte copy of STG-12 is then defined.
3. A padding byte has an unspecified value, which a program can read
   through any character type.

## Provisional answer

A padding byte has no value until the program writes it, and a copy through `char` needs a value in each byte (option 1).
Option 1 keeps the exception of INIT-1 as Q-0039 gives it. A program
that copies an object with padding uses `unsigned char` or `std::byte`.

Kept open on 2026-11-03 under decision 0054, because the review reads against the answer with doubt. STG-12 now allows an array of `char`, and sets the condition only on a copy that reads each byte through a `char` glvalue.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as unclear, leaning to option 2 or 3. The reference never says that a padding byte of a class has an indeterminate value. An automatic object with no initialization has one as a whole, and for an initialized object the reference does not address its padding. A note on integer types gives padding bits an unspecified value that cannot trap. Separately, the reference allows a byte copy into an array of plain `char`, also for an object whose value is not valid. Only `unsigned char` and `std::byte` can carry an indeterminate value, so a copy written by hand that reads such a byte through `char` is undefined. A copy by the library's copy functions has no such limit.
