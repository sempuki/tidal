# Forms of initialization

An initializer gives a variable its first value. This page covers scalars
and arrays. Initialization by constructors is on the page on
{doc}`constructors and destructors </classes/constructors>`.

## Principles

:::{principle} INIT-P1
:rules: INIT-2 INIT-4 INIT-7 INIT-5 INIT-8

An initializer gives an object its first value by converting the
initializer to the object's type ({principle}`VC-P2`).

| Initializer | Meaning |
|---|---|
| `=` | The conversion. This is copy-initialization, so it does not use an `explicit` constructor. |
| Parentheses | The same conversion as `=` |
| A list of values, for an array | One value per element |
| Empty braces, `T()` | The zero of the type |

:::

:::{principle} INIT-P2
:rules: INIT-1

An automatic object with no initializer has no value until the program
assigns one ({principle}`OBJ-P4`).
:::

:::{principle} INIT-P3
:rules: INIT-3 INIT-6

Two facts about form. Empty parentheses after a name declare a function,
because the grammar reads them as a parameter list. Declarators in one
declaration are initialized from left to right.
:::

## Syntax

```{eval-rst}
.. productionlist:: init_forms
   init: "=" `expression`
       : | "(" `expression_list` ")"
       : | `braced`
       : | "=" `braced`
```

These terms name the forms of initialization:

| Form | Term |
|---|---|
| An initializer after `=` | *copy-initialization* |
| An initializer in parentheses | *direct-initialization* |
| A braced initializer | *list-initialization* |
| A variable without an initializer | *default-initialized* |
| Empty parentheses or braces | *value-initialization* |

Objects with static or thread storage are *zero-initialized* first
(STO-3).

## Constraints

:::{rule} INIT-3
:kind: required
:tests: declarations/initialization/forms/empty-parens-declares-function.cpp

A declaration with empty parentheses after the name declares a function.
It never declares a variable.
:::

:::{rule} INIT-4
:kind: required
:tests: declarations/initialization/forms/copy-init-converts.cpp declarations/initialization/forms/paren-init-explicit-conversion.cpp declarations/initialization/forms/nullptr-bool-direct.cpp

An initializer after `=` or in parentheses is converted to the type of the
variable by implicit conversion *(conversion sequences
referenced)*. A conversion that loses
information is permitted. `nullptr` converts to `bool` in direct
initialization, with parentheses or braces, and gives `false`. INIT-7
lists what is not valid.
:::

:::{rule} INIT-7
:kind: reject
:tests: declarations/initialization/forms/copy-init-no-conversion.cpp declarations/initialization/forms/direct-init-multiple.cpp declarations/initialization/forms/copy-init-explicit-rejected.cpp declarations/initialization/forms/scalar-list-in-parens.cpp declarations/initialization/forms/incomplete-type.cpp declarations/initialization/forms/nullptr-bool-copy.cpp

An initializer after `=` or in parentheses must meet these conditions:

- The expression must be convertible to the type of the variable.
- For a scalar, the parentheses must hold exactly one expression.
- For a scalar, the expression in parentheses must not be a braced list.
- The type of the variable must be complete.
- An `explicit` constructor ({rule}`CTOR-9`) must not be used with `=`.
- `nullptr` converts to `bool` only in direct initialization.
:::

:::{rule} INIT-5
:kind: required
:tests: declarations/initialization/forms/array-partial.cpp declarations/initialization/forms/array-paren-init.cpp declarations/initialization/forms/paren-array-unknown-bound.cpp

An array is initialized from a list of values in braces or in parentheses.
In parentheses, a conversion that loses information is permitted. In parentheses, an
array without a bound gets the number of values as its bound. INIT-8
lists what is not valid.
:::

:::{rule} INIT-8
:kind: reject
:tests: declarations/initialization/forms/array-from-array.cpp

An array must not be initialized from another array.
:::

## Runtime Behavior

:::{rule} INIT-2
:kind: required
:tests: declarations/initialization/forms/value-init-forms.cpp

Empty braces, `= {}`, and `T()` give a scalar the value zero. Zero is:

| Type | Zero |
|---|---|
| An integer | `0` |
| A floating-point type | `0.0` |
| `bool` | `false` |
| A pointer | A null pointer |

An array element without a value in the list also gets zero.
:::

:::{rule} INIT-6
:kind: required
:tests: declarations/initialization/forms/declarators-in-order.cpp

A declaration with several declarators initializes them in order, from
left to right.
:::

## Undefined Behavior

:::{rule} INIT-1
:kind: undefined
:tests: declarations/initialization/forms/read-uninitialized.cpp declarations/initialization/forms/read-uninitialized-array.cpp declarations/initialization/forms/write-before-read.cpp declarations/initialization/forms/copy-indeterminate-unsigned-char-run.cpp declarations/initialization/forms/copy-indeterminate-unsigned-char.cpp declarations/initialization/forms/self-initialization.cpp

A variable with {term}`automatic storage` and no initializer has no
value until the program assigns one. A read of the variable before that
time is undefined. These rules also apply:

- A variable that its own initializer reads has no value at that read.
- Each element of an array with no initializer has no value in the same
  way.
- A copy of such a value into an object of type `unsigned char` or
  `std::byte` is not undefined, and the copy also has no value
  ({question}`Q-0039`).
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

All seven compilers agree on 21 of 22 tests. The results are in
`tests/RESULTS.md`.

- **INIT-1.** All compilers reject a read of an uninitialized scalar during
  {term}`constant evaluation` ("read of uninitialized object", "access to
  uninitialized object").
- **INIT-1 divergence.** For an array element, Clang and GCC reject the
  read during constant evaluation. EDG 7.0 accepts it. The provisional
  answer is that INIT-1 applies to array elements and that EDG has a
  possible defect. See {question}`Q-0013`.

Inputs: clang-sema-init, edg-decl-inits, clang-diag-sema (see
`inputs/MANIFEST.md`).
