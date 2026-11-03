# Q-0208: Does the noexcept of a function type give a value to a bool constant parameter?

**Status:** provisional
**Affects:** DED-11, `tests/templates/deduction/match-noexcept.cpp`

## Context

A function template takes `void (*)() noexcept(B)`, where `B` is a
`bool` constant parameter. A call passes a function that is `noexcept`,
or one that is not.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | `B` is `true` or `false` as the argument's type says |
| GCC 14, 16, host | the same as Clang |
| EDG 7.0 | no match: the template is not viable |

## Evidence

`tests/templates/deduction/match-noexcept.cpp`, with an override for
EDG. EDG reports that no instance of the function template matches the
argument list. Item E5 of the deduction inventory describes the
behavior.

## Options

1. The `noexcept` of a function type is a position that gives a value
   to a `bool` constant parameter (Clang, GCC).
2. The `noexcept` gives no value, and the parameter needs an explicit
   argument (EDG).

## Provisional answer

Yes; Clang and GCC agree, EDG differs (option 1). Two of three implementations and the inventory agree. The
`noexcept` is part of the function type (ICS-1 lets it drop in
a conversion), so the position matches like the other parts of the type.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 give `B` the value of the argument's `noexcept`, and EDG 7.0 finds no match, so two families disagree. DED-P3 supports the provisional answer. Matching takes a type apart along its structure, and its table lists `noexcept(B)` of a function type as a position that gives `B` a value (DED-11). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
