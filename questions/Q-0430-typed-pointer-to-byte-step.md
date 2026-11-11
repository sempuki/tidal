# Q-0430: Where is `p + 1` when `p` is a `T*` that points to a byte, and can placement `new` use that position?

**Status:** provisional
**Affects:** ADD-14, STG-29, STG-17, `tests/foundations/storage/byte-array-typed-step-placement.cpp`, `tests/foundations/storage/byte-array-typed-step-distance.cpp`, `tests/expressions/additive/typed-pointer-to-byte-two-past.cpp`
**Source:** the round 11 programmer report (`reports/clean-room/roadmap-11/programmer.md`, sections 1.5 and 6), reductions `placement_at_typed_step_into_byte_array.cpp` and `typed_step_over_byte_array_distance.cpp`, probe `typed_step_two_past_byte.cpp`.
**Waits for:** a first review

## Context

A program casts the address of an `unsigned char` array to `int*`
through `void*`. By STG-29 the result `p` points to the first byte of
the array. The type of `p` differs from the type of the byte, so ADD-14
counts the byte as an array of one, and `p + 1` is the position one past
it. `p + 2` is undefined by ADD-14.

The text does not say which address `p + 1` holds. Two readings fit:

- the address one `int` on, `sizeof(int)` bytes after `p`, since the
  arithmetic counts in units of the pointer's type,
- the address one byte on, the end of the byte that `p` points to.

The text also does not say whether placement `new` can build an object
at that position.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 | Tidal front end |
|---|---|---|---|---|
| `p + 1`, cast back to a byte pointer, minus the array | `sizeof(int)` | `sizeof(int)` | accepts | 1 |
| placement `new` of an `int` at `p + 1` | runs | runs | accepts | undefined (STG-17, not aligned) |
| `p + 2`, with no read | builds | builds | accepts | runs, no report |

The GCC and Clang results come from the matrix run of the tests in
**Affects**. Host GCC 16 and Clang 22 run the `p + 2` program with
`-fsanitize=address,undefined` and report nothing. The round 11
programmer report gives the front end's results.

## Evidence

The tests in **Affects**. The placement test reads both `int` objects
through the pointers that placement `new` returns, so it reads no object
through `p + 1` itself.

## Options

1. `p + 1` holds the address `sizeof(T)` bytes after `p`. Placement
   `new` at that position is valid when the storage holds the object,
   and the pointer that placement `new` returns reaches the object
   (every compiler).
2. `p + 1` holds the address one byte after `p`. Placement `new` of an
   `int` there gives storage that is not aligned for the type (STG-17).
3. The position has no address that a program can use, so a cast of
   `p + 1` and placement `new` at it are undefined.

## Provisional answer

`p + 1` holds the address `sizeof(T)` bytes after `p`, and placement
`new` can use it (option 1). Every compiler gives this result, and an
arena or a vector over a byte array needs the step. `p + 2` stays
undefined by ADD-14.

Rule text on 2026-11-11 (decision 0062, item F4): ADD-14 says that a move counts in the size of the pointer's type, so `p + 1` is `sizeof(T)` bytes after a `T*` that points to a byte. STG-29 and STG-P2 say that placement `new` can use that position, and STG-28 says that a `T*` at a multiple of `sizeof(T)` bytes into allocated storage points to that element. The question now waits for a first review.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
