# Q-0022: Is `__func__` outside a function rejected?

**Status:** provisional
**Affects:** FN-5, `tests/declarations/functions/definitions/func-outside-function.cpp`

## Context

`__func__` names a string holding the enclosing function's name. At
namespace scope there is no enclosing function. GCC 16 and 14 ("not
defined outside of function scope") and EDG 7.0 reject such a use. Clang
22 and 18 accept the use with a warning ("predefined identifier is only
valid inside function"), even under `-pedantic-errors`.

## Evidence

`tests/RESULTS.md`, row `func-outside-function.cpp`.

## Options

1. Rejected. Clang's acceptance is an extension.
2. Accepted, with an implementation-defined value.

## Provisional answer

Yes (option 1). Two of three implementations reject the use, and Clang's own
warning says `__func__` is only valid inside a function.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. FN-P5 says what `__func__` names inside a function body, and it does not say that a use outside a body is rejected. Clang 22 accepts the use with a warning.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). FN-P5 fails step 3, because it speaks only of a use inside a function body, so option 2 also fits. Item PQ widened FN-P5: each function body declares `__func__` at its start as a local variable, and outside a body nothing declares the name, so a use there is rejected. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses FN-P5.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. `__func__` exists only in a function body, as if a local variable were declared there. Outside a function the name is not declared, so the use is not valid and needs a diagnostic. Clang's warning is a diagnostic, and its acceptance after the warning is an extension.
