# Q-0300: Does a `new` expression for a type of ordinary alignment fall back to an aligned allocation function?

**Status:** provisional
**Affects:** NEW-29, NEW-30, `tests/expressions/new-delete/aligned-form-only-for-ordinary-type.cpp`

## Context

For a type whose alignment exceeds the default new alignment, a `new`
expression passes the alignment. If no allocation function accepts it,
the expression tries again without the alignment (NEW-29). The checked
source-reader inventory of 2026-10-31 also describes the reverse step:
for a type of ordinary alignment, if no function accepts the size
alone, the expression tries again with the alignment added.

Clang 18 and 22, GCC 14 and 16, and EDG all reject a `new` expression
for a class of ordinary alignment whose only `operator new` takes an
alignment. None of them takes the reverse step.

## Evidence

`tests/expressions/new-delete/aligned-form-only-for-ordinary-type.cpp`.
Clang and GCC say "no matching function for call to 'operator new'".
EDG says the function "cannot be called".

## Options

1. No reverse step (NEW-30). The program is rejected.
2. A reverse step, as the inventory describes. No implementation in
   the matrix supports it in C++23 mode. The inventory may describe a
   later version of the language (Q-0007).

## Provisional answer

No; there is no reverse step, and the program is rejected (NEW-30, option 1). Every implementation in the matrix behaves this way.

Reopened on 2026-11-01. The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) reads the reference as having the reverse step, so the program would be valid. A probe of 2026-11-01 with the implementation's own `<new>` header gives the same rejection on GCC 16 and Clang 22, so the self-declared `std::align_val_t` of the test is not the cause. The specification follows the matrix until the maintainer decides between the matrix and the review's reading.

Kept on 2026-11-03. Every compiler rejects the program, and Tidal keeps the matrix's answer until the maintainer decides. The question is in row 4 of `documentation/decisions/pending.md`, and it stays open under decision 0054, because the review reads against the answer.

Kept open on 2026-11-04 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`, row 4).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) reads option 2, as the cluster 1 review did. When no allocation function matches, the argument list changes and overload resolution runs again. For a type with an extended alignment, the alignment is removed. For any other type, the alignment is added after the size. So the reverse step exists, and the program is valid. No compiler in the table takes the reverse step in C++23 mode.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) finds that the answer contradicts the reference. The C++23 text has the reverse step: a type of ordinary alignment that finds no function gets an alignment argument after the size, and lookup runs again. The class in the test then gets its aligned allocation function, and the program is valid. Clang 22, GCC 16, and EDG 7.0 all reject, so they share a defect or the test has a confound. The review names one confound to rule out by probe: the test declares its own alignment tag type in `std`.
