# The if statement

An `if` statement executes one of two branches, or no branch, as a
condition selects. There are three forms:

- The *ordinary form* tests a condition at run time.
- The *`if constexpr` form* tests a condition during translation. The
  form discards the branch that the condition does not select.
- The *`if consteval` form* tests if the statement executes during
  {term}`constant evaluation`.

## Principles

:::{principle} IF-P1
:rules: IF-1 IF-2 IF-3 IF-4 IF-5 IF-6

An `if` statement evaluates one condition and takes one of two paths.
The condition is converted to `bool` by {term}`contextual conversion`
and evaluated one time. The condition can also declare one variable,
whose value selects the path, so that the branches can use the value
that the condition computed. The condition variable is a plain variable
of the statement, so the variable has an initializer and no storage
class. An `else` belongs to the nearest `if` that has none, because that
reading has no ambiguity.
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
of the statement's names at its top level would make the name mean two
things in one place, so the redeclaration is rejected, while a block
inside the branch can hide the name. The initializer can be any
declaration that a block can hold, which
excludes the kinds that declare no variable.
:::

:::{principle} IF-P5
:rules: IF-11 IF-12

The statement's objects follow the general rules of {term}`lifetime`
({principle}`OBJ-P2`). Temporaries of the condition end before a branch
runs, and the variables of the initializer and the condition end with
the statement. A jump into a branch skips the condition, as a jump skips
any statement.
:::

:::{principle} IF-P3
:rules: IF-13 IF-14 IF-15 IF-16 IF-17 IF-22 IF-23

`if constexpr` moves the decision to translation. The condition is a
constant, so the implementation knows which branch is dead and discards
that branch. For a discarded branch:

- The branch is still parsed and, outside a template, still checked,
  because the branch is still part of the program.
- In a template the branch is not instantiated, so the branch can hold
  code that is wrong for the {term}`template arguments <template argument>`.
- The `return` statements of the branch do not deduce the return type,
  and nothing that the branch names needs a definition, because the
  branch never runs.

A `goto`, `case` label, or `default` label outside the statement cannot
jump into either branch, whether the branch is selected or discarded.
The same holds for `if consteval`.
:::

:::{principle} IF-P4
:rules: IF-18 IF-19 IF-20 IF-24 IF-25

`if consteval` asks which mode the program is running in
({principle}`TR-P4`):

| Form | Runs in constant evaluation | Runs at run time |
|---|---|---|
| `if consteval` | The first branch | The second branch |
| `if !consteval` | The second branch | The first branch |

The branch that runs in constant evaluation is a place where a
`consteval` call needs no constant arguments, because the whole branch
is already a constant evaluation. The form has no condition, because the
mode is the condition. The branches of the form are blocks, so that the
form is never mistaken for the ordinary one.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   if_stmt: "if" ["constexpr"] "(" [`init_stmt`] `condition` ")"
          :    `branch` ["else" `branch`]
          : | "if" ["!"] "consteval" `block` ["else" `block`]
   init_stmt: `expression_stmt` | `null_stmt` | `simple_decl` | `alias_decl`
   condition: `expression` | `cond_decl`
   cond_decl: {`attribute_spec`} `decl_specifiers` `declarator`
            :    ("=" `assignment_expr` | "=" `braced_init` | `braced_init`)
   branch: `statement`
   block: `compound_stmt`
