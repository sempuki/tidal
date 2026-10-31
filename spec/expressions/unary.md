# Unary operators

A unary operator takes one operand. This page covers these operators:

- `+` and `-`,
- `*` and `&`,
- prefix `++` and `--`,
- `sizeof`, `alignof`, and `noexcept`.

The operators `!` and `~` are on the logical and bitwise pages.

## Principles

:::{principle} UN-P1
:rules: UN-1 UN-2 UN-3 UN-4

Unary `+` and `-` are arithmetic, so their operand is promoted and the
result has the promoted type. Negating an unsigned value wraps, because
the type is modular. Negating the most negative value of a signed type
has no result in that type, so it is undefined. A pointer can be the
operand of `+`, which only converts it, and not of `-`.
:::

:::{principle} UN-P2
:rules: UN-5 UN-6 UN-7 UN-8 UN-9

`&` and `*` go between an object and its address:

```text
                 &
   object  ------------>  pointer
   (lvalue)  <-----------
                 *
```

`&` takes an lvalue, because only an object with an identity has an
address ({principle}`VC-P1`), and gives a pointer with the object's
cv-qualifiers. `*` takes a pointer and gives back the object as an
lvalue, so the two undo each other. A pointer to `void` points at no
particular object and cannot be dereferenced. A pointer that points at
nothing gives no object, and using the result is undefined. Such
pointers include:

- a null pointer,
- a past-the-end pointer,
- a pointer to a dead object.
:::

:::{principle} UN-P3
:rules: UN-10 UN-11 UN-20 UN-21

Prefix `++` and `--` change their operand in place by one and hand the
same object back as an lvalue. So the operand has to be something a
program can change, and its type has to have a notion of "one more".

| Operand type | Has "one more" |
|---|---|
| A number other than `bool` | Yes |
| `bool` | No. It has only two values. |
| A pointer to a complete type | Yes. One more means the next element (ADD-P3). |
| An enumeration | No |

Where the
step overflows the type or leaves the array, the result is undefined,
as it is for `+` and `-`.
:::

:::{principle} UN-P4
:rules: UN-12 UN-13 UN-14 UN-15 UN-16 UN-17 UN-19

`sizeof` and `alignof` ask a question about a type and answer it at
translation time. Their operands are never evaluated, so side effects in
them do not happen. The question needs a complete object type. These
have no size or alignment:

- a function type,
- `void`,
- an incomplete class.

A reference answers for the type it refers to, and an array for all its
elements. The sizes of most types are an implementation's choice, with
the character types fixed at one. An implementation should warn when
`sizeof` is applied to a parameter declared as an array, since that
parameter is a pointer.
:::

:::{principle} UN-P5
:rules: UN-18

`noexcept(e)` asks whether anything in `e` can throw, counting the
implicit calls that `e` makes, and answers with a `bool` constant. Like
`sizeof`, it never evaluates its operand.
:::

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
:tests: expressions/unary/plus-minus-types.cpp expressions/unary/plus-pointer.cpp expressions/unary/minus-bool.cpp expressions/unary/plus-function-operand.cpp

The operand of unary `+` or `-` can have an arithmetic or unscoped
enumeration type. The operand goes through {term}`integer promotion`, and
the result has the promoted type. Unary `+` also accepts a pointer. An
array operand becomes a pointer to its first element, and a function
operand becomes a pointer to the function.
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
result is a pointer to member ({rule}`MPTR-2`).
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
:tests: expressions/unary/increment-bool.cpp expressions/unary/decrement-bool.cpp expressions/unary/increment-enum.cpp expressions/unary/increment-const.cpp expressions/unary/increment-rvalue.cpp expressions/unary/increment-incomplete-pointer.cpp expressions/unary/increment-function-pointer.cpp

The operand of prefix `++` or `--` must obey these rules:

- It must not have type `bool` or an enumeration type.
- It must not be `const` or an rvalue.
- A pointer operand must point to a complete object type, so a pointer
  to a function or to an incomplete class is not valid.
:::

:::{rule} UN-12
:kind: required
:tests: expressions/unary/sizeof-basic.cpp expressions/unary/sizeof-at-least-one.cpp

`sizeof` gives the size in bytes of a type or of the type of an
expression. These rules apply:

