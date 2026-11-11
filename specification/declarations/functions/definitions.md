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
| each default argument, given in one declaration of each scope only | {rule}`DEFARG-9` |
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

Each function body declares `__func__` at its start, before the
member initializers of a constructor. The name is local to the body,
and the array it names has static storage duration, so a pointer to it
stays valid after the call. In the body, `__func__` names a string that
holds the function's name, in a form the implementation chooses. Outside a
function body, nothing declares `__func__`, so the implementation
rejects a use there.
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
:tests: declarations/functions/definitions/redefinition.cpp declarations/functions/definitions/return-type-mismatch.cpp declarations/functions/definitions/same-parameter-types-two-definitions.cpp declarations/linkage-specifications/c-name-unnamed-namespace-redefined.cpp

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
:tests: declarations/functions/definitions/func-predefined.cpp declarations/functions/definitions/func-outside-function.cpp declarations/functions/definitions/func-in-member-initializer.cpp

In a function body, `__func__` is a static array of `const char`. FN-11
gives the contents of the array. `__func__` must not occur outside a
function body ({question}`Q-0022`).
:::

:::{rule} FN-9
:kind: reject
:tests: declarations/functions/definitions/nested-function-definition.cpp declarations/functions/definitions/nested-function-definition-member.cpp

A function definition must be at namespace scope or in a class, and must
not be inside a function body.
:::

:::{rule} FN-12
:kind: reject
:tests: declarations/functions/definitions/incomplete-param-definition.cpp declarations/functions/definitions/incomplete-param-declaration.cpp declarations/functions/definitions/typedef-function-definition.cpp declarations/functions/definitions/member-in-class-specialization-over-class.cpp declarations/functions/definitions/member-in-class-type-completed-after-class.cpp declarations/functions/definitions/nested-member-type-completed-in-enclosing-class.cpp declarations/functions/definitions/friend-in-class-specialization-over-class.cpp declarations/functions/definitions/friend-in-class-type-completed-after-class.cpp

In a function definition, each parameter type and the return type must
be complete, except `void` and except in a deleted definition. If a
class defines a member or friend function, its types must be complete
where the outermost enclosing class ends. A declaration that is not a definition can
use incomplete types. A definition must use a function
{term}`declarator`, and a `typedef` name of a function type cannot
define a function.
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
:tests: declarations/functions/definitions/main-one-param.cpp declarations/functions/definitions/main-three-params.cpp declarations/functions/definitions/main-wrong-first-param.cpp declarations/functions/definitions/main-four-params.cpp declarations/functions/definitions/argv-zero.cpp foundations/translation/main-wrong-parameter-types.cpp declarations/functions/definitions/main-one-param-ilp32.cpp declarations/functions/definitions/main-three-params-ilp32.cpp declarations/functions/definitions/main-wrong-first-param-ilp32.cpp declarations/functions/definitions/main-four-params-ilp32.cpp foundations/translation/main-wrong-parameter-types-ilp32.cpp

An implementation accepts `main` with the parameter lists `()` and
`(int, char**)`. Acceptance of other parameter lists, and the values
of their parameters, are implementation-defined ({question}`Q-0024`).
If `argc` is greater than zero, `argv[0]` points to the name used to
start the program or to an empty string. The matrix compilers accept
these other lists, on x86-64 and with `-m32`:

| Parameters | Clang | GCC | EDG |
|---|---|---|---|
| `(int)` | accepts | rejects | accepts |
| `(double)` | rejects | rejects | accepts |
| `(long, char**)` | rejects | rejects | accepts |
| `(int, char**, char**)` | accepts, and the third parameter points to the list of environment variables | the same as Clang | accepts |
| `(int, char**, char**, char**)` | rejects | rejects | accepts |
:::

:::{rule} FN-10
:kind: impl-defined
:tests: declarations/functions/definitions/parameter-destruction-order.cpp declarations/functions/definitions/parameter-destruction-order-operator.cpp

