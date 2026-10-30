# Sequencing and evaluation order

Sequencing rules say which parts of an expression are evaluated before
others. Some operators fix the order of their operands. These are `&&`,
`||`, the comma operator, `?:`, the shift operators, assignment, and
subscripts. For other operands the order is not fixed.

## Dynamic Semantics

:::{rule} SEQ-2
:kind: required
:tests: expressions/sequencing/callee-before-arguments.cpp expressions/sequencing/object-before-member-call-arguments.cpp

In a function call, the expression that gives the function is evaluated
before the arguments. For a member function call, this includes the object
expression.
:::

:::{rule} SEQ-3
:kind: required
:tests: expressions/sequencing/subscript-order.cpp expressions/sequencing/assignment-increment-defined.cpp

In `a[b]`, `a` is evaluated before `b`. In an assignment, the right
operand is evaluated before the left operand, with all its side effects.
:::

:::{rule} SEQ-5
:kind: required
:tests: expressions/sequencing/full-expression-side-effects.cpp

All side effects of a full expression are complete before the next full
expression is evaluated.
:::

## Undefined Behavior

:::{rule} SEQ-4
:kind: undefined
:tests: expressions/sequencing/unsequenced-increments.cpp expressions/sequencing/unsequenced-constant-evaluation.cpp

If two {term}`unsequenced` evaluations both modify the same scalar
object, or one modifies it and the other reads its value, the behavior is
undefined.
:::

## Implementation Requirements

:::{rule} SEQ-1
:kind: unspecified
:tests: expressions/sequencing/function-arguments-order.cpp expressions/sequencing/function-arguments-not-interleaved.cpp

The arguments of a function call are evaluated in an unspecified order.
Each argument is evaluated completely before the evaluation of another
argument starts.
:::

## Examples

```cpp
sum(a(), b());        // a() or b() can be called first
pick()(arg());        // pick() is called before arg()
base()[index()];      // base() is called before index()

int i = 1;
i = i++ + 1;          // fine: i is 2
i++ + i++;            // undefined
pair(i++, i++);       // fine: the order is unspecified
```

## Evidence

All seven compilers agree on all 9 tests. The results are in
`tests/RESULTS.md`.

- SEQ-1: Clang 22 evaluates function arguments from left to right and GCC
  16 from right to left, at `-O0` and `-O2`.
- SEQ-4: Clang warns "multiple unsequenced modifications" and GCC warns
  "operation may be undefined". The test makes these warnings errors.

Inputs: clang-sema-expr, clang-diag-sema, gcc-cp-typeck, gcc-cp-semantics.
