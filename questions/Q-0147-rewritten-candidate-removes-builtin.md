# Q-0147: Does a rewritten candidate remove the built-in candidate with the same parameter list?

**Status:** provisional
**Affects:** OVL-10, `tests/overloading/overload-resolution/builtin-removed-by-rewritten.cpp`

## Context

An unscoped enumeration `Color` has a non-member `operator<=>(Color,
Color)`. The expression `red < blue` has the built-in candidate for `<`
with the parameters `(Color, Color)`, and the rewritten candidate
`(red <=> blue) < 0` with the same parameters.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | the rewritten candidate |
| EDG 7.0 | the rewritten candidate |
| GCC 14, GCC 16 | the built-in candidate |

If both candidates stay, the built-in one wins, because a candidate
that is not rewritten beats a rewritten one (OVL-32). A non-member
`operator==(Color, Color)` that is not rewritten removes the built-in
`==` on every compiler (`builtin-same-shape-removed.cpp`).

## Evidence

`tests/overloading/overload-resolution/builtin-removed-by-rewritten.cpp`.
Item BI-5 of the overload resolution inventory names rewritten
candidates among those that remove a built-in candidate.

## Options

1. The rewritten candidate removes the built-in candidate (Clang, EDG).
2. Only a candidate that is not rewritten removes it (GCC).

## Provisional answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the inventory
describes the removal this way.

The question stays open under decision 0051 (2026-11-03). GCC 16 keeps the built-in candidate, and Clang 22 and EDG 7.0 remove it, so two families disagree. No principle on Tidal's pages separates the options. The provisional answer rests on the two families and on item BI-5 of the inventory.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page separates the options.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). OVL-P4 failed step 3 of the test, because the model did not say which declared functions remove a built-in candidate. Item PQ widened OVL-P4. A non-member function with the parameter types of a built-in candidate replaces that candidate, also when the function takes part only as a rewritten candidate. OVL-10 and `builtin-removed-by-rewritten.cpp` check the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses OVL-P4. GCC 16 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) agrees with option 1. A built-in candidate drops out when a non-member candidate, or a rewritten non-member candidate, has the same parameter types and is not a template specialization. The three-way comparison function on the enumeration is a rewritten candidate of that kind.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
