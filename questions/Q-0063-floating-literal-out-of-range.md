# Q-0063: Is a floating literal outside the range of its type valid?

**Status:** provisional
**Affects:** PRIM-15, `tests/expressions/primary/float-overflow.cpp`

## Context

The literal `1e999f` is larger than the largest `float`. Clang and GCC
accept it with a warning, also under `-pedantic-errors`, and give
infinity. EDG 7.0 rejects it.

## Options

1. The value is implementation-defined. An implementation can reject the
   literal.
2. The program is not valid, and implementations must reject it.

## Provisional answer

The value is implementation-defined, and an implementation can reject the literal (option 1). Two of the three implementations accept the literal in their
strictest mode.

Kept on 2026-11-03. The review reads against the answer with doubt, and PRIM-P4 lists a floating value outside its type's range among the parts of a literal that belong to the implementation. Clang 22 and GCC 16 accept, and EDG 7.0 rejects, so PRIM-15 allows both. The question stays open under decision 0054, because the review reads with doubt.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) reads option 2, with doubt. A floating literal whose value is outside the range of its type makes the program not valid, and nothing permits acceptance with a warning, so Clang and GCC miss a required diagnostic. The doubt is the meaning of the range for a type with infinities. If that range covers every real number, every finite literal is in range and rounds to infinity, which is the result of Clang and GCC.

- The second gap review
  (`reports/review/2026-10-31-gap-review-expr-decl.md`, a summary by a
  source reader) holds that such a literal is not valid (option 2).
  Clang and GCC still accept the literal in their strictest mode, so the
  provisional answer stays option 1 until more opinions arrive.
