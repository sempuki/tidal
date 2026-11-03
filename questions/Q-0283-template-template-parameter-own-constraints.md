# Q-0283: Does an implementation diagnose a use of a template template parameter that fails the parameter's own constraints?

**Status:** provisional
**Affects:** CNS-38, `tests/templates/constraints/template-template-parameter-constraint-use.cpp`

## Context

A class template has the parameter `template <Wide> class Tmpl`, and
its body declares a member of type `Tmpl<char>`. `char` fails `Wide`.
The arguments of `Tmpl<char>` do not depend on a template parameter.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts the definition |
| GCC 14, 16, host | accepts the definition |
| EDG 7.0 | rejects: template constraint not satisfied |

## Evidence

The test above. Clang and GCC also accept the template when it is
instantiated with an argument template that has no constraints. EDG
rejects the definition in both cases. When the argument template has
its own constraint that `char` fails, every compiler rejects the
instantiation (`template-template-argument-constraint-use.cpp`). The
gap review of concepts and constraints (item 5) describes the rule as
one that needs no diagnostic.

## Options

1. The program is not valid, and no diagnostic is required (Clang and
   GCC accept, EDG diagnoses).
2. The implementation must diagnose the use (EDG).
3. The use is valid, and only the argument template's constraints
   apply (Clang, GCC).

## Provisional answer

No diagnostic required; EDG diagnoses (option 1). The parameter's constraints describe every argument the
template can receive, so a use that fails them can never be valid. The
compilers differ only in whether they diagnose the use.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept the definition, and EDG 7.0 rejects it, so the newest versions give two outcomes. A probe with an argument template that has no constraint gives the same split. Option 1 is the only option that all three outcomes fit, and the standing rule asks for one outcome. CNS-P3 supports the provisional answer. Its table makes a use of a template template parameter with independent arguments that fail the parameter's constraints not valid, with no diagnostic required (CNS-38). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
