# Q-0040: Does parenthesized aggregate initialization extend the life of a temporary bound to a reference member?

**Status:** settled
**Affects:** REF-4, REF-5, `tests/declarations/initialization/references/aggregate-member-extends.cpp`, `aggregate-member-paren-not-extended.cpp`

## Context

With braces, `Holder h{Note{5}};` keeps the temporary alive as long as `h` on every compiler. With parentheses, `Holder h(1);` followed by a read of the member during constant evaluation is rejected by Clang and GCC as a read after the temporary's lifetime, and accepted by EDG 7.0.

## Options

1. Braces extend the lifetime; parentheses do not. EDG's acceptance is a possible defect.
2. Both forms extend the lifetime.

## Answer

Only braces extend the lifetime; parentheses do not (option 1). Clang and GCC agree, and the review summary describes the same difference.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. REF-P2 lists the bindings that extend a temporary and does not name parenthesized aggregate initialization.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). REF-P2 fails step 3, because it lists a binding through a member and does not say whether the form of the aggregate's initializer matters, so option 2 also fits. Item PQ widened REF-P2: braces bind the reference members as part of the declaration and extend the temporaries, parentheses pass the members as a call passes arguments and do not, and a `new` expression extends no temporary. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses REF-P2.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`), under decision 0056. The review applied REF-P2 to the program of `aggregate-member-extends.cpp` and `aggregate-member-paren-not-extended.cpp` without the rule text and reached the provisional answer: braces extend the temporary, and with parentheses the temporary ends with the full expression, so the later read in constant evaluation is outside its lifetime and is rejected.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and REF-P2 gives the same answer by the test of the style guide (section 3), as the review's use shows. EDG 7.0 still differs, and the tests keep its overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. A temporary bound to a reference member of an aggregate that a parenthesized list initializes lasts only to the end of the full expression. With braces the usual extension applies, and EDG's acceptance is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1. The reference ends a temporary bound to a reference member at the end of the full expression when the aggregate is initialized from a parenthesized list.
