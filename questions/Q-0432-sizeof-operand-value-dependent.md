# Q-0432: Is `sizeof(N)` value-dependent when `N` is a constant template parameter?

**Status:** provisional
**Affects:** DEP-4, `tests/templates/dependent-names/sizeof-value-dependent-operand-call-bound-early.cpp`, `tests/templates/dependent-names/sizeof-value-dependent-operand-no-typename.cpp`
**Source:** the I6 report (`reports/clean-room/I6/report.md`, I6-6), the round 11 writer probe `reports/clean-room/roadmap-11/writer-probes/i6-instantiation-checks.json`.
**Waits for:** a first review

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
