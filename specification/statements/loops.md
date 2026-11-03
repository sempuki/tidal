# Iteration statements

The following statements execute a statement again and again while a
condition is true:

- `while`,
- `do`,
- `for`.

## Principles

:::{principle} LOOP-P1
:rules: LOOP-1 LOOP-2 LOOP-3 LOOP-5 LOOP-6

A loop repeats a body while a condition, converted to `bool` by
{term}`contextual conversion`, holds. The three forms differ only in
when the condition is tested and what runs around the body.

| Form | Tests the condition | Runs around the body |
|---|---|---|
| `while` | Before the body | Nothing |
| `do` | After the body, so the body runs at least once | Nothing |
| `for` | Before the body | A step that runs once before the loop, and a step that runs after each body |

The condition of a `do` cannot be a declaration, because a declaration
there would have no place to live. A `for` can leave any part out, and
an absent condition means `true`.
:::

:::{principle} LOOP-P2
:rules: LOOP-4 LOOP-7 LOOP-8

The scopes and {term}`lifetimes <lifetime>` of a loop follow the general rules
({principle}`NAM-P1`, {principle}`OBJ-P2`). The body is a scope, and
each iteration enters the body afresh, so a variable the condition
declares is created and destroyed in each iteration. The initializer of
a `for` runs once, so a variable that the initializer declares lives for
the whole statement. Names from the initializer and condition are
visible to the end of the statement and must not be redeclared at the
top level of the body, as in an `if` ({principle}`IF-P2`). The body of a
`do` ends before its condition, so the condition cannot use the body's
names.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   while_stmt: "while" "(" `condition` ")" `statement`
   do_stmt: "do" `statement` "while" "(" `expression` ")" ";"
   for_stmt: "for" "(" `init_stmt` [`condition`] ";" [`expression`] ")"
           :    `statement`
```

## Constraints

:::{rule} LOOP-4
:kind: required
:tests: statements/loops/for-init-scope.cpp statements/loops/for-condition-declaration.cpp statements/loops/body-own-scope.cpp statements/loops/for-init-alias.cpp

The body of each loop has its own scope, also without braces. In a `for`
or `while` statement:

- the condition can be a declaration,
- a name that the initializer or condition declares is visible to the end
  of the statement only.

The initializer of a `for` statement has the forms of an `if` initializer
(IF-7), including an alias declaration. The body of a `do` statement is
its own scope, and the condition comes after that scope ends.
:::

:::{rule} LOOP-8
:kind: reject
:tests: statements/loops/for-init-redeclare-body.cpp statements/loops/while-condition-redeclared.cpp

In a `for` or `while` statement, a name that the initializer or
condition declares must not be declared again in the outermost block of
the body (LOOP-4).
:::

:::{rule} LOOP-7
:kind: reject
:tests: statements/loops/for-init-redeclared-in-condition.cpp statements/loops/for-init-static-assert.cpp statements/loops/do-body-name-not-in-condition.cpp

These conditions apply:

- In a `for` statement, the condition must not declare a name that the
  initializer declares.
- In a `for` statement, the initializer must not be a declaration that
  IF-21 excludes.
- The condition of a `do` statement must not use a name that the body
  block declares.
:::

:::{rule} LOOP-5
:kind: reject
:tests: statements/loops/condition-not-bool.cpp

The condition of `while`, `do`, and `for` must be convertible to
`bool` by {term}`contextual conversion`.
:::

:::{rule} LOOP-6
:kind: reject
:tests: statements/loops/do-condition-declaration.cpp

The condition of `do` must be an expression.
:::

## Runtime Behavior

:::{rule} LOOP-1
:kind: required
:tests: statements/loops/while-basic.cpp statements/loops/while-condition-declaration.cpp statements/loops/while-condition-destroyed-after-body.cpp

`while` evaluates its condition before each iteration. The body executes
if the value is `true`. A condition declaration is created and destroyed
in each iteration. The variable is destroyed after the objects of the
body.
:::

:::{rule} LOOP-2
:kind: required
:tests: statements/loops/do-basic.cpp

`do` executes its body and then evaluates its condition. The body
executes at least one time.
:::

:::{rule} LOOP-3
:kind: required
:tests: statements/loops/for-order.cpp statements/loops/for-sequence.cpp statements/loops/for-parts-optional.cpp statements/loops/for-condition-destroyed-after-step.cpp statements/loops/for-init-destroyed-once.cpp

`for` executes its initializer one time. In each iteration, `for`:

1. evaluates the condition,
2. executes the body,
3. evaluates the iteration expression.

A variable that the condition declares is destroyed after the
iteration expression. Each part can be absent, and an absent condition is
`true`. A variable that the initializer declares is created one time and
destroyed one time, when the statement ends, also after a `break`.
:::

## Examples

```cpp
int i = 0;
while (i < 3) ++i;            // i is 3

int runs = 0;
do { ++runs; } while (false); // runs is 1

for (int k = 0; k < 2; ++k) {
  int k = 5;                  // rejected: k is declared in the initializer
}

for (;;) { break; }           // no condition: always true

do {} while (int x = 0);      // rejected: do needs an expression
```

## Evidence

The results are in `tests/RESULTS.md`.

- LOOP-1: Clang and GCC destroy the objects of a `while` body, then
  the condition variable, in each iteration.
- LOOP-3: GCC 14 destroys a `for` condition variable before the iteration
  expression. GCC 16 and Clang destroy the variable after. EDG does not
  run the test. During constant evaluation EDG destroys the variable
  before the body ({question}`Q-0044`).

Inputs: clang-parse-stmt, clang-sema-stmt, clang-diag-sema, edg-
statements.
