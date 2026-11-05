# Q-0387: Does constant evaluation reject a relational comparison that involves a base class subobject?

**Status:** settled
**Affects:** CMP-8, CMP-P3, `tests/expressions/comparison/base-subobject-pointers-constant.cpp`, `tests/expressions/comparison/base-and-member-pointers-constant.cpp`
**Source:** reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md, the CMP-P3 section

## Context

CMP-7 orders the elements of one array and the members of one class.
No rule orders two base class subobjects of one object, or a base class
subobject and a member. The gap review of the text phase 8 wrote on
expressions reads such a comparison as having no fixed result, also
inside one complete object, so constant evaluation rejects it.

```cpp
struct Left { int a; };
struct Right { int b; };
struct Both : Left, Right { };
constexpr Both both{};
static_assert(static_cast<const void*>(static_cast<const Left*>(&both))
              < static_cast<const void*>(static_cast<const Right*>(&both)) || true);
```

| Form | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| Two base class subobjects | rejects | accepts | rejects |
| A base class subobject and a member | rejects | accepts | rejects |

Every newest compiler gives a result during constant evaluation for two
members of a union and for a member that occupies no storage, which the
review also reads as having no fixed result. Tidal keeps the compilers'
answer there (`documentation/decisions/pending.md`, row 4).

## Evidence

The two tests in **Affects**. Each expects a rejection and records
GCC's acceptance as an override.

## Options

1. Constant evaluation rejects the comparison (Clang, EDG, and the
   review's reading).
2. Constant evaluation gives the result of the layout (GCC).

## Answer

Yes; constant evaluation rejects the comparison (option 1). Two of the
three newest families reject it, and the review reads it the same way.
CMP-P3 lists the case among the comparisons that constant evaluation
rejects.

Kept open on 2026-11-04 under decision 0054, because CMP-P3 changed in the commit that opened the question, and no reader or reviewer has used the changed principle.

Kept open on 2026-11-04 under decision 0055, because CMP-P3 changed in this commit after finding PU1-8. CMP-P3 now says that a row of an array of arrays is not a complete object, with the row case as an example. The question waits for the next use of CMP-P3.

Used on 2026-11-04 by the principle-use batch PU2 (`reports/clean-room/PU2/`), under decision 0056. A clean-room reader applied CMP-P3, without the rule text, to the programs of `base-and-member-pointers-constant.cpp` and `base-subobject-pointers-constant.cpp`. Each answer names CMP-P3 and gives the outcome that the tests expect (constant evaluation rejects the comparison). The use supports the answer.

Settled on 2026-11-04 under decision 0054. The use in batch PU2 supports the answer, the gap review of 2026-11-04 supports it with no doubt, and CMP-P3 gives the same answer by the test of the style guide (section 3). GCC 16 still differs, and the tests keep their overrides.

Clarified on 2026-11-05 after finding G150 of the front end (`reports/clean-room/rust-frontend/report-21.md`). CMP-8 now says that a pointer to a member of a base class subobject involves that subobject. The new test `tests/expressions/comparison/base-member-and-member-pointers-constant.cpp` compares `&x.b` with `&x.d`. Clang 22 and EDG 7.0 reject the comparison in constant evaluation, and GCC 16 gives a result, as for the other forms of this question. The answer stays.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) reads a relational comparison as having a fixed result only for elements of one array and for different non-static data members of one object, members of zero size and union members excluded. Two base class subobjects, or a base class subobject and a member, have no required order, so constant evaluation rejects the comparison. Clang and EDG follow the review.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
