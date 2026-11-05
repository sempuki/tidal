# Q-0347: Can `[[likely]]` apply to a label that is an identifier?

**Status:** settled
**Affects:** ATTR-8, `tests/declarations/attributes/likely-identifier-label.cpp`

## Context

ATTR-8 says that `[[likely]]` and `[[unlikely]]` can apply to a
statement or a label. A label can be a `case` label, a `default` label,
or an identifier that a `goto` names.

GCC 14 and 16 and EDG 7.0 accept `[[likely]]` before an identifier
label and run the program as without the attribute. Clang 18 and 22
reject the program with "'likely' attribute cannot be applied to a
declaration". Clang accepts the same attribute before a `case` label
and a `default` label (`tests/declarations/attributes/likely-unlikely.cpp`).

## Evidence

`tests/declarations/attributes/likely-identifier-label.cpp`, with an
override for Clang.

## Options

1. Valid. The attribute applies to every kind of label, and Clang has a
   defect.
2. Valid only on `case` and `default` labels. ATTR-8 names the label
   kinds, and a program with the attribute on an identifier label is
   rejected.

## Answer

Valid (option 1). Two of three implementations accept the attribute,
and ATTR-8 makes no difference between the kinds of label.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. ATTR-P2 says that a likelihood hint changes nothing, and ATTR-P1 says that each standard attribute has a fixed set of places without naming them. Clang 22 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). ATTR-P1 and ATTR-P2 failed step 3, because neither named the kinds of label that a likelihood hint can go on. ATTR-P1 now has a table of the places of the standard attributes, which gives `likely` and `unlikely` a statement or a label of any kind, a name that a `goto` uses among them. The review reads with no doubt, and Clang 22 is the one newest family that differs. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses ATTR-P1.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`), under decision 0056. The review applied ATTR-P1 to the program of `likely-identifier-label.cpp` without the rule text and reached the provisional answer: the hint can go on a label of any kind, the target of a `goto` included, so the program is valid.

Kept open on 2026-11-04 under decisions 0054 and 0055, because ATTR-P1 changed in this commit for two small points of the review on `fallthrough` and `nodiscard`. The question waits for the next use of the changed text.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied ATTR-P1, without the rule text, to the programs of `likely-identifier-label.cpp` and `likely-unlikely.cpp`. Each answer names ATTR-P1 and gives the outcome that the tests expect, so the use supports the answer.

Settled on 2026-11-04 under decision 0054. The use in batch PU1 supports the answer, the gap review of 2026-11-04 supports it with no doubt, and ATTR-P1 gives the same answer by the test of the style guide (section 3). Clang 22 still differs, and the test keeps its override.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The two likelihood attributes apply to labels or statements, and the label grammar allows an attribute before each of the three kinds of label. Clang's rejection is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1.
