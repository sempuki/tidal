# Q-0243: Can a deduced class template name follow a dependent qualifier?

**Status:** provisional
**Affects:** CTAD-1, `tests/templates/class-template-deduction/dependent-qualifier.cpp`, `dependent-qualifier-template-keyword.cpp`

## Context

In a function template, `typename Outer<T>::In x(1);`, where `In` is a
member class template of `Outer<T>`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, `Outer<T>::In<int>` after instantiation |
| GCC 14, 16, host | accepts, the same type |
| EDG 7.0 | accepts, the same type |

The source reader's inventory (item G7) says that the qualifier before
a deduced name must not be dependent. Item 19 of the gap review
`reports/review/2026-10-31-gap-review-packs-and-ctad.md` reads the
reference the same way, and notes that the compilers accept the
dependent form once the template is instantiated.

The compilers differ when the name has the keyword `template` before
it: `typename Outer<T>::template In x(1);`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: an argument list is expected after the name |
| GCC 14, 16, host | accepts, `Outer<T>::In<int>` |
| EDG 7.0 | accepts, `Outer<T>::In<int>` |

## Evidence

The tests above.

## Options

1. The qualifier can be dependent, as every compiler accepts.
2. The qualifier must not be dependent, as the inventory and the gap
   review read the reference.

## Provisional answer

Yes; every compiler accepts (option 1). Every implementation accepts the form without the keyword
`template`, and the specification follows the behavior that
implementations share. The form with `template` follows the same
answer, which GCC and EDG give. The question keeps both readings open
for contributors.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
