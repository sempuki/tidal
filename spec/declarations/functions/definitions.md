# Function definitions

A function definition gives the body of a function. This page covers
parameters, the end of a function body, the predefined variable
`__func__`, and the function `main`.

## Principles

:::{principle} FN-P1
:rules: FN-2 FN-14 FN-9 FN-12

A function can be declared many times and defined once in a
translation unit ({principle}`TR-P2`). A definition is the whole
function, so it needs complete types for its parameters and its return
value, where a declaration can leave them incomplete. A definition
stands at namespace scope or in a class, never inside another function.
:::

:::{principle} FN-P2
:rules: FN-1 FN-13 FN-10

A parameter is a local variable of the outermost block of the body,
declared by the caller's argument. So the body cannot declare its name
again at that level ({principle}`NAM-P1`), and a nested block can. The
implementation chooses whether the parameter object ends when the
function returns or when the caller's full expression ends.
:::

:::{principle} FN-P3
:rules: FN-3 FN-7 FN-8

A function that promises a value must return one. If control reaches
the end of its body, there is no value, and the behavior is undefined.
`main` is the one exception: reaching its end returns zero.
:::

:::{principle} FN-P4
:rules: FN-4 FN-6

`main` is the program's entry point ({principle}`TR-P3`) and nothing
else, so it keeps a fixed shape and the program cannot treat it as an
ordinary function. The two parameter lists that every implementation
accepts carry the program's arguments.
:::

:::{principle} FN-P5
:rules: FN-5 FN-11

Inside every function body, `__func__` names a string that holds the
function's name, in a form the implementation chooses.
:::

## Syntax

```{eval-rst}
.. productionlist:: fn_definitions
   function_definition: `decl_specifiers` `declarator` `function_body`
   function_body: `compound_statement` | "=" "delete" ";" | "=" "default" ";"
```

## Constraints

:::{rule} FN-1
:kind: required
:tests: declarations/functions/definitions/unnamed-parameters.cpp declarations/functions/definitions/parameter-nested-block.cpp

A parameter can have no name. The scope of a parameter is the outermost
block of the function body. A nested block can declare the name again.
FN-13 lists what is not valid.
:::

:::{rule} FN-13
:kind: reject
:tests: declarations/functions/definitions/parameter-scope-redeclare.cpp declarations/functions/definitions/parameter-duplicate.cpp

The program must not declare a parameter name again in the outermost
block of the function body (FN-1), or twice in one parameter list.
:::

:::{rule} FN-2
:kind: required
:tests: declarations/functions/definitions/redeclaration-ok.cpp

A function can have several declarations in a translation unit. FN-14
lists what is not valid.
:::

:::{rule} FN-14
:kind: reject
:tests: declarations/functions/definitions/redefinition.cpp declarations/functions/definitions/return-type-mismatch.cpp

A function can have only one definition in a translation unit (FN-2).
All declarations of a function must have the same return type.
:::

:::{rule} FN-4
:kind: reject
:tests: declarations/functions/definitions/main-wrong-return-type.cpp declarations/functions/definitions/main-inline.cpp declarations/functions/definitions/main-constexpr.cpp declarations/functions/definitions/main-consteval.cpp declarations/functions/definitions/main-template.cpp declarations/functions/definitions/main-global-variable.cpp declarations/functions/definitions/main-call.cpp declarations/functions/definitions/main-address.cpp declarations/functions/definitions/main-in-namespace.cpp declarations/functions/definitions/main-auto.cpp declarations/functions/definitions/c-linkage-main-in-namespace.cpp

The function `main` in the global namespace:

- must have the return type `int`, written out and not deduced,
- must not be `inline`, `constexpr`, `consteval`, or a template,
- must not be called or have its address taken by the program.

A global variable must not have the name `main`. A function `main` in
another namespace is an ordinary function, and it must not have C
language linkage ({question}`Q-0076`).
:::

:::{rule} FN-5
:kind: reject
:tests: declarations/functions/definitions/func-predefined.cpp declarations/functions/definitions/func-outside-function.cpp

In a function body, `__func__` is a static array of `const char`. FN-11
gives its contents. `__func__` must not occur outside a
function body ({question}`Q-0022`).
:::

:::{rule} FN-9
:kind: reject
:tests: declarations/functions/definitions/nested-function-definition.cpp

A function definition must be at namespace scope or in a class. It must
not be inside a function body.
:::

