# Q-0246: Is a member of the current instantiation that names another single member at the instantiation an error?

**Status:** provisional
**Affects:** DEP-35, `tests/templates/dependent-names/qualified-member-other-at-instantiation.cpp`

## Context

`Fixed` is a virtual base of `Both<T>` and of `Extra`, and both
`Fixed` and `Extra` declare `m`. In `Both<T>`, `this->m` and `Both::m`
find `Fixed::m` at the definition. With `T = Extra`, `Extra::m` hides
`Fixed::m`, and the second lookup finds `Extra::m` alone.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, and uses `Extra::m` |
| GCC 14, 16, host | accepts, and uses `Extra::m` |
| EDG 7.0 | accepts |

## Evidence

The test above. Item 6 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says
that a second lookup with a different result makes the program not
valid, and predicts a rejection. No compiler rejects.

## Options

1. The program is not valid, and no diagnostic is required.
2. The program is valid and uses the member of the second lookup.
3. The implementation must reject the program.

## Provisional answer

Option 1. Every compiler accepts, and option 1 allows that result. The gap review reads the program as not valid. Option 2 would define a result that the review's reading leaves invalid.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
