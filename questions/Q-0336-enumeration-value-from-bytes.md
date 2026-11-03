# Q-0336: Is a read of an enumeration object undefined when its bytes hold a value outside the enumeration's values?

**Status:** provisional
**Affects:** ENUM-6, ENUM-22, STG-12, `tests/declarations/enumerations/bytes-out-of-range-sanitizer.cpp`

## Context

ENUM-22 gives the values of an enumeration. For `enum Color { red,
green, blue }`, which has no fixed underlying type, the values are 0 to
3. ENUM-6 makes a conversion of another value to the enumeration
undefined. STG-12 covers bytes copied out of an object and back into
the same object, so the value is one that the object had.

A program can also copy the bytes of another object into an object of
the enumeration type, for example the bytes of an `int` that holds 5.
No conversion takes place, so ENUM-6 as a rule about conversions does
not cover the case.

The undefined behavior sanitizer of Clang and GCC
(`-fsanitize=undefined`) stops the program at the read of the stored
value, with "load of value 5, which is not a valid value for type". The
sanitizer gives the same report for a value that a `static_cast`
stored. It does not stop the `static_cast` itself.

## Evidence

- `tests/declarations/enumerations/bytes-out-of-range-sanitizer.cpp`:
  Clang and GCC stop the read of the value that came from the bytes.
- `tests/declarations/enumerations/cast-out-of-range-sanitizer.cpp`:
  Clang and GCC stop the read of a value that came from a cast.

## Options

1. A read of an enumeration object whose stored value is not a value of
   the enumeration is undefined, however the value got there. The
   sanitizers of Clang and GCC report exactly this read.
2. Only the conversion is undefined. A value that a byte copy puts in
   the object reads back as that value.

## Provisional answer

Yes; the read is undefined, whatever stored the value (option 1).
ENUM-6 covers a conversion and every other means that leaves such a
value in the object. The sanitizers of both compilers check the read
and report the two cases in the same way.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
