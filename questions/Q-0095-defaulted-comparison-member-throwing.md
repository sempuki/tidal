# Q-0095: Is a defaulted == non-throwing if a member == is potentially throwing?

**Status:** provisional
**Affects:** EXS-4, `tests/declarations/exception-specifications/defaulted-comparison-member-throwing.cpp`

## Context

A class defaults `operator==`. A member's class has an `==`
without `noexcept`. Clang and GCC treat the defaulted `==` as potentially
throwing. EDG 7.0 treats it as non-throwing.

## Options

1. The defaulted comparison is potentially throwing, as for defaulted
   special members.
2. It is non-throwing.

## Provisional answer

Option 1. Clang and GCC agree, and the rule matches
defaulted special members.

## Opinions
