# Q-0096: Can a case or default label be the last item of a block?

**Status:** settled
**Affects:** STMT-5, `tests/statements/blocks/case-label-ends-block.cpp`, `tests/statements/blocks/default-label-ends-block.cpp`

## Context

A switch body ends with a case label or a default label, with no statement after it. Clang and EDG accept the program. GCC 14 and 16 reject it, though they accept an identifier label in that place. The host GCC 16.2 rejects it too, so no newer GCC settles the split.

## Options

1. Any label can be the last item of a block.

2. Only an identifier label can.

## Answer

Yes; any label can be the last item of a block (option 1). Clang and EDG agree, and option 1 matches the rule for identifier labels.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) supports this answer with no doubt, and STMT-P1 gives the same answer: a label marks a position, and the end of a block is a position, so a label can stand there. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. The grammar of a block allows labels after the last statement, and case and default labels are labels. Such a label acts as if a null statement followed it. GCC's rejection is a defect.
