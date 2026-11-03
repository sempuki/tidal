# Q-0266: Do the constraints of special member functions decide which ones make a class trivial?

**Status:** provisional
**Affects:** CNS-21, `tests/templates/constraints/eligible-default.cpp`, `tests/templates/constraints/eligible-copy.cpp`, `tests/templates/constraints/eligible-more-constrained.cpp`, `tests/templates/constraints/eligible-unsatisfied-more-constrained.cpp`

## Context

A class template declares a defaulted constructor and a
user-provided constructor of the same kind, with different
constraints. For the arguments used, only the defaulted one is
satisfied, or the defaulted one is more constrained. A union with a
member of that class needs the member's constructor to be trivial.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts: the constructor is trivial |
| GCC 14, 16, host | accepts: the constructor is trivial |
| EDG 7.0 | rejects: the union's constructor is deleted |

## Evidence

The four tests above. In `eligible-unsatisfied-more-constrained.cpp` the user-provided constructor is more constrained but not satisfied, and EDG again treats the default constructor as not trivial. With destructors, every compiler chooses the trivial one (`destructor-trivial.cpp`).

## Options

1. Only eligible special members count (Clang, GCC). The others can
   never be called for those arguments.
2. Every declared special member counts (EDG).

## Provisional answer

Yes; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and EDG already selects destructors by their constraints.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 treat the defaulted constructor as the trivial one, and EDG 7.0 deletes the union's constructor, so two families disagree. CNS-P3 supports the provisional answer. A special member function whose constraints fail is not eligible (CNS-21), and EDG 7.0 already chooses destructors by their constraints (CNS-19). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
