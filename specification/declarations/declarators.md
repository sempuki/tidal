# Declarators and cv-qualifiers

A declaration gives a base type and one or more {term}`declarators
<declarator>`. Each declarator gives a name and adds these to the base
type:

- pointers,
- references,
- arrays,
- function parameters.

A {term}`cv-qualifier` can apply at each level.

## Principles

:::{principle} DCL-P1
:rules: DCL-1 DCL-13 CV-1 CV-2

A declarator builds a type around one name, from the inside out, and
the declarator reads as an expression does.

- Suffixes bind before prefixes, and parentheses regroup.
- A qualifier attaches to the type on its left, or to the type on its
  right when nothing is on its left.
- A declaration that can be read as a function declaration is a
  function declaration, because the grammar prefers that reading.
- Saying the same qualifier twice is a mistake, unless an alias repeats
  it.
:::

:::{principle} DCL-P2
:rules: DCL-2 DCL-17 DCL-3 DCL-18 DCL-4 DCL-11 DCL-12 DCL-23

A pointer holds the address of anything that has one:

- an object,
- a function,
- `void`,
- another pointer.

A pointer to member names a member without an object. A reference is
another name for an existing object or function, so:

- the reference is bound when it is created,
- the reference refers to something,
- a reference to a reference collapses into one, because there is
  nothing to refer to in between.
:::

:::{principle} DCL-P5
:rules: DCL-5 DCL-19 DCL-6 DCL-20 DCL-7 DCL-21

An array holds a known, positive number of complete elements, so its
bound is a constant, or comes from the initializer or from a definition
elsewhere. Only the first bound of an array of arrays can be left out,
because the element type must be complete. A function returns a value,
so a function cannot return an array or a function, though it can
return a pointer or a reference to an array or a function. The largest
array is a limit of the implementation ({rule}`TR-14`).
:::

:::{principle} DCL-P3
:rules: DCL-8 DCL-9 DCL-22 DCL-14 DCL-15 DCL-16

A function type is made of:

- its parameter types,
- its return type,
- its qualifiers.

Parameters are passed as values, so an array or a function parameter is
a pointer, and a top-level qualifier on a parameter is the function
body's business and not part of the type. A parameter of type
reference to array keeps the length, and a template can deduce the
length from it ({doc}`/templates/deduction`). `noexcept` is part of the
type, so all declarations agree on it. A function type with a
cv-qualifier or a ref-qualifier describes a member function, so such a
type can be named and used for member functions and pointers to
members, and for nothing else.
:::

:::{principle} DCL-P4
:rules: CV-3 CV-4 CV-5 CV-6 CV-7 CV-10 CV-8 CV-11 CV-9

`const` is a promise that an object does not change after
initialization. Three rules keep the promise:

- a `const` scalar must state its value in its definition, also at
  namespace scope, where the storage is zero before any initializer
  runs,
- no assignment goes through a `const` path,
- a conversion can add `const` to what a pointer reaches but never
  remove it.

Changing an object that was defined `const` breaks the promise, so the
behavior is undefined. A `const` integer with a constant initializer is
a constant that the program can use at translation time. An alias
carries a qualifier to the elements of an array and to nothing in a
function type. `volatile` on a parameter or a return type is
deprecated.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   declarator: `ptr_part` `declarator`
             : | `direct_declarator`
   ptr_part: "*" {`cv`} {`attribute_spec`}
           : | `nested_name` "*" {`cv`} {`attribute_spec`}
           : | "&" {`attribute_spec`}
           : | "&&" {`attribute_spec`}
   direct_declarator: ["..."] `declarator_id` {`attribute_spec`}
                    : | "(" `declarator` ")"
                    : | `direct_declarator` `array_suffix`
                    : | `direct_declarator` `function_suffix`
   declarator_id: `id_expression`
                : | [`nested_name`] `operator_function_id`
                : | [`nested_name`] `literal_operator_id`
                : | [`nested_name`] `conversion_function_id`
                : | [`nested_name`] "~" `class_name`
   array_suffix: "[" [`constant_expr`] "]" {`attribute_spec`}
   function_suffix: "(" `parameters` ")" {`cv`} [`ref_qualifier`]
                  :    [`noexcept_specifier`] {`attribute_spec`} ["->" `type_id`]
   parameters: [`parameter` {"," `parameter`} [[","] "..."]]
             : | "..."
             : | "void"
   cv: "const" | "volatile"
   ref_qualifier: "&" | "&&"
   type_id: `type_specifiers` [`abstract_declarator`]
   abstract_declarator: `ptr_part` [`abstract_declarator`]
                      : | `abstract_direct`
                      : | "..."
   abstract_direct: "(" `abstract_declarator` ")"
                  : | [`abstract_direct`] `array_suffix`
                  : | [`abstract_direct`] `function_suffix`
