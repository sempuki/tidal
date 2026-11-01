# Q-0231: Is the initializer of an inline static data member instantiated with its class?

**Status:** provisional
**Affects:** INST-5, `tests/templates/instantiation/class-inline-static-member-unused.cpp`

## Context

`static inline int count = T::nothing;` in a class template, with
`Box<int>` defined as an object and `count` not used.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: the initializer is not valid for `int` |

## Evidence

The test above. Inventory item H18 says that a static data member is
instantiated, with its initialization, only when its definition is
needed.

## Options

1. The initializer waits for a use (Clang, GCC).
2. The initializer is made with the class (EDG).

## Provisional answer

No; Clang and GCC agree, EDG differs (option 1). Two of three implementations and the inventory agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
