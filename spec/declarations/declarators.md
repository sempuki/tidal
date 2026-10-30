# Declarators and cv-qualifiers

A declaration gives a base type and one or more {term}`declarators
<declarator>`. Each declarator gives a name and adds pointers, references,
arrays, and function parameters to the base type. A {term}`cv-qualifier`
can apply at each level.

## Syntax

```{eval-rst}
.. productionlist:: declarators
   declarator: `ptr_part` `declarator` | `direct`
   ptr_part: "*" {`cv`} | "&" | "&&"
   direct: `name`
         : | "(" `declarator` ")"
         : | `direct` "[" [`constant`] "]"
         : | `direct` "(" `params` ")" {`cv`} ["->" `type`]
   params: [`param` {"," `param`} [[","] "..."]] | "void"
   cv: "const" | "volatile"
```

## Legality Rules

:::{rule} DCL-1
:kind: required
:tests: declarations/declarators/star-binds-to-declarator.cpp declarations/declarators/nested-declarators.cpp

A declarator applies to one name. `*`, `&`, and `&&` belong to the
declarator that contains them. Suffixes (`[]` and `()`) bind more tightly
than prefixes (`*` and `&`), and parentheses change the binding.
:::

:::{rule} DCL-2
:kind: required
:tests: declarations/declarators/pointer-to-pointer-and-function.cpp declarations/declarators/pointer-to-void-ok.cpp declarations/declarators/pointer-to-reference.cpp

A pointer can point to an object type, `void`, a function type, or a
pointer. A pointer to a reference must not be declared.
:::

:::{rule} DCL-3
:kind: required
:tests: declarations/declarators/reference-binds-same-object.cpp declarations/declarators/extern-reference-no-initializer.cpp declarations/declarators/reference-requires-initializer.cpp declarations/declarators/reference-to-reference.cpp declarations/declarators/array-of-references.cpp declarations/declarators/reference-to-void.cpp declarations/declarators/cv-qualified-reference.cpp

A reference refers to an existing object or function. Each use of the
reference is a use of that object. A reference variable must have an
initializer, except in an `extern` declaration. A program must not
declare:

- a reference to a reference,
- an array of references,
- a reference to `void`,
- a reference with a {term}`cv-qualifier` applied directly to it.
:::

:::{rule} DCL-4
:kind: required
:tests: declarations/declarators/reference-collapsing.cpp declarations/declarators/cv-on-reference-alias-ignored.cpp

A reference added to a reference type from an alias or a template argument
collapses into one reference. The result is an rvalue reference only if
both references are rvalue references. A cv-qualifier applied to such an
alias has no effect.
:::

:::{rule} DCL-5
:kind: required
:tests: declarations/declarators/array-bound-deduced.cpp declarations/declarators/array-bound-constant.cpp declarations/declarators/array-bound-runtime.cpp declarations/declarators/array-bound-zero.cpp declarations/declarators/array-bound-negative.cpp declarations/declarators/array-bound-floating.cpp declarations/declarators/array-incomplete-extern.cpp declarations/declarators/array-incomplete-no-init.cpp declarations/declarators/array-too-many-initializers.cpp

An array bound must be a constant expression of integer or unscoped
enumeration type. It is converted to the type of `sizeof` without loss of
value. It must be greater than zero. The bound can be omitted in these
cases:

- an initializer gives the number of elements,
- the declaration does not define the array, for example an `extern`
  declaration.

The initializer must not have more elements than the bound.
:::

:::{rule} DCL-6
:kind: required
:tests: declarations/declarators/multidim-outer-omitted.cpp declarations/declarators/multidim-inner-bound.cpp declarations/declarators/array-of-functions.cpp declarations/declarators/array-of-void.cpp

An array of arrays can omit only its first bound. An array element must
have a complete object type. An array of functions or of `void` must not
be declared.
:::

:::{rule} DCL-7
:kind: required
:tests: declarations/declarators/function-returning-pointers-ok.cpp declarations/declarators/function-returning-array.cpp declarations/declarators/function-returning-function.cpp declarations/declarators/const-void-return.cpp declarations/declarators/qualified-free-function.cpp

A function can return a pointer or a reference to an array or a function.
A function must not return an array or a function. A trailing return type
gives the same type as the type at the start. The return type can be
`const void`. A function that is not a class member must not have a
cv-qualifier after its parameter list.
:::

:::{rule} DCL-8
:kind: required
:tests: declarations/declarators/parameter-adjustment.cpp declarations/declarators/parameter-cv-same-function.cpp declarations/declarators/parameter-array-is-pointer.cpp

A parameter declared as an array of `T` has the type pointer to `T`. A
parameter declared as a function has the type pointer to that function.
The bound of an array parameter has no effect. A top-level cv-qualifier
of a parameter is not part of the function type.
:::

:::{rule} DCL-9
:kind: required
:tests: declarations/declarators/void-parameter-list.cpp declarations/declarators/void-parameter-named.cpp declarations/declarators/void-parameter-with-others.cpp declarations/declarators/variadic.cpp declarations/declarators/ellipsis-first.cpp

The parameter list `(void)` declares no parameters, as `()` does. `void`
must not occur in a parameter list in other forms. `...` must be the last
item in the list, and the comma before it is optional.
:::

:::{rule} CV-1
:kind: required
:tests: declarations/declarators/cv-position.cpp

