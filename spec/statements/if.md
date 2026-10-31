# The if statement

An `if` statement executes one of two branches, or no branch, as a
condition selects. There are three forms:

- The *ordinary form* tests a condition at run time.
- The *`if constexpr` form* tests a condition during translation. It
  discards the branch that it does not select.
- The *`if consteval` form* tests if the statement executes during
  {term}`constant evaluation`.

## Principles

:::{principle} IF-P1
:rules: IF-1 IF-2 IF-3 IF-4 IF-5 IF-6

An `if` statement asks one question and takes one of two paths. The
question is a condition converted to `bool` by {term}`contextual
conversion`, evaluated one time. The condition can also declare one
variable, whose value is the answer, so that the branches can use what
the question computed. That variable is a plain variable of the
statement, so it has an initializer and no storage class. An `else`
belongs to the nearest `if` that has none, because that is the reading
with no ambiguity.
:::

:::{principle} IF-P2
:rules: IF-7 IF-21 IF-8 IF-9 IF-10

The statement's names follow the general rules of scopes
({principle}`NAM-P1`):

```text
if ( initializer ; condition )    scope of the statement: holds the
                                  names of the initializer and the
                                  condition, and encloses both branches
  first branch                    a scope of its own
else
  second branch                   a scope of its own, which sees the
                                  condition variable too
```

A branch is a scope with or without braces. A branch that redeclares one
of those names at its top level would make the name mean two things in
one place, so it is rejected, while a block inside the branch can hide
it. The initializer can be any declaration that a block can hold, which
excludes the kinds that declare no variable.
:::

:::{principle} IF-P5
:rules: IF-11 IF-12

The statement's objects follow the general rules of lifetime
({principle}`OBJ-P2`). Temporaries of the condition end before a branch
runs, and the variables of the initializer and the condition end with
the statement. A jump into a branch skips the condition, as a jump skips
any statement.
:::

:::{principle} IF-P3
:rules: IF-13 IF-14 IF-15 IF-16 IF-17 IF-22 IF-23

`if constexpr` moves the decision to translation. The condition is a
constant, so the implementation knows which branch is dead and discards
it. For a discarded branch:

- It is still parsed and, outside a template, still checked, because it
  is still part of the program.
- In a template it is not instantiated, so it can hold code that is
  wrong for the template arguments.
- Its `return` statements do not deduce the return type, and nothing it
  names needs a definition, because it never runs.

A `goto`, `case` label, or `default` label outside the statement cannot
jump into either branch, whether it is selected or discarded. The same
holds for `if consteval`.
:::

:::{principle} IF-P4
:rules: IF-18 IF-19 IF-20 IF-24

`if consteval` asks which mode the program is running in
({principle}`TR-P4`):

| Form | Runs in constant evaluation | Runs at run time |
|---|---|---|
| `if consteval` | The first branch | The second branch |
| `if !consteval` | The second branch | The first branch |

The branch that runs in constant evaluation is a place where a
`consteval` call needs no constant arguments, because the whole branch
is already a constant evaluation. The form has no condition, because the
mode is the condition, and its branches are blocks, so that the form is
never mistaken for the ordinary one.
:::

## Syntax

```{eval-rst}
.. productionlist:: if
   if_stmt: "if" ["constexpr"] "(" [`if_init`] `if_cond` ")"
          :    `branch` ["else" `branch`]
          : "if" ["!"] "consteval" `block` ["else" `block`]
   if_init: `expression_stmt` | `simple_declaration` | `alias_declaration` | ";"
   if_cond: `expression` | `cond_decl`
   cond_decl: `decl_specifiers` `declarator` ("=" `expression` | "=" `braced_init` | `braced_init`)
   branch: `statement`
   block: `compound_statement`
```

The *then branch* is the first `branch` or `block`. The *else branch* is
the `branch` or `block` after `else`.

## Constraints

:::{rule} IF-2
:kind: required
:tests: statements/if/dangling-else.cpp

An `else` belongs to the nearest `if` before it that has no `else`.
:::

:::{rule} IF-3
:kind: required
:tests: statements/if/condition-converted-to-bool.cpp statements/if/condition-assignment.cpp

If the condition is an expression, its value is converted to `bool` by
{term}`contextual conversion`. The expression can be any expression,
including an assignment.
:::

