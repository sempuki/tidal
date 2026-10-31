# Q-0081: Does a braced list with one object of the same class use the initializer-list constructor?

**Status:** provisional
**Affects:** ILC-3, `tests/classes/initializer-lists/copy-from-single-same-class.cpp`

## Context

A class `S` has a copy constructor and a constructor that takes
`std::initializer_list<S>`. The program writes `S b{a};` where `a` is an
`S`. Clang 22 and GCC call the initializer-list constructor. Clang 18
calls the copy constructor. EDG does not run programs.

## Options

1. The initializer-list constructor is used, as for other braced lists.
2. The copy constructor is used.

## Provisional answer

Option 1. The newer Clang agrees with GCC.

## Opinions
