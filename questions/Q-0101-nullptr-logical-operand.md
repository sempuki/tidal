# Q-0101: Can nullptr be an operand of a logical operator?

**Status:** provisional
**Affects:** LOG-12, `tests/expressions/logical/nullptr-operand.cpp`

## Context

`!nullptr` and `nullptr && b` convert `nullptr` to `bool` by contextual conversion, which gives `false`. GCC 16, Clang 18 and 22, and EDG 7.0 accept it. GCC 14 rejects it.

## Options

1. The operand is valid, as contextual conversion to `bool` permits.

2. The operand is not valid.

## Provisional answer

Option 1. The newer GCC agrees with Clang and EDG.

## Opinions
