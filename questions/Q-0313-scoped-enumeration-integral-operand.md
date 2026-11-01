# Q-0313: Can an enumerator of a scoped enumeration give the operand of `alignas`, the width of a bit-field, or the size in an array `new`?

**Status:** provisional
**Affects:** ALN-8, BF-2, NEW-2, `tests/declarations/alignment/operand-scoped-enumeration.cpp`, `tests/classes/unions/bitfield-width-scoped-enumeration.cpp`, `tests/expressions/new-delete/array-scoped-enumeration-size.cpp`

## Context

A scoped enumeration has no implicit conversion to an integer type.
Three places take an integer value and split on such an enumerator:

| Place | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| `alignas(Width::eight)` | rejects | rejects | accepts |
| a bit-field width `Bits::three` | rejects | rejects | accepts |
| `new int[Count::two]` | rejects | accepts | rejects |

Every compiler accepts an enumerator of an unscoped enumeration in all
three places.

## Evidence

The three tests above. Clang says that the operand must have integral
or unscoped enumeration type. GCC says that the `alignas` operand and
the width have a non-integral type. EDG says that the `new` size must
have integral or unscoped enumeration type.

## Options

1. A scoped enumeration is not valid in any of the three places. Each
   place takes a value of integral or unscoped enumeration type, as
   Clang does everywhere.
2. Each place converts a scoped enumerator to its integer value.

## Provisional answer

Option 1. Two of three compilers reject the enumerator in each place,
and option 1 gives the three places one rule. The tests expect a
rejection and record GCC's and EDG's results with overrides.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
