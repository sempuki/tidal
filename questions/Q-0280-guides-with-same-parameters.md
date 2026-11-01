# Q-0280: Can two deduction guides have the same parameters and different results?

**Status:** provisional
**Affects:** CTAD-12, `tests/templates/class-template-deduction/guide-same-parameters.cpp`, `guide-template-same-parameters.cpp`

## Context

`Box(int) -> Box<long>;` and `Box(int) -> Box<short>;` are two guides
that are not templates. `template <class T> Box(T*) -> Box<long>;` and
`template <class T> Box(T*) -> Box<short>;` are two guide templates.

| Compiler | Guides that are not templates | Guide templates |
|---|---|---|
| Clang 18, 22, host | rejects: functions that differ only in the return type | accepts; a use is ambiguous |
| GCC 14, 16, host | rejects: an ambiguating new declaration | accepts; a use is ambiguous |
| EDG 7.0 | accepts | accepts; a use is ambiguous |

## Evidence

The tests above. Item 37 of the gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` says that two guides
for one class template with equivalent parameter lists are not valid
if either is reachable from the other, even with different results. It
predicts a rejection of both pairs.

## Options

1. Two guides that are not templates with the same parameters are not
   valid. Two guide templates are valid, and a use can be ambiguous.
   Clang and GCC give this result.
2. Both pairs are not valid, as the gap review reads.
3. Both pairs are valid (EDG).

## Provisional answer

Templates yes, others no; Clang and GCC agree, EDG differs (option 1). The guides follow the rules for the overloading of functions
and function templates, and two of three implementations agree on each
pair. No compiler rejects the guide templates.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
