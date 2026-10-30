# Structured bindings

A structured binding declaration introduces several names at once, each
referring to one element of an object: an array element, a data member of
a class, or an element obtained through the tuple protocol.

    auto [x, y] = point;

The declaration creates one hidden variable initialized from the
initializer, then makes each name refer to one element of it. Rules are
prefixed `SB`.

Depends on: [placeholder types](auto.md), [initialization](initialization/forms.md),
[reference binding](initialization/references.md), classes *(referenced)*,
templates *(referenced)*, `std::tuple_size` and `std::tuple_element`
*(library, referenced)*.

Inputs: clang-sema-declcxx, gcc-cp-decl, edg-decls, clang-diag-sema (see
`inputs/MANIFEST.md`).

## Syntax

    sb-decl = [ "static" | "thread_local" ] { cv } "auto" [ "&" | "&&" ]
                "[" name { "," name } "]" sb-init ";"
    sb-init = "=" expression | "(" expression ")" | "{" expression "}"

## Rules

### SB-1 (required)

A structured binding declaration creates one hidden variable, declared with
the written `auto`, qualifiers, and reference, and initialized from the
initializer. Each name then refers to one element of that hidden variable.
With plain `auto` the hidden variable is a copy, so changing a binding
doesn't change the original. With `auto&` the hidden variable is a
reference, so the bindings refer to the original's elements.

    int a[2] = {1, 2};
    auto [x, y] = a;     // x refers to a copy of a[0]
    auto& [p, q] = a;    // p refers to a[0]

Tests: declarations/structured-bindings/array-copy.cpp, declarations/structured-bindings/array-reference.cpp

### SB-2 (reject)

A program is rejected if the number of names differs from the number of
elements: the array's bound, the number of data members, or the tuple size
(SB-5).

Tests: declarations/structured-bindings/count-mismatch-array.cpp, declarations/structured-bindings/count-mismatch-class.cpp

### SB-3 (required)

When the hidden variable has class type and SB-5 does not apply, the names
refer to its non-static data members in declaration order. All of those
members must belong to the same class, which may be a base class. A program
is rejected if a member is not public, if the class has an anonymous union
member, if members are split between a class and its base, or if the type is
a union or a scalar.

Tests: declarations/structured-bindings/aggregate-members.cpp, declarations/structured-bindings/members-in-base.cpp, declarations/structured-bindings/private-member.cpp, declarations/structured-bindings/anonymous-union-member.cpp, declarations/structured-bindings/members-split-across-bases.cpp, declarations/structured-bindings/union-rejected.cpp, declarations/structured-bindings/scalar-rejected.cpp

### SB-4 (required)

For an array or a data member, `decltype` of a binding gives the element's
declared type, with the hidden variable's `const` and `volatile` added. It
is never a reference type, even when the declaration uses `auto&`.

    struct S { int a; const double b; };
    auto& [r, t] = s;      // decltype(r) is int, decltype(t) is const double
    const auto [u, v] = s; // decltype(u) is const int

Tests: declarations/structured-bindings/binding-types.cpp

### SB-5 (required)

When `std::tuple_size<E>` is a complete type for the hidden variable's type
`E`, its `value` gives the number of elements. Element `i` is obtained by
calling `get<i>()` *(lookup referenced)*, and its type is
`std::tuple_element<i, E>::type`. The library declares these templates
*(library referenced)*. A program is rejected if `std::tuple_element` is
missing for an element.

Tests: declarations/structured-bindings/tuple-protocol.cpp, declarations/structured-bindings/tuple-count-mismatch.cpp, declarations/structured-bindings/tuple-element-missing.cpp

### SB-6 (required)

When the hidden variable is a reference bound to a temporary, the
temporary lives as long as the bindings, as in REF-4.

Tests: declarations/structured-bindings/lifetime-extended.cpp

### SB-7 (required)

A structured binding declaration uses `auto`, optionally with `const`,
`volatile`, `&` or `&&`, and optionally with `static` or
`thread_local`. It needs an initializer after `=`, in parentheses, or in
braces. A program is rejected if the declaration

- uses an explicit type, `auto*`, or `decltype(auto)`,
- is `constexpr` (allowed in a later language version, Q-0007),
- has no initializer or no names,
- is a class member or a function parameter,
- has other declarators,
- puts the bracket list in parentheses,
- uses a constrained `auto` (question Q-0016), or
- is a template.

Tests: declarations/structured-bindings/initializer-equals.cpp, declarations/structured-bindings/initializer-parens.cpp, declarations/structured-bindings/initializer-braces.cpp, declarations/structured-bindings/static-and-thread-local.cpp, declarations/structured-bindings/explicit-type.cpp, declarations/structured-bindings/pointer-declarator.cpp, declarations/structured-bindings/decltype-auto.cpp, declarations/structured-bindings/constexpr-binding.cpp, declarations/structured-bindings/no-initializer.cpp, declarations/structured-bindings/empty-bindings.cpp, declarations/structured-bindings/as-class-member.cpp, declarations/structured-bindings/as-parameter.cpp, declarations/structured-bindings/with-other-declarator.cpp, declarations/structured-bindings/parenthesized.cpp, declarations/structured-bindings/constrained.cpp, declarations/structured-bindings/template.cpp

### SB-8 (reject)

A program is rejected if two names in one declaration are the same
(including two `_`, which a later language version allows), if a name is
already declared in the same scope, or if a name is used in the
declaration's own initializer.

Tests: declarations/structured-bindings/duplicate-names.cpp, declarations/structured-bindings/underscore-twice.cpp, declarations/structured-bindings/names-in-scope.cpp, declarations/structured-bindings/own-initializer.cpp

## Evidence

Results are in `tests/RESULTS.md`. All seven compilers agree on 34 of 36
tests.

- **SB-7.** Clang and GCC accept `constexpr` structured bindings and
  repeated `_` as newer-standard extensions, and reject them only under
  `-pedantic-errors`. Clang reports an empty bracket list as a pedantic
  diagnostic.

## Divergences

### SB-D1: braced array initializer

Clang 18.1.3 rejects `auto [r, s]{a};` when `a` is an array. Clang 22,
GCC, and EDG accept it. **Provisional answer:** valid. See Q-0015.

### SB-D2: constrained auto

EDG 7.0 accepts `Any auto [x, y] = a;`. Clang and GCC reject it.
**Provisional answer:** rejected. See Q-0016.
