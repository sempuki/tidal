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
has no result in that type, so the negation is undefined. A pointer
can be the operand of `+`, which only converts the pointer, and not of
`-`.
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
program can change, and the type of the operand has to have a notion
of "one more".

| Operand type | Has "one more" |
|---|---|
| A number other than `bool` | Yes |
| `bool` | No. `bool` has only two values. |
| A pointer to a complete type | Yes. One more means the next element (ADD-P3). |
| An enumeration | No |

The step works as `+ 1` and `- 1` do. So an unsigned value wraps
around, and a value of a type narrower than `int` is promoted, stepped,
and converted back to its type. Where the step overflows a signed type
of `int` or wider, or leaves the array, the result is undefined, as it
is for `+` and `-`.
:::

:::{principle} UN-P4
:rules: UN-12 UN-13 UN-14 UN-15 UN-16 UN-17 UN-19

`sizeof` and `alignof` ask a question about a type and answer it at
translation time. Their operands are never evaluated, so {term}`side effects <side effect>` in
the operands do not happen. The question needs a complete object type.
These types have no size or alignment:

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
`sizeof`, `noexcept` never evaluates its operand.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   unary_expr: `postfix_expr`
             : | ("+" | "-" | "*" | "&" | "!" | "~") `cast_expr`
             : | ("++" | "--") `unary_expr`
             : | "sizeof" `unary_expr` | "sizeof" "(" `type_id` ")"
             : | "sizeof" "..." "(" `identifier` ")"
             : | "alignof" "(" `type_id` ")"
             : | "noexcept" "(" `expression` ")"
             : | `new_expr` | `delete_expr`
