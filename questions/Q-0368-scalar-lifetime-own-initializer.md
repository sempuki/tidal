# Q-0368: Is a write to a scalar during its own initializer outside the scalar's lifetime?

**Status:** provisional
**Affects:** OBJ-1, OBJ-2, `tests/foundations/objects/scalar-write-in-own-initializer-constant.cpp`, `tests/foundations/objects/scalar-write-through-pointer-in-own-initializer-constant.cpp`

## Context

OBJ-1 starts the lifetime of a scalar when its storage is obtained and
its initializer completes. A scalar with no initializer begins with its
storage. While the initializer runs, the name of the variable is already
in scope, so the initializer can write the variable.

```cpp
constexpr int set(int* p) { *p = 5; return 7; }
constexpr int direct() { int x = ((x = 5), x + 1); return x; }
constexpr int indirect() { int x = set(&x); return x; }
```

| Probe, in constant evaluation | Clang 18, 22 | GCC 14, 16 | EDG 7.0 |
|---|---|---|---|
| `direct()` | rejects | accepts, 6 | accepts, 6 |
| `indirect()` | rejects | accepts, 7 | accepts, 7 |

The source-reader gap review of the object model
(`reports/review/2026-11-03-gap-review-object-model.md`) reads the
reference as starting the lifetime of every object when its
initialization completes, so a write during the initializer is
undefined.

## Evidence

The two tests in **Affects** expect a rejection and record GCC and EDG
with overrides.

## Options

1. The write is outside the scalar's lifetime and undefined (OBJ-2).
   Constant evaluation rejects it (Clang).
2. The lifetime of a scalar starts with its storage, so the write is
   valid (GCC, EDG).

## Provisional answer

The write is outside the lifetime and undefined (option 1). The review
reads the reference this way, and OBJ-1 then gives one rule for class
and scalar objects: the lifetime begins when the initialization
completes. Clang 22 follows the answer, and GCC 16 and EDG 7.0 differ.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
