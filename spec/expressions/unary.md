# Unary operators

A unary operator takes one operand. This page covers `+`, `-`, `*`,
`&`, prefix `++` and `--`, `sizeof`, `alignof`, and `noexcept`.
The operators `!` and `~` are on the logical and bitwise pages.

## Syntax

```{eval-rst}
.. productionlist:: unary
   unary_expr: `postfix_expr`
             : | ("+" | "-" | "*" | "&" | "++" | "--") `unary_expr`
             : | "sizeof" `unary_expr` | "sizeof" "(" `type_id` ")"
             : | "alignof" "(" `type_id` ")"
             : | "noexcept" "(" `expression` ")"
```

## Constraints

:::{rule} UN-1
:kind: required
:tests: expressions/unary/plus-minus-types.cpp expressions/unary/plus-pointer.cpp expressions/unary/minus-bool.cpp

The operand of unary `+` or `-` can have an arithmetic or unscoped
enumeration type. The operand goes through {term}`integer promotion`, and
the result has the promoted type. Unary `+` also accepts a pointer, and an
array operand becomes a pointer.
:::

:::{rule} UN-2
:kind: reject
:tests: expressions/unary/plus-minus-scoped-enum.cpp expressions/unary/minus-pointer.cpp

The operand of unary `-` must not be a pointer. The operand of unary `+`
or `-` must not have a scoped enumeration type.
:::

:::{rule} UN-5
:kind: required
:tests: expressions/unary/indirection-lvalue.cpp expressions/unary/indirection-function.cpp expressions/unary/indirection-incomplete-lvalue.cpp

`*p` gives an lvalue for the object or function that `p` points to. The
type that `p` points to can be incomplete.
:::

:::{rule} UN-6
:kind: reject
:tests: expressions/unary/indirection-void-pointer.cpp expressions/unary/indirection-non-pointer.cpp expressions/unary/indirection-nullptr-literal.cpp expressions/unary/indirection-incomplete-value.cpp

The operand of `*` must be a pointer, and must not be a pointer to
`void`. A program must not read the value of an object of incomplete type
through the result.
:::

:::{rule} UN-8
:kind: required
:tests: expressions/unary/address-of.cpp expressions/unary/address-of-qualified-member.cpp

The operand of `&` is an lvalue or a qualified name of a class member. For
an lvalue of type `T`, the result is a pointer to `T` with the same
{term}`cv-qualifiers <cv-qualifier>`. For a qualified member name, the
result is a pointer to member *(classes referenced)*.
:::

:::{rule} UN-9
:kind: reject
:tests: expressions/unary/address-of-rvalue.cpp

The operand of `&` must not be a prvalue or an xvalue.
:::

:::{rule} UN-10
:kind: required
:tests: expressions/unary/prefix-increment-lvalue.cpp

For prefix `++` and `--`, these conditions apply:

- The operand must be an lvalue that the program can change.
- The operand must have arithmetic type, other than `bool`, or be a
  pointer to an object.
- The result is the operand itself, as an lvalue.
:::

:::{rule} UN-11
:kind: reject
:tests: expressions/unary/increment-bool.cpp expressions/unary/decrement-bool.cpp expressions/unary/increment-enum.cpp expressions/unary/increment-const.cpp expressions/unary/increment-rvalue.cpp

The operand of prefix `++` or `--` must not have type `bool` or an
enumeration type. It must not be `const` or an rvalue.
:::

:::{rule} UN-12
:kind: required
:tests: expressions/unary/sizeof-basic.cpp

`sizeof` gives the size in bytes of a type or of the type of an
expression. These rules apply:

- `char`, `signed char`, and `unsigned char` have size 1.
- An array has the size of all its elements.
- A reference type gives the size of the referenced type.
- The result has the type of `sizeof 0`, an unsigned integer type.
:::

:::{rule} UN-14
:kind: reject
:tests: expressions/unary/sizeof-function.cpp expressions/unary/sizeof-incomplete.cpp expressions/unary/sizeof-void.cpp

The operand of `sizeof` must not have a function type, an incomplete
type, or the type `void`.
:::

:::{rule} UN-16
:kind: required
:tests: expressions/unary/alignof-basic.cpp

`alignof` gives the alignment of a type, as a power of two. `char` has
alignment 1. An array has the alignment of its element type. A reference
type gives the alignment of the referenced type.
:::

:::{rule} UN-17
:kind: reject
:tests: expressions/unary/alignof-expression.cpp expressions/unary/alignof-function.cpp

The operand of `alignof` must be a type, and must not be a function type.
:::

:::{rule} UN-18
:kind: required
:tests: expressions/unary/noexcept-operator.cpp

`noexcept(e)` is a constant of type `bool`. It is `false` if `e`
contains:

- a call to a function that is not `noexcept`,
- a throw-expression.

Otherwise it is `true`.
:::

## Dynamic Semantics

:::{rule} UN-3
:kind: required
:tests: expressions/unary/minus-values.cpp

Unary `-` gives the negative of its operand. For an unsigned type, the
result is 2{sup}`N` minus the value, where N is the width of the type.
:::

:::{rule} UN-13
:kind: required
:tests: expressions/unary/sizeof-not-evaluated.cpp expressions/unary/noexcept-not-evaluated.cpp

The operands of `sizeof` and `noexcept` are not evaluated. Their side
effects do not occur.
:::

:::{rule} UN-20
:kind: required
:tests: expressions/unary/prefix-increment.cpp

Prefix `++` adds 1 to its operand, and prefix `--` subtracts 1. For a
pointer, the result points to the next or previous element of the array.
:::

## Undefined Behavior

:::{rule} UN-4
:kind: undefined
:tests: expressions/unary/minus-int-min.cpp

If the operand of unary `-` has a signed type and its negative does not
fit in that type, the behavior is undefined.
:::

:::{rule} UN-7
:kind: undefined
:tests: expressions/unary/indirection-null.cpp

If the operand of `*` is a null pointer, the behavior is undefined.
:::

## Implementation Requirements

:::{rule} UN-15
:kind: impl-defined
:tests: expressions/unary/sizeof-unsigned-lp64.cpp expressions/unary/sizeof-unsigned-ilp32.cpp

The sizes of types other than the character types are
implementation-defined. This includes the size of the type of `sizeof 0`.
:::

## Recommended Practice

:::{rule} UN-19
:kind: recommended
:tests: expressions/unary/sizeof-array-parameter-warns.cpp expressions/unary/sizeof-array-reference-silent.cpp

An implementation should warn if `sizeof` applies to a parameter declared
as an array, because the parameter is a pointer (DCL-8).
:::

## Examples

```cpp
char c = 1;
+c;                     // int
-1u;                    // ~0u
int arr[3];
+arr;                   // int*

int i = 0;
sizeof(i++);            // i stays 0
++i = 5;                // fine: ++i is an lvalue
bool b = false;
++b;                    // rejected

void may_throw();
noexcept(may_throw());  // false
alignof(char);          // 1
```

## Evidence

All seven compilers agree on all 39 tests. The results are in
`tests/RESULTS.md`. EDG skips the tests that use `-m32` or `-Werror`.

- UN-4, UN-7: all compilers reject the operation during
  {term}`constant evaluation`.
- UN-15: `sizeof(void*)` and `sizeof(sizeof 0)` are 8 on x86-64 and 4
  with `-m32`.
- UN-17: Clang and GCC accept `alignof` of an expression or a function
  type as an extension. They reject it only under `-pedantic-errors`.
- UN-19: Clang and GCC warn with `-Wsizeof-array-argument`.

Inputs: clang-sema-expr, clang-diag-sema.
