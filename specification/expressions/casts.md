# Casts

A cast converts a value to a named type. The casts are `static_cast`,
`const_cast`, `reinterpret_cast`, and the cast notations `(T)e`, `T(e)`,
and `T{e}`. `dynamic_cast` applies to classes ({rule}`VIRT-10`).

## Principles

:::{principle} CAST-P1
:rules: CAST-1 CAST-2 CAST-4 CAST-7 CAST-15 CAST-3 CAST-19

Each named cast has one job, so a reader sees what a cast is for.

| Cast | Job |
|---|---|
| `static_cast` | The conversions that an initialization could do. The inverses of implicit conversions that are safe to undo, such as base to derived and `void*` to an object pointer. |
| `const_cast` | Changes qualifiers and nothing else. The result is the same object. |
| `reinterpret_cast` | Changes how storage is read and nothing else. |

A cast that would need two jobs, such as a change of qualifiers inside a
`static_cast`, is rejected, because the reader could not tell which was
intended.
:::

:::{principle} CAST-P2
:rules: CAST-9 CAST-10 CAST-11 CAST-13 CAST-18
:exceptions: CAST-17

The older cast notations are defined by the named casts.

| Notation | Meaning |
|---|---|
| `(T)e`, `T(e)` | Tries the named casts in a fixed order and takes the first that works |
| `T{e}` | List initialization, so a narrowing conversion is not valid |
| `T(a, b)` | Construction |
| `auto(e)` | A plain prvalue copy |
| `T()` | A value-initialized prvalue |

Only the old notation can convert to or from a base that is not
accessible.
:::

:::{principle} CAST-P3
:rules: CAST-6 CAST-8 CAST-12 CAST-16

`reinterpret_cast` promises only that a round trip gives back the
original value. The implementation decides what the bits mean in
between. So the integer that a pointer becomes, and whether a function
pointer can become an object pointer at all, are implementation-defined.
The cast is not valid in {term}`constant evaluation`, because constant
evaluation has no bits to show. The rules on raw storage say when a program can read or
write an object through the result ({rule}`STG-14`, {rule}`STG-16`,
{rule}`STG-29`).
:::

:::{principle} CAST-P4
:rules: CAST-5 CAST-14

A cast changes the type through which an object is seen, and never the
object. So in these uses the cast is valid, and the use of its result
is undefined:

- writing through a `const_cast` to an object that is `const`,
- treating an object as a derived class that it is not,
- giving an enumeration a value it cannot hold.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   cast_expr: `unary_expr` | "(" `type_id` ")" `cast_expr`
   named_cast: `cast_keyword` "<" `type_id` ">" "(" `expression` ")"
   cast_keyword: "static_cast" | "const_cast" | "reinterpret_cast"
   functional_cast: `cast_type` "(" [`expression_list`] ")"
                  : | `cast_type` `braced_init`
   cast_type: [`nested_name`] `identifier` | `decltype_spec` | "auto"
            : | "bool" | "void" | "int" | "short" | "long" | "signed" | "unsigned"
            : | "float" | "double" | "char" | "wchar_t" | "char8_t" | "char16_t" | "char32_t"
