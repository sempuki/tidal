# Q-0090: Can constant evaluation use a reference whose referent is unknown, if it does not read through it?

**Status:** settled
**Affects:** CE-18, `tests/expressions/constant-expressions/unknown-reference-not-read.cpp`

## Context

A function that is not `constexpr` takes a reference parameter
and uses it to call a `constexpr` member function that does not read the
object, in the initializer of a `constexpr` variable. Clang 22, GCC 14
and 16, and EDG accept the program. Clang 18 rejects it.

## Options

1. The use is valid when the evaluation does not read through the
   reference.
2. Any use of such a reference makes the expression not constant.

## Answer

Yes; Clang 22, GCC, and EDG agree, Clang 18 rejects (option 1). The evaluation does not read through the reference (CE-18, CE-P1).

Settled on 2026-11-03 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. Clang 18 still rejects the program, and the test keeps its override for Clang 18. The front end's interpreter also accepts the program.

Kept on 2026-11-03. The source-reader gap review of question cluster 4 agrees with option 1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 4 (`reports/review/2026-11-03-gap-review-question-cluster-4.md`) reads the reference as supporting option 1. A reference whose referent is not known is treated as bound to some object that lives through the whole evaluation. The evaluation can use the reference as long as it reads no value through it and does not depend on the dynamic type. The reference marks valid a call of a member function that reads no member through such a reference, which is the case of the test. Clang 18 predates this change, which explains its rejection.
