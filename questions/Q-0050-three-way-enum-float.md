# Q-0050: May `<=>` compare an unscoped enumeration with a floating-point value?

**Status:** provisional
**Affects:** CMP-18, `tests/expressions/comparison/three-way-enum-float.cpp`

## Context

Clang and GCC reject `<=>` between an unscoped enumeration value and a floating-point value. EDG 7.0 accepts it. All accept `<=>` between an unscoped enumeration and an integer.

## Options

1. Rejected.
2. Accepted, after the usual arithmetic conversions.

## Provisional answer

Option 1. Clang and GCC, in both versions, reject the comparison.

## Opinions
