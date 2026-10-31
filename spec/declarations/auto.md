# Placeholder types in variable declarations

A variable declaration can use a {term}`placeholder type`, `auto` or
`decltype(auto)`, in the position of a type. The type of the variable
then comes from its initializer.

This page covers variables, including variables in conditions and
initializer statements. Placeholders in these positions are on other
pages *(templates referenced)*:

- return types,
- parameters,
- template parameters.

## Principles

:::{principle} AUTO-P1
:rules: AUTO-1 AUTO-2

A variable declared with `auto` holds a value, so its type is the type
of the value that the initializer produces.

| In the initializer's type | In the variable's type | Why |
|---|---|---|
| A reference | Dropped | The variable is a new object and does not refer to the old one. |
| Top-level `const` and `volatile` | Dropped | The qualifiers belonged to the old object. |
| An array or a function | The pointer that the initializer converts to ({principle}`VC-P1`) | An array or a function cannot be held as a value. |

:::

:::{principle} AUTO-P2
:rules: AUTO-3 AUTO-18 AUTO-4 AUTO-5 AUTO-19 AUTO-13 AUTO-16

Anything that the declaration writes around `auto` applies to the
deduced type.

| Written around `auto` | Effect |
|---|---|
| `auto&` | Makes a reference, so the reference keeps the initializer's type whole, with its qualifiers and without decay. The reference needs something to refer to. |
| `auto&&` | Binds to anything. The reference becomes an lvalue or an rvalue reference as the initializer is one or the other. |
| `const`, `volatile`, `*` | Adds to the deduced type as it would to a written one. |
| A concept before `auto` | Checks the result. |

:::

:::{principle} AUTO-P3
:rules: AUTO-6 AUTO-20 AUTO-7 AUTO-21 AUTO-10

The form of the initializer sets how the type is read from the
initializer.

| Form | How the type is read |
|---|---|
| Parentheses, or braces without `=` | The parentheses or braces hold one expression, and the type of that expression is read. |
| A braced list after `=` | The initializer is a list, so the variable becomes a `std::initializer_list`. |
| `decltype(auto)` | As `decltype` reads the initializer. A name gives its declared type. For any other expression, the value category decides between a reference and a plain type. |

:::

:::{principle} AUTO-P4
:rules: AUTO-8 AUTO-9 AUTO-22 AUTO-11 AUTO-15

Deduction must have exactly one answer. These constraints follow from
the principle:

- The variable needs an initializer.
- The initializer must have a type to give (so no `void`, no
  overloaded name, and no use of the variable itself).
- `auto` cannot be mixed with another type or spread over an array.
- Several declarators in one declaration must agree on what `auto`
  stands for.
- `decltype(auto)` takes no decoration and no list, and it can name an
  array type, which no initializer can copy.
:::

:::{principle} AUTO-P5
:rules: AUTO-12 AUTO-14 AUTO-17

A placeholder means the same thing wherever a variable is declared:

- in a condition,
- in the initializer statement of `if`, `switch`, or `for`,
- in a static data member that carries its initializer in the class.
:::

## Syntax

```{eval-rst}
.. productionlist:: auto
   placeholder: [`type_constraint`] ("auto" | "decltype" "(" "auto" ")")
   auto_var_decl: `decl_specifier`* `placeholder` `decl_specifier`*
                : `auto_declarator` ("," `auto_declarator`)* ";"
   auto_declarator: `declarator` `auto_init`
   auto_init: "=" `expression`
            : | "=" "{" [`expression` ("," `expression`)*] "}"
            : | "(" `expression` ")"
            : | "{" `expression` "}"
```

`decl_specifier` includes `const`, `volatile`, `static`, `constexpr`,
and the other declaration specifiers. `type_constraint` names a concept
*(concepts referenced)*.

## Constraints

:::{rule} AUTO-1
:kind: required
:tests: declarations/auto/drops-references-and-cv.cpp declarations/auto/keeps-pointee-cv.cpp declarations/auto/nullptr.cpp

For `auto` with `= expression`, the type is the type of the initializer
without its reference. Then the top-level `const` and `volatile` are
removed. Qualifiers below the top level stay.
:::

:::{rule} AUTO-2
:kind: required
:tests: declarations/auto/decays-arrays-and-functions.cpp declarations/auto/string-literal-prefixes.cpp

These initializers give a pointer:

