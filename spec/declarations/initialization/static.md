# Initialization of static and thread storage

When variables with static or thread storage get their initial values:
which initialization happens before the program runs, which happens at run
time, and in what order. Rules are prefixed `SINIT`.

Depends on: [declarators](../declarators.md), [storage duration](../storage-linkage.md),
[value category](../../glossary.md#value-category), conversions *(referenced)*,
classes *(referenced)*, [constant evaluation](../../glossary.md#constant-evaluation).

Inputs: clang-sema-init, gcc-cp-typeck2, gcc-cp-init, edg-decl-inits,
clang-diag-sema (see `inputs/MANIFEST.md`).

## Rules

### SINIT-1 (required)

A variable with static or thread storage whose initializer is a constant
expression is *constant-initialized*. It has its value before any code runs,
including the initializers of other variables.

    int read_later();
    int early = read_later();   // sees 5
    int later = 5;
    int read_later() { return later; }

Tests: declarations/initialization/static/constant-before-dynamic.cpp, declarations/initialization/static/constexpr-constant-initialized.cpp

### SINIT-2 (required)

Other variables at namespace scope are *dynamically initialized*, before
`main` runs, in the order their definitions appear within one translation
unit. Until its initializer runs, a variable holds zero (STO-3), and an
earlier initializer that reads it sees zero.

Tests: declarations/initialization/static/dynamic-in-order.cpp, declarations/initialization/static/dynamic-reads-zero.cpp

### SINIT-3 (unspecified)

The order of dynamic initialization between translation units is
unspecified. An initializer that reads a variable from another translation
unit may see its final value or zero.

Tests: declarations/initialization/static/across-units-order.cpp

### SINIT-4 (unspecified)

An implementation may perform a dynamic initialization before the program
starts, as if it were constant initialization, when doing so gives the
variable the value it would get anyway. Whether it does is unspecified, so
an earlier initializer that reads the variable may see either zero or that
value (question Q-0014).

Tests: declarations/initialization/static/maybe-static.cpp

## Evidence

All seven compilers agree on all 6 tests. Results are in
`tests/RESULTS.md`.

- **SINIT-3.** With Clang 22 and GCC 16 at `-O0` and `-O2`, the order
  follows link order. The first unit's initializer sees 0 when its unit is
  linked first and 7 when it is linked second.
- **SINIT-4.** Clang 22 and GCC 16 by default always run `int b =
  three();` (an `inline` function that is not `constexpr`) at program
  start, so an earlier reader sees 0. GCC with `-fimplicit-constexpr`
  initializes it before the program starts, and the earlier reader sees 3.

## Divergences

None.
