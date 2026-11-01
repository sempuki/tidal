# Q-0222: Is a `new` expression valid when two placement deallocation templates match and neither is preferred?

**Status:** provisional
**Affects:** NEW-24, `tests/expressions/new-delete/throw-placement-templates-unordered.cpp`

## Context

A class has the placement allocation function
`operator new(size, Arena*)` and two deallocation function templates.
Both templates have the function parameters `(void*, T*)`. One template
has a second template parameter with a default argument. Both match
with `T = Arena`, and partial ordering prefers neither. The constructor
throws.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, warns that two match, and calls no deallocation function |
| GCC 14, 16, host | accepts and calls no deallocation function |
| EDG 7.0 | rejects: the `operator delete` is ambiguous |

## Evidence

`tests/expressions/new-delete/throw-placement-templates-unordered.cpp`,
with an override for EDG.

## Options

1. The program is valid, and no deallocation function is called
   (Clang, GCC). The storage is not freed, as with no match.
2. The program is not valid (EDG).

## Provisional answer

Yes, and no deallocation function is called; Clang and GCC agree, EDG rejects (option 1). Clang and GCC agree. The answer treats two matches as no
single match, the same as for a placement form with no match.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
