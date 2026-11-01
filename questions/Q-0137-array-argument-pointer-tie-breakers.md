# Q-0137: Do the pointer tie-breakers apply after an array-to-pointer step?

**Status:** provisional
**Affects:** ICS-26, ICS-33, ICS-36, `tests/foundations/conversion-sequences/array-pointer-tie-breaker.cpp`

## Context

The argument is an array of `Derived`, where `Derived` derives from
`Middle` and `Middle` from `Base`. The array becomes a pointer, and the
pointer converts to `void*`, `Base*`, or `Middle*`. For a pointer
argument, every compiler applies ICS-33 (a base pointer beats `void*`)
and ICS-36 (the nearer base wins).

| Compiler | `void*` against `Base*` | `Base*` against `Middle*` |
|---|---|---|
| Clang 18, Clang 22 | `Base*` | `Middle*` |
| EDG 7.0 | `Base*` | `Middle*` |
| GCC 14, GCC 16 | ambiguous | ambiguous |

## Evidence

`tests/foundations/conversion-sequences/array-pointer-tie-breaker.cpp`,
and the row "Derived[2]" of the pointers grid on the page.

## Options

1. The tie-breakers apply. Sequences compare without their lvalue
   transformations (ICS-26), so an array argument ranks as a pointer
   argument does (Clang, EDG).
2. The tie-breakers do not apply after an array-to-pointer step (GCC).

## Provisional answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the answer keeps
lvalue transformations out of every comparison.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
