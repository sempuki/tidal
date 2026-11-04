# Q-0111: Does a using declaration see a default argument that a later namespace declaration adds?

**Status:** provisional
**Affects:** DEFARG-2, `tests/declarations/functions/default-arguments/using-sees-later-default.cpp`

## Context

A namespace declares a function with one parameter. A using declaration brings the function into the global scope. A later declaration in the namespace adds a default argument. An unqualified call with no arguments follows. GCC and EDG accept the call. Clang rejects it.

## Options

1. The default argument is usable through the using declaration.

2. The using declaration sees only the default arguments declared before it.

## Provisional answer

Yes; the default argument is usable through the using declaration (option 1). GCC and EDG agree.

Reopened as provisional on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt. DEFARG-P2 fails step 3 of the test. The model says that the declarations in one scope add up their default arguments. The model says nothing of a using declaration in another scope, so option 2, where the using declaration sees only the defaults before it, also fits the model. No other principle on the page gives the answer. Clang 22 differs, and the tests keep their overrides for Clang 22.

On 2026-11-04, item PQ widened DEFARG-P2: a using declaration brings the function into another scope with no defaults of its own, and a call through it uses the defaults that the function's scope has at the call, also those added after the using declaration. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses DEFARG-P2.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A call through an overload set uses the default arguments of every reachable declaration in the function's own scope. A note says that such a call can find the function through a using declaration from which the declaration with the default argument is not reachable. Clang's rejection is a defect.
