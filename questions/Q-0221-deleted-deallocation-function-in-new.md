# Q-0221: Is a `new` expression valid when its matching deallocation function is deleted?

**Status:** provisional
**Affects:** NEW-25, `tests/expressions/new-delete/deallocation-deleted.cpp`, `tests/expressions/new-delete/deallocation-deleted-no-throw.cpp`, `tests/expressions/new-delete/deallocation-placement-deleted.cpp`

## Context

A class declares its own `operator new` and a deleted
`operator delete` that matches it. A `new` expression creates an object
of the class.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: the deleted function is used |
| EDG 7.0 | rejects: the deleted function cannot be referenced |
| GCC 14, 16, host | accepts |

The results are the same for a usual form and for a placement form.
The results are also the same when the initialization cannot throw.
All three compilers reject a private deallocation function in the same
cases.

## Evidence

The three tests above, each with an override for GCC.

## Options

1. The program is not valid (Clang, EDG). The deallocation function is
   checked as the access check already does.
2. The program is valid (GCC).

## Provisional answer

No; Clang and EDG reject, GCC accepts (option 1). Clang and EDG agree, and the access check, on which all
three agree, treats the deallocation function as used.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
