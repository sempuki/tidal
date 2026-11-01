# Q-0305: Does an array of plain `char` host an object created in it?

**Status:** provisional
**Affects:** STG-4, STG-6, `tests/foundations/storage/char-array-holds-object.cpp`

## Context

STG-4 lets an array of `std::byte` or `unsigned char` host an object
that placement `new` creates inside it. The hosted object is nested in
the array, so the array keeps its lifetime (STG-6). STG-14 lets `char`
read and write any object, as `unsigned char` does. An array of plain
`char` is a common buffer in existing programs.

| Implementation | Result |
|---|---|
| GCC, Clang, at `-O0` and `-O2` | a `long` created in an array of `char` gives its value, as in an array of `unsigned char` |
| sanitizers | no report for either array |

## Evidence

`char-array-holds-object.cpp` runs on every configuration. No
implementation in the matrix treats the two arrays differently.

## Options

1. Only arrays of `unsigned char` and `std::byte` host objects. A new
   object in an array of `char` ends the array's lifetime (STG-6), and
   a later use of the array as `char` elements is undefined.
2. Arrays of `char` host objects too.

## Provisional answer

No; only arrays of `unsigned char` and `std::byte` host objects (STG-4, option 1). The answer follows the description of the language in the phase 5 inventory. No implementation behavior separates the options, so the
test accepts the program's run under both.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
