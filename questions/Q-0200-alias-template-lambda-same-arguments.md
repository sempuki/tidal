# Q-0200: Do two uses of one alias template specialization with a lambda give one closure type?

**Status:** provisional
**Affects:** TPL-46

## Context

`template <class T> using Closure = decltype([] {});`. The program
compares `Closure<int>` with `Closure<int>`.

| Compiler | `Closure<int>` and `Closure<int>` | `Closure<int>` and `Closure<char>` |
|---|---|---|
| Clang 18, 22, host | different types | different types |
| GCC 14, 16, host | the same type | different types |
| EDG 7.0 | the same type | different types |

## Evidence

`tests/templates/declarations/alias-template-lambda-same.cpp`, with an
override for Clang, and `alias-template-lambda.cpp`.

## Options

1. One specialization gives one closure type (GCC, EDG).
2. Each use of the alias gives a new closure type (Clang).

## Provisional answer

Yes; GCC and EDG agree, Clang gives two types (option 1). GCC and EDG agree, and an alias template specialization names
one type (TPL-28).

Kept open on 2026-11-03 under decision 0054, because the review finds the question unclear.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) finds the question unclear. A lambda in an alias template gives different closure types for different instantiations of the alias, which covers the second column, where every compiler agrees. Nothing says whether two uses of one alias specialization with the same arguments are one instantiation or two. The wording on instantiations points toward option 1, and Clang's reading of each use as a new substitution is also possible.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
