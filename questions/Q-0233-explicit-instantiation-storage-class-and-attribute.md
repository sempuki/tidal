# Q-0233: Can an explicit instantiation have a storage class or an attribute after the keyword template?

**Status:** provisional
**Affects:** INST-25, `tests/templates/instantiation/explicit-storage-class.cpp`, `tests/templates/instantiation/explicit-attribute-after.cpp`

## Context

`template static int get(int);` and `template [[nodiscard]] int
get(int);`.

| Compiler | `static` | attribute after `template` | attribute before `template` |
|---|---|---|---|
| Clang 18, 22, host | rejects | rejects | rejects |
| EDG 7.0 | rejects | rejects | rejects |
| GCC 14, 16, host | accepts | accepts | rejects |

## Evidence

The tests above and `explicit-attribute-before.cpp`. Clang's diagnostic
list (clang-diag-sema) has "explicit instantiation cannot have a
storage class". Inventory item G1 forbids attributes and storage classes
other than `thread_local`.

## Options

1. Both are not valid (Clang, EDG).
2. Both are accepted (GCC).

## Provisional answer

Option 1. Two of three implementations and the inventory agree.
`thread_local` is not probed.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
