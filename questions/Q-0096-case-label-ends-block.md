# Q-0096: Can a case or default label be the last item of a block?

**Status:** provisional
**Affects:** STMT-5, `tests/statements/blocks/case-label-ends-block.cpp`, `tests/statements/blocks/default-label-ends-block.cpp`

## Context

A switch body ends with a case label or a default label, with no statement after it. Clang and EDG accept the program. GCC 14 and 16 reject it, though they accept an identifier label in that place.

## Options

1. Any label can be the last item of a block.

2. Only an identifier label can.

## Provisional answer

Option 1. Clang and EDG agree, and option 1 matches the rule for identifier labels.

## Opinions
