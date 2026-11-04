# Default arguments

A parameter can have a default argument. A call that leaves out the
argument uses the default argument.

Rules on other pages also govern default arguments:

| Topic | Rule |
|---|---|
| A default argument names entities only when a call uses it | {rule}`ODR-6` |
| An `inline` function that uses another function's defaults | {rule}`ODR-10` |
| Access in a default argument | {rule}`ACC-16` |
| Members that a member function's default argument sees | {rule}`CLS-2`, {principle}`NAM-P4` |
| When a template instantiates a default argument | {rule}`INST-9` |
| An explicit specialization | {rule}`TSP-8` |
| A function parameter pack | {rule}`PACK-4` |
| A parameter of a requires expression | {rule}`CPT-29` |
| An explicit object parameter | {rule}`CLS-14` |
| Operator functions and literal operators | {rule}`OPR-2`, {rule}`OPR-3`, {rule}`OPR-10` |
| A virtual call | {rule}`VIRT-1` |
| A lambda | {rule}`LAM-1` |
| Default, copy, and move constructors | the glossary entry {term}`default constructor`, {rule}`CPY-2`, {rule}`CPY-15` |
| Overload resolution and deduction | {rule}`OVL-25`, {rule}`OVL-38`, {rule}`DED-9` |
| Temporaries in a default argument | {rule}`VC-9` |

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

A parameter or a local also has a type, which the declaration already
knows. So an operand that is not evaluated, such as the operand of
`sizeof`, can name a parameter or a local. `this` is not allowed
anywhere in a default argument, also in such an operand.
:::

:::{principle} DEFARG-P2
:rules: DEFARG-1 DEFARG-8 DEFARG-2 DEFARG-9 DEFARG-11 DEFARG-12 DEFARG-13 DEFARG-5 DEFARG-10 DEFARG-6 DEFARG-7

Default arguments belong to the declarations of a function, and the
declarations in one scope add them up. Each later declaration can add
defaults, and none can repeat one. A using declaration brings the
function into another scope with no defaults of its own. A call through
the using declaration uses the defaults that the function's scope has
at the call, also those that a declaration after the using declaration
added. A template is the exception, because
each of its specializations has to see the defaults at once: a function
template and a member function of a class template take their defaults
from the first declaration only. A declaration in a block starts a
set of its own. A call matches arguments to parameters from the left,
so defaults fill the parameter list from the right. A pointer or a
reference to the function carries no default arguments. An
{term}`operator function` other than `operator()` and `operator[]` has
no use for them, because operator notation gives every operand. Across
{term}`translation units <translation unit>`, an `inline` function must
end up with the same defaults everywhere.
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
:tests: declarations/functions/default-arguments/added-later.cpp declarations/functions/default-arguments/block-scope-declaration.cpp declarations/functions/default-arguments/using-sees-later-default.cpp declarations/functions/default-arguments/member-defined-outside-adds-default.cpp declarations/functions/default-arguments/block-declaration-hides-outer-default.cpp declarations/functions/default-arguments/block-declaration-repeats-outer-default.cpp

A later declaration in the same scope can add default arguments, except
when the function is a template, or its class is a class template or
lies inside one (DEFARG-13). These rules apply:

- A declaration outside its class or namespace, such as the definition
  `int S::f(int x = 4)`, counts as a declaration in the scope of that
  class or namespace.
- A declaration in a block has its own default arguments. The block
  declaration hides the default arguments of the declarations outside
  the block.
- A default argument that a later declaration in a namespace adds is
  also usable through an earlier using declaration of the function
  ({question}`Q-0111`).

DEFARG-9, DEFARG-11, DEFARG-12, and DEFARG-13 list what is not valid.
:::

:::{rule} DEFARG-9
:kind: reject
:tests: declarations/functions/default-arguments/redefined.cpp declarations/functions/default-arguments/redefined-different.cpp declarations/functions/default-arguments/block-declaration-repeats-outer-default.cpp

A later declaration in the same scope (DEFARG-2) must not give a
default argument again for the same parameter. The rule applies also
when the value is the same. A declaration in a block is in another
scope, so the block declaration can give the parameter a default
argument again ({question}`Q-0140`).
:::

:::{rule} DEFARG-11
:kind: reject
:tests: declarations/functions/default-arguments/friend-default-not-definition.cpp declarations/functions/default-arguments/friend-default-later-declaration.cpp declarations/functions/default-arguments/friend-default-definition.cpp

A friend declaration that gives a default argument must be a
definition. That friend declaration must be the only declaration of the
function in the translation unit.
:::

:::{rule} DEFARG-12
:kind: reject
:tests: declarations/functions/default-arguments/outside-default-makes-default-constructor.cpp declarations/functions/default-arguments/outside-default-makes-copy-constructor.cpp declarations/functions/default-arguments/outside-default-makes-move-constructor.cpp

A declaration outside the class must not add a default argument that
makes a constructor a {term}`default constructor`, a copy constructor,
or a move constructor ({rule}`CPY-2`,
{question}`Q-0370`).
:::

:::{rule} DEFARG-13
:kind: reject
:tests: declarations/functions/default-arguments/function-template-added-later.cpp declarations/functions/default-arguments/class-template-member-outside-adds.cpp declarations/functions/default-arguments/member-template-outside-adds.cpp declarations/functions/default-arguments/nested-class-of-template-outside-adds.cpp

A later declaration must not add a default argument to these
functions:

- a function template, also a member function template of a class,
- a member function whose class is a class template or lies inside
  one. Its default arguments must be on its declaration in the class.

