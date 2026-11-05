# Q-0123: Does an attribute in a pointer declarator come before or after the cv-qualifiers?

**Status:** provisional
**Affects:** ATTR-18, the `ptr_part` production, `tests/declarations/attributes/pointer-attribute-after-cv.cpp`, `tests/declarations/attributes/pointer-attribute-before-cv.cpp`

## Context

An attribute specifier can follow the `*` of a pointer declarator. With
a cv-qualifier there too, the compilers disagree on the order. GCC 14 and 16 and
EDG 7.0 accept `* const [[]]` and reject `* [[]] const`. Clang 18 and 22
accept `* [[]] const` and reject `* const [[]]`. All three accept `* [[]]`
with no qualifier.

## Evidence

`tests/RESULTS.md`, rows `pointer-attribute-after-cv.cpp` and
`pointer-attribute-before-cv.cpp`.

## Options

1. The attribute comes after the cv-qualifiers. Clang differs.
2. The attribute comes before the cv-qualifiers. GCC and EDG differ.

## Provisional answer

After; Clang differs (option 1). Two of three implementations take this order.

Kept on 2026-11-03. The review reads option 2, which only Clang follows. GCC 16 and EDG 7.0 take the order of option 1, and no principle on the page states an order. Tidal keeps option 1 until the maintainer decides. The question is in row 4 of `documentation/decisions/pending.md`, and it stays open under decision 0054, because the review reads against the answer.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). ATTR-P1 fails step 3, and no principle can state the order. The matrix splits by family: GCC and EDG take one order and Clang takes the other, and the review reads the order of Clang. The order of an attribute and the cv-qualifiers after a `*` is a fact of the grammar, and no model of the page decides between the two. The question stays open and waits on the maintainer (row 4 of `documentation/decisions/pending.md`).

Kept open on 2026-11-04 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`, row 4).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) reads option 2. In a pointer declarator, the optional attributes come right after the `*` and before the optional cv-qualifiers, and the same order holds for a pointer to member. An attribute after the cv-qualifiers fits no position in the grammar there, so GCC's and EDG's acceptance looks like an extension. Clang follows this reading.
