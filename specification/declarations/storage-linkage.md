# Storage duration and linkage

Each variable has a {term}`storage duration`, which controls when its
storage exists. Most names have a {term}`linkage`, which controls if the
same name in other scopes or
{term}`translation units <translation unit>` refers to the same entity.

This page covers variables and functions at namespace scope and block
scope. Other pages cover these topics:

- class members ({doc}`/classes/members`),
- dynamic storage ({doc}`/expressions/new-delete`),
- `extern "C"` ({doc}`/declarations/linkage-specifications`).

Threads and modules are out of scope. Rules with the prefix `STO` are
about storage duration and the storage class specifiers. Rules with the
prefix `LNK` are about linkage.

## Principles

:::{principle} STO-P1
:rules: STO-1 STO-2 STO-3 STO-4 STO-12 STO-8 STO-9 STO-10

The place of a declaration and its storage class decide the storage
duration of the variable ({principle}`OBJ-P2`).

| Declaration | Storage | Objects |
|---|---|---|
| In a block | Automatic | A new object each time control passes the declaration |
| At namespace scope | Static | One for the program |
| `static` in a block | Static | One for the program |
| `thread_local` | Thread | One per thread |

A block `static` is initialized the first time control reaches its
declaration, so a second arrival during that initialization has no
defined meaning.

Static storage starts at zero before any initializer runs, and the
program ends static objects in the reverse of the order in which their
initialization completed.
:::

:::{principle} STO-P2
:rules: STO-5 STO-6 STO-7 STO-13 STO-11

A storage class is one word in one place, and only on things that have
storage. So two storage classes do not combine, except `thread_local`
with the one that says where the object lives, and none of these takes a
storage class:

- a function,
- a parameter,
- a type alias,
- a class definition.

`extern` says that the object lives outside the current declaration, so
in a block an `extern` declaration cannot carry an initializer, and an
anonymous union at namespace scope, which has no name to share, must be
`static`.
:::

:::{principle} STO-P3
:rules: LNK-1 LNK-2 LNK-3 LNK-5 LNK-8 LNK-10 LNK-9

Linkage says whether declarations in different scopes or translation
units name one entity. A name at namespace scope is shared across the
program unless something keeps it private.

| Declaration | Linkage |
|---|---|
| A name at namespace scope | Shared across the program |
| The same, with `static` | Private |
| A name in an unnamed namespace | Private |
| A `const` variable at namespace scope | Private, because `const` marks a local constant rather than a shared object |
| A block-scope variable | Shared with nothing |
| A block-scope variable with `extern` | The namespace-scope entity of that name |

The first declaration settles the linkage, and later declarations
inherit the linkage. So `static` cannot come second.
:::

:::{principle} STO-P4
:rules: LNK-4 LNK-6 LNK-7

