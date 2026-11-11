# Q-0432: Is `sizeof(N)` value-dependent when `N` is a constant template parameter?

**Status:** provisional
**Affects:** DEP-4, `tests/templates/dependent-names/sizeof-value-dependent-operand-call-bound-early.cpp`, `tests/templates/dependent-names/sizeof-value-dependent-operand-no-typename.cpp`
**Source:** the I6 report (`reports/clean-room/I6/report.md`, I6-6), the round 11 writer probe `reports/clean-room/roadmap-11/writer-probes/i6-instantiation-checks.json`.
**Waits for:** a use of DEP-P2

## Context

`N` is a constant template parameter of type `int`, so `N` is
value-dependent and not type-dependent. `sizeof(N)` gives the size of
`int`, and `noexcept(N + 1)` gives `true`, whatever the argument. DEP-4
made an expression value-dependent when any operand is, which read
literally includes these operands.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| `Box<sizeof(N)>::type value = N;` in a function body, no `typename` | accepts | accepts | rejects, asks for `typename` |
| the same with `noexcept(N + 1)` | accepts | accepts | rejects |
| a call with the argument `Slot<sizeof(N)>{}`, and a better function declared after the template | binds at the definition | binds at the definition | finds the later function |
| the same with `noexcept(N + 1)` | binds at the definition | binds at the definition | finds the later function |

## Evidence

The tests in **Affects**, and the grid
`tests/templates/dependent-names/grids/sizeof-operand.json`.

## Options

1. The operands of `sizeof`, `alignof`, and `noexcept` do not make the
   expression value-dependent; only a dependent type or a
   type-dependent operand does (Clang, GCC).
2. A value-dependent operand makes these expressions value-dependent
   too (EDG).

## Provisional answer

The operands of `sizeof`, `alignof`, and `noexcept` do not make the
expression value-dependent (option 1). Clang 22 and GCC 16 agree, and EDG alone differs. DEP-4's last
bullet leaves out these operands.

Kept open on 2026-11-11 under decision 0054 (item RV7). The review supports the answer with no doubt, and only EDG differs. DEP-P2 says that a value-dependent operand does not make `sizeof`, `alignof`, or `noexcept` value-dependent, so the model rules out option 2. DEP-P2 changed in phase 14, so it gives no answer until a reader or a review uses it (decision 0055). The review reads DEP-4 and does not use DEP-P2.

Rule text on 2026-11-11 (decision 0062, item RV7): DEP-4 names `typeid` with `sizeof`, `alignof`, and `noexcept`, as the review reads (RV7-B-12). No test checks `typeid`, which stays referenced. DEP-P2 is unchanged.

## Opinions

### Gap review, 2026-11-11 (phase 14 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 14 wrote on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md`) reads the question: supports with no doubt (RV7-B-12). For `sizeof` of an expression or a type, `alignof`, `noexcept`, and `typeid`, the result is value-dependent only when the type is dependent or the operand is type-dependent. A value-dependent operand alone is not enough (option 1). DEP-4 states this for three of the operators and leaves out `typeid`.