```

The *then branch* is the first `branch` or `block`. The *else branch* is
the `branch` or `block` after `else`.

## Constraints

:::{rule} IF-2
:kind: required
:tests: statements/if/dangling-else.cpp statements/if/dangling-else-in-loop.cpp

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
:tests: statements/if/condition-not-convertible.cpp statements/if/condition-scoped-enumeration.cpp

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
- a simple declaration, including a declaration of several variables or
  of a `static` variable, a class or enumeration definition, and a
  function declaration.
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
:tests: statements/if/substatement-scope.cpp statements/if/else-substatement-scope.cpp

Each branch has its own scope, also when the branch has no braces. A
name that a branch declares is not visible after the `if` statement.
:::

:::{rule} IF-9
:kind: required
:tests: statements/if/condition-declaration.cpp statements/if/init-declaration.cpp statements/if/init-scope-ends.cpp statements/if/else-if-scope.cpp

A name that the initializer or the condition declares is visible from its
declaration to the end of the `if` statement, including the else branch.
An `else if` is a new `if` statement in the else branch.
:::

:::{rule} IF-10
:kind: reject
:tests: statements/if/redeclare-in-branch.cpp statements/if/redeclare-in-else.cpp statements/if/redeclare-init-in-branch.cpp statements/if/redeclare-init-and-condition.cpp statements/if/redeclare-in-else-statement.cpp statements/if/redeclare-in-nested-block.cpp

A name that the initializer or the condition declares must not be declared
again in these places:

- the condition, for a name from the initializer,
- the outermost block of a branch,
- a branch that is one declaration without braces.

A block inside a branch can declare the name again. The inner
declaration hides the first one.
:::

:::{rule} IF-13
:kind: required
:tests: statements/if/constexpr-discard-in-template.cpp statements/if/constexpr-discard-generic-lambda.cpp

In the `if constexpr` form, the branch that the condition does not select
is a {term}`discarded statement`. In a {term}`templated context`, a
discarded statement is not instantiated and can hold code that is not
valid for the arguments.
A {term}`requires expression` in the condition can test that code
({rule}`CPT-24`).
:::

:::{rule} IF-22
:kind: reject
:tests: statements/if/constexpr-template-nondependent-checked.cpp statements/if/constexpr-template-nondependent-call-checked.cpp

In a {term}`templated context`, the parts of a {term}`discarded
statement` that are the same for every template argument must be valid
(IF-13). The implementation checks these parts. Such a part includes a
name that is not a {term}`dependent name`, such as a function called
with no arguments ({rule}`DEP-6`).
:::

:::{rule} IF-14
:kind: required
:tests: statements/if/constexpr-discard-return-type.cpp statements/if/constexpr-non-template-return-type.cpp

A `return` statement in a discarded statement does not take part in
return type {term}`deduction`. This rule applies in templates and in other
functions.
:::

:::{rule} IF-15
:kind: required
:tests: statements/if/constexpr-non-template-no-definition.cpp statements/if/constexpr-non-template-variable-no-definition.cpp

A function or variable that only discarded statements use needs no
definition.
:::

:::{rule} IF-23
:kind: reject
:tests: statements/if/constexpr-non-template-checked.cpp statements/if/constexpr-non-template-type-error.cpp

Outside a {term}`templated context`, a {term}`discarded statement` must
be valid (IF-15).
:::

:::{rule} IF-16
:kind: reject
:tests: statements/if/constexpr-condition-not-constant.cpp statements/if/constexpr-condition-narrowing.cpp statements/if/constexpr-condition-pointer.cpp statements/if/constexpr-condition-explicit-bool.cpp statements/if/constexpr-with-init.cpp

The condition of the `if constexpr` form must be a constant expression
after contextual conversion to `bool`. A conversion that loses
information is permitted, such as an integer or an address to `bool`.
:::

:::{rule} IF-17
:kind: reject
:tests: statements/if/constexpr-goto-into.cpp statements/if/constexpr-case-into.cpp statements/if/consteval-goto-into.cpp

The following must not transfer control from outside an `if constexpr`
or `if consteval` statement into a branch of the statement:

- a `goto`,
- a `case` label,
- a `default` label.
:::

:::{rule} IF-19
:kind: required
:tests: statements/if/consteval-immediate-context.cpp statements/if/not-consteval-else-immediate.cpp

The branch that executes during constant evaluation is the then branch
of `if consteval` and the else branch of `if !consteval`. That branch is
an {term}`immediate function context`.
:::

:::{rule} IF-24
:kind: reject
:tests: statements/if/consteval-immediate-outside.cpp statements/if/consteval-immediate-else.cpp

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
the statement has one.
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
:tests: statements/if/lifetimes.cpp statements/if/lifetimes-else-and-two-variables.cpp

Temporary objects that the condition creates are destroyed before a
branch executes. Variables that the initializer or the condition declares
are destroyed when the `if` statement ends. The variable that is declared
last is destroyed first.
:::

:::{rule} IF-12
:kind: required
:tests: statements/if/goto-into-branch.cpp statements/if/case-into-branch.cpp

In the ordinary form, the following can transfer control into a branch:

- a `goto`,
- a `case` label of an enclosing `switch`,
- a `default` label of an enclosing `switch`.

The condition is not evaluated in that case. The general rules for jumps
over initialization apply ({rule}`JMP-6`).
:::

:::{rule} IF-18
:kind: required
:tests: statements/if/consteval-branches.cpp statements/if/consteval-in-consteval.cpp statements/if/consteval-plain-function.cpp declarations/initialization/static/consteval-branch-in-initializer.cpp statements/if/consteval-local-initializers.cpp

The `if consteval` form executes its then branch during
{term}`constant evaluation` and its else branch at run time (IF-25).
The `if !consteval` form
does the opposite. In a `consteval` function, the then branch of
`if consteval` always executes. In a function that is not `constexpr` or
`consteval`, the then branch never executes.
:::

:::{rule} IF-25
:kind: required
:tests: statements/if/consteval-local-initializers.cpp declarations/initialization/static/consteval-branch-in-initializer.cpp statements/if/consteval-local-reference.cpp statements/if/consteval-static-fallback.cpp

An evaluation is a constant evaluation only if it starts in one of
these places:

- a place in the table of {principle}`CE-P4`,
- an initializer that is a constant expression, of a namespace-scope,
  `static`, or `thread_local` variable of any type, a class type
  included ({rule}`SINIT-1`),
- an initializer that is a constant expression, of a `const` variable
  of integer or enumeration type that is not `volatile` ({rule}`CE-3`).

If such an initializer is not a constant expression, the whole
initializer runs at run time. Every other evaluation happens at run
time, such as the initializer of a local variable that is not `const`
or of a local reference that binds a temporary ({question}`Q-0329`).
:::

## Examples

% tidal-example: reject
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
int offset = 0;
int fixed = where();           // 1: a constant initializer (SINIT-1)
int moved = where() + offset;  // 2: offset is not a constant

struct Pair { int first; int second; };

void use() {
  int local = where();         // 2: a local initializer runs at run time
  const int kept = where();    // 1: a const integer (IF-25)
  static int once = where();   // 1: a constant initializer
  static int late = where() + offset;          // 2: falls back to run time
  static Pair both{where(), where()};          // {1, 1}: a class type counts
  static Pair mixed{where(), offset};          // {2, 0}: the whole initializer runs at run time
}

template <class T> void check(T t) {
  if constexpr (false) {
    t.anything();              // valid: depends on T, never instantiated
    undeclared();              // rejected: no arguments depend on T
  }
}

constexpr int g = 0;
void conditions() {
  if constexpr (2) {}          // valid: 2 converts to true
  if constexpr (&g) {}         // valid: the address converts to true
}
```

## Evidence

All rules in this page are `required` or `reject`. All eight compilers
agree on 77 of 78 tests. The results are in `tests/RESULTS.md`.

One question on this page settled on 2026-11-03 under decision 0054 after the source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`). For each, the review supports the answer with no doubt, and a principle gives the same answer. Each test keeps its override for the family that still differs:

| Question | Principle | Family that still differs |
|---|---|---|
| {question}`Q-0329` | {principle}`CE-P2` | GCC 16 |

- IF-25: Clang and GCC take the then branch of `if consteval` in the
  initializer of a local `const int` or `const` enumeration variable.
  Both take the else branch for a local `const double`, a local
  `const volatile int`, and a local variable that is not `const`. For a
  local `const int&` that binds a temporary, Clang takes the else
  branch and GCC takes the then branch
  ({question}`Q-0329`).