Each implementation chooses the time at which a parameter object ends:
either as the called function returns, or later, when the caller
finishes the statement or expression that holds the call. The choice is
implementation-defined. {rule}`VC-10` states the same choice from the
caller's side and gives the choice of the matrix targets.
:::

:::{rule} FN-11
:kind: impl-defined
:tests: declarations/functions/definitions/func-name-impl.cpp declarations/functions/definitions/func-name-member.cpp

The string in `__func__` is implementation-defined. On the
implementations in the matrix the string is the plain name of the
function.
:::

## Recommended Practice

:::{rule} FN-7
:kind: recommended
:tests: declarations/functions/definitions/flow-off-end-warns.cpp declarations/functions/definitions/switch-every-enumerator-no-return.cpp declarations/functions/definitions/flow-off-end-constant-paths.cpp declarations/functions/definitions/flow-off-end-ending-calls.cpp declarations/functions/definitions/flow-off-end-loop-break-warns.cpp declarations/functions/definitions/flow-off-end-switch-no-default-warns.cpp declarations/functions/definitions/flow-off-end-plain-call-warns.cpp declarations/functions/definitions/flow-off-end-goto-loop.cpp declarations/functions/definitions/flow-off-end-main-no-warning.cpp

An implementation should warn if control can reach the end of a function
that returns a value, other than `main`. Reaching the end of `main`
returns zero (FN-8). For this check, these statements and expressions
end a path:

- a `return` statement,
- a throw-expression,
- a call of a function declared `[[noreturn]]`,
- a loop whose condition is a constant `true`, if no `break` leaves it.

An `if` with a constant condition takes only the selected branch. A
`switch` without a `default` label can complete, whatever its cases are.
:::

## Examples

% tidal-example: reject
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

All eight compilers agree on 55 of 68 tests. The results are in
`tests/RESULTS.md`.

### Divergences

- {question}`Q-0022` (FN-5; settled on FN-P5): whether `__func__`
  outside a function is rejected. It is, as GCC and EDG say. Clang
  accepts the use with a warning.
- {question}`Q-0024` (FN-6, TR-3; settled on TR-P3): which parameter
  lists `main` can have. Every implementation accepts `()` and
  `(int, char**)`, and the other lists are its own choice. GCC rejects
  `(int)`, and EDG accepts every list, which the answer allows. No
  family differs.
- {question}`Q-0076` (FN-4; settled): whether `main` can have C
  language linkage in a namespace. It cannot, as the newest version of
  each family says. GCC 14 and Clang 18 accept it, and no newest family
  differs.
- {question}`Q-0369` (LSP-4, FN-14; open): whether C names with
  internal linkage in two namespaces are one entity, and whether a C
  name in an unnamed namespace joins the C names of its spelling. The
  names are two entities, and a C name in an unnamed namespace joins
  the others, as Clang and GCC say. EDG accepts a second definition of
  such a name in another namespace.

### Split tests

EDG skips the tests that use `-Wall` or `-Werror`.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `c-linkage-main-in-namespace.cpp` | FN-4 | reject | Clang 22: reject; Clang 18: accept | GCC 16: reject; GCC 14: accept | reject | {question}`Q-0076` |
| `declarations/linkage-specifications/c-name-unnamed-namespace-redefined.cpp` | LSP-4, FN-14 | reject | reject | reject | accept | {question}`Q-0369` |
| `flow-off-end-goto-loop.cpp` | FN-7 | accept | accept | reject | skips | |
| `foundations/translation/main-wrong-parameter-types-ilp32.cpp` | FN-6 | reject | reject | reject | accept | {question}`Q-0024` |
| `foundations/translation/main-wrong-parameter-types.cpp` | TR-3, FN-6 | reject | reject | reject | accept | {question}`Q-0024` |
| `func-outside-function.cpp` | FN-5 | reject | accept | reject | reject | {question}`Q-0022` |
| `main-four-params-ilp32.cpp` | FN-6 | reject | reject | reject | accept | {question}`Q-0024` |
| `main-four-params.cpp` | FN-6 | reject | reject | reject | accept | {question}`Q-0024` |
| `main-one-param-ilp32.cpp` | FN-6 | accept | accept | reject | accept | {question}`Q-0024` |
| `main-one-param.cpp` | FN-6 | accept | accept | reject | accept | {question}`Q-0024` |
| `main-wrong-first-param-ilp32.cpp` | FN-6 | reject | reject | reject | accept | {question}`Q-0024` |
| `main-wrong-first-param.cpp` | FN-6 | reject | reject | reject | accept | {question}`Q-0024` |
| `switch-every-enumerator-no-return.cpp` | FN-7 | reject | accept | reject | skips | |

