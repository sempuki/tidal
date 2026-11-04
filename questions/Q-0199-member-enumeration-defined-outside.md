# Q-0199: Can a member enumeration of a class template be defined after the class body?

**Status:** settled
**Affects:** TPL-42

## Context

A class template declares `enum class Kind : T;` and `enum Plain : int;`
in its body, and defines both after the body with the class's template
head.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | rejects under `-pedantic-errors`: the definition is an enumeration template |
| EDG 7.0 | accepts |

GCC accepts the program without `-pedantic-errors`. All compilers accept
a member class defined after the body.

## Evidence

`tests/templates/declarations/member-enum-outside.cpp`, with an
override for GCC, and `member-class-outside.cpp`.

## Options

1. The definition is valid (Clang, EDG).
2. The definition is not valid in strict mode (GCC).

## Answer

Yes; Clang and EDG accept, GCC rejects under -pedantic-errors (option 1). Clang and EDG agree, and GCC treats the form as an
extension, so GCC gives it the same meaning.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and TPL-P5 gives the same answer: a member of a class template can be defined outside the class with the class's head. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A template declaration can define a member enumeration of a class template, also outside the class, including one with a fixed base that depends on the parameter. GCC's pedantic error has no support.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
