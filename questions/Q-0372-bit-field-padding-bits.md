# Q-0372: What does a byte read give for the bits of a byte that no bit-field covers?

**Status:** provisional
**Affects:** BF-9, STG-11

## Context

A class such as `struct Flags { unsigned char low : 3; char next; };`
has a byte whose first three bits hold `low`. No member covers the
other five bits. A wide bit-field such as `unsigned char c : 12` also
has bits that hold no value (BF-3). A program can read the byte through
`unsigned char`, for example in a byte copy (STG-12).

STG-11 calls bits outside the value representation padding, and
Q-0328 gives a padding byte no value until the program writes it. No
rule says what a byte with value bits and padding bits gives.

## Evidence

A probe built with Clang and GCC at `-O0` copies such objects into an
array of `unsigned char`. The padding bits hold bits that the program
did not write, also after the object is initialized with `{}` and the
bit-fields are assigned. For a union of one bit-field of 4 bits, Clang
leaves the three other bytes as the stack held them. GCC leaves the
bytes after a wide bit-field of a class as the stack held them. No
sanitizer in the matrix tracks such bits, and no constant evaluation
reads a byte of a class object.

## Options

1. The bits are padding bits. A byte read gives the values of the
   bit-fields in their bits, and unspecified values in the padding
   bits. The read is defined.
2. A byte with a padding bit that has no value has no value as a whole,
   as a padding byte under Q-0328. A read through `unsigned char` or
   `std::byte` copies the missing value (INIT-1).
3. The padding bits read as 0.

## Provisional answer

The bits are padding bits with unspecified values, and the read is
defined (option 1). The probes show values that the program did not
write, so option 3 does not match the compilers. Option 1 lets a byte
copy of a class with bit-fields go through any byte type, as the
probes do. A front end that gives the padding bits the value 0 follows
option 1.

Kept open on 2026-11-03 under decision 0054, because no review has read the provisional answer. The answer rests on probes of the values that a byte read gives.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
