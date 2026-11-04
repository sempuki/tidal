# Q-0357: What value does a derived member have after a byte copy into the base subobject writes over it?

**Status:** provisional
**Affects:** STG-25, STG-11, STG-12, `tests/foundations/storage/base-bytes-shared-with-member.cpp`

## Context

On the matrix targets, a member of a derived class can occupy the
padding at the end of a base subobject (STG-25). A program copies the
`sizeof(Base)` bytes of another `Base` into the base subobject. The
copy writes the bytes of the derived member too. STG-12 does not cover
a copy into a base subobject. STG-11 gives a padding byte no value
until the program writes the byte through a byte pointer.

| Compiler | The member after the copy |
|---|---|
| Clang 18, Clang 22 | holds the copied byte |
| GCC 14, GCC 16 | holds the copied byte |
| EDG 7.0 | builds no program |

## Evidence

`tests/foundations/storage/base-bytes-shared-with-member.cpp`, which
gives the source byte a value first and checks the member at `-O0` and
`-O2`.

## Options

1. The member takes the value that its new bytes give. A copied padding
   byte with no value gives the member no value, and a read of the
   member is then undefined (INIT-1).
2. The copy is undefined as a whole, because it writes outside the
   object that STG-12 names.

## Provisional answer

The member takes the value of its new bytes, and a padding byte with no value gives it none (option 1). The compilers show the copied
byte in the member, and option 1 follows STG-11 and STG-13 without a
new kind of undefined behavior.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