A cv-qualifier applies to the type on its left. If there is no type on
its left, it applies to the type on its right. In a pointer declarator, a
cv-qualifier before `*` applies to the pointed-to type. A cv-qualifier
after `*` applies to the pointer.
:::

:::{rule} CV-2
:kind: reject
:tests: declarations/declarators/cv-duplicate-direct.cpp declarations/declarators/cv-duplicate-through-alias.cpp

The same cv-qualifier must not occur two times directly in one type. A
cv-qualifier that an alias repeats is permitted and counts one time.
:::

:::{rule} CV-3
:kind: reject
:tests: declarations/declarators/const-requires-initializer.cpp declarations/declarators/extern-const-declaration.cpp

A definition of a `const` variable of scalar type must have an
initializer. An `extern` declaration that is not a definition needs no
initializer. Class types have a separate rule *(referenced)*.
:::

:::{rule} CV-4
:kind: reject
:tests: declarations/declarators/assign-to-const.cpp declarations/declarators/assign-through-pointer-to-const.cpp declarations/declarators/increment-const.cpp

A program must not assign to, increment, or decrement an lvalue of `const`
type. This applies to a `const` variable and to an access through a
pointer or reference to `const`.
:::

:::{rule} CV-6
:kind: required
:tests: declarations/declarators/cv-on-array-applies-to-elements.cpp declarations/declarators/cv-on-function-alias.cpp

A cv-qualifier applied to an array type through an alias applies to the
elements. A cv-qualifier applied to a function type through an alias has
no effect.
:::

:::{rule} CV-7
:kind: required
:tests: declarations/declarators/qualification-conversion-ok.cpp declarations/declarators/qualification-conversion-unsafe.cpp declarations/declarators/qualification-conversion-drops-const.cpp

A pointer converts implicitly to a pointer with more cv-qualifiers on the
pointed-to type. Across several levels of pointers, `const` added at one
level needs `const` at each higher level except the outermost. No
implicit conversion removes a cv-qualifier.
:::

:::{rule} CV-8
:kind: required
:tests: declarations/declarators/const-integral-constant.cpp declarations/declarators/const-integral-runtime.cpp declarations/declarators/const-floating-not-constant.cpp

A `const` variable of integer or enumeration type with a constant
initializer can occur in a constant expression. A `const` variable with
an initializer that is not constant cannot. A `const` floating-point
variable cannot.
:::

## Undefined Behavior

:::{rule} CV-5
:kind: undefined
:tests: declarations/declarators/modify-const-object-via-cast.cpp declarations/declarators/modify-nonconst-object-via-cast.cpp

If a program changes an object that is defined `const`, the behavior is
undefined. This is also true when a cast removes `const`. A program can
change an object that is not defined `const` through such a cast.
:::

## Implementation Requirements

:::{rule} DCL-10
:kind: impl-defined
:tests: declarations/declarators/array-too-large.cpp declarations/declarators/array-4gib-lp64.cpp declarations/declarators/array-4gib-ilp32.cpp

The largest array that an implementation accepts is
implementation-defined. The limit can depend on the target. An array
whose size in bytes does not fit in the type of `sizeof` must not be
declared.
:::

## Examples

```cpp
int *p, q;                     // p is int*, q is int
int* a[3];                     // array of 3 pointers to int
int (*b)[3];                   // pointer to array of 3 int
int (*c)(int);                 // pointer to function (int) returning int
int (*(*d)(char))(double);     // pointer to function (char) returning
                               // pointer to function (double) returning int

int x = 1;
int& r = x;                    // r and x are the same object
int& s;                        // rejected: no initializer
int& const t = x;              // rejected

using L = int&;
using R = int&&;
// L& is int&, L&& is int&, R& is int&, R&& is int&&, const L is int&

int e[] = {1, 2, 3};           // int[3]
char w[] = "abc";              // char[4]
void f(int n) { int g[n]; }    // rejected: bound is not constant
int m[][2] = {{1, 2}, {3, 4}}; // int[2][2]
int k[][] = {{1}, {2}};        // rejected

void h(int a[3], void g(), const int n);
                               // type: void(int*, void (*)(), int)

const int* pc;                 // pointer to const int
int* const cp = &x;            // const pointer to int
const const int y = 1;         // rejected
using C = const int;
const C z = 1;                 // const int

using A = int[2];
const A arr = {1, 2};          // const int[2]

const int ci = 1;
const_cast<int&>(ci) = 2;      // undefined
int v = 1;
const int& cr = v;
const_cast<int&>(cr) = 2;      // v is 2
```

## Evidence

All seven compilers agree on 60 of 61 tests. The results are in
`tests/RESULTS.md`.

- **DCL-5.** Clang and GCC accept variable-length arrays and arrays of size
  zero as extensions. They reject them only under `-pedantic-errors`. EDG
  rejects both in all modes.
- **DCL-10.** Clang 22 and 18 reject `char huge[1ull << 62];`. GCC 16 and
  14 and EDG 7.0 accept it. The provisional answer is that the limit is
  implementation-defined. See {question}`Q-0012`.
- **CV-5.** All compilers reject the change during constant evaluation.
  Their notes give the cause ("modification of object of const-qualified
  type", "modifying a const object", "attempt to modify const storage").
  A change to an object that is not `const`, through the same cast, is
  accepted and executes.

Inputs: clang-sema-type, clang-sema-decl, gcc-cp-decl, edg-decls,
clang-diag-sema (see `inputs/MANIFEST.md`).
