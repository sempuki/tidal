# Q-0365: Is the result of a return statement destroyed when a local destructor then throws?

**Status:** provisional
**Affects:** UNW-1, `tests/exceptions/unwinding/return-local-destructor-throws-result-destroyed.cpp`, `tests/exceptions/unwinding/return-local-destructor-throws-caught-inside.cpp`

## Context

A return statement builds the result of the call, and then the
function's local objects end. If the destructor of a local object
throws at that point, the result has been built and the call does not
complete.

```cpp
struct T { ~T() noexcept(false) { throw 1; } };
R named() { T t; R r; return r; }
R fresh() { T t; return R(); }
```

| Probe | Clang 18, 22 | GCC 14, 16 |
|---|---|---|
| The result of `named()` is destroyed before the handler runs | no | yes |
| The result of `fresh()` is destroyed before the handler runs | no | yes |

EDG runs no programs, so its choice is not observed.

When a handler in the same function catches the exception and runs a
second return statement, GCC destroys the first result before the
handler runs, and Clang never destroys it. The caller receives the
result of the second return statement on both.

The source-reader gap review of the exceptions chapter
(`reports/review/2026-11-03-gap-review-exceptions.md`) reads the
reference as destroying the result in this case.

## Evidence

The test in **Affects** expects the result to be destroyed and records
Clang's result with an override.

## Options

1. The result is destroyed during unwinding, like any other object that
   was fully built (GCC).
2. The result is not destroyed (Clang).

## Provisional answer

The result is destroyed (option 1). UNW-P2 ends every object that was fully built, and the
result is fully built before the local objects end. Option 2 leaves an
object that no code can ever destroy.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
