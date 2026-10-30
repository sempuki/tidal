# The if statement

An `if` statement executes one of two branches, or no branch, as a
condition selects. There are three forms:

- The *ordinary form* tests a condition at run time.
- The *`if constexpr` form* tests a condition during translation. It
  discards the branch that it does not select.
- The *`if consteval` form* tests if the statement executes during
  {term}`constant evaluation`.

## Syntax

```{eval-rst}
.. productionlist:: if
   if_stmt: "if" ["constexpr"] "(" [`if_init`] `if_cond` ")"
          :    `branch` ["else" `branch`]
          : "if" ["!"] "consteval" `block` ["else" `block`]
   if_init: `expression_stmt` | `simple_declaration` | `alias_declaration` | ";"
   if_cond: `expression` | `cond_decl`
   cond_decl: `decl_specifiers` `declarator` ("=" `expression` | `braced_init`)
   branch: `statement`
   block: `compound_statement`
```

The *then branch* is the first `branch` or `block`. The *else branch* is
the `branch` or `block` after `else`.

## Legality Rules

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
- a declaration that a block can contain. This includes several
  variables, a `static` variable, a class definition, and a function
  declaration.
:::

:::{rule} IF-8
:kind: required
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
:tests: statements/if/constexpr-discard-in-template.cpp statements/if/constexpr-template-nondependent-checked.cpp

In the `if constexpr` form, the branch that the condition does not select
is a {term}`discarded statement`. In a template, a discarded statement is
not instantiated. It can contain code that is not valid for the template
arguments. The implementation still checks the parts of it that are the
same for every template argument.
:::

:::{rule} IF-14
:kind: required
:tests: statements/if/constexpr-discard-return-type.cpp statements/if/constexpr-non-template-return-type.cpp

A `return` statement in a discarded statement does not take part in
return type deduction. This applies in templates and in other functions.
:::

:::{rule} IF-15
:kind: required
:tests: statements/if/constexpr-non-template-checked.cpp statements/if/constexpr-non-template-no-definition.cpp

Outside a template, a discarded statement must be valid. A function or
variable that only discarded statements use needs no definition.
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

A `goto`, `case` label, or `default` label must not transfer control from
outside an `if constexpr` or `if consteval` statement into one of its
branches.
:::

:::{rule} IF-19
:kind: required
:tests: statements/if/consteval-immediate-context.cpp statements/if/not-consteval-else-immediate.cpp statements/if/consteval-immediate-outside.cpp

The branch that executes during constant evaluation is an
{term}`immediate function context`. This is the then branch of
`if consteval` and the else branch of `if !consteval`. A call to a
`consteval` function with arguments that are not constant is valid only
in that branch.
:::

:::{rule} IF-20
:kind: reject
:tests: statements/if/consteval-requires-braces.cpp statements/if/consteval-else-requires-braces.cpp statements/if/consteval-no-condition.cpp statements/if/not-constexpr.cpp

In the `if consteval` form:

- each branch must be a compound statement,
- `consteval` must not have a condition in parentheses.

`!` after `if` must be followed by `consteval`.
:::

## Dynamic Semantics

:::{rule} IF-1
:kind: required
:tests: statements/if/basic-branches.cpp statements/if/condition-evaluated-once.cpp

The ordinary form evaluates the condition one time. If the value is
`true`, the then branch executes. If not, the else branch executes, if
there is one.
:::

:::{rule} IF-5
:kind: required
:tests: statements/if/condition-declaration.cpp statements/if/condition-declaration-constexpr.cpp statements/if/condition-declaration-function-pointer.cpp

A condition declaration declares and initializes one variable. The value
of the variable, after contextual conversion to `bool`, selects the
branch. The declaration can use `auto` and `constexpr`.
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

In the ordinary form, a `goto` or a `case` or `default` label of an
enclosing `switch` can transfer control into a branch. The condition is
not evaluated in that case. The general rules for jumps over
initialization apply *(referenced)*.
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
the matrix agree on all 56 tests. The results are in `tests/RESULTS.md`.
