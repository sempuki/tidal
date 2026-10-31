# Q-0086: Is a call to a function that returns an abstract class valid?

**Status:** provisional
**Affects:** VIRT-8, `tests/classes/virtual/call-returning-abstract.cpp`

## Context

A function is declared to return an abstract class, and a program calls it. GCC and EDG reject the call. Clang accepts it.

## Options

1. The call is not valid, because it creates an object of the abstract class.
2. The call is valid.

## Provisional answer

Option 1. GCC and EDG agree.

## Opinions
