# Q-0296: Must an implementation reject a later declaration whose new default comes before a parameter that has no default in any declaration?

**Status:** settled
**Affects:** TPL-14, `tests/templates/declarations/defaults-merge-gap.cpp`

## Context

`template <class T, class U> struct Pair;` and then
`template <class T = char, class U> struct Pair;`. No declaration gives
`U` a default.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: a template parameter misses a default |
| EDG 7.0 | rejects: the default is not at the end |
| GCC 14, 16, host | accepts, also with a definition and a use |

GCC rejects the same head on a first declaration. Every compiler accepts
declarations that add defaults from right to left
(`tests/templates/declarations/defaults-merge-gap-later.cpp`).

## Evidence

The test above. The front end's round-8 report (finding G73) asked
whether TPL-14 counts the defaults of earlier declarations.

## Options

1. TPL-14 counts the defaults of the declaration together with those of
   the earlier declarations, and the program is rejected (Clang, EDG).
2. TPL-14 counts only the defaults written on one declaration (GCC).

## Answer

Yes, TPL-14 counts the defaults of all declarations; Clang and EDG agree, GCC differs (option 1). Clang and EDG agree, and TPL-15 already merges the defaults
of all declarations.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and TPL-P3 gives the same answer: the defaults of all declarations add up, and in a class template only a trailing parameter can be left out. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. The defaults of all reachable earlier declarations merge, as default function arguments do, and each parameter after one with a default must have a default. With the merge, the second parameter has no default in any declaration. The rule is not exempt from diagnosis.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
