# Q-0234: Can an explicit instantiation declaration name a specialization with internal linkage?

**Status:** provisional
**Affects:** INST-29, `tests/templates/instantiation/explicit-declaration-internal.cpp`

## Context

`template <class T> static int get(T)` and `extern template int
get(int);`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects |
| EDG 7.0 | rejects |
| GCC 14, 16, host | accepts |

An explicit instantiation definition of the same specialization is
accepted by every compiler.

## Evidence

The test above and `explicit-definition-internal.cpp`. Clang's
diagnostic list (clang-diag-sema) has "explicit instantiation
declaration of ... with internal linkage". Inventory item G14 agrees.

## Options

1. Not valid (Clang, EDG).
2. Valid (GCC).

## Provisional answer

No; Clang and EDG agree, GCC differs (option 1). A specialization with internal linkage has no definition in
another unit, so the declaration could never be satisfied.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
