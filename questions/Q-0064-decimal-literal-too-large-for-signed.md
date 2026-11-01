# Q-0064: Is a decimal literal that fits only an unsigned type valid?

**Status:** provisional
**Affects:** PRIM-2, `tests/expressions/primary/int-too-large-signed-decimal.cpp`

## Context

The literal `9223372036854775808` has no suffix and does not fit in
`long long`. It fits in `unsigned long long`. Clang and GCC reject it
under `-pedantic-errors`. Without that option they warn and give the
literal an unsigned type. EDG 7.0 accepts the literal in strict mode.

## Options

1. The program is not valid.
2. The literal has an unsigned type.

## Provisional answer

No; the program is not valid (option 1). Clang and GCC treat the unsigned type as an extension.

## Opinions
