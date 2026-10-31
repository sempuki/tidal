# Jump statements and labels

The following statements transfer control:

- `break`,
- `continue`,
- `return`,
- `goto`.

A label names a statement that `goto` can transfer control to.

## Principles

:::{principle} JMP-P1
:rules: JMP-1 JMP-2 JMP-5 JMP-7 JMP-8 JMP-14

A jump statement names a place and goes there, and the place must exist
in the same function.

| Statement | Goes to |
|---|---|
| `break` | Past the innermost loop or `switch` |
| `continue` | The end of the innermost loop body |
| `goto` | A label |

A label belongs to the whole function and to no other name space
({principle}`NAM-P4`), so a `goto` can name a label that comes later,
and a label can share a name with a variable. A lambda body is a
different function, so a jump cannot cross the boundary of a lambda body
in either direction.
:::

:::{principle} JMP-P2
:rules: JMP-4 JMP-6 JMP-9 JMP-16

Control that leaves a scope ends the scope's objects, and control that
enters a scope must not skip the creation of an object at the target
({principle}`OBJ-P2`). The following cases result:

| Jump | Result |
|---|---|
| Out of a block | The block's variables are destroyed, in reverse order. `return` also destroys the variables, after `return` has taken its value. |
| Backward past a declaration | The variable is destroyed, and the declaration makes the variable again. |
| Forward past an initialization | Rejected, because the variable would exist without a value |

A type that needs no construction and no destruction has nothing to
skip, so a jump can pass a declaration of such a type when the
declaration has no initializer.
:::

:::{principle} JMP-P3
:rules: JMP-3 JMP-10 JMP-11 JMP-12 JMP-13 JMP-15

`return` hands the caller what the function promised:

- a value in a function that returns one,
- nothing in a `void` function,
- nothing in a constructor or destructor.

A local variable or a parameter that is returned by name is about to
end, so the name is treated as an rvalue and the value is moved out
({principle}`CPY-P4`). The implementation may go further and make a
local variable the result object itself, but never a parameter, whose
storage belongs to the caller.
:::

## Syntax

```{eval-rst}
.. productionlist:: stmt_jumps
   jump_stmt: "break" ";"
            : "continue" ";"
            : "return" [`expression` | `braced_init`] ";"
            : "goto" `identifier` ";"
   labeled_stmt: `identifier` ":" `statement`
```

## Constraints

:::{rule} JMP-7
:kind: reject
:tests: statements/jumps/break-outside.cpp statements/jumps/continue-outside.cpp statements/jumps/break-in-lambda-in-loop.cpp

`break` must be inside a loop or a `switch`. `continue` must be inside a
loop. A lambda body is a separate function, so the loop or `switch` must
be inside the lambda too.
:::

:::{rule} JMP-3
:kind: required
:tests: statements/jumps/return-converts.cpp statements/jumps/return-braced.cpp statements/jumps/return-void-expression.cpp statements/jumps/void-function-return-void-expression.cpp

In a function that returns a value, the value of `return` is converted
to the return type, and a braced list initializes the result. In a
`void` function, `return` has no expression or an expression of type
`void`.
:::

:::{rule} JMP-13
:kind: reject
:tests: statements/jumps/return-missing-value.cpp statements/jumps/return-value-in-void.cpp

In a function that returns a value, `return` must have an expression.
In a `void` function, `return` must not have an expression of a type
other than `void` (JMP-3).
:::

:::{rule} JMP-5
:kind: required
:tests: statements/jumps/label-function-scope.cpp statements/jumps/label-separate-names.cpp

A label is visible in all of its function. A `goto` can name a label
that comes before or after the `goto`. A label name does not conflict
with the names of
variables or types.
:::

:::{rule} JMP-14
:kind: reject
:tests: statements/jumps/duplicate-label.cpp statements/jumps/goto-undefined-label.cpp statements/jumps/goto-other-function.cpp statements/jumps/goto-out-of-lambda.cpp

These conditions apply to labels (JMP-5):

- Each label name must occur only once in a function.
- A `goto` must name a label of its own function. A lambda body is a
  separate function.
:::

