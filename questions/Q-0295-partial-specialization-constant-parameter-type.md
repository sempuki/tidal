# Q-0295: Must a constant argument's type fit the declared type of a partial specialization's constant parameter with no conversion?

**Status:** provisional
**Affects:** DED-12, TSP-14, `tests/templates/specialization/partial-constant-type-exact.cpp`, `tests/templates/specialization/partial-placeholder-pattern-const.cpp`, `tests/templates/specialization/partial-placeholder-pattern-variable.cpp`

## Context

A class template has the parameter `auto V`. A partial specialization
declares a constant parameter with a narrower type and uses it as the
argument list, as in `template <int V> struct Kind<V>` or
`template <const auto* P> struct Kind<P>`.

| Case | Clang 22, host | Clang 18 | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|---|
| `auto* P`, argument `&n` or `1` | matches pointers only | matches pointers only | matches pointers only | matches pointers only |
| `int V`, argument `1L` or `'c'` | partial specialization | partial specialization | primary template | primary template |
| `const auto* P`, argument of type `int*` | partial specialization | primary template | primary template | primary template |
| variable template, `auto* P` | as for a class | rejects: no argument is specialized | as for a class | as for a class |

## Evidence

The three tests above. The front end's round-8 report (finding G71)
asked how the form `auto*` constrains a match. In a function template,
every compiler fails the pair when the types differ
(`tests/templates/deduction/match-constant-type-differs.cpp`,
`tests/templates/deduction/match-constant-placeholder-pattern.cpp`).

## Options

1. The argument's type must fit the declared type with no conversion,
   as in a function template (GCC, EDG).
2. The argument converts to the declared type, as an argument of a
   specialization does (Clang 22).

## Provisional answer

Yes, as in a function template (DED-12); GCC and EDG agree, Clang differs (option 1). GCC and EDG agree, and option 1 gives partial specializations
the matching rule of function templates (DED-12).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
