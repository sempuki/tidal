# Q-0215: Are default template arguments substituted before the check of parameters with no template parameter?

**Status:** provisional
**Affects:** DED-28, `tests/templates/deduction/substitution-default-before-check.cpp`

## Context

`template <class T, class U = typename Bad<T>::type> One pick(int*, T)`
is called as `pick(1, 1)`, with a fallback `pick(...)`. The body of
`Bad<int>` has an error. The first argument `1` does not convert to
`int*`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | error in the body of `Bad<int>` |
| GCC 14, 16, host | error in the body of `Bad<int>` |
| EDG 7.0 | the fallback, with no error |

Clang and GCC substitute the default of `U` first. The default
instantiates `Bad<int>`, and the error in its body makes the program
not valid (DED-29). EDG checks the first argument first, removes the
template, and never substitutes the default.

When the class appears in a function parameter type instead of a
default template argument, every compiler checks the first argument
first and chooses the fallback
(`substitution-nondependent-first.cpp`).

## Evidence

`tests/templates/deduction/substitution-default-before-check.cpp`, with
an override for EDG. Item O2 of the gap review of 2026-10-31
(`reports/review/2026-10-31-gap-review-deduction-and-ordering.md`)
describes the order of option 1.

## Options

1. The defaults of the template parameters are substituted, then the
   parameters with no template parameter are checked, then the function
   type is substituted (Clang, GCC).
2. The parameters with no template parameter are checked before any
   substitution (EDG).

## Provisional answer

Yes; Clang and GCC agree, EDG checks first (option 1). Clang and GCC agree, and the gap review describes the same
order.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
