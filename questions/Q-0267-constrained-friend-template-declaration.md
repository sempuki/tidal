# Q-0267: Must a friend template whose constraint uses the enclosing class's template parameter be a definition?

**Status:** provisional
**Affects:** CNS-31, `tests/templates/constraints/friend-template-declaration.cpp`

## Context

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: must be a definition |
| GCC 14, 16, host | rejects: must be a definition |
| EDG 7.0 | accepts |

## Evidence

The test above. Every compiler rejects a friend that is not a template, has a trailing clause, and is not a definition (`friend-declaration.cpp`).

## Options

1. A definition is required (Clang, GCC). Each specialization of the
   class then declares its own template.
2. A declaration is enough (EDG).

## Provisional answer

Yes; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and the same rule applies to a friend that is not a template.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject the friend template declaration, and EDG 7.0 accepts it, so two families disagree. CNS-P5 supports the provisional answer. A friend whose constraint names the class's template parameters is defined in the class, so each specialization of the class defines its own function (CNS-31). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