- `char`, `signed char`, and `unsigned char` have size 1.
- An array has the size of all its elements.
- A reference type gives the size of the referenced type.
- The result has the type of `sizeof 0`, an unsigned integer type.
- The size of a complete object type is at least 1, also for an empty
  class. The size of a class includes padding.
:::

:::{rule} UN-14
:kind: reject
:tests: expressions/unary/sizeof-function.cpp expressions/unary/sizeof-incomplete.cpp expressions/unary/sizeof-void.cpp expressions/decltype/incomplete-return-sizeof.cpp

The operand of `sizeof` must not have one of these types:

- a function type,
- an incomplete type,
- the type `void`.
:::

:::{rule} UN-16
:kind: required
:tests: expressions/unary/alignof-basic.cpp expressions/unary/alignof-unknown-bound.cpp

`alignof` gives the alignment of a type, as a power of two. These rules
apply:

- `char` has alignment 1.
- An array, of known or unknown bound, has the alignment of its element
  type.
- A reference type gives the alignment of the referenced type.
:::

:::{rule} UN-17
:kind: reject
:tests: expressions/unary/alignof-expression.cpp expressions/unary/alignof-function.cpp expressions/unary/alignof-incomplete.cpp expressions/unary/alignof-void.cpp

The operand of `alignof` must be a complete object type or a reference.
It must not be one of these:

- a function type,
- `void`,
- an incomplete class.
:::

:::{rule} UN-18
:kind: required
:tests: expressions/unary/noexcept-operator.cpp expressions/unary/noexcept-new-false.cpp expressions/unary/noexcept-temporary-ctor-false.cpp expressions/unary/noexcept-conversion-false.cpp expressions/unary/noexcept-dynamic-cast-reference.cpp

`noexcept(e)` is a constant of type `bool`. It is `false` if `e`
contains:

- a call to a function that is not `noexcept`, also an implicit call such
  as the constructor of a temporary or a conversion function,
- a `new` expression whose allocation function is not `noexcept`, such
  as the default one,
- a `dynamic_cast` to a reference type that can fail,
- a throw-expression ({rule}`THR-5`).

Otherwise it is `true`.
:::

## Runtime Behavior

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
:tests: expressions/unary/minus-int-min.cpp expressions/unary/minus-int-min-sanitizer.cpp

If the operand of unary `-` has a signed type and its negative does not
fit in that type, the behavior is undefined.
:::

:::{rule} UN-7
:kind: undefined
:tests: expressions/unary/indirection-null.cpp expressions/unary/indirection-past-end-constant.cpp expressions/unary/indirection-dangling-constant.cpp expressions/unary/indirection-null-sanitizer.cpp foundations/objects/read-after-block-ends-sanitizer.cpp

If the operand of `*` does not point to an object or a function, and the
program uses the result, the behavior is undefined. This includes:

- a null pointer,
- a pointer past the end of an array,
- a pointer to an object whose lifetime has ended.
:::

:::{rule} UN-21
:kind: undefined
:tests: expressions/unary/increment-overflow-constant.cpp expressions/unary/decrement-overflow-constant.cpp expressions/unary/increment-pointer-past-end-constant.cpp expressions/unary/increment-overflow-sanitizer.cpp

The behavior of prefix `++` and `--` is undefined in these cases:

- The result for a signed integer is outside the range of its type.
- A pointer moves outside its array, other than to one past the last
  element ({question}`Q-0031`).
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

All seven compilers agree on 53 of 54 tests. The results are in
`tests/RESULTS.md`. EDG skips the tests that use `-m32` or `-Werror`.

- UN-21: GCC accepts a pointer moved two past the end of an array during
  constant evaluation. Clang and EDG reject it ({question}`Q-0031`).

- UN-4, UN-7: all compilers reject the operation during
  {term}`constant evaluation`.
- UN-15: `sizeof(void*)` and `sizeof(sizeof 0)` are 8 on x86-64 and 4
  with `-m32`.
- UN-17: Clang and GCC accept `alignof` of an expression or a function
  type as an extension. They reject it only under `-pedantic-errors`.
- UN-19: Clang and GCC warn with `-Wsizeof-array-argument`.

Inputs: clang-sema-expr, clang-diag-sema.
