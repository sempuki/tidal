# Q-0170: Is `int()` a type or a value when the parameter is a constant parameter?

**Status:** settled
**Affects:** TARG-7, TARG-8

## Context

`template <int N> struct Num;` and the use `Num<int()>`. The argument
`int()` can be read as a function type or as a value-initialized `int`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: the argument is the function type `int ()` |
| GCC 14, 16, host | rejects: a type for a constant parameter |
| EDG 7.0 | accepts, with the value `0` |

## Evidence

`tests/templates/arguments/type-or-expression-constant.cpp`, with an
override for EDG. `tests/templates/arguments/type-or-expression.cpp`
shows that all compilers read `int()` as a type for a type parameter.

## Options

1. The type reading wins in every template argument, and the argument
   is not valid for a constant parameter (Clang, GCC).
2. The kind of the parameter decides the reading (EDG).

## Answer

A type; Clang and GCC agree, EDG reads the value 0 (option 1). Clang and GCC agree, and the reading of an argument then does
not depend on the template's parameters.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 read `int()` as a type and reject the argument for the constant parameter, and EDG 7.0 reads the value 0, so two families disagree. TARG-P2 supports the provisional answer. An argument must be of the kind that its parameter takes, and TARG-7 lists a type for a constant parameter as not valid. The form of the argument decides its reading, and the parameter then checks the kind. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) supports this answer with no doubt, and TARG-P2 gives the same answer: the form of an argument decides its reading, and the parameter then checks the kind. EDG 7.0 still differs. Tidal records EDG's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. When a template argument can be read as a type or as an expression, the reference reads it as a type, whatever the kind of the parameter.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
