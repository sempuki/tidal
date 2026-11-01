# Q-0223: Does an implementation diagnose an independent error in a template that is never instantiated?

**Status:** provisional
**Affects:** DEP-30, `tests/templates/dependent-names/independent-error-unused-conversion.cpp`, `tests/templates/dependent-names/independent-error-unused-call.cpp`

## Context

A function template that is never instantiated holds a construct with no
dependent operand that is not valid for any argument, such as
`int* p = 1;` or a call `take(1)` where `take` takes `int*`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects the definition |
| EDG 7.0 | rejects the definition |
| GCC 14, 16, host | accepts the definition |

Every compiler rejects an undeclared name and a name that only a
dependent base declares in such a template.

## Evidence

The two tests above, with an override for GCC. The tests
`independent-error-unused-name.cpp` and
`independent-error-unused-base-member.cpp` show the cases where every
compiler rejects. Inventory item B3 describes the rule as one that needs
no diagnostic.

## Options

1. The error needs no diagnostic while no specialization is made. Both
   results are allowed.
2. The implementation must diagnose the error at the definition. GCC is
   wrong.

## Provisional answer

Option 1. DEP-30 is a `no-diagnostic` rule, so each compiler's result
lies within the rule. A program that relies on GCC's result is not
valid.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
