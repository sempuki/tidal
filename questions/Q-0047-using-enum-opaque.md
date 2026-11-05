# Q-0047: May `using enum` name an enumeration that is declared but not defined?

**Status:** settled
**Affects:** ENUM-9, `tests/declarations/enumerations/using-enum-opaque.cpp`

## Context

`enum class E : int; using enum E;` names an enumeration whose enumerators are not yet known. Clang and GCC reject the `using enum`. EDG 7.0 accepts it.

## Options

1. Rejected: the enumeration must be complete.
2. Valid.

## Answer

No; the enumeration must be complete, so the `using enum` is rejected (option 1). Clang and GCC, in both versions, reject the `using enum`.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. ENUM-P2 says that `using enum` brings the enumerators into a scope, and it does not say that the enumeration must be complete. EDG 7.0 accepts the declaration.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). ENUM-P2 failed step 3, and ENUM-P3 pointed the other way, because ENUM-P3 makes an enumeration with a fixed type complete at its opaque declaration. ENUM-P2 now says that `using enum` needs the enumerators, so the definition with the list must come first, and that an opaque declaration names no enumerator although the declaration makes the enumeration complete. The review reads with no doubt, and EDG 7.0 is the one newest family that differs. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses ENUM-P2.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`), under decision 0056. The review applied ENUM-P2 to the program of `using-enum-opaque.cpp` without the rule text and reached the provisional answer: the opaque declaration names no enumerator, so the declaration that brings in the enumerators is rejected.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and ENUM-P2 gives the same answer by the test of the style guide (section 3), as the review's use shows. EDG 7.0 still differs, and the tests keep its overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The enumeration that `using enum` names must have a reachable definition with its list of enumerators. An opaque declaration gives no such list, so EDG's acceptance is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1. The reference asks that the enumeration a `using enum` declaration names have a reachable definition with its list of enumerators, and an opaque declaration does not count.
