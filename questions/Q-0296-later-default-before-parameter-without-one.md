# Q-0296: Must an implementation reject a later declaration whose new default comes before a parameter that has no default in any declaration?

**Status:** provisional
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

## Provisional answer

Option 1. Clang and EDG agree, and TPL-15 already merges the defaults
of all declarations.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
