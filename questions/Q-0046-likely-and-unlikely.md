# Q-0046: Is `[[likely]] [[unlikely]]` on one statement rejected or ignored?

**Status:** provisional
**Affects:** ATTR-2, `tests/declarations/attributes/likely-and-unlikely.cpp`

## Context

Clang 22 and 18 and EDG 7.0 reject `likely` and `unlikely` on the same statement. GCC 16 and 14 accept the pair and ignore the second attribute with a warning ("ignoring attribute 'unlikely' after earlier 'likely'"). The host GCC 16.2 and Clang 22.1 give the same results.

## Evidence

`tests/declarations/attributes/likely-and-unlikely.cpp`, with an
override for GCC.

## Options

1. Rejected.
2. Ignored, with a recommended warning.

## Provisional answer

Rejected (option 1). Two of three implementations reject the pair.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. ATTR-P1 says that each standard attribute has a fixed set of places where it can go, and it does not say whether two attributes can share a statement. GCC 16 accepts the pair with a warning.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). ATTR-P1 failed step 3, because the model did not say whether two attributes can share a statement. ATTR-P1 now has a table of the places of the standard attributes, which allows at most one of `likely` and `unlikely` on one statement or label, and says that the implementation rejects the pair because the two hints contradict each other. The review reads with no doubt, and GCC 16 is the one newest family that differs. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses ATTR-P1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. `likely` and `unlikely` must not appear in one sequence of attribute specifiers, and two adjacent bracketed specifiers form one such sequence. The rule is not exempt from diagnosis. GCC's warning alone does not meet it, because GCC then accepts the program.
