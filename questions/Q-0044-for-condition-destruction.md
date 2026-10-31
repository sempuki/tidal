# Q-0044: Is a `for` condition variable destroyed before or after the iteration expression?

**Status:** compiler defect
**Affects:** LOOP-3, `tests/statements/loops/for-condition-destroyed-after-step.cpp`

## Context

In `for (init; T c = cond(); step) body`, the condition declares a variable
on each iteration. Clang 22 and 18, GCC 16, and EDG 7.0 destroy it after the
iteration expression `step`. GCC 14 destroys it before `step`.

## Options

1. After the iteration expression. GCC 14's order is a defect fixed in
   GCC 16.
2. The order is unspecified.

## Provisional answer

Option 1. The newest version of every implementation agrees.

## Opinions
