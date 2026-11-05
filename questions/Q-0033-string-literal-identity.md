# Q-0033: May constant evaluation compare the addresses of two string literals?

**Status:** provisional
**Affects:** CE-13, PRIM-9, `tests/foundations/constant-evaluation/string-literal-identity.cpp`, `tests/expressions/primary/string-compare-constant.cpp`
**Waits for:** a review (doubt)

## Context

`static_assert("abc" == "abc" || true);` asks constant evaluation to
compare the addresses of two string literals with the same text. Whether
two such literals share storage is not fixed. Clang 22 and 18 reject the
comparison ("comparison of addresses of potentially overlapping literals").
GCC 16 and 14 and EDG 7.0 accept it and give a value.

EDG folds each comparison of literal addresses that the probes tried.
GCC folds the comparison of two equal literals, and rejects the
comparison of `"abc" + 1` with `"bc"`.

## Options

1. Rejected during constant evaluation, because the result depends on
   storage that the implementation chooses. This rule matches the
   treatment of other unspecified pointer comparisons (CMP-8).
2. Accepted, with an implementation-defined result.

## Provisional answer

No; rejected (option 1). A constant expression should not depend on a choice that the
implementation need not make the same way each time.

Kept open on 2026-11-03 under decision 0054, because the review reads with doubt.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1, with doubt. Whether two string literals with one text share storage is unspecified, and an equality comparison with an unspecified result cannot be a constant expression. The doubt is that the equality rules define the result by whether the pointers point to one object, so the text does not plainly put this comparison under that bar. A later defect resolution says that such a comparison is not a constant expression. Clang follows that change, and GCC and EDG do not. The comparison in the test applies `==` to two array operands, which is deprecated.
