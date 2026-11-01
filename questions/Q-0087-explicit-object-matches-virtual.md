# Q-0087: Can an explicit object member function have the name and parameters of a base virtual function?

**Status:** provisional
**Affects:** VIRT-19, `tests/classes/virtual/explicit-object-overrides.cpp`

## Context

A derived class declares a member function with an explicit object parameter. The name and other parameters of that function match a virtual function of a base. Clang and GCC reject the declaration. EDG 7.0 accepts it.

## Options

1. Not valid.
2. Valid, and the function does not override.

## Provisional answer

No; the declaration is not valid (option 1). Clang and GCC agree.

## Opinions
