# Q-0387: Does constant evaluation reject a relational comparison that involves a base class subobject?

**Status:** provisional
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

## Provisional answer

Yes; constant evaluation rejects the comparison (option 1). Two of the
three newest families reject it, and the review reads it the same way.
CMP-P3 lists the case among the comparisons that constant evaluation
rejects.

Kept open on 2026-11-04 under decision 0054, because CMP-P3 changed in the commit that opened the question, and no reader or reviewer has used the changed principle.

Kept open on 2026-11-04 under decision 0055, because CMP-P3 changed in this commit after finding PU1-8. CMP-P3 now says that a row of an array of arrays is not a complete object, with the row case as an example. The question waits for the next use of CMP-P3.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) reads a relational comparison as having a fixed result only for elements of one array and for different non-static data members of one object, members of zero size and union members excluded. Two base class subobjects, or a base class subobject and a member, have no required order, so constant evaluation rejects the comparison. Clang and EDG follow the review.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
