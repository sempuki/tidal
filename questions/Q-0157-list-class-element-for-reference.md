# Q-0157: Does a braced list with one class element bind a reference through the element's conversion function?

**Status:** settled
**Affects:** ICS-20, REF-6, REF-14, `tests/overloading/conversion-sequences/list-reference-class-element-scalar.cpp`, `tests/overloading/conversion-sequences/list-reference-class-element-class.cpp`, `tests/declarations/initialization/references/braced-list-conversion-temporary.cpp`, `tests/declarations/initialization/references/braced-list-conversion-nonconst.cpp`
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences

## Context

A class `Source` has a conversion function that returns an lvalue
reference. The argument or initializer is `{s}`, where `s` is a
`Source` lvalue. The element's type is not the referenced type or a
class derived from it.

REF-6 binds the reference to the element, as `=` does, when the element
has the referenced type, ignoring cv-qualifiers, or a class derived
from it. REF-14 makes a temporary from the list in the other cases.

| Reference | REF-6 and REF-14 | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|---|
| `int&`, from `operator int&()` | temporary, so rejected | temporary, so rejected | binds to the result | temporary, so rejected |
| `const int&`, from `operator int&()` | temporary | temporary | binds to the result | temporary |
| `int&&`, from `operator int&()` | temporary | temporary | binds to the lvalue, so rejected | temporary |
| `Item&`, from `operator Item&()` | temporary, so rejected | temporary, so rejected | binds to the result | temporary, so rejected |
| `const Item&`, from `operator Item&()` | temporary | temporary | binds to the result | temporary |
| `Item&&`, from `operator Item&()` | temporary | temporary | temporary | temporary |

The same split holds for a parameter. For overload resolution, a
temporary that the reference cannot bind gives no sequence.

For a reference to an array, every compiler binds one braced lvalue of
the array type directly (REF-6). The same condition then decides for
scalars, classes, and arrays.

## Evidence

The tests in **Affects**. A run of
`braced-list-conversion-temporary.cpp` on Clang shows that `const int&`
and `int&&` refer to a temporary that keeps the old value after the
source object changes. GCC accepts `const Item& r{s};` with `&r` equal
to the address of the conversion function's result. A source reader's
gap review (item E1) reports that the source binds the reference to the
element only when the referenced type is reference-related to the
element's type, and makes a temporary otherwise. For `int&&` from
`operator int&()`, GCC's message is "cannot bind rvalue reference ...
to lvalue".

## Options

1. Bind through the element when the referenced type is not a class or
   an array, and make a temporary otherwise. GCC agrees on the scalar
   lvalue rows, Clang and EDG on the class rows, and no compiler on all
   rows.
2. Bind to the element only when the element has the referenced type or
   a derived class, and make a temporary otherwise (Clang, EDG).
3. Always bind through the element (GCC, except for a class rvalue
   reference).

## Answer

No; a temporary unless the element has the referenced type or a derived class; Clang and EDG agree, GCC differs (option 2). Clang and EDG agree on every row, and the gap review agrees.
The condition is the one that every compiler uses for an array element,
so REF-6 states one condition for every object type.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) supports this answer with no doubt, and REF-P3 gives the same answer: a braced element binds directly only if it is already what the reference names, and one value that converts to the referenced type makes a temporary. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 2. The reference binds through the one element only when the referenced type is related to the element's type: the same type up to qualifiers, a similar array type, or a base. Otherwise a temporary is made by copy list initialization, and an lvalue reference that is not `const` cannot bind to it. Clang and EDG follow this reading.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
