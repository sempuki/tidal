# Q-0230: Is a qualified member name of the current instantiation looked up again at the instantiation?

**Status:** provisional
**Affects:** DEP-13, `tests/templates/dependent-names/qualified-member-found-again.cpp`, `tests/templates/dependent-names/this-member-ambiguous.cpp`

## Context

`template <class T> struct Both : Fixed, T` reads `Both::m`. At the
definition, the name finds `Fixed::m`. With `T = Extra`, `Extra::m`
also exists.

| Compiler | `Both::m` | `m` | `this->m` |
|---|---|---|---|
| Clang 18, 22, host | rejects: ambiguous | accepts `Fixed::m` | rejects |
| EDG 7.0 | rejects: ambiguous | accepts `Fixed::m` | rejects |
| GCC 14, 16, host | accepts `Fixed::m` | accepts `Fixed::m` | rejects |

## Evidence

The tests above, `unqualified-member-kept.cpp` for the unqualified
form, and inventory items A14 and A15. Item 5 of the gap review
`reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md`
uses the `this->m` form, which every compiler rejects.

## Options

1. The qualified name is looked up again and is ambiguous (Clang,
   EDG).
2. The qualified name keeps the member found at the definition (GCC).

## Provisional answer

Option 1. Two of three implementations and the inventory agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
