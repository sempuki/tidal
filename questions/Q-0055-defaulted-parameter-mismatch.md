# Q-0055: Is a defaulted copy or move constructor with a different parameter type deleted or invalid?

**Status:** provisional
**Affects:** CPY-14, `tests/classes/copy-move/defaulted-volatile-param.cpp`, `tests/classes/copy-move/defaulted-move-const-param.cpp`

## Context

A class can default a copy constructor with a `const volatile` reference
parameter, or a move constructor with a `const` rvalue reference
parameter. These types differ from the types of the implicit functions.
GCC 16, Clang 18 and 22, and EDG 7.0 accept the declaration and define the
function as deleted. A use of it is rejected. GCC 14 rejects the
declaration itself.

## Options

1. The function is deleted. Only a use is rejected.
2. The declaration is not valid.

## Provisional answer

Option 1. The newer GCC agrees with Clang and EDG, so the behavior of
GCC 14 is treated as an older implementation choice.

## Opinions
