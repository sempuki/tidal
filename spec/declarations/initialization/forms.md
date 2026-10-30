# Forms of initialization

An initializer gives a variable its first value. This page covers scalars
and arrays. Initialization by constructors belongs to classes
*(referenced)*.

## Syntax

```{eval-rst}
.. productionlist:: init_forms
   init: "=" `expression`
       : | "(" `expression_list` ")"
       : | `braced`
       : | "=" `braced`
```

An initializer after `=` is *copy-initialization*. An initializer in
parentheses is *direct-initialization*. A braced initializer is
*list-initialization*. A variable without an initializer is
*default-initialized*. Empty parentheses or braces give
*value-initialization*. Objects with static or thread storage are
*zero-initialized* first (STO-3).

## Legality Rules

:::{rule} INIT-3
:kind: required
:tests: declarations/initialization/forms/empty-parens-declares-function.cpp

A declaration with empty parentheses after the name declares a function.
It never declares a variable.
:::

:::{rule} INIT-4
:kind: required
:tests: declarations/initialization/forms/copy-init-converts.cpp declarations/initialization/forms/copy-init-no-conversion.cpp declarations/initialization/forms/direct-init-multiple.cpp declarations/initialization/forms/paren-init-explicit-conversion.cpp declarations/initialization/forms/copy-init-explicit-rejected.cpp declarations/initialization/forms/scalar-list-in-parens.cpp declarations/initialization/forms/incomplete-type.cpp

An initializer after `=` or in parentheses is converted to the type of the
variable by implicit conversion *(referenced)*. A conversion that loses
information is permitted. These conditions apply:

- The expression must be convertible to the type of the variable.
- For a scalar, the parentheses must hold exactly one expression.
- For a scalar, the expression in parentheses must not be a braced list.
- The type of the variable must be complete.
- An `explicit` constructor *(referenced)* must not be used with `=`.
:::

:::{rule} INIT-5
:kind: required
:tests: declarations/initialization/forms/array-partial.cpp declarations/initialization/forms/array-paren-init.cpp declarations/initialization/forms/array-from-array.cpp

An array is initialized from a list of values in braces or in parentheses.
In parentheses, a conversion that loses information is permitted. An
array must not be initialized from another array.
:::

## Dynamic Semantics

:::{rule} INIT-2
:kind: required
:tests: declarations/initialization/forms/value-init-forms.cpp

Empty braces, `= {}`, and `T()` give a scalar the value zero. Zero is `0`
for an integer, `0.0` for a floating-point type, `false` for `bool`, and
a null pointer for a pointer. An array element without a value in the
list also gets zero.
:::

## Undefined Behavior

:::{rule} INIT-1
:kind: undefined
:tests: declarations/initialization/forms/read-uninitialized.cpp declarations/initialization/forms/read-uninitialized-array.cpp declarations/initialization/forms/write-before-read.cpp declarations/initialization/forms/copy-indeterminate-unsigned-char-run.cpp declarations/initialization/forms/copy-indeterminate-unsigned-char.cpp declarations/initialization/forms/self-initialization.cpp

A variable with {term}`automatic storage` and no initializer has no value
until the program assigns one. A read of the variable before that time is
undefined. This also applies to each element of an array. A copy of such a
value into an object of type `unsigned char` or `std::byte` is not
undefined, and the copy also has no value ({question}`Q-0039`).
This includes a variable that its own initializer reads.
:::

## Examples

```cpp
int x;
int y = x;           // undefined: x has no value
x = 3;               // x now has a value

int z();             // a function declaration

int a = 3.7;         // 3
int b(3.7);          // 3
int* p = 1;          // rejected
int c(1, 2);         // rejected

int d[4] = {1, 2};   // 1, 2, 0, 0
int e[3](1, 2);      // 1, 2, 0
int f[2] = d;        // rejected
```

## Evidence

All seven compilers agree on 14 of 15 tests. The results are in
`tests/RESULTS.md`.

- **INIT-1.** All compilers reject a read of an uninitialized scalar during
  {term}`constant evaluation` ("read of uninitialized object", "access to
  uninitialized object").
- **INIT-1 divergence.** For an array element, Clang and GCC reject the
  read during constant evaluation. EDG 7.0 accepts it. The provisional
  answer is that INIT-1 applies to array elements and that EDG has a
  possible defect. See {question}`Q-0013`.

Inputs: clang-sema-init, gcc-cp-typeck2, gcc-cp-init, edg-decl-inits,
clang-diag-sema (see `inputs/MANIFEST.md`).
