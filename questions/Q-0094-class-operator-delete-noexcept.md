# Q-0094: Is a class-specific operator delete without a specifier non-throwing?

**Status:** provisional
**Affects:** EXS-4, `tests/declarations/exception-specifications/class-delete-operator-noexcept.cpp`

## Context

A class declares a static `operator delete(void*, size)` with no
`noexcept` specifier. Clang and EDG treat a direct call of it as
non-throwing. GCC treats the call as potentially throwing.

## Options

1. A deallocation function is non-throwing without the keyword.
2. The function follows the general rule and is potentially throwing.

## Provisional answer

Option 1. Clang and EDG agree.

## Opinions
