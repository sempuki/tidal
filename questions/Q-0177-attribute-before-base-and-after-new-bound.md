# Q-0177: Can an attribute specifier come before a base class and after the array bound of a `new` expression?

**Status:** settled
**Affects:** ATTR-19

## Context

ATTR-19 lists the positions of an attribute specifier. Two positions
divide the compilers:

| Position | Clang 18, 22, host | GCC 14 | GCC 16, host | EDG 7.0 |
|---|---|---|---|---|
| `struct D : [[a]] Base {};` | accepts | rejects, "expected class-name before '[' token" | accepts | accepts |
| `new int[3] [[a]]` | accepts | rejects, "'tidal' was not declared in this scope" | accepts | accepts |

Every compiler accepts the other positions of ATTR-19.

## Evidence

- `tests/declarations/attributes/attribute-positions-base-and-new-bound.cpp`,
  with an override for GCC 14.
- `tests/declarations/attributes/attribute-positions.cpp` for the other
  positions.

## Options

1. Both positions are valid. GCC 14 has a defect that GCC 16 fixes.
2. Both positions are not valid. Clang, EDG, and GCC 16 have the same
   extension.

## Answer

Yes; GCC 14 defect (option 1). Every other compiler in the matrix accepts both positions,
including the newer GCC.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. GCC 14 still rejects both positions, and the test keeps its override for GCC 14.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
