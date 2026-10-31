# Q-0038: May a default argument name a parameter in an operand that is not evaluated?

**Status:** provisional
**Affects:** DEFARG-4, `tests/declarations/functions/default-arguments/parameter-in-sizeof.cpp`

## Context

`int f(int a, int b = sizeof(a));` names the parameter `a` inside `sizeof`, which doesn't evaluate `a`. Clang 22 and 18 and GCC 16 and 14 accept the declaration. EDG 7.0 rejects it ("a parameter is not allowed"). A gap review summary also says the declaration is valid.

## Options

1. Valid: the ban covers only evaluated uses of a parameter.
2. Rejected: a default argument must not name a parameter at all.

## Provisional answer

Option 1. Two of three implementations accept the declaration, and the review summary agrees.

## Opinions