:::{rule} IF-4
:kind: reject
:tests: statements/if/condition-not-convertible.cpp

The condition expression must be convertible to `bool` by contextual
conversion.
:::

:::{rule} IF-6
:kind: reject
:tests: statements/if/condition-declaration-no-init.cpp statements/if/condition-declaration-two.cpp statements/if/condition-structured-binding.cpp statements/if/condition-declaration-array.cpp statements/if/condition-declaration-class.cpp statements/if/condition-declaration-static.cpp statements/if/condition-declaration-extern.cpp statements/if/condition-declaration-thread-local.cpp statements/if/condition-declaration-function.cpp statements/if/condition-declaration-inline.cpp statements/if/condition-declaration-typedef.cpp

A condition declaration must:

- have an initializer,
- declare exactly one variable, and not a structured binding or a
  function,
- not declare an array,
- not define a class or an enumeration,
- use no specifiers except type specifiers and `constexpr`. `static`,
  `extern`, `thread_local`, `inline`, and `typedef` are not valid.
:::

:::{rule} IF-7
:kind: required
:tests: statements/if/init-expression.cpp statements/if/init-declaration.cpp statements/if/init-alias.cpp statements/if/init-empty.cpp statements/if/init-declaration-kinds.cpp statements/if/constexpr-with-init.cpp

The ordinary form and the `if constexpr` form can start with an
initializer. The initializer can be:

- an expression statement,
- an empty statement,
- an alias declaration,
- a simple declaration. This includes several variables, a `static`
  variable, a class or enumeration definition, and a function
  declaration.
:::

:::{rule} IF-21
:kind: reject
:tests: statements/if/init-static-assert.cpp statements/if/init-using-declaration.cpp statements/if/init-using-directive.cpp

The initializer must not be a declaration of another kind, such as:

- a `static_assert`,
- a using-declaration,
- a using-directive.
:::

:::{rule} IF-8
:kind: reject
:tests: statements/if/substatement-scope.cpp

Each branch has its own scope. This is also true for a branch without
braces. A name that a branch declares is not visible after the `if`
statement.
:::

:::{rule} IF-9
:kind: required
:tests: statements/if/condition-declaration.cpp statements/if/init-declaration.cpp statements/if/init-scope-ends.cpp statements/if/else-if-scope.cpp

A name that the initializer or the condition declares is visible from its
declaration to the end of the `if` statement. This includes the else
branch. An `else if` is a new `if` statement in the else branch.
:::

:::{rule} IF-10
:kind: reject
:tests: statements/if/redeclare-in-branch.cpp statements/if/redeclare-in-else.cpp statements/if/redeclare-init-in-branch.cpp statements/if/redeclare-init-and-condition.cpp statements/if/redeclare-in-else-statement.cpp statements/if/redeclare-in-nested-block.cpp

A name that the initializer or the condition declares must not be declared
again in these places:

- the condition, for a name from the initializer,
- the outermost block of a branch,
- a branch that is one declaration without braces.

A block inside a branch can declare the name again. That declaration hides
the first one.
:::

:::{rule} IF-13
:kind: required
:tests: statements/if/constexpr-discard-in-template.cpp

In the `if constexpr` form, the branch that the condition does not select
is a {term}`discarded statement`. In a template, a discarded statement is
not instantiated. It can contain code that is not valid for the template
arguments.
:::

:::{rule} IF-22
:kind: reject
:tests: statements/if/constexpr-template-nondependent-checked.cpp

In a template, the parts of a {term}`discarded statement` that are the
same for every template argument must be valid (IF-13). The
implementation checks them.
:::

:::{rule} IF-14
:kind: required
:tests: statements/if/constexpr-discard-return-type.cpp statements/if/constexpr-non-template-return-type.cpp

A `return` statement in a discarded statement does not take part in
return type deduction. This applies in templates and in other functions.
:::

:::{rule} IF-15
:kind: required
:tests: statements/if/constexpr-non-template-no-definition.cpp

A function or variable that only discarded statements use needs no
definition.
:::

:::{rule} IF-23
:kind: reject
:tests: statements/if/constexpr-non-template-checked.cpp

Outside a template, a {term}`discarded statement` must be valid (IF-15).
:::