| Initializer | Pointer |
|---|---|
| Of array type | A pointer to its first element |
| A function | A pointer to the function |
| A string literal | A pointer to `const` of its character type, for example `const char*` or `const char8_t*` |
:::

:::{rule} AUTO-3
:kind: required
:tests: declarations/auto/lvalue-reference.cpp declarations/auto/const-lvalue-reference-to-rvalue.cpp declarations/auto/ref-const-xvalue.cpp

`auto&` gives an lvalue reference to the type of the initializer. The
type keeps its `const` and `volatile`, and arrays and functions do not
decay. AUTO-18 states what the initializer must be.
:::

:::{rule} AUTO-18
:kind: reject
:tests: declarations/auto/lvalue-reference-to-rvalue.cpp

The initializer of `auto&` (AUTO-3) must be an lvalue, unless the
deduced type is `const`. The `const` can come from the declaration or
from the initializer, so an xvalue of `const` type is valid.
:::

:::{rule} AUTO-4
:kind: required
:tests: declarations/auto/forwarding-reference.cpp

`auto&&` accepts any initializer. An lvalue initializer gives `T&`. Any
other initializer gives `T&&`. The type keeps the `const` and `volatile`
of the initializer.
:::

:::{rule} AUTO-5
:kind: required
:tests: declarations/auto/added-qualifiers.cpp declarations/auto/constexpr-adds-const.cpp

`const`, `volatile`, and `*` written with `auto` apply to the deduced
type. `constexpr` makes the variable `const`. AUTO-19 states what
`auto*` needs.
:::

:::{rule} AUTO-19
:kind: reject
:tests: declarations/auto/pointer-from-non-pointer.cpp

With `auto*` (AUTO-5), the initializer must have pointer type.
:::

:::{rule} AUTO-6
:kind: required
:tests: declarations/auto/braced-direct-single.cpp declarations/auto/paren-single.cpp

An initializer in parentheses, or in braces without `=`, can hold one
expression. That expression gives the type, as an expression does after
`=`. AUTO-20 lists what is not valid.
:::

:::{rule} AUTO-20
:kind: reject
:tests: declarations/auto/braced-direct-multiple.cpp declarations/auto/braced-direct-empty.cpp declarations/auto/paren-multiple.cpp declarations/auto/paren-braced.cpp

An initializer in parentheses, or in braces without `=` (AUTO-6), must
contain exactly one expression. A braced list in parentheses is not
permitted.
:::

:::{rule} AUTO-7
:kind: required
:tests: declarations/auto/copy-list-deduces-initializer-list.cpp

`auto x = { ... };` gives `x` the type `std::initializer_list<T>`.
AUTO-21 lists what is not valid.
:::

:::{rule} AUTO-21
:kind: reject
:tests: declarations/auto/copy-list-mixed-types.cpp declarations/auto/copy-list-without-library.cpp declarations/auto/copy-list-empty.cpp

In `auto x = { ... };` (AUTO-7):

- each element must give the same `T`,
- the list must not be empty,
- a declaration of `std::initializer_list` must come first
  *(library referenced)*.
:::

:::{rule} AUTO-8
:kind: reject
:tests: declarations/auto/no-initializer.cpp declarations/auto/own-initializer.cpp declarations/auto/void-initializer.cpp declarations/auto/overloaded-function.cpp declarations/auto/combined-type-specifier.cpp declarations/auto/array-of-auto.cpp

A variable declared with `auto` must:

- have an initializer,
- not occur in its own initializer,
- not have an initializer of type `void`,
- not have the name of an overloaded function as its initializer,
- not combine `auto` with another type specifier,
- not be an array of `auto`.
:::

:::{rule} AUTO-9
:kind: required
:tests: declarations/auto/several-declarators.cpp declarations/auto/later-declarator-uses-earlier.cpp

One declaration can declare several variables with `auto`. Each
{term}`declarator` gives its type separately, and can add `*` or `&`.
AUTO-22 states what the declarators must agree on.

A later declarator can use a name that an earlier declarator in the same
declaration introduced.
:::

:::{rule} AUTO-22
:kind: reject
:tests: declarations/auto/several-declarators-conflict.cpp

If one declaration declares several variables with `auto` (AUTO-9),
`auto` must stand for the same type in all declarators.
:::

:::{rule} AUTO-10
:kind: required
:tests: declarations/auto/decltype-auto-value-category.cpp declarations/auto/decltype-auto-keeps-cv.cpp

