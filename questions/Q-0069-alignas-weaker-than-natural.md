# Q-0069: Can alignas make an alignment weaker than the natural one?

**Status:** provisional
**Affects:** ALN-2, `tests/declarations/alignment/weaker-than-natural.cpp`, `tests/declarations/alignment/weaker-on-class-member.cpp`, `tests/declarations/alignment/weaker-on-class.cpp`

## Context

A declaration such as `alignas(1) int x;` asks for an alignment weaker than the alignment of `int`. Clang 18 and 22 and EDG 7.0 reject the declaration. GCC 14 and 16 accept it with no warning, also with `-Wall -Wextra`. GCC keeps the natural alignment: a class with an `alignas(1) int` member still has alignment 4 and size 8 on x86-64, which is option 2. The split is the same for `alignas(1)` on a class, `struct alignas(1) S { int x; };`. GCC keeps alignment 4 for the class.

## Options

1. The program is not valid.
2. The weaker value has no effect.

## Provisional answer

No; the program is not valid (option 1). Clang and EDG agree.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. ALN-P1 says that `alignas` never makes an alignment weaker, and both options keep that promise: option 1 rejects the request, and option 2 ignores it. GCC 16 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). ALN-P1 failed step 3, because both options kept its promise that `alignas` never weakens an alignment. ALN-P1 now says that an `alignas` that asks for a weaker alignment than the natural one is rejected, which rules out option 2. The existing tests check the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses ALN-P1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. Alignment specifiers whose combined effect is weaker than the alignment the entity would have without them make the program not valid, and a diagnostic is required.
