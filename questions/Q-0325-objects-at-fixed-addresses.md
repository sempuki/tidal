# Q-0325: Does an object exist at an address that a program gets from an integer, such as a device register?

**Status:** provisional
**Affects:** CAST-6, CAST-16, VOL-1, `tests/expressions/casts/integer-to-pointer-address.cpp`

## Context

A program for a device converts an address from a data sheet to a
pointer to `volatile` and reads and writes through it. CAST-6 lets
`reinterpret_cast` convert the integer, and CAST-16 says that the
pointer has the address that the integer gives on the matrix targets.
No rule says that an object exists at that address. The program never
created one there.

The matrix runs on a hosted system, where such an address holds no
storage. A test can form the pointer and compare it, and cannot access
it.

## Evidence

`integer-to-pointer-address.cpp` forms a pointer from `0x4000'1000`
and converts it back. Clang and GCC give the same integer.

## Options

1. Tidal sets no requirement. The platform documents which addresses
   hold objects and of which types. An access through such a pointer is
   then a volatile access (VOL-1).
2. An implementation-defined rule says that objects exist at the
   addresses that the implementation documents.
3. The access is undefined, as for any storage without an object.

## Provisional answer

The platform decides (option 1). The matrix targets have no such
addresses, so no test can back a rule. The specification gives the
conversion (CAST-6, CAST-16) and the meaning of a volatile access
(VOL-1), and the volatile page shows a register at a fixed address in
its example.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
