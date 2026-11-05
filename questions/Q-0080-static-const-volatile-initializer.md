# Q-0080: Can a static const volatile member have an initializer in its class?

**Status:** provisional
**Affects:** CLS-3, CLS-23, `tests/declarations/volatile/static-const-volatile-in-class.cpp`, `tests/declarations/volatile/static-inline-const-volatile-in-class.cpp`
**Waits for:** newest families that agree

## Context

`struct S { static const volatile int n = 1; };` gives a `static const`
integer member that is also `volatile` an initializer in the class. GCC
and EDG accept the declaration. Clang rejects it and asks for the
initializer outside the class.

## Options

1. The initializer in the class is valid.
2. The member must be initialized outside the class.

## Provisional answer

No; the member must be initialized outside the class unless it is `inline` (option 2). Clang rejects the declaration. GCC and EDG accept it.

Changed on 2026-11-03 to option 2. The review reads option 2, Clang 22 rejects, and CLS-P2 with VOL-P1 gives the same answer: an in-class initializer makes a constant that the class can use directly, and constant evaluation cannot read a volatile object. CLS-3 now excludes a `volatile` member, CLS-23 names the case, and VOL-2 no longer lists it. `static-const-volatile-in-class.cpp` cites CLS-23 and expects a rejection, with overrides for GCC and EDG. The new test `static-inline-const-volatile-in-class.cpp` accepts the `inline` form. The question stays open under decision 0054, because two newest families differ from the answer.

Kept open on 2026-11-04 under decision 0054, because two newest families, GCC 16 and EDG 7.0, differ from the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) reads option 2. A static data member can have an initializer in its class only if it is a `const` member of integer or enumeration type that is neither `volatile` nor `inline`, or if it is `inline`. A `const volatile` member that is not `inline` fits neither case, so the initializer must be diagnosed. Clang follows this reading. An `inline` member of the same type would be valid.
