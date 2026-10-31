# The conditional operator

The operator `?:` evaluates a condition. Then it evaluates and gives one of
two other operands, as the condition selects.

## Syntax

```{eval-rst}
.. productionlist:: expr_conditional
   conditional_expr: `logical_or_expr` ["?" `expression` ":" `assignment_expr`]
```

## Constraints

:::{rule} COND-1
:kind: reject
:tests: expressions/conditional/condition-converted.cpp expressions/conditional/condition-not-convertible.cpp

The first operand must be convertible to `bool` by {term}`contextual
conversion`.
:::

:::{rule} COND-3
:kind: required
:tests: expressions/conditional/lvalue-result.cpp expressions/conditional/arithmetic-prvalue.cpp expressions/conditional/mixed-category-prvalue.cpp expressions/conditional/pointer-operands.cpp expressions/conditional/bitfield-or-enum.cpp expressions/conditional/class-one-way.cpp expressions/conditional/mixed-enum-types.cpp expressions/conditional/cv-lvalues-and-strings.cpp expressions/conditional/two-zeros.cpp expressions/conditional/member-pointer-nullptr.cpp expressions/conditional/xvalue-operands.cpp expressions/conditional/const-volatile-lvalues-prvalue.cpp expressions/conditional/const-and-plain-lvalue.cpp expressions/conditional/derived-base-lvalue.cpp expressions/conditional/class-converts-to-int.cpp expressions/conditional/zero-and-nullptr.cpp

The second and third operands give one result type and
{term}`value category`:

- If both are lvalues of the same type, apart from cv-qualifiers, and the
  cv-qualifiers of one include those of the other, the result is an
  lvalue with the larger set. If neither set includes the other, as for
  `const` and `volatile`, the result is a prvalue.
- If both are xvalues, the result is an xvalue. If one is an lvalue and
  the other an xvalue, the result is a prvalue.
- If both have arithmetic or enumeration type, the
  {term}`usual arithmetic conversions` give the type of a prvalue result.
  Two integer literals `0` give `int`.
- If one is a pointer and the other is a pointer or a null pointer
  constant, the result is a pointer prvalue with the combined
  cv-qualifiers.
- If both are `nullptr`, or one is `nullptr` and the other the literal
  `0`, the result has the type of `nullptr` ({question}`Q-0102`).
- A pointer to member and `nullptr` give a pointer to member.
- If one operand has class type and converts to the type of the other,
  the result has the other type. A derived class lvalue and a base class
  lvalue give a base class lvalue. A class that converts to `int` and an
  `int` give an `int` prvalue.
:::

:::{rule} COND-4
:kind: required
:tests: expressions/conditional/throw-operand.cpp expressions/conditional/void-operands.cpp expressions/conditional/void-and-value.cpp

A throw-expression operand does not affect the result. In that case the
other operand gives the result its type and category. If both operands
have type `void`, the result is `void`. A `void` operand with a value
operand is not valid.
:::

:::{rule} COND-5
:kind: reject
:tests: expressions/conditional/no-common-type.cpp expressions/conditional/pointer-and-int.cpp expressions/conditional/class-ambiguous.cpp

The second and third operands must have a common type by COND-3 or COND-4.
If each of two class types converts to the other, the expression is
ambiguous and not valid.
:::

:::{rule} COND-6
:kind: required
:tests: expressions/conditional/grouping.cpp expressions/conditional/precedence-assignment.cpp expressions/conditional/gnu-omitted-operand.cpp

The operator groups from right to left. The third operand can be an
assignment. The second operand must be present.
:::

:::{rule} COND-7
:kind: required
:tests: expressions/conditional/constant-unselected-fails.cpp expressions/conditional/constant-selected-fails.cpp

During {term}`constant evaluation`, only the selected operand must be a
constant expression. The operand that is not selected can contain any
expression.
:::

:::{rule} COND-8
:kind: reject
:tests: expressions/conditional/class-lvalue-prvalue-copy.cpp

If one operand is an lvalue and the other a prvalue of the same class
type, the result is a prvalue copied from the selected operand. The copy
must be valid, so a class with a deleted copy constructor is rejected.
:::

:::{rule} COND-9
:kind: reject
:tests: expressions/conditional/bitfield-address.cpp

An lvalue result that refers to a bit-field operand *(referenced)* has
the limits of a bit-field. A program must not apply `&` to it.
:::

:::{rule} COND-10
:kind: required
:tests: expressions/conditional/comma-in-middle.cpp

The second operand can be a comma expression without parentheses.
:::

:::{rule} COND-11
:kind: required
:tests: expressions/conditional/two-classes-convert-to-int.cpp

If the operands have different types, at least one is a class, and
neither converts to the other, overload resolution *(referenced)* over
the built-in operator candidates picks a conversion for each operand.
Two unrelated classes that both convert to `int` give an `int` prvalue.
:::

## Dynamic Semantics

:::{rule} COND-2
:kind: required
:tests: expressions/conditional/only-selected-evaluated.cpp expressions/conditional/condition-first.cpp expressions/conditional/condition-side-effects-first.cpp

The condition is evaluated first, with all its side effects. Then exactly
one of the other operands is evaluated. The operand that is not selected
is not evaluated.
:::

:::{rule} COND-12
:kind: required
:tests: expressions/conditional/prvalue-no-copy.cpp

If both operands are prvalues of the same class type, the result is a
prvalue that initializes its target directly. No copy or move
constructor is called, so the class needs none.
:::

## Examples

```cpp
int x = 1, y = 2;
(c ? x : y) = 10;              // fine: both are lvalues of type int
auto a = c ? 1 : 2.5;          // double
auto p = c ? &x : nullptr;     // int*
int& r = c ? x : throw 1;      // the type of x
int v = n == 0 ? 10 : n == 1 ? 11 : 12;   // groups as n == 0 ? 10 : (...)
auto bad = c ? &x : 1;         // rejected: no common type
```

## Evidence

All seven compilers agree on 36 of 37 tests. The results are in
`tests/RESULTS.md`.

- COND-6: Clang and GCC accept `a ?: b` as an extension. They reject it
  under `-pedantic-errors`. EDG rejects it in all modes.
- COND-3: operands of two different enumeration types give `int`. Clang
  and GCC warn that this use is deprecated.
- COND-3: GCC rejects a conditional with the literal `0` and `nullptr`.
  Clang and EDG accept it ({question}`Q-0102`).

Inputs: clang-sema-expr, clang-diag-sema.
