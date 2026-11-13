# Q-0020: How should tests record a compiler that doesn't implement a feature yet?

**Status:** provisional
**Affects:** `documentation/specification-format.md`, ATTR-10, ATTR-13, tests that use `[[assume]]`
**Waits for:** a review (silent)

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

Kept open on 2026-11-11 (decision 0063, item RV8). The gap review of the phase 15 text on expressions names the question at CTAD-17 and does not read the question of the test format, so the last reading of the question stays silent. `pending.md` row 4 holds the CTAD-17 case as a rule item.

Evidence on 2026-11-11 (decision 0063, item RV8): the CTAD divergence line for this question and a CTAD-17 Note record the review's reading of the guide case. The test's override for Clang 18 still names this question.

Named on 2026-11-12 (decision 0066, item RV12). Part B of the gap review of the phase 18 text reads Clang 18's answer in `copy-init-guide-user-conversion.cpp` as the conforming one, so a missing feature is the wrong record for its override (RV12-18). {question}`Q-0457` now holds the case. Ledger row RV12-18 plans the change of the test's note and the CTAD lines in part 2 of item RV12. The answer and the **Waits for:** line do not change.

Evidence on 2026-11-12 (decision 0066, item RV12): the note of `copy-init-guide-user-conversion.cpp` and the CTAD lines now name Q-0457 for Clang 18's override, and no longer name this question (RV12-18). The answer and the **Waits for:** line do not change.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) finds the reference silent. The question concerns Tidal's test format.

### Gap review, 2026-11-11 (phase 15 text, expressions, overloading, and templates)

The source-reader gap review of the phase 15 text on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`) names the question in its reading of CTAD-17 (RV8-B-5) and does not read the question of the test format. The review reads the second bullet of CTAD-17 against the reference. The guides act as the constructors of a class, and nothing in the reference exempts them from the limit on a user-defined conversion for the first parameter in copy initialization. Read so, Clang 18, whose override in `copy-init-guide-user-conversion.cpp` names this question, follows the text, and Clang 22, GCC, and EDG do not. The review adds that the choice of a guide runs no conversion, which may explain the newer compilers, and that the text does not make that distinction.

### Read of the RV8 repair diff, 2026-11-11

The source-reader read of the RV8 repair diff (`reports/review/2026-11-11-read-of-rv8-repair-diff.md`) reads the Q-0020 line on the class template deduction page: agrees as a record. The line states RV8-B-5 correctly. The text keeps the compilers' answer for CTAD-17, and the CTAD-17 Note and `pending.md` row 4 record the review. The read gives no new reading of the question's answer.

### Read of the RV10 repair diff, 2026-11-11

The source-reader read of the RV10 repair diff (`reports/review/2026-11-11-read-of-rv10-repair-diff.md`) reads the Q-0020 line and the split row of the enumerations page, which record Clang 18's rejection of `using-enum-alias-template.cpp`: no reference content. The read gives no new reading of the question's answer.

### Gap review, 2026-11-12 (phase 18 text)

The source-reader gap review of the phase 18 text (`reports/review/2026-11-12-gap-review-phase-18.md`) names the question in Part B, on RV8-B-5 at CTAD-17 (RV12-18). The override of `copy-init-guide-user-conversion.cpp` for Clang 18 names this question as a missing feature. The review reads Clang 18's rejection as the reference's answer, so that record does not fit, and the review asks for a question of its own. The review gives no reading of the question of the test format.
