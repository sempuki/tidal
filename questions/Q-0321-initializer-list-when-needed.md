# Q-0321: When does a program need the class `std::initializer_list`?

**Status:** provisional
**Affects:** LSUP-1, `tests/foundations/language-support/initializer-list-parameter-not-needed.cpp`, `tests/foundations/language-support/initializer-list-candidate-not-viable.cpp`, `tests/foundations/language-support/initializer-list-auto-deduction.cpp`

## Context

A program declares `std::initializer_list` itself as a class template
with no members. LSUP-1 requires a fixed shape when the program needs
the class. The question is which operations create that need.

| Program | Clang 22, 18 | GCC 16, 14 | EDG 7.0 |
|---|---|---|---|
| declares a function with a parameter of type `std::initializer_list<int>`, and never calls it | accepts | rejects | accepts |
| calls an overloaded function with `{1, 2}`, where the elements do not convert to the element type of the `std::initializer_list` candidate | accepts | rejects | accepts |
| a braced list initializes the parameter | rejects | rejects | rejects |
| `auto x = {1, 2};` | rejects | rejects | rejects |

GCC checks every definition of `std::initializer_list` against its own
header, at the definition. Clang and EDG check the shape where a braced
list initializes an object of the type.

## Evidence

The three tests above and `initializer-list-no-members.cpp`.

## Options

1. The need arises where a braced list initializes an object of type
   `std::initializer_list<E>`, or where `auto` deduction gives that
   type (Clang, EDG).
2. The need arises where the program defines `std::initializer_list`
   (GCC).
3. The need arises where overload resolution forms a conversion
   sequence to a parameter of that type.

## Provisional answer

Where a braced list initializes an object of that type or `auto` deduces it (LSUP-1); GCC checks every definition (option 1). Two of three implementations agree, and option 1 names the
operations that use the members. The tests record GCC as an override.

Kept open on 2026-11-03 under decision 0054, because the review is silent.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as silent on the options. In the reference, every use of the class, named or implicit, needs a declaration from the library. That trigger is wider than option 1 and closest to GCC. A declaration that the program writes is outside the reference.
