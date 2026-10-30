# Iteration statements

The `while`, `do`, and `for` statements execute a statement again and
again while a condition is true.

## Syntax

```{eval-rst}
.. productionlist:: stmt_loops
   while_stmt: "while" "(" `condition` ")" `statement`
   do_stmt: "do" `statement` "while" "(" `expression` ")" ";"
   for_stmt: "for" "(" `init_stmt` [`condition`] ";" [`expression`] ")"
           :    `statement`
```

## Legality Rules

:::{rule} LOOP-4
:kind: required
:tests: statements/loops/for-init-scope.cpp statements/loops/for-init-redeclare-body.cpp statements/loops/for-condition-declaration.cpp statements/loops/body-own-scope.cpp

The body of each loop has its own scope, also without braces. In a `for`
statement:

- the condition can be a declaration,
- a name that the initializer or condition declares is visible to the end
  of the `for` statement only,
- that name must not be declared again in the outermost block of the body.
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

## Dynamic Semantics

:::{rule} LOOP-1
:kind: required
:tests: statements/loops/while-basic.cpp statements/loops/while-condition-declaration.cpp

`while` evaluates its condition before each iteration. The body executes
if the value is `true`. A condition declaration is created and destroyed
in each iteration.
:::

:::{rule} LOOP-2
:kind: required
:tests: statements/loops/do-basic.cpp

`do` executes its body and then evaluates its condition. The body
executes at least one time.
:::

:::{rule} LOOP-3
:kind: required
:tests: statements/loops/for-order.cpp statements/loops/for-sequence.cpp statements/loops/for-parts-optional.cpp

`for` executes its initializer one time. Each iteration evaluates the
condition, executes the body, and then evaluates the iteration expression.
Each part can be absent, and an absent condition is `true`.
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

All seven compilers agree on all tests in this page. The results are in
`tests/RESULTS.md`.

Inputs: clang-parse-stmt, clang-sema-stmt, clang-diag-sema, gcc-cp-parser,
gcc-cp-semantics, edg-statements.
