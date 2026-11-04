# Q-0376: What type does the index of `get` have in the tuple protocol of a structured binding?

**Status:** provisional
**Affects:** SB-5, `tests/declarations/structured-bindings/tuple-index-size-type.cpp`

## Context

SB-5 calls `e.get<i>()` or `get<i>(e)` for element `i`. A member `get`
declared as `template <auto I>` shows the type of the index through
`decltype(I)`.

| Compiler | Type of `I` |
|---|---|
| Clang 18, 22, host | `std::size_t` |
| EDG 7.0 | `std::size_t` |
| GCC 14, 16, host | `int` |

Every compiler accepts a member `get` declared as `template <int I>`
(`tests/declarations/structured-bindings/tuple-index-int-parameter.cpp`),
because the index converts to `int` as a template argument.

## Evidence

The test in Affects, and a probe that asserts `decltype(I)` is `int`,
which Clang and EDG reject and GCC accepts.

## Options

1. The index is a constant of type `std::size_t` (Clang, EDG). The type
   matches the first parameter of `std::tuple_element` and the type of
   `std::tuple_size<E>::value`.
2. The index is an `int` constant (GCC).

## Provisional answer

`std::size_t` (option 1). Two of three families agree, and the type
matches the parameter of `std::tuple_element` that SB-5 also uses with
the index.

Kept open on 2026-11-03 under decision 0054, because no review has read the provisional answer. GCC 16 is the one newest family that differs.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
