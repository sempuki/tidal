# Default arguments

A parameter can have a default argument. A call that leaves out the
argument uses the default argument.

## Principles

:::{principle} DEFARG-P1
:rules: DEFARG-3 DEFARG-4

A default argument is an expression that the caller leaves out and the
call fills in. The default argument is evaluated at each call that uses
it, as if written at the call, but its names mean what they meant at the
declaration. So a default argument cannot name these entities, which
exist only inside a call:

- other parameters,
- `this`,
- the locals of an enclosing function.
:::

:::{principle} DEFARG-P2
:rules: DEFARG-1 DEFARG-8 DEFARG-2 DEFARG-9 DEFARG-5 DEFARG-6 DEFARG-7

Default arguments belong to the declarations of a function, and the
declarations in one scope add them up. Each later declaration can add
defaults, and none can repeat one. A call matches arguments to
parameters from the left, so defaults fill the parameter list from the
right. A pointer to the function carries no default arguments, and an
{term}`operator function` has no use for them. Across {term}`translation units <translation unit>`, an
`inline` function must end up with the same defaults everywhere.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   parameter: ["this"] `decl_specifiers` [`declarator` | `abstract_declarator`]
            :    ["=" (`assignment_expr` | `braced_init`)]
```

## Constraints

:::{rule} DEFARG-1
:kind: required
:tests: declarations/functions/default-arguments/basic.cpp declarations/functions/default-arguments/variadic-after-default.cpp

The last parameters of a function can have default arguments. A `...`
can follow parameters with default arguments. DEFARG-8 states which
parameters must have one.
:::

:::{rule} DEFARG-8
:kind: reject
:tests: declarations/functions/default-arguments/not-trailing.cpp declarations/functions/default-arguments/pack-or-ellipsis-after-default.cpp

After a parameter with a default argument, each later parameter must
also have one, in this declaration or in an earlier declaration in the
same scope. A {term}`function parameter pack` ({rule}`PACK-4`) and an ellipsis
are exempt.
:::

:::{rule} DEFARG-2
:kind: required
:tests: declarations/functions/default-arguments/added-later.cpp declarations/functions/default-arguments/block-scope-declaration.cpp declarations/functions/default-arguments/using-sees-later-default.cpp

A later declaration in the same scope can add default arguments. A
declaration in a block has its own default arguments. A default argument
that a later declaration in a namespace adds is also usable through an
earlier using declaration of the function ({question}`Q-0111`). DEFARG-9
lists what is not valid.
:::

:::{rule} DEFARG-9
:kind: reject
:tests: declarations/functions/default-arguments/redefined.cpp declarations/functions/default-arguments/redefined-different.cpp

A later declaration (DEFARG-2) must not give a default argument again
for the same parameter. The rule applies also when the value is the
same.
:::

:::{rule} DEFARG-4
:kind: reject
:tests: declarations/functions/default-arguments/refers-to-parameter.cpp declarations/functions/default-arguments/refers-to-local.cpp declarations/functions/default-arguments/local-unevaluated.cpp declarations/functions/default-arguments/parameter-in-sizeof.cpp declarations/functions/default-arguments/this-in-default.cpp

A default argument must not use `this`. A default argument can use a
parameter of the function, or a local variable of an enclosing function,
only in an operand that is not evaluated, such as the operand of
`sizeof` ({question}`Q-0038`).
:::

:::{rule} DEFARG-5
:kind: reject
:tests: declarations/functions/default-arguments/function-pointer-no-default.cpp declarations/functions/default-arguments/pointer-declaration-default.cpp declarations/functions/default-arguments/default-checked-unused.cpp

Only the declaration of a function can have default arguments. A pointer
to a function has none. A call through the pointer must give each
argument. A default argument must be valid for its parameter type at the
declaration, also if no call uses it.
:::

:::{rule} DEFARG-6
:kind: reject
:tests: declarations/functions/default-arguments/operator-default-argument.cpp declarations/functions/default-arguments/member-operator-default-argument.cpp

An operator function other than `operator()` and `operator[]` must not
have default arguments ({rule}`OPR-2`, {rule}`OPR-3`).
:::

## Runtime Behavior

:::{rule} DEFARG-3
:kind: required
:tests: declarations/functions/default-arguments/evaluated-each-call.cpp declarations/functions/default-arguments/uses-global-at-call.cpp declarations/functions/default-arguments/name-bound-at-declaration.cpp

A default argument is evaluated at each call that uses it, and reads the
current values of the variables that it names. The names in a default
argument refer to the entities that are visible at the function
declaration, not at the call.
:::

## Implementation Requirements

:::{rule} DEFARG-7
:kind: no-diagnostic
:tests: declarations/functions/default-arguments/inline-defaults-differ.cpp declarations/functions/default-arguments/inline-default-only-one-unit.cpp

If an `inline` function is declared in several translation units, each
parameter must have the same default argument in all of them after all
declarations. An implementation need not diagnose a difference.
:::

## Examples

```cpp
int f(int a, int b = 2, int c = 3);
f(1);                       // f(1, 2, 3)
int g(int a = 1, int b);    // rejected: b has no default argument

int h(int a, int b);
int h(int a, int b = 2);
int h(int a = 1, int b);    // adds a default for a
h();                        // h(1, 2)

int next();
int k(int v = next());      // next() is called at each k()
```

## Evidence

All eight compilers agree on 22 of 24 tests. The results are in
`tests/RESULTS.md`.

- DEFARG-2: Clang rejects a call that needs a default argument added by
  a later namespace declaration, through an earlier using declaration.
  GCC and EDG accept the call ({question}`Q-0111`).
- DEFARG-7: two units with different defaults compile and link without a
  diagnostic. The test accepts either value.

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema,
edg-func-def.
