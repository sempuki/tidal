# The switch statement

A `switch` statement transfers control to one of several labels in its
body. The value of its condition selects the label.

## Syntax

```{eval-rst}
.. productionlist:: stmt_switch
   switch_stmt: "switch" "(" [`init_stmt`] `condition` ")" `statement`
   case_label: "case" `constant_expression` ":"
   default_label: "default" ":"
```

## Legality Rules

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
:tests: statements/switch/case-not-constant.cpp statements/switch/case-narrowing.cpp statements/switch/case-duplicate.cpp statements/switch/case-duplicate-after-conversion.cpp statements/switch/two-defaults.cpp statements/switch/case-outside-switch.cpp statements/switch/case-out-of-char-range.cpp statements/switch/scoped-enum-int-case.cpp

Each case value must be a constant expression that converts to the
promoted type of the condition without narrowing. For a scoped
enumeration condition, each case value must have that enumeration type.
A case value that the condition's own type cannot hold is valid and never
matches. In one `switch`:

- two case values must not be equal after conversion,
- there must be at most one `default` label.

A `case` or `default` label must be inside a `switch`.
:::

:::{rule} SW-4
:kind: required
:tests: statements/switch/init-statement.cpp statements/switch/condition-declaration.cpp

A `switch` can have an initializer statement, as in IF-7. Its condition
can be a declaration, as in IF-5.
:::

:::{rule} SW-5
:kind: reject
:tests: statements/switch/jump-past-initialization.cpp statements/switch/jump-past-block.cpp statements/switch/jump-past-uninitialized.cpp

A transfer to a `case` or `default` label must not skip the
initialization of a variable that is in scope at the label. A transfer can
skip a whole block, or a scalar variable that has no initializer.
:::

## Dynamic Semantics

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

All seven compilers agree on all tests in this page. The results are in
`tests/RESULTS.md`.

Inputs: clang-parse-stmt, clang-sema-stmt, clang-diag-sema, gcc-cp-parser,
gcc-cp-semantics, edg-statements.
