# Q-0366: Can `main` be declared inside an `extern "C++"` block?

**Status:** provisional
**Affects:** TR-3, `tests/foundations/translation/main-cpp-linkage-specification.cpp`

## Context

TR-3 rejects `main` with C language linkage. A program can also declare
`main` inside a linkage specification for C++, which gives the linkage
that `main` has anyway.

```cpp
extern "C++" int main() { return 0; }
```

| Compiler | Result |
|---|---|
| Clang 18, 22 | accepts |
| GCC 14, 16 | accepts |
| EDG 7.0 | rejects: `main` cannot be declared in a linkage specification |

The source-reader gap review of program structure
(`reports/review/2026-11-03-gap-review-program-structure.md`) reads the
reference as rejecting `main` declared with any language linkage
specification.

## Evidence

The test in **Affects** expects a run and records EDG with an override.
Every compiler rejects `extern "C"` on `main`, apart from GCC 14 and
Clang 18 (`main-c-linkage.cpp`).

## Options

1. Accept the form, as Clang and GCC do. Only C language linkage is
   rejected.
2. Reject any linkage specification on `main`, as EDG does and as the
   review reads the reference.

## Provisional answer

Accept the form (option 1). Clang 22 and GCC 16 agree, and Tidal follows
the two when EDG alone differs (`documentation/decisions/pending.md`,
row 3). The form gives `main` no property that it lacks without the
block.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
