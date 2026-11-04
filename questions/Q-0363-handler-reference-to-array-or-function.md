# Q-0363: Does a handler of a reference to an array or a function match a thrown array or function?

**Status:** provisional
**Affects:** TRY-2, THR-2, `tests/exceptions/handlers/match-reference-to-array-never.cpp`, `tests/exceptions/handlers/match-reference-to-function-never.cpp`

## Context

A thrown array or function becomes a pointer (THR-2), so the exception
object has a pointer type. A handler can still declare a reference to
an array of known bound, such as `catch (int (&)[3])`, or a reference
to a function, such as `catch (int (&)())`. TRY-5 accepts both
declarations.

| Program | Clang 18, 22 | GCC 14, 16 |
|---|---|---|
| `int arr[3]; throw arr;` caught by `catch (int (&)[3])` | no match | matches |
| `int f(); throw f;` caught by `catch (int (&)())` | no match | matches |

EDG runs no programs, so its choice is not observed.

The source-reader gap review of the exceptions chapter
(`reports/review/2026-11-03-gap-review-exceptions.md`) reads the
reference as saying that such a handler never matches.

## Evidence

The two tests in **Affects**. Each expects no match and records GCC's
match with an override.

## Options

1. The handler never matches. The exception object is a pointer, and no
   bullet of TRY-2 matches a reference to an array or a function with a
   pointer (Clang).
2. The handler matches the pointer that the array or function decays to
   (GCC).

## Provisional answer

The handler never matches (option 1). TRY-P2 lets a handler accept the exception object only by
conversions that keep the object what it is. A reference to an array
or a function would have to refer to an object that the exception
object only points to.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
