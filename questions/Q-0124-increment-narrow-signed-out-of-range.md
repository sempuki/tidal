# Q-0124: Is `++` or `--` on a signed operand narrower than `int` defined when the new value does not fit the operand's type?

**Status:** provisional
**Affects:** UN-20, UN-21, PFX-12, PFX-18, `tests/expressions/unary/prefix-narrow-signed-wraps.cpp`, `tests/expressions/postfix/postfix-narrow-signed-wraps.cpp`
**Waits for:** a review (doubt)

## Context

An operand of type `short` or `signed char` is promoted to `int` for
the step. The new value then goes back into the operand's type. For a
`short` that holds 32767, `++` gives 32768 in `int`, and that value does
not fit in `short`.

During constant evaluation, Clang and GCC accept the step. The operand
then holds the value that the integer conversion gives (CONV-2), so
32767 becomes -32768. EDG 7.0 rejects the step and reports that the
value exceeds the range of the operand's type. At run time, Clang and
GCC give the converted value, and `-fsanitize=undefined` reports
nothing. Clang reports the change only with the separate
`-fsanitize=implicit-conversion` check, which is not a check for
undefined behavior.

For an operand of type `int` or wider, all three compilers reject an
overflow during constant evaluation (UN-21, PFX-18).

A compound assignment converts back in the same way (ASN-5). EDG 7.0
also rejects `s += 1` on a `short` that holds 32767 during constant
evaluation, and Clang and GCC give -32768. So
`tests/expressions/assignment/compound-narrow-operand-promoted.cpp`
checks the `short` case only at run time (finding X17-7).

## Evidence

`tests/RESULTS.md`, rows `prefix-narrow-signed-wraps.cpp` and
`postfix-narrow-signed-wraps.cpp`.

## Options

1. The step is done in the promoted type, and the result converts back
   to the operand's type. The behavior is defined. EDG differs.
2. A result outside the operand's type is undefined, as for `int`.
   Clang and GCC differ.

## Provisional answer

The step is defined and the value converts back; EDG differs (option 1). Two of three implementations take this behavior, and the
sanitizer of both agrees that the step is defined.

Kept open on 2026-11-03 under decision 0054, because the review reads with doubt.

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with a narrower doubt (DQ-4), and EDG 7.0 alone differs. The prefix form is supported, and the doubt is only on the postfix form.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1, with doubt. The prefix form is defined as a compound assignment that adds one, so the addition runs in the promoted type and the assignment converts the sum back, which wraps for every value. EDG's rejection in a constant expression is a defect. For the postfix form the text says only that the operand changes by adding one, and does not state that the postfix form uses the same promotion and conversion. The doubt is only in the wording of the postfix form.

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports with a doubt that remains (DQ-4). The prefix form is defined through the compound assignment, which supports the answer. The postfix form says only that the operand gains one, and it points the reader to the rules of addition and assignment. The postfix text also gives an implementation-defined value to an increment of a bit-field whose new value does not fit. The doubt is now only whether those pointers bring the promotion and the conversion back into the postfix form.
