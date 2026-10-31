# Q-0121: Can constant evaluation write a variable whose declaration a jump skipped?

**Status:** provisional
**Affects:** JMP-16, `tests/statements/switch/skipped-declaration-written-then-read.cpp`

## Context

A `switch` can jump past the declaration of a scalar with no
initializer (JMP-6). At run time every compiler lets the program then
write the variable and read the value back. In constant evaluation the
compilers split. Clang accepts the write. GCC 14 and 16 reject it as a
change to an object from outside the evaluation, and EDG rejects it as
an access to run-time storage. All three reject a read before the
write.

## Evidence

`tests/RESULTS.md`, rows `skipped-declaration-written-then-read.cpp`,
`skipped-declaration-read-first.cpp`, and
`goto-past-declaration-then-write.cpp`.

## Options

1. The variable exists in its block after the jump, as at run time, and
   constant evaluation can write it. GCC and EDG differ.
2. Constant evaluation treats a variable whose declaration it did not
   pass as outside the evaluation. Clang differs, and constant
   evaluation then differs from a run.

## Provisional answer

Option 1. The run-time behavior is the same on every compiler, and
CE-P1 says constant evaluation is the same program run at translation
time.

## Opinions
