# Q-0264: Does an implementation diagnose an atomic constraint whose result changes between two checks?

**Status:** provisional
**Affects:** CNS-10, `tests/templates/constraints/satisfaction-changes.cpp`

## Context

`Complete<Late>` is checked before and after `Late` is defined. The
requirement `sizeof(T)` fails the first time and is valid the second
time.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, and gives false both times |
| GCC 14 | accepts, and gives false both times |
| GCC 16, host | rejects: satisfaction value of atomic constraint changed from false to true |
| EDG 7.0 | accepts, and gives false both times |

## Evidence

The test above.

## Options

1. The program is not valid, and no diagnostic is required. Both
   results are allowed (CNS-10).
2. A diagnostic is required (GCC 16).

## Provisional answer

No diagnostic required; GCC 16 diagnoses (option 1). Finding the change needs a record of every earlier check, which most implementations do not keep.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
