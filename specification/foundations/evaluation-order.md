# Sequencing and evaluation order

Sequencing rules say which parts of an expression are evaluated before
others. These operators fix the order of their operands:

- `&&` and `||`,
- the comma operator,
- `?:`,
- the shift operators,
- assignment,
- subscripts.

For other operands the order is not fixed.

## Principles

:::{principle} SEQ-P1
:rules: SEQ-2 SEQ-3 SEQ-5 SEQ-6 SEQ-9 SEQ-10 SEQ-11

The language fixes the order of evaluation only where meaning needs it.
A full expression finishes, with all its side effects, before the next
full expression starts, so a program is a sequence of steps at that
level. Within a full expression, these orders are fixed:

| Construct | Evaluated first | Evaluated after |
|---|---|---|
| Call | The function | Its arguments |
| Call | Its arguments, with their side effects | The body of the function |
| Assignment | The right side, complete | The left side |
| Subscript | The array | Its index |
| Member pointer | The object | The member pointer applied to the object |
| Braced list | Each element | The elements after that element, in the order written |

An overloaded operator keeps the order of the built-in operator it
replaces, because the program reads the same either way.
:::

:::{principle} SEQ-P2
:rules: SEQ-1 SEQ-4 SEQ-7 SEQ-8

Everywhere else, the order is the implementation's choice, and the
choice can differ from one call to the next. The language promises only
that each argument of a call is evaluated as a whole, and that the
called function runs as a whole, so two evaluations do not interleave. A
program whose result depends on an order that the language did not fix
has no meaning, so two {term}`unsequenced` accesses to one memory
location, where at least one writes, are undefined. Where the
implementation can see such a pair, the implementation should report
the pair.
:::

## Runtime Behavior

:::{rule} SEQ-2
:kind: required
:tests: expressions/sequencing/callee-before-arguments.cpp expressions/sequencing/object-before-member-call-arguments.cpp

In a function call, the expression that gives the function is evaluated
before the arguments. For a member function call, the expression that
gives the function includes the object expression. {rule}`SEQ-1` gives
the place of the initialization of an explicit object parameter. In
operator syntax the object is an operand, and SEQ-8 gives the order of
the object.
:::

:::{rule} SEQ-9
:kind: required
:tests: expressions/sequencing/implicit-calls.cpp

A program calls a function without naming it in these places:

| Place | Functions called | Rule |
|---|---|---|
| The start of an object's life | its constructor, and the constructors of its bases and members | {rule}`CTOR-6` |
| The end of an object's life, also during unwinding | its destructor, and the destructors of its members and bases | {rule}`DTOR-5`, {rule}`UNW-1` |
| An implicit conversion | a converting constructor or a conversion function | {rule}`CTOR-10`, {rule}`CVF-2` |
| A function argument or parameter of class type | a constructor that initializes the parameter, and its destructor | {rule}`PFX-6` |
| Operator syntax on a class or enumeration operand | the selected operator function, also a rewritten or reversed comparison | {rule}`OPR-7`, {rule}`DCMP-6` |
| A braced list for a class | the selected constructor, also an initializer-list constructor | {rule}`ILC-1` |
| A `new` or `delete` expression | the allocation or deallocation function | {rule}`NEW-9` |
| A range-based `for` | `begin`, `end`, and the iterator operations | {rule}`RFOR-1`, {rule}`RFOR-2` |
| A structured binding by the tuple protocol | `get` | {rule}`SB-5` |
| A `throw` | the constructor that initializes the exception object | {rule}`THR-1` |
:::

:::{rule} SEQ-3
:kind: required
:tests: expressions/sequencing/subscript-order.cpp expressions/sequencing/assignment-increment-defined.cpp expressions/sequencing/braced-list-order.cpp expressions/sequencing/compound-assignment-order.cpp expressions/sequencing/member-pointer-object-first.cpp expressions/sequencing/chained-assignment-order.cpp expressions/sequencing/assignment-right-before-left-index.cpp

These operands are evaluated in a fixed order:

| Expression | Evaluated first | Evaluated after |
|---|---|---|
| `a[b]` | `a` | `b` |
| An assignment or a compound assignment | The right operand, with all its side effects | The left operand |
| `a.*b` and `a->*b` | `a` | `b` |
| A braced list (LIST-7) | Each element | The elements to its right |
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
:tests: expressions/sequencing/overloaded-assignment-order.cpp expressions/sequencing/overloaded-shift-order.cpp expressions/sequencing/overloaded-subscript-order.cpp expressions/sequencing/overloaded-compound-assignment-order.cpp expressions/sequencing/overloaded-member-pointer-order.cpp

An overloaded operator ({rule}`OPR-7`) keeps the order of the built-in
operator for these operators:

- shifts,
- assignment and compound assignment,
- subscripts,
- the comma operator,
- the pointer-to-member operators.

An overloaded `&&` or `||` evaluates both operands, the left one first
(OPR-7).
:::

:::{rule} SEQ-10
:kind: required
:tests: expressions/sequencing/arguments-before-body.cpp

The arguments of a call, with all their side effects, are complete
before the body of the called function starts.
:::

:::{rule} SEQ-11
:kind: required
:tests: expressions/sequencing/operand-used-twice-evaluated-once.cpp

One evaluation of a construct evaluates each of its operands at most
once. If the construct uses the result of an operand more than once,
each use refers to that one result. These constructs use a result more
than once:

