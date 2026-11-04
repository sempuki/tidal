# Initialization of static and thread storage

Variables with static or thread {term}`storage duration` get their first
values in two steps. Some values are set before the program runs. Others
are set at run time, in an order that this page gives.

## Principles

:::{principle} SINIT-P1
:rules: SINIT-1 SINIT-2 SINIT-7

Static storage is initialized in two steps.

```text
+--------------------------------------------------+
| 1. Before any code runs                          |
|    every variable is zero                        |
|    each variable with a constant initializer     |
|    gets its value                                |
+--------------------------------------------------+
                         |
                         v
+--------------------------------------------------+
| 2. At run time, in each translation unit         |
|    the remaining variables are initialized       |
|    in the order of their definitions             |
+--------------------------------------------------+
```

So a later initializer in the same unit can read an earlier variable and
get its final value.
:::

:::{principle} SINIT-P2
:rules: SINIT-3 SINIT-4 SINIT-5 SINIT-6

Across translation units there is no order, so an initializer that reads
a variable from another unit can get its final value or zero. An
`inline` variable is defined in several units, so its place in any one
unit's order is only partly known. An implementation can do a dynamic
initialization early, when the value is the same, or late, up to the
first use of something in that unit. A `thread_local` variable can wait
for its first use.

Two forms make a variable safe to read from another unit's initializer:

| Form | Why the order does not matter |
|---|---|
| a {term}`constant initializer`, checked with `constinit` | The variable has its value before any code runs (SINIT-1, {rule}`CINIT-1`). |
| a block-scope `static` variable in a function that returns a reference to it | The first call initializes the variable ({rule}`STO-2`), so every caller gets a finished object. |

The initializer of such a block-scope variable must not call its own
function again ({rule}`STO-9`).
:::

## Runtime Behavior

:::{rule} SINIT-1
:kind: required
:tests: declarations/initialization/static/constant-before-dynamic.cpp declarations/initialization/static/constexpr-constant-initialized.cpp declarations/initialization/static/consteval-branch-in-initializer.cpp declarations/initialization/static/constexpr-constructor-constant-initialized.cpp declarations/initialization/static/const-variable-in-initializer.cpp declarations/initialization/static/reference-constant-initialized.cpp

A variable with static or thread storage is *constant-initialized* if
its initializer is a constant expression. Such a variable has its value
before any code runs, including the initializers of other variables.
The initializer can call a `constexpr` constructor, read a `const`
integer variable with a constant initializer, or bind a reference to a
variable with static storage.
:::

:::{rule} SINIT-2
:kind: required
:tests: declarations/initialization/static/dynamic-in-order.cpp declarations/initialization/static/dynamic-reads-zero.cpp declarations/initialization/static/inline-variables-ordered.cpp declarations/initialization/static/dynamic-reads-zero-other-scalars.cpp

Other namespace-scope variables are *dynamically initialized*. In one
{term}`translation unit`, they are initialized in the order of their
definitions. An `inline` variable is ordered only after an earlier
`inline` variable that comes first in every translation unit that
defines both. The order of an `inline` variable with other variables is
unspecified (SINIT-3). Before its initializer runs, a variable of
{term}`scalar type` holds zero, so a pointer is null and a `bool` is `false`
(STO-3). A member of a class object has no value
until the initialization of the object gives the member one
({rule}`OBJ-2`, {question}`Q-0175`).
:::

:::{rule} SINIT-7
:kind: required
:tests: declarations/initialization/static/static-member-in-order.cpp declarations/initialization/static/static-member-order-not-class-order.cpp

A static data member that is defined outside its class takes its place
in the order of SINIT-2 at its definition. The place of the class
definition does not matter.
:::

## Implementation Requirements

:::{rule} SINIT-3
:kind: unspecified
:tests: declarations/initialization/static/across-units-order.cpp declarations/initialization/static/inline-order-unspecified.cpp declarations/initialization/static/accessor-across-units.cpp declarations/initialization/static/template-member-unordered.cpp

The order of dynamic initialization between translation units is
unspecified. An initializer that reads a variable from another unit gets
its final value or zero. The same holds in one translation unit in
these cases:

- between an `inline` variable and a variable that is not `inline`,
  because another unit can initialize the `inline` variable first,
- between a variable that is a {term}`specialization` of a template, such
  as a class template's static data member, and any other variable.
:::

:::{rule} SINIT-4
:kind: unspecified
:tests: declarations/initialization/static/maybe-static.cpp declarations/initialization/static/maybe-static-non-const-read.cpp

An implementation can do a dynamic initialization before the program
starts, if the variable gets the same value. Whether the early
initialization occurs is unspecified. An earlier initializer that reads
the variable gets zero or that value.
:::

:::{rule} SINIT-5
:kind: impl-defined
:tests: declarations/initialization/static/before-main-observed.cpp declarations/initialization/static/deferred-initialization.cpp declarations/initialization/static/unit-not-named.cpp

