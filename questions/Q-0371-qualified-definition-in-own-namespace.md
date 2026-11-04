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

Kept open on 2026-11-04 under decision 0054, because the gap review of the rules that phase 7 rewrote reads the reference as option 2, and Clang 22 follows that reading. Tidal keeps the answer of GCC 16 and EDG 7.0 until the maintainer decides (`documentation/decisions/pending.md`, row 4).

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that the phase 7 front-end and Propose repairs rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`) reads the reference as supporting option 2. The reference requires a declaration with a qualified name to inhabit a namespace scope that contains the entity it declares again, and to match a declaration that the qualifier's namespace can name, its inline namespaces included. A namespace contains its own members in this sense, so a member can be defined with a qualified name inside its own namespace. A core defect resolution long before C++23 made such redundant qualification valid, and Clang's warning reflects that history. GCC and EDG conflict with the reference.
