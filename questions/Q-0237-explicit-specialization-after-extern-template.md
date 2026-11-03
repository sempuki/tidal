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

No; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and both forms of
explicit instantiation name the specialization as one that comes from
the template.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject the explicit specialization, and EDG 7.0 accepts it, so two families disagree. INST-P3 supports the provisional answer. Both forms of explicit instantiation say that the specialization comes from the template, and INST-19 is the exception of INST-P3 that keeps an explicit specialization from following either form. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
