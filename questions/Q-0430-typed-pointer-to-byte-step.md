# Q-0430: Where is `p + 1` when `p` is a `T*` that points to a byte, and can placement `new` use that position?

**Status:** provisional
**Affects:** ADD-14, STG-29, STG-17, `tests/foundations/storage/byte-array-typed-step-placement.cpp`, `tests/foundations/storage/byte-array-typed-step-distance.cpp`, `tests/expressions/additive/typed-pointer-to-byte-two-past.cpp`
**Source:** the round 11 programmer report (`reports/clean-room/roadmap-11/programmer.md`, sections 1.5 and 6), reductions `placement_at_typed_step_into_byte_array.cpp` and `typed_step_over_byte_array_distance.cpp`, probe `typed_step_two_past_byte.cpp`.
**Waits for:** the maintainer (pending.md row 4)

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

Kept open on 2026-11-11 under decision 0054. Both reviews read against the answer, and every compiler that runs the programs gives the matrix's answer, so the answer stays and the question joins `pending.md` row 4 (item RV7). EDG runs no program. The two reviews differ on the case of STG-28 in allocated storage: part A reads it against, and part B says that the objects that the allocation creates can include an array of the pointer's type.

## Opinions

### Gap review, 2026-11-11 (phase 14 text, classes, declarations, foundations, and statements)

The source-reader gap review of the text phase 14 wrote on classes, declarations, foundations, and statements (`reports/review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md`) reads the question: reads against the answer (RV7-A-10). A cast through `void*` from a byte to `T*` gives a pointer that still points to the byte, as STG-29 says. Addition to a pointer is undefined when the pointer's type is not similar to the element type of the array that holds the object, and the byte is an element of a byte array. So `p + 1` is undefined, even for a step of one, and the step has no address. Option 3 is the closest. The same holds after an allocation function: byte arithmetic needs the created object to be a byte array, and then the typed step is undefined; if the created object is an array of `T`, the byte arithmetic is undefined. A defined path to the same address is byte arithmetic on the byte pointer, then placement `new` at that address, then use of the pointer that placement `new` returns. STG-28's sentence on a `T*` cast straight from the result of the allocation is not affected.

### Gap review, 2026-11-11 (phase 14 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 14 wrote on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md`) reads the question: reads against the answer (RV7-B-3). Addition and subtraction are undefined when the pointer's type and the element type of the array are not similar, and a `T*` to a byte of a byte array meets that condition. So the step by one is undefined and has no address. The review reads ADD-14's older sentence, which counts the object as an array of one for a pointer of another type, against the reference for the same reason (Q-0049). The allocated-storage case of STG-28 differs, because the objects that the allocation creates can supply an array of the pointer's type. A declared byte array gets no new pointer value from a cast.
