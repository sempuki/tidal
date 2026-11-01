# Q-0265: Can a template template argument be more constrained than its parameter?

**Status:** provisional
**Affects:** CNS-25, `tests/templates/constraints/template-template-more-constrained.cpp`

## Context

The parameter is `template <A> class`, and the argument is a class
template whose parameter has the type constraint `AB`, which subsumes
`A`.

| Compiler | Result |
|---|---|
| Clang 18 | accepts |
| Clang 22, host | rejects: template template argument is more constrained than template template parameter |
| GCC 14, 16, host | rejects: constraint mismatch |
| EDG 7.0 | accepts |

## Evidence

The test above. Every compiler accepts an argument that is less constrained than its parameter (`template-template-constrained.cpp`).

## Options

1. Not valid (Clang 22, GCC). The template could use the parameter
   with an argument that the argument template does not accept.
2. Valid (Clang 18, EDG).

## Provisional answer

Option 1. The current Clang and GCC agree, and the rule keeps a use of the parameter inside the template valid.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
