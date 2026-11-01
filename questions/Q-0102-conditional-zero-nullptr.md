# Q-0102: Does a conditional with the literal 0 and nullptr have the type of nullptr?

**Status:** provisional
**Affects:** COND-3, `tests/expressions/conditional/zero-and-nullptr.cpp`

## Context

In `c ? 0 : nullptr`, one operand is the literal `0` and the other is `nullptr`. Clang and EDG accept the expression and give the result the type of `nullptr`. GCC rejects it and reports that the operand types differ.

## Options

1. The result has the type of `nullptr`.

2. The program is not valid.

## Provisional answer

Yes; the result has the type of `nullptr` (option 1). Clang and EDG agree.

## Opinions
