# Q-0048: If `std::tuple_size<E>` is complete but has no `value`, do structured bindings use the data members?

**Status:** provisional
**Affects:** SB-5, `tests/declarations/structured-bindings/tuple-size-without-value.cpp`

## Context

Clang and GCC fall back to binding the data members of `E` when `std::tuple_size<E>` is a complete type without a `value` member. EDG 7.0 rejects the declaration.

## Options

1. Fall back to the data members (SB-3).
2. Reject the declaration.

## Provisional answer

Yes; the bindings fall back to the data members (SB-3) (option 1). Clang and GCC, in both versions, fall back.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. SB-P2 gives the order of the tests, and it does not say what happens when `std::tuple_size<E>` is complete and has no `value` member. EDG 7.0 rejects the declaration.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The tuple protocol applies only when `std::tuple_size<E>` is complete and has a member named `value`. Without that member, the bindings use the data members.
