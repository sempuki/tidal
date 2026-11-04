# Q-0111: Does a using declaration see a default argument that a later namespace declaration adds?

**Status:** settled
**Affects:** DEFARG-2, `tests/declarations/functions/default-arguments/using-sees-later-default.cpp`

## Context

A namespace declares a function with one parameter. A using declaration brings the function into the global scope. A later declaration in the namespace adds a default argument. An unqualified call with no arguments follows. GCC and EDG accept the call. Clang rejects it.

## Options

1. The default argument is usable through the using declaration.

2. The using declaration sees only the default arguments declared before it.

## Answer

Yes; the default argument is usable through the using declaration (option 1). GCC and EDG agree.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and DEFARG-P2 gives the same answer: default arguments belong to the declarations of a function, and the declarations in one scope add them up. A using declaration adds none of its own. Clang 22 still differs. Tidal records Clang 22's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for Clang 22.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A call through an overload set uses the default arguments of every reachable declaration in the function's own scope. A note says that such a call can find the function through a using declaration from which the declaration with the default argument is not reachable. Clang's rejection is a defect.
