# The comma operator

The comma operator evaluates its left operand and discards the value. Then
it evaluates its right operand, which gives the result.

## Principles

:::{principle} COMMA-P1
:rules: COMMA-1 COMMA-2 COMMA-3 COMMA-7

The comma operator runs two expressions in order and keeps the second.
The left operand is evaluated first, with all its side effects, and its
value is dropped, so it can even be `void`. The right operand gives the
result, with its own type and value category. The overloaded operator
keeps the same order. The operator groups to the left and is allowed in
a constant expression.
:::

:::{principle} COMMA-P2
:rules: COMMA-4 COMMA-6 COMMA-8

A comma also separates items in lists: arguments, initializers,
declarators, enumerators, subscript arguments, and others. In such a
place the separator wins, and a comma operator needs parentheses to be
one. The same holds where the syntax asks for a single constant
expression.
:::

:::{principle} COMMA-P3
:rules: COMMA-5

A left operand with no effect is almost surely a mistake, and an
implementation should warn about it. A cast to `void` says it is
intended.
:::

## Syntax

```{eval-rst}
.. productionlist:: expr_comma
   expression: `assignment_expr` {"," `assignment_expr`}
```

## Constraints

:::{rule} COMMA-2
:kind: required
:tests: expressions/comma/result-category.cpp expressions/comma/left-void.cpp

The result has the type and {term}`value category` of the right operand.
The left operand can have type `void`.
:::

:::{rule} COMMA-3
:kind: required
:tests: expressions/comma/grouping.cpp expressions/comma/constant-expression.cpp

The operator groups from left to right. It can occur in a constant
expression.
:::

:::{rule} COMMA-4
:kind: required
:tests: expressions/comma/argument-separator.cpp expressions/comma/initializer-separator.cpp expressions/comma/separators.cpp

In these lists, a comma separates the items:

- function arguments and initializers,
- declarators in one declaration,
- base and member initializers,
- enumerators,
- template arguments *(templates referenced)*.

A comma operator in such a list must be in parentheses.
:::

:::{rule} COMMA-6
:kind: reject
:tests: expressions/comma/subscript-comma.cpp expressions/comma/subscript-parenthesized.cpp

A comma at the top level of a subscript separates subscript arguments
({rule}`OPR-3`). The built-in subscript takes one argument, so the comma
must be in parentheses to act as an operator.
:::

:::{rule} COMMA-8
:kind: reject
:tests: expressions/comma/array-bound-unparenthesized.cpp expressions/comma/array-bound-parenthesized.cpp expressions/comma/case-label-unparenthesized.cpp

Where the syntax asks for a constant expression, such as an array bound
or a `case` label, a comma operator must be in parentheses.
:::

## Runtime Behavior

:::{rule} COMMA-1
:kind: required
:tests: expressions/comma/left-before-right.cpp expressions/comma/left-side-effects-complete.cpp

The left operand is evaluated first, and its value is discarded. All its
side effects complete before the right operand is evaluated.
:::

:::{rule} COMMA-7
:kind: required
:tests: expressions/comma/overloaded-comma-order.cpp

An overloaded comma operator ({rule}`OPR-7`) evaluates its left operand
before its right operand.
:::

## Recommended Practice

:::{rule} COMMA-5
:kind: recommended
:tests: expressions/comma/unused-left-warns.cpp expressions/comma/unused-left-void-cast.cpp

An implementation should warn if the left operand has no effect. A cast of
the left operand to `void` should prevent the warning.
:::

## Examples

```cpp
int r = (a(), b());        // a() first, then b(); r gets the value of b()
(x, y) = 9;                // assigns to y
count(1, 2);               // two arguments
count((1, 2));             // one argument, the value 2
arr[i, j];                 // rejected for a built-in array
arr[(i, j)];               // fine: the comma operator, the value j
```

## Evidence

All seven compilers agree on all 17 tests. The results are in
`tests/RESULTS.md`.

- COMMA-5: Clang and GCC warn "left operand of comma operator has no
  effect" under `-Wunused-value`. EDG skips these tests.
- COMMA-6: GCC says that a top-level comma in a subscript "changed meaning
  in C++23".

Inputs: clang-sema-expr, clang-diag-sema, clang-parse-expr.
