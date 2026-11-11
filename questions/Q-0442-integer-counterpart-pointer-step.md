# Q-0442: Is `p + 1` defined for a pointer to the matching unsigned type that points to an element of an `int` array?

**Status:** provisional
**Affects:** ADD-14, STG-14, `tests/expressions/additive/integer-counterpart-one-past.cpp`
**Source:** point RV8-R-9 of the read of the RV7 repair diff (`reports/review/2026-11-11-read-of-rv7-repair-diff.md`), with a probe that a clean-room writer ran for it.
**Waits for:** the maintainer (pending.md row 4)

## Context

ADD-14 counts an object as an array of one when the pointer's type
differs from the element type of the array. Its second paragraph covers
a pointer to the signed or unsigned integer type that matches the
element type, and STG-14 lets a program read the element through such a
pointer:

```cpp
int values[4] = {1, 2, 3, 4};
unsigned* p = reinterpret_cast<unsigned*>(&values[0]);
unsigned first = *p;     // valid (STG-14)
unsigned* end = p + 1;   // one past the object, by ADD-14
```

By ADD-14, `p + 1` is the position one past the object, a read through
it is undefined, and `p + 2` is undefined.

The writer's host probe, under `-std=c++23 -pedantic-errors -O0
-fsanitize=undefined`, compares `p + 1` with the address of
`values[1]` cast to `unsigned*`:

| Form | GCC 16 | Clang 22 | EDG 7.0 |
|---|---|---|---|
| `p + 1 == reinterpret_cast<unsigned*>(&values[1])` | true, runs | true, runs | runs no program |

Constant evaluation cannot form the pointer (CAST-8), so no
test can check the step at compile time.

## Evidence

The test in **Affects**, and the probe in **Context**. The test checks `p + 1 - p` and the read through `p`, and every compiler that runs it passes.

## Options

1. `p + 1` is the position one past the object, as for an array of
   one. A read through it, and a move of two, are undefined (ADD-14).
2. Any addition other than zero on such a pointer is undefined, because
   the pointer's type and the element type are not similar. The
   pointer has no position one past.

## Provisional answer

`p + 1` is the position one past the object (option 1). Every compiler
that runs the program gives the address `sizeof(int)` bytes after `p`,
and ADD-14 states the case. The read of the RV7 repair diff reads the
case against the answer, so the question waits with `pending.md` row 4.

## Opinions

### Read of the RV7 repair diff, 2026-11-11

The source-reader read of the RV7 repair diff (`reports/review/2026-11-11-read-of-rv7-repair-diff.md`) reads the case: reads against the answer (RV8-R-9). The pointer's type and the element type of the array are not similar, so the reference makes any addition on the pointer undefined, the move to one past included. The reference's own example of the rule uses this integer case. The case differs from a pointer to a base class subobject, which is no element of an array, and for which ADD-14 and the reference agree (Q-0049).
