# Q-0376: What type does the index of `get` have in the tuple protocol of a structured binding?

**Status:** provisional
**Affects:** SB-5, `tests/declarations/structured-bindings/tuple-index-size-type.cpp`
**Waits for:** a use of SB-P2

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

Kept open on 2026-11-04 under decision 0055, because SB-5 changed in the commit that read the review, in the condition for the member form of `get`, and no reader or review has used the changed rule. The gap review of the rules that phase 7 rewrote supports the answer with no doubt. GCC 16 is the one newest family that differs.

Kept open on 2026-11-05 under decision 0054, because the use of SB-P2 did not give the provisional answer, so the principle needs a repair and a new use (gap review of the text phase 9 wrote, item RV3).

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that the phase 7 front-end and Propose repairs rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`) agrees with option 1. The reference defines the index as a prvalue of type `std::size_t`.

### Gap review, 2026-11-05 (declarations, statements, and foundations)

The source-reader gap review of the text phase 9 wrote on declarations, statements, and foundations (`reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`) used SB-P2 on ``declarations/structured-bindings/tuple-index-size-type.cpp``, without the rule text. The model's answer: The principle says the elements are the results of get, and does not give the type of the index. No answer. Matches the provisional answer: no (not decided). Reference verdict: supports the provisional answer with no doubt.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
