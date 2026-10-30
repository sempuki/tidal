# Forms of initialization

How a variable gets its first value. An initializer may be written after
`=`, in parentheses, or in braces, or left out. Without one, an object is
*default-initialized*. With empty parentheses or braces, it is
*value-initialized*. Objects with static or thread storage are
*zero-initialized* first (STO-3).

This part covers scalars and arrays. Initialization by constructors belongs
to classes *(referenced)*. Rules are prefixed `INIT`.

Depends on: [declarators](../declarators.md), [storage duration](../storage-linkage.md),
[value category](../../glossary.md#value-category), conversions *(referenced)*,
classes *(referenced)*, [constant evaluation](../../glossary.md#constant-evaluation).

Inputs: clang-sema-init, gcc-cp-typeck2, gcc-cp-init, edg-decl-inits,
clang-diag-sema (see `inputs/MANIFEST.md`).

## Syntax

    init = "=" expression          // copy-initialization
         | "(" expression-list ")" // direct-initialization
         | braced                  // list-initialization (list.md)
         | "=" braced

## Rules

### INIT-1 (undefined)

A variable with automatic storage and no initializer has no value until one
is assigned. Reading it before then has undefined behavior. This applies to
each element of an array as well.

    int x;
    int y = x;     // undefined
    x = 3;         // fine; x now has a value

Tests: declarations/initialization/forms/read-uninitialized.cpp, declarations/initialization/forms/read-uninitialized-array.cpp, declarations/initialization/forms/write-before-read.cpp

### INIT-2 (required)

Empty braces, `= {}`, and `T()` give a scalar the value zero: `0` for
integers, `0.0` for floating-point types, `false` for `bool`, and a null
pointer for pointers. `int a[3]{}` makes every element zero.

Tests: declarations/initialization/forms/value-init-forms.cpp

### INIT-3 (required)

A declaration with empty parentheses, such as `int x();`, declares a
function, never a variable.

Tests: declarations/initialization/forms/empty-parens-declares-function.cpp

### INIT-4 (required)

An initializer after `=` or in parentheses holds one expression, which is
converted to the variable's type by the implicit conversions *(referenced)*.
Conversions that lose information are allowed in these forms. A
constructor marked `explicit` *(classes referenced)* is used by the
parenthesized form and by braces, but not by `=`. A program is rejected if
the expression can't be converted, if a parenthesized initializer for a
scalar holds more than one expression or a braced list, or if the
variable's type is incomplete.

    int a = 3.7;      // 3
    int b(3.7);       // 3
    int* p = 1;       // rejected
    int c(1, 2);      // rejected

Tests: declarations/initialization/forms/copy-init-converts.cpp, declarations/initialization/forms/copy-init-no-conversion.cpp, declarations/initialization/forms/direct-init-multiple.cpp, declarations/initialization/forms/paren-init-explicit-conversion.cpp, declarations/initialization/forms/copy-init-explicit-rejected.cpp, declarations/initialization/forms/scalar-list-in-parens.cpp, declarations/initialization/forms/incomplete-type.cpp

### INIT-5 (required)

An array is initialized from a list of values, in braces or in parentheses.
Elements without a value become zero (INIT-2). In parentheses, conversions
that lose information are allowed. An array can't be initialized from
another array.

    int a[4] = {1, 2};   // 1, 2, 0, 0
    int b[3](1, 2);      // 1, 2, 0
    int c[2] = a;        // rejected

Tests: declarations/initialization/forms/array-partial.cpp, declarations/initialization/forms/array-paren-init.cpp, declarations/initialization/forms/array-from-array.cpp

## Evidence

All seven compilers agree on 14 of 15 tests. Results are in
`tests/RESULTS.md`.

- **INIT-1.** All compilers reject reading an uninitialized scalar during
  constant evaluation ("read of uninitialized object", "access to
  uninitialized object"). For an array element, Clang and GCC reject and
  EDG accepts. See INIT-D1.

## Divergences

### INIT-D1: uninitialized array element during constant evaluation

EDG 7.0 accepts reading an uninitialized array element during constant
evaluation. **Provisional answer:** INIT-1 applies to array elements, and
EDG's behavior is a possible defect. See question Q-0013.
