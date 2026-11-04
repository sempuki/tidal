# Q-0362: Can a member of an anonymous union share its name with a class?

**Status:** provisional
**Affects:** NAM-39, `tests/foundations/names/shared-name-anonymous-union-member.cpp`

## Context

NAM-39 lets a class share its name with a data member of the same
class. A member of an anonymous union is a member of the enclosing
class for lookup.

```cpp
struct C { union { int S; }; struct S {}; };
```

| Probe | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| the class above | rejects | accepts | accepts |

The source-reader gap review of names and lookup
(`reports/review/2026-11-03-gap-review-names.md`) reads the reference
as leaving a member of an anonymous union out of the exemption, so the
class conflicts with the member.

## Evidence

The test in **Affects** expects acceptance and records Clang's result
with an override.

## Options

1. The member of the anonymous union can share the name, like any
   other data member (GCC, EDG).
2. The member cannot share the name (Clang, and the review's reading).

## Provisional answer

The member can share the name (option 1). NAM-P10 lets a class share its name with a data
member, and the member of an anonymous union is used as a data member
of the class. Two of the three newest families accept the class. The
reading of the reference differs, and the Evidence of the names page
says so.

Kept open on 2026-11-03 under decision 0054, because no review has read the provisional answer. The gap review of names, which raised the question, reads the reference as option 2, and Clang 22 follows that reading.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
