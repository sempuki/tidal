# Q-0011: Is using an entity with no definition `no-diagnostic` or `reject`?

**Status:** provisional
**Affects:** LNK-7, `tests/declarations/storage-linkage/missing-definition.cpp`

## Context

A program that calls a function declared in every translation unit but
defined in none compiles unit by unit without any diagnostic. Linking fails
with "undefined reference to" on the matrix compilers' linkers. The
compilers can't see the problem, because no single unit shows that the
definition is missing.

## Evidence

`tests/RESULTS.md`, rows `missing-definition.cpp` (link-fail on every
compiler that links) and `unused-missing-definition.cpp` (runs, because
an unused declaration needs no definition).

## Options

1. `no-diagnostic`. The rule can be broken without any implementation being
   required to report it, as with LNK-6.
2. `reject`, with the linker counted as part of the implementation. Every
   linker in the matrix reports the error.

## Provisional answer

No-diagnostic (option 1). Whether a use needs a definition can depend on what the
optimizer removes, so a link failure is likely but not guaranteed. A test
that shows an unreported case would settle the question.

## Opinions
