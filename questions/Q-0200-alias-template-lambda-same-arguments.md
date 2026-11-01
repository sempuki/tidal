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

Option 1. GCC and EDG agree, and an alias template specialization names
one type (TPL-28).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
