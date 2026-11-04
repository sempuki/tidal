# Q-0190: Do ref-qualifiers take part in the match of `!=` and `==`?

**Status:** provisional
**Affects:** DCMP-13, `tests/classes/comparisons/ne-matching-ref-qualifier.cpp`

## Context

A class has `bool operator==(int) const&` and
`bool operator!=(int) const&&`. If the `!=` matches the `==`, the
reversed `==` is not a candidate, and `1 == m` has no viable function.

| Compiler | `const&` and `const&&` | no qualifier and `const&` | `const` and `const volatile` |
|---|---|---|---|
| Clang 18, Clang 22 | match | match | match |
| GCC 14, GCC 16 | no match | match | match |
| EDG 7.0 | match | match | match |

## Evidence

`tests/classes/comparisons/ne-matching-ref-qualifier.cpp` and
`ne-matching-other-result.cpp`.

## Options

1. The cv-qualifiers and ref-qualifiers of members take no part (Clang,
   EDG).
2. Two ref-qualifiers of different kinds make the functions differ
   (GCC).

## Provisional answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the rule then ignores
every qualifier of the object parameter.

Kept on 2026-11-03. For cv-qualifiers, every compiler disagrees with the review. For ref-qualifiers, only GCC 16 follows the review, and no principle on the page states when a `!=` corresponds to an `==`. Tidal keeps option 1 until the maintainer decides. The question is in row 4 of `documentation/decisions/pending.md`, and it stays open under decision 0054, because the review reads against the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) reads option 2 and goes further. An equality operator is not used reversed when a search finds an inequality operator that corresponds to it, and for two members correspondence compares the object parameters. Two ref-qualifiers of different kinds make the object parameters differ, which is GCC's result. Two different cv-qualifiers, such as `const` and `const volatile`, or `const` and none, make them differ too, and no compiler follows that part. When exactly one member has no ref-qualifier, the object types are compared after the references are removed. The doubt is that every compiler ignores cv-qualifiers here.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
