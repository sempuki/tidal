# Throw expressions and the exception object

A `throw` expression leaves the current point of execution with a value.
The value becomes the exception object, which lives until the last
handler for it finishes. This page covers:

- the operand of `throw`,
- how the exception object is made from it,
- its type and lifetime,
- `throw` with no operand,
- `throw` during constant evaluation.

Depends on: value categories and temporaries, copy and move, constant
evaluation, objects and lifetime.

## Principles

:::{principle} THR-P1
:rules: THR-1 THR-2 THR-3

A `throw` makes the exception object from its operand the way a
declaration makes a variable from an initializer.

- A prvalue operand becomes the exception object directly, so no copy
  or move happens and none needs to exist ({principle}`VC-P2`).
- A local variable is about to end, so it is moved.
- Any other lvalue is copied.

Arrays and functions decay to pointers, and cv-qualifiers are dropped,
because the exception object is a fresh object of its own. So the
operand's type must be complete and not abstract, and the copy or move
that would run must be usable, as must the destructor that ends the
exception object.
:::

:::{principle} THR-P2
:rules: THR-4 THR-6 THR-8

The exception object belongs to no scope ({principle}`OBJ-P2` does not
apply to it). The object:

- begins at the `throw`,
- outlives every block that the exception leaves,
- ends when the last handler for it ends.

Its type is fixed at the `throw` as the static type of the operand, so a
derived object thrown through a base reference is sliced. A rethrow
passes the same object on, with any changes a handler made to it, and a
handler that takes it by value works on a copy.
:::

:::{principle} THR-P3
:rules: THR-5

A `throw` is an expression that never produces a value, so its type is
`void`. It can stand wherever no value is needed, including as an
operand of the conditional operator, where it takes no part in the
result type (COND-4).
:::

:::{principle} THR-P4
:rules: THR-7 THR-9

A `throw` with no operand passes on the exception that a handler is
handling. When there is no such exception, or when an exception leaves
a function that promised not to throw, no handler can take over, and the
program terminates instead of continuing in an unknown state. An
implementation should warn where it can see that happen.
:::

:::{principle} THR-P5
:rules: THR-10

During constant evaluation a `throw` has nowhere to go, so an evaluation
that reaches one is not a constant and the program is rejected there
({principle}`TR-P4`). A `throw` on a path the evaluation does not take
costs nothing, and the same function throws normally when called at run
time.
:::

## Syntax

```{eval-rst}
.. productionlist:: throw
   throw_expr: "throw" [`assignment_expression`]
```

A `throw` with no operand is a *rethrow*.

## Constraints

:::{rule} THR-3
:kind: reject
:tests: exceptions/throw/deleted-copy-lvalue-rejected.cpp exceptions/throw/private-copy-rejected.cpp exceptions/throw/abstract-operand-rejected.cpp exceptions/throw/incomplete-operand-rejected.cpp exceptions/throw/private-destructor-rejected.cpp exceptions/throw/throwing-destructor-accepted.cpp

The operand of `throw` must have a complete type that is not abstract.
If the exception object is copied or moved from the operand, that
constructor must be usable and accessible, and so must the destructor
of the exception object. A destructor that is potentially throwing is
permitted.
:::

:::{rule} THR-5
:kind: required
:tests: exceptions/throw/throw-expression-type-void.cpp

A `throw` expression is a prvalue of type `void`. It can be an operand
of the conditional operator (COND-4).
:::

:::{rule} THR-10
:kind: required
:tests: exceptions/throw/constexpr-throw-at-runtime.cpp

A `constexpr` function that contains a `throw` can be called at run
time, and it throws then. Constant evaluation of the `throw` follows
{rule}`CE-8`.
:::

## Runtime Behavior

:::{rule} THR-1
:kind: required
:tests: exceptions/throw/prvalue-initializes-directly.cpp exceptions/throw/deleted-copy-prvalue.cpp exceptions/throw/global-lvalue-copied.cpp exceptions/throw/local-variable-moved.cpp

A `throw` initializes the exception object from its operand:

- A prvalue operand initializes it directly, with no copy or move.
- A local variable operand is moved.
- Any other lvalue operand is copied.
:::

:::{rule} THR-2
:kind: required
:tests: exceptions/throw/array-operand-decays.cpp exceptions/throw/function-operand-decays.cpp exceptions/throw/string-literal-decays.cpp exceptions/throw/cv-qualifiers-dropped.cpp

An array operand becomes a pointer to its first element, and a function
operand becomes a pointer to the function. The exception object has no
cv-qualifiers, so a handler can bind a non-`const` reference to an
object thrown from a `const` operand.
:::

:::{rule} THR-4
:kind: required
:tests: exceptions/throw/static-type-not-dynamic.cpp

The type of the exception object is the static type of the operand. An
object of a derived class thrown through a reference to its base gives
an exception object of the base type, and a handler for the derived
type does not match it.
:::

:::{rule} THR-6
:kind: required
:tests: exceptions/throw/rethrow-same-object.cpp exceptions/throw/rethrow-keeps-changes.cpp exceptions/throw/rethrow-in-handler-accepted.cpp

A rethrow in a handler passes on the exception object that the handler
is handling. It is the same object, with any changes that the handler
made to it.
:::

:::{rule} THR-7
:kind: required
:tests: exceptions/throw/rethrow-outside-handler-terminates.cpp exceptions/throw/rethrow-after-handler-left-terminates.cpp

A rethrow when no exception is being handled terminates the program.
This includes a rethrow after the handler that was handling the
exception has ended.
:::

:::{rule} THR-8
:kind: required
:tests: exceptions/throw/object-lives-through-handler.cpp exceptions/throw/object-lives-through-rethrow.cpp exceptions/throw/catch-by-value-copies.cpp exceptions/throw/new-throw-in-handler-ends-old.cpp

The exception object lives until the last handler for it ends, through
rethrows. A handler that takes the exception by value gets a copy, and
the exception object still lives until the handler ends. A new exception
thrown from a handler ends the handler, and so ends the old exception
object.
:::

:::{rule} THR-9
:kind: required
:tests: exceptions/throw/throw-in-noexcept-terminates.cpp

If an exception leaves a function declared `noexcept`, the program
terminates. No handler in a caller catches it.
:::

## Recommended Practice

:::{rule} THR-11
:kind: withdrawn

{rule}`UNW-8` states this recommendation.
:::

## Examples

```cpp
struct Error { int code; };

int parse(const char* text) {
  if (text == nullptr) throw Error{1};     // Error{1} is the exception object
  return text[0] == '-' ? throw Error{2} : 0;   // throw as an operand
}

void retry() {
  try { parse(nullptr); }
  catch (Error& e) { e.code += 10; throw; }  // passes on the same object
}
// A handler further up sees code == 11.

constexpr int checked(int v) { return v < 0 ? throw Error{3} : v; }
static_assert(checked(5) == 5);          // the throw is on an untaken path
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on all
27 tests. EDG runs no programs, so its results for the run and
run-fail tests say only that the programs are valid. A `run-fail` test
shows that the program ended with a nonzero status or a signal; the
harness cannot see that termination was the reason.


Inputs: clang-sema-expr, clang-diag-sema, clang-diag-ast, edg-statements.
