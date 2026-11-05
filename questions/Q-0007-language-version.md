# Q-0007: Which language version does Tidal track, and how are newer features handled?

**Status:** provisional
**Affects:** charter, decision 0002, `tests/statements/if/condition-structured-binding.cpp`
**Waits for:** the maintainer (pending.md row 7)

## Context

Tidal's baseline is C++23. Compilers already implement some newer features
and accept them in C++23 mode with a warning (for example, a structured
binding as an `if` condition). Under `-pedantic-errors` the compilers
reject these features.

## Options

1. Track C++23 only. Newer features count as rejected in strict mode, as
   the tests do now.
2. Track the newest version implemented by all baseline compilers, and
   mark rules with the version they apply to.

## Provisional answer

C++23, with newer features rejected under strict mode, for the proof of concept (option 1). Option 2 is needed before Tidal can
follow the language forward.

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`, row 7).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) finds two answers that follow a change to the language made after C++23. In Q-0087, Clang and GCC follow the later change, and EDG follows the C++23 text. In Q-0181, every newest compiler follows the later change. Both questions wait on this one (row 7 of `documentation/decisions/pending.md`).

### Gap review of cluster 3, 2026-11-03

The source-reader gap review of question cluster 3 (`reports/review/2026-11-03-gap-review-question-cluster-3.md`) names the question once. It reads the normalization of folds through their operands as a feature of the next language version, outside the C++23 scope that this question sets. The mention fits the provisional answer.
