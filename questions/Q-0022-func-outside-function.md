# Q-0022: Is `__func__` outside a function rejected?

**Status:** provisional
**Affects:** FN-5, `tests/declarations/functions/definitions/func-outside-function.cpp`

## Context

`__func__` names a string holding the enclosing function's name. At
namespace scope there is no enclosing function. GCC 16 and 14 ("not defined
outside of function scope") and EDG 7.0 reject it. Clang 22 and 18 accept it
with a warning ("predefined identifier is only valid inside function"), even
under `-pedantic-errors`.

## Evidence

`tests/RESULTS.md`, row `func-outside-function.cpp`.

## Options

1. Rejected. Clang's acceptance is an extension.
2. Accepted, with an implementation-defined value.

## Provisional answer

Option 1. Two of three implementations reject it, and Clang's own warning
says it is only valid inside a function.

## Opinions
