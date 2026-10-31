# Q-0075: Can a name with C language linkage match a global variable?

**Status:** provisional
**Affects:** LSP-4, `tests/declarations/linkage-specifications/c-name-conflicts-global.cpp`

## Context

A namespace declares `extern "C" int clash;` and the global namespace declares a variable `clash` with C++ linkage. The two names refer to the same symbol name. Clang and EDG reject the program. GCC accepts it.

## Options

1. The program is not valid.
2. The two declarations are separate entities.

## Provisional answer

Option 1. Clang and EDG agree, and the two names would collide when linked.

## Opinions
