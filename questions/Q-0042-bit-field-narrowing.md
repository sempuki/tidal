# Q-0042: Is converting a bit-field whose values all fit the target a narrowing conversion?

**Status:** provisional
**Affects:** LIST-2, `tests/declarations/initialization/list/narrowing-bit-field-fits.cpp`

## Context

`char c{s.bits};`, where `bits` is an `unsigned` bit-field of width 4, converts a value that always fits in `char`. Clang 22 and GCC 16 accept it in braces. Clang 18, GCC 14, and EDG 7.0 reject it as narrowing.

## Options

1. Not narrowing when every value of the bit-field's width fits the target. The newer Clang and GCC agree; the older versions predate the change.
2. Narrowing, judged by the bit-field's declared type.

## Provisional answer

Option 1. The newest versions of two implementations accept it, and both changed in the same direction.

## Opinions
