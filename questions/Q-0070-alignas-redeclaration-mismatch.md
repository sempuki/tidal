# Q-0070: Must all declarations of a variable agree on alignas?

**Status:** provisional
**Affects:** ALN-3, `tests/declarations/alignment/redeclare-different.cpp`, `tests/declarations/alignment/definition-without-earlier-with.cpp`

## Context

A variable is declared with one `alignas` and defined with a different one, or declared with `alignas` and defined without it. Clang and EDG reject both. GCC accepts both.

GCC also accepts two declarations of one class with different `alignas`
(`tests/declarations/alignment/`, gap review 2026-10-31). Clang and EDG
reject them.

## Options

1. If a declaration has `alignas`, the definition must have the same alignment.
2. Declarations can differ.

## Provisional answer

Option 1. Clang and EDG agree.

## Opinions
