# Q-0145: Can copy-list-initialization use an explicit conversion function for the copy constructor's parameter?

**Status:** provisional
**Affects:** OVL-40, `tests/foundations/overload-resolution/init-copy-list-explicit-conversion.cpp`

## Context

`Source` has an `explicit` conversion function to `Target`. `Target` has
a default constructor and the implicit copy and move constructors. The
program writes `Target t = {s};`.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | accepts |
| GCC 14, GCC 16 | rejects |
| EDG 7.0 | rejects |

In direct-initialization, `Target t(s);` and `Target t{s};`, every
compiler uses the explicit conversion function (initialization grid).

## Evidence

`tests/foundations/overload-resolution/init-copy-list-explicit-conversion.cpp`,
and the row "copy list init" of the initialization grid. Item CI-4 of
the overload resolution inventory names direct-initialization only.

## Options

1. Copy-list-initialization takes only conversion functions that are
   not `explicit` (GCC, EDG).
2. The explicit conversion function can make the temporary (Clang).

## Provisional answer

No; GCC and EDG agree, Clang differs (option 1). Two of three implementations agree, and the inventory names
direct-initialization only.

The question stays open under decision 0051 (2026-11-03). Clang 22 accepts, and GCC 16 and EDG 7.0 reject, so two families disagree. The provisional answer follows ICS-45, which gives `explicit` conversion functions to direct initialization only. Copy-list-initialization is a form of copy initialization, and every compiler keeps `explicit` conversion functions out of copy initialization without braces (`init-copy-explicit-conversion.cpp`).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) agrees with option 1. The braced list chooses a constructor of the target, and the copy and move constructors take a reference parameter that is copy-initialized from the source. `explicit` conversion functions take part only in a direct initialization of a reference, or in making the temporary for a constructor's reference parameter during a direct initialization of the class. Copy-list-initialization is neither case.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
