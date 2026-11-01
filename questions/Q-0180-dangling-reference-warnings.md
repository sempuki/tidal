# Q-0180: Which dangling references should an implementation warn about?

**Status:** provisional
**Affects:** REF-15, `tests/declarations/initialization/references/dangling-new-member-warns.cpp`

## Context

REF-15 recommends a warning when a reference outlives the temporary that
it binds to. The compilers warn about different cases.

| Program | Clang 18 | Clang 22, host | GCC 14 | GCC 16, host |
|---|---|---|---|---|
| `new Holder{1}`, with a reference member | warns (`-Wdangling-field`) | warns | no warning | no warning |
| `Holder h(1);`, with a reference member | no warning | warns (`-Wdangling`) | no warning | no warning |
| `const int& w = pass(1);` | no warning | no warning | warns under `-Wall` (`-Wdangling-reference`) | no warning |
| `const int& b = Box{2}.get();` | no warning | no warning | warns under `-Wall` | no warning |

The Clang warnings are on by default. GCC 16 no longer warns for the
last two cases, where GCC 14 does.

## Evidence

Probes on the six Clang and GCC compilers.

## Options

1. A recommended rule in general terms, with a test for the case that
   one compiler family warns about in every version (the `new` case).
2. No recommended rule until the compilers agree on a case.

## Provisional answer

A general recommended rule, tested on the `new` case (option 1). A recommended rule allows an implementation that does not
warn, and the `new` case is stable across the Clang versions.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
