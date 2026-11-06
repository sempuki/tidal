# Q-0295: Must a constant argument's type fit the declared type of a partial specialization's constant parameter with no conversion?

**Status:** provisional
**Affects:** DED-12, TSP-14, `tests/templates/specialization/partial-constant-type-exact.cpp`, `tests/templates/specialization/partial-placeholder-pattern-const.cpp`, `tests/templates/specialization/partial-placeholder-pattern-variable.cpp`
**Waits for:** a review (doubt)

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

Kept open on 2026-11-03 under decision 0054, because the review reads the answer with doubt.

Kept open on 2026-11-05 under decision 0054, because the review reads the answer with a doubt (gap review of the text phase 9 wrote, item RV3).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1, with doubt. A partial specialization matches by deduction, which fails when a constant has a type other than that of the matching parameter. When the primary template's parameter has a placeholder type, the text does not say clearly which type the rule compares with. The natural reading takes the argument's own type, which gives option 1. For `const auto*` against an `int*`, the text does not settle whether the added `const` is a mismatch.

### Gap review, 2026-11-05 (expressions and templates)

The source-reader gap review of the text phase 9 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) reads the question: supports with a doubt. Deduction fails when a constant's type differs from the type of the matching parameter of the named template. The reference does not say plainly what that type is when the primary parameter is a placeholder.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
