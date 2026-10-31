# Declarators and cv-qualifiers

A declaration gives a base type and one or more {term}`declarators
<declarator>`. Each declarator gives a name and adds pointers, references,
arrays, and function parameters to the base type. A {term}`cv-qualifier`
can apply at each level.

## Principles

:::{principle} DCL-P1
:rules: DCL-1 DCL-13 CV-1 CV-2

A declarator builds a type around one name, from the inside out, and it
reads as an expression does. Suffixes bind before prefixes, and
parentheses regroup. A qualifier attaches to the type on its left, or to
the type on its right when nothing is on its left. A declaration that
can be read as a function declaration is one, because the grammar
prefers that reading. Saying the same qualifier twice is a mistake,
unless an alias repeats it.
:::

:::{principle} DCL-P2
:rules: DCL-2 DCL-3 DCL-4 DCL-11 DCL-5 DCL-6 DCL-7 DCL-12 DCL-10

Each compound type has the shape that its purpose requires. A pointer
holds the address of anything that has one. A reference is another name
for an existing object or function, so it must be bound when it is
created, it cannot refer to nothing or to another reference, and a
reference to a reference collapses into one. An array holds a known,
positive number of complete elements, so its bound is a constant, or
comes from the initializer or from a definition elsewhere, and its
elements cannot be functions or references. A function returns a value,
so it cannot return an array or a function, though it can return a
pointer or a reference to one. The largest array is a limit of the
implementation.
:::

:::{principle} DCL-P3
:rules: DCL-8 DCL-9 DCL-14 DCL-15 DCL-16

A function type is its parameter types, its return type, and its
qualifiers. Parameters are passed as values, so an array or a function
parameter is a pointer, and a top-level qualifier on a parameter is the
function body's business and not part of the type. `noexcept` is part
of the type, so all declarations agree on it. A function type with a
cv-qualifier or a ref-qualifier describes a member function, so it can
be named and used for member functions and pointers to members, and for
nothing else.
:::

:::{principle} DCL-P4
:rules: CV-3 CV-4 CV-5 CV-6 CV-7 CV-8 CV-9

`const` is a promise that an object does not change after
initialization. The promise is kept at both ends. A `const` scalar must
get its value when it is defined, no assignment goes through a `const`
path, and a conversion can add `const` to what a pointer reaches but
never remove it. Changing an object that was defined `const` breaks the
promise, so the behavior is undefined. A `const` integer with a constant
initializer is a constant that the program can use at translation time.
An alias carries a qualifier to the elements of an array and to nothing
in a function type. `volatile` on a parameter or a return type is
deprecated.
:::

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

## Constraints

:::{rule} DCL-1
:kind: required
:tests: declarations/declarators/star-binds-to-declarator.cpp declarations/declarators/nested-declarators.cpp declarations/declarators/redundant-parentheses.cpp

A declarator applies to one name. `*`, `&`, and `&&` belong to the
declarator that contains them. Suffixes (`[]` and `()`) bind more tightly
than prefixes (`*` and `&`), and parentheses change the binding.

Parentheses around the name in a declarator are permitted and have no
effect.
:::

:::{rule} DCL-2
:kind: required
:tests: declarations/declarators/pointer-to-pointer-and-function.cpp declarations/declarators/pointer-to-void-ok.cpp declarations/declarators/pointer-to-reference.cpp

A pointer can point to an object type, `void`, a function type, or a
pointer. A pointer to a reference must not be declared.
:::

:::{rule} DCL-3
:kind: required
:tests: declarations/declarators/extern-reference-no-initializer.cpp declarations/declarators/reference-requires-initializer.cpp declarations/declarators/reference-to-reference.cpp declarations/declarators/array-of-references.cpp declarations/declarators/reference-to-void.cpp declarations/declarators/cv-qualified-reference.cpp declarations/declarators/reference-parameter-no-initializer.cpp

A reference variable must have an initializer, except in an `extern`
declaration. A reference parameter, a reference return type, and a
reference data member in a class need no initializer in their
declaration. A program must not declare:

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
:tests: declarations/declarators/array-bound-deduced.cpp declarations/declarators/array-bound-constant.cpp declarations/declarators/array-bound-runtime.cpp declarations/declarators/array-bound-zero.cpp declarations/declarators/array-bound-negative.cpp declarations/declarators/array-bound-floating.cpp declarations/declarators/array-incomplete-extern.cpp declarations/declarators/array-incomplete-no-init.cpp declarations/declarators/array-too-many-initializers.cpp declarations/declarators/array-bound-class-conversion.cpp declarations/declarators/array-bound-scoped-enum.cpp declarations/declarators/static-member-unknown-bound.cpp declarations/declarators/unknown-bound-empty-braces.cpp

