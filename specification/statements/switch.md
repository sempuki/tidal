# The switch statement

A `switch` statement transfers control to one of several labels in its
body. The value of its condition selects the label.

## Principles

:::{principle} SW-P1
:rules: SW-1 SW-2 SW-3 SW-7 SW-8

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

A `switch` over an enumeration usually means to handle each enumerator.
So an implementation should warn when an enumerator has no `case` label
and the `switch` has no `default`. A case value that no enumerator has
is likely a mistake, so an implementation should warn about it too.
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
  functions can convert to that one type, and overload resolution
  chooses one of them ({rule}`CVF-4`).
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
:tests: statements/switch/jump-past-initialization.cpp statements/switch/jump-past-block.cpp statements/switch/block-holds-case-declaration.cpp statements/switch/jump-past-uninitialized.cpp statements/switch/case-skips-trivial-struct.cpp statements/switch/case-skips-nontrivial-destructor.cpp

A transfer to a `case` or `default` label must not skip the
initialization of a variable that is in scope at the label. A transfer
can skip a whole block, or a declaration without an initializer whose
type {rule}`JMP-6` lists ({question}`Q-0041`).
:::

## Runtime Behavior

:::{rule} SW-1
:kind: required
:tests: statements/switch/dispatch.cpp statements/switch/no-match-no-default.cpp statements/switch/fallthrough.cpp statements/switch/condition-once.cpp statements/switch/break-exits.cpp statements/switch/case-inside-loop.cpp statements/switch/nested-switch-labels.cpp statements/switch/enum-value-outside-cases.cpp

The condition is evaluated one time. Control goes to the first of these
that exists:

- the `case` label with the equal value,
- the `default` label,
- the end of the `switch`.

Execution then continues through later labels until a `break`. A label can
be inside a nested statement of the body, such as a loop. Each label
belongs to the innermost `switch` that contains it.
:::

## Recommended Practice

:::{rule} SW-7
:kind: recommended
:tests: statements/switch/enum-missing-case-warns.cpp statements/switch/enum-all-cases-or-default-silent.cpp statements/switch/enum-class-condition-warns.cpp statements/switch/enum-shared-value-covered-silent.cpp statements/switch/enum-template-specialization-warns.cpp statements/switch/enum-template-not-specialized-silent.cpp

An implementation should warn about a `switch` if all of these
conditions are true:

- Before promotion, the condition has an enumeration type, or it has a
  class type that converts to an enumeration type (SW-2).
- The `switch` has no `default` label.
- An enumerator of the type has no `case` label with its value.

In a template, the check applies to each {term}`specialization`.
:::

:::{rule} SW-8
:kind: recommended
:tests: statements/switch/case-value-not-enumerator-warns.cpp statements/switch/case-value-not-enumerator-scoped-warns.cpp

An implementation should warn about a case value in a `switch` whose
condition has an enumeration type, as in SW-7, if no enumerator of the
type has that value.
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

struct Code {
  operator int() const { return 1; }
  operator int() { return 2; }
};
void choose(Code c, const Code fixed) {
  switch (c) { case 2: break; }       // calls the non-const function
  switch (fixed) { case 1: break; }   // calls the const function
}

void g(int v) {
  switch (v) {
    case 1:
      int x = 1;
    case 2:                           // rejected: skips the initialization of x
      break;
  }
}

void g2(int v) {
  switch (v) {
    case 1: {
      int x = 1;                      // valid: the block ends before case 2
      use(x);
      break;
    }
    case 2:
      break;
  }
}

enum Light { red, amber, green };
void h(Light l) {
  switch (l) { case red: break; }     // should warn: no case for amber or green
}

enum class State { idle, busy };
int code(State s) {
  switch (s) {                        // no default, so SW-7 still checks
    case State::idle: return 0;
    case State::busy: return 1;
  }
  return -1;                          // State(7) matches no case (SW-1)
}
```

## Evidence

All eight compilers agree on 41 of 42 tests. The results are in
`tests/RESULTS.md`.

- SW-7: Clang warns with no flag and GCC warns under `-Wall`
  (`-Wswitch`). Both name each enumerator without a label. Both warn
  for an unscoped enumeration, whose promoted type is an integer type,
  and for a class condition that converts to an enumeration. A case
  for one of two enumerators with the same value covers both. In a
  function template, both warn at the specialization for an
  enumeration, and neither warns for a template with no specialization.
- SW-8: Clang warns with no flag and GCC warns under `-Wall`
  (`-Wswitch`) about a case value that no enumerator has, for a scoped
  and an unscoped enumeration. The tests
  use `-Wall -Werror`, so EDG skips them
  ({ref}`baseline compilers <baseline-compilers>`). EDG gives no
  warning, even with remarks on. The warning is recommended practice,
  so the rule is in the recommended class.
- SW-2, SW-3: Clang 18 and Clang 22 warn with no flag about a `bool`
  condition (`-Wswitch-bool`). GCC 14 and GCC 16 warn with no flag
  about a case value that the condition's type cannot hold
  (`-Wswitch-outside-range`). Clang gives no warning for that case
  value. EDG gives neither warning.
- SW-1, FN-3: a `switch` with a `return` for every enumerator can
  still reach its end, because a scoped enumeration holds every value
  of its underlying type ({principle}`ENUM-P3`). Without the last
  `return`, GCC warns under `-Wall` (`-Wreturn-type`) and Clang gives
  no warning. A `return` after the `switch` keeps the SW-7 warning,
  which a `default` label would turn off.
- SW-5: GCC accepts a `case` label that skips a variable whose class
  has a trivial default constructor and a destructor that is not
  trivial. Clang and EDG reject the label. SW-5 follows JMP-6 and GCC
  ({question}`Q-0041`).

Inputs: clang-parse-stmt, clang-sema-stmt, clang-diag-sema, edg-
statements.
