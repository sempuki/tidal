# Q-0372: What does a byte read give for the bits of a byte that no bit-field covers?

**Status:** provisional
**Affects:** BF-9, STG-11
**Waits for:** a review (silent)

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

Kept open on 2026-11-04 under decision 0054, because the gap review of the rules that phase 7 rewrote finds the reference silent on the value of such a byte, leaning to option 2. BF-9 now says that the bits of an unnamed bit-field are padding bits.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that the phase 7 front-end and Propose repairs rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`) finds the reference silent on the case, leaning to option 2. The reference calls the bits outside the value representation padding bits, the extra bits of a wide bit-field among them, and says that an unnamed bit-field is not a member. It says nothing direct about the value that a read through `unsigned char` gives for a byte with value bits and padding bits. Three places lean to option 2. A bit cast makes a bit of its result indeterminate when the bit comes from outside the source's value representation, and then the smallest object that holds the bit has an indeterminate value. A note on atomic compare-and-exchange says that padding bits outside the active member's value representation are indeterminate. And an automatic object with no initializer has an indeterminate value as a whole. Zero-initialization sets padding bits to zero, for a class and for a union, so after static initialization, or after value initialization of a class without a user-provided default constructor, the bits read as zero. The probes used aggregate initialization, which does not zero padding. Option 1 allows a byte copy through plain `char`, and option 2 only through `unsigned char` or `std::byte`. Committee work on access to object representations continues.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
