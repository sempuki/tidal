# Q-0327: Can a byte pointer made from the address of a member move over the bytes of the whole object?

**Status:** provisional
**Affects:** STG-13, Q-0304, `tests/foundations/storage/byte-pointer-from-member-walks-object.cpp`, `tests/foundations/storage/padding-bytes-not-in-value.cpp`
**Waits for:** a review (doubt)

## Context

STG-13 (Q-0304) lets a pointer to `unsigned char`, `char`, or
`std::byte` that points to the first byte of an object move over the
bytes of that object. A cast of the address of a member gives a byte
pointer to the first byte of the member. No rule said whether that
pointer can move past the member into the rest of the enclosing object,
or compare with a byte pointer made from the address of the enclosing
object. `padding-bytes-not-in-value.cpp` compares byte pointers made
from `&b.tag` and `&b.count` with a byte pointer that walks the whole
object `b`.

## Evidence

`byte-pointer-from-member-walks-object.cpp` makes a byte pointer from
the address of the second member of a class, moves it back to the
first byte of the object and forward to one past the last byte, reads
each byte, and compares the pointers with byte pointers made from the
address of the object. Every configuration of Clang and GCC, with the
address and undefined sanitizers, runs the program and gives the
expected bytes. Constant evaluation cannot form a byte pointer
(CAST-8).

## Options

1. A byte pointer that points to any byte of a complete object can move
   over all the bytes of that complete object, wherever the pointer
   came from. The bytes act as one array for arithmetic and comparison.
2. A byte pointer can move only over the bytes of the object whose
   address the program cast. A step from a member into the rest of the
   object is undefined, and so is a comparison with a byte pointer from
   the enclosing object.

## Provisional answer

Yes; the byte pointer can move over the bytes of the complete object (option 1).
Every implementation behaves this way, and code that finds the
enclosing object from the address of a member needs the step.

Kept open on 2026-11-03 under decision 0054, because the review reads the reference with doubt.

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with a narrower doubt (DQ-19). The doubt is the gap of Q-0304 (DQ-16), and the step from a member's bytes into the rest of the object has less support still.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as unclear, with option 1 supported by its intent, for the reasons it gives under Q-0304. The letter of the reference does not support the step from a member's bytes into the rest of the object. The reference still promises byte copies, which need such steps. Option 1 matches the intent and every implementation.

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports with a doubt that remains (DQ-19). The reasons of DQ-16 apply. A step from a member's bytes into the rest of the object has less support still, because the object representation is described for each object, and a member's representation covers only the member. Byte copies of the whole object still need such steps. The doubt is the same gap between the letter and the intent.
