# Q-0056: Is delete through a base pointer without a virtual destructor rejected during constant evaluation?

**Status:** provisional
**Affects:** VIRT-12, `tests/classes/virtual/nonvirtual-dtor-delete-constant.cpp`

## Context

A program creates a derived object with `new` and deletes it through a
pointer to a base whose destructor is not virtual. During constant
evaluation, Clang and EDG reject this. GCC 14 and 16 accept it.

## Options

1. The behavior is undefined, so constant evaluation rejects it.
2. Constant evaluation accepts it.

## Provisional answer

Option 1. Clang and EDG agree, and the same delete with a virtual
destructor is accepted by all compilers.

## Opinions
