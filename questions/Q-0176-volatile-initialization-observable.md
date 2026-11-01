# Q-0176: Is the initialization of a volatile object an observable write?

**Status:** provisional
**Affects:** VOL-1, VOL-12, TR-7

## Context

VOL-1 and TR-7 make each read and write of a volatile object through a
volatile lvalue an observable effect. An initializer does not go through
an lvalue, so these rules do not settle whether an initializer of a
volatile object is a write that the implementation must perform.

| Initialization | Clang 22 and GCC 16 at `-O2` |
|---|---|
| `volatile int v = 5;` in a function, never read | The store is kept |
| `volatile int v = seed();` in a function, never read | The store is kept |
| `volatile int v;` in a function | No store |
| `volatile R r = {1, 2};` in a function | One 8-byte store for both members |
| `volatile int g = 7;` at namespace scope | The value is in the data, no store |
| `volatile int g;` at namespace scope | Zero storage, no store |
| `volatile int g = seed();` at namespace scope | The store runs in the dynamic initialization |

## Evidence

- `tests/declarations/volatile/initialization-write.cpp` runs at `-O2`
  and checks the values. A test cannot observe whether a store was
  removed.
- The code that Clang 22 and GCC 16 generate at `-O2` for the cases in
  the table.

## Options

1. An initializer that runs at run time writes the volatile object, and
   the write is observable. Zero-initialization and constant
   initialization happen before the program runs and are not accesses.
2. Only accesses after the initialization are observable. An
   implementation can drop the store of an initializer.

## Provisional answer

Option 1. Both compilers keep the store, even for a local that the
program never reads. Option 1 does not say how many stores write a
class object. The compilers write two `int` members with one wide store.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
