# Q-0315: Does a constant template argument use the variable or function that it names?

**Status:** settled
**Affects:** ODR-3, ODR-P1, `tests/foundations/odr/template-argument-address-is-use.cpp`, `tests/foundations/odr/template-argument-address-links.cpp`

## Context

ODR-3 says that an evaluated expression that names a variable or a
function uses it. A constant template argument such as `&g` names `g`.
ODR-P1 had no row for a template argument.

| Probe | Clang 22 | GCC 16 |
|---|---|---|
| `Holder<&g>` with `int g();` never defined and never called | links and runs | links and runs |
| the same with `static int g();` and `-Werror` | rejects: `g` has internal linkage but is not defined | accepts |

Clang's diagnostic is the warning that ODR-11 asks for, which Clang
gives only for a used function. GCC gives the same warning for a call
of such a function, and none for the template argument. EDG skips the
second test, because it does not take `-Werror`.

## Evidence

The two tests above. The link test shows nothing about the question,
because ODR-8 allows a program that uses an entity with no definition
to link. The GCC build of the program with the function of internal
linkage links and runs at `-O0` and at `-O2`.

## Options

1. A constant template argument is an evaluated expression, so it uses
   what it names, by the existing rule. Clang's diagnostic matches.
2. A template argument uses nothing, because only its value matters.

## Answer

Yes (ODR-3); Clang reports an undefined internal function, GCC does not (option 1). ODR-3 has no exception for a template argument, and the
argument's value is the address of the entity. ODR-8 and ODR-9 need no
diagnostic, so both compilers conform.

Settled on 2026-11-03 under decision 0054. The review supports this answer with no doubt, and ODR-P1 gives the same answer in its row for a constant template argument. GCC 16 gives no diagnostic for the function with internal linkage, which ODR-9 allows, so no family differs on the use.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of program structure (`reports/review/2026-11-03-gap-review-program-structure.md`) reads the reference as supporting option 1, the provisional answer, with no doubt. A template argument is not an unevaluated operand, so it is potentially evaluated, and an address taken there names the function and uses it.