An array bound must be a constant expression that converts to the type of
`sizeof` without loss of value. Integer types, unscoped enumerations, and
class types with a `constexpr` conversion function to an integer type are
valid. A scoped enumeration is not valid. The bound must be greater than
zero. The bound can be omitted in these cases:

- an initializer gives the number of elements,
- the declaration does not define the array, for example an `extern`
  declaration.

The initializer must not have more elements than the bound. An array
without a bound must not have an empty braced initializer, because it
would have zero elements.

A static data member declared in a class can have an array type without a
bound. Its definition outside the class gives the bound.
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
:tests: declarations/declarators/function-returning-pointers-ok.cpp declarations/declarators/function-returning-array.cpp declarations/declarators/function-returning-function.cpp declarations/declarators/const-void-return.cpp declarations/declarators/qualified-free-function.cpp declarations/declarators/trailing-return-with-type.cpp declarations/declarators/ref-qualified-free-function.cpp

A function can return a pointer or a reference to an array or a function.
A function must not return an array or a function. A trailing return type
requires `auto` at the start, and gives the return type. The return type
can be `const void`. A function that is not a class member must not have
a cv-qualifier after its parameter list.

A function that is not a member, and a static member function, must not
have a ref-qualifier.
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
:tests: declarations/declarators/const-integral-constant.cpp declarations/declarators/const-integral-runtime.cpp declarations/declarators/const-floating-not-constant.cpp declarations/declarators/const-volatile-not-constant.cpp

A `const` variable of integer or enumeration type with a constant
initializer can occur in a constant expression. A `const` variable with
an initializer that is not constant cannot. A `const` floating-point
variable cannot. A `volatile` variable cannot, even with a constant
initializer.
:::

:::{rule} DCL-12
:kind: required
:tests: declarations/declarators/pointer-to-member.cpp declarations/declarators/pointer-to-reference-member.cpp declarations/declarators/pointer-to-void-member.cpp

A declarator of the form `C::* name` declares a pointer to a member of
class `C` *(classes referenced)*. The member type must not be a reference
type or `void`.
:::

:::{rule} DCL-13
:kind: required
:tests: declarations/declarators/function-declaration-ambiguity.cpp

A declaration that can be read as a function declaration is a function
declaration. This includes a parenthesized initializer that has the form
of a parameter list, such as a type name followed by `()`.
:::

:::{rule} DCL-14
:kind: reject
:tests: declarations/declarators/noexcept-in-type.cpp declarations/declarators/noexcept-redeclaration.cpp

The `noexcept` specification is part of the type of a function. All
declarations of a function must agree on it.
:::

:::{rule} DCL-15
:kind: required
:tests: declarations/declarators/qualified-function-type-member.cpp

A function type with a cv-qualifier or a ref-qualifier after its
parameter list can be named through a type alias. Such a type can
declare a non-static member function and can be the target of a pointer
to member.
:::

:::{rule} DCL-16
:kind: reject
:tests: declarations/declarators/qualified-function-type-pointer.cpp declarations/declarators/qualified-function-type-parameter.cpp

A function type with a cv-qualifier or a ref-qualifier must not be the
target of a pointer or a reference, the type of a parameter, or the type
of a variable ({question}`Q-0106`).
:::

## Dynamic Semantics

:::{rule} DCL-11
:kind: required
:tests: declarations/declarators/reference-binds-same-object.cpp

A reference refers to an existing object or function. Each use of the
reference is a use of that object or function.
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

## Recommended Practice

:::{rule} CV-9
:kind: recommended
:tests: declarations/declarators/volatile-return-deprecated.cpp

An implementation should warn about a `volatile` return type and a
`volatile` parameter type. These uses are deprecated.
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

All seven compilers agree on 78 of 80 tests. The results are in
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
- **DCL-16.** EDG 7.0 accepts a reference parameter to a function type
  with a ref-qualifier. Clang and GCC reject it ({question}`Q-0106`).

Inputs: clang-sema-type, clang-sema-decl, edg-decls, clang-diag-sema (see
`inputs/MANIFEST.md`).