```

## Constraints

:::{rule} CAST-1
:kind: required
:tests: expressions/casts/static-cast-arithmetic.cpp expressions/casts/static-cast-enum.cpp expressions/casts/static-cast-void-pointer.cpp expressions/casts/static-cast-to-void.cpp expressions/casts/static-cast-rvalue-reference.cpp expressions/casts/static-cast-explicit-ctor.cpp expressions/casts/static-cast-float-to-enum.cpp expressions/casts/static-cast-enum-fixed-in-range.cpp expressions/casts/static-cast-null-to-pointer.cpp

`static_cast` can do these conversions:

- any conversion that a direct initialization can do, including one
  through an `explicit` constructor or an `explicit` conversion
  function,
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

`static_cast` must not:

- convert between unrelated pointer types,
- remove a {term}`cv-qualifier`,
- convert a pointer to an integer,
- convert an integer other than a null pointer constant to a pointer,
- convert between a pointer to a function and a pointer to an object,
- convert a pointer to a base class to a pointer to a derived class,
  if the base is not accessible at the cast (for example, a private
  base).
:::

:::{rule} CAST-4
:kind: reject
:tests: expressions/casts/const-cast-changes-type.cpp expressions/casts/const-cast-value.cpp expressions/casts/const-cast-member-pointer.cpp expressions/casts/const-cast-function-pointer.cpp

`const_cast` can only add or remove cv-qualifiers. The target type must
be one of these:

- a pointer,
- a pointer to data member,
- a reference.

`const_cast` can change each level of a multi-level pointer, but must
not change a pointer to a function.
:::

:::{rule} CAST-7
:kind: reject
:tests: expressions/casts/reinterpret-integer-too-small.cpp expressions/casts/reinterpret-removes-const.cpp

`reinterpret_cast` must not convert a pointer to an integer type that is
too small to hold the pointer. The cast must not remove a cv-qualifier.
:::

:::{rule} CAST-8
:kind: reject
:tests: expressions/casts/reinterpret-not-constant.cpp expressions/casts/reinterpret-not-constant-function.cpp

A constant expression must not contain `reinterpret_cast`.
:::

:::{rule} CAST-9
:kind: required
:tests: expressions/casts/c-style-removes-const-and-type.cpp expressions/casts/c-style-static.cpp

`(T)e` and `T(e)` with one operand try these casts in order and use the
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
({rule}`CTOR-10`).
:::

:::{rule} CAST-11
:kind: reject
:tests: expressions/casts/braced-cast-narrowing.cpp expressions/casts/braced-cast-ok.cpp

`T{e}` must not need a narrowing conversion (LIST-2).
:::

:::{rule} CAST-13
:kind: required
:tests: expressions/casts/decay-copy-auto.cpp expressions/casts/decay-copy-type.cpp expressions/casts/decay-copy-class.cpp

`auto(e)` and `auto{e}` give a prvalue copy of `e`. The type of the copy
is the type of `e` without cv-qualifiers or a reference, and with arrays
and functions converted to pointers.
:::

:::{rule} CAST-15
:kind: required
:tests: expressions/casts/static-cast-downcast.cpp expressions/casts/static-cast-downcast-rvalue-reference.cpp

`static_cast` converts a pointer or reference to a base `B` to a pointer
or reference to a derived class `D`. `B` must be an accessible,
unambiguous base of `D` that is not virtual.
:::

:::{rule} CAST-17
:kind: required
:tests: expressions/casts/c-style-private-base.cpp expressions/casts/c-style-private-base-downcast.cpp

A cast of the form `(T)e` can convert to or from a base class that is
not accessible. `static_cast` cannot.
:::

:::{rule} CAST-18
:kind: required
:tests: expressions/casts/functional-empty-parens.cpp expressions/casts/functional-braced-unknown-bound.cpp

`T()` value-initializes a prvalue of type `T`. For a {term}`scalar type` the
value is zero, and `void()` is a `void` expression. `T{...}` with an
array type of unknown bound gives an array with one element for each
value.
:::

:::{rule} CAST-19
:kind: reject
:tests: expressions/casts/static-cast-unrelated-pointers.cpp expressions/casts/static-cast-integer-to-pointer.cpp expressions/casts/c-style-no-valid-cast.cpp

A `static_cast` must do one of the conversions that CAST-1 lists. A
cast `(T)e` or `T(e)` with one operand must match one of the steps of
CAST-9. CAST-2 names the most common conversions that `static_cast`
cannot do (decision 0042).
:::

## Runtime Behavior

:::{rule} CAST-3
:kind: required
:tests: expressions/casts/const-cast-pointer.cpp expressions/casts/const-cast-reference.cpp

The result of `const_cast` refers to the same object as its operand.
:::

:::{rule} CAST-6
:kind: required
:tests: expressions/casts/reinterpret-round-trip.cpp expressions/casts/reinterpret-unrelated-pointer.cpp expressions/casts/reinterpret-reference.cpp expressions/casts/reinterpret-nullptr-to-integer.cpp expressions/casts/reinterpret-function-pointer-round-trip.cpp expressions/casts/integer-to-pointer-address.cpp

`reinterpret_cast` can do these conversions:

- a pointer to a large enough integer type. Converting that integer
  back gives the original pointer.
- an integer or an unscoped enumeration to a pointer. CAST-16 gives the
  address of the result.
- between unrelated pointer types.
- between two pointer to function types. Converting back gives the
  original pointer. A call through the converted pointer follows
  {rule}`PFX-19`.
- to a reference type `T&`. The result is an lvalue that refers to the
  operand as type `T`.
- a null pointer value of type `std::nullptr_t` to an integer type.

A cast of a pointer into a byte array gives a pointer to a byte, also
when the array hosts an object of the target type there. Such a result
does not reach the hosted object ({rule}`STG-29`).
:::

## Undefined Behavior

:::{rule} CAST-5
:kind: undefined
:tests: expressions/casts/const-cast-write-const-object.cpp expressions/casts/const-cast-write-const-object-sanitizer.cpp expressions/primary/string-modify-sanitizer.cpp

A write through the result of `const_cast` to an object that is `const`
is undefined (CV-5). {rule}`PRIM-8` is the specific case of CAST-5 for
the characters of a string literal (decision 0052).
:::

:::{rule} CAST-14
:kind: undefined
:tests: expressions/casts/static-cast-enum-out-of-range-constant.cpp expressions/casts/static-cast-downcast-wrong-object-constant.cpp classes/derived/static-cast-downcast-wrong-object-sanitizer.cpp declarations/enumerations/cast-out-of-range-sanitizer.cpp

The behavior is undefined in these cases:

- `static_cast` converts a value outside the range of an enumeration
  without a fixed underlying type ({rule}`ENUM-6`, {question}`Q-0017`).
- A `static_cast` to a derived class `D` applies to an object that is
  not a base {term}`subobject` of a `D`.
:::

## Implementation Requirements

:::{rule} CAST-12
:kind: impl-defined
:tests: expressions/casts/reinterpret-function-to-object.cpp expressions/casts/reinterpret-function-to-char-pointer.cpp expressions/casts/reinterpret-function-to-object-ilp32.cpp

Each implementation chooses whether `reinterpret_cast` can turn a
pointer to a function into a pointer to an object type, and back. The
matrix compilers choose in this way:

| Compiler | x86-64 Linux | `-m32` |
|---|---|---|
| Clang, GCC, EDG | accepts both directions, and the round trip gives the original pointer | the same |
:::

:::{rule} CAST-16
:kind: impl-defined
:tests: expressions/casts/reinterpret-pointer-integer-mapping.cpp expressions/casts/integer-to-pointer-address.cpp expressions/casts/pointer-sized-integer.cpp expressions/casts/pointer-sized-integer-ilp32.cpp foundations/storage/pointer-bytes-are-address.cpp

The integer that `reinterpret_cast` gives for a pointer, and the pointer
that it gives for an integer, are implementation-defined. On the x86-64
Linux targets in the matrix, the integer is the address in both
directions. The integer types that hold a pointer are these:

| Target | Size of a pointer | Integer types of that size |
|---|---|---|
| x86-64 | 8 | `std::size_t` ({rule}`LSUP-4`), `unsigned long`, `long` |
| `-m32` | 4 | `std::size_t`, `unsigned long`, `long`, `unsigned`, `int` |
:::

## Examples

% tidal-example: reject
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

using size_t = decltype(sizeof 0);
size_t at = reinterpret_cast<size_t>(&x);  // holds the address on every target
bool aligned = at % alignof(int) == 0;     // true
auto* port = reinterpret_cast<volatile unsigned*>(size_t{0x4000'1000});
```

