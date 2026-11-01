# Q-0169: Can a template template argument have more parameters than the parameter, if the extra ones have defaults?

**Status:** settled
**Affects:** TARG-17

## Context

`template <template <class> class C> struct Apply` takes
`template <class T, class U = char> struct Pair`. Inside `Apply`,
`C<int>` is `Pair<int, char>`.

| Compiler | Result |
|---|---|
| Clang 22, host | accepts |
| Clang 18 | rejects: the parameter lists differ in length |
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |

## Evidence

`tests/templates/arguments/template-template-defaults.cpp`, with an
override for Clang 18. The other template template tests pass on every
compiler.

## Options

1. The argument is valid when every argument list that the parameter
   accepts is valid for the argument (Clang 22, GCC, EDG).
2. The parameter lists must match exactly (Clang 18).

## Answer

Yes; Clang 22, GCC, and EDG agree, Clang 18 rejects (option 1). Every newer compiler accepts the argument, and option 1 also
explains why a parameter with a pack accepts a template with fixed
parameters.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. Clang 18 still rejects the argument, and the test keeps its override for Clang 18.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
