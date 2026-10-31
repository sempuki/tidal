# Q-0032: May the right operand of a compound assignment be a braced list?

**Status:** provisional
**Affects:** ASN-6, `tests/expressions/assignment/braced-compound.cpp`

## Context

`int x; x += {1};` uses a braced list as the right operand of `+=`. Clang
22 and 18 accept it without a diagnostic. GCC 16 and 14 ("invalid use of
brace-enclosed initializer list") and EDG 7.0 reject it. All compilers
accept a braced list on the right of simple `=`.

## Options

1. A braced list is allowed only as the right operand of simple `=`.
2. A braced list is also allowed for compound assignment to a scalar, as
   Clang does.

## Provisional answer

Option 1. Two of three implementations reject a braced list with `+=`.

## Opinions
