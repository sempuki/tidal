# Q-0075: Can a name with C language linkage match a global variable?

**Status:** settled
**Affects:** LSP-4, `tests/declarations/linkage-specifications/c-name-conflicts-global.cpp`

## Context

A namespace declares `extern "C" int clash;` and the global namespace declares a variable `clash` with C++ linkage. The two names refer to the same symbol name. Clang and EDG reject the program. GCC accepts it.

## Options

1. The program is not valid.
2. The two declarations are separate entities.

## Answer

No; the program is not valid (option 1). Clang and EDG agree, and the two names would collide when linked.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and LSP-P2 gives the same answer: a C name cannot share its spelling with a global variable, because the linker would see one symbol. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A declaration with C language linkage and a global variable of the same name declare one entity. The two declarations then give that entity different language linkages, which makes the program not valid. A diagnostic is required when one declaration is reachable from the other.
