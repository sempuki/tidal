# Q-0348: Does a pointer to member formed from an alias of a ref-qualified function type keep the ref-qualifier?

**Status:** provisional
**Affects:** DCL-15, `tests/declarations/declarators/ref-qualified-function-type-member.cpp`

## Context

DCL-15 lets a type alias name a function type with a cv-qualifier or a
ref-qualifier, and lets such a type be the target of a pointer to
member. With `using G = int() &&;`, the declaration `G S::* pm` declares
a pointer to a member function of `S` with the ref-qualifier `&&`.

Clang 18 and 22 and EDG 7.0 accept `G S::* pm = &S::take;`, where
`S::take` is declared with `G`, and the call `(S{}.*pm)()` returns the
member's value. GCC 14 and 16 reject the initialization ("cannot convert
'int (S::*)() &&' to 'int (S::*)()'"). GCC drops the ref-qualifier from
the type of `pm`, for `&` and for `const &&` as well. GCC keeps a
cv-qualifier alone (`tests/declarations/declarators/qualified-function-type-member.cpp`).

## Evidence

`tests/declarations/declarators/ref-qualified-function-type-member.cpp`,
with an override for GCC.

## Options

1. The pointer to member keeps the ref-qualifier, and GCC has a defect.
2. The ref-qualifier is dropped, and the initialization is not valid.

## Provisional answer

The ref-qualifier is kept (option 1). Two of three implementations
keep it, and GCC keeps a cv-qualifier in the same position.

## Opinions
