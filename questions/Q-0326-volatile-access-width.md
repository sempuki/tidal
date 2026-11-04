# Q-0326: What size of memory access does a volatile read or write use, and can an implementation merge or split volatile accesses?

**Status:** provisional
**Affects:** VOL-1, VOL-12, VOL-13, BF-9, `tests/declarations/volatile/bitfield-write-keeps-neighbors.cpp`

## Context

A device register can react to the size of an access. A register map
often uses bit-fields in a volatile class. VOL-1 says that each access
happens, and does not say how wide the access is.

At `-O2` on x86-64, Clang 22 and GCC 16 compile these accesses as
follows:

| Access | Clang 22 | GCC 16 |
|---|---|---|
| a write to a volatile `unsigned short` member | one 2-byte store | one 2-byte store |
| a write to a volatile `unsigned` bit-field of 3 bits | a 4-byte load, then a 4-byte store | a 4-byte load, then a 4-byte store |
| an initializer of a volatile class object | can use one wide store for several members (VOL-12) | the same |

## Evidence

The assembly of a probe that writes each kind of member, and the VOL-12
Evidence. The test harness cannot observe the size of an access, so the
test checks only that a write to one bit-field keeps the values of the
others.

## Options

1. Implementation-defined. Tidal records the sizes of the matrix
   targets in VOL-13.
2. Required: an access to a volatile scalar uses exactly its size, and
   an access to a bit-field uses the unit of BF-9.
3. Unspecified, with no table.

## Provisional answer

Implementation-defined (option 1). The size depends on the target's
instructions, and the compilers document the related options, such as
the GCC option for strict volatile bit-fields on other targets. Whether
an implementation can merge accesses, as in the VOL-12 initializer,
stays open.

Kept open on 2026-11-03 under decision 0054, because the review does not decide the open part. Whether an implementation can merge volatile accesses, as in the VOL-12 initializer, stays open, and VOL-P1 says only that each access is performed as written.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The meaning of a volatile access is implementation-defined, and a conforming implementation must still perform volatile accesses as the abstract machine does. Nothing says directly whether two accesses may merge into one. For the open part about an initializer, the review notes that volatile semantics do not apply to an object while its constructor runs.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