A definition outside the class is such a later declaration. The rule
applies also if no call uses the default argument.
:::

:::{rule} DEFARG-4
:kind: reject
:tests: declarations/functions/default-arguments/refers-to-parameter.cpp declarations/functions/default-arguments/refers-to-local.cpp declarations/functions/default-arguments/local-unevaluated.cpp declarations/functions/default-arguments/parameter-in-sizeof.cpp declarations/functions/default-arguments/this-in-default.cpp declarations/functions/default-arguments/this-in-sizeof-default.cpp

A default argument must not use `this`. A default argument can use a
parameter of the function, or a local variable of an enclosing function,
only in an operand that is not evaluated, such as the operand of
`sizeof` ({question}`Q-0038`).
:::

:::{rule} DEFARG-5
:kind: reject
:tests: declarations/functions/default-arguments/function-pointer-no-default.cpp declarations/functions/default-arguments/pointer-declaration-default.cpp declarations/functions/default-arguments/function-reference-no-default.cpp declarations/functions/default-arguments/alias-function-type-default.cpp declarations/functions/default-arguments/typedef-function-type-default.cpp

Only the declaration of a function can have default arguments. A
function type that a `typedef` or an alias declaration names must not
have one. A pointer or a reference to a function has none, so a call
through the pointer or the reference must give each argument.
:::

:::{rule} DEFARG-10
:kind: reject
:tests: declarations/functions/default-arguments/default-checked-unused.cpp declarations/functions/default-arguments/template-default-independent-unused.cpp declarations/functions/default-arguments/default-immediate-call-checked-at-call.cpp declarations/functions/default-arguments/default-immediate-call-unused-not-checked.cpp declarations/functions/default-arguments/member-default-sees-later-members.cpp

A default argument must be valid for its parameter type, also if no
call uses it. The check happens at the declaration, with two
exceptions:

- A default argument of a member function is checked as if at the end
  of the class, so it can use members and nested classes that the class
  declares later (DEFARG-3).
- A call of an immediate function in a default argument is neither
  evaluated nor checked as a constant at the declaration. Each call
  that uses the default argument checks it
  ({question}`Q-0379`).

In a template, a default argument that depends on a template parameter
is checked when a call uses it
({rule}`INST-9`). For a default argument that depends on no template
parameter, {rule}`DEP-30` applies.
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
:tests: declarations/functions/default-arguments/evaluated-each-call.cpp declarations/functions/default-arguments/uses-global-at-call.cpp declarations/functions/default-arguments/name-bound-at-declaration.cpp declarations/functions/default-arguments/member-default-sees-later-member.cpp

A default argument is evaluated at each call that uses it, and reads the
current values of the variables that it names. The names in a default
argument refer to the entities that are visible at the function
declaration, not at the call. A default argument of a member function
also sees the members declared after the function ({rule}`CLS-2`).
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

% tidal-example: reject
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

void m() {
  int f(int a, int b, int c);  // block declaration: no defaults here
  f(1);                     // rejected: the outer defaults are hidden
}

struct S { int size(int unit); };
int S::size(int unit = 4) { return unit; }  // adds a default for unit

int (&r)(int, int, int) = f;
r(1);                       // rejected: a reference has no defaults
```

## Evidence

All eight compilers agree on 38 of 46 tests. The results are in
`tests/RESULTS.md`.

{question}`Q-0111` is provisional. A gap review supports its answer with no doubt, and DEFARG-P2 does not give the answer under the test of the style guide (section 3). The test keeps its override for Clang 22.

- DEFARG-2: Clang rejects a call that needs a default argument added by
  a later namespace declaration, through an earlier using declaration.
  GCC and EDG accept the call ({question}`Q-0111`).
- DEFARG-4: EDG rejects a parameter in an unevaluated default argument,
  such as the operand of `sizeof`. Clang and GCC accept it
  ({question}`Q-0038`).
- DEFARG-10: GCC accepts a function template whose default argument
  depends on no template parameter and is not valid, if no call uses
  the default. Clang and EDG reject the template. {rule}`DEP-30` allows
  both results.
- DEFARG-12: Clang and EDG reject a definition outside the class that
  adds a default argument and so makes a constructor a default, copy,
  or move constructor. GCC accepts it
  ({question}`Q-0370`). A
  definition outside the class that adds a default argument to another
  member function of a class that is not a template is valid on every
  compiler.
  GCC accepts each form that the probes tried, also a program that uses
  the new default constructor. So every test of DEFARG-12 needs an
  override for GCC.
- DEFARG-13: every compiler rejects a default argument that a later
  declaration adds to a function template, to a member function
  template, or to a member function whose class is a class template or
  lies inside one. The gap review of the rules that phase 7 rewrote
  (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`) found no
  rule that stated this limit.
- DEFARG-10: a call of a `consteval` function in a default argument is
  checked at each call that uses the default, on Clang and GCC. EDG
  checks the call at the declaration, and rejects a call whose function
  has no definition yet or whose argument is not a constant
  ({question}`Q-0379`). Every compiler
  accepts a default argument of a member function that uses a static
  member and a nested class declared later in the class.
- DEFARG-7 needs no diagnostic because each unit is translated alone.
  Two units with different defaults compile and link without a
  diagnostic. The test accepts either value. The tests of DEFARG-7
  agree on every compiler.

Inputs: clang-sema-decl, clang-sema-declcxx, clang-diag-sema,
edg-func-def.
