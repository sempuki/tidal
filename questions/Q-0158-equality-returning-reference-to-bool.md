# Q-0158: Can an `==` used for a reversed or rewritten comparison return a reference to `bool`?

**Status:** settled
**Affects:** DCMP-12, `tests/classes/comparisons/eq-reference-to-bool-result.cpp`

## Context

A class has an `operator==` that returns `bool&` or `bool&&`. A comparison
uses that operator reversed (`key == acct`) or rewritten
(`acct != 1`).

| Compiler | `bool&`, reversed `==` | `bool&&`, reversed `==` | `bool&`, rewritten `!=` |
|---|---|---|---|
| Clang 18, Clang 22 | rejects | rejects | rejects |
| EDG 7.0 | rejects | rejects | rejects |
| GCC 14, GCC 16 | accepts | accepts | accepts |

Every compiler accepts a result of `const bool` or `const volatile
bool`, and every compiler rejects a result of `int`.

## Evidence

`tests/classes/comparisons/eq-reference-to-bool-result.cpp`,
`eq-cv-bool-result.cpp`, and `reversed-eq-needs-bool.cpp`. Clang reports
that the return type `bool &` of the selected `operator==` is not
`bool`.

## Options

1. The result must have the type `bool`, with or without cv-qualifiers.
   A reference to `bool` is not valid (Clang, EDG).
2. A reference to `bool` is also valid (GCC).

## Answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the cv-qualified
results that all accept are values of type `bool`.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page states the case. DCMP-P2 describes which rewritten and reversed candidates an expression tries, and does not give the result type that a chosen `==` must have. GCC 16 is the one newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). DCMP-P2 failed step 3 of the test, because the model did not give the result type of a chosen `==`. Item PQ widened DCMP-P2. The rewrite takes the result as a `bool` value, so the chosen `==` must return the type `bool`. A cv-qualifier makes no difference, and a reference to `bool` is rejected. DCMP-12 and its tests `eq-cv-bool-result.cpp`, `eq-reference-to-bool-result.cpp`, and `ne-rewrite-reference-to-bool-result.cpp` check each part. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses DCMP-P2.

Used on 2026-11-04 under decision 0056 by the source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`). The review applies DCMP-P2 to the program of `eq-reference-to-bool-result.cpp` without the rule text and reaches the provisional answer: the reversed `==` returns a reference to `bool`, which is not the type `bool`, so the comparison is rejected.

Settled on 2026-11-04 under decision 0054. The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) supports this answer with no doubt, and DCMP-P2 gives the same answer by the test of the style guide (section 3). DCMP-P2 did not change in the commit that recorded the use. GCC 16 still differs, and the test keeps its override for it.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A rewritten or reversed equality candidate that is chosen must return `bool`, possibly cv-qualified. A reference to `bool` is not that type, and GCC's acceptance is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) agrees with option 1. The `==` chosen for a reversed or rewritten comparison must return `bool`, with or without cv-qualifiers, so a reference to `bool` is not valid.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
