# Q-0020: How should tests record a compiler that doesn't implement a feature yet?

**Status:** provisional
**Affects:** `documentation/specification-format.md`, ATTR-10, ATTR-13, tests that use `[[assume]]`

## Context

Clang 18 doesn't implement `[[assume]]`. Clang 18 treats the attribute as
an unknown attribute and ignores it, so tests that expect a rejection get an
acceptance. The compiler predates the feature, so the difference says
nothing about the language. Clang 22 implements the attribute and agrees
with GCC and EDG on every `assume` test. Clang 18 accepts each program
that the tests expect to be rejected, a comma in an assumption included
(`tests/declarations/attributes/assume-comma.cpp`), and its sanitizer
does not stop at a false assumption
(`tests/declarations/attributes/assume-false-sanitizer.cpp`).

Clang 18 also rejects a `constexpr` function that can never give a
constant (CE-12, CX-3). The newer compilers accept the function
(`tests/declarations/constexpr/function-never-constant.cpp`). Clang 18
also rejects a `constexpr` function with a non-literal parameter type
(`function-non-literal-param.cpp`) or a non-literal return type
(`non-literal-return.cpp`), and Clang 22 accepts both.

GCC 14 and Clang 18 destroy the temporaries of a range expression in a
range-based `for` before the body executes. GCC 16 and Clang 22 keep
them until the loop ends, as C++23 changed the rule (RFOR-4,
`tests/statements/range-for/temporary-in-range-expression.cpp`).

## Options

1. A per-compiler expectation (`tidal-expect-clang-18: accept`) with a note
   naming this question. Keeps the compiler in the matrix for everything
   else.
2. A new directive listing compilers a test should skip.
3. Drop older compilers from the matrix once they lack a feature in scope.

## Provisional answer

Per-compiler expectation naming this question (option 1). This option needs no new tooling, and the note makes the reason
visible in the test.

Kept open on 2026-11-03 under decision 0054, because the review finds the reference silent.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) finds the reference silent. The question concerns Tidal's test format.
