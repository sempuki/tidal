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

## Constraints

:::{rule} CAST-1
:kind: required
:tests: expressions/casts/static-cast-arithmetic.cpp expressions/casts/static-cast-enum.cpp expressions/casts/static-cast-void-pointer.cpp expressions/casts/static-cast-to-void.cpp expressions/casts/static-cast-rvalue-reference.cpp expressions/casts/static-cast-explicit-ctor.cpp expressions/casts/static-cast-float-to-enum.cpp expressions/casts/static-cast-enum-fixed-in-range.cpp expressions/casts/static-cast-null-to-pointer.cpp

`static_cast` can do these conversions:

- any conversion that a direct initialization can do, including one
  through an `explicit` constructor or an `explicit` conversion function,
- an enumeration to an integer, and an integer or a floating value to an
  enumeration,
- a null pointer constant to a pointer type,
- a pointer to `void` to a pointer to an object type,
- any expression to `void`,
- an lvalue to an rvalue reference.
:::

:::{rule} CAST-2
:kind: reject
:tests: expressions/casts/static-cast-unrelated-pointer.cpp expressions/casts/static-cast-removes-const.cpp expressions/casts/static-cast-pointer-to-integer.cpp expressions/casts/static-cast-integer-to-pointer.cpp expressions/casts/static-cast-function-to-object-pointer.cpp expressions/casts/static-cast-downcast-private-base.cpp

`static_cast` must not convert between unrelated pointer types. It must
not remove a cv-qualifier. It must not convert a pointer to an integer,
or an integer other than a null pointer constant to a pointer. It must
not convert between a pointer to a function and a pointer to an object,
or to a derived class through a base that is not accessible.
:::

:::{rule} CAST-4
:kind: reject
:tests: expressions/casts/const-cast-changes-type.cpp expressions/casts/const-cast-value.cpp expressions/casts/const-cast-member-pointer.cpp expressions/casts/const-cast-function-pointer.cpp

`const_cast` can only add or remove cv-qualifiers. Its target type must be
a pointer, a pointer to data member, or a reference. It can change each
level of a multi-level pointer. It must not change a pointer to a
function.
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

:::{rule} CAST-13
:kind: required
:tests: expressions/casts/decay-copy-auto.cpp expressions/casts/decay-copy-type.cpp

`auto(e)` and `auto{e}` give a prvalue copy of `e`. Its type is the type
of `e` without cv-qualifiers or a reference, and with arrays and
functions converted to pointers.
:::

:::{rule} CAST-15
:kind: required
:tests: expressions/casts/static-cast-downcast.cpp

`static_cast` converts a pointer or reference to a base `B` to a pointer
or reference to a derived class `D`. `B` must be an accessible,
unambiguous base of `D` that is not virtual.
:::

:::{rule} CAST-17
:kind: required
:tests: expressions/casts/c-style-private-base.cpp

A cast of the form `(T)e` can convert to or from a base class that is not
accessible. `static_cast` cannot.
:::

:::{rule} CAST-18
:kind: required
:tests: expressions/casts/functional-empty-parens.cpp expressions/casts/functional-braced-unknown-bound.cpp

`T()` value-initializes a prvalue of type `T`. For a scalar type the value
is zero, and `void()` is a `void` expression. `T{...}` with an array type
of unknown bound gives an array with one element for each value.
:::

## Dynamic Semantics

:::{rule} CAST-3
:kind: required
:tests: expressions/casts/const-cast-pointer.cpp expressions/casts/const-cast-reference.cpp

The result of `const_cast` refers to the same object as its operand.
:::

:::{rule} CAST-6
:kind: required
:tests: expressions/casts/reinterpret-round-trip.cpp expressions/casts/reinterpret-unrelated-pointer.cpp expressions/casts/reinterpret-reference.cpp expressions/casts/reinterpret-nullptr-to-integer.cpp

`reinterpret_cast` can convert a pointer to a large enough integer type.
Converting that integer back gives the original pointer.
`reinterpret_cast` can also convert between unrelated pointer types. A
cast to a reference type `T&` gives an lvalue that refers to the operand
as type `T`. A null pointer value of type `std::nullptr_t` converts to an
integer type.
:::

## Undefined Behavior

:::{rule} CAST-5
:kind: undefined
:tests: expressions/casts/const-cast-write-const-object.cpp

A write through the result of `const_cast` to an object that is `const` is
undefined (CV-5).
:::

:::{rule} CAST-14
:kind: undefined
:tests: expressions/casts/static-cast-enum-out-of-range-constant.cpp expressions/casts/static-cast-downcast-wrong-object-constant.cpp

The behavior is undefined in these cases:

- `static_cast` converts a value outside the range of an enumeration
  without a fixed underlying type ({question}`Q-0017`).
- A `static_cast` to a derived class `D` applies to an object that is not a
  base subobject of a `D`.
:::

## Implementation Requirements

:::{rule} CAST-12
:kind: impl-defined
:tests: expressions/casts/reinterpret-function-to-object.cpp

It is implementation-defined if `reinterpret_cast` converts between a
pointer to a function and a pointer to an object type.
:::

:::{rule} CAST-16
:kind: impl-defined
:tests: expressions/casts/reinterpret-pointer-integer-mapping.cpp

The integer value that `reinterpret_cast` gives for a pointer is
implementation-defined. On the x86-64 Linux targets in the matrix, the
integer is the address, and the round trip gives the original pointer.
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

All seven compilers agree on 44 of 45 tests. The results are in
`tests/RESULTS.md`.

- CAST-14: GCC and EDG accept an out-of-range enumeration value during
  constant evaluation. Clang rejects it ({question}`Q-0017`).
- CAST-5, CAST-8: all compilers reject these during constant evaluation.
- CAST-12: every compiler in the matrix accepts the conversion and the
  round trip works.

Inputs: clang-sema-expr, clang-diag-sema.
