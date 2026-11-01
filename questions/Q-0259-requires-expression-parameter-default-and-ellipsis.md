# Q-0259: Can a parameter of a requires expression have a default argument, or the list end in an ellipsis?

**Status:** provisional
**Affects:** CPT-29, `tests/templates/concepts/requires-parameter-default.cpp`, `tests/templates/concepts/requires-parameter-ellipsis.cpp`

## Context

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects both |
| GCC 14, 16, host | accepts both |
| EDG 7.0 | rejects both |

## Evidence

The two tests above. Clang's diagnostic list has an error for each case (clang-diag-sema, clang-diag-parse).

## Options

1. Not valid (Clang, EDG). The parameters are only names with types,
   and no call ever gives them arguments.
2. Valid (GCC).

## Provisional answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and a default argument or an ellipsis has no use where nothing is called.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
