# Q-0397: Is a partial specialization with `int V` more specialized than one with a constrained `auto V`?

**Status:** provisional
**Affects:** ORD-17, ORD-14, `tests/templates/ordering/partial-specializations-int-against-constrained-auto.cpp`
**Source:** finding I2-7 of `reports/clean-room/I2/report.md`, from the probes that a clean-room writer ran for it.

## Context

ORD-17 gives an invented value the declared type of its parameter, and
an invented type for each placeholder in that type. For `int V` the
invented value has type `int`. For `auto V` it has a new invented
type, which `int V` does not accept. So a partial specialization with
`int V` is more specialized than one with `auto V` of the same pattern.

The test declares a primary template with `auto V`, a partial
specialization `Kind<V>` with `int V`, and a partial specialization
`Kind<V>` with `auto V` and `requires true`. Both partial
specializations match `Kind<1>`.

| Implementation | Chosen for `Kind<1>` |
|---|---|
| Clang 22 | the one with `int V` |
| GCC 16 | the one with `int V` |
| EDG 7.0 | the one with `requires true` |

Without the constrained partial specialization, every compiler chooses
the one with `int V` over the primary template
(`partial-specializations-int-against-auto.cpp`). The heads differ, so
the constraint does not decide under ORD-14 even if the patterns tie.

## Evidence

The tests in **Affects**, with an override for EDG.

## Options

1. The invented value for `auto V` has an invented type, so `int V` is
   more specialized (Clang, GCC).
2. The two partial specializations tie, and the constraint decides
   (EDG). Under ORD-14, the constraint then needs equivalent heads.

## Provisional answer

The one with `int V` (option 1). Two of three families agree, and
option 1 follows ORD-17. Option 2 would also need constraints to decide
between heads that are not equivalent, which ORD-13 and ORD-14 do not
allow (Q-0146).

The question stays open under decision 0051, because EDG 7.0 differs
from Clang 22 and GCC 16.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
