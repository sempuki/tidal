# Q-0239: Does a class template that inherits constructors get the deduction guides of its base?

**Status:** provisional
**Affects:** CTAD-16, CTAD-19, `tests/templates/class-template-deduction/inherited-constructors.cpp`, `inherited-user-guide.cpp`, `tie-own-constructor-types-differ.cpp`

## Context

`template <class T> struct Derived : Base<T> { using Base<T>::Base; };`
and then `Derived d(1);`, where `Base<T>` has the constructor
`Base(T)`.

| Compiler | Result |
|---|---|
| GCC 14, 16, host | accepts, `Derived<int>` |
| EDG 7.0 | accepts, `Derived<int>` |
| Clang 18, 22, host | rejects: no viable constructor or deduction guide |

GCC and EDG also take a user guide of the base, and order the guides
from the class's own constructors before the inherited ones
(`tie-own-constructor-template.cpp`, `tie-own-constructor-user-guide.cpp`).
With `Derived(const T&)` and an inherited `Base(T)`, GCC and EDG find
the two guides ambiguous, because their parameter types differ and the
tie-breaker does not apply. Clang has only the first guide and deduces
`Derived<int>` (`tie-own-constructor-types-differ.cpp`).

## Evidence

The tests above, the grid `forms.json` (column "inherited
constructor"), and the inventory items E1 to E4 of
`reports/review/2026-10-31-inventory-packs-forwarding-ctad.md`.

## Options

1. The class gets the guides of the base through its inherited
   constructors (GCC, EDG, and the inventory).
2. The class gets no guides from its base (Clang).

## Provisional answer

Yes; GCC and EDG agree, Clang differs (option 1). Two of three implementations agree, and the source reader's
inventory lists the behavior as part of the language. The tie-breaker
tests of CTAD-19 give the same results on Clang, because Clang has no
inherited guide to compare.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
