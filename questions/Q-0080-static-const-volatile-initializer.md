# Q-0080: Can a static const volatile member have an initializer in its class?

**Status:** provisional
**Affects:** VOL-2, `tests/declarations/volatile/static-const-volatile-in-class.cpp`

## Context

`struct S { static const volatile int n = 1; };` gives a `static const`
integer member that is also `volatile` an initializer in the class. GCC
and EDG accept the declaration. Clang rejects it and asks for the
initializer outside the class.

## Options

1. The initializer in the class is valid.
2. The member must be initialized outside the class.

## Provisional answer

Option 1. GCC and EDG accept the declaration. An opinion from a
contributor is welcome, because the member cannot be used in a constant
expression either way.

## Opinions
