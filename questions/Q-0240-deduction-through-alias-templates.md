# Q-0240: Can a program deduce the arguments of a class template through an alias template?

**Status:** provisional
**Affects:** CTAD-14, `tests/templates/class-template-deduction/alias-template.cpp`, `alias-template-fixed-argument.cpp`, `alias-template-fixed-argument-exact.cpp`, `alias-template-guides.cpp`, `alias-template-unreachable.cpp`, `template-template-parameter-alias.cpp`, `list-single-specialization-alias.cpp`

## Context

`template <class T> using Same = Pair<T, T>;` and then `Same s(1, 2);`.
A second form fixes an argument, as in
`template <class U> using IntPair = Pair<int, U>;`, and a third form
names an alias of an alias.

| Compiler | Alias that passes its parameters on | Alias that fixes an argument, alias of an alias, argument of the fixed type | The same, argument that converts to the fixed type |
|---|---|---|---|
| Clang 22, host | accepts | accepts | accepts |
| GCC 14, 16, host | accepts | accepts | accepts |
| EDG 7.0 | accepts | accepts | rejects: cannot deduce class template arguments |
| Clang 18 | rejects: argument deduction only allowed for class templates | rejects | rejects |

## Evidence

The tests above and the grid `forms.json` (column "alias template").

## Options

1. Deduction works through an alias template that stands for a
   specialization of a class template, in each form (Clang 22, GCC).
2. Deduction through an alias template is not valid (Clang 18).
3. Deduction works through each alias, but an argument for a fixed
   position must have exactly the fixed type (EDG 7.0).

## Provisional answer

Yes; Clang 22 and GCC agree, Clang 18 and EDG 7.0 differ (option 1). The newest compilers of two families agree, and the source
reader's inventory (items B2 to B8) describes the derived guides.
Clang 18 and EDG 7.0 are older or partial implementations.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept all three forms, and EDG 7.0 rejects an argument that converts to the fixed type, so two families disagree on that form. Clang 18 rejects every form. CTAD-P2 supports the provisional answer. An alias template gets the guides of its class template, matched to the alias (CTAD-14), and a guide's parameter of a fixed type takes an argument that converts to it, as any function parameter does. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. The guides for the alias come from the guides of the class template. A fixed argument becomes a plain parameter type in the new guide, so an argument that converts to that type is accepted.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
