# Q-0357: What value does a derived member have after a byte copy into the base subobject writes over it?

**Status:** provisional
**Affects:** STG-25, STG-11, STG-12, `tests/foundations/storage/base-bytes-shared-with-member.cpp`
**Waits for:** a review (against)

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

Kept open on 2026-11-04 under decision 0054, because the gap review of the rules that phase 7 added reads a byte copy into a base or a marked member as undefined by omission, with doubt. STG-25 keeps the result that the compilers show.

Kept open on 2026-11-05 under decision 0054, because the review reads against the answer (gap review of the text phase 9 wrote, item RV3).

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that phase 7 added (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) reads the reference with doubt, against the answer. The reference gives a byte copy its result only when neither the source nor the target is a subobject that can overlap. For a base or a marked member, the reference does not say what a byte copy into it does, not even for the target's own value, so the case is undefined by omission. STG-25 and this answer promise a result that the reference does not give.

### Gap review, 2026-11-05 (declarations, statements, and foundations)

The source-reader gap review of the text phase 9 wrote on declarations, statements, and foundations (`reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`) reads the question: reads against, with doubt. The reference's guarantee for byte copies excludes potentially overlapping subobjects, and a base subobject is one. So the copy into the base has no meaning in the reference (undefined by omission). Option 2 is closer.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as unclear, leaning to option 1. The guarantees of the reference for byte copies leave out subobjects that can overlap, so the reference promises nothing for the base. A write through `unsigned char` to the bytes of the member is an allowed access, and the reference does not say what the member then holds. Nothing makes the copy undefined as a whole.
