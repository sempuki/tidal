# Declarators and cv-qualifiers

A declaration names a base type (`int`, a class, a placeholder) and then
one or more *declarators*. Each declarator gives a name and wraps the base
type in pointers, references, arrays, and functions. The *cv-qualifiers*
`const` and `volatile` may be applied at each level.

This section covers how declarators build types, the limits on which
combinations are allowed, and what `const` and `volatile` mean for objects.
Pointers to members and member function qualifiers belong to classes
*(referenced)*. Default arguments belong to function definitions.

Rules prefixed `DCL` cover declarators. Rules prefixed `CV` cover
cv-qualifiers.

Depends on: types *(referenced)*, [placeholder types](auto.md),
initialization *(referenced)*, [constant evaluation](../glossary.md#constant-evaluation),
classes *(referenced)*.

Inputs: clang-sema-type, clang-sema-decl, gcc-cp-decl, edg-decls,
clang-diag-sema (see `inputs/MANIFEST.md`).

## Syntax

    declarator  = ptr-part declarator
                | direct
    ptr-part    = "*" { cv } | "&" | "&&"
    direct      = name
                | "(" declarator ")"
                | direct "[" [ constant ] "]"
                | direct "(" params ")" { cv } [ "->" type ]
    params      = [ param { "," param } [ [ "," ] "..." ] ] | "void"
    cv          = "const" | "volatile"

## Rules

### DCL-1 (required)

A declarator applies to one name. `*`, `&`, and `&&` belong to the
declarator they're written in, so `int *p, q;` declares a pointer `p` and
an `int` `q`. Suffixes (`[]` and `()`)
bind more tightly than prefixes (`*`, `&`), and parentheses override that.
A declarator is read from the name outward, suffixes first.

    int* a[3];        // array of 3 pointers to int
    int (*b)[3];      // pointer to array of 3 int
    int (*c)(int);    // pointer to function taking int, returning int
    int (*(*d)(char))(double);
                      // pointer to function taking char, returning
                      // pointer to function taking double, returning int

Tests: declarations/declarators/star-binds-to-declarator.cpp, declarations/declarators/nested-declarators.cpp

### DCL-2 (required)

A pointer may point to an object type, `void`, a function type, or
another pointer. A pointer to a reference is rejected.

Tests: declarations/declarators/pointer-to-pointer-and-function.cpp, declarations/declarators/pointer-to-void-ok.cpp, declarations/declarators/pointer-to-reference.cpp

### DCL-3 (required)

A reference names an existing object or function, and every use of the
reference is a use of that object. A reference variable must be
initialized, except in a declaration with `extern`. A program is rejected
if it declares a reference to a reference, an array of references, a
reference to `void`, or applies `const` or `volatile` directly to a
reference.

    int x = 1;
    int& r = x;       // r and x are the same object
    int& s;           // rejected
    int& const t = x; // rejected

Tests: declarations/declarators/reference-binds-same-object.cpp, declarations/declarators/extern-reference-no-initializer.cpp, declarations/declarators/reference-requires-initializer.cpp, declarations/declarators/reference-to-reference.cpp, declarations/declarators/array-of-references.cpp, declarations/declarators/reference-to-void.cpp, declarations/declarators/cv-qualified-reference.cpp

### DCL-4 (required)

When a reference type comes from an alias or a template argument, adding
another reference collapses them into one. The result is an rvalue
reference only when both are rvalue references. `const` or `volatile`
applied to such an alias is ignored.

    using L = int&;
    using R = int&&;
    L&   // int&      L&&  // int&
    R&   // int&      R&&  // int&&
    const L          // int&

Tests: declarations/declarators/reference-collapsing.cpp, declarations/declarators/cv-on-reference-alias-ignored.cpp

### DCL-5 (required)

An array bound is a constant expression of integer or unscoped enumeration
type, converted to the type of `sizeof` without loss. It must be greater
than zero. The bound may be left out when an initializer supplies it, or in
a declaration that doesn't define the array, such as one with `extern`. A
program is rejected if the bound is not a constant, is zero or negative, is
not an integer, is missing where it is needed, or if the initializer has
more elements than the bound.

    int a[] = {1, 2, 3};   // int[3]
    char s[] = "abc";      // char[4]
    void f(int n) { int b[n]; }   // rejected

Tests: declarations/declarators/array-bound-deduced.cpp, declarations/declarators/array-bound-constant.cpp, declarations/declarators/array-bound-runtime.cpp, declarations/declarators/array-bound-zero.cpp, declarations/declarators/array-bound-negative.cpp, declarations/declarators/array-bound-floating.cpp, declarations/declarators/array-incomplete-extern.cpp, declarations/declarators/array-incomplete-no-init.cpp, declarations/declarators/array-too-many-initializers.cpp

### DCL-6 (required)

An array of arrays may leave out only its first bound. Array elements must
be complete object types, so a program is rejected if it declares an array
of functions or of `void`.

    int a[][2] = {{1, 2}, {3, 4}, {5, 6}};   // int[3][2]
    int b[][] = {{1}, {2}};                  // rejected

Tests: declarations/declarators/multidim-outer-omitted.cpp, declarations/declarators/multidim-inner-bound.cpp, declarations/declarators/array-of-functions.cpp, declarations/declarators/array-of-void.cpp

### DCL-7 (required)

A function may return a pointer or reference to an array or function, but
not an array or function. A trailing return type (`auto f() -> T`) gives
the same type as writing `T` first. `const void` is an accepted return
type. A program is rejected if a function returns an array or function, or
if a function that is not a class member has a `const` or `volatile`
qualifier after its parameter list.

Tests: declarations/declarators/function-returning-pointers-ok.cpp, declarations/declarators/function-returning-array.cpp, declarations/declarators/function-returning-function.cpp, declarations/declarators/const-void-return.cpp, declarations/declarators/qualified-free-function.cpp

### DCL-8 (required)

A parameter declared as an array is a pointer to its element type, and a
parameter declared as a function is a pointer to that function type. Any
array bound on the parameter is ignored. `const` and `volatile` at the
top level of a parameter are not part of the function's type, so `void
f(int)` and `void f(const int)` declare the same function.

    void f(int a[3], void g(), const int n);
    // type: void(int*, void (*)(), int)

Tests: declarations/declarators/parameter-adjustment.cpp, declarations/declarators/parameter-cv-same-function.cpp, declarations/declarators/parameter-array-is-pointer.cpp

### DCL-9 (required)

A parameter list of `(void)` means no parameters, the same as `()`.
`void` is allowed only in that form, so a program is rejected if it names
a `void` parameter or lists `void` with other parameters. A trailing
`...` accepts any further arguments, and the comma before it is optional.
`...` must come last.

Tests: declarations/declarators/void-parameter-list.cpp, declarations/declarators/void-parameter-named.cpp, declarations/declarators/void-parameter-with-others.cpp, declarations/declarators/variadic.cpp, declarations/declarators/ellipsis-first.cpp

### DCL-10 (impl-defined)

Each implementation sets the largest array it accepts, and the limit can
depend on the target. An array whose size in bytes does not fit the type of
`sizeof` is rejected (question Q-0012).

Tests: declarations/declarators/array-too-large.cpp, declarations/declarators/array-4gib-lp64.cpp, declarations/declarators/array-4gib-ilp32.cpp

### CV-1 (required)

`const` and `volatile` apply to the type to their left, or to the type
on their right when nothing is on their left. `const int` and `int const`
are the same type. In a pointer, a qualifier before `*` applies to the
pointed-to type, and a qualifier after `*` applies to the pointer.

    const int* a;        // pointer to const int
    int* const b = &x;   // const pointer to int

Tests: declarations/declarators/cv-position.cpp

### CV-2 (reject)

A program is rejected if the same qualifier is written twice directly in
one type. A qualifier repeated through an alias is accepted and counts once.

    const const int x = 1;   // rejected
    using C = const int;
    const C y = 1;           // const int

Tests: declarations/declarators/cv-duplicate-direct.cpp, declarations/declarators/cv-duplicate-through-alias.cpp

### CV-3 (reject)

A program is rejected if it defines a `const` variable of a scalar type
without an initializer. A declaration with `extern` that isn't a
definition needs none. Class types *(referenced)* have their own rule.

Tests: declarations/declarators/const-requires-initializer.cpp, declarations/declarators/extern-const-declaration.cpp

### CV-4 (reject)

A program is rejected if it assigns to, increments, or decrements an
lvalue whose type is `const`, whether it names a `const` variable or is
reached through a pointer or reference to `const`.

Tests: declarations/declarators/assign-to-const.cpp, declarations/declarators/assign-through-pointer-to-const.cpp, declarations/declarators/increment-const.cpp

### CV-5 (undefined)

Modifying an object that was defined `const` has undefined behavior, even
when a cast removes `const` from the path to it. Removing `const` with a
cast and modifying an object that was not defined `const` is allowed.

    const int x = 1;
    const_cast<int&>(x) = 2;     // undefined
    int y = 1;
    const int& r = y;
    const_cast<int&>(r) = 2;     // fine: y is 2

Tests: declarations/declarators/modify-const-object-via-cast.cpp, declarations/declarators/modify-nonconst-object-via-cast.cpp

### CV-6 (required)

`const` or `volatile` applied to an array type, through an alias, applies
to its elements. Applied to a function type through an alias, it is
ignored.

    using A = int[2];
    const A a = {1, 2};      // const int[2]

Tests: declarations/declarators/cv-on-array-applies-to-elements.cpp, declarations/declarators/cv-on-function-alias.cpp

### CV-7 (required)

A pointer converts implicitly to a pointer with more qualifiers on the
pointed-to type, as in `int*` to `const int*`. Through several levels of
pointers, adding `const` at one level requires `const` at every level
above it except the outermost, so `int**` converts to `const int* const*`
but not to `const int**`. Removing a qualifier is never implicit.

Tests: declarations/declarators/qualification-conversion-ok.cpp, declarations/declarators/qualification-conversion-unsafe.cpp, declarations/declarators/qualification-conversion-drops-const.cpp

### CV-8 (required)

A `const` variable of integer or enumeration type whose initializer is a
constant expression may be used in constant expressions, for example as an
array bound. A `const` variable initialized at run time can't be, and
neither can a `const` floating-point variable. Use `constexpr` for those
cases *(referenced)*.

Tests: declarations/declarators/const-integral-constant.cpp, declarations/declarators/const-integral-runtime.cpp, declarations/declarators/const-floating-not-constant.cpp

## Evidence

All seven compilers agree on 60 of 61 tests. Results are in
`tests/RESULTS.md`.

- **DCL-5.** Clang and GCC accept variable-length arrays and zero-size
  arrays as extensions and reject them only under `-pedantic-errors`. EDG
  rejects both in every mode.
- **DCL-10.** Limits differ by implementation and target. See DCL-D1.
- **CV-5.** All compilers reject the modification during constant
  evaluation and name the cause ("modification of object of const-qualified
  type", "modifying a const object", "attempt to modify const storage").
  Modifying a non-`const` object through the same cast is accepted and
  runs.

## Divergences

### DCL-D1: largest array

`char huge[1ull << 62];` is rejected by Clang 22 and 18 and accepted by GCC
16 and 14 and EDG 7.0. **Provisional answer:** the limit is
implementation-defined (DCL-10). See question Q-0012.
