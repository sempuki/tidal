# Q-0058: Can an unnamed bit-field have a cv-qualifier?

**Status:** provisional
**Affects:** BF-2, `tests/classes/unions/bitfield-unnamed-cv.cpp`

## Context

An unnamed bit-field only reserves bits. A program cannot name it, so a
`const` or `volatile` on it has no effect. Clang and EDG reject the
qualifier. GCC accepts it.

## Options

1. A program must not put a cv-qualifier on an unnamed bit-field.
2. The qualifier is valid and has no effect.

## Provisional answer

No; a program must not put a cv-qualifier on an unnamed bit-field (option 1). Clang and EDG agree, and the qualifier cannot change the
meaning of the program.

## Opinions
