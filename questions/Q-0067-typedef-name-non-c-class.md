# Q-0067: Can a typedef name an unnamed class that is not C-like?

**Status:** settled
**Affects:** TYN-3, `tests/declarations/type-names/typedef-unnamed-class-member-function.cpp`

## Context

`typedef struct { ... } Name;` gives an unnamed class a name for linkage.
If the class has a member function, GCC 16, Clang, and EDG reject the
typedef. GCC 14 accepts it.

## Options

1. A class that gets its name from a typedef must be C-like: no member
   functions, bases, or default member initializers.
2. Any unnamed class can get its name from a typedef.

## Answer

No; a class that gets its name from a typedef must be C-like (option 1). The newer GCC agrees with Clang and EDG, so the behavior of
GCC 14 is treated as an older implementation choice.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and TYN-P1 gives the same answer: an alias can give a type a name for linkage only when the type is an unnamed class with nothing in it that needs a name of its own. Clang 22, GCC 16, and EDG 7.0 reject the class, and only GCC 14 accepts it, so the test keeps its override for GCC 14. No newest family differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. An unnamed class that gets a typedef name for linkage may have only data members, member classes, and member enumerations. It may have no base and no default member initializer, and nested classes have the same limits. The rule entered with C++20, so GCC 14's acceptance is a defect.
