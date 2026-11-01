# Q-0243: Can a deduced class template name follow a dependent qualifier?

**Status:** provisional
**Affects:** CTAD-1, `tests/templates/class-template-deduction/dependent-qualifier.cpp`

## Context

In a function template, `typename Outer<T>::In x(1);`, where `In` is a
member class template of `Outer<T>`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, `Outer<T>::In<int>` after instantiation |
| GCC 14, 16, host | accepts, the same type |
| EDG 7.0 | accepts, the same type |

The source reader's inventory (item G7) says that the qualifier before
a deduced name must not be dependent. The compilers differ when the
name has the keyword `template` before it: Clang rejects
`typename Outer<T>::template In x(1);`, and GCC and EDG accept it. No
test covers the form with `template`.

## Evidence

The test above, and a probe with the keyword `template`.

## Options

1. The qualifier can be dependent, as every compiler accepts.
2. The qualifier must not be dependent, as the inventory says.

## Provisional answer

Option 1. Every implementation accepts the form, and the specification
follows the behavior that implementations share. A gap review can show
whether the inventory item names a different form.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
