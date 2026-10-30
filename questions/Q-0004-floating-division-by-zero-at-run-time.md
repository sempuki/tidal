# Q-0004: Should floating division by zero be implementation-defined on IEEE 754 targets?

**Status:** provisional
**Affects:** MUL-11

## Context

At run time on x86-64, Clang and GCC produce IEEE 754 results for
floating division by zero: infinity for a nonzero dividend, NaN for zero
divided by zero. Numeric code relies on this. During constant evaluation,
Clang and GCC reject the same operations (EDG accepts them, see Q-0002).

## Evidence

Run-time behavior on x86-64 Linux with Clang 22.1.8 and GCC 16.2.1. No
committed test yet.

## Options

1. Keep MUL-11 `undefined` everywhere.
2. Make the run-time result `impl-defined`, with IEEE 754 results required
   on implementations that document IEEE 754 arithmetic, and keep rejection
   during constant evaluation.

## Provisional answer

Option 1 until a test shows the IEEE behavior is reliable under
optimization on the matrix compilers. Option 2 is a likely future change.

## Opinions
