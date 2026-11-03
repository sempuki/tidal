# Function definitions

A function definition gives the body of a function. This page covers:

- parameters,
- the end of a function body,
- the predefined variable `__func__`,
- the function `main`.

## Principles

:::{principle} FN-P1
:rules: FN-2 FN-14 FN-15 FN-16 FN-9 FN-12

A function can be declared many times and defined once in a {term}`translation unit`
({principle}`TR-P2`). The parameter types tell whether two
declarations name one function or two overloads (FN-15). The
declarations of one function must then agree on the other properties,
apart from the parameter names:

| Property | Rule |
|---|---|
| return type | FN-14, {rule}`RET-4` for a placeholder |
| `noexcept` specification | {rule}`DCL-14`, {rule}`EXS-8` |
| `constexpr` and `consteval` | {rule}`CX-5` |
| `= delete`, on the first declaration only | {rule}`DEL-2` |
| language linkage | {rule}`LSP-7` |
| `static` among member functions | {rule}`CLS-9` |
| ref-qualifiers among overloads | {rule}`CLS-26` |
| {term}`template head`, for a {term}`function template` | {rule}`TPL-20` |
| each default argument, given in one declaration only | {rule}`DEFARG-9` |
| parameter names, which can differ or be left out | FN-16 |

A definition is the whole function, so the definition needs complete
types for its parameters and its return value, where a declaration can
leave them incomplete. A definition stands at
namespace scope or in a class, never inside another function.
:::

:::{principle} FN-P2
:rules: FN-1 FN-13 FN-10

A parameter is a local variable of the outermost block of the body,
declared by the caller's argument. So the body cannot declare the
parameter's name again at that level ({principle}`NAM-P1`), and a nested
block can. The implementation chooses whether the parameter object ends
when the function returns or when the caller's {term}`full expression` ends.
:::

:::{principle} FN-P3
:rules: FN-3 FN-7 FN-8

A function that promises a value must return one. If control reaches the
end of its body, there is no value, and the behavior is undefined. As an
exception, reaching the end of `main` returns zero.
:::

:::{principle} FN-P4
:rules: FN-4 FN-6

`main` is the program's entry point ({principle}`TR-P3`) and nothing
else, so `main` keeps a fixed shape and the program cannot treat it as
an ordinary function. The two parameter lists that every implementation
accepts carry the program's arguments.
:::

:::{principle} FN-P5
:rules: FN-5 FN-11

Inside every function body, `__func__` names a string that holds the
function's name, in a form the implementation chooses.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   function_definition: {`attribute_spec`} [`decl_specifiers`] `declarator`
                      :    [`virt_specifiers`] `function_body`
                      : | {`attribute_spec`} [`decl_specifiers`] `declarator`
                      :    `requires_clause` `function_body`
   function_body: [`ctor_init`] `compound_stmt`
                : | `function_try_block`
                : | "=" "delete" ";"
                : | "=" "default" ";"
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
:tests: declarations/functions/definitions/redeclaration-ok.cpp declarations/functions/definitions/same-parameter-types-one-function.cpp

A function can have several declarations in a translation unit. FN-14
lists what is not valid.
:::

:::{rule} FN-14
:kind: reject
:tests: declarations/functions/definitions/redefinition.cpp declarations/functions/definitions/return-type-mismatch.cpp declarations/functions/definitions/same-parameter-types-two-definitions.cpp

A function can have only one definition in a translation unit (FN-2).
All declarations of a function must have the same return type.
:::

:::{rule} FN-15
:kind: required
:tests: declarations/functions/definitions/same-parameter-types-one-function.cpp declarations/functions/definitions/different-parameter-types-overload.cpp declarations/functions/definitions/same-parameter-types-two-definitions.cpp

Two declarations of one name in one scope declare one function if they
have the same parameter types after the adjustments of {rule}`DCL-8`.
For member functions, the {term}`cv-qualifiers <cv-qualifier>` and ref-qualifiers must also be
the same. Otherwise the declarations declare two functions, which
overload ({rule}`OVL-1`).
:::

:::{rule} FN-16
:kind: required
:tests: declarations/functions/definitions/parameter-names-may-differ.cpp declarations/functions/definitions/parameter-name-from-definition.cpp

Declarations of one function can give a parameter different names, or
no name. In the body, only the names that the definition gives are
visible. An overriding function ({rule}`VIRT-1`) can also use names
other than those of the function that it overrides.
:::

:::{rule} FN-4
:kind: reject
:tests: declarations/functions/definitions/main-wrong-return-type.cpp declarations/functions/definitions/main-inline.cpp declarations/functions/definitions/main-constexpr.cpp declarations/functions/definitions/main-consteval.cpp declarations/functions/definitions/main-template.cpp declarations/functions/definitions/main-global-variable.cpp declarations/functions/definitions/main-call.cpp declarations/functions/definitions/main-address.cpp declarations/functions/definitions/main-in-namespace.cpp declarations/functions/definitions/main-auto.cpp declarations/functions/definitions/c-linkage-main-in-namespace.cpp

