# Q-0431: Must an implementation reject a declared `std::initializer_list` whose count comes before the pointer?

**Status:** provisional
**Affects:** LSUP-1, `tests/foundations/language-support/initializer-list-count-before-pointer.cpp`
**Source:** the P2 report (`reports/clean-room/P2/report.md`, P2-6), the round 11 writer probe `reports/clean-room/roadmap-11/writer-probes/p2-points.json`.
**Waits for:** a first review

## Context

A program declares `std::initializer_list` itself (LSUP-1) and then
initializes a parameter of that type from a braced list.

| Shape | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| a pointer, then a count of type `decltype(sizeof 0)`, private constructor | accepts | accepts | accepts |
| a count, then a pointer | rejects | rejects | accepts |
| a pointer, then a count of type `unsigned int` | rejects | rejects | accepts |

## Evidence

The test in **Affects**, and the probe in **Source**.

## Options

1. The shape of LSUP-1 is fixed: the pointer comes first, and the count
   has the type of `sizeof`. Another shape is rejected (Clang, GCC).
2. An implementation can accept other orders and count types (EDG).

## Provisional answer

The shape of LSUP-1 is fixed, and another shape is rejected (option 1).
Clang and GCC reject the other shapes, and EDG alone accepts them.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
