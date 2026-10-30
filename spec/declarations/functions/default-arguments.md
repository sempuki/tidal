# Default arguments

A parameter can have a default argument. A call that leaves out the
argument uses the default argument.

## Syntax

```{eval-rst}
.. productionlist:: fn_defarg
   parameter: `decl_specifiers` `declarator` ["=" `expression`]
```

## Legality Rules

:::{rule} DEFARG-1
:kind: required
:tests: declarations/functions/default-arguments/basic.cpp declarations/functions/default-arguments/not-trailing.cpp declarations/functions/default-arguments/variadic-after-default.cpp

After a parameter with a default argument, each later parameter must also
have one, in this declaration or in an earlier declaration in the same
scope. A `...` can follow parameters with default arguments.
:::

:::{rule} DEFARG-2
:kind: required
:tests: declarations/functions/default-arguments/added-later.cpp declarations/functions/default-arguments/redefined.cpp declarations/functions/default-arguments/redefined-different.cpp declarations/functions/default-arguments/block-scope-declaration.cpp

A later declaration in the same scope can add default arguments. It must
not give a default argument again for the same parameter. This applies
also when the value is the same. A declaration in a block has its own
default arguments.
:::

:::{rule} DEFARG-4
:kind: reject
:tests: declarations/functions/default-arguments/refers-to-parameter.cpp declarations/functions/default-arguments/refers-to-local.cpp declarations/functions/default-arguments/local-unevaluated.cpp declarations/functions/default-arguments/parameter-in-sizeof.cpp declarations/functions/default-arguments/this-in-default.cpp

A default argument must not use `this`. It must not use a parameter of the
function, or a local variable of an enclosing function, except in an
operand that is not evaluated, such as the operand of `sizeof`
({question}`Q-0038`).
:::

:::{rule} DEFARG-5
:kind: reject
:tests: declarations/functions/default-arguments/function-pointer-no-default.cpp declarations/functions/default-arguments/pointer-declaration-default.cpp

Only the declaration of a function can have default arguments. A pointer
to a function has none. A call through the pointer must give each
argument.
:::

:::{rule} DEFARG-6
:kind: reject
:tests: declarations/functions/default-arguments/operator-default-argument.cpp

An operator function other than `operator()` must not have default
arguments *(operator overloading referenced)*.
:::

## Dynamic Semantics

:::{rule} DEFARG-3
:kind: required
:tests: declarations/functions/default-arguments/evaluated-each-call.cpp declarations/functions/default-arguments/uses-global-at-call.cpp declarations/functions/default-arguments/name-bound-at-declaration.cpp

A default argument is evaluated at each call that uses it. It reads the
current values of the variables that it names. Its names refer to the
entities that are visible at the function declaration, not at the call.
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

All seven compilers agree on all 14 tests. The results are in
`tests/RESULTS.md`.

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema,
gcc-cp-decl, edg-func-def.