A shared entity has one definition in the program. An `inline` entity
can be defined in every unit that needs it, provided the definitions say
the same thing, and the definitions are then one entity with one
address. A unit is translated alone ({principle}`TR-P2`), so a missing
or duplicated definition is found when the program is linked, if at all.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   storage_class: "static" | "extern" | "thread_local" | "mutable"
```

`mutable` applies to class members ({rule}`CLS-25`).

## Constraints

:::{rule} STO-4
:kind: required
:tests: declarations/storage-linkage/thread-local-combinations.cpp declarations/storage-linkage/thread-local-member-and-block.cpp

A variable declared `thread_local` has thread {term}`storage duration`.
`thread_local` can be combined with `static` or `extern`. In a block,
`thread_local` implies `static`. STO-12 lists what is not valid.
:::

:::{rule} STO-12
:kind: reject
:tests: declarations/storage-linkage/thread-local-after-plain.cpp declarations/storage-linkage/plain-after-thread-local.cpp

All declarations of a variable must agree on `thread_local` (STO-4).
:::

:::{rule} STO-5
:kind: reject
:tests: declarations/storage-linkage/register.cpp declarations/storage-linkage/register-parameter.cpp

A program must not use `register` as a storage class.
:::

:::{rule} STO-6
:kind: reject
:tests: declarations/storage-linkage/two-storage-classes.cpp declarations/storage-linkage/storage-class-on-typedef.cpp declarations/storage-linkage/static-parameter.cpp declarations/storage-linkage/extern-parameter.cpp declarations/storage-linkage/thread-local-function.cpp declarations/storage-linkage/mutable-namespace.cpp declarations/storage-linkage/static-block-function.cpp declarations/storage-linkage/static-main.cpp declarations/storage-linkage/storage-class-on-class.cpp declarations/storage-linkage/mutable-block.cpp declarations/storage-linkage/mutable-const-member.cpp declarations/storage-linkage/mutable-function.cpp declarations/storage-linkage/mutable-member-ok.cpp declarations/storage-linkage/mutable-reference-member.cpp declarations/storage-linkage/mutable-static-member.cpp

A declaration must not:

- have two storage classes, except `thread_local` with `static` or
  `extern`,
- combine a storage class with `typedef`,
- give a storage class to a function parameter,
- declare a function `thread_local`,
- declare anything `mutable` except a non-static data member. A variable
  at namespace or block scope, a function, a static data member, a
  `const` member, and a reference member cannot be `mutable`,
- declare a function `static` in a block,
- declare `main` `static`,
- give a storage class to a declaration that only defines a class.
:::

:::{rule} STO-7
:kind: required
:tests: declarations/storage-linkage/block-extern-same-unit.cpp declarations/storage-linkage/extern-with-initializer.cpp

A block-scope `extern` declaration refers to a variable with linkage
outside the block. At namespace scope, `extern` with an initializer is a
definition. LNK-9 says which entity a block-scope `extern` declaration
refers to. STO-13 lists what is not valid.
:::

:::{rule} STO-13
:kind: reject
:tests: declarations/storage-linkage/block-extern-initializer.cpp declarations/storage-linkage/block-extern-after-local.cpp

A block-scope `extern` declaration (STO-7) must not have an initializer,
or follow a local variable of the same name in the same block.
:::

:::{rule} LNK-1
:kind: required
:tests: declarations/storage-linkage/external-default.cpp declarations/storage-linkage/external-namespace-member.cpp

A variable or function at namespace scope has external linkage, unless a
rule on this page gives it another linkage. A name with external linkage
refers to one entity in all translation units.
:::

:::{rule} LNK-2
:kind: required
:tests: declarations/storage-linkage/static-internal.cpp declarations/storage-linkage/static-not-visible.cpp

`static` gives a variable or function at namespace scope internal
linkage. Each translation unit that declares the name has its own
entity. Other translation units cannot refer to that entity.
:::

:::{rule} LNK-3
:kind: required
:tests: declarations/storage-linkage/const-internal.cpp declarations/storage-linkage/extern-const-external.cpp declarations/storage-linkage/volatile-const-external.cpp declarations/storage-linkage/extern-then-const.cpp foundations/odr/header-constant-three-forms.cpp

A `const` variable at namespace scope has internal linkage. The rule
includes a `constexpr` variable, but does not apply if the variable is
also `volatile`, `inline`, or `extern`.

A `const` variable keeps external linkage if an earlier declaration of
the variable is `extern`.

A {term}`variable template` follows {rule}`ODR-12` instead: each {term}`specialization`
of a `const` or `constexpr` variable template is one object in the
program ({question}`Q-0297`).
:::

:::{rule} LNK-4
:kind: required
:tests: declarations/storage-linkage/inline-one-entity.cpp declarations/storage-linkage/inline-function-static-local.cpp

Each translation unit that uses an `inline` variable or function with
external linkage can define that variable or function. All these
definitions are one entity with one address.
:::

:::{rule} LNK-5
:kind: required
:tests: declarations/storage-linkage/unnamed-namespace-internal.cpp declarations/storage-linkage/unnamed-namespace-nested.cpp

Each name in an unnamed namespace has internal {term}`linkage`. The name
refers to an entity of its own translation unit only.
:::

:::{rule} LNK-8
:kind: required
:tests: declarations/storage-linkage/static-then-extern.cpp declarations/storage-linkage/static-then-extern-function.cpp

A later `extern` declaration keeps the internal linkage of an earlier
`static` declaration. A later function declaration without a storage
class also keeps the internal linkage. LNK-10 lists what is not valid.
:::

:::{rule} LNK-10
:kind: reject
:tests: declarations/storage-linkage/extern-then-static.cpp declarations/storage-linkage/function-then-static.cpp declarations/storage-linkage/qualified-static-after-extern.cpp

A `static` declaration must not follow a declaration with external
linkage (LNK-8).
:::

:::{rule} LNK-9
:kind: required
:tests: declarations/storage-linkage/block-function-declaration.cpp declarations/storage-linkage/block-variables-no-linkage.cpp declarations/storage-linkage/block-extern-internal.cpp declarations/storage-linkage/extern-in-block-hidden.cpp

A block-scope variable without `extern` has no linkage. A block-scope
function declaration, or a block-scope `extern` variable, refers to an
entity at namespace scope. Another translation unit can define that
entity.

A block-scope `extern` declaration redeclares the namespace-scope entity
of that name if one was declared earlier, and takes the linkage of that
entity, including internal linkage. A variable of the same name in an
outer block that hides the namespace-scope entity does not change this
result ({question}`Q-0105`).
:::

:::{rule} STO-11
:kind: reject
:tests: declarations/storage-linkage/anonymous-union-namespace.cpp declarations/storage-linkage/anonymous-union-static.cpp declarations/storage-linkage/anonymous-union-unnamed-namespace.cpp

An anonymous union at namespace scope must be declared `static`. The
rule also applies in an unnamed namespace ({question}`Q-0045`).
:::

## Runtime Behavior

:::{rule} STO-1
:kind: required
:tests: declarations/storage-linkage/automatic-lifetime.cpp declarations/storage-linkage/automatic-loop-iterations.cpp

A block-scope variable without `static`, `thread_local`, or `extern` has
{term}`automatic storage`. Each pass through its declaration creates a
new object, and the object ends when control leaves the block
({rule}`OBJ-3`).
:::

:::{rule} STO-2
:kind: required
:tests: declarations/storage-linkage/block-static-once.cpp declarations/storage-linkage/block-static-not-reached.cpp declarations/initialization/static/accessor-across-units.cpp

A block-scope `static` variable has static storage, with one object for
the program run. Its initialization occurs the first time control
reaches the declaration. If control never reaches the declaration, the
initialization does not occur.
:::

:::{rule} STO-3
:kind: required
:tests: declarations/storage-linkage/zero-initialized.cpp declarations/storage-linkage/zero-before-dynamic.cpp

A variable at namespace scope has static storage. The storage of each
variable with static storage is set to zero before other initialization.
:::

:::{rule} STO-8
:kind: required
:tests: declarations/storage-linkage/thread-local-block.cpp declarations/storage-linkage/thread-local-block-first-reach.cpp

A `thread_local` variable has one object for each thread *(threads
referenced)*. In a block, the object is initialized once in each thread,
the first time control reaches its declaration.
:::

:::{rule} STO-10
:kind: required
:tests: declarations/storage-linkage/static-destruction-order.cpp declarations/storage-linkage/static-destruction-block-order.cpp

When the program ends, objects with static storage duration are
destroyed. The object whose initialization completed last is destroyed
first. The rule includes block-scope `static` variables that the program
reached.
:::

## Undefined Behavior

:::{rule} STO-9
:kind: undefined
:tests: declarations/storage-linkage/recursive-static-init.cpp declarations/storage-linkage/recursive-static-init-constructor.cpp

If control reaches the declaration of a block-scope `static` variable
again while the variable's initialization is still running, the behavior
is undefined.
:::

## Implementation Requirements

:::{rule} LNK-6
:kind: no-diagnostic
:tests: declarations/storage-linkage/duplicate-variable.cpp declarations/storage-linkage/duplicate-function.cpp declarations/storage-linkage/inline-different-bodies.cpp

An entity with external linkage that is not `inline` must have one
definition in the program. All definitions of an `inline` entity must be
the same. An implementation need not diagnose a break of this rule.
{rule}`ODR-8` and {rule}`ODR-10` state the general rules.
:::

:::{rule} LNK-7
:kind: no-diagnostic
:tests: declarations/storage-linkage/missing-definition.cpp declarations/storage-linkage/unused-missing-definition.cpp

A variable or function that the program uses must have a definition. A
declaration that the program does not use needs no definition. An
implementation need not diagnose a break of this rule
({question}`Q-0011`). {rule}`ODR-3` says what counts as a use.
:::

## Examples

```cpp
int next() {
  static int counter = 0;      // initialized on the first call only
  return ++counter;            // 1, 2, 3, ...
}

