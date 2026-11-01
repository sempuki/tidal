# Q-0096: Can a case or default label be the last item of a block?

**Status:** provisional
**Affects:** STMT-5, `tests/statements/blocks/case-label-ends-block.cpp`, `tests/statements/blocks/default-label-ends-block.cpp`

## Context

A switch body ends with a case label or a default label, with no statement after it. Clang and EDG accept the program. GCC 14 and 16 reject it, though they accept an identifier label in that place. The host GCC 16.2 rejects it too, so no newer GCC settles the split.

## Options

1. Any label can be the last item of a block.

2. Only an identifier label can.

## Provisional answer

Yes; any label can be the last item of a block (option 1). Clang and EDG agree, and option 1 matches the rule for identifier labels.

## Opinions
