# Throw expressions and the exception object

A `throw` expression leaves the current point of execution with a value.
The value becomes the exception object, which lives until the last
handler for it finishes. This page covers:

- the operand of `throw`,
- how the exception object is made from the operand,
- the type and lifetime of the exception object,
- `throw` with no operand,
- `throw` during constant evaluation.

Depends on: value categories and temporaries, copy and move, constant
evaluation, objects and lifetime.

## Principles

:::{principle} THR-P1
:rules: THR-1 THR-2 THR-3

A `throw` makes the {term}`exception object` from its operand the way a
declaration makes a variable from an initializer.

| Operand | Exception object |
|---|---|
| A prvalue | The operand becomes the exception object directly, so no copy or move happens and none needs to exist ({principle}`VC-P2`) |
| A local variable or parameter that every try block around the `throw` also contains | Moved from the operand, because the variable ends before any {term}`handler` of the function can run |
| Any other lvalue | Copied from the operand |

A local variable that a try block around the `throw` does not contain
can still be in use in a handler of that try block, so the `throw`
copies the variable. The copy or move belongs to the `throw`. An
exception from the copy or move leaves the `throw` like any other
exception.

Arrays and functions decay to pointers, and {term}`cv-qualifiers <cv-qualifier>` are dropped,
because the exception object is a fresh object of its own. So the
operand's type must be complete and not abstract, and the copy or move
that would run must be usable, as must the destructor that ends the
exception object.
:::

:::{principle} THR-P2
:rules: THR-4 THR-6 THR-8 THR-12 THR-13

The exception object belongs to no scope ({principle}`OBJ-P2` does not
apply to the exception object). The object:

- begins at the `throw`,
- outlives every block that the exception leaves,
- ends when the last handler for it ends.

A destructor of the exception object that throws at that end starts a
new exception at the end of the handler. The implementation provides the
storage of the exception object by its own means, so a program's
allocation functions never see the exception object.

The type of the exception object is fixed at the `throw` as the static
type of the operand, so a derived object thrown through a base reference
is sliced. A rethrow passes the same object on, with any changes a
handler made to the object, and a handler that takes the object by value
works on a copy.
:::

:::{principle} THR-P3
:rules: THR-5

A `throw` is an expression that never produces a value, so its type is
`void`. A `throw` can stand wherever no value is needed, including as an
operand of the conditional operator, where the `throw` takes no part in
the result type (COND-4).
:::

:::{principle} THR-P4
:rules: THR-7 THR-9

A `throw` with no operand passes on the {term}`handled exception`. When
no such exception exists, or when an exception leaves
a function that promised not to throw, no handler can take over, and the
program terminates instead of continuing in an unknown state. An
implementation should warn where it can see such a termination happen.
:::

:::{principle} THR-P5
:rules: THR-10

