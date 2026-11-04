# Q-0354: Does EDG checking a call whose argument names `decltype` of a generic lambda's parameter before the call is instantiated change DEP-1?

**Status:** provisional
**Affects:** DEP-1, DEP-2, `tests/programs/W12/compose.cpp`

## Context

A generic lambda has a parameter `auto&& x`. Its body calls a function
pointer `g` whose type does not depend on a template parameter, with
the argument `static_cast<decltype(x)&&>(x)`. DEP-1 makes `decltype(x)`
a dependent type, because `x` has the type of an invented template
parameter. So the call is checked when the lambda's call operator is
instantiated for an argument type.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | accepts, and the program runs |
| GCC 14, GCC 16 | accepts, and the program runs |
| EDG 7.0 | rejects: an argument of type `auto &&` does not match the parameter type `int` |

EDG gives the same error for the cast `static_cast<decltype(x)>(x)`,
for an alias `using T = decltype(x);`, at namespace scope, and inside a
function template. EDG accepts `g(x)`, and EDG accepts the same call in
a lambda with an explicit template parameter list (`[]<class T>(T&& x)`
with `static_cast<T&&>(x)`). Clang and GCC accept every form.

## Evidence

`tests/programs/W12/compose.cpp`, the W12 Write program, with an
override for EDG. Probes by the clean-room writer on the matrix
containers gave the table above.

## Options

1. `decltype(x)` depends on the invented template parameter (DEP-1),
   so the call is checked only at instantiation. EDG's rejection is a
   defect.
2. A placeholder parameter's `decltype` is not dependent, and the
   program is invalid.

## Provisional answer

No; the program is valid, and EDG differs (option 1). DEP-1 lists
`decltype` of a type-dependent expression as a dependent type, and
DEP-2 makes a name declared with a dependent type type-dependent. EDG
accepts the same program written with an explicit template parameter.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
