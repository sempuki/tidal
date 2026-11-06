# Q-0413: Is a call rejected when the implicit conversion of an argument to a parameter calls a consteval constructor that is not a constant expression?

**Status:** provisional
**Affects:** CEV-1, `tests/declarations/constexpr/consteval-conversion-for-parameter-not-constant.cpp`
**Source:** roadmap round 8 programmer reduction `consteval_converting_constructor_not_constant.cpp` (`reports/clean-room/roadmap-8/programmer.md`, section 1.5), from the probe that a clean-room writer ran for it.
**Waits for:** a first review

## Context

A class has a `consteval` converting constructor that calls a function that is not `constexpr` for a negative value. A call passes `-1` to a parameter of that class. EDG rejects the same failure when the conversion initializes a variable (`consteval-converting-constructor-throws.cpp`).

| Implementation | Result |
|---|---|
| Clang 22 | rejects the call |
| GCC 16 | rejects the call |
| EDG 7.0 | accepts |

## Evidence

The test in **Affects**, with an override for EDG.

## Options

1. The conversion calls the `consteval` constructor outside a constant evaluation that can succeed, so the call is rejected (Clang, GCC).
2. The program is accepted (EDG).

## Provisional answer

Rejected (option 1). Two of three families agree, and CEV-1 counts a `consteval` constructor that an implicit conversion calls.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
