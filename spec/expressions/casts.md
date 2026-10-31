# Casts

A cast converts a value to a named type. The casts are `static_cast`,
`const_cast`, `reinterpret_cast`, and the cast notations `(T)e`, `T(e)`,
and `T{e}`. `dynamic_cast` applies to classes *(referenced)*.

## Syntax

```{eval-rst}
.. productionlist:: expr_casts
   named_cast: ("static_cast" | "const_cast" | "reinterpret_cast") "<" `type` ">" "(" `expression` ")"
   cast_notation: "(" `type` ")" `expression`
                : `type` "(" [`expression_list`] ")"
                : `type` `braced_init`
```

## Static Semantics

:::{rule} CAST-1
:kind: required
:tests: expressions/casts/static-cast-arithmetic.cpp expressions/casts/static-cast-enum.cpp expressions/casts/static-cast-void-pointer.cpp expressions/casts/static-cast-to-void.cpp expressions/casts/static-cast-rvalue-reference.cpp

`static_cast` can do these conversions:

- an implicit conversion, including arithmetic conversions,
- an enumeration to an integer, and an integer to an enumeration,
- a pointer to `void` to a pointer to an object type,
- any expression to `void`,
- an lvalue to an rvalue reference.
:::

:::{rule} CAST-2
:kind: reject
:tests: expressions/casts/static-cast-unrelated-pointer.cpp expressions/casts/static-cast-removes-const.cpp expressions/casts/static-cast-pointer-to-integer.cpp

`static_cast` must not convert between unrelated pointer types. It must
not remove a cv-qualifier. It must not convert a pointer to an integer.
:::

:::{rule} CAST-4
:kind: reject
:tests: expressions/casts/const-cast-changes-type.cpp expressions/casts/const-cast-value.cpp

`const_cast` can only add or remove cv-qualifiers. Its target type must be
a pointer or a reference.
:::

:::{rule} CAST-7
:kind: reject
:tests: expressions/casts/reinterpret-integer-too-small.cpp expressions/casts/reinterpret-removes-const.cpp

`reinterpret_cast` must not convert a pointer to an integer type that is
too small to hold it. It must not remove a cv-qualifier.
:::

:::{rule} CAST-8
:kind: reject
:tests: expressions/casts/reinterpret-not-constant.cpp

A constant expression must not contain `reinterpret_cast`.
:::

:::{rule} CAST-9
:kind: required
:tests: expressions/casts/c-style-removes-const-and-type.cpp expressions/casts/c-style-static.cpp

`(T)e` and `T(e)` with one operand try these casts in order. They use the
first cast that is valid:

1. `const_cast`,
2. `static_cast`,
3. `static_cast` and then `const_cast`,
4. `reinterpret_cast`,
5. `reinterpret_cast` and then `const_cast`.
:::

:::{rule} CAST-10
:kind: reject
:tests: expressions/casts/functional-cast-several-arguments.cpp expressions/casts/functional-cast-several-scalar.cpp

`T(a, b)` with several operands is valid only for a class type `T`
*(classes referenced)*.
:::

:::{rule} CAST-11
:kind: reject
:tests: expressions/casts/braced-cast-narrowing.cpp expressions/casts/braced-cast-ok.cpp

`T{e}` must not need a narrowing conversion (LIST-2).
:::

## Dynamic Semantics

:::{rule} CAST-3
:kind: required
:tests: expressions/casts/const-cast-pointer.cpp expressions/casts/const-cast-reference.cpp

The result of `const_cast` refers to the same object as its operand.
:::

:::{rule} CAST-6
:kind: required
:tests: expressions/casts/reinterpret-round-trip.cpp expressions/casts/reinterpret-unrelated-pointer.cpp

`reinterpret_cast` can convert a pointer to a large enough integer type.
Converting that integer back gives the original pointer.
`reinterpret_cast` can also convert between unrelated pointer types.
:::

## Undefined Behavior

:::{rule} CAST-5
:kind: undefined
:tests: expressions/casts/const-cast-write-const-object.cpp

A write through the result of `const_cast` to an object that is `const` is
undefined (CV-5).
:::

## Implementation Requirements

:::{rule} CAST-12
:kind: impl-defined
:tests: expressions/casts/reinterpret-function-to-object.cpp

It is implementation-defined if `reinterpret_cast` converts between a
pointer to a function and a pointer to an object type.
:::

## Examples

```cpp
static_cast<int>(3.9);          // 3
enum class E { A = 2 };
static_cast<E>(2);              // E::A
int x = 0;
static_cast<long*>(&x);         // rejected
reinterpret_cast<long*>(&x);    // fine

const int c = 1;
int* p = const_cast<int*>(&c);
*p = 2;                         // undefined

const int* cp = &x;
unsigned* u = (unsigned*)cp;    // reinterpret_cast, then const_cast
int{2.5};                       // rejected: narrowing
```

## Evidence

All seven compilers agree on all 25 tests. The results are in
`tests/RESULTS.md`.

- CAST-5, CAST-8: all compilers reject these during constant evaluation.
- CAST-12: every compiler in the matrix accepts the conversion and the
  round trip works.

Inputs: clang-sema-expr, clang-diag-sema.
