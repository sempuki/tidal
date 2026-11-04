# Q-0252: When does an unevaluated operand instantiate a constexpr function template?

**Status:** settled
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

## Answer

In a braced initializer; EDG and GCC differ on single rows (option 1). Two of three implementations agree on each row, and the gap review supports the braced rows.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and INST-P1 gives the same answer: a braced initializer that names a `constexpr` function instantiates it, and `decltype` and `sizeof` otherwise do not. EDG 7.0 still differs on the braced conditional, and GCC 16 on `sizeof`. Tidal records each result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A `constexpr` function is instantiated when an expression that may be constant evaluated names it. That covers potentially evaluated expressions, expressions that must be constant evaluated, and each direct element of a braced list, with their subexpressions, also on a branch that is not taken. A plain operand of `sizeof` is none of these, so GCC's instantiation there is a defect. The parenthesized cast is not a braced list. EDG's result on the braced conditional row is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
