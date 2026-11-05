# Q-0247: Is a base list that expands the same pack twice diagnosed before instantiation?

**Status:** provisional
**Affects:** DEP-33, DEP-37, `tests/templates/dependent-names/base-pack-expanded-twice.cpp`
**Waits for:** a use of DEP-P6

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

No diagnostic required; Clang and EDG diagnose, GCC does not (option 2). Only an empty pack makes the base list valid, and DEP-33 needs no diagnostic for such a template (DEP-P6). Each compiler's result lies within the rule.

Changed on 2026-11-03 to option 2. The review reads option 2, GCC 16 follows it, and DEP-P6 gives the same answer: a pack that every valid argument list leaves empty need not be diagnosed (DEP-33). DEP-33 now names the base list, and DEP-37 keeps only the operator function. `base-pack-expanded-twice.cpp` now cites DEP-33 and keeps its override for GCC.

Kept open on 2026-11-03 under decision 0054, because DEP-33 and DEP-37 changed on the same day, and a question settles only after a reader or a review has used the changed rule (decision 0055). Every compiler's result fits DEP-33.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) reads option 2. A variadic template whose only valid specializations need an empty pack makes the program not valid, with no diagnostic required. A base list that expands the same pack twice is valid only for an empty pack. So an implementation may report the error at the definition, as Clang and EDG do, and need not, as GCC does. A specialization with a non-empty pack needs a diagnostic, and every compiler rejects it. The earlier gap review's reading does not match this rule.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
