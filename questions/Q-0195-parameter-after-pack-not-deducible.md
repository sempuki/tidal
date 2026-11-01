# Q-0195: Must an implementation diagnose a function template parameter after a pack that deduction cannot find?

**Status:** provisional
**Affects:** TPL-35, TPL-30

## Context

A function template declares a type pack, then a type parameter with no
default. No function parameter uses the second parameter, so no call
can give it an argument.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts the declaration |
| GCC 14, 16, host | accepts the declaration |
| EDG 7.0 | rejects: the parameter follows a pack and cannot be deduced |

The same split holds for two type packs in a row, and for a constant
pack whose type is the type pack before it.

## Evidence

`tests/templates/declarations/pack-then-not-deducible.cpp`, with an
override for EDG. `tests/templates/declarations/pack-then-deducible.cpp`
shows the valid forms, which all compilers accept.

## Options

1. The declaration is not valid, and an implementation need not
   diagnose it. All three compilers conform.
2. The declaration is not valid, and an implementation must reject it
   (EDG).
3. The declaration is valid (Clang, GCC).

## Provisional answer

No diagnostic required; Clang and GCC accept, EDG rejects (option 1). No program can use such a template, and two of three
compilers accept the declaration.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
