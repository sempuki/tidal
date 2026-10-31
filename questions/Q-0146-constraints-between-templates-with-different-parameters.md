# Q-0146: Do constraints decide between two templates whose function parameters differ?

**Status:** provisional
**Affects:** OVL-28, `tests/foundations/overload-resolution/tie-constraints-different-parameters.cpp`

## Context

One template takes `const T&`, and another takes `T` and has a
constraint. A call with an `int` lvalue gives both specializations the
identity, and neither template is more specialized by its parameters.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | ambiguous |
| GCC 14, GCC 16 | ambiguous |
| EDG 7.0 | the constrained template |

When both templates take `T`, every compiler chooses the constrained
one (`tie-more-specialized-constraints.cpp`, and the templates grid).

## Evidence

`tests/foundations/overload-resolution/tie-constraints-different-parameters.cpp`,
and the columns "template const lref / constrained template" and
"template forwarding / constrained template" of the templates grid.
Gap G-15 of the overload resolution inventory.

## Options

1. Constraints decide only between templates with the same function
   parameters (Clang, GCC).
2. Constraints decide whenever the parameters do not (EDG).

## Provisional answer

Option 1. Two of three implementations agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