```

A `...` before the `declarator_id`, or as the last part of an
`abstract_declarator`, declares a {term}`function parameter pack`
({rule}`PACK-1`).

## Constraints

:::{rule} DCL-1
:kind: required
:tests: declarations/declarators/star-binds-to-declarator.cpp declarations/declarators/nested-declarators.cpp declarations/declarators/redundant-parentheses.cpp

A declarator applies to one name. `*`, `&`, and `&&` belong to the
declarator that contains them. Suffixes (`[]` and `()`) bind more
tightly than prefixes (`*` and `&`), and parentheses change the binding.

Parentheses around the name in a declarator are permitted and have no
effect.
:::

:::{rule} DCL-2
:kind: required
:tests: declarations/declarators/pointer-to-pointer-and-function.cpp declarations/declarators/pointer-to-void-ok.cpp

A pointer can point to:

- an object type,
- `void`,
- a function type,
- a pointer.

DCL-17 lists what is not valid.
:::

:::{rule} DCL-17
:kind: reject
:tests: declarations/declarators/pointer-to-reference.cpp declarations/declarators/pointer-to-reference-alias.cpp

A pointer to a reference must not be declared.
:::

:::{rule} DCL-3
:kind: required
:tests: declarations/declarators/extern-reference-no-initializer.cpp declarations/declarators/reference-parameter-no-initializer.cpp

These declarations need no initializer:

- an `extern` declaration of a reference variable,
- the declaration of a reference parameter,
- the declaration of a reference return type,
- the declaration of a reference data member in a class.

DCL-18 lists what is not valid.
:::

:::{rule} DCL-18
:kind: reject
:tests: declarations/declarators/reference-requires-initializer.cpp declarations/declarators/reference-to-reference.cpp declarations/declarators/array-of-references.cpp declarations/declarators/reference-to-void.cpp declarations/declarators/cv-qualified-reference.cpp

A reference variable must have an initializer, except in an `extern`
declaration. A program must not declare:

- a reference to a reference,
- an array of references,
- a reference to `void`,
- a reference with a {term}`cv-qualifier` applied directly to it.
:::

:::{rule} DCL-4
:kind: required
:tests: declarations/declarators/reference-collapsing.cpp declarations/declarators/cv-on-reference-alias-ignored.cpp

A reference added to a reference type from an alias, a {term}`template argument`, or `decltype` collapses into one reference ({rule}`PACK-24`). The result is an rvalue reference
only if both references are rvalue references. A cv-qualifier applied to
such an alias has no effect.
:::

:::{rule} DCL-5
:kind: required
:tests: declarations/declarators/array-bound-deduced.cpp declarations/declarators/array-bound-constant.cpp declarations/declarators/array-incomplete-extern.cpp declarations/declarators/array-bound-class-conversion.cpp declarations/declarators/static-member-unknown-bound.cpp declarations/declarators/unknown-bound-converted-class-elements.cpp templates/deduction/array-bound-from-converted-elements.cpp

These types are valid types for an array bound:

- integer types,
- unscoped enumerations,
- class types with a `constexpr` conversion function to an integer
  type.

The bound can be omitted in these cases:

- an initializer gives the number of elements,
- the declaration does not define the array, for example an `extern`
  declaration.

A static data member declared in a class can have an array type without
a bound. Its definition outside the class gives the bound. DCL-19 lists
what is not valid.
:::

:::{rule} DCL-19
:kind: reject
:tests: declarations/declarators/array-bound-runtime.cpp declarations/declarators/array-bound-zero.cpp declarations/declarators/array-bound-negative.cpp declarations/declarators/array-bound-floating.cpp declarations/declarators/array-incomplete-no-init.cpp declarations/declarators/array-too-many-initializers.cpp declarations/declarators/array-bound-scoped-enum.cpp declarations/declarators/unknown-bound-empty-braces.cpp declarations/declarators/array-4gib-ilp32.cpp foundations/storage/array-size-exceeds-size-type.cpp

An array declaration must obey these rules:

- The bound must be a constant expression of a type that DCL-5 lists.
  A floating-point bound is not valid, even with an integer value.
- The value of the bound must convert to the type of `sizeof` without
  loss of value.
- The bound must not have a scoped enumeration type.
- The bound must be greater than zero.
- The bound can be omitted only in the cases that DCL-5 lists.
- The initializer must not have more elements than the bound.
- An array without a bound must not have an empty braced initializer,
  because the array would have zero elements.
- The size of the array in bytes must fit in the type of `sizeof`
  ({rule}`STG-1`).
:::

:::{rule} DCL-6
:kind: required
:tests: declarations/declarators/multidim-outer-omitted.cpp declarations/declarators/multidim-outer-omitted-strings.cpp

An array of arrays can omit its first bound. DCL-20 lists what is not
valid.
:::

:::{rule} DCL-20
:kind: reject
:tests: declarations/declarators/multidim-inner-bound.cpp declarations/declarators/array-of-functions.cpp declarations/declarators/array-of-void.cpp

An array of arrays must not omit a bound other than its first. An array
element must have a complete object type. An array of functions or of
`void` must not be declared.
:::

:::{rule} DCL-7
:kind: required
:tests: declarations/declarators/function-returning-pointers-ok.cpp declarations/declarators/const-void-return.cpp

A function can return a pointer or a reference to an array or a
function. A trailing return type follows `auto` at the start, and gives
the return type. The return type can be `const void`. DCL-21 lists what
is not valid.
:::

:::{rule} DCL-21
:kind: reject
:tests: declarations/declarators/function-returning-array.cpp declarations/declarators/function-returning-function.cpp declarations/declarators/qualified-free-function.cpp declarations/declarators/trailing-return-with-type.cpp declarations/declarators/ref-qualified-free-function.cpp

A function must not return an array or a function. A trailing return
type requires `auto` at the start. A function that is not a class member
must not have a cv-qualifier after its parameter list.

A function that is not a member, and a static member function, must not
have a ref-qualifier.
:::

:::{rule} DCL-8
:kind: required
:tests: declarations/declarators/parameter-adjustment.cpp declarations/declarators/parameter-cv-same-function.cpp declarations/declarators/parameter-array-is-pointer.cpp declarations/functions/definitions/same-parameter-types-one-function.cpp

A parameter declared as an array of `T` has the type pointer to `T`. A
parameter declared as a function has the type pointer to that function.
The bound of an array parameter has no effect. A top-level cv-qualifier
of a parameter is not part of the function type.
:::

:::{rule} DCL-9
:kind: required
:tests: declarations/declarators/void-parameter-list.cpp declarations/declarators/variadic.cpp

The parameter list `(void)` declares no parameters, as `()` does. `...`
can be the last item in the list, and the comma before it is optional.
DCL-22 lists what is not valid.
:::

:::{rule} DCL-22
:kind: reject
:tests: declarations/declarators/void-parameter-named.cpp declarations/declarators/void-parameter-with-others.cpp declarations/declarators/ellipsis-first.cpp

`void` must not occur in a parameter list in forms other than the one
that DCL-9 gives. `...` must be the last item in the list.
:::

:::{rule} CV-1
:kind: required
:tests: declarations/declarators/cv-position.cpp declarations/declarators/cv-position-alias-and-levels.cpp

A cv-qualifier applies to the type on its left. If there is no type on
its left, the cv-qualifier applies to the type on its right. In a
pointer declarator, a cv-qualifier before `*` applies to the pointed-to
type. A cv-qualifier after `*` applies to the pointer.
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

A definition of a `const` variable of {term}`scalar type` must have an
initializer. An `extern` declaration that is not a definition needs no
initializer. Class types have a separate rule, {rule}`CTOR-4`.
:::

:::{rule} CV-4
:kind: reject
:tests: declarations/declarators/assign-to-const.cpp declarations/declarators/assign-through-pointer-to-const.cpp declarations/declarators/increment-const.cpp

A program must not do these to an lvalue of `const` type:

- assign to it,
- increment it,
- decrement it.

This rule applies to a `const` variable and to an access through a
pointer or reference to `const`.
:::

:::{rule} CV-6
:kind: required
:tests: declarations/declarators/cv-on-array-applies-to-elements.cpp declarations/declarators/cv-on-function-alias.cpp declarations/declarators/cv-on-function-template-parameter.cpp templates/deduction/function-type-const-dropped.cpp

A cv-qualifier applied to an array type through an alias or a template
parameter applies to the elements. A cv-qualifier applied to a function
type through an alias or a template parameter has no effect.
:::

:::{rule} CV-7
:kind: required
:tests: declarations/declarators/qualification-conversion-ok.cpp declarations/declarators/qualification-conversion-array.cpp

A pointer converts implicitly to a pointer with more cv-qualifiers on
the pointed-to type, and the cv-qualifiers of an array's elements count
as those of the array (CV-6). A pointer to an array of known bound also
converts to a pointer to an array of unknown bound. CV-10 lists what is
not valid.
:::

:::{rule} CV-10
:kind: reject
:tests: declarations/declarators/qualification-conversion-unsafe.cpp declarations/declarators/qualification-conversion-drops-const.cpp declarations/declarators/qualification-conversion-array-to-known.cpp declarations/declarators/qualification-conversion-array-inner-bound.cpp

Across several levels of pointers, `const` added at one level needs
`const` at each higher level except the outermost. An array bound
dropped at one level needs no `const` ({question}`Q-0191`). No implicit
conversion removes a cv-qualifier or adds an array bound.
:::

:::{rule} CV-8
:kind: required
:tests: declarations/declarators/const-integral-constant.cpp foundations/constant-evaluation/constant-initializer-usable.cpp

A `const` variable of integer or enumeration type with a
{term}`constant initializer` is {term}`usable in constant expressions`
({rule}`CE-3`). CV-11 lists what is not valid.
:::

:::{rule} CV-11
:kind: reject
:tests: declarations/declarators/const-integral-runtime.cpp declarations/declarators/const-floating-not-constant.cpp declarations/declarators/const-volatile-not-constant.cpp

These variables cannot occur in a constant expression:

- a `const` variable with an initializer that is not constant,
- a `const` floating-point variable,
- a `volatile` variable, even with a constant initializer.
:::

:::{rule} DCL-12
:kind: required
:tests: declarations/declarators/pointer-to-member.cpp declarations/declarators/pointer-to-member-function.cpp

A declarator of the form `C::* name` declares a pointer to a member of
class `C` ({rule}`MPTR-1`). DCL-23 lists what is not valid.
:::

:::{rule} DCL-23
:kind: reject
:tests: declarations/declarators/pointer-to-reference-member.cpp declarations/declarators/pointer-to-void-member.cpp

A pointer to member can name only a member that an object can hold. So
a program must not declare a pointer to member whose member would be a
reference or would have type `void`.
:::

:::{rule} DCL-13
:kind: required
:tests: declarations/declarators/function-declaration-ambiguity.cpp declarations/declarators/function-declaration-ambiguity-type.cpp declarations/initialization/forms/paren-temporary-declares-function.cpp

If the grammar ({rule}`TR-12`) derives a declaration as a function
declaration and in another way, the declaration is a function
declaration. This rule includes a parenthesized initializer that has the
form of a parameter list, such as a type name followed by `()`.
:::

:::{rule} DCL-14
:kind: reject
:tests: declarations/declarators/noexcept-in-type.cpp declarations/declarators/noexcept-redeclaration.cpp

The `noexcept` specification is part of the type of a function. All
declarations of a function must agree on the `noexcept` specification.
:::

:::{rule} DCL-15
:kind: required
:tests: declarations/declarators/qualified-function-type-member.cpp declarations/declarators/ref-qualified-function-type-member.cpp

A function type with a cv-qualifier or a ref-qualifier after its
parameter list can be named through a type alias. Such a type can
declare a non-static member function and can be the target of a pointer
to member.
:::

:::{rule} DCL-16
:kind: reject
:tests: declarations/declarators/qualified-function-type-pointer.cpp declarations/declarators/qualified-function-type-parameter.cpp

A function type with a cv-qualifier or a ref-qualifier must not be any
of these ({question}`Q-0106`):

- the target of a pointer or a reference,
- the type of a parameter,
- the type of a variable.
:::

## Runtime Behavior

:::{rule} DCL-11
:kind: required
:tests: declarations/declarators/reference-binds-same-object.cpp declarations/declarators/reference-to-function-and-element.cpp

A reference refers to an existing object or function. Each use of the
reference is a use of that object or function.
:::

## Undefined Behavior

:::{rule} CV-5
:kind: undefined
:tests: declarations/declarators/modify-const-object-via-cast.cpp declarations/declarators/modify-nonconst-object-via-cast.cpp declarations/declarators/modify-const-object-sanitizer.cpp expressions/casts/const-cast-write-const-object-sanitizer.cpp declarations/declarators/modify-const-object-constexpr-constructor-sanitizer.cpp declarations/declarators/modify-const-static-local-sanitizer.cpp

If a program changes an object that is defined `const`, the behavior is
undefined, also when a cast removes `const`. A program can change an
object that is not defined `const` through such a cast. The specific
cases are {rule}`CAST-5` and {rule}`PRIM-8` (decision 0052), and
{term}`constant evaluation` rejects both ({rule}`CE-9`).
:::

## Implementation Requirements

:::{rule} DCL-10
:kind: withdrawn

This rule merged into {rule}`TR-14`.
:::

## Recommended Practice

:::{rule} CV-9
:kind: recommended
:tests: declarations/declarators/volatile-return-deprecated.cpp declarations/volatile/volatile-return-warns.cpp declarations/volatile/volatile-parameter-warns.cpp declarations/volatile/pointer-to-volatile-parameter-ok.cpp

An implementation should warn about a return type or a parameter type
with a top-level `volatile`. These uses are deprecated. A pointer or a
reference to a `volatile` type has no top-level `volatile` and is valid.
:::

## Examples

% tidal-example: reject
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

int row[3];
const int (*all)[] = &row;     // const elements, bound dropped
int (*some)[3] = all;          // rejected: adds a bound and drops const

const int ci = 1;
const_cast<int&>(ci) = 2;      // undefined
int v = 1;
const int& cr = v;
const_cast<int&>(cr) = 2;      // v is 2

volatile int read();           // deprecated (CV-9)
void put(volatile int n);      // deprecated
volatile int* reg();           // valid: volatile is not top-level
void send(volatile int* to);   // valid
```