:::{rule} FN-12
:kind: reject
:tests: declarations/functions/definitions/incomplete-param-definition.cpp declarations/functions/definitions/incomplete-param-declaration.cpp declarations/functions/definitions/typedef-function-definition.cpp

In a function definition, each parameter type and the return type must
be complete, except `void` and except in a deleted definition. A
declaration that is not a definition can use incomplete types. A
definition must use a function declarator. A `typedef` name of a
function type cannot define a function.
:::

## Runtime Behavior

:::{rule} FN-8
:kind: required
:tests: declarations/functions/definitions/main-returns-zero.cpp declarations/functions/definitions/main-return-value.cpp declarations/functions/definitions/main-parameters.cpp

If control reaches the end of `main`, the effect is `return 0;`. If
`main` has the parameters `(int argc, char* argv[])`, `argv[argc]` is a
null pointer.
:::

## Undefined Behavior

:::{rule} FN-3
:kind: undefined
:tests: declarations/functions/definitions/flow-off-end-value.cpp declarations/functions/definitions/flow-off-end-not-reached.cpp declarations/functions/definitions/flow-off-end-void.cpp declarations/functions/definitions/flow-off-end-sanitizer.cpp

If control reaches the end of a function that returns a value, the
behavior is undefined. This does not apply to `main` (FN-8). A function
that returns `void` returns normally at the end.
:::

## Implementation Requirements

:::{rule} FN-6
:kind: impl-defined
:tests: declarations/functions/definitions/main-one-param.cpp declarations/functions/definitions/main-three-params.cpp declarations/functions/definitions/main-wrong-first-param.cpp declarations/functions/definitions/main-four-params.cpp declarations/functions/definitions/argv-zero.cpp foundations/translation/main-wrong-parameter-types.cpp

An implementation accepts `main` with the parameter lists `()` and
`(int, char**)`. It is implementation-defined if it accepts other
parameter lists ({question}`Q-0024`). If `argc` is greater than zero,
`argv[0]` points to the name used to start the program or to an empty
string.
:::

:::{rule} FN-10
:kind: impl-defined
:tests: declarations/functions/definitions/parameter-destruction-order.cpp

Each implementation chooses the time at which a parameter object ends:
either as the called function returns, or later, when the caller finishes
the statement or expression that holds the call. The choice is
implementation-defined. VC-10 states the same choice from the caller's
side.
:::

:::{rule} FN-11
:kind: impl-defined
:tests: declarations/functions/definitions/func-name-impl.cpp

The string in `__func__` is implementation-defined. On the
implementations in the matrix it is the plain name of the function.
:::

## Recommended Practice

:::{rule} FN-7
:kind: recommended
:tests: declarations/functions/definitions/flow-off-end-warns.cpp

An implementation should warn if control can reach the end of a function
that returns a value.
:::

## Examples

```cpp
int pick(int, int b) { return b; }   // the first parameter has no name

int f(int x) {
  int x = 2;                         // rejected: same block as the parameter
}

int sign(int v) {
  if (v > 0) return 1;
}                                    // sign(0) is undefined

int main() {}                        // returns 0

void name() {
  static_assert(sizeof(__func__) == 5);   // "name" and a null character
}
```

## Evidence

All seven compilers agree on 33 of 38 tests. The results are in
`tests/RESULTS.md`.

- FN-3: all compilers reject a call that reaches the end of the function
  during constant evaluation.
- FN-4: Clang and GCC reject calls to `main` and its address only as
  pedantic diagnostics. EDG rejects them in all modes.
- FN-5: Clang accepts `__func__` outside a function with a warning. GCC
  and EDG reject it ({question}`Q-0022`).
- FN-6: GCC rejects `(int)` under `-pedantic-errors`. EDG accepts
  `(double)` and four parameters. All compilers accept three parameters
  ({question}`Q-0024`).
- FN-7: Clang and GCC warn with `-Wreturn-type`.
- FN-10: Clang 22 and GCC 16 destroy a by-value parameter at the end of
  the caller's full expression, after the next operand of a comma.
- FN-4: GCC 14 and Clang 18 accept `main` with C language linkage in a
  namespace. GCC 16, Clang 22, and EDG reject it ({question}`Q-0076`).

Inputs: clang-sema-decl, clang-sema-declcxx, clang-sema-stmt, clang-diag-
sema, edg-func-def.
