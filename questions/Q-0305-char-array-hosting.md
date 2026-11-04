# Q-0305: Does an array of plain `char` host an object created in it?

**Status:** settled
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

## Answer

No; only arrays of `unsigned char` and `std::byte` host objects (STG-4, option 1). The answer follows the description of the language in the phase 5 inventory. No implementation behavior separates the options, so the
test accepts the program's run under both.

The question stays open under decision 0051 (2026-11-01). No implementation behavior separates the options, so the newest versions give no ground to settle the answer.

Settled on 2026-11-03 under decision 0054. The review supports this answer with no doubt, and STG-P2 gives the same answer. The elements of a byte array have the type `unsigned char` or `std::byte`, and an array of `char` holds no object in this way. No family differs, because no implementation behavior separates the options, and the test accepts the run under both.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as supporting option 1, the provisional answer, with no doubt. Only arrays of `unsigned char` and of `std::byte` provide storage for other objects.

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) reads the reference as supporting option 1. Only arrays of `unsigned char` and `std::byte` provide storage for a nested object, and plain `char` is not in the list.
