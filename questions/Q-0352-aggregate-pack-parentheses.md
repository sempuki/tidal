# Q-0352: Does a list in parentheses form the aggregate candidate for a class with a pack of bases?

**Status:** settled
**Affects:** CTAD-9, CTAD-27, `tests/templates/class-template-deduction/aggregate-pack-parentheses.cpp`, `tests/templates/class-template-deduction/aggregate-pack-length-parentheses.cpp`

## Context

`template <class... F> struct Both : F... {};` and then
`Both both(Left{}, Right{});`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, `Both<Left, Right>` |
| EDG 7.0 | accepts, `Both<Left, Right>` |
| GCC 14, 16, host | rejects: class template argument deduction failed |

Every compiler accepts the same list in braces, and
`Both<Left, Right> both(Left{}, Right{});` with the type given. Every
compiler accepts a list in parentheses for a class with one dependent
base that is not a pack. GCC also rejects the list in parentheses when
an earlier base `List<T...>` fixes the length of the pack, with as many
later elements as bases or with fewer.

## Evidence

The two tests above, and probes that compare braces and parentheses
for each of these classes on the baseline compilers.

## Options

1. A list in parentheses forms the aggregate candidate for a class with
   a pack of bases, as a list in braces does (Clang, EDG).
2. A list in parentheses forms no aggregate candidate for such a class
   (GCC).

## Answer

Yes; the candidate is formed as for braces (option 1). Clang and EDG
agree, and CTAD-9 forms the candidate from a list in parentheses for
every other aggregate.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and CTAD-P1 gives the same answer: the same steps form the guides, the aggregate candidate among them, for every initializer. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. The aggregate candidate is formed from a parenthesized expression list under the same conditions as from a braced list, and the treatment of a trailing pack of bases does not depend on the kind of list. GCC's rejection is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
