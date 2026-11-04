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

Kept open on 2026-11-03. The review reads against the undefined behavior sanitizers of Clang and GCC, which stop the read in both tests, and EDG runs no program. Tidal keeps option 1, and the question waits on the maintainer (`documentation/decisions/pending.md`, row 4).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as supporting option 2. An enumeration has the object and value representation of its underlying type, and a value is stored with the same bits in either type. A note on the values of an enumeration says that the range serves promotions and conversions, and it does not stop an object of the enumeration type from holding another value. The undefined case is the conversion of a value outside the range to an enumeration with no fixed underlying type. No rule makes the later read undefined, so a value that a byte copy puts in the object reads back as that value. The sanitizers check the read because they can see the value there.
