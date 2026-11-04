# Q-0086: Is a call to a function that returns an abstract class valid?

**Status:** provisional
**Affects:** VIRT-8, `tests/classes/virtual/call-returning-abstract.cpp`

## Context

A function is declared to return an abstract class, and a program calls it. GCC and EDG reject the call. Clang accepts it.

## Options

1. The call is not valid, because the call creates an object of the abstract class.
2. The call is valid.

## Provisional answer

No; the call creates an object of the abstract class (option 1). GCC and EDG agree.

Kept open on 2026-11-03 under decision 0054, because no principle on the page states the case. VIRT-P4 says that an abstract class cannot be an object on its own, and does not say whether the result of a call to a function that returns such a class is an object. Clang 22 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A function can be declared with an abstract return type. A call gives a prvalue of that type, and a prvalue of abstract class type is allowed only in the operand of `decltype`. Clang's acceptance is a defect.
