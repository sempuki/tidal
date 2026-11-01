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

## Opinions
