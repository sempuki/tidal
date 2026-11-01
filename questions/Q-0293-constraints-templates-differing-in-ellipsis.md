# Q-0293: Do constraints decide between two templates that differ only in an ellipsis?

**Status:** provisional
**Affects:** ORD-13, `tests/templates/ordering/constraints-ellipsis.cpp`

## Context

Two function templates have the head `template <class T>`. One takes
`T` and has a constraint. The other takes `T` and an ellipsis. A call
passes one `int`. Without the constraint, every compiler finds the call
ambiguous, so ordering by types ties.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | the constrained template |
| GCC 14, GCC 16 | ambiguous |
| EDG 7.0 | the constrained template |

For two plain functions `f(int)` and `f(int, ...)`, Clang 22 and GCC
find the call ambiguous, and Clang 18 and EDG choose the constrained
function (Q-0146). Clang 22 is the only compiler that counts the
ellipsis for plain functions and not for templates.

## Evidence

`tests/templates/ordering/constraints-ellipsis.cpp`, with an override
for GCC. A probe with the constraint removed is ambiguous on every
compiler.

## Options

1. For templates, the ellipsis takes no part in the conditions of
   ORD-13, and the constraint decides (Clang, EDG).
2. The ellipsis counts for templates as for plain functions, and the
   call is ambiguous (GCC). ORD-13 then states one condition for both.

## Provisional answer

Yes, the ellipsis does not count for templates (ORD-13); Clang and EDG agree, GCC differs (option 1). Two of three implementations agree. ORD-13 states the
ellipsis condition for plain functions only. Option 2 gives a simpler
model, and a contributor can argue for it with the plain-function
results of Q-0146.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
