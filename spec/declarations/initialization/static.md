# Initialization of static and thread storage

Variables with static or thread {term}`storage duration` get their first
values in two steps. Some values are set before the program runs. Others
are set at run time, in an order that this page gives.

## Dynamic Semantics

:::{rule} SINIT-1
:kind: required
:tests: declarations/initialization/static/constant-before-dynamic.cpp declarations/initialization/static/constexpr-constant-initialized.cpp

A variable with static or thread storage is *constant-initialized* if its
initializer is a constant expression. It has its value before any code
runs. This includes the initializers of other variables.
:::

:::{rule} SINIT-2
:kind: required
:tests: declarations/initialization/static/dynamic-in-order.cpp declarations/initialization/static/dynamic-reads-zero.cpp declarations/initialization/static/inline-variables-ordered.cpp

Other namespace-scope variables are *dynamically initialized*. In one
{term}`translation unit`, they are initialized in the order of their
definitions. An `inline` variable is ordered only after an earlier
`inline` variable that comes first in every translation unit that
defines both. Its order with other variables is unspecified (SINIT-3). Before its initializer
runs, a variable holds zero (STO-3).
:::

## Implementation Requirements

:::{rule} SINIT-3
:kind: unspecified
:tests: declarations/initialization/static/across-units-order.cpp declarations/initialization/static/inline-order-unspecified.cpp

The order of dynamic initialization between translation units is
unspecified. An initializer that reads a variable from another unit gets
its final value or zero. The same holds between an `inline` variable and
a variable that is not `inline` in one translation unit, because another
unit can initialize the `inline` variable first.
:::

:::{rule} SINIT-4
:kind: unspecified
:tests: declarations/initialization/static/maybe-static.cpp

An implementation can do a dynamic initialization before the program
starts, if the variable gets the same value. It is unspecified if this
occurs. An earlier initializer that reads the variable gets zero or that
value.
:::

:::{rule} SINIT-5
:kind: impl-defined
:tests: declarations/initialization/static/before-main-observed.cpp declarations/initialization/static/deferred-initialization.cpp

It is implementation-defined if the dynamic initialization of a
namespace-scope variable is complete before the first statement of
`main`. An implementation can defer it until the first use of a function
or variable of the same translation unit.
:::

:::{rule} SINIT-6
:kind: impl-defined
:tests: declarations/initialization/static/thread-local-init-deferred.cpp

A `thread_local` variable at namespace scope with dynamic initialization
is initialized before the first statement of the thread, or when the
thread first uses it. The choice is implementation-defined. Clang and GCC
initialize it at the first use.
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
```

## Evidence

All seven compilers agree on all 11 tests. The results are in
`tests/RESULTS.md`.

- **SINIT-3.** With Clang 22 and GCC 16 at `-O0` and `-O2`, the order
  follows the link order. The initializer in the first unit gets 0 when
  its unit is linked first. It gets 7 when its unit is linked second.
- **SINIT-4.** By default, Clang 22 and GCC 16 always initialize
  `int b = three();` at run time. `three` is an `inline` function that is
  not `constexpr`. An earlier reader gets 0. With `-fimplicit-constexpr`,
  GCC initializes `b` before the program starts, and the earlier reader
  gets 3. See {question}`Q-0014`.
- **SINIT-3, `inline` order.** The test accepts 8 and 1. The reviewer
  summary reports that Clang gives 1 in both link orders and GCC gives 8
  or 1 with the link order.

Inputs: clang-sema-init, edg-decl-inits, clang-diag-sema (see
`inputs/MANIFEST.md`).
