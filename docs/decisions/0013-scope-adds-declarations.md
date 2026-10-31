# 0013. Scope adds declarations

Date: 2026-10-30
Status: Accepted. Extends 0002.

## Context

Expressions and statements constantly refer to declarations: condition
declarations in `if`, `auto` in initializers, storage duration in
lifetimes. Without them, many in-scope rules point at concepts marked
*(referenced)*. Declarations are large, so they need an order of work.

## Decision

Declarations are in scope, as listed in `docs/charter.md`. Sections are
written in this order, each through the full pipeline:

1. Placeholder types in variable declarations (`auto`, `decltype(auto)`)
2. Storage classes, storage duration, and linkage
3. Declarators and cv-qualifiers (pointers, references, arrays, functions)
4. Initialization (default, value, copy, direct, list, aggregate)
5. Structured bindings
6. Enumerations
7. Namespaces, `using` declarations, and `using` directives
8. Standard attributes
9. Function definitions (default arguments, `= default`, `= delete`,
   return type deduction)
10. The `constexpr`, `consteval`, and `constinit` specifiers

The first section comes first because existing rules (IF-5, IF-7) already
depend on `auto`, and its behavior is observable with compile-time checks
alone, which every compiler in the matrix supports.

## Consequences

- Classes, templates, overload resolution, and name lookup stay
  referenced. Declaration rules that need them say so.
- `std::initializer_list` comes from the library, which is out of scope.
  Rules that involve `std::initializer_list` reference the library, and
  their tests declare the minimum they need, or the rules are deferred.
- Glossary entries currently marked *(referenced)* for declaration topics
  are replaced as those sections are written.
