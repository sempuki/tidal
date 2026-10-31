# Q-0069: Can alignas make an alignment weaker than the natural one?

**Status:** provisional
**Affects:** ALN-2, `tests/declarations/alignment/weaker-than-natural.cpp`, `tests/declarations/alignment/weaker-on-class-member.cpp`

## Context

A declaration such as `alignas(1) int x;` asks for an alignment weaker than the alignment of `int`. Clang and EDG reject it. GCC accepts it.

## Options

1. The program is not valid.
2. The weaker value has no effect.

## Provisional answer

Option 1. Clang and EDG agree.

## Opinions
