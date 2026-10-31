# Q-0063: Is a floating literal outside the range of its type valid?

**Status:** provisional
**Affects:** PRIM-15, `tests/expressions/primary/float-overflow.cpp`

## Context

The literal `1e999f` is larger than the largest `float`. Clang and GCC
accept it with a warning, also under `-pedantic-errors`, and give
infinity. EDG 7.0 rejects it.

## Options

1. The value is implementation-defined. An implementation can reject the
   literal.
2. The program is not valid, and implementations must reject it.

## Provisional answer

Option 1. Two of the three implementations accept the literal in their
strictest mode.

## Opinions

- The second gap review
  (`reports/review/2026-10-31-gap-review-expr-decl.md`, a summary by a
  source reader) holds that such a literal is not valid (option 2).
  Clang and GCC still accept the literal in their strictest mode, so the
  provisional answer stays option 1 until more opinions arrive.
