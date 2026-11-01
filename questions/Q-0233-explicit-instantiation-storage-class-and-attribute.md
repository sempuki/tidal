# Q-0233: Can an explicit instantiation have a storage class or an attribute after the keyword template?

**Status:** provisional
**Affects:** INST-25, `tests/templates/instantiation/explicit-storage-class.cpp`, `tests/templates/instantiation/explicit-attribute-after.cpp`, `tests/templates/instantiation/explicit-thread-local.cpp`

## Context

`template static int get(int);` and `template [[nodiscard]] int
get(int);`.

`template thread_local int counter<int>;` repeats `thread_local` for
a variable template declared `thread_local`.

| Compiler | `static` | attribute after `template` | attribute before `template` | `thread_local` |
|---|---|---|---|---|
| Clang 18, 22, host | rejects | rejects | rejects | accepts |
| EDG 7.0 | rejects | rejects | rejects | rejects |
| GCC 14, 16, host | accepts | accepts | rejects | accepts |

## Evidence

The tests above and `explicit-attribute-before.cpp`. Item 34 of the gap
review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md`
gave the `thread_local` probe. Clang's diagnostic
list (clang-diag-sema) has "explicit instantiation cannot have a
storage class". Inventory item G1 forbids attributes and storage classes
other than `thread_local`.

## Options

1. `static` and the attribute are not valid, and `thread_local` is
   valid (Clang; EDG except for `thread_local`).
2. All are accepted (GCC).
3. No storage class specifier is valid (EDG).

## Provisional answer

Option 1. Two of three implementations agree on each column, and the
inventory and the gap review (item 34) allow `thread_local`.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
