# The conditional operator

The operator `?:` evaluates a condition. Then it evaluates and gives one of
two other operands, as the condition selects.

## Syntax

```{eval-rst}
.. productionlist:: expr_conditional
   conditional_expr: `logical_or_expr` ["?" `expression` ":" `assignment_expr`]
```

## Legality Rules

:::{rule} COND-1
:kind: reject
:tests: expressions/conditional/condition-converted.cpp expressions/conditional/condition-not-convertible.cpp

The first operand must be convertible to `bool` by {term}`contextual
conversion`.
:::

:::{rule} COND-3
:kind: required
:tests: expressions/conditional/lvalue-result.cpp expressions/conditional/arithmetic-prvalue.cpp expressions/conditional/mixed-category-prvalue.cpp expressions/conditional/pointer-operands.cpp expressions/conditional/bitfield-or-enum.cpp expressions/conditional/class-one-way.cpp expressions/conditional/mixed-enum-types.cpp expressions/conditional/cv-lvalues-and-strings.cpp

The second and third operands give one result type and
{term}`value category`:

- If both are lvalues of the same type, apart from cv-qualifiers, the
  result is an lvalue of that type with the combined cv-qualifiers.
- If both have arithmetic or enumeration type, the
  {term}`usual arithmetic conversions` give the type of a prvalue result.
- If both are pointers or null pointer constants, the result is a pointer
  prvalue with the combined cv-qualifiers.
- If one class type converts to the other, the result has the other type.
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

## Dynamic Semantics

:::{rule} COND-2
:kind: required
:tests: expressions/conditional/only-selected-evaluated.cpp expressions/conditional/condition-first.cpp

The condition is evaluated first. Then exactly one of the other operands is
evaluated. The operand that is not selected is not evaluated.
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

All seven compilers agree on all 23 tests. The results are in
`tests/RESULTS.md`.

- COND-6: Clang and GCC accept `a ?: b` as an extension. They reject it
  under `-pedantic-errors`. EDG rejects it in all modes.
- COND-3: operands of two different enumeration types give `int`. Clang
  and GCC warn that this use is deprecated.

Inputs: clang-sema-expr, clang-diag-sema, gcc-cp-typeck.