```

`sizeof...` counts the elements of a pack ({rule}`PACK-19`).

## Constraints

:::{rule} UN-1
:kind: required
:tests: expressions/unary/plus-minus-types.cpp expressions/unary/plus-pointer.cpp expressions/unary/minus-bool.cpp expressions/unary/plus-function-operand.cpp expressions/lambdas/unary-plus-function-pointer.cpp expressions/unary/minus-narrow-unsigned-promoted.cpp

The operand of unary `+` or `-` can have an arithmetic or unscoped
enumeration type. The operand goes through {term}`integer promotion`,
and the result has the promoted type. Unary `+` also accepts a pointer.
An array operand becomes a pointer to its first element, and a function
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
type that `p` points to can be incomplete. A function operand of `*`
becomes a pointer to the function first, so `**f` and `*f` give the
same function.
:::

:::{rule} UN-6
:kind: reject
:tests: expressions/unary/indirection-void-pointer.cpp expressions/unary/indirection-non-pointer.cpp expressions/unary/indirection-nullptr-literal.cpp expressions/unary/indirection-incomplete-value.cpp

The operand of `*` must be a pointer or a function (UN-5), and must
not be a pointer to `void`. A program must not read the value of an object of incomplete
type through the result.
:::

:::{rule} UN-8
:kind: required
:tests: expressions/unary/address-of.cpp expressions/unary/address-of-qualified-member.cpp

The operand of `&` is an lvalue or a qualified name of a class member.
For an lvalue of type `T`, the result is a pointer to `T` with the same
{term}`cv-qualifiers <cv-qualifier>`. For a qualified member name, the
result is a pointer to member ({rule}`MPTR-2`).
:::

:::{rule} UN-9
:kind: reject
:tests: expressions/unary/address-of-rvalue.cpp expressions/unary/address-of-class-prvalue.cpp

The operand of `&` must not be a prvalue or an xvalue.
:::

:::{rule} UN-10
:kind: required
:tests: expressions/unary/prefix-increment-lvalue.cpp expressions/unary/prefix-decrement-pointer-double.cpp

For prefix `++` and `--`, these conditions apply:

- The operand must be an lvalue that the program can change.
- The operand must have {term}`arithmetic type`, other than `bool`, or be a
  pointer to an object.
- The result is the operand itself, as an lvalue.
:::

:::{rule} UN-11
:kind: reject
:tests: expressions/unary/increment-bool.cpp expressions/unary/decrement-bool.cpp expressions/unary/increment-enum.cpp expressions/unary/increment-const.cpp expressions/unary/increment-rvalue.cpp expressions/unary/increment-incomplete-pointer.cpp expressions/unary/increment-function-pointer.cpp

The operand of prefix `++` or `--` must obey these rules:

- The operand must not have type `bool` or an enumeration type.
- The operand must not be `const` or an rvalue.
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

The operand of `alignof` must be a type. The type must be a complete
object type, a reference, or an array of unknown bound (UN-16). The
operand must not be one of these:

- an expression,
- a function type,
- `void`,
- an incomplete class.
:::

:::{rule} UN-18
:kind: required
:tests: expressions/unary/noexcept-operator.cpp expressions/unary/noexcept-new-false.cpp expressions/unary/noexcept-temporary-ctor-false.cpp expressions/unary/noexcept-conversion-false.cpp expressions/unary/noexcept-dynamic-cast-reference.cpp expressions/unary/noexcept-dynamic-cast-to-base.cpp expressions/unary/noexcept-temporary-destructor-false.cpp expressions/unary/noexcept-rewritten-comparison-false.cpp expressions/new-delete/global-functions-specifiers.cpp expressions/unary/noexcept-destructor-alone.cpp

`noexcept(e)` is a constant of type `bool`. The constant is `false` if
`e` contains:

- a call to a function that is not `noexcept`, also an implicit call
  from the list in {rule}`SEQ-9`, such as the constructor or the
  destructor of a temporary, a conversion function, or a rewritten
  comparison,
- a `new` expression whose allocation function is not `noexcept`, such
  as the default one,
- a `dynamic_cast` to a reference type that checks at run time
  ({rule}`VIRT-10`). A cast to a reference to the class of the operand,
  or to an accessible unambiguous base of that class, makes no check.
- a throw-expression ({rule}`THR-5`).

Otherwise the constant is `true`. A destructor without a `noexcept`
specifier is `noexcept` if the destructors of its bases and members are
({rule}`DTOR-4`). The global `operator delete` forms are `noexcept`
({rule}`NEW-19`).
:::

## Runtime Behavior

:::{rule} UN-3
:kind: required
:tests: expressions/unary/minus-values.cpp expressions/unary/minus-values-wide-unsigned.cpp expressions/unary/minus-narrow-unsigned-promoted.cpp

Unary `-` gives the negative of its promoted operand (UN-1). If the
promoted operand has an unsigned type, the result is 2{sup}`N` minus
the value, where N is the width of the promoted type.
:::

:::{rule} UN-13
:kind: required
:tests: expressions/unary/sizeof-not-evaluated.cpp expressions/unary/noexcept-not-evaluated.cpp expressions/unary/noexcept-destructor-alone.cpp

The operands of `sizeof` and `noexcept` are not evaluated. Their side
effects do not occur.
:::

:::{rule} UN-20
:kind: required
:tests: expressions/unary/prefix-increment.cpp expressions/unary/prefix-unsigned-wraps.cpp expressions/unary/prefix-narrow-signed-wraps.cpp expressions/postfix/postfix-unsigned-wraps.cpp expressions/postfix/postfix-narrow-signed-wraps.cpp

Prefix `++` adds 1 to its operand, and prefix `--` subtracts 1. For a
pointer, the result points to the next or previous element of the array.
For an integer operand, these rules apply:

- For an unsigned type, the result wraps around. `++` on the largest
  value gives 0, and `--` on 0 gives the largest value.
- For a type that {term}`integer promotion` changes, the step uses the
  promoted type. The result then converts to the operand's type
  ({rule}`CONV-2`, {question}`Q-0124`).
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
:tests: expressions/unary/indirection-null.cpp expressions/unary/indirection-past-end-constant.cpp expressions/unary/indirection-null-sanitizer.cpp expressions/postfix/member-call-null-pointer-sanitizer.cpp expressions/postfix/member-call-null-pointer-constant.cpp foundations/storage/integer-pointer-no-object-sanitizer.cpp

If the operand of `*` does not point to an object or a function, and the
program uses the result, the behavior is undefined. Such operands
include:

- a null pointer,
- a pointer past the end of an array,
- a pointer to an object whose {term}`lifetime` has ended.

For a read or write of an object whose lifetime has ended,
{rule}`OBJ-2` is the rule that applies. The specific cases of UN-7 are
{rule}`PFX-21` for a member call and {rule}`REF-5` for a reference
binding, each through a null pointer (decision 0052).
:::

:::{rule} UN-21
:kind: undefined
:tests: expressions/unary/increment-overflow-constant.cpp expressions/unary/decrement-overflow-constant.cpp expressions/unary/increment-pointer-past-end-constant.cpp expressions/unary/increment-overflow-sanitizer.cpp

The behavior of prefix `++` and `--` is undefined in these cases:

- The operand has a signed type that integer promotion does not change,
  and the result is outside the range of that type.
- A pointer moves outside its array, other than to one past the last
  element ({question}`Q-0031`).
:::

## Implementation Requirements

:::{rule} UN-15
:kind: impl-defined
:tests: expressions/unary/sizeof-unsigned-lp64.cpp expressions/unary/sizeof-unsigned-ilp32.cpp

The sizes of types other than the character types are
implementation-defined. These sizes include the size of the type of
`sizeof 0`. {rule}`TYP-1` gives the sizes on the matrix targets.
:::

## Recommended Practice

:::{rule} UN-19
:kind: recommended
:tests: expressions/unary/sizeof-array-parameter-warns.cpp expressions/unary/sizeof-array-reference-silent.cpp

An implementation should warn if `sizeof` applies to a parameter
declared as an array, because the parameter is a pointer (DCL-8).
:::

## Examples

% tidal-example: reject
```cpp
char c = 1;
+c;                     // int
-1u;                    // ~0u
int arr[3];
+arr;                   // int*

