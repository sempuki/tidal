# Q-0260: Can a lambda or a member of a local class inside a function template have a trailing requires clause?

**Status:** provisional
**Affects:** CPT-11, CNS-18, `tests/templates/concepts/trailing-clause-lambda.cpp`, `tests/templates/concepts/trailing-clause-local-class.cpp`, `tests/templates/constraints/lambda-conversion.cpp`

## Context

The lambda has no template parameter list of its own. The local class
is not a template. Both are declared in the body of a function
template, and the clause names the function template's parameter.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: a requires-clause is not allowed here (not a templated function) |

## Evidence

The three tests above. A member of a class nested in a class template is accepted by every compiler (`trailing-clause-member.cpp`).

## Options

1. Valid (Clang, GCC). The lambda and the local class are part of the
   function template, so the clause has template arguments to check.
2. Not valid (EDG).

## Provisional answer

Option 1. Two of three implementations agree, and the case is the same as a member of a class nested in a class template.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
