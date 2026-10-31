# Q-0110: Is a reference bound through a null pointer rejected during constant evaluation?

**Status:** provisional
**Affects:** REF-5, `tests/declarations/initialization/references/null-dereference-bind-constant.cpp`

## Context

A constexpr function binds a reference to `*p` where `p` is a null pointer, and does not read through it. GCC and Clang 22 reject the evaluation. EDG 7.0 and Clang 18 accept it.

## Options

1. The binding is undefined, so constant evaluation rejects it.

2. Only a read through the reference is undefined.

## Provisional answer

Option 1. The newer Clang agrees with GCC.

## Opinions
