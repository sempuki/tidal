# Assignment operators

The operator `=` stores the value of its right operand in the object of its
left operand. A compound assignment operator, such as `+=`, combines an
operation with the assignment.

## Syntax

```{eval-rst}
.. productionlist:: expr_assignment
   assignment_expr: `conditional_expr`
                  : `logical_or_expr` `assign_op` (`assignment_expr` | `braced_init`)
   assign_op: "=" | "*=" | "/=" | "%=" | "+=" | "-=" | "<<=" | ">>=" | "&=" | "^=" | "|="
```

## Static Semantics

:::{rule} ASN-1
:kind: required
:tests: expressions/assignment/converts-right.cpp expressions/assignment/no-conversion.cpp expressions/assignment/self-assignment.cpp

The right operand is converted to the type of the left operand by an
implicit conversion. If no implicit conversion exists, the expression is
not valid. An object can be assigned to itself.
:::

:::{rule} ASN-2
:kind: reject
:tests: expressions/assignment/const-left.cpp expressions/assignment/rvalue-left.cpp expressions/assignment/array-left.cpp expressions/assignment/function-left.cpp

The left operand must be a modifiable lvalue. It must not be:

- `const`,
- a prvalue,
- an array,
- a function.
:::

:::{rule} ASN-3
:kind: required
:tests: expressions/assignment/result-lvalue.cpp expressions/assignment/right-to-left-grouping.cpp

The result is the left operand, as an lvalue. Assignment operators group
from right to left, so `a = b = c` assigns `c` to `b` and then to `a`.
:::

:::{rule} ASN-5
:kind: required
:tests: expressions/assignment/compound-operators.cpp expressions/assignment/compound-pointer.cpp expressions/assignment/compound-float-modulo.cpp expressions/assignment/compound-converts.cpp expressions/assignment/bool-compound.cpp

`E1 op= E2` has the effect of `E1 = E1 op E2`. The operands must be valid
for `op`. The result of `op` is converted to the type of `E1`, so a
`bool` operand becomes `true` or `false`. For `+=` and `-=`, `E1` can be
a pointer and `E2` an integer.
:::

:::{rule} ASN-6
:kind: required
:tests: expressions/assignment/braced-empty.cpp expressions/assignment/braced-narrowing.cpp expressions/assignment/braced-ok.cpp expressions/assignment/braced-compound.cpp expressions/assignment/braced-compound-class.cpp

The right operand of `=` can be a braced list. For a scalar, the list
must have zero or one value, and the conversion must not narrow (LIST-2).
The right operand of a compound assignment to a scalar must not be a
braced list ({question}`Q-0032`). For a class type, the braced list is the
argument of the operator function *(referenced)*.
:::

:::{rule} ASN-7
:kind: required
:tests: expressions/assignment/volatile-simple.cpp expressions/assignment/volatile-compound.cpp

The left operand can be `volatile`. Simple and compound assignment both
apply to it.
:::

## Dynamic Semantics

:::{rule} ASN-4
:kind: required
:tests: expressions/assignment/right-before-left.cpp expressions/assignment/unsequenced-same-object.cpp

The right operand is evaluated before the left operand. All side effects of
the right operand complete before the store.
:::

:::{rule} ASN-9
:kind: required
:tests: expressions/assignment/compound-evaluates-once.cpp

In `E1 op= E2`, `E1` is evaluated one time.
:::

## Examples

```cpp
int i;
i = 3.9;          // 3
(i = 1) = 2;      // fine: the result is the lvalue i
a = b = c;        // b = c, then a = b

int& slot();
slot() = make();  // make() is called before slot()

slot() += 4;      // slot() is called once

int x = 0;
x = {};           // 0
x = {2.5};        // rejected: narrowing
x += {1};         // rejected: braced list in compound assignment
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 21 of
22 tests.

- ASN-4: `i = i++ + 1;` gives 2 on all compilers. The right operand,
  with its side effect, completes before the store.
- ASN-6: Clang 22 and 18 accept `x += {1}` for a scalar `x`. GCC and EDG
  reject it. This is an open divergence.

Inputs: clang-sema-expr, clang-diag-sema.
