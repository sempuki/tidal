# Q-0335: Must a temporary that a constant evaluation makes have a literal type?

**Status:** provisional
**Affects:** CE-17, CE-23, CE-P1, `tests/foundations/constant-evaluation/literal-temporary-volatile-member.cpp`, `tests/foundations/constant-evaluation/nonliteral-variable-reached.cpp`, `tests/foundations/constant-evaluation/nonliteral-temporary-in-function.cpp`
**Waits for:** newest families that agree

## Context

CE-P1 says that the objects an evaluation makes need a literal type.
CE-17 states the requirement for variables, and for temporaries under
this question. A temporary is also an object that an evaluation
makes.

The test uses a class with a `volatile` member, a `constexpr`
constructor, and a `constexpr` member function. The class is not a
literal type (CE-23). The evaluation of `Port(1).get()` in a
`static_assert` makes a temporary of the class:

- GCC 14 and 16 reject the evaluation ("temporary of non-literal type
  in a constant expression").
- Clang 18 and EDG 7.0 reject the evaluation.
- Clang 22 accepts it.

## Evidence

- `tests/foundations/constant-evaluation/literal-temporary-volatile-member.cpp`.
- `tests/foundations/constant-evaluation/literal-volatile-member.cpp`:
  every compiler rejects a `constexpr` variable of the same kind of
  class.

## Options

1. A temporary of a type that is not literal is not valid in constant
   evaluation, as for a variable. GCC, Clang 18, and EDG conform, and
   Clang 22 accepts a program that breaks CE-17.
2. Only a variable needs a literal type. Clang 22 conforms, and the
   other compilers reject a valid program.

## Provisional answer

No; only a `constexpr` variable needs a literal type, and a constant evaluation can make a temporary or a local variable of any type (option 2). Clang 22 accepts every form. GCC rejects a temporary, and EDG rejects a temporary in the condition of a `static_assert`.

Changed on 2026-11-03 to option 2. A probe on the matrix shows that the variable item of CE-17 disagrees with every newest compiler: for a local variable of a class with a `volatile` member in a `constexpr` function, Clang 22, GCC 16, and EDG 7.0 accept the evaluation, and only Clang 18 rejects it. For a temporary in a `constexpr` function, Clang 22 and EDG accept, and GCC rejects. For a temporary in the condition of a `static_assert`, Clang 22 accepts, and GCC and EDG reject. CE-P1 now says that only a `constexpr` variable needs a literal type, because its value outlives the evaluation, and that an evaluation fails only at an operation that it cannot do, which is its model for every other construct. CE-17 drops the variable and temporary items. `literal-temporary-volatile-member.cpp` expects acceptance, with overrides for GCC, EDG, and Clang 18, and the new tests `nonliteral-variable-reached.cpp` and `nonliteral-temporary-in-function.cpp` cover the other two forms. The question stays open under decision 0054, because two newest families differ for the temporary in a `static_assert`.

Kept open on 2026-11-04 under decision 0054, because two newest families, GCC 16 and EDG 7.0, differ from the answer for a temporary in a `static_assert`.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) reads option 2. Nothing stops a constant evaluation from making a temporary or a local variable of a type that is not literal. Only a variable declared `constexpr` needs a literal type. A `constexpr` constructor and member function of a class that is not literal are allowed, and the call in the test reads only the member that is not volatile. Clang 22's acceptance conforms. The same reasoning removes the item of CE-17 about a variable of a type that is not literal.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