| Construct | Operand | Uses | Rule |
|---|---|---|---|
| `E1 op= E2` | `E1` | a read and a write | {rule}`ASN-9` |
| `++E`, `--E`, `E++`, `E--` | `E` | a read and a write | {rule}`UN-10`, {rule}`PFX-12` |
| A structured binding declaration | the initializer | each name, through the hidden variable | {rule}`SB-1` |
| A range-based `for` | the range expression | `begin` and `end`, through a hidden reference | {rule}`RFOR-3` |
| A `switch` | the condition | the comparison with each case label | {rule}`SW-1` |
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
:tests: expressions/sequencing/function-arguments-order.cpp expressions/sequencing/function-arguments-not-interleaved.cpp expressions/sequencing/calls-not-interleaved.cpp expressions/sequencing/default-argument-order.cpp expressions/sequencing/argument-read-and-write.cpp expressions/sequencing/explicit-object-order-unspecified.cpp

The arguments of a function call, including its default arguments, are
evaluated in an unspecified order. Each argument is evaluated completely
before the evaluation of another argument starts, so a write in one
argument and a read of the same object in another argument are not
undefined. The body of a called function also does not overlap with
other evaluations in the caller. The initialization of an explicit
object parameter ({rule}`CLS-13`) from the object takes an unspecified
place in this order ({question}`Q-0319`).
:::

:::{rule} SEQ-8
:kind: unspecified
:tests: expressions/sequencing/calls-not-interleaved.cpp expressions/sequencing/comparison-operand-order.cpp expressions/sequencing/bitwise-operand-order.cpp expressions/sequencing/multiplicative-operand-order.cpp expressions/sequencing/member-operator-order.cpp expressions/sequencing/reversed-comparison-order.cpp

The operands of an operator are evaluated in an unspecified order,
unless one of the rules below fixes the order. The order is unspecified
for a built-in operator, whose operands are {term}`unsequenced`, and for
an overloaded operator, including a member operator and a reversed
comparison ({rule}`DCMP-6`). These rules fix an order:

- SEQ-3: subscripts, assignments, member pointers, and braced lists,
- {rule}`SHF-8`: shifts,
- {rule}`LOG-5`: `&&` and `||`,
- {rule}`COMMA-1`: the comma operator,
- {rule}`COND-2`: the conditional operator,
- SEQ-6: the overloaded operators that keep a built-in order.

A function called in one operand still does not overlap with the other
operand (SEQ-1).
:::

## Recommended Practice

:::{rule} SEQ-7
:kind: recommended
:tests: expressions/sequencing/unsequenced-increments.cpp

An implementation should warn when it can see two unsequenced
evaluations in one expression where both modify the same object, or
where one modifies the object and the other reads the object.
:::

## Examples

```cpp
sum(a(), b());        // a() or b() can be called first
pick()(arg());        // pick() is called before arg()
base()[index()];      // base() is called before index()

int i = 1;
i = i++ + 1;          // fine: i is 2
i++ + i++;            // undefined
i + i++;              // undefined: a read and a write
pair(i++, i++);       // fine: the order is unspecified
pair(i++, i);         // fine: pair(1, 2) or pair(1, 1) when i is 1
cells[i++] = i;       // fine: the right side is read first
show(g++);            // show's body sees the new value of g

struct Flags { unsigned on : 1, dirty : 1; } f;
(f.on = 1) + (f.dirty = 1);   // undefined: one memory location
```

## Evidence

All eight compilers agree on all 34 tests. The results are in
`tests/RESULTS.md`.

- SEQ-1: Clang and GCC 16 initialize an explicit object parameter
  before they evaluate the other arguments. GCC 14 evaluates another
  argument first ({question}`Q-0319`).
- SEQ-1 and SEQ-8: the table gives the order that each compiler takes.
  Each operand is a call that records its turn. The run-time results are
  the same at `-O0` and `-O2`. EDG 7.0 generates no code, so its column
  comes from constant evaluation only.

  | Construct | Clang 18, Clang 22 | GCC 14, GCC 16 at run time | GCC in constant evaluation | EDG 7.0 |
  |---|---|---|---|---|
  | `f(a(), b())` (SEQ-1) | Left to right | Right to left | Left to right | Left to right |
  | An explicit argument and a default argument (SEQ-1) | The explicit argument first | The default argument first | Not observed | Not observed |
  | Built-in `+`, `*`, `<`, `&` (SEQ-8) | The left operand first | The left operand first | The left operand first | The left operand first |
  | A member `operator-` (SEQ-8) | The left operand first | The right operand first | The left operand first | The left operand first |
  | A reversed `==` candidate, `x == y` called as `y == x` (SEQ-8) | `y` first | `x` first | `y` first | `x` first |

  Clang gives the same order at run time and in constant evaluation. The
  built-in operators in constant evaluation were checked with `+` only.
- SEQ-1: when `i` is 0, every compiler calls `pair(0, 1)` for
  `pair(i++, i)`, at `-O0` and `-O2`. GCC gives this result although
  GCC evaluates the arguments of `f(a(), b())` from right to left. The
  test accepts `pair(0, 0)` too, because the rule allows it.
- SEQ-4: during constant evaluation, no compiler rejects two
  unsequenced modifications of one object, such as `i++ + i++`.
  {rule}`CE-26` allows this.
- SEQ-4: Clang warns "multiple unsequenced modifications" and GCC warns
  "operation may be undefined". The test makes these warnings errors. No
  compiler warns about two bit-fields of one memory location or two
  reads of a volatile object; those tests only show the programs
  compile.
- SEQ-4: no sanitizer of GCC or Clang stops two unsequenced
  modifications at run time.

Inputs: clang-sema-expr, clang-diag-sema.
