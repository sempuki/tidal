# Q-0236: Can an explicit instantiation of a variable leave out the array bound?

**Status:** provisional
**Affects:** INST-22, `tests/templates/instantiation/explicit-array-bound-left-out.cpp`

## Context

`template <class T> int table[] = {1, 2, 3};` and `template int
table<int>[];`. The specialization has the type `int[3]`.

| Compiler | Result |
|---|---|
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |
| Clang 18, 22, host | rejects: `int[]` does not match `int[3]` |

Every compiler accepts `template int table<int>[3];`.

## Evidence

The test above and `explicit-deduced-arguments.cpp`. Inventory item
G17 says that an array bound can be left out.

## Options

1. The bound can be left out (GCC, EDG).
2. The type must match exactly (Clang).

## Provisional answer

Option 1. Two of three implementations and the inventory agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
