# Q-0120: Does EDG's 8-byte alignment of `long long` on i686 change TYP-2?

**Status:** settled
**Affects:** TYP-2, TYP-12, `tests/foundations/types/align-ilp32.cpp`, `tests/foundations/types/align-member-ilp32.cpp`

## Context

On the `-m32` target, GCC and Clang give `long long` an alignment of 4,
so a struct of a `char` and a `long long` takes 12 bytes. EDG 7.0, built
with its i686 configuration, gives `long long` an alignment of 8, and
the same struct takes 16 bytes. The three agree on `double` (4) and
`long double` (4).

The alignment also moves bit-fields (BF-9). With `-m32`, Clang and GCC
let an `unsigned long long` bit-field start at any multiple of 4 bytes,
so a field of 30 bits after one of 40 bits starts at bit 40. EDG starts
such a field at bit 64
(`tests/classes/unions/bitfield-next-unit-ilp32.cpp`,
`tests/classes/unions/bitfield-wide-layout-ilp32.cpp`).

## Evidence

`tests/RESULTS.md`, rows `align-ilp32.cpp` and `align-member-ilp32.cpp`
for `edg-7.0-ilp32`.

## Options

1. Alignment is implementation-defined (TYP-2), so EDG's choice is
   valid. TYP-2 records both values. Programs that share structs with
   code built by GCC or Clang need the same layout, so EDG's choice is
   worth reporting to the EDG project.
2. Treat the GCC and Clang value as the target's rule, and EDG's as a
   compiler defect.

## Answer

No; the alignment is implementation-defined (option 1). TYP-2 is implementation-defined, and EDG's configuration is
internally consistent. The tests carry an override for
`edg-7.0-ilp32`.

Settled on 2026-11-03 under decision 0054. The review supports this answer with no doubt, and TYP-P3 gives the same answer: the target fixes the sizes and alignments, and the alignment of 8-byte types is one of the choices. EDG 7.0 with `-m32` differs, and the tests keep their override.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of program structure (`reports/review/2026-11-03-gap-review-program-structure.md`) reads the reference as supporting option 1, the provisional answer, with no doubt. Alignment is implementation-defined in the reference.