The function `main` in the global namespace:

- must have the return type `int`, written out and not deduced,
- must not be `inline`, `constexpr`, `consteval`, or a template,
- must not be called or have its address taken by the program.

A global variable must not have the name `main`. A function `main` in
another namespace is an ordinary function, and such a function must not
have C language linkage ({question}`Q-0076`).
:::

:::{rule} FN-5
:kind: reject
:tests: declarations/functions/definitions/func-predefined.cpp declarations/functions/definitions/func-outside-function.cpp

In a function body, `__func__` is a static array of `const char`. FN-11
gives the contents of the array. `__func__` must not occur outside a
function body ({question}`Q-0022`).
:::

:::{rule} FN-9
:kind: reject
:tests: declarations/functions/definitions/nested-function-definition.cpp

A function definition must be at namespace scope or in a class, and must
not be inside a function body.
:::

:::{rule} FN-12
:kind: reject
:tests: declarations/functions/definitions/incomplete-param-definition.cpp declarations/functions/definitions/incomplete-param-declaration.cpp declarations/functions/definitions/typedef-function-definition.cpp

In a function definition, each parameter type and the return type must
be complete, except `void` and except in a deleted definition. A
declaration that is not a definition can use incomplete types. A
definition must use a function {term}`declarator`. A `typedef` name of a
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
behavior is undefined. The rule does not apply to `main` (FN-8). A
function that returns `void` returns normally at the end.
:::

## Implementation Requirements

:::{rule} FN-6
:kind: impl-defined
:tests: declarations/functions/definitions/main-one-param.cpp declarations/functions/definitions/main-three-params.cpp declarations/functions/definitions/main-wrong-first-param.cpp declarations/functions/definitions/main-four-params.cpp declarations/functions/definitions/argv-zero.cpp foundations/translation/main-wrong-parameter-types.cpp

An implementation accepts `main` with the parameter lists `()` and
`(int, char**)`. Acceptance of other parameter lists, and the values
of their parameters, are implementation-defined ({question}`Q-0024`).
If `argc` is greater than zero, `argv[0]` points to the name used to
start the program or to an empty string.
:::

:::{rule} FN-10
:kind: impl-defined
:tests: declarations/functions/definitions/parameter-destruction-order.cpp

Each implementation chooses the time at which a parameter object ends:
either as the called function returns, or later, when the caller
finishes the statement or expression that holds the call. The choice is
implementation-defined. VC-10 states the same choice from the caller's
side.
:::

:::{rule} FN-11
:kind: impl-defined
:tests: declarations/functions/definitions/func-name-impl.cpp

The string in `__func__` is implementation-defined. On the
implementations in the matrix the string is the plain name of the
function.
:::

## Recommended Practice

:::{rule} FN-7
:kind: recommended
:tests: declarations/functions/definitions/flow-off-end-warns.cpp declarations/functions/definitions/switch-every-enumerator-no-return.cpp

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

All eight compilers agree on 39 of 46 tests. The results are in
`tests/RESULTS.md`.

- FN-3: all compilers reject a call that reaches the end of the function
  during constant evaluation. At run time, the undefined behavior
  sanitizer of Clang and GCC (`-fsanitize=undefined`) stops the program
  when control reaches the end ("execution reached the end of a
  value-returning function").
- FN-7: Clang gives no warning for a function whose `switch` over a
  scoped enumeration returns in a case for every enumerator and has no
  `return` after the `switch`. GCC warns with no flag
  (`-Wreturn-type`), because the condition can hold a value that no
  enumerator has ({rule}`ENUM-22`). The table in
  {doc}`/foundations/translation` gives the flags for the other cases.
- FN-4: Clang and GCC reject calls to `main` and its address only as
  pedantic diagnostics. EDG rejects them in all modes.
- FN-5: Clang accepts `__func__` outside a function with a warning. GCC
  and EDG reject such a use ({question}`Q-0022`).
- FN-6: the compilers accept these parameter lists of `main`
  ({question}`Q-0024`):

  | Parameters | Clang | GCC | EDG |
  |---|---|---|---|
  | `(int)` | accepts | rejects | accepts |
  | `(double)` | rejects | rejects | accepts |
  | `(long, char**)` | rejects | rejects | accepts |
  | `(int, char**, char**)` | accepts | accepts | accepts |
  | `(int, char**, char**, char**)` | rejects | rejects | accepts |

  GCC rejects `(int)` only under `-pedantic-errors`. With three
  parameters, Clang and GCC on Linux pass a pointer that is not null as
  the third argument, the list of environment variables.
- FN-7: Clang and GCC warn with `-Wreturn-type`.
- FN-10: Clang 22 and GCC 16 destroy a by-value parameter at the end of
  the caller's full expression, after the next operand of a comma.
- FN-4: GCC 14 and Clang 18 accept `main` with C language linkage in a
  namespace. GCC 16, Clang 22, and EDG reject such a `main`
  ({question}`Q-0076`).

Inputs: clang-sema-decl, clang-sema-declcxx, clang-sema-stmt,
clang-diag-sema, edg-func-def.
