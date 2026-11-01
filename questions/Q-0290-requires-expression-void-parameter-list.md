# Q-0290: Does `(void)` give a requires expression no parameters inside a template?

**Status:** provisional
**Affects:** CPT-28, `tests/templates/concepts/requires-parameters-void-template.cpp`

## Context

A concept is `requires (void) { T(); }`, and the program checks it for
`int`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | `false` |
| GCC 14, 16, host | `true` |
| EDG 7.0 | `true` |

## Evidence

The test above. Outside a template, every compiler accepts
`requires (void) { 1; }` with the value `true`
(`requires-parameters-void.cpp`). The gap review of concepts and
constraints (item 23) says that the parameter list follows the rules
for a function's parameter list.

## Options

1. `(void)` means no parameters, as in a function (GCC, EDG).
2. The requires expression is `false` inside a template (Clang).

## Provisional answer

Option 1. Two of three implementations agree, and every compiler
accepts the form outside a template.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
