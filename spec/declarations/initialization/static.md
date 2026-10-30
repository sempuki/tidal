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
:tests: declarations/initialization/static/dynamic-in-order.cpp declarations/initialization/static/dynamic-reads-zero.cpp

Other namespace-scope variables are *dynamically initialized* before
`main` runs. In one {term}`translation unit`, they are initialized in the
order of their definitions. Before its initializer runs, a variable holds
zero (STO-3).
:::

## Implementation Requirements

:::{rule} SINIT-3
:kind: unspecified
:tests: declarations/initialization/static/across-units-order.cpp

The order of dynamic initialization between translation units is
unspecified. An initializer that reads a variable from another unit gets
its final value or zero.
:::

:::{rule} SINIT-4
:kind: unspecified
:tests: declarations/initialization/static/maybe-static.cpp

An implementation can do a dynamic initialization before the program
starts, if the variable gets the same value. It is unspecified if this
occurs. An earlier initializer that reads the variable gets zero or that
value.
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

All seven compilers agree on all 6 tests. The results are in
`tests/RESULTS.md`.

- **SINIT-3.** With Clang 22 and GCC 16 at `-O0` and `-O2`, the order
  follows the link order. The initializer in the first unit gets 0 when
  its unit is linked first. It gets 7 when its unit is linked second.
- **SINIT-4.** By default, Clang 22 and GCC 16 always initialize
  `int b = three();` at run time. `three` is an `inline` function that is
  not `constexpr`. An earlier reader gets 0. With `-fimplicit-constexpr`,
  GCC initializes `b` before the program starts, and the earlier reader
  gets 3. See {question}`Q-0014`.

Inputs: clang-sema-init, gcc-cp-typeck2, gcc-cp-init, edg-decl-inits,
clang-diag-sema (see `inputs/MANIFEST.md`).
