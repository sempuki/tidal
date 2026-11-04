# Q-0371: Can a member of `N` be defined with the name `N::f` inside `N` itself?

**Status:** provisional
**Affects:** NS-15, `tests/declarations/namespaces/qualified-definition-inside-own-namespace.cpp`

## Context

NS-15 lets a program define a member of `N` with a qualified name in a
namespace that encloses `N`. The program
`namespace N { int f(); int N::f() { return 0; } }` writes the
qualified name inside `N` itself.

| Compiler | Result |
|---|---|
| GCC 14 and 16 | rejects ("explicit qualification in declaration") |
| EDG 7.0 | rejects |
| Clang 18 and 22 | accepts, with a warning ("extra qualification on member", `-Wextra-qualification`) that `-pedantic-errors` leaves a warning |

## Evidence

The test in Affects, and probe `q1.cpp` of the Propose exercise X15.

## Options

1. The definition is not valid. A namespace does not enclose itself
   (GCC, EDG).
2. The definition is valid, and the qualification is redundant (Clang).

## Provisional answer

No; the definition is not valid (option 1). Two of three families
reject it, and Clang's warning marks the form as one it does not expect.

Kept open on 2026-11-03 under decision 0054, because no review has read the provisional answer. Clang 22 is the one newest family that differs.

## Opinions
