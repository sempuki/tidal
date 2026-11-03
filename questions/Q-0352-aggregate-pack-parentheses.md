# Q-0352: Does a list in parentheses form the aggregate candidate for a class with a pack of bases?

**Status:** provisional
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

## Provisional answer

Yes; the candidate is formed as for braces (option 1). Clang and EDG
agree, and CTAD-9 forms the candidate from a list in parentheses for
every other aggregate.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
