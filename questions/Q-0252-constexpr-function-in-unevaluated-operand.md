# Q-0252: When does an unevaluated operand instantiate a constexpr function template?

**Status:** provisional
**Affects:** INST-7, `tests/templates/instantiation/function-unevaluated-braced-untaken.cpp`, `tests/templates/instantiation/function-unevaluated-sizeof-constant.cpp`

## Context

`fail<T>` is a `constexpr` function template whose body fails a
`static_assert`.

| Operand | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| `decltype(int{fail<int>()})` | instantiates | instantiates | instantiates |
| `decltype(char{true ? 1 : fail<int>()})` | instantiates | instantiates | does not |
| `decltype(char(true ? 1 : fail<int>()))` | does not | does not | does not |
| `decltype(fail<int>() + 1)` | does not | does not | does not |
| `sizeof(fail<int>() + 1)` | does not | instantiates | does not |

## Evidence

The tests above, `function-unevaluated-braced.cpp`, and
`function-unevaluated-parentheses.cpp`. Item 26 of the gap review
`reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says that a `constexpr` function is instantiated
whenever an expression may need its value in a constant evaluation,
also inside an unevaluated operand.

## Options

1. A braced initializer instantiates each `constexpr` function that it names, also on a branch that is not taken. Other unevaluated operands do not (Clang, and GCC except for `sizeof`).
2. A braced initializer instantiates a `constexpr` function only on a branch that is taken (EDG).
3. Each unevaluated operand instantiates the `constexpr` functions that it names, except `decltype` (GCC).

## Provisional answer

Option 1. Two of three implementations agree on each row, and the gap review supports the braced rows.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
