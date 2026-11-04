# Q-0244: Does a parenthesized list in a `new` expression form the aggregate deduction candidate?

**Status:** settled
**Affects:** CTAD-9, `tests/templates/class-template-deduction/aggregate-new-parentheses.cpp`

## Context

`template <class T> struct Agg { T v; };` and then `new Agg(1)`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, `Agg<int>*` |
| EDG 7.0 | accepts, `Agg<int>*` |
| GCC 14, 16, host | rejects: class template argument deduction failed |

GCC forms the aggregate candidate for `Agg a(1);`, for `Agg(1)`, and
for `new Agg{1}`.

## Evidence

The test above and the grid `forms.json` (row "new", column
"aggregate").

## Options

1. The candidate is formed for a parenthesized list in every place
   (Clang, EDG).
2. The candidate is formed for a parenthesized list only outside a
   `new` expression (GCC).

## Answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and GCC itself forms the
candidate for the same list in a variable declaration and a functional
cast.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and CTAD-P1 gives the same answer: the same steps form the guides wherever the name stands with an initializer. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CTAD-P1 passes. In the model, the class template has one set of guides, the aggregate candidate among them, and the place of the name only decides whether deduction runs. So the guides for `new Agg(1)` are the guides for `Agg a(1);`, and option 2 does not fit the model. The question stays settled.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. The aggregate candidate is formed when the initializer is a non-empty braced list or a parenthesized expression list, with no exception for a `new` expression, where a placeholder for a deduced class type may appear. GCC's rejection is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
