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

No; rejected (option 1). Clang and GCC, in both versions, reject the comparison.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. CMP-P5 lists the operand forms that `<=>` rejects, and an unscoped enumeration against a floating-point value is not in the list. EDG 7.0 is the only newest family that differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. `<=>` uses the usual arithmetic conversions only when both operands are arithmetic, or when one is an integer and the other an unscoped enumeration. An enumeration against a floating-point value fits no case, so the program is not valid.
