# Q-0105: Does a block-scope extern declaration redeclare a hidden internal-linkage variable?

**Status:** settled
**Affects:** LNK-9, `tests/declarations/storage-linkage/extern-in-block-hidden.cpp`

## Context

A namespace-scope `static` variable is hidden by a local variable of the same name. An inner block declares the name `extern`. Clang and GCC accept the declaration and read the namespace-scope variable. EDG 7.0 rejects the declaration.

## Options

1. The declaration redeclares the namespace-scope entity and takes its internal linkage.

2. The declaration is not valid when the entity is hidden.

## Answer

Yes; the declaration redeclares the namespace-scope entity and takes its internal linkage (option 1). Clang and GCC agree.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and STO-P3 gives the same answer: a block-scope variable with `extern` names the namespace-scope entity of that name, and the first declaration settles the linkage. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A block-scope declaration with `extern` redeclares the earlier entity even when a local declaration hides that entity, and the redeclaration takes internal linkage. EDG follows an older rule, under which lookup did not find the hidden entity and the two linkages conflicted.
