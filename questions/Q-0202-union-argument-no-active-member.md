# Q-0202: Can a union with no active member be a template argument?

**Status:** provisional
**Affects:** TARG-19, TARG-16

## Context

A union has a `constexpr` default constructor that activates no member.
The program uses `Pick<Choice{}>` twice and compares the two types.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts; the types are the same |
| GCC 14, 16, host | rejects: `Choice()` is not a constant expression |
| EDG 7.0 | accepts; the types are the same |

All compilers accept unions with an active member as arguments and
compare them by the active member and its value.

## Evidence

`tests/templates/arguments/same-specialization-union-empty.cpp`, with
an override for GCC, and `same-specialization-kinds.cpp`.

## Options

1. A union with no active member is a valid argument, and two such
   values of one union type are the same (Clang, EDG).
2. A union with no active member is not a constant (GCC).

## Provisional answer

Option 1. Clang and EDG agree, and the comparison table of TARG-P4 has
a row for the case.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