Whether the dynamic initialization of a namespace-scope variable is
complete before the first statement of `main` is implementation-defined.
An implementation can defer the initialization until the first use of a
function or variable of the same translation unit. Clang and GCC finish
every such initialization before `main` starts, also in a unit that no
other unit names.
:::

:::{rule} SINIT-6
:kind: impl-defined
:tests: declarations/initialization/static/thread-local-init-deferred.cpp declarations/initialization/static/thread-local-unused.cpp

A `thread_local` variable at namespace scope with dynamic initialization
is initialized before the first statement of the thread, or when the
thread first uses the variable. The choice is implementation-defined.
Clang and GCC initialize the variable at the first use, also when the
variable has a class type with a constructor ({rule}`OBJ-11`).
:::

## Examples

```cpp
int read_later();
int early = read_later();   // 5: later is constant-initialized
int later = 5;
int read_later() { return later; }

volatile int source = 7;
int read_y();
int first = read_y();       // 0: y is not initialized yet
int y = source;             // dynamic initialization
int read_y() { return y; }

struct Point { int x; Point() : x(5) {} };
int read_p();
int early_x = read_p();     // undefined: p.x has no value yet
Point p;
int read_p() { return p.x; }

constexpr int mode() {
  if consteval { return 1; } else { return 2; }
}
int offset = 0;
int fixed = mode();         // 1: constant-initialized
int moved = mode() + offset;  // 2: offset is not a constant

const int limit = 4;
int twice = limit * 2;      // constant-initialized: limit is const
int next();
struct Registry { static int id; };
int Registry::id = next();  // ordered here, at its definition
```

A variable that initializers in other units read, in two safe forms:

```cpp
// table.cpp
int load_size();
int& table_size() {
  static int size = load_size();  // runs at the first call (STO-2)
  return size;
}
constinit int table_limit = 64;   // set before any code runs

// user.cpp
int& table_size();
extern constinit int table_limit;
int slots = table_size() * 2;     // the final value, in any unit order
int cap = table_limit;            // 64, in any unit order
```

## Evidence

All eight compilers agree on all 23 tests. The results are in
`tests/RESULTS.md`.

- **SINIT-3.** With Clang 22 and GCC 16 at `-O0` and `-O2`, the order
  follows the link order. The initializer in the first unit gets 0 when
  its unit is linked first, and 7 when its unit is linked second.
- **SINIT-4.** By default, Clang 22 and GCC 16 always initialize
  `int b = three();` at run time. `three` is an `inline` function that
  is not `constexpr`. An earlier reader gets 0. With
  `-fimplicit-constexpr`, GCC initializes `b` before the program starts,
  and the earlier reader gets 3. See {question}`Q-0014`. Every compiler
  at `-O0` and `-O2` initializes `int v = base * 2;` at run time when
  `base` is not `const`, and an earlier reader gets 0. With a `const`
  `base`, `v` is constant-initialized (SINIT-1), and the earlier reader
  gets 8.
- **SINIT-3, safe forms.** A block-scope `static` behind a function and
  a `constinit` variable give the final value to an initializer in
  another unit. Clang 22 and GCC 16 give the same result in both link
  orders, at `-O0` and `-O2` (`accessor-across-units.cpp`).
- **SINIT-5.** Clang 22 and GCC 16 run the dynamic initializer of a
  unit that `main` never names before `main` starts, in both link
  orders (`unit-not-named.cpp`).
- **SINIT-3, templates.** A class template's static data member
  with a dynamic initializer is initialized before the variables of its
  unit on Clang 22, 18, and the host Clang at `-O0`. GCC, and Clang 22
  at `-O2`, initialize it after them
  (`template-member-unordered.cpp`).
- **SINIT-3, `inline` order.** The test accepts 8 and 1. The reviewer
  summary reports that Clang gives 1 in both link orders and GCC gives 8
  or 1 with the link order.

- **SINIT-2, class members.** A dynamic initializer that reads a member
  of a later class-type variable gets 0 at run time with Clang 22 and
  GCC 16, at `-O0` and `-O2`, and with the address and undefined
  behavior sanitizers. OBJ-2 allows this result. In constant
  evaluation, every compiler rejects the same read of a member before
  its constructor runs ({question}`Q-0175`).
- **SINIT-1 and `if consteval`.** The meaning of an initializer that
  contains `if consteval` depends on its constant evaluation. If the
  evaluation succeeds, the constant branch gives the value. If the
  evaluation fails, the initializer runs at run time and takes the other
  branch. With every compiler, `fixed` is 1 and `moved` is 2.
- SINIT-5, SINIT-6 with `-m32`: the matrix runs no program built with
  `-m32`, because the host compilers have no 32-bit C library. In the
  writer's probes, the run tests of the two rules give the same
  results with `-m32` in the GCC 14, Clang 18, and Clang 22 containers.

Inputs: clang-sema-init, edg-decl-inits, clang-diag-sema (see
`inputs/MANIFEST.md`).
