# Q-0277: How does a string literal initialize an array member in the aggregate candidate?

**Status:** provisional
**Affects:** CTAD-9, CTAD-10, `tests/templates/class-template-deduction/aggregate-dependent-element.cpp`, `aggregate-string-fixed-array.cpp`
**Waits for:** the maintainer (pending.md row 3, row 4)

## Context

Two forms give different results.

`template <class T> struct Three { T v[3]; };` and then
`Three w{"ab"};`. The element type of the array depends on `T`, and
the bound does not.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, `Three<const char*>` |
| GCC 14, 16, host | accepts, `Three<const char*>` |
| EDG 7.0 | accepts, `Three<const char*>` |

`template <class T> struct Named { char v[3]; T n; };` and then
`Named b{"ab", 1};`. The array does not depend on `T`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, `Named<int>` |
| GCC 14, 16, host | accepts, `Named<int>` |
| EDG 7.0 | rejects: no constructor matches |

## Evidence

The tests above. Item 26 of the gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` says that braces are not
elided for an array member with a dependent element type when the
element of the list is a string literal. Under that reading,
`Three w{"ab"}` gives `Three<char>`.

## Options

1. The braces of an array with a fixed bound are elided, also for a
   string literal. A string literal initializes a whole array of
   `char` that does not depend on a parameter. Every compiler agrees on
   the first form, and Clang and GCC on the second.
2. A string literal for an array with a dependent element type keeps
   its braces, as the gap review reads.

## Provisional answer

Braces elided for a fixed bound; Clang and GCC agree, EDG differs on a fixed char array (option 1). Every implementation elides the braces in the first form,
and two of three accept the second.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept the string literal for the fixed `char` array and deduce `Named<int>`, and EDG 7.0 finds no guide, so two families disagree on the second form. Every compiler agrees on the first form. CTAD-P2 supports the provisional answer. The aggregate candidate gives an array member that a string literal initializes an lvalue reference to the `const` array type (CTAD-9). For `Named b{"ab", 1}`, the candidate then has the parameters `const char(&)[3]` and `T`, and `T` is `int`. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) agrees with option 1 for the second form, so EDG's rejection of `Named b{"ab", 1}` is a defect. For the first form, the review reads the reference as giving `Three<char>`, because the aggregate candidate does not elide braces for an array with a dependent element type when the element of the list is a string literal. Every compiler gives `Three<const char*>`. No family follows the reading, so the specification follows the shared behavior until the maintainer decides, as for Q-0243 and Q-0275. The question is on the maintainer's list in `documentation/status.md`.

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`).

Kept open on 2026-11-11 (decision 0063, item RV8). The gap review of the phase 15 text names the question and gives no new reading. The question still waits on the maintainer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 2 for the first form and option 1 for the second form. The aggregate candidate does not consider brace elision for an array member whose element type is dependent when the element of the list is a string literal. The string literal then corresponds to the whole array, its parameter is a reference to a `const` array of the member's type, and `T` is deduced as `char`. Every compiler gives an array of pointers, so all three share a defect there. In the second form the array does not depend on the parameter, so brace elision is considered, the string literal initializes the whole `char` array, and the other element gives `int`. Clang and GCC give that result, and EDG's rejection is a defect. An example of the reference deduces `char` for a class with a dependent element type and a dependent bound, which points the same way as the first form.

### Gap review, 2026-11-11 (phase 15 text, expressions, overloading, and templates)

The source-reader gap review of the phase 15 text on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`) names the question in its reading of CTAD-10. The new sentence on a member or base whose type is not dependent agrees: braces are elided as usual, because the reference turns elision off only for the dependent cases. The older sentence on a string literal for an array member of dependent element type stays with this question, whose cluster 5 review reads against it. The review gives no new reading of that case.
