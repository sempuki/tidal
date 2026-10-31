# The switch statement

A `switch` statement transfers control to one of several labels in its
body. The value of its condition selects the label.

## Principles

:::{principle} SW-P1
:rules: SW-1 SW-2 SW-3

A `switch` is a computed jump. Its condition is an integer or an
enumeration, or a class that converts to exactly one of these types,
evaluated one time. Control jumps to one of the following places:

- the `case` label with that value,
- `default`,
- past the statement.

Control then runs on through the later labels until a `break`.

A case value is a constant of the
condition's promoted type, because the jump table is built at
translation time. Two equal case values or two `default` labels would
give the jump two targets, so such labels are rejected. A label belongs
to the innermost `switch` around it, and a lambda or local class body is
a different function, so a label in such a body belongs to nothing
outside.
:::

:::{principle} SW-P2
:rules: SW-4 SW-5 SW-6

The statement has the scopes of an `if` ({principle}`IF-P2`), with an
initializer and a condition that can declare a variable. Because the
body is entered by a jump, the general rule for jumps applies
({principle}`JMP-P2`). A label must not sit past the initialization of
a variable that is in scope at the label.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   switch_stmt: "switch" "(" [`init_stmt`] `condition` ")" `statement`
```

## Constraints

:::{rule} SW-2
:kind: reject
:tests: statements/switch/condition-types.cpp statements/switch/condition-floating.cpp statements/switch/condition-pointer.cpp statements/switch/condition-two-conversions.cpp statements/switch/condition-explicit-conversion.cpp statements/switch/condition-two-conversions-same-type.cpp

The condition must have one of these types:

- an integer type,
- an enumeration type,
- a class type whose conversion functions that are not `explicit`
  convert to exactly one integer or enumeration type. Several of these
  functions can convert to that one type.
:::

:::{rule} SW-3
:kind: reject
:tests: statements/switch/case-not-constant.cpp statements/switch/case-narrowing.cpp statements/switch/case-duplicate.cpp statements/switch/case-duplicate-after-conversion.cpp statements/switch/two-defaults.cpp statements/switch/case-outside-switch.cpp statements/switch/case-out-of-char-range.cpp statements/switch/scoped-enum-int-case.cpp statements/switch/case-in-lambda-body.cpp statements/switch/case-in-local-class.cpp

Each case value must be a constant expression that converts to the
promoted type of the condition without narrowing. For a scoped
enumeration condition, each case value must have that enumeration type.
A case value that the condition's own type cannot hold is valid and never
matches. In one `switch`:

- two case values must not be equal after conversion,
- there must be at most one `default` label.

A `case` or `default` label must be inside a `switch`. The body of a
lambda or of a member function of a local class is a separate function,
so a label in such a body belongs to no enclosing `switch`.
:::

:::{rule} SW-4
:kind: required
:tests: statements/switch/init-statement.cpp statements/switch/condition-declaration.cpp

A `switch` can have an initializer statement, as in IF-7. The condition
can be a declaration, as in IF-5. The scope rules of IF-9 and IF-10
apply. A name that the initializer or the condition declares is visible
to the end of the statement.
:::

:::{rule} SW-6
:kind: reject
:tests: statements/switch/condition-name-redeclared.cpp statements/switch/init-name-redeclared.cpp

A name that the initializer or the condition of a `switch` declares
must not be declared again in the outermost block of the body (SW-4).
:::

:::{rule} SW-5
:kind: reject
:tests: statements/switch/jump-past-initialization.cpp statements/switch/jump-past-block.cpp statements/switch/jump-past-uninitialized.cpp statements/switch/case-skips-trivial-struct.cpp statements/switch/case-skips-nontrivial-destructor.cpp

A transfer to a `case` or `default` label must not skip the
initialization of a variable that is in scope at the label. A transfer
can skip a whole block, or a declaration without an initializer whose
type JMP-6 permits ({question}`Q-0097`):

- a scalar,
- a class whose default constructor and destructor are both trivial
  ({rule}`CTOR-23`, {rule}`DTOR-9`),
- an array of these types.
:::

## Runtime Behavior

:::{rule} SW-1
:kind: required
:tests: statements/switch/dispatch.cpp statements/switch/no-match-no-default.cpp statements/switch/fallthrough.cpp statements/switch/condition-once.cpp statements/switch/break-exits.cpp statements/switch/case-inside-loop.cpp statements/switch/nested-switch-labels.cpp

The condition is evaluated one time. Control goes to the first of these
that exists:

- the `case` label with the equal value,
- the `default` label,
- the end of the `switch`.

Execution then continues through later labels until a `break`. A label can
be inside a nested statement of the body, such as a loop. Each label
belongs to the innermost `switch` that contains it.
:::

## Examples

```cpp
int pick(int v) {
  int r = 0;
  switch (v) {
    case 0: r += 1; [[fallthrough]];  // continues into case 1
    case 1: r += 10; break;
    default: r = -1; break;
  }
  return r;                           // pick(0) == 11, pick(1) == 10
}

void f(unsigned u, double d) {
  switch (u) { case -1: break; }      // rejected: -1 narrows
  switch (d) { case 1: break; }       // rejected: double condition
}

void g(int v) {
  switch (v) {
    case 1:
      int x = 1;
    case 2:                           // rejected: skips the initialization of x
      break;
  }
}
```

## Evidence

All seven compilers agree on all tests in this page, except where a
question is noted. The results are in `tests/RESULTS.md`.

- SW-5: GCC accepts a `case` label that skips a variable whose destructor
  is not trivial. Clang and EDG reject the label ({question}`Q-0097`).

Inputs: clang-parse-stmt, clang-sema-stmt, clang-diag-sema, edg-
statements.
