# Q-0192: Does a class with a trivial default constructor and no initializer have a constant initializer?

**Status:** provisional
**Affects:** CE-24, CE-21, CINIT-1, `tests/expressions/constant-expressions/constant-initializer-zero-trivial-class.cpp`

## Context

A variable with static storage holds zero before any other
initialization (STO-3). For a variable with no initializer, CE-24 judges
the default initialization. CE-21 says that the zero does not give a
part a value.

The compilers agree on these cases:

| Variable at namespace scope | Clang | GCC | EDG |
|---|---|---|---|
| `constinit int counter;` | accepts | accepts | accepts |
| `constinit int values[3];` | accepts | accepts | accepts |
| `constinit` class whose `constexpr` constructor leaves a member unset | rejects | rejects | rejects |

The compilers split on one case. Take
`struct Pair { int first; int second; }; constinit Pair pair;`. The
default constructor of `Pair` is trivial. GCC and EDG accept the
variable, and the members read as zero at run time. Clang 18 and Clang
22 reject the variable and say that it has no constant initializer. A
`static constinit` variable in a block gives the same split.

## Evidence

- `tests/expressions/constant-expressions/constant-initializer-zero-scalar.cpp`
- `tests/expressions/constant-expressions/constant-initializer-zero-trivial-class.cpp`
- `tests/expressions/constant-expressions/result-unset-member.cpp`

## Options

1. A default initialization that calls no constructor, or only a
   trivial one, leaves the zero, and the zero is a constant initializer.
   A class with a trivial default constructor then behaves as a scalar
   does. Clang differs.
2. Only a scalar or an array of scalars keeps the zero as a constant
   initializer. A class must get a value for each member from its
   initialization. GCC and EDG differ.

## Provisional answer

Yes, the zero counts, as for a scalar; Clang differs (option 1). Two of three implementations accept the variable. Option 1
treats the trivial class the same as the scalar case, where every
compiler accepts.

## Opinions