### Notes

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
- FN-5: every compiler accepts `__func__` in the member initializers of
  a constructor. The gap review of the text phase 8 wrote
  (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`)
  adds that whether the `__func__` arrays of two functions have
  distinct addresses is unspecified. No test checks the addresses.
- FN-6: GCC rejects `(int)` only under `-pedantic-errors`. With three
  parameters, Clang and GCC on Linux pass a pointer that is not null as
  the third argument, the list of environment variables. The matrix
  runs no program built with `-m32`, so the `-m32` tests check
  acceptance only. The base expectation of each test follows Clang's
  column of the table of FN-6. So the tests accept `()`, `(int)`,
  `(int, char**)`, and `(int, char**, char**)`, and reject the other
  lists. The tests record the choices of GCC and EDG as overrides, and
  FN-6 allows each of them. A test that cites TR-3 for a parameter list
  follows the same column.
- FN-7 is Tidal's own advice. The gap review of the rules that phase 7
  rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`)
  finds no such recommendation in the reference, which makes reaching
  the end undefined (FN-3) and recommends a warning only for a
  `[[noreturn]]` function that can return. FN-7 follows the warnings
  that Clang and GCC give. Neither warns at the end of `main`.
- FN-7: Clang and GCC warn with `-Wreturn-type`. Both give no warning
  for the paths that FN-7 ends, and both warn after a loop with a
  `break`, a `switch` without `default`, and a call of a function
  without `[[noreturn]]`. For a `goto` back to its own label, Clang
  gives no warning, and GCC warns that the function has no `return`
  statement. For `while (k)` with a `constexpr bool k = true`, GCC 16
  at `-O2` warns, and at `-O0` it does not.
- FN-12: a probe grid ran on every compiler of the matrix, and the
  compilers agree on each case. They accept these member functions
  defined in their class:

  - a return or parameter type that is a specialization with a member
    of the class's type, such as `Holder<Coin>` in `Coin`,
  - such a specialization over a nested class that the class defines
    after the function.

  They reject a return type that is completed only after the class, and
  `sizeof` of such a specialization in a member declaration
  ({rule}`CLS-22`).
- FN-12: the gap review of the phase 16 text (RV9-1) reads the check
  as made in the body, where the class is complete. The review names
  two more cases, and every compiler of the matrix agrees on each
  (`reports/clean-room/roadmap-13/writer-probes/rv9-points.json`):

  - A member function of a nested class can return a type that the
    enclosing class completes after the nested class
    (`nested-member-type-completed-in-enclosing-class.cpp`).
  - A friend function defined in a class can return a specialization
    with a member of the class's type, and cannot return a type that is
    completed only after the class (`friend-in-class-specialization-over-class.cpp`,
    `friend-in-class-type-completed-after-class.cpp`).
- FN-10: Clang 22 and GCC 16 destroy a by-value parameter at the end of
  the caller's full expression, after the next operand of a comma.
- FN-6, FN-10, FN-11 with `-m32`: the matrix runs no program built with
  `-m32`, because the host compilers have no 32-bit C library. In the
  writer's probes, the run tests of the three rules give the same
  results with `-m32` in the GCC 14, Clang 18, and Clang 22 containers.

### Sources

Inputs: clang-sema-decl, clang-sema-declcxx, clang-sema-stmt,
clang-diag-sema, edg-func-def.
