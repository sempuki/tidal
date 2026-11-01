# Q-0218: Is the object parameter of a swapped member candidate added before its parameters are reversed?

**Status:** provisional
**Affects:** ORD-7, `tests/templates/ordering/reversed-member-object.cpp`

## Context

`p == x`, with `p` an `int*` and `x` a `const X`, has two candidates
that are templates:

- the member `template <class T> bool X::operator==(T* const&) const`,
  as a swapped candidate,
- the non-member `template <class T> bool operator==(const T&, const X&)`.

The sequences tie. The member gets the object parameter `const X&`
(ORD-6), and its parameters are `(const X&, T* const&)`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | the member |
| GCC 14, 16, host | the non-member |
| EDG 7.0 | the non-member |

If the object parameter is added first and the two parameters are then
reversed, the member compares as `(T* const&, const X&)`, which is more
specialized than the non-member. Without the reversal, or with the
reversal before the object parameter, neither is more specialized, and
the written candidate wins (OVL-32). GCC does not reverse the
parameters of a swapped candidate at all (Q-0211). EDG reverses them
for a non-member swapped candidate.

## Evidence

`tests/templates/ordering/reversed-member-object.cpp`, with overrides
for GCC and EDG. Item O11 of the gap review of 2026-10-31
(`reports/review/2026-10-31-gap-review-deduction-and-ordering.md`)
describes option 1.

## Options

1. The object parameter is added, and then the two parameters are
   reversed (Clang).
2. The parameters of a swapped member are compared in the other order,
   or not reversed (GCC, EDG).

## Provisional answer

Yes; Clang, GCC and EDG differ (option 1). The gap review describes it, and Clang implements it. The
reversal of Q-0211 pairs the parameters that meet the same operand.
After the object parameter is added and the two are reversed, each
parameter meets the operand at its own position in `p == x`.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
