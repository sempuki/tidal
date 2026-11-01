# Q-0120: Does EDG's 8-byte alignment of `long long` on i686 change TYP-2?

**Status:** provisional
**Affects:** TYP-2, TYP-12, `tests/foundations/types/align-ilp32.cpp`, `tests/foundations/types/align-member-ilp32.cpp`

## Context

On the `-m32` target, GCC and Clang give `long long` an alignment of 4,
so a struct of a `char` and a `long long` takes 12 bytes. EDG 7.0, built
with its i686 configuration, gives `long long` an alignment of 8, and
the same struct takes 16 bytes. The three agree on `double` (4) and
`long double` (4).

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

## Provisional answer

No; the alignment is implementation-defined (option 1). TYP-2 is implementation-defined, and EDG's configuration is
internally consistent. The tests carry an override for
`edg-7.0-ilp32`.

## Opinions
