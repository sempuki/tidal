# Q-0269: Does a function with a placeholder parameter redeclare the function template with the same invented head?

**Status:** provisional
**Affects:** CNS-27, `tests/templates/constraints/redeclare-placeholder.cpp`, the spellings grid

## Context

`template <A T> int f(T);` and then `int f(A auto) { return 1; }`.
The same holds without the constraint.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts: the second declaration defines the first |
| GCC 14, 16, host | rejects: the call is ambiguous |
| EDG 7.0 | rejects: the call is ambiguous |

## Evidence

The test above, and the columns `A T / A auto` and `AB T / AB auto` of the spellings grid, where Clang reports a redefinition.

## Options

1. One template (Clang). DED-34 makes the placeholder an
   invented type parameter, so the two heads are equivalent by
   TPL-20.
2. Two templates (GCC, EDG).

## Provisional answer

Yes, following DED-34 and TPL-20; Clang agrees, GCC and EDG differ (option 1). The rules for invented parameters already put them in the head (ORD-13), and equivalent heads declare one template.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
