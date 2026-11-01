# Q-0306: Can a program read or write another object through a `signed char` glvalue?

**Status:** provisional
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
program with `float` in place of `signed char` exits 1 at `-O2`
(`access-unrelated-scalar.cpp`), so the optimizers do assume the rule
for other types.

## Options

1. `signed char` is not an allowed type, and the access is undefined
   (STG-23). The implementations' treatment is one result that the
   undefined behavior allows.
2. `signed char` is an allowed type, as all character types are in
   the implementations.

## Provisional answer

No; `signed char` is not an allowed type, and the access is undefined (STG-23, option 1). The answer follows the description of the language in the phase 5 inventory. The implementations' results lie within what option 1
allows (decision 0047). A program that needs byte access can use
`unsigned char` under both options.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
