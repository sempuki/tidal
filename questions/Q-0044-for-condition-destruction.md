# Q-0044: Is a `for` condition variable destroyed before or after the iteration expression?

**Status:** compiler defect
**Affects:** LOOP-3, `tests/statements/loops/for-condition-destroyed-after-step.cpp`

## Context

In `for (init; T c = cond(); step) body`, the condition declares a
variable on each iteration. At run time, Clang 22 and 18 and GCC 16
destroy the variable after the iteration expression `step`. GCC 14
destroys it before `step`. EDG 7.0 does not link, so the run test
shows nothing for EDG.

During constant evaluation the compilers differ more:

| Compiler | Order of events, two passes and a final check |
|---|---|
| Clang 22, GCC 16, host | body, step, destruction, body, step, destruction, destruction |
| GCC 14 | body, destruction, step, body, destruction, step, destruction |
| Clang 18 | as Clang 22, without the last destruction |
| EDG 7.0 | destruction, body, step, destruction, body, step, destruction |

EDG destroys the condition variable before the body, at the same
place as a temporary in the condition.

## Evidence

`tests/statements/loops/for-condition-destroyed-after-step.cpp` at run
time. A probe of 2026-11-01 runs the same loop in a `constexpr`
function, with the condition variable of a class with a `constexpr`
destructor. The probe records each event in an array and checks each
element with `static_assert`, so EDG takes part. The probe gives the
same order for an aggregate in braces and for a copy initialization.

## Options

1. After the iteration expression. GCC 14's order is a defect fixed in
   GCC 16.
2. The order is unspecified.

## Provisional answer

After; GCC 14 is fixed in GCC 16 (option 1). The newest Clang and the newest GCC agree at run time.

The question stays open under decision 0051. EDG 7.0, the newest EDG,
destroys the variable before the body during constant evaluation, and
the matrix has no run-time result for EDG.

## Opinions
