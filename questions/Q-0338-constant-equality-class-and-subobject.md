# Q-0338: During constant evaluation, does the address of a class object compare equal to the address of its first member or of its base?

**Status:** settled
**Affects:** CLS-30, `tests/classes/members/standard-layout-address-constant.cpp`

## Context

A `constexpr` object `entry` of a standard-layout class has a base `Tag`
and a first member `key`. A constant expression converts `&entry`,
`&entry.key`, and the address of the base to `const void*` and
compares them with `==`.

| Comparison | Clang 18, 22 | GCC 14, 16 | EDG 7.0 |
|---|---|---|---|
| object and first member | `true` | `true` | `false` |
| object and base | `true` | `true` | `false` |
| array and first element | `true` | `true` | `true` |
| first and second member (`!=`) | `true` | `true` | `true` |

At run time the addresses are equal (CLS-30). EDG gives `false` also
for a local object inside a lambda that constant evaluation calls.

## Evidence

The test in **Affects**, with an override for EDG. Probes of
2026-11-03 with the comparisons in the table.

## Options

1. `true`, as at run time. A constant evaluation gives the same result
   as the run.
2. `false` or `true`. The comparison of pointers to two different
   objects at one address is not fixed.

## Answer

True (option 1). CLS-30 gives the addresses, and no rule makes the
comparison unspecified. EDG's `false` is a possible defect.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of item O (`reports/review/2026-11-03-gap-review-item-o.md`) supports this answer with no doubt, and CLS-P4 gives the same answer: the object, each base subobject, and the first member share one address. EDG 7.0 still differs. Tidal records EDG's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of item O (`reports/review/2026-11-03-gap-review-item-o.md`) reads the reference as supporting option 1, the provisional answer. Two pointers that hold one address compare equal, and the comparison has a specified result, so constant evaluation gives `true`. EDG's `false` conflicts with that reading.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
