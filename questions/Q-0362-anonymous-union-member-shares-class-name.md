# Q-0362: Can a member of an anonymous union share its name with a class?

**Status:** provisional
**Affects:** NAM-39, `tests/foundations/names/shared-name-anonymous-union-member.cpp`
**Waits for:** the maintainer (pending.md row 4)

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

Kept open on 2026-11-04 under decision 0054, because the gap review of the rules that phase 7 added reads the reference as option 2, as the gap review of names does. Clang 22 follows that reading, and GCC 16 and EDG 7.0 follow the answer. Tidal keeps option 1, which NAM-P10 gives, until the maintainer decides (`documentation/decisions/pending.md`, row 4).

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that phase 7 added (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) reads the reference as supporting option 2. The reference lists the pairs of declarations of one name that do not conflict in one scope, and one pair is a type and a non-static data member. A data member of an anonymous union is named as outside that pair. The member is bound in the scope of the enclosing class, so the nested class and the member conflict, and the class is not valid. Clang follows this reading, and GCC and EDG accept. NAM-39 would need the exception to follow the reference.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
