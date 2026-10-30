# Q-0033: May constant evaluation compare the addresses of two string literals?

**Status:** provisional
**Affects:** CE-13, `tests/expressions/constant-expressions/string-literal-identity.cpp`

## Context

`static_assert("abc" == "abc" || true);` asks constant evaluation to
compare the addresses of two string literals with the same text. Whether
two such literals share storage is not fixed. Clang 22 and 18 reject the
comparison ("comparison of addresses of potentially overlapping literals").
GCC 16 and 14 and EDG 7.0 accept it and give a value.

## Options

1. Rejected during constant evaluation, because the result depends on
   storage that the implementation chooses. This matches the treatment of
   other unspecified pointer comparisons (CMP-8).
2. Accepted, with an implementation-defined result.

## Provisional answer

Option 1. A constant expression should not depend on a choice that the
implementation need not make the same way each time.

## Opinions