## Evidence

All eight compilers agree on 61 of 62 tests. The results are in
`tests/RESULTS.md`.

### Divergences

- {question}`Q-0017` (CAST-14, ENUM-6; open): whether converting a
  value outside an enumeration's range is undefined. It is, so constant
  evaluation rejects the conversion, as Clang does. GCC and EDG accept
  the value during constant evaluation.
- {question}`Q-0325` (CAST-16; open): whether an object exists at an
  address that the program did not obtain from an object, such as a
  device register. The platform decides, and Tidal sets no requirement.
  No family differs.

### Split tests

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `static-cast-enum-out-of-range-constant.cpp` | CAST-14 | reject | reject | accept | accept | {question}`Q-0017` |

### Notes

- CAST-5, CAST-8: all compilers reject these operations during constant
  evaluation.
- CAST-5: at run time, `-fsanitize=address` in Clang and GCC stops a
  write through `const_cast` to a `const` object with static storage
  duration. The object is in storage that cannot be written, so the
  write faults (`const-cast-write-const-object-sanitizer.cpp`). No
  sanitizer stops a write to a `const` automatic object
  ({rule}`CV-5`).
- CAST-14: at run time, `-fsanitize=vptr` in Clang and GCC stops a
  `static_cast` downcast to the wrong derived class of a polymorphic
  base. `-fsanitize=undefined` stops the read of an enumeration
  variable that holds a value outside its range. The sanitizer reports
  the read, after the conversion. The tests are
  `static-cast-downcast-wrong-object-sanitizer.cpp` and
  `cast-out-of-range-sanitizer.cpp`.
- CAST-12: every compiler in the matrix accepts the conversion and the
  round trip works. A pointer to a function and a pointer to an object
  have the same size on both targets.
- CAST-12, CAST-16 with `-m32`: the matrix runs no program built with
  `-m32`, because the host compilers have no 32-bit C library. The
  `-m32` tests check that the compilers accept the conversions. In the
  writer's probes, the run tests of the two rules give the same results
  with `-m32` in the GCC 14, Clang 18, and Clang 22 containers. The
  exception is `pointer-sized-integer.cpp`, which checks the x86-64
  sizes.
- CAST-16: the library names integer types for a pointer value
  *(library referenced)*. The table gives the core types of the same
  size.

### Sources

Inputs: clang-sema-expr, clang-diag-sema.
