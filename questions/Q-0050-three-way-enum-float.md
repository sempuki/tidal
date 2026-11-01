# Q-0050: May `<=>` compare an unscoped enumeration with a floating-point value?

**Status:** provisional
**Affects:** CMP-18, `tests/expressions/comparison/three-way-enum-float.cpp`

## Context

Clang 22 and 18 and GCC 16 and 14 reject `<=>` between an unscoped
enumeration value and a floating-point value. EDG 7.0 accepts it. All
accept `<=>` between an unscoped enumeration and an integer. All reject
`<=>` between two different unscoped enumerations.

EDG's acceptance is general:

- EDG accepts the enumeration on either side, with `float` or `double`,
  and with a fixed underlying type.
- EDG gives `std::partial_ordering`, and `A <=> 1.5` is `less` when `A`
  is `1`.

The relational operators accept an enumeration and a floating-point
value on every compiler, with a deprecation warning (CMP-21).

## Evidence

The test in Affects, and probes of the forms above on every compiler
in the matrix. EDG 7.0 is the only EDG version in the matrix.

## Options

1. Rejected.
2. Accepted, after the usual arithmetic conversions.

## Provisional answer

Option 1. Clang and GCC, in both versions, reject the comparison.

## Opinions
