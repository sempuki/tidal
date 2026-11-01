# Q-0158: Can an `==` used for a reversed or rewritten comparison return a reference to `bool`?

**Status:** provisional
**Affects:** DCMP-12, `tests/classes/comparisons/eq-reference-to-bool-result.cpp`

## Context

A class has an `operator==` that returns `bool&` or `bool&&`. A comparison
uses that operator reversed (`key == acct`) or rewritten
(`acct != 1`).

| Compiler | `bool&`, reversed `==` | `bool&&`, reversed `==` | `bool&`, rewritten `!=` |
|---|---|---|---|
| Clang 18, Clang 22 | rejects | rejects | rejects |
| EDG 7.0 | rejects | rejects | rejects |
| GCC 14, GCC 16 | accepts | accepts | accepts |

Every compiler accepts a result of `const bool` or `const volatile
bool`, and every compiler rejects a result of `int`.

## Evidence

`tests/classes/comparisons/eq-reference-to-bool-result.cpp`,
`eq-cv-bool-result.cpp`, and `reversed-eq-needs-bool.cpp`. Clang reports
that the return type `bool &` of the selected `operator==` is not
`bool`.

## Options

1. The result must have the type `bool`, with or without cv-qualifiers.
   A reference to `bool` is not valid (Clang, EDG).
2. A reference to `bool` is also valid (GCC).

## Provisional answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the cv-qualified
results that all accept are values of type `bool`.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
