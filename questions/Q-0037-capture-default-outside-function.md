# Q-0037: May a lambda outside a function have a capture default?

**Status:** provisional
**Affects:** LAM-6, `tests/expressions/lambdas/capture-default-at-namespace.cpp`

## Context

`auto g = [=] { return 1; };` at namespace scope has a capture default but nothing to capture. Clang 22 and 18 and GCC 16 and 14 reject the lambda ("non-local lambda expression cannot have a capture-default"). EDG 7.0 accepts it.

## Options

1. Rejected: a capture default needs an enclosing function.
2. Accepted, because the lambda has nothing to capture.

## Provisional answer

Option 1. Clang and GCC, in both versions, reject the lambda.

## Opinions
