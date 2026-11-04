# Q-0306: Can a program read or write another object through a `signed char` glvalue?

**Status:** settled
**Affects:** STG-14, STG-23, `tests/foundations/storage/access-signed-char.cpp`

## Context

STG-14 lists the glvalue types that can read or write an object of
another type: `char`, `unsigned char`, and `std::byte`. `signed char`
is not on the list, so STG-23 makes such an access undefined.

| Implementation | Result |
|---|---|
| GCC 14, 16, host 16, at `-O0` and `-O2` | a write through `signed char` changes the `int` that a later read sees |
| Clang 18, 22, host 22, at `-O0` and `-O2` | the same |
| Clang 22 type sanitizer | no report |

## Evidence

`access-signed-char.cpp` exits 0 on every configuration. The same
program with `float` in place of `signed char`
(`access-unrelated-scalar-sanitizer.cpp`, built without the sanitizer)
exits 1 at `-O2` with GCC and Clang, so the optimizers do assume the
rule for other types.

## Options

1. `signed char` is not an allowed type, and the access is undefined
   (STG-23). The implementations' treatment is one result that the
   undefined behavior allows.
2. `signed char` is an allowed type, as all character types are in
   the implementations.

## Answer

No; `signed char` is not an allowed type, and the access is undefined (STG-23, option 1). The answer follows the description of the language in the phase 5 inventory. The implementations' results lie within what option 1
allows (decision 0047). A program that needs byte access can use
`unsigned char` under both options.

Settled on 2026-11-03 under decision 0054. The review supports this answer with no doubt, and STG-P7 gives the same answer, because its table lists `signed char` as not allowed. Every family runs the program as option 2 would, which the undefined behavior allows (decision 0047). The test checks only that the program compiles, so no family differs.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as supporting option 1, the provisional answer, with no doubt. The list of types through which a program can reach another object has `char`, `unsigned char`, and `std::byte` only.
