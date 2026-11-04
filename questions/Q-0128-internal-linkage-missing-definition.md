# Q-0128: Must an implementation diagnose a used function with internal linkage that has no definition?

**Status:** settled
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

The table gives each compiler's diagnostic under `-pedantic-errors`
and no other flag, for three uses of a function with internal linkage
that has no definition. An error from Clang 22 and GCC is a warning
that `-pedantic-errors` makes an error. A warning from Clang 18 stays a
warning, so the program builds. The tests that add `-Werror` reject on
Clang 18 too, and EDG skips them.

| The use | Test | Clang 18 | Clang 22 | GCC | EDG |
|---|---|---|---|---|---|
| a call in a function that is not `inline` | `internal-function-not-defined-warns.cpp` (`-Werror`) | warning | error | error | error |
| a call only in an `inline` function that nothing calls | `internal-function-used-in-unused-inline.cpp` | warning | error | none | error |
| the address as a template argument | `template-argument-address-is-use.cpp` (`-Werror`) | warning | error | none | error |

With `-Wall` or `-Wunused-function`, GCC 16 gives a `-Wunused-function`
warning in place of that diagnostic, and `-pedantic-errors` leaves the
new warning a warning.

## Options

1. `no-diagnostic`, with a `recommended` rule that asks for a warning.
   GCC's silence in the second case is then valid.
2. `reject`. Every compiler except GCC in the second case and Clang 18
   reports the fault as an error under strict flags.

## Answer

No; the rule needs no diagnostic and a warning is recommended; GCC is silent when the only use is in an uncalled inline function (option 1). GCC finds the fault only where it emits code, so GCC treats
the fault as a link-time fault that it reports early. Each compiler
warns about the plain case.

Settled on 2026-11-03 under decision 0054. The review supports this answer with no doubt, and ODR-P2 gives the same answer: for a function with internal linkage, the compiler finds the fault, usually as a warning. GCC 16 gives no diagnostic in one form, which the rule allows, and the tests keep their overrides.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of program structure (`reports/review/2026-11-03-gap-review-program-structure.md`) reads the reference as supporting option 1, the provisional answer, with no doubt. The reference needs no diagnostic. A call inside an `inline` function that nothing calls is still a use, so GCC's silence is allowed only because no diagnostic is required.
