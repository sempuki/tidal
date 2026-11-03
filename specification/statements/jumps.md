# Jump statements and labels

The following statements transfer control:

- `break`,
- `continue`,
- `return`,
- `goto`.

A label names a statement that `goto` can transfer control to.

## Principles

:::{principle} JMP-P1
:rules: JMP-1 JMP-2 JMP-5 JMP-7 JMP-8 JMP-14 JMP-17

A jump statement names a place and goes there, and the place must exist
in the same function.

| Statement | Goes to |
|---|---|
| `break` | Past the innermost loop or `switch` |
| `continue` | The end of the innermost loop body |
| `goto` | A label |

The loops are `while`, `do`, `for`, and the range-based `for`
({doc}`/statements/range-for`).

`break` and `continue` take no label, so each one acts on one loop. A
program leaves two or more loops at once in one of these ways:

- `goto` a label after the outer loop, under the limits of JMP-6,
- `return` from a function that holds the loops,
- a flag that every loop condition tests.

A `constexpr` function can contain `goto`, but {term}`constant evaluation`
rejects a `goto` that it reaches ({rule}`CE-17`).

A label belongs to the whole function and to no other name space
({principle}`NAM-P4`), so a `goto` can name a label that comes later,
and a label can share a name with a variable. A lambda body is a
different function, so a jump cannot cross the boundary of a lambda body
in either direction.

Some regions limit the jumps that enter or leave them:

| Region | Entry | Exit | Jumps | Rule |
|---|---|---|---|---|
| A lambda body | not valid | not valid | `break`, `continue`, `goto` | JMP-7, JMP-14 |
| A try block or a {term}`handler` | not valid | valid | `goto`, `case`, `default` | {rule}`TRY-7` |
| A branch of `if constexpr` or `if consteval` | not valid | valid | `goto`, `case`, `default` | {rule}`IF-17` |
| A handler of a constructor's function try block | | no `return` | `return` | {rule}`TRY-17` |
| A constructor or destructor | | no `return` with a value | `return` | JMP-12 |
| A scope with a variable that has an initializer | not valid past the initialization | valid | `goto`, `case`, `default` | JMP-6, {rule}`SW-5` |
:::

:::{principle} JMP-P2
:rules: JMP-4 JMP-6 JMP-9 JMP-16

Control that leaves a scope ends the scope's objects, and control that
enters a scope must not skip the creation of an object at the target
({principle}`OBJ-P2`). The following cases result:

| Jump | Result |
|---|---|
| Out of a block | The block's variables are destroyed in the order of {rule}`OBJ-3`. `return` also destroys the variables, after `return` has taken its value. |
| Backward past a declaration | The variable is destroyed, and the declaration makes the variable again. |
| Forward past an initialization | Rejected, because the variable would exist without a value |

A declaration with no initializer whose {term}`default initialization` does
nothing has nothing to skip, so a jump can pass it. The variable still
exists after the jump, and the end of the block destroys it as usual.
:::

:::{principle} JMP-P3
:rules: JMP-3 JMP-10 JMP-11 JMP-12 JMP-13 JMP-15

`return` hands the caller what the function promised:

- a value in a function that returns one,
- nothing in a `void` function,
- nothing in a constructor or destructor.

A local variable or a parameter that is returned by name is about to
end, so the name is treated as an rvalue and the value is moved out
({principle}`CPY-P4`). The implementation can go further and make a
local variable the result object itself, but never a parameter, whose
storage belongs to the caller.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   jump_stmt: "break" ";"
            : | "continue" ";"
            : | "return" [`expression` | `braced_init`] ";"
            : | "goto" `identifier` ";"
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

- a {term}`scalar type`,
- a class type whose {term}`default constructor` is trivial ({rule}`CTOR-23`),
- an array of these types.

The destructor of the class plays no part ({question}`Q-0041`).

A jump can skip the declaration of a `static` local variable.
:::

:::{rule} JMP-16
:kind: required
:tests: statements/jumps/goto-past-declaration-then-write.cpp statements/switch/skipped-declaration-written-then-read.cpp statements/switch/skipped-declaration-read-first.cpp

A variable whose declaration a jump skips (JMP-6) still exists in its
block after the jump. The variable has no value until the program
writes one. A read before the write is undefined ({rule}`OBJ-2`), and
constant evaluation rejects the read.
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
returned, because JMP-10 does not count such a variable. A class with a
user-declared move constructor is move-only ({rule}`CPY-4`).
:::

:::{rule} JMP-12
:kind: reject
:tests: statements/jumps/constructor-return-void-expression.cpp statements/jumps/destructor-return-void-expression.cpp statements/jumps/destructor-return-value.cpp

In a constructor or a destructor, `return` must have no expression,
including an expression of type `void`.
:::

:::{rule} JMP-17
:kind: reject
:tests: statements/jumps/break-with-label.cpp statements/jumps/continue-with-label.cpp

A `break` or `continue` statement must not name a label. The grammar
has no place for a label after either keyword ({rule}`TR-12`). The
statement always acts on the innermost loop or `switch` (JMP-1, JMP-2).
:::

