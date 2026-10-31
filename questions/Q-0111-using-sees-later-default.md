# Q-0111: Does a using declaration see a default argument that a later namespace declaration adds?

**Status:** provisional
**Affects:** DEFARG-2, `tests/declarations/functions/default-arguments/using-sees-later-default.cpp`

## Context

A namespace declares a function with one parameter. A using declaration brings it into the global scope. A later declaration in the namespace adds a default argument. An unqualified call with no arguments follows. GCC and EDG accept the call. Clang rejects it.

## Options

1. The default argument is usable through the using declaration.

2. The using declaration sees only the default arguments declared before it.

## Provisional answer

Option 1. GCC and EDG agree.

## Opinions
