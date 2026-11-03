# Q-0328: What does a read of a padding byte give, and can a program copy it through `char`?

**Status:** provisional
**Affects:** STG-11, STG-12, INIT-1, `tests/foundations/storage/byte-round-trip.cpp`, `tests/foundations/storage/padding-bytes-not-in-value.cpp`

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
can tell the options apart.

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