`decltype(auto)` gives the type that `decltype` gives the initializer
({rule}`DTY-2`). The {term}`value category` of the initializer controls
the result:

| Initializer | Type |
|---|---|
| A name | Its declared type, including `const` |
| A parenthesized lvalue | An lvalue reference |
| An xvalue | An rvalue reference |
| A prvalue | A type that is not a reference |
:::

:::{rule} AUTO-11
:kind: reject
:tests: declarations/auto/decltype-auto-with-qualifier.cpp declarations/auto/decltype-auto-with-pointer.cpp declarations/auto/decltype-auto-copy-list.cpp declarations/auto/decltype-auto-function.cpp declarations/auto/decltype-auto-braced-multiple.cpp declarations/auto/decltype-auto-braced.cpp

`decltype(auto)` must not:

- be combined with `const`, `volatile`, `*`, or `&`,
- have an initializer of the form `= { ... }`,
- have a function name as its initializer,
- have braces with more than one expression.

Braces without `=` that contain one expression are permitted
({question}`Q-0010`).
:::

:::{rule} AUTO-12
:kind: required
:tests: declarations/auto/in-conditions.cpp declarations/auto/switch-condition-init.cpp declarations/auto/while-condition.cpp

The rules on this page also apply to variables in the conditions of
`if`, `while`, and `switch`, and in the initializer statements of `if`,
`switch`, and `for`.
:::

:::{rule} AUTO-13
:kind: required
:tests: declarations/auto/constrained-satisfied.cpp

A concept name can come before `auto` or `decltype(auto)`
*(concepts referenced)*. AUTO-16 states the constraint on the concept.
:::

:::{rule} AUTO-16
:kind: reject
:tests: declarations/auto/constrained-not-satisfied.cpp

If a concept name comes before `auto` or `decltype(auto)` (AUTO-13), the
deduced type must satisfy the concept.
:::

:::{rule} AUTO-14
:kind: required
:tests: declarations/auto/inline-static-member.cpp declarations/auto/static-const-member.cpp

A static data member can have a {term}`placeholder type` if it has an
initializer in the class and is `inline`, `constexpr`, or `const`. The
type comes from the initializer.
:::

:::{rule} AUTO-17
:kind: reject
:tests: declarations/auto/static-member-no-initializer.cpp

The static data members that AUTO-14 describes are the only ones that
can have a {term}`placeholder type`. Other static data members must not
have a placeholder type.
:::

:::{rule} AUTO-15
:kind: reject
:tests: declarations/auto/decltype-auto-array.cpp

`decltype(auto)` with the name of an array as its initializer gives the
array type itself. An array cannot be initialized from another array, so
the declaration is not valid.
:::

## Examples

```cpp
const int& r = ci;
auto a = r;                     // int
const int* const p = &ci;
auto b = p;                     // const int*
auto n = nullptr;               // decltype(nullptr)

int arr[3];
auto q = arr;                   // int*
auto s = "hi";                  // const char*

auto& c = ci;                   // const int&
auto& d = arr;                  // int (&)[3]
auto& e = 1;                    // rejected
const auto& f = 1;              // const int&

auto&& g = i;                   // int&
auto&& h = 1;                   // int&&

const auto j = i;               // const int
auto* const k = &i;             // int* const
auto* m = 1;                    // rejected
constexpr auto t = 1;           // const int

auto u{1};                      // int
auto v(1);                      // int
auto w{1, 2};                   // rejected
auto x({1});                    // rejected

auto y = {1, 2};                // std::initializer_list<int>
auto z = {1, 2.0};              // rejected

auto aa = 1, *pa = &aa, &ra = aa;   // int, int*, int&
auto bb = 1, cc = 2.0;              // rejected

decltype(auto) da = i;          // int
decltype(auto) db = (i);        // int&
decltype(auto) dc = static_cast<int&&>(i);   // int&&
decltype(auto) dd{(i)};         // int&

template <class T> concept Int = same<T, int>;
Int auto ia = 1;                // int
Int auto ib = 1.0;              // rejected
```

## Evidence

All rules on this page are `required` or `reject`. All seven compilers
in the matrix agree on all 49 tests. The results are in
`tests/RESULTS.md`.

The tests for `auto x = { ... }` declare a small
`std::initializer_list`, because tests include no headers. Without that
declaration, each compiler rejects the `auto` declaration and names the
missing header.

Inputs: clang-sema-deduction, clang-sema-decl, edg-overload, edg-decls
(see `inputs/MANIFEST.md`).
