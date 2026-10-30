# Jump statements and labels

The `break`, `continue`, `return`, and `goto` statements transfer
control. A label names a statement that `goto` can transfer control to.

## Syntax

```{eval-rst}
.. productionlist:: stmt_jumps
   jump_stmt: "break" ";"
            : "continue" ";"
            : "return" [`expression` | `braced_init`] ";"
            : "goto" `identifier` ";"
   labeled_stmt: `identifier` ":" `statement`
```

## Legality Rules

:::{rule} JMP-7
:kind: reject
:tests: statements/jumps/break-outside.cpp statements/jumps/continue-outside.cpp

`break` must be inside a loop or a `switch`. `continue` must be inside a
loop.
:::

:::{rule} JMP-3
:kind: required
:tests: statements/jumps/return-converts.cpp statements/jumps/return-braced.cpp statements/jumps/return-missing-value.cpp statements/jumps/return-value-in-void.cpp statements/jumps/return-void-expression.cpp

In a function that returns a value, `return` must have an expression. The
value is converted to the return type, and a braced list initializes the
result. In a `void` function, `return` must have no expression or an
expression of type `void`.
:::

:::{rule} JMP-5
:kind: required
:tests: statements/jumps/label-function-scope.cpp statements/jumps/label-separate-names.cpp statements/jumps/duplicate-label.cpp statements/jumps/goto-undefined-label.cpp statements/jumps/goto-other-function.cpp

A label is visible in all of its function. A `goto` can name a label
before it or after it. These conditions apply:

- Each label name must occur only once in a function.
- A `goto` must name a label of its own function.
- A label name does not conflict with the names of variables or types.
:::

:::{rule} JMP-6
:kind: reject
:tests: statements/jumps/goto-past-initialization.cpp statements/jumps/goto-past-class-object.cpp statements/jumps/goto-past-uninitialized-scalar.cpp statements/jumps/goto-past-static.cpp

A jump must not skip the initialization of a variable with
{term}`automatic storage` that is in scope at the target. A jump can skip
a scalar variable that has no initializer. A jump can also skip a
`static` local variable.
:::

## Dynamic Semantics

:::{rule} JMP-1
:kind: required
:tests: statements/jumps/break-innermost.cpp

`break` ends the innermost loop or `switch` that contains it.
:::

:::{rule} JMP-2
:kind: required
:tests: statements/jumps/continue-for.cpp statements/jumps/continue-while-condition.cpp statements/jumps/continue-in-switch.cpp

`continue` goes to the end of the body of the innermost loop that contains
it. The loop then evaluates its iteration expression, if any, and its
condition. A `switch` between the `continue` and the loop does not
change this.
:::

:::{rule} JMP-4
:kind: required
:tests: statements/jumps/return-destroys-locals.cpp

`return` first evaluates the returned value. Then it destroys the local
variables in scope. The variable that was created last is destroyed first.
:::

:::{rule} JMP-8
:kind: required
:tests: statements/jumps/goto-forward-backward.cpp

`goto` transfers control to the statement that has the label.
:::

:::{rule} JMP-9
:kind: required
:tests: statements/jumps/goto-out-destroys.cpp statements/jumps/goto-back-recreates.cpp

A jump out of a block destroys the variables of the block. A jump back to
a point before a declaration destroys the variable. The declaration then
creates the variable again.
:::

## Examples

```cpp
for (int i = 0; i < 3; ++i) {
  for (int j = 0; j < 3; ++j) {
    if (j == 1) break;           // ends the inner loop only
  }
}

for (int i = 0; i < 4; ++i) {
  if (i % 2) continue;           // next: ++i, then i < 4
  sum += i;
}

int f() { return 3.9; }          // returns 3
void g() { return; }

again:
  if (++n < 3) goto again;       // backward jump

goto skip;
int x = 1;                       // rejected: the jump skips this
skip:
```

## Evidence

All seven compilers agree on every test. The results are in
`tests/RESULTS.md`.

Inputs: clang-parse-stmt, clang-sema-stmt, clang-diag-sema, gcc-cp-parser,
gcc-cp-semantics, edg-statements.
