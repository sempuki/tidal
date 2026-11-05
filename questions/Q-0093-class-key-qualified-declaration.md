# Q-0093: Is a declaration of only a class key and a qualified name valid?

**Status:** settled
**Affects:** TYN-7, `tests/declarations/type-names/elaborated-qualified-declaration-only.cpp`

## Context

After `namespace n { struct S; }`, the declaration `struct n::S;`
consists only of a class key and a qualified name. Clang and EDG reject
it. GCC accepts it.

## Options

1. The declaration is not valid.
2. The declaration is valid and has no effect.

## Answer

No; the declaration is not valid (option 1). Clang and EDG agree.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. TYN-P2 and TYN-P3 say how a class key restricts lookup and when a class is complete, and neither says which forms a declaration of only a class key and a name can take. GCC 16 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). TYN-P2 and TYN-P3 fail step 3, because neither says which forms a declaration of only a class key and a name can take, so option 2 also fits. Item PQ widened TYN-P2: a declaration of only a class key and a plain name declares the class in the current scope, and with a qualified name the declaration declares nothing and is rejected. TYN-P2 now lists TYN-7. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses TYN-P2.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`), under decision 0056. The review applied TYN-P2 to the program of `elaborated-qualified-declaration-only.cpp` without the rule text and reached the provisional answer: a class key with a qualified name alone declares nothing, so the declaration is rejected.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and TYN-P2 gives the same answer by the test of the style guide (section 3), as the review's use shows. GCC 16 still differs, and the tests keep its overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A declaration of only a class key and a name is valid only in a few forms, and each form has a plain identifier or a template name with arguments. A qualified name is not one of them, except in an explicit specialization or an explicit instantiation. A diagnostic is required.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1, and a diagnostic is required. The review notes that the reference excepts an explicit specialization and an explicit instantiation, which can have a similar shape. Each of those starts with `template`, so it is not a declaration of only a class key and a qualified name, and a test now checks that every compiler accepts them.
