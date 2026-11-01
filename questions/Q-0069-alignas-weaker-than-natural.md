# Q-0069: Can alignas make an alignment weaker than the natural one?

**Status:** provisional
**Affects:** ALN-2, `tests/declarations/alignment/weaker-than-natural.cpp`, `tests/declarations/alignment/weaker-on-class-member.cpp`

## Context

A declaration such as `alignas(1) int x;` asks for an alignment weaker than the alignment of `int`. Clang 18 and 22 and EDG 7.0 reject the declaration. GCC 14 and 16 accept it with no warning, also with `-Wall -Wextra`. GCC keeps the natural alignment: a class with an `alignas(1) int` member still has alignment 4 and size 8 on x86-64, which is option 2.

## Options

1. The program is not valid.
2. The weaker value has no effect.

## Provisional answer

No; the program is not valid (option 1). Clang and EDG agree.

## Opinions