## Runtime Behavior

:::{rule} JMP-1
:kind: required
:tests: statements/jumps/break-innermost.cpp statements/jumps/break-range-for.cpp statements/jumps/leave-two-loops.cpp

`break` ends the innermost loop or `switch` that contains it.
:::

:::{rule} JMP-2
:kind: required
:tests: statements/jumps/continue-for.cpp statements/jumps/continue-while-condition.cpp statements/jumps/continue-in-switch.cpp statements/jumps/continue-range-for.cpp statements/jumps/continue-ends-blocks-before-step.cpp

`continue` goes to the end of the body of the innermost loop that contains
it. The objects of each block that `continue` leaves, the body
included, end first ({rule}`OBJ-3`). The loop then evaluates its
iteration expression, if any, and its condition. A range-based `for` then advances to the next element
({principle}`RFOR-P1`). A `switch` between the `continue` and the loop does not
change this rule.
:::

:::{rule} JMP-4
:kind: required
:tests: statements/jumps/return-destroys-locals.cpp

`return` first evaluates the returned value. Then `return` destroys the
local variables in scope, in the order of {rule}`OBJ-3`.
:::

:::{rule} JMP-8
:kind: required
:tests: statements/jumps/goto-forward-backward.cpp statements/jumps/leave-two-loops.cpp

`goto` transfers control to the statement that has the label.
:::

:::{rule} JMP-9
:kind: required
:tests: statements/jumps/goto-out-destroys.cpp statements/jumps/goto-back-recreates.cpp

A jump out of a block destroys the variables of the block
({rule}`OBJ-3`). A jump back to
a point before a declaration destroys the variable. The declaration then
creates the variable again.
:::

## Implementation Requirements

:::{rule} JMP-11
:kind: unspecified
:tests: statements/jumps/copy-elision-named.cpp statements/jumps/return-parameter-always-moves.cpp statements/jumps/elided-local-changed-by-destructor.cpp

If a `return` statement names a local variable of the return type, it is
unspecified if a copy or move constructor runs. Without the copy, the
local variable is the result object. A destructor that runs after the
`return` and changes the local variable then changes the result. A returned prvalue of
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

// Three ways to leave two loops when a cell matches.
bool find_goto(int target) {
  for (int r = 0; r < 3; ++r)
    for (int c = 0; c < 3; ++c)
      if (grid[r][c] == target) goto found;
  return false;
found:                           // JMP-6: no declaration skipped
  return true;
}

bool find_return(int target) {
  for (int r = 0; r < 3; ++r)
    for (int c = 0; c < 3; ++c)
      if (grid[r][c] == target) return true;   // ends both loops
  return false;
}

bool find_flag(int target) {
  bool found = false;
  for (int r = 0; r < 3 && !found; ++r)
    for (int c = 0; c < 3 && !found; ++c)
      found = grid[r][c] == target;
  return found;
}

for (int i = 0; i < 4; ++i) {
  if (i % 2) continue;           // next: ++i, then i < 4
  sum += i;
}

struct Result { int v = 1; };
struct Spoiler { Result* r; ~Spoiler() { r->v = 99; } };
Result make() {
  Result r;
  Spoiler s{&r};                 // ends after the return takes its value
  return r;                      // v is 1 with the copy, 99 without it
}

int f() { return 3.9; }          // returns 3
void g() { return; }

again:
  if (++n < 3) goto again;       // backward jump

goto skip;
int x = 1;                       // rejected: the jump skips this
skip:

int pick(int v) {
  switch (v) {
    int x;                       // valid to skip: no initializer
  case 1:
    x = 5;
    return x;                    // 5
  case 2:
    return x;                    // undefined: x has no value (JMP-16)
  }
  return 0;
}
```

## Evidence

All eight compilers agree on 56 of 59 tests. The results are in
`tests/RESULTS.md`.

- JMP-6: GCC accepts a `goto` past a variable without an initializer
  whose class has a trivial default constructor and a destructor that
  is not trivial. Clang and EDG reject the `goto`. JMP-6 follows GCC
  and the class-properties inventory (item OBS-jump-vacuous), which
  name only the default constructor ({question}`Q-0041`).

- JMP-17: GCC rejects `break outer;` as a missing `;`. Clang names the
  form "named break" and accepts it only in a later C mode.

- JMP-16: Clang accepts a write to a skipped variable during constant
  evaluation and a read after the write. GCC and EDG reject the write
  ({question}`Q-0121`). Every compiler rejects a read before the write
  during constant evaluation.

- JMP-11: Clang and GCC elide the copy of a named local at `-O0`. The test
  accepts zero or one copy. When the function has one `return` of the
  local, a later destructor's change to the local reaches the caller on
  both compilers. When two `return` statements name different locals,
  both compilers copy, and the change does not reach the caller. For a
  scalar, the value is always taken before the destructors run.
- JMP-2: Clang and GCC end the objects of the inner block, then those of
  the body, and then evaluate the iteration expression.

Inputs: clang-parse-stmt, clang-sema-stmt, clang-diag-sema, edg-
statements.
