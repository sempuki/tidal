# Q-0124: Is `++` or `--` on a signed operand narrower than `int` defined when the new value does not fit the operand's type?

**Status:** provisional
**Affects:** UN-20, UN-21, PFX-12, PFX-18, `tests/expressions/unary/prefix-narrow-signed-wraps.cpp`, `tests/expressions/postfix/postfix-narrow-signed-wraps.cpp`

## Context

An operand of type `short` or `signed char` is promoted to `int` for
the step. The new value then goes back into the operand's type. For a
`short` that holds 32767, `++` gives 32768 in `int`, and that value does
not fit in `short`.

During constant evaluation, Clang and GCC accept the step. The operand
then holds the value that the integer conversion gives (CONV-2), so
32767 becomes -32768. EDG 7.0 rejects the step and reports that the
value exceeds the range of the operand's type. At run time, Clang and
GCC give the converted value, and `-fsanitize=undefined` reports
nothing. Clang reports the change only with the separate
`-fsanitize=implicit-conversion` check, which is not a check for
undefined behavior.

For an operand of type `int` or wider, all three compilers reject an
overflow during constant evaluation (UN-21, PFX-18).

## Evidence

`tests/RESULTS.md`, rows `prefix-narrow-signed-wraps.cpp` and
`postfix-narrow-signed-wraps.cpp`.

## Options

1. The step is done in the promoted type, and the result converts back
   to the operand's type. The behavior is defined. EDG differs.
2. A result outside the operand's type is undefined, as for `int`.
   Clang and GCC differ.

## Provisional answer

The step is defined and the value converts back; EDG differs (option 1). Two of three implementations take this behavior, and the
sanitizer of both agrees that the step is defined.

## Opinions