int i = 0;
sizeof(i++);            // i stays 0
++i = 5;                // fine: ++i is an lvalue
unsigned u = 0;
--u;                    // the largest unsigned value
bool b = false;
++b;                    // rejected

void may_throw();
noexcept(may_throw());  // false
struct Calm { ~Calm() {} };
noexcept(Calm());       // true: ~Calm is noexcept (DTOR-4)
struct Loud { ~Loud() noexcept(false); };
noexcept(static_cast<Loud*>(nullptr)->~Loud());  // false: asks about
                                                 // ~Loud alone
void take(Loud&) noexcept;
noexcept(take(*static_cast<Loud*>(nullptr)));    // true: the operand is
                                                 // not evaluated (UN-13)
alignof(char);          // 1
alignof(double[]);      // alignof(double)
alignof(i);             // rejected: the operand is an expression

int one();
int (*f)() = one;
(**f)();                // 1: *f is a function, which becomes a pointer again

int data[6];
sizeof(data) / sizeof(data[0]);  // 6: the element count
void fill(int a[6]) {
  sizeof(a);                     // size of int*: should warn (UN-19)
}
template <class T, decltype(sizeof 0) N>
constexpr auto count(const T (&)[N]) { return N; }
count(data);                     // 6: a reference keeps the length
```

## Evidence

All eight compilers agree on 71 of 75 tests. The results are in
`tests/RESULTS.md`. EDG skips the tests that use `-m32` or `-Werror`.

### Divergences

- {question}`Q-0124` (UN-20; open): whether `++` or `--` on a `short`
  or `signed char` is defined when the new value does not fit. It is,
  and the value converts back to the type, as Clang and GCC do. EDG
  rejects the step during constant evaluation.
- {question}`Q-0031` (UN-21; settled on {principle}`ADD-P6`): whether
  forming a pointer outside an array is undefined without a read
  through it. It is, so constant evaluation rejects a pointer moved two
  past the end, as Clang and EDG do. GCC accepts it.
- {question}`Q-0318` (UN-7, PFX-21; open): whether constant evaluation
  must reject a member function call through a null pointer. It must,
  as Clang and GCC do. EDG accepts the call.

### Split tests

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `expressions/postfix/member-call-null-pointer-constant.cpp` | PFX-21, UN-7, CE-9 | reject | reject | reject | accept | {question}`Q-0318` |
| `expressions/postfix/postfix-narrow-signed-wraps.cpp` | PFX-12, UN-20 | accept | accept | accept | reject | {question}`Q-0124` |
| `increment-pointer-past-end-constant.cpp` | UN-21 | reject | reject | accept | reject | {question}`Q-0031` |
| `prefix-narrow-signed-wraps.cpp` | UN-20 | accept | accept | accept | reject | {question}`Q-0124` |

### Notes

- UN-4, UN-7: all compilers reject the operation during
  {term}`constant evaluation`.
- UN-4, UN-7, UN-21: with `-fsanitize=undefined`, Clang and GCC stop
  these cases at run time. The reports are "negation of ... cannot be
  represented" for `-INT_MIN`, "load of null pointer" for a read
  through a null pointer, and "signed integer overflow" for `++` past
  the largest value. EDG does not run programs.
- UN-15: `sizeof(void*)` and `sizeof(sizeof 0)` are 8 on x86-64 and 4
  with `-m32`.
- UN-17: Clang and GCC accept `alignof` of an expression or a function
  type as an extension. They reject it only under `-pedantic-errors`.
- UN-19: Clang and GCC warn with `-Wsizeof-array-argument`.

Inputs: clang-sema-expr, clang-diag-sema.
