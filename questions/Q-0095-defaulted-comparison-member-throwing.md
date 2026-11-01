# Q-0095: Is a defaulted == non-throwing if a member == is potentially throwing?

**Status:** provisional
**Affects:** EXS-4, `tests/declarations/exception-specifications/defaulted-comparison-member-throwing.cpp`

## Context

A class defaults `operator==`. A member's class has an `==`
without `noexcept`. Clang and GCC treat the defaulted `==` as potentially
throwing. EDG 7.0 treats it as non-throwing.

GCC 14 and 16 give the same result, and so do Clang 18 and 22. The host
GCC 16.2 and Clang 22.1 agree with them. No newer version settles the
split.

## Options

1. The defaulted comparison is potentially throwing, as for defaulted
   special members.
2. The defaulted comparison is non-throwing.

## Provisional answer

Option 1. Clang and GCC agree, and the rule matches
defaulted special members.

## Opinions
