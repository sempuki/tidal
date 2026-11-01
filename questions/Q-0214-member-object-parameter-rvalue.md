# Q-0214: Is the object parameter of a member with no ref-qualifier an rvalue reference in partial ordering against an rvalue reference?

**Status:** provisional
**Affects:** ORD-6, `tests/templates/ordering/member-object-rvalue.cpp`

## Context

`X{} + 1` has two candidates:

- the member `template <class T> One X::operator+(T)`, with no
  ref-qualifier,
- the non-member `template <class T> Two operator+(X&&, T)`.

The sequences tie, because the implicit object parameter of a member
with no ref-qualifier binds an rvalue with no preference. Partial
ordering gives the member an object parameter. The kind of that
reference decides the result.

| Compiler | Result |
|---|---|
| Clang 22, host | ambiguous |
| GCC 14, 16, host | ambiguous |
| Clang 18 | the member |
| EDG 7.0 | the member |

With an object parameter `X&&`, both pairs are the same in both
directions, and neither template is more specialized. With `X&`, the
lvalue reference wins by ORD-10, and the member is chosen.

## Evidence

`tests/templates/ordering/member-object-rvalue.cpp`, with overrides for
Clang 18 and EDG. Item E6 of the gap review of 2026-10-31
(`reports/review/2026-10-31-gap-review-deduction-and-ordering.md`)
describes the rvalue reference.

## Options

1. The object parameter is an rvalue reference when the other
   template's first parameter is an rvalue reference (Clang 22, GCC).
2. The object parameter is always an lvalue reference for a member with
   no ref-qualifier (Clang 18, EDG).

## Provisional answer

Yes; Clang 22 and GCC agree, Clang 18 and EDG differ (option 1). The current Clang and GCC agree, and the gap review describes
the same behavior. Clang changed from option 2 to option 1 between
versions 18 and 22.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
