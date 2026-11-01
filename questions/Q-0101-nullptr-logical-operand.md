# Q-0101: Can nullptr be an operand of a logical operator?

**Status:** settled
**Affects:** LOG-12, `tests/expressions/logical/nullptr-operand.cpp`

## Context

`!nullptr` and `nullptr && b` convert `nullptr` to `bool` by contextual conversion, which gives `false`. GCC 16, Clang 18 and 22, and EDG 7.0 accept both expressions, and so do the host GCC 16.2 and Clang 22.1. GCC 14 rejects both.

## Evidence

`tests/expressions/logical/nullptr-operand.cpp`, with an override for
GCC 14.

## Options

1. The operand is valid, as contextual conversion to `bool` permits.

2. The operand is not valid.

## Answer

Yes; GCC 14's rejection is fixed in GCC 16 (option 1). The newest version of every compiler in the matrix accepts
the operand. GCC 14 has a defect that GCC 16 fixes. The override for
GCC 14 stays while GCC 14 is in the matrix.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. GCC 14 still rejects both expressions, and the test keeps its override for GCC 14.

## Opinions
