# Q-0163: Does the result tie-breaker prefer the identity to a function pointer conversion?

**Status:** provisional
**Affects:** OVL-26, `tests/foundations/overload-resolution/tie-conversion-result-templates.cpp`, `tests/foundations/overload-resolution/tie-conversion-result-noexcept.cpp`

## Context

A class has two conversion functions. One gives `int (*)()`, the other
`int (*)() noexcept`. The class initializes an `int (*)()`. The first
result reaches the target with the identity, and the second with a
function pointer conversion.

| Compiler | Two plain functions | Two templates | Template `int (*)()`, plain `noexcept` |
|---|---|---|---|
| Clang 18, Clang 22 | the identity | the identity | the template, by the result |
| GCC 14, GCC 16 | the identity | the identity | the template, by the result |
| EDG 7.0 | ambiguous | ambiguous | the plain function |

For arguments, every compiler prefers the identity to a function pointer
conversion.

## Evidence

`tests/foundations/overload-resolution/tie-conversion-result-noexcept.cpp`
and `tie-conversion-result-templates.cpp`. Item O-3 of the gap review of
overload resolution
(`reports/review/2026-10-31-gap-review-overload-resolution.md`)
describes the identity as the better result.

## Options

1. The result sequences are compared as for arguments, so the identity
   wins (Clang, GCC).
2. A function pointer conversion ties with the identity in the result
   (EDG).

## Provisional answer

Yes; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, the gap review describes
it, and the answer matches the ranking of arguments.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
