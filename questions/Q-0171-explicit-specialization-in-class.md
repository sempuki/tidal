# Q-0171: Can an explicit specialization of a member template be declared in the class?

**Status:** provisional
**Affects:** TSP-5, TSP-19

## Context

A class declares `template <class T> static constexpr int of()` and,
in its body, `template <> constexpr int of<int>()`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | rejects: explicit specialization in a scope that is not a namespace |
| EDG 7.0 | accepts |

## Evidence

`tests/templates/specialization/explicit-class-scope.cpp`, with an
override for GCC.

## Options

1. The class that declares the member template is a valid place for the
   explicit specialization (Clang, EDG).
2. Only namespace scope is valid (GCC).

## Provisional answer

Option 1. Clang and EDG agree, and the class is the scope where the
member template is declared, which matches TSP-5's rule for namespaces.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
