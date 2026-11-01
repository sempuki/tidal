# Q-0078: Can the operand of noexcept narrow when it is converted to bool?

**Status:** provisional
**Affects:** EXS-7, `tests/declarations/exception-specifications/narrowing-operand.cpp`, `tests/declarations/exception-specifications/pointer-operand.cpp`

## Context

`noexcept(2)` and `noexcept(&x)` give the specifier an operand that is not `bool`. Clang and GCC reject both. EDG 7.0 converts the operand to `bool` and accepts both.

GCC 14 and 16 give the same result, and so do Clang 18 and 22. The host
GCC 16.2 and Clang 22.1 agree with them. No newer version settles the
split.

## Options

1. The operand must convert to `bool` without narrowing.
2. Any conversion to `bool` is valid.

## Provisional answer

No; the operand must convert to `bool` without narrowing (option 1). Clang and GCC agree. `static_assert` and `if constexpr` permit narrowing, so the rule for `noexcept` differs from the rules for those two.

## Opinions
