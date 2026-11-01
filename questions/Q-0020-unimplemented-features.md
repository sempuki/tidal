# Q-0020: How should tests record a compiler that doesn't implement a feature yet?

**Status:** provisional
**Affects:** `documentation/specification-format.md`, ATTR-10, ATTR-13, tests that use `[[assume]]`

## Context

Clang 18 doesn't implement `[[assume]]`. Clang 18 treats the attribute as
an unknown attribute and ignores it, so tests that expect a rejection get an
acceptance. The compiler predates the feature, so the difference says
nothing about the language. Clang 22 implements the attribute and agrees
with GCC and EDG on every `assume` test.

## Options

1. A per-compiler expectation (`tidal-expect-clang-18: accept`) with a note
   naming this question. Keeps the compiler in the matrix for everything
   else.
2. A new directive listing compilers a test should skip.
3. Drop older compilers from the matrix once they lack a feature in scope.

## Provisional answer

Option 1. This option needs no new tooling, and the note makes the reason
visible in the test.

## Opinions
