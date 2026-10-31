# Q-0091: Is delete[] of a derived array through a base pointer rejected during constant evaluation?

**Status:** provisional
**Affects:** NEW-5, `tests/expressions/new-delete/delete-derived-array-through-base-constant.cpp`

## Context

Constant evaluation creates an array of a derived class with
`new[]` and deletes it with `delete[]` through a pointer to the base,
which has a virtual destructor. Clang and EDG reject the evaluation. GCC
accepts it.

## Options

1. The behavior is undefined, so constant evaluation rejects it.
2. Constant evaluation accepts it.

## Provisional answer

Option 1. Clang and EDG agree, and VIRT-12 treats a
single object the same way (Q-0056).

## Opinions
