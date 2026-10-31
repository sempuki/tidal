# Q-0128: Must an implementation diagnose a used function with internal linkage that has no definition?

**Status:** provisional
**Affects:** ODR-9, ODR-11, `tests/foundations/odr/internal-function-not-defined-warns.cpp`, `tests/foundations/odr/unnamed-namespace-function-not-defined-warns.cpp`, `tests/foundations/odr/internal-function-used-in-unused-inline.cpp`

## Context

A function with internal linkage can be defined only in its own
translation unit. If the unit uses the function and does not define the
function, no other unit can supply the definition. The compilers report
the fault in different ways:

| Case | Clang 22 | Clang 18 | GCC 14, GCC 16 | EDG 7.0 |
|---|---|---|---|---|
| `main` calls the function | warning, an error under `-pedantic-errors` | warning | warning, an error under `-pedantic-errors` | error |
| Only an inline function that nothing calls uses the function | as above | warning | no diagnostic | error |

Clang's warning is `-Wundefined-internal`. GCC's warning says that the
function is "used but never defined". GCC gives no diagnostic when the
only use is in a function that GCC does not translate to code.

## Evidence

The three tests above. Clang 18 needs `-Werror` to make the warning an
error.

## Options

1. `no-diagnostic`, with a `recommended` rule that asks for a warning.
   GCC's silence in the second case is then valid.
2. `reject`. Every compiler except GCC in the second case and Clang 18
   reports the fault as an error under strict flags.

## Provisional answer

Option 1. GCC finds the fault only where it emits code, so GCC treats
the fault as a link-time fault that it reports early. Each compiler
warns about the plain case.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
