# Q-0211: Are the parameters of a swapped candidate reversed for partial ordering?

**Status:** provisional
**Affects:** ORD-7, `tests/templates/ordering/reversed-candidate.cpp`

## Context

`x == p` has a written candidate from
`template <class T> bool operator==(const X&, const T&)` and a swapped
candidate from `template <class T> bool operator==(T* const&, const X&)`.
The sequences tie, and both are templates.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | the swapped candidate |
| EDG 7.0 | the swapped candidate |
| GCC 14, 16, host | the written candidate |

With its parameters reversed, the swapped template takes
`(const X&, T* const&)`, which is more specialized than
`(const X&, const T&)`. Without the reversal, neither template is more
specialized, and the preference for a written candidate
(OVL-32) chooses the written one.

## Evidence

`tests/templates/ordering/reversed-candidate.cpp`, with an override for
GCC. The sanitized Clang source (clang-sema-deduction) reverses the
compared parameters of a reversed candidate. Item L7 of the deduction
inventory describes the behavior.

## Options

1. Partial ordering reverses the parameters of a swapped candidate
   (Clang, EDG).
2. Partial ordering compares the parameters in declared order (GCC).

## Provisional answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations and the inventory agree. The
reversal compares the parameters that meet the same operand.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
