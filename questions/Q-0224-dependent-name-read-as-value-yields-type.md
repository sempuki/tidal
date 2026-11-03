# Q-0224: Is a dependent qualified name without typename that becomes a type at instantiation an error?

**Status:** provisional
**Affects:** DEP-19, `tests/templates/dependent-names/typename-needed-expression.cpp`

## Context

In `sizeof(T::type)`, a functional cast `T::type(1)`, and a function
declared in a block with a parameter `T::type`, the name has no
`typename` and the position is not type-only. The template is
instantiated with a class whose `type` is `int`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: missing `typename` |
| GCC 14, 16, host | rejects: parsed as a non-type, but instantiation yields a type |
| EDG 7.0 | accepts, and uses the type |

## Evidence

`tests/templates/dependent-names/typename-needed-expression.cpp` and
the grid `typename-positions` (rows "sizeof operand", "functional
cast", "function declared in a block"). Inventory item F3 says that the
name is read as an expression and the specialization fails.

## Options

1. The name is a value, and the instantiation fails (Clang, GCC).
2. The implementation can accept the name as a type (EDG).

## Provisional answer

Yes; Clang and GCC agree, EDG differs (option 1). Two of three implementations and the inventory agree. The
parse of a template must not depend on the arguments.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject the instantiation, and EDG 7.0 uses the type, so two families disagree. Probes of the functional cast and of a function declared in a block give the same split. DEP-P5 supports the provisional answer. The parser decides whether `T::name` is a type before it knows the arguments, and outside a type-only position the name is a value, checked at the instantiation (DEP-19). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
