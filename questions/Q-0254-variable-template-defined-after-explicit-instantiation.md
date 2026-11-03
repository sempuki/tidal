# Q-0254: Can a variable template's definition follow its explicit instantiation definition?

**Status:** provisional
**Affects:** INST-20, `tests/templates/instantiation/explicit-variable-before-definition.cpp`

## Context

`template <class T> extern int value;`, then
`template int value<int>;`, then `template <class T> int value = 2;`,
in one translation unit.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| EDG 7.0 | accepts |
| GCC 14, 16, host | rejects: no definition available |

Every compiler accepts the same order for a function template, and the
specialization is emitted
(`explicit-definition-before-template-definition.cpp`).

## Evidence

The tests above. Item 38 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says
that the definition must be in the unit and need not come first.

## Options

1. The definition can follow the explicit instantiation (Clang, EDG).
2. The definition of a variable template must come first (GCC).

## Provisional answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations and the gap review agree, and every compiler allows the order for a function.

The question stays open under decision 0051 (2026-11-03). GCC 16 rejects the order for a variable template, and Clang 22 and EDG 7.0 accept it, so two families disagree. Probes of 2026-11-03 found that every compiler accepts the same order for a class template's static data member, and every compiler rejects the explicit instantiation when the unit never defines the variable template. INST-P3 supports the provisional answer through INST-20. The definition must be in the translation unit, and the order does not matter for a function or a static data member on any compiler.

Kept on 2026-11-03. The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) agrees with option 1. INST-20 keeps the order free for a variable template.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) reads the reference as supporting option 1. For a variable template, a declaration must be reachable from the explicit instantiation, and the definition must be somewhere in the unit. Only a class template and its member classes need a reachable definition. A unit that never defines the variable template is not valid, and a diagnostic is required.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
