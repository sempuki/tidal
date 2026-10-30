# Q-0045: Must an anonymous union in an unnamed namespace be declared `static`?

**Status:** provisional
**Affects:** STO-11, `tests/declarations/storage-linkage/anonymous-union-unnamed-namespace.cpp`

## Context

A namespace-scope anonymous union must be declared `static`. Inside an unnamed namespace, Clang 22 and 18 accept one without `static`. GCC 16 and 14 and EDG 7.0 reject it. The gap review summary says the unnamed namespace form is valid.

## Options

1. Rejected without `static`, as GCC and EDG do.
2. Valid in an unnamed namespace, as Clang does and the review summary says.

## Provisional answer

Option 1, the majority of implementations. The review summary's reading is a strong reason to revisit it.

## Opinions
