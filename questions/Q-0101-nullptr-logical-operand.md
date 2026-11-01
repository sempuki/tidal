# Q-0101: Can nullptr be an operand of a logical operator?

**Status:** compiler defect
**Affects:** LOG-12, `tests/expressions/logical/nullptr-operand.cpp`

## Context

`!nullptr` and `nullptr && b` convert `nullptr` to `bool` by contextual conversion, which gives `false`. GCC 16, Clang 18 and 22, and EDG 7.0 accept both expressions, and so do the host GCC 16.2 and Clang 22.1. GCC 14 rejects both.

## Evidence

`tests/expressions/logical/nullptr-operand.cpp`, with an override for
GCC 14.

## Options

1. The operand is valid, as contextual conversion to `bool` permits.

2. The operand is not valid.

## Provisional answer

Option 1. The newest version of every compiler in the matrix accepts
the operand. GCC 14 has a defect that GCC 16 fixes. The override for
GCC 14 stays while GCC 14 is in the matrix.

## Opinions