:::{rule} JMP-6
:kind: reject
:tests: statements/jumps/goto-past-initialization.cpp statements/jumps/goto-past-class-object.cpp statements/jumps/goto-past-uninitialized-scalar.cpp statements/jumps/goto-past-static.cpp statements/jumps/goto-past-nontrivial-destructor.cpp statements/jumps/goto-past-trivial-class.cpp statements/jumps/goto-skips-nontrivial-destructor.cpp classes/constructors/trivial-default-base-nontrivial.cpp classes/constructors/trivial-default-defaulted.cpp classes/constructors/trivial-default-implicit.cpp classes/constructors/trivial-default-member-initializer.cpp classes/constructors/trivial-default-member-nontrivial.cpp classes/constructors/trivial-default-user-provided.cpp classes/constructors/trivial-default-virtual-base.cpp classes/constructors/trivial-default-virtual-function.cpp classes/constructors/trivial-default-defaulted-later.cpp

A jump must not skip the declaration of a variable with
{term}`automatic storage` that is in scope at the target. This rule does
not apply to a declaration with no initializer whose type is one of
these:

- a scalar type,
- a class type whose default constructor and destructor are both
  trivial ({rule}`CTOR-23`, {rule}`DTOR-9`),
- an array of these types.

A jump can skip the declaration of a `static` local variable.
:::

:::{rule} JMP-16
:kind: required
:tests: statements/jumps/goto-past-declaration-then-write.cpp statements/switch/skipped-declaration-written-then-read.cpp statements/switch/skipped-declaration-read-first.cpp

A variable whose declaration a jump skips (JMP-6) still exists in its
block after the jump. The variable has no value until the program
writes one (OBJ-2).
:::

:::{rule} JMP-10
:kind: required
:tests: statements/jumps/return-move-only-parameter.cpp statements/jumps/return-rvalue-ref-param-as-rvalue-ref.cpp statements/jumps/return-parenthesized-move-only.cpp

If the expression of a `return` statement is the name of a local
variable or a parameter of the function, the name is an xvalue (see
{term}`value category`). A move-only object can be returned this way.
These details apply:

- A variable or parameter of rvalue reference type counts.
- Parentheses around the name do not change the result.
- A `static` local variable does not count.
:::

:::{rule} JMP-15
:kind: reject
:tests: statements/jumps/return-parameter-as-lvalue-ref.cpp statements/jumps/return-static-local-move-only.cpp

A name that JMP-10 makes an xvalue does not bind to an lvalue reference
return type. A `static` local variable of a move-only type cannot be
returned, because JMP-10 does not count such a variable.
:::

:::{rule} JMP-12
:kind: reject
:tests: statements/jumps/constructor-return-void-expression.cpp statements/jumps/destructor-return-void-expression.cpp statements/jumps/destructor-return-value.cpp

In a constructor or a destructor, `return` must have no expression,
including an expression of type `void`.
:::

## Runtime Behavior

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
change this rule.
:::

:::{rule} JMP-4
:kind: required
:tests: statements/jumps/return-destroys-locals.cpp

`return` first evaluates the returned value. Then `return` destroys the
local variables in scope. The variable that was created last is
destroyed first.
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

## Implementation Requirements

:::{rule} JMP-11
:kind: unspecified
:tests: statements/jumps/copy-elision-named.cpp statements/jumps/return-parameter-always-moves.cpp

If a `return` statement names a local variable of the return type, it is
unspecified if a copy or move constructor runs. A returned prvalue of
the return type never runs such a constructor (VC-4). A parameter is not
such a variable. A `return` of a parameter by name runs the move
constructor one time (CPY-12).
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

All eight compilers agree on 49 of 52 tests. The results are in
`tests/RESULTS.md`.

- JMP-6: GCC accepts a `goto` past a variable without an initializer
  whose destructor is not trivial. Clang and EDG reject the `goto`, so a
  class type whose destructor is not trivial is not an exception
  ({question}`Q-0041`, {question}`Q-0097`).

- JMP-11: Clang and GCC elide the copy of a named local at `-O0`. The test
  accepts zero or one copy.

Inputs: clang-parse-stmt, clang-sema-stmt, clang-diag-sema, edg-
statements.
