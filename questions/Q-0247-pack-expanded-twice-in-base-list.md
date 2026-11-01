# Q-0247: Is a base list that expands the same pack twice diagnosed before instantiation?

**Status:** provisional
**Affects:** DEP-37, `tests/templates/dependent-names/base-pack-expanded-twice.cpp`

## Context

`template <class... T> struct Mix : T..., T... {};` is never
instantiated. Any non-empty pack names a base twice.

| Compiler | Unused template | `sizeof(Mix<A>)` |
|---|---|---|
| Clang 18, 22, host | rejects: the base is specified more than once | rejects |
| EDG 7.0 | rejects: duplicate base class name | rejects |
| GCC 14, 16, host | accepts | rejects: duplicate base type |

## Evidence

The test above. Item 21 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says
that the implementation reports this error at the definition, as an
error for every size of the pack.

## Options

1. The implementation rejects the template at the definition (Clang, EDG).
2. DEP-33 covers the template, and no diagnostic is required (GCC).

## Provisional answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations and the gap review agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
