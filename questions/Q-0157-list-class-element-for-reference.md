# Q-0157: Does a braced list with one class element bind a reference through the element's conversion function?

**Status:** provisional
**Affects:** ICS-20, REF-6, REF-14, `tests/foundations/conversion-sequences/list-reference-class-element-scalar.cpp`, `tests/foundations/conversion-sequences/list-reference-class-element-class.cpp`

## Context

A class `Source` has a conversion function that returns an lvalue
reference. The argument is `{s}`, where `s` is a `Source` lvalue. The
element's type is not the referenced type or a class derived from it.

REF-6 binds the reference to the element, as `=` does, when the
referenced type is not a class or an array. REF-14 makes a temporary
from the list in the other cases.

| Parameter | REF-6 and REF-14 | Clang 18, 22 | GCC 14, 16 | EDG 7.0 |
|---|---|---|---|---|
| `int&`, from `operator int&()` | binds through the conversion function | no sequence | binds | no sequence |
| `Item&`, from `operator Item&()` | temporary, so no sequence | no sequence | binds | no sequence |
| `int&&`, from `operator int&()` | binds to the lvalue, so no sequence | temporary | no sequence | temporary |

The same split holds for a declaration such as `int& r{s};`.

## Evidence

The two tests in **Affects**. A source reader's gap review (item E1)
reports that the source binds the reference to the element only when
the referenced type is reference-related to the element's type, and
makes a temporary otherwise.

## Options

1. Follow REF-6 and REF-14 as written. A non-class referenced type binds
   through the element (GCC), and a class referenced type makes a
   temporary (Clang, EDG).
2. Bind to the element only when the element has the referenced type or
   a derived class, and make a temporary otherwise (Clang, EDG).
3. Always bind through the element (GCC).

## Provisional answer

Option 1, so that ICS-20 agrees with the reference page. Option 2 has
two of three implementations and the gap review. The reference page
owns the choice, and ICS-20 follows REF-6 and REF-14.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
