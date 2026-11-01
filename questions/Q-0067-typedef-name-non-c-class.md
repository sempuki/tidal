# Q-0067: Can a typedef name an unnamed class that is not C-like?

**Status:** provisional
**Affects:** TYN-3, `tests/declarations/type-names/typedef-unnamed-class-member-function.cpp`

## Context

`typedef struct { ... } Name;` gives an unnamed class a name for linkage.
If the class has a member function, GCC 16, Clang, and EDG reject the
typedef. GCC 14 accepts it.

## Options

1. A class that gets its name from a typedef must be C-like: no member
   functions, bases, or default member initializers.
2. Any unnamed class can get its name from a typedef.

## Provisional answer

No; a class that gets its name from a typedef must be C-like (option 1). The newer GCC agrees with Clang and EDG, so the behavior of
GCC 14 is treated as an older implementation choice.

## Opinions
