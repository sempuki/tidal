# Sequencing and evaluation order

Sequencing rules say which parts of an expression are evaluated before
others. Some operators fix the order of their operands. These are `&&`,
`||`, the comma operator, `?:`, the shift operators, assignment, and
subscripts. For other operands the order is not fixed.

## Principles

:::{principle} SEQ-P1
:rules: SEQ-2 SEQ-3 SEQ-5 SEQ-6

The language fixes the order of evaluation only where meaning needs it.
A full expression finishes, with all its side effects, before the next
one starts, so a program is a sequence of steps at that level. Within a
full expression, the function is known before its arguments are
evaluated, the right side of an assignment is complete before the left
side is touched, an array is known before its index, the object before
the member pointer applied to it, and the elements of a braced list come
in the order written. An overloaded operator keeps the order of the
built-in operator it replaces, because the program reads the same
either way.
:::

:::{principle} SEQ-P2
:rules: SEQ-1 SEQ-4 SEQ-7

Everywhere else, the order is the implementation's choice, and the
choice can differ from one call to the next. The one promise is that
each argument of a call is evaluated as a whole, and the called function
runs as a whole, so two evaluations do not interleave. A program whose
result depends on an order that the language did not fix has no meaning,
so two {term}`unsequenced` accesses to one memory location, where at
least one writes, are undefined. Where the implementation can see such
a pair, it should say so.
:::

## Runtime Behavior

:::{rule} SEQ-2
:kind: required
:tests: expressions/sequencing/callee-before-arguments.cpp expressions/sequencing/object-before-member-call-arguments.cpp

In a function call, the expression that gives the function is evaluated
before the arguments. For a member function call, this includes the object
expression.
:::

:::{rule} SEQ-3
:kind: required
:tests: expressions/sequencing/subscript-order.cpp expressions/sequencing/assignment-increment-defined.cpp expressions/sequencing/braced-list-order.cpp expressions/sequencing/compound-assignment-order.cpp expressions/sequencing/member-pointer-object-first.cpp

These operands are evaluated in a fixed order:

- In `a[b]`, `a` before `b`.
- In an assignment or a compound assignment, the right operand before the
  left operand, with all its side effects.
- In `a.*b` and `a->*b`, `a` before `b`.
- In a braced list, the elements from left to right (LIST-7).
:::

:::{rule} SEQ-5
:kind: required
:tests: expressions/sequencing/full-expression-side-effects.cpp expressions/sequencing/default-argument-temporary.cpp

All side effects of a full expression are complete before the next full
expression is evaluated. Default arguments of a call, and temporaries
that they create, belong to the full expression that contains the call.
:::

:::{rule} SEQ-6
:kind: required
:tests: expressions/sequencing/overloaded-assignment-order.cpp expressions/sequencing/overloaded-shift-order.cpp

An overloaded operator *(referenced)* keeps the order of the built-in
operator for shifts, assignment, compound assignment, subscripts, the
comma operator, and the pointer-to-member operators. An overloaded `&&`
or `||` evaluates both operands, the left one first (OPR-7).
:::

## Undefined Behavior

:::{rule} SEQ-4
:kind: undefined
:tests: expressions/sequencing/unsequenced-increments.cpp expressions/sequencing/unsequenced-constant-evaluation.cpp expressions/sequencing/adjacent-bitfields-unsequenced.cpp expressions/sequencing/volatile-reads-unsequenced.cpp

If two {term}`unsequenced` evaluations both modify the same memory
location, or one modifies it and the other reads its value, the behavior
is undefined. Adjacent bit-fields can share one memory location. A read
of a volatile object is a side effect, so two unsequenced reads of the
same volatile object are undefined too.
:::

## Implementation Requirements

:::{rule} SEQ-1
:kind: unspecified
:tests: expressions/sequencing/function-arguments-order.cpp expressions/sequencing/function-arguments-not-interleaved.cpp expressions/sequencing/calls-not-interleaved.cpp

The arguments of a function call are evaluated in an unspecified order.
Each argument is evaluated completely before the evaluation of another
argument starts. The body of a called function also does not overlap with
other evaluations in the caller.
:::

## Recommended Practice

:::{rule} SEQ-7
:kind: recommended
:tests: expressions/sequencing/unsequenced-increments.cpp

An implementation should warn when it can see two unsequenced
evaluations in one expression that modify the same object, or that
modify it and read it.
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

All seven compilers agree on all 18 tests. The results are in
`tests/RESULTS.md`.

- SEQ-1: Clang 22 evaluates function arguments from left to right and GCC
  16 from right to left, at `-O0` and `-O2`.
- SEQ-4: Clang warns "multiple unsequenced modifications" and GCC warns
  "operation may be undefined". The test makes these warnings errors.
  No compiler warns about two bit-fields of one memory location or two
  reads of a volatile object; those tests only show the programs compile.

Inputs: clang-sema-expr, clang-diag-sema.
