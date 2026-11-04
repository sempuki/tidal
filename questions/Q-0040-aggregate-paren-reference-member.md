# Q-0040: Does parenthesized aggregate initialization extend the life of a temporary bound to a reference member?

**Status:** provisional
**Affects:** REF-4, REF-5, `tests/declarations/initialization/references/aggregate-member-extends.cpp`, `aggregate-member-paren-not-extended.cpp`

## Context

With braces, `Holder h{Note{5}};` keeps the temporary alive as long as `h` on every compiler. With parentheses, `Holder h(1);` followed by a read of the member during constant evaluation is rejected by Clang and GCC as a read after the temporary's lifetime, and accepted by EDG 7.0.

## Options

1. Braces extend the lifetime; parentheses do not. EDG's acceptance is a possible defect.
2. Both forms extend the lifetime.

## Provisional answer

Only braces extend the lifetime; parentheses do not (option 1). Clang and GCC agree, and the review summary describes the same difference.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. REF-P2 lists the bindings that extend a temporary and does not name parenthesized aggregate initialization.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. A temporary bound to a reference member of an aggregate that a parenthesized list initializes lasts only to the end of the full expression. With braces the usual extension applies, and EDG's acceptance is a defect.

