# Q-0107: Can constinit apply to a structured binding declaration?

**Status:** provisional
**Affects:** SB-11, `tests/declarations/structured-bindings/constinit-specifier.cpp`

## Context

A structured binding declaration at namespace scope has `constinit`. Clang and GCC 16 reject the declaration. EDG 7.0 and GCC 14 accept it.

## Options

1. Not valid; only static, thread_local, const, and volatile can occur.

2. Valid.

## Provisional answer

No; only static, thread_local, const, and volatile can occur (option 1). The newer GCC agrees with Clang.

## Opinions