## Evidence

All eight compilers agree on 100 of 103 tests. The results are in
`tests/RESULTS.md`.

### Divergences

- {question}`Q-0106` (DCL-16; settled on DCL-P3): whether a parameter
  can be a reference to a function type with a ref-qualifier. It
  cannot, as Clang and GCC say. EDG 7.0 accepts the parameter.
- {question}`Q-0348` (DCL-15; settled on DCL-P3): whether a pointer to
  member declared through an alias of a function type with a
  ref-qualifier keeps the ref-qualifier. It does, as Clang and EDG say.
  GCC drops the ref-qualifier and rejects the initialization.
- {question}`Q-0191` (CV-10; open): whether an array bound dropped at
  an inner pointer level needs `const` at the levels above. It does
  not, as GCC and EDG say. Clang rejects the conversion. The choice
  waits on the maintainer (`documentation/decisions/pending.md`, row 4).

### Split tests

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `qualification-conversion-array-inner-bound.cpp` | CV-10 | accept | reject | accept | accept | {question}`Q-0191` |
| `qualified-function-type-parameter.cpp` | DCL-16 | reject | reject | reject | accept | {question}`Q-0106` |
| `ref-qualified-function-type-member.cpp` | DCL-15 | run | run | reject | accept | {question}`Q-0348` |

