# Q-0365: Is the result of a return statement destroyed when a local destructor then throws?

**Status:** settled
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

## Answer

The result is destroyed (option 1). UNW-P2 ends every object that was fully built, and the
result is fully built before the local objects end. Option 2 leaves an
object that no code can ever destroy.

Settled on 2026-11-04 under decision 0054. The gap review of the rules that phase 7 added (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) supports this answer with no doubt, and UNW-P1 gives the same answer: by the time the handler runs, nothing that the code inside the try block created still exists. Clang 22 still differs, and the tests keep their overrides for Clang. EDG does not run the programs. The place of the result in the order of destruction is a separate item, which `documentation/decisions/pending.md` row 4 records.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that phase 7 added (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) agrees with option 1. The reference says that when the destructor of a local object or a temporary throws during a return statement, the returned object is destroyed too. Its example also shows a handler in the same function: the first result is destroyed, and a second return statement builds a new one. The rule came from a core defect resolution that Clang appears not to implement. The gap review of the rules that phase 7 rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`) reads the order differently from UNW-1. In its reading the result ends before the local objects that were built before it, in the reverse order of construction.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
