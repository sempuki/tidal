# Q-0078: Can the operand of noexcept narrow when it is converted to bool?

**Status:** provisional
**Affects:** EXS-7, `tests/declarations/exception-specifications/narrowing-operand.cpp`, `tests/declarations/exception-specifications/pointer-operand.cpp`

## Context

`noexcept(2)` and `noexcept(&x)` give the specifier an operand that is not `bool`. Clang and GCC reject both. EDG 7.0 converts the operand to `bool` and accepts both.

GCC 14 and 16 give the same result, and so do Clang 18 and 22. The host
GCC 16.2 and Clang 22.1 agree with them. No newer version settles the
split.

EDG 7.0 also accepts `explicit(2)` and converts the condition to `bool`
(CE-11).

## Options

1. The operand must convert to `bool` without narrowing.
2. Any conversion to `bool` is valid.

## Provisional answer

No; the operand must convert to `bool` without narrowing (option 1). Clang and GCC agree. `static_assert` and `if constexpr` permit narrowing, so the rule for `noexcept` differs from the rules for those two.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. EXS-P1 says that the operand is a constant decided at translation time, and no principle says which conversions to `bool` it allows. EDG 7.0 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). EXS-P1 fails step 3, because it says only that the operand is a constant, so option 2 also fits. CE-P4 gives the answer in its table, and no principle or rule of the exception specifications page cites CE-P4. Item PQ widened EXS-P1: the operand needs an exact `bool` value as in the other places of CE-P4, so its conversion must not narrow, and EXS-P1 now cites CE-P4. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses EXS-P1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The operand must be a constant expression converted to `bool` with only a short list of conversions, and the conversions from an integer or a pointer to `bool` are not in the list. The C++23 change that relaxed this for `static_assert` and `if constexpr` did not cover the exception specifier.
