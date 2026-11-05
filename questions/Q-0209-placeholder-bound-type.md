# Q-0209: What type does a placeholder constant parameter get from an array bound?

**Status:** settled
**Affects:** DED-11, `tests/templates/deduction/match-bound-type.cpp`

## Context

A function template has `template <auto N>` and takes `int (&)[N]`. A
call passes an array of three `int`. `N` is 3 on every compiler. The
type of `N` differs.

| Compiler | Type of `N` |
|---|---|
| Clang 18, 22, host | the type of a `sizeof` expression |
| EDG 7.0 | the type of a `sizeof` expression |
| GCC 14, 16, host | `int` |

## Evidence

`tests/templates/deduction/match-bound-type.cpp`, with an override for
GCC. The sanitized Clang source (clang-sema-deduction) deduces a bound
from a braced list with the size type. Item E2 of the deduction
inventory describes the behavior.

## Options

1. The bound has the type of a `sizeof` expression (Clang, EDG).
2. The bound has the type `int` (GCC).

## Answer

The type of `sizeof`; Clang and EDG agree, GCC gives `int` (option 1). Two of three implementations and the inventory agree, and the
number of elements of an array has the type of `sizeof` elsewhere.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. DED-P3 gives the value of `N` and not its type.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). DED-P3 failed step 3, because DED-P3 gave the value of `N` and not its type. Item PQ widened DED-P3. A value keeps the type it has at its place, and a bound counts elements, so a bound gives an `auto` parameter the type of a `sizeof` expression. The new test `match-argument-type.cpp` checks the same model for a class template argument. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses DED-P3.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied DED-P3 to the question's program, a placeholder bound deduced from an array (`match-bound-type.cpp`), without the rule text: a bound counts elements, so a value deduced from it has the type of a size expression. That is the provisional answer, and the model rules out the other options.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and DED-P3 gives the same answer under the test of the style guide (section 3). GCC 16 still differs, and the tests keep their overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. The bound of an array type has the standard size type, and a constant parameter declared with a placeholder takes the type of the deduced value. GCC's result is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. The reference's own example deduces the type of a constant parameter from an array bound as the size type.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