### Notes

- DCL-19: Clang and GCC accept variable-length arrays and arrays of
  size zero as extensions. Both reject these arrays only under
  `-pedantic-errors`. EDG rejects both in all modes.
- CV-5: all compilers reject the change during constant evaluation.
  Their notes give the cause ("modification of object of const-qualified
  type", "modifying a const object", "attempt to modify const storage").
  A change to an object that is not `const`, through the same cast, is
  accepted and executes.
- CV-7: every compiler converts a pointer to an array of `int` to a
  pointer to an array of `const int`, with or without the bound.
- CV-5: the address sanitizer of GCC and Clang stops a write to a
  `const` object with static storage duration and a constant
  initializer, because the object is in storage that cannot be written.
  The rule holds for a scalar, for a class object with a braced
  initializer or a `constexpr` constructor, and for a block-scope
  `static` variable. A `const` object whose initializer runs at run
  time, such as a call or a constructor that is not `constexpr`, is in
  storage that can be written. A class object with a `mutable` member
  is in such storage too. No sanitizer stops a write to these objects,
  or to a `const` automatic object.
- CV-9: Clang and GCC warn by default about a `volatile` parameter
  type and a `volatile` return type, also under `-Wall` and
  `-Wdeprecated`. The flag is `-Wdeprecated-volatile` for Clang and
  `-Wvolatile` for GCC. Neither warns about a pointer to a `volatile`
  type.

### Sources

Inputs: clang-sema-type, clang-sema-decl, edg-decls, clang-diag-sema
(see `inputs/MANIFEST.md`).
