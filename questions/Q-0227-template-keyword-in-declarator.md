# Q-0227: Can the template keyword follow the qualifier of a name that a declaration declares?

**Status:** provisional
**Affects:** DEP-27, `tests/templates/dependent-names/template-keyword-in-declarator.cpp`

## Context

The definition `template <class T> template <class U> void
Box<T>::template put(U) {}` writes `template` before the declared name.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: expected an unqualified-id |
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |

## Evidence

The test above. Inventory item F7 says that the keyword must not
follow the qualifier of a name being declared.

## Options

1. The keyword is not valid there (Clang).
2. The keyword is allowed and has no effect (GCC, EDG).

## Provisional answer

No; Clang only, GCC and EDG accept (option 1). The inventory supports it, and the keyword has no purpose in
a declarator, where the name must already be a member template. The
provisional answer goes against two of three implementations, so a
contributor can argue for option 2.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
