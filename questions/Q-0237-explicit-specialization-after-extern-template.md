# Q-0237: Can an explicit specialization follow an explicit instantiation declaration?

**Status:** provisional
**Affects:** INST-19, `tests/templates/instantiation/specialization-after-explicit-declaration.cpp`

## Context

`extern template int get(int);` and then `template <> int get(int)`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: explicit specialization after instantiation |
| GCC 14, 16, host | rejects: specialization after instantiation |
| EDG 7.0 | accepts |

Every compiler rejects an explicit specialization after an explicit
instantiation definition.

## Evidence

The test above and `specialization-after-explicit-definition.cpp`.

## Options

1. Not valid, as after a definition (Clang, GCC).
2. Valid, because the declaration instantiates nothing in the unit
   (EDG).

## Provisional answer

Option 1. Two of three implementations agree, and both forms of
explicit instantiation name the specialization as one that comes from
the template.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
