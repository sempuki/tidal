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

Reopened as provisional on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt. TYN-P1 fails step 3 of the test. The model allows a name for linkage only for an unnamed class "with nothing in it that needs a name of its own". The model does not say whether a member function, a base, or a default member initializer is such a thing, so the answer turns on the list in TYN-3. No other principle on the page gives the answer. No newest family differs. GCC 14 accepts the class, and the test keeps its override for GCC 14.

On 2026-11-04, item PQ widened TYN-P1 to list what a class that gets its name for linkage from an alias must not have: member functions, bases, and default member initializers. The new test `tests/declarations/type-names/typedef-unnamed-class-default-member-initializer.cpp` checks the third item. Clang 22, GCC 16, and EDG 7.0 reject that class in a probe, and GCC 14 accepts it. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses TYN-P1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. An unnamed class that gets a typedef name for linkage may have only data members, member classes, and member enumerations. It may have no base and no default member initializer, and nested classes have the same limits. The rule entered with C++20, so GCC 14's acceptance is a defect.