During {term}`constant evaluation` a `throw` has nowhere to go, so an evaluation
that reaches one is not a constant and the program is rejected there
({principle}`TR-P4`). A `throw` on a path the evaluation does not take
costs nothing, and the same function throws normally when called at run
time.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   throw_expr: "throw" [`assignment_expr`]
```

A `throw` with no operand is a *rethrow*.

## Constraints

:::{rule} THR-3
:kind: reject
:tests: exceptions/throw/deleted-copy-lvalue-rejected.cpp exceptions/throw/private-copy-rejected.cpp exceptions/throw/abstract-operand-rejected.cpp exceptions/throw/incomplete-operand-rejected.cpp exceptions/throw/private-destructor-rejected.cpp exceptions/throw/throwing-destructor-accepted.cpp

The operand of `throw` must have a complete type that is not abstract.
If the exception object is copied or moved from the operand, the copy or
move constructor must be usable and accessible, and so must the
destructor of the exception object. A destructor that is potentially
throwing is permitted.
:::

:::{rule} THR-5
:kind: required
:tests: exceptions/throw/throw-expression-type-void.cpp

A `throw` expression is a prvalue of type `void`. A `throw` expression
can be an operand of the conditional operator (COND-4).
:::

:::{rule} THR-10
:kind: required
:tests: exceptions/throw/constexpr-throw-at-runtime.cpp

A `constexpr` function that contains a `throw` can be called at run
time, and the call throws then. Constant evaluation of the `throw`
follows {rule}`CE-8`.
:::

## Runtime Behavior

:::{rule} THR-1
:kind: required
:tests: exceptions/throw/prvalue-initializes-directly.cpp exceptions/throw/deleted-copy-prvalue.cpp exceptions/throw/global-lvalue-copied.cpp exceptions/throw/local-variable-moved.cpp exceptions/throw/local-outside-try-copied.cpp exceptions/throw/local-outside-inner-try-copied.cpp exceptions/throw/parameter-moved.cpp exceptions/throw/parameter-in-try-copied.cpp exceptions/throw/copy-into-exception-object-throws.cpp

A `throw` initializes the exception object from its operand:

- A prvalue operand initializes the exception object directly, with no
  copy or move.
- An operand that names a local variable or a parameter is moved if
  each try block around the `throw` in the same function also contains
  the declaration of the variable.
- Any other lvalue operand is copied.

If the copy or move throws, the new exception leaves the `throw`, and a
handler can catch the new exception. The program does not end.
{rule}`TRY-18` gives the copy into a handler parameter, which ends the
program.
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
type does not match the exception object.
:::

:::{rule} THR-6
:kind: required
:tests: exceptions/throw/rethrow-same-object.cpp exceptions/throw/rethrow-keeps-changes.cpp exceptions/throw/rethrow-in-handler-accepted.cpp

A rethrow passes on the {term}`handled exception`. The rethrown object
is the same exception object, with any changes that a handler made to
it. {rule}`TRY-8` gives where the rethrown exception goes.
:::

:::{rule} THR-7
:kind: required
:tests: exceptions/throw/rethrow-outside-handler-terminates.cpp exceptions/throw/rethrow-after-handler-left-terminates.cpp

A rethrow when no {term}`handled exception` exists terminates the
program ({rule}`UNW-5`). This rule includes a rethrow after the handler
of the exception has ended.
:::

:::{rule} THR-8
:kind: required
:tests: exceptions/throw/object-lives-through-handler.cpp exceptions/throw/object-lives-through-rethrow.cpp exceptions/throw/catch-by-value-copies.cpp exceptions/throw/new-throw-in-handler-ends-old.cpp exceptions/throw/new-throw-in-handler-order.cpp

The exception object lives until the last handler for it ends, through
rethrows. A handler that takes the exception by value gets a copy, and
the exception object still lives until the handler ends. A new exception
thrown from a handler ends the handler. The following happen in this
order:

1. the local objects and the parameter of the handler end
   ({rule}`TRY-15`),
2. the old exception object ends,
3. a handler for the new exception starts.
:::

:::{rule} THR-9
:kind: required
:tests: exceptions/throw/throw-in-noexcept-terminates.cpp

If an exception leaves a non-throwing function, the program terminates.
This includes a function that is non-throwing without a written
specifier ({rule}`EXS-4`). No handler in a caller catches the exception.
:::

:::{rule} THR-12
:kind: required
:tests: exceptions/throw/exception-object-destructor-throws.cpp

If the destructor of the exception object throws when the last handler
for the object ends, the new exception leaves from the end of that
handler. A try block that encloses the handler's try block can catch the
new exception.
:::

:::{rule} THR-13
:kind: required
:tests: exceptions/throw/storage-not-from-operator-new.cpp

The implementation chooses where the storage of the exception object
comes from. A `throw` gets that storage without a call to an allocation
function, also when the program replaces the global `operator new` or
the class declares its own ({rule}`NEW-13`).
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

The results are in `tests/RESULTS.md`. All eight compilers agree on all
35 tests. EDG runs no programs, so its results for the run and run-fail
tests say only that the programs are valid. A `run-fail` test shows that
the program ended with a nonzero status or a signal; the harness cannot
see that termination was the reason.

- THR-1: Clang and GCC agree on each case of the operand that names a
  variable. The variable is moved when the variable lives inside every
  try block around the `throw`. The variable is copied when it is
  declared outside a try block around the `throw`, outside the
  innermost of two nested try blocks, or as a parameter of a function
  whose body or function try block encloses the `throw`.
- THR-1: when the copy constructor that makes the exception object
  throws an `int`, Clang and GCC propagate the `int` to a handler for
  `int`. Neither compiler ends the program.
- THR-8: Clang and GCC end the by-value handler parameter, then the
  old exception object, and then start the outer handler.
- THR-12: Clang and GCC propagate the exception from the destructor of
  the exception object to the enclosing try block. No compiler ends the
  program there.

Inputs: clang-sema-expr, clang-diag-sema, clang-diag-ast, edg-statements.
