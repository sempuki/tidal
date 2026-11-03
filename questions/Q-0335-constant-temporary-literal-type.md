# Q-0335: Must a temporary that a constant evaluation makes have a literal type?

**Status:** provisional
**Affects:** CE-20, CE-23, CE-P1, `tests/expressions/constant-expressions/literal-temporary-volatile-member.cpp`

## Context

CE-P1 says that the objects an evaluation makes need a literal type.
CE-20 states the requirement for variables only. A temporary is also an
object that an evaluation makes.

The test uses a class with a `volatile` member, a `constexpr`
constructor, and a `constexpr` member function. The class is not a
literal type (CE-23). The evaluation of `Port(1).get()` in a
`static_assert` makes a temporary of the class:

- GCC 14 and 16 reject the evaluation ("temporary of non-literal type
  in a constant expression").
- Clang 18 and EDG 7.0 reject the evaluation.
- Clang 22 accepts it.

## Evidence

- `tests/expressions/constant-expressions/literal-temporary-volatile-member.cpp`.
- `tests/expressions/constant-expressions/literal-volatile-member.cpp`:
  every compiler rejects a `constexpr` variable of the same kind of
  class.

## Options

1. A temporary of a type that is not literal is not valid in constant
   evaluation, as for a variable. GCC, Clang 18, and EDG conform, and
   Clang 22 accepts a program that breaks CE-20.
2. Only a variable needs a literal type. Clang 22 conforms, and the
   other compilers reject a valid program.

## Provisional answer

Yes; constant evaluation rejects a temporary of a type that is not
literal (option 1). CE-P1 gives the requirement for every object that
an evaluation makes, and option 1 states CE-20 the same way for
variables and temporaries. A program that follows the rule builds on
every compiler.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
