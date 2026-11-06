# Q-0325: Does an object exist at an address that a program gets from an integer, such as a device register?

**Status:** provisional
**Affects:** CAST-6, CAST-16, STG-15, VOL-1, `tests/expressions/casts/integer-to-pointer-address.cpp`, `tests/foundations/storage/integer-pointer-no-object-sanitizer.cpp`
**Waits for:** a review (silent)

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
`integer-pointer-no-object-sanitizer.cpp` reads through that pointer.
The address sanitizer of Clang and GCC stops the read with a fault at
the address.

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

Where no object lies at the address, the result of the conversion is an
invalid pointer value (STG-15). The round trip of CAST-16 still gives
the integer, and indirection through the pointer is undefined (UN-7).
On the matrix targets no object lies at such an address.

Kept open on 2026-11-03 under decision 0054, because the review is silent.

Kept open on 2026-11-05 under decision 0054, because the review reads the reference as silent (gap review of the text phase 9 wrote, item RV3).

## Opinions

### Gap review, 2026-11-05 (declarations, statements, and foundations)

The source-reader gap review of the text phase 9 wrote on declarations, statements, and foundations (`reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`) reads the question: silent. The reference leaves the mapping from integers to pointers to the implementation and has no notion of an object that exists at a fixed address without being created.

### Gap review, 2026-11-05 (expressions and templates)

The source-reader gap review of the text phase 9 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) reads the question: silent. The reference leaves the integer to pointer mapping to the implementation and says nothing about objects at such addresses.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as silent between options 1 and 2. The mapping from an integer to a pointer is implementation-defined apart from the round trip. Nothing in the reference makes an object exist at such an address, and nothing calls the result an invalid pointer value. So the last paragraph of the provisional answer goes beyond the reference.
