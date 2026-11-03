# Q-0189: Does the result tie-breaker compare a constructor with a conversion function?

**Status:** provisional
**Affects:** OVL-26, `tests/foundations/overload-resolution/tie-conversion-result-constructor.cpp`

## Context

`Target` has a constructor from `const Source&`. `Source` has a
conversion function to `Derived`, a class derived from `Target`. A
`Target` is copy-initialized from a `Source` lvalue. Both candidates bind
`const Source&` to the lvalue, so the argument sequences tie. The
constructor's result is a `Target`. The conversion function's result
needs a derived-to-base conversion.

| Compiler | Copy initialization | Argument of a function |
|---|---|---|
| Clang 18, Clang 22 | ambiguous | ambiguous |
| GCC 14, GCC 16 | the constructor | the constructor |
| EDG 7.0 | the constructor | the constructor |

When both results are a `Target`, every compiler finds the two
ambiguous.

## Evidence

`tests/foundations/overload-resolution/tie-conversion-result-constructor.cpp`
and `init-constructor-conversion-tie.cpp`.

## Options

1. The result tie-breaker compares a constructor, whose result has the
   identity, with a conversion function (GCC, EDG).
2. The result tie-breaker compares only two conversion functions
   (Clang).

## Provisional answer

Yes; the constructor's result has the identity; GCC and EDG agree, Clang differs (option 1). Two of three implementations agree.

The question stays open under decision 0051 (2026-11-03). Clang 22 finds the two ambiguous, and GCC 16 and EDG 7.0 choose the constructor, so two families disagree. A probe of 2026-11-03 gives the same split for a function argument. The provisional answer follows OVL-26 with ICS-27. The constructor's result is the target itself, which is the identity, and the conversion function's result needs a derived-to-base conversion, which has conversion rank.

Kept on 2026-11-03. The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) finds the reference unclear. A strict reading, in which the result tie-breaker needs a return type, gives option 2. The reference also says that the chosen constructor gives a value of the target type, which fits option 1. With the reference unclear, the answer keeps GCC 16, EDG 7.0, and OVL-26 with ICS-27, where the constructor's result is the target itself.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) finds the reference unclear. The result tie-breaker compares the conversions from each candidate's return type, and a constructor has no return type. If the constructor's result counts as the target type, option 1 follows. If the tie-breaker needs a return type, option 2 follows. The reference gives no example that settles the point.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
