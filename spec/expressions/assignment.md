# Assignment operators

The operator `=` stores the value of its right operand in the object of its
left operand. A compound assignment operator, such as `+=`, combines an
operation with the assignment.

## Principles

:::{principle} ASN-P1
:rules: ASN-1 ASN-2 ASN-3 ASN-6 ASN-7 ASN-10 ASN-12 ASN-14

Assignment stores a value into an object. So the left operand must be an
object that can be changed. It must be:

- an lvalue,
- not `const`,
- not an array,
- not a function.

A `volatile` object can be assigned. The right operand is converted to
the object's type by an implicit conversion, and a braced list converts
as list initialization does, without narrowing. The result is the object
itself, as an lvalue, so assignments chain from the right. A bit-field
is an object with no address, so the result of assigning to it has none
either. For a class, assignment is a call to the class's own operator,
so the class decides what its left operand can be.
:::

:::{principle} ASN-P2
:rules: ASN-5 ASN-9 ASN-11 ASN-13

A compound assignment `E1 op= E2` means `E1 = E1 op E2`, with one
difference: `E1` is evaluated once. The read, the operation, and the
write are one evaluation, so a function call that is unordered with it
runs either before the read or after the write, never in between.
:::

:::{principle} ASN-P3
:rules: ASN-4

The right operand is evaluated before the left one, and all its side
effects complete before the store. This is an exception to the usual
freedom (SEQ-1).
:::

## Syntax

```{eval-rst}
.. productionlist:: expr_assignment
   assignment_expr: `conditional_expr`
                  : `logical_or_expr` `assign_op` (`assignment_expr` | `braced_init`)
   assign_op: "=" | "*=" | "/=" | "%=" | "+=" | "-=" | "<<=" | ">>=" | "&=" | "^=" | "|="
```

## Constraints

:::{rule} ASN-1
:kind: required
:tests: expressions/assignment/converts-right.cpp expressions/assignment/self-assignment.cpp

The right operand is converted to the type of the left operand by an
implicit conversion. An object can be assigned to itself.
:::

:::{rule} ASN-12
:kind: reject
:tests: expressions/assignment/no-conversion.cpp

If no implicit conversion from the right operand to the type of the
left operand exists (ASN-1), the expression is not valid.
:::

:::{rule} ASN-2
:kind: reject
:tests: expressions/assignment/const-left.cpp expressions/assignment/rvalue-left.cpp expressions/assignment/array-left.cpp expressions/assignment/function-left.cpp expressions/assignment/class-prvalue-left.cpp expressions/assignment/scalar-xvalue-left.cpp expressions/member-pointers/result-const.cpp expressions/primary/this-const-member-modify.cpp expressions/primary/member-const-in-const-function.cpp

The left operand of a built-in assignment must be a modifiable lvalue. It
must not be:

- `const`,
- a prvalue or an xvalue,
- an array,
- a function.

For a class type, assignment calls an operator function ({rule}`OPR-7`),
so a class prvalue can be the left operand.
:::

:::{rule} ASN-3
:kind: required
:tests: expressions/assignment/result-lvalue.cpp expressions/assignment/right-to-left-grouping.cpp

The result is the left operand, as an lvalue. Assignment operators group
from right to left, so `a = b = c` assigns `c` to `b` and then to `a`.
:::

:::{rule} ASN-5
:kind: required
:tests: expressions/assignment/compound-operators.cpp expressions/assignment/compound-pointer.cpp expressions/assignment/compound-converts.cpp expressions/assignment/bool-compound.cpp

`E1 op= E2` has the effect of `E1 = E1 op E2`. The result of `op` is
converted to the type of `E1`, so a `bool` operand becomes `true` or
`false`. For `+=` and `-=`, `E1` can be a pointer and `E2` an integer.
:::

:::{rule} ASN-13
:kind: reject
:tests: expressions/assignment/compound-float-modulo.cpp

The operands of `E1 op= E2` must be valid for `op` (ASN-5).
:::

:::{rule} ASN-6
:kind: required
:tests: expressions/assignment/braced-empty.cpp expressions/assignment/braced-ok.cpp expressions/assignment/braced-compound-class.cpp

The right operand of `=` can be a braced list. For a class type, the
braced list is the argument of the operator function ({rule}`OPR-7`).
:::

:::{rule} ASN-14
:kind: reject
:tests: expressions/assignment/braced-narrowing.cpp expressions/assignment/braced-compound.cpp

For a scalar, a braced list on the right of `=` (ASN-6) must have zero
or one value, and the conversion must not narrow (LIST-2). The right
operand of a compound assignment to a scalar must not be a braced list
({question}`Q-0032`).
:::

:::{rule} ASN-7
:kind: required
:tests: expressions/assignment/volatile-simple.cpp expressions/assignment/volatile-compound.cpp

The left operand can be `volatile`. Simple and compound assignment both
apply to it.
:::

:::{rule} ASN-10
:kind: reject
:tests: expressions/assignment/bitfield-result-address.cpp

If the left operand is a bit-field, the result is that bit-field. A
program must not take the address of the result.
:::

## Runtime Behavior

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

## Implementation Requirements

:::{rule} ASN-11
:kind: unspecified
:tests: expressions/assignment/compound-indeterminate-with-call.cpp expressions/assignment/compound-indeterminate-run.cpp

A compound assignment is one evaluation with respect to a function call
whose order with it is not set. The call runs before the read or after
the write, never between them. Which of the two orders occurs is
unspecified, and the behavior is not undefined.
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

The results are in `tests/RESULTS.md`. All seven compilers agree on 28 of 29 tests.

- ASN-4: `i = i++ + 1;` gives 2 on all compilers. The right operand,
  with its side effect, completes before the store.
- ASN-6: Clang 22 and 18 accept `x += {1}` for a scalar `x`. GCC and EDG
  reject it. This is an open divergence.

Inputs: clang-sema-expr, clang-diag-sema.
