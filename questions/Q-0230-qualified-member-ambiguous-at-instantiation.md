# Q-0230: Is a qualified member name of the current instantiation looked up again at the instantiation?

**Status:** settled
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

## Answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations and the inventory agree.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and DEP-P3 gives the same answer: a qualified name into the current instantiation is looked up again at the instantiation, and an ambiguity then makes the program not valid. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A qualified name into the current instantiation that finds a member is looked up again when a specialization is made. A different second result makes the lookup ambiguous, so the program is not valid. The unqualified form is valid. GCC's acceptance of the qualified form is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