static struct S { int v; };    // rejected

int shared = 5;
int read() {
  int shared = 1;
  { extern int shared; return shared; }   // 5
}

const int limit = 1;           // internal: each unit has its own
extern const int answer = 42;  // external

static int x = 1;
extern int x;                  // internal
extern int y;
static int y;                  // rejected
```

## Evidence

All eight compilers agree on 66 of 68 tests. The results are in
`tests/RESULTS.md`. 21 tests use two translation units (decision 0014).
EDG checks each unit and does not link.

- **LNK-6.** Each translation unit compiles without a diagnostic. The
  linker reports a second definition of a variable or function that is
  not `inline` ("multiple definition of"). Two different definitions of
  an `inline` function link without a diagnostic, and the body that
  executes changes. With Clang 22 and GCC 16 at `-O0`, both units call
  the definition that the linker sees first. At `-O2`, each unit uses
  its own definition. The test accepts each of these results.
- **LNK-6, LNK-7.** Linkers usually report a second definition of an
  entity that is not `inline`, but usually do not report different
  definitions of an `inline` entity.
- **LNK-7.** Each translation unit compiles without a diagnostic. Only
  the linker reports the missing definition ("undefined reference to").
- **STO-6.** Clang reports a storage class on a declaration that only
  defines a class as a pedantic warning. `-pedantic-errors` makes the
  warning an error. GCC and EDG reject such a declaration in each mode.
- **STO-11.** Clang accepts an anonymous union without `static` in an
  unnamed namespace. GCC and EDG reject the union
  ({question}`Q-0045`).
- **STO-9.** The C++ run-time library of Clang and GCC detects a second
  entry into the initialization of a block-scope `static` variable,
  also at `-O0` without a sanitizer. The program ends with an uncaught
  exception (`recursive_init_error`) and a nonzero status.
- **LNK-9.** EDG 7.0 rejects a block-scope `extern` declaration that
  redeclares an internal-linkage variable hidden by a local variable.
  Clang and GCC accept the declaration and read the namespace-scope
  variable ({question}`Q-0105`).

Inputs: clang-sema-decl, edg-decls, clang-diag-sema (see
`inputs/MANIFEST.md`).
