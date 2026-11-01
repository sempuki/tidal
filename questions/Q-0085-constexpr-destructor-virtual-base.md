# Q-0085: Can a class with a virtual base have a constexpr destructor?

**Status:** provisional
**Affects:** DER-12, `tests/classes/derived/constexpr-dtor-virtual-base.cpp`

## Context

A class with a virtual base declares a constexpr destructor. Clang and GCC reject the declaration. EDG 7.0 accepts it.

## Options

1. Not valid, as for constexpr constructors.
2. Valid.

## Provisional answer

No, as for constexpr constructors (option 1). Clang and GCC agree, and option 1 matches the rule for constructors.

## Opinions
