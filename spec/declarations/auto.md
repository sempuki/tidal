# Placeholder types in variable declarations

A variable declaration can use a {term}`placeholder type`, `auto` or
`decltype(auto)`, in the position of a type. The type of the variable then
comes from its initializer.

This page covers variables, including variables in conditions and
initializer statements. Placeholders in return types, parameters, and
template parameters are on other pages *(referenced)*.

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

`decl_specifier` includes `const`, `volatile`, `static`, `constexpr`, and
the other specifiers *(referenced)*. `type_constraint` names a concept
*(referenced)*.

## Legality Rules

:::{rule} AUTO-1
:kind: required
:tests: declarations/auto/drops-references-and-cv.cpp declarations/auto/keeps-pointee-cv.cpp declarations/auto/nullptr.cpp

For `auto` with `= expression`, the type is the type of the initializer
without its reference. Then the top-level `const` and `volatile` are
removed. Qualifiers below the top level stay.
:::

:::{rule} AUTO-2
:kind: required
:tests: declarations/auto/decays-arrays-and-functions.cpp

An initializer of array type gives a pointer to its first element. A
function gives a pointer to the function. A string literal gives
`const char*`.
:::

:::{rule} AUTO-3
:kind: required
:tests: declarations/auto/lvalue-reference.cpp declarations/auto/lvalue-reference-to-rvalue.cpp declarations/auto/const-lvalue-reference-to-rvalue.cpp

`auto&` gives an lvalue reference to the type of the initializer. The
type keeps its `const` and `volatile`, and arrays and functions do not
decay. The initializer must be an lvalue, unless the declaration adds
`const`.
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
:tests: declarations/auto/added-qualifiers.cpp declarations/auto/pointer-from-non-pointer.cpp declarations/auto/constexpr-adds-const.cpp

`const`, `volatile`, and `*` written with `auto` apply to the deduced
type. With `auto*`, the initializer must have pointer type. `constexpr`
makes the variable `const`.
:::

:::{rule} AUTO-6
:kind: required
:tests: declarations/auto/braced-direct-single.cpp declarations/auto/braced-direct-multiple.cpp declarations/auto/braced-direct-empty.cpp declarations/auto/paren-single.cpp declarations/auto/paren-multiple.cpp declarations/auto/paren-braced.cpp

An initializer in parentheses, or in braces without `=`, must contain
exactly one expression. That expression gives the type, as it does after
`=`. A braced list in parentheses is not permitted.
:::

:::{rule} AUTO-7
:kind: required
:tests: declarations/auto/copy-list-deduces-initializer-list.cpp declarations/auto/copy-list-mixed-types.cpp declarations/auto/copy-list-without-library.cpp declarations/auto/copy-list-empty.cpp

`auto x = { ... };` gives `x` the type `std::initializer_list<T>`. Each
element must give the same `T`, and the list must not be empty. A
declaration of `std::initializer_list` must come first *(library
referenced)*.
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
:tests: declarations/auto/several-declarators.cpp declarations/auto/several-declarators-conflict.cpp declarations/auto/later-declarator-uses-earlier.cpp

One declaration can declare several variables with `auto`. Each
{term}`declarator` gives its type separately, and can add `*` or `&`.
`auto` must stand for the same type in all declarators.

A later declarator can use a name that an earlier declarator in the same
declaration introduced.
:::

:::{rule} AUTO-10
:kind: required
:tests: declarations/auto/decltype-auto-value-category.cpp declarations/auto/decltype-auto-keeps-cv.cpp

`decltype(auto)` gives the type that `decltype` gives the initializer
*(referenced)*. The {term}`value category` of the initializer controls the
result:

- a name gives its declared type, including `const`,
- a parenthesized lvalue gives an lvalue reference,
- an xvalue gives an rvalue reference,
- a prvalue gives a type that is not a reference.
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
:tests: declarations/auto/in-conditions.cpp

These rules also apply to variables in `if` conditions, in `if`
initializer statements, and in `for` initializer statements.
:::

:::{rule} AUTO-13
:kind: required
:tests: declarations/auto/constrained-satisfied.cpp declarations/auto/constrained-not-satisfied.cpp

A concept name can come before `auto` or `decltype(auto)` *(referenced)*.
The deduced type must satisfy the concept.
:::

:::{rule} AUTO-14
:kind: required
:tests: declarations/auto/inline-static-member.cpp declarations/auto/static-member-no-initializer.cpp

A static data member can have a {term}`placeholder type` if it is `inline` or
`constexpr` and has an initializer in the class. The type comes from the
initializer.
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

All rules on this page are `required` or `reject`. All seven compilers in
the matrix agree on all 40 tests. The results are in `tests/RESULTS.md`.

The tests for `auto x = { ... }` declare a small `std::initializer_list`,
because tests include no headers. Without it, each compiler rejects the
declaration and names the missing header.

Inputs: clang-sema-deduction, clang-sema-decl, edg-overload, edg-decls
(see `inputs/MANIFEST.md`).
