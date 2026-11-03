# Q-0201: Does a template template argument match when its constant parameter has another fixed type?

**Status:** provisional
**Affects:** TARG-17, TARG-18

## Context

`template <long V> struct Long` is the argument for
`template <template <int> class C> struct Use`.

| Compiler | Result |
|---|---|
| Clang 22, host | accepts |
| Clang 18 | rejects: different template parameters |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: not compatible with the template template parameter |

All compilers accept a placeholder parameter on either side: a
`template <auto>` argument for a `template <int>` head, and a
`template <int>` argument for a `template <auto>` head.

## Evidence

`tests/templates/arguments/template-template-constant-type.cpp`, with
overrides for EDG and Clang 18, and `template-template-auto.cpp`.

## Options

1. The argument matches when each argument list that the head accepts
   is valid for the argument template (Clang 22, GCC). This is the model
   of TARG-17.
2. The two constant parameters must have the same type (Clang 18, EDG).

## Provisional answer

Yes; Clang 22 and GCC accept, Clang 18 and EDG reject (option 1). The newest Clang and GCC agree, and the answer follows the
model that TARG-17 states for the other cases (Q-0169).

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept `Long` for a `template <int>` head, and EDG 7.0 rejects it, so two families disagree. Clang 18 also rejects it. TARG-P2 supports the provisional answer through TARG-17. The argument is valid when each argument list that the head accepts is valid for the argument template, and every `int` argument converts to `long`. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
