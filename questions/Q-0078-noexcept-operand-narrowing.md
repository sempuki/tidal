# Q-0078: Can the operand of noexcept narrow when it is converted to bool?

**Status:** provisional
**Affects:** EXS-1, `tests/declarations/exception-specifications/narrowing-operand.cpp`, `tests/declarations/exception-specifications/pointer-operand.cpp`

## Context

`noexcept(2)` and `noexcept(&x)` give the specifier an operand that is not `bool`. Clang and GCC reject both. EDG 7.0 converts the operand to `bool` and accepts both.

## Options

1. The operand must convert to `bool` without narrowing.
2. Any conversion to `bool` is valid.

## Provisional answer

Option 1. Clang and GCC agree. `static_assert` and `if constexpr` permit narrowing, so the rule for `noexcept` differs from the rules for those two.

## Opinions
