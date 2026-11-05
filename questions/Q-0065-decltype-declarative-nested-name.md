# Q-0065: Can decltype name the class in a member definition outside the class?

**Status:** settled
**Affects:** DTY-3, `tests/expressions/decltype/declarative-nested-name.cpp`

## Context

A member function can be defined outside its class with a qualified
name. A program can write the class part of that name with `decltype`,
as in `void decltype(s)::f() {}`. Clang and EDG reject the definition.
GCC accepts it.

GCC also accepts `decltype` in the qualified name of a nested class
definition and of an enumeration definition, and EDG accepts the
enumeration definition. Both DTY-5 tests, `declarative-nested-name.cpp`
and `declarative-nested-name-static-member.cpp`, have an override for
GCC.

## Options

1. `decltype` must not occur in the qualified name of a declaration.
2. `decltype` is valid there.

## Answer

No; `decltype` must not occur in the qualified name of a declaration (option 1). Clang and EDG agree. `decltype` stays valid in a qualified
name in an expression.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and DTY-P2 gives the same answer: the result of `decltype` cannot stand in the qualified name of a declaration, where the name must be written out. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The `decltype` form must not appear in the qualifier of a name that a declaration declares. The ban does not reach qualifiers in expressions.
