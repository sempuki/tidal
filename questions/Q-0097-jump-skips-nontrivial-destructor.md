# Q-0097: Can a jump skip a variable whose destructor is not trivial?

**Status:** provisional
**Affects:** SW-5, JMP-6, `tests/statements/switch/case-skips-nontrivial-destructor.cpp`, `tests/statements/jumps/goto-skips-nontrivial-destructor.cpp`

## Context

A case label or a goto skips the declaration, without an initializer, of a variable whose class has a user-provided destructor and a trivial default constructor. Clang and EDG reject the jump. GCC 14 and 16 accept it.

## Options

1. The jump is not valid.

2. The jump is valid.

## Provisional answer

Option 1. Clang and EDG agree, and JMP-6 lists only types whose destructor is trivial.

## Opinions