:::{rule} IF-16
:kind: reject
:tests: statements/if/constexpr-condition-not-constant.cpp statements/if/constexpr-condition-narrowing.cpp statements/if/constexpr-condition-pointer.cpp statements/if/constexpr-condition-explicit-bool.cpp statements/if/constexpr-with-init.cpp

The condition of the `if constexpr` form must be a constant expression
after contextual conversion to `bool`. A conversion that loses
information is permitted.
:::

:::{rule} IF-17
:kind: reject
:tests: statements/if/constexpr-goto-into.cpp statements/if/constexpr-case-into.cpp statements/if/consteval-goto-into.cpp

These must not transfer control from outside an `if constexpr` or
`if consteval` statement into one of its branches:

- a `goto`,
- a `case` label,
- a `default` label.
:::

:::{rule} IF-19
:kind: required
:tests: statements/if/consteval-immediate-context.cpp statements/if/not-consteval-else-immediate.cpp

The branch that executes during constant evaluation is an
{term}`immediate function context`. This is the then branch of
`if consteval` and the else branch of `if !consteval`.
:::

:::{rule} IF-24
:kind: reject
:tests: statements/if/consteval-immediate-outside.cpp

A call to a `consteval` function with arguments that are not constant is
valid only in the branch that IF-19 gives.
:::

:::{rule} IF-20
:kind: reject
:tests: statements/if/consteval-requires-braces.cpp statements/if/consteval-else-requires-braces.cpp statements/if/consteval-no-condition.cpp statements/if/not-constexpr.cpp

In the `if consteval` form:

- each branch must be a compound statement,
- `consteval` must not have a condition in parentheses.

`!` after `if` must be followed by `consteval`.
:::

## Runtime Behavior

:::{rule} IF-1
:kind: required
:tests: statements/if/basic-branches.cpp statements/if/condition-evaluated-once.cpp

The ordinary form evaluates the condition one time. If the value is
`true`, the then branch executes. If not, the else branch executes, if
there is one.
:::

:::{rule} IF-5
:kind: required
:tests: statements/if/condition-declaration.cpp statements/if/condition-declaration-constexpr.cpp statements/if/condition-declaration-function-pointer.cpp statements/if/condition-declaration-equals-braces.cpp

A condition declaration declares and initializes one variable. The value
of the variable, after contextual conversion to `bool`, selects the
branch. The declaration can use `auto` and `constexpr`. Its initializer
can have one of these forms:

- `= e`
- `= {e}`
- `{e}`
:::

:::{rule} IF-11
:kind: required
:tests: statements/if/lifetimes.cpp

Temporary objects that the condition creates are destroyed before a
branch executes. Variables that the initializer or the condition declares
are destroyed when the `if` statement ends. The variable that is declared
last is destroyed first.
:::

:::{rule} IF-12
:kind: required
:tests: statements/if/goto-into-branch.cpp statements/if/case-into-branch.cpp

In the ordinary form, these can transfer control into a branch:

- a `goto`,
- a `case` label of an enclosing `switch`,
- a `default` label of an enclosing `switch`.

The condition is not evaluated in that case. The general rules for jumps
over initialization apply ({rule}`JMP-6`).
:::

:::{rule} IF-18
:kind: required
:tests: statements/if/consteval-branches.cpp statements/if/consteval-in-consteval.cpp statements/if/consteval-plain-function.cpp

The `if consteval` form executes its then branch during constant
evaluation and its else branch at other times. The `if !consteval` form
does the opposite. In a `consteval` function, the then branch of
`if consteval` always executes. In a function that is not `constexpr` or
`consteval`, the then branch never executes.
:::

## Examples

```cpp
if (a)
  if (b) x();
  else y();                    // y() runs if a is true and b is false

if (Widget* w = find(id))      // w is visible in both branches
  w->show();

if (auto it = m.find(k); it != m.end())
  use(it->second);

if (Lock l{m}; Entry e = load())
  use(e);                      // after the branch: ~e, then ~l

template <class T> int size_of(const T& t) {
  if constexpr (requires { t.size(); }) return t.size();
  else return 1;               // valid for T = int
}

constexpr int where() {
  if consteval { return 1; } else { return 2; }
}
static_assert(where() == 1);   // at run time, where() returns 2
```

## Evidence

All rules in this page are `required` or `reject`. All seven compilers in
the matrix agree on all 65 tests. The results are in `tests/RESULTS.md`.
