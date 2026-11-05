# Q-0246: Is a program in error when a name found in the current instantiation finds a different single member at the instantiation?

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

Yes, no diagnostic required; every compiler accepts (option 1). Every compiler accepts, and option 1 allows that result. The gap review reads the program as not valid. Option 2 would define a result that the review's reading leaves invalid.

Kept on 2026-11-03. The review's first reading needs a diagnostic, and every compiler accepts the program. Its second reading gives option 1, the answer. Tidal keeps option 1 until the maintainer decides, and the form with a diagnostic is in row 4 of `documentation/decisions/pending.md`. The question stays open under decision 0054, because the review reads with doubt.

Kept open on 2026-11-04 under decision 0054, because the gap review reads with doubt, and the form with a diagnostic waits on the maintainer (`documentation/decisions/pending.md`, row 4).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) reads against the answer, with doubt. The second lookup of a qualified name into the current instantiation makes the lookup ambiguous whenever its result differs, also when it finds one different member. An ambiguous lookup needs a diagnostic, so this reading gives option 3, which no compiler follows. Under a second reading, the general rule for an independent construct whose meaning changes at the instantiation applies, which needs no diagnostic and gives option 1. Option 2 has no support. Clang and EDG treat only a second result with two members as ambiguous (Q-0230).

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
