# Q-0093: Is a declaration of only a class key and a qualified name valid?

**Status:** provisional
**Affects:** TYN-7, `tests/declarations/type-names/elaborated-qualified-declaration-only.cpp`

## Context

After `namespace n { struct S; }`, the declaration `struct n::S;`
consists only of a class key and a qualified name. Clang and EDG reject
it. GCC accepts it.

## Options

1. The declaration is not valid.
2. The declaration is valid and has no effect.

## Provisional answer

No; the declaration is not valid (option 1). Clang and EDG agree.

## Opinions
