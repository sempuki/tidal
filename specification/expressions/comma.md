# The comma operator

The comma operator evaluates its left operand and discards the value.
Then the operator evaluates its right operand, which gives the result.

## Principles

:::{principle} COMMA-P1
:rules: COMMA-1 COMMA-2 COMMA-3 COMMA-7 COMMA-9

The comma operator runs two expressions in order and keeps the second.
The left operand is evaluated first, with all its side effects, and its
value is dropped, so the left operand can even be `void`. The right
operand gives the result unchanged, with its own type and value
category. The overloaded operator keeps the same order. The operator
groups to the left and is allowed in a constant expression.
:::

:::{principle} COMMA-P2
:rules: COMMA-4 COMMA-6 COMMA-8

A comma also separates the items in lists such as these:

- arguments,
- initializers,
- declarators,
- enumerators,
- subscript arguments.

There are others. In such a place the separator wins, and a comma needs
parentheses to be a comma operator. A comma operator also needs
parentheses where the syntax asks for a single constant expression.
:::

:::{principle} COMMA-P3
:rules: COMMA-5

A left operand with no effect is almost surely a mistake, and an
implementation should warn about the operand. A cast to `void` says the
operand is intended.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   expression: `assignment_expr` {"," `assignment_expr`}
```

## Constraints

:::{rule} COMMA-2
:kind: required
:tests: expressions/comma/result-category.cpp expressions/comma/left-void.cpp expressions/comma/result-not-converted.cpp expressions/comma/result-lvalue-address.cpp

The result has the type and {term}`value category` of the right operand.
The right operand is not converted, so an array stays an array and a
function stays a function. The left operand can have type `void`.
:::

:::{rule} COMMA-3
:kind: required
:tests: expressions/comma/grouping.cpp expressions/comma/constant-expression.cpp expressions/comma/template-argument.cpp

The operator groups from left to right and can occur in a constant
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
- template arguments ({rule}`TARG-5`).

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

:::{rule} COMMA-9
:kind: reject
:tests: expressions/comma/braced-list-operand.cpp

Each operand of the comma operator must be an expression. A braced list
is not an operand.
:::

## Runtime Behavior

:::{rule} COMMA-1
:kind: required
:tests: expressions/comma/left-before-right.cpp expressions/comma/left-side-effects-complete.cpp expressions/comma/temporaries-end-with-full-expression.cpp expressions/comma/in-loop-and-condition.cpp

The left operand is evaluated first, and its value is discarded. All
side effects of the left operand complete before the right operand is
evaluated. A temporary that the left operand creates lives to the end
of the full expression ({rule}`VC-6`).
:::

:::{rule} COMMA-7
:kind: required
:tests: expressions/comma/overloaded-comma-order.cpp expressions/comma/overloaded-void-operand-built-in.cpp

An overloaded comma operator ({rule}`OPR-7`) evaluates its left operand
before its right operand. A parameter cannot have type `void`, so if an
operand has type `void`, the built-in comma operator applies.
:::

## Recommended Practice

:::{rule} COMMA-5
:kind: recommended
:tests: expressions/comma/unused-left-warns.cpp expressions/comma/unused-left-void-cast.cpp

An implementation should warn if the left operand has no effect. A cast
of the left operand to `void` should prevent the warning.
:::

## Examples

```cpp
int r = (a(), b());        // a() first, then b(); r gets the value of b()
(x, y) = 9;                // assigns to y
count(1, 2);               // two arguments
count((1, 2));             // one argument, the value 2
arr[i, j];                 // rejected for a built-in array
arr[(i, j)];               // fine: the comma operator, the value j
sizeof((0, arr));          // sizeof(arr): the array is not converted
return (1, {2});           // rejected: a braced list is not an operand
```

## Evidence

All eight compilers agree on all 24 tests. The results are in
`tests/RESULTS.md`.

- COMMA-5: Clang and GCC warn "left operand of comma operator has no
  effect" under `-Wunused-value`. EDG skips these tests.
- COMMA-6: GCC says that a top-level comma in a subscript "changed
  meaning in C++23".

Inputs: clang-sema-expr, clang-diag-sema, clang-parse-expr.
