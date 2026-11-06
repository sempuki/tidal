# Q-0413: Is a call rejected when the implicit conversion of an argument to a parameter calls a consteval constructor that is not a constant expression?

**Status:** settled
**Affects:** CEV-1, `tests/declarations/constexpr/consteval-conversion-for-parameter-not-constant.cpp`
**Source:** roadmap round 8 programmer reduction `consteval_converting_constructor_not_constant.cpp` (`reports/clean-room/roadmap-8/programmer.md`, section 1.5), from the probe that a clean-room writer ran for it.

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

## Answer

Rejected (option 1). Two of three families agree, and CEV-1 counts a `consteval` constructor that an implicit conversion calls.

Settled on 2026-11-05 under decision 0054. The gap review of the text phase 10 wrote (item RV4) supports the answer with no doubt. At most one newest family differs in each form, and each test keeps its override.

## Opinions

### Gap review, 2026-11-05 (phase 10 text, classes, declarations, foundations, overloading, and statements)

The source-reader gap review of the text phase 10 wrote on classes, declarations, foundations, overloading, and statements (`reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`) reads the question: supports with no doubt. The conversion of the argument calls an immediate constructor outside an immediate context, so it is an immediate invocation. The call is not constant for the value given, and main cannot escalate, so the program is not valid.
