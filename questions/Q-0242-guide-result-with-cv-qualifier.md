# Q-0242: Can the result of a deduction guide have a cv-qualifier?

**Status:** provisional
**Affects:** CTAD-12, `tests/templates/class-template-deduction/guide-result-qualified.cpp`

## Context

`template <class T> Box(T*) -> const Box<T>;`

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: the result is not a specialization of the template |
| EDG 7.0 | rejects: the return type must name a specialization directly |
| GCC 14, 16, host | accepts |

## Evidence

The test above.

## Options

1. The result must be a template-id of the class template, with no
   cv-qualifier (Clang, EDG).
2. A cv-qualifier is allowed and has no effect (GCC).

## Provisional answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the result of a guide
names the specialization that a declaration gets. A declaration adds
its own cv-qualifiers (CTAD-1).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
