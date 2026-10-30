# Placeholder types in variable declarations

A variable declaration may use `auto` or `decltype(auto)` in place of a
type. The variable's type is then worked out from its initializer. `auto`
may be combined with `const`, `volatile`, `*`, `&`, and `&&` to shape the
deduced type.

This section covers variables only, including variables declared in
conditions and initializer statements. Placeholders in function return
types, parameters, and template parameters belong to function definitions
and templates *(referenced)*.

Depends on: declarators *(referenced)*, initialization *(referenced)*,
[value category](../glossary.md#value-category), template argument
deduction *(referenced)*, `std::initializer_list` *(library, referenced)*.

Inputs: clang-sema-deduction, clang-sema-decl, gcc-cp-pt, gcc-cp-decl,
edg-overload, edg-decls (see `inputs/MANIFEST.md`).

## Syntax

    placeholder   = [ type-constraint ] ( "auto" | "decltype" "(" "auto" ")" )
    auto-var-decl = { decl-specifier } placeholder { decl-specifier }
                      auto-declarator { "," auto-declarator } ";"
    auto-declarator = declarator auto-init
    auto-init     = "=" expression
                  | "=" "{" [ expression { "," expression } ] "}"
                  | "(" expression ")"
                  | "{" expression "}"

`decl-specifier` covers `const`, `volatile`, `static`, `constexpr`, and the
other specifiers *(referenced)*. `type-constraint` names a concept
*(referenced)*.

## Rules

### AUTO-1 (required)

For a variable declared `auto` with `= expression`, the type is the
initializer's type with any reference removed and then any top-level
`const` or `volatile` removed. Qualifiers below the top level stay, so a
pointer to `const` remains a pointer to `const`. `auto n = nullptr;` has the
type of `nullptr`.

    const int& r = ...;
    auto a = r;           // int
    const int* const p = ...;
    auto b = p;           // const int*

Tests: declarations/auto/drops-references-and-cv.cpp, declarations/auto/keeps-pointee-cv.cpp, declarations/auto/nullptr.cpp

### AUTO-2 (required)

An initializer of array type gives a pointer to the array's first element,
and a function gives a pointer to the function. A string literal gives
`const char*`.

    int arr[3];
    auto p = arr;        // int*
    auto s = "hi";       // const char*

Tests: declarations/auto/decays-arrays-and-functions.cpp

### AUTO-3 (required)

`auto&` makes the variable an lvalue reference to the initializer's type.
The initializer's `const` and `volatile` are kept, and arrays and functions
don't decay. The initializer must be an lvalue unless the declaration adds
`const`, as in `const auto& r = 1;`.

    const int ci = 1;
    auto& a = ci;        // const int&
    auto& b = arr;       // int (&)[3]
    auto& c = 1;         // rejected

Tests: declarations/auto/lvalue-reference.cpp, declarations/auto/lvalue-reference-to-rvalue.cpp, declarations/auto/const-lvalue-reference-to-rvalue.cpp

### AUTO-4 (required)

`auto&&` gives an lvalue reference when the initializer is an lvalue and an
rvalue reference otherwise. The initializer's `const` and `volatile` are
kept.

    auto&& a = i;        // int&
    auto&& b = 1;        // int&&

Tests: declarations/auto/forwarding-reference.cpp

### AUTO-5 (required)

`const`, `volatile`, `*`, and `* const` written with `auto` apply to the
deduced type. `auto*` requires an initializer of pointer type and is
rejected otherwise. `constexpr` makes the variable `const`.

    const auto a = i;    // const int
    auto* const p = &i;  // int* const
    auto* q = 1;         // rejected
    constexpr auto n = 1;  // const int

Tests: declarations/auto/added-qualifiers.cpp, declarations/auto/pointer-from-non-pointer.cpp, declarations/auto/constexpr-adds-const.cpp

### AUTO-6 (required)

An initializer in parentheses or in braces without `=` must hold exactly one
expression. The type is deduced from that expression as if it were written
after `=`. More than one expression, an empty pair of braces, or a braced
list inside parentheses is rejected.

    auto a{1};           // int
    auto b(1);           // int
    auto c{1, 2};        // rejected
    auto d({1});         // rejected

Tests: declarations/auto/braced-direct-single.cpp, declarations/auto/braced-direct-multiple.cpp, declarations/auto/braced-direct-empty.cpp, declarations/auto/paren-single.cpp, declarations/auto/paren-multiple.cpp, declarations/auto/paren-braced.cpp

### AUTO-7 (required)

`auto x = { ... };` gives `x` the type `std::initializer_list<T>`, where `T`
is the type deduced from every element. All elements must deduce the same
`T`, and the list must not be empty. `std::initializer_list` comes from the
library *(referenced)* and must be declared before this form is used.

    auto a = {1, 2};     // std::initializer_list<int>
    auto b = {1};        // std::initializer_list<int>
    auto c = {1, 2.0};   // rejected

Tests: declarations/auto/copy-list-deduces-initializer-list.cpp, declarations/auto/copy-list-mixed-types.cpp, declarations/auto/copy-list-without-library.cpp, declarations/auto/copy-list-empty.cpp

### AUTO-8 (reject)

A program is rejected if a variable declared with `auto`

- has no initializer,
- is used in its own initializer,
- is initialized from an expression of type `void`,
- is initialized from the name of an overloaded function,
- combines `auto` with another type specifier, as in `auto int`, or
- is declared as an array of `auto`.

Tests: declarations/auto/no-initializer.cpp, declarations/auto/own-initializer.cpp, declarations/auto/void-initializer.cpp, declarations/auto/overloaded-function.cpp, declarations/auto/combined-type-specifier.cpp, declarations/auto/array-of-auto.cpp

### AUTO-9 (required)

One declaration may declare several variables with `auto`. Each declarator
is deduced on its own, and `auto` must stand for the same type in all of
them. Declarators may add `*` or `&`.

    auto a = 1, *p = &a, &r = a;   // int, int*, int&
    auto x = 1, y = 2.0;           // rejected

Tests: declarations/auto/several-declarators.cpp, declarations/auto/several-declarators-conflict.cpp

### AUTO-10 (required)

`decltype(auto)` gives the variable the type that `decltype` gives its
initializer *(referenced)*. A name gives its declared type, including
`const`. A parenthesized lvalue gives an lvalue reference. An xvalue gives
an rvalue reference. A prvalue gives a plain type.

    decltype(auto) a = i;      // int
    decltype(auto) b = (i);    // int&
    decltype(auto) c = static_cast<int&&>(i);  // int&&

Tests: declarations/auto/decltype-auto-value-category.cpp, declarations/auto/decltype-auto-keeps-cv.cpp

### AUTO-11 (reject)

A program is rejected if `decltype(auto)`

- is combined with `const`, `volatile`, `*`, or `&`,
- is initialized with `= { ... }`,
- is initialized from a function name, which would give a function type, or
- has a braced initializer with more than one expression.

A braced initializer without `=` holding one expression is accepted, and
deduces as if the braces were absent (question Q-0010).

Tests: declarations/auto/decltype-auto-with-qualifier.cpp, declarations/auto/decltype-auto-with-pointer.cpp, declarations/auto/decltype-auto-copy-list.cpp, declarations/auto/decltype-auto-function.cpp, declarations/auto/decltype-auto-braced-multiple.cpp, declarations/auto/decltype-auto-braced.cpp

### AUTO-12 (required)

The same rules apply to variables declared in `if` conditions and
initializer statements and in `for` initializer statements.

Tests: declarations/auto/in-conditions.cpp

### AUTO-13 (required)

A concept name may come before `auto` or `decltype(auto)`. The type is
deduced as usual and then checked against the concept *(referenced)*. The
program is rejected if the deduced type does not satisfy it.

    template <class T> concept Int = same<T, int>;
    Int auto a = 1;      // int
    Int auto b = 1.0;    // rejected

Tests: declarations/auto/constrained-satisfied.cpp, declarations/auto/constrained-not-satisfied.cpp

## Evidence

Every rule in this section is `required` or `reject`. All seven compilers
in the matrix agree on all 40 tests. Results are in `tests/RESULTS.md`.

Tests that use `auto x = { ... }` declare a minimal `std::initializer_list`
themselves, because tests include no headers. Without it, each compiler
rejects the declaration and names the missing header.

## Divergences

None.
