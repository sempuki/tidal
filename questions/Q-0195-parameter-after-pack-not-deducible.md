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

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept the declaration, and EDG 7.0 rejects it, so the newest versions give two outcomes. Option 1 is the only option that all three outcomes fit, and the standing rule asks for one outcome. DEP-P6 supports the provisional answer. No call can give the parameter an argument, and a construct that no argument list makes valid needs no diagnostic (DEP-32). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
