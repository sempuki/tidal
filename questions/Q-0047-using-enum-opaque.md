# Q-0047: May `using enum` name an enumeration that is declared but not defined?

**Status:** provisional
**Affects:** ENUM-9, `tests/declarations/enumerations/using-enum-opaque.cpp`

## Context

`enum class E : int; using enum E;` names an enumeration whose enumerators are not yet known. Clang and GCC reject the `using enum`. EDG 7.0 accepts it.

## Options

1. Rejected: the enumeration must be complete.
2. Valid.

## Provisional answer

No; the enumeration must be complete, so the `using enum` is rejected (option 1). Clang and GCC, in both versions, reject the `using enum`.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. ENUM-P2 says that `using enum` brings the enumerators into a scope, and it does not say that the enumeration must be complete. EDG 7.0 accepts the declaration.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The enumeration that `using enum` names must have a reachable definition with its list of enumerators. An opaque declaration gives no such list, so EDG's acceptance is a defect.
