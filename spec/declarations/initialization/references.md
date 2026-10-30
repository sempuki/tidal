# Reference binding

How a reference is bound by its initializer: which expressions each kind of
reference accepts, when a temporary object is created for it, and how long
that temporary lives. Rules are prefixed `REF`.

Depends on: [declarators](../declarators.md), [storage duration](../storage-linkage.md),
[value category](../../glossary.md#value-category), conversions *(referenced)*,
classes *(referenced)*, [constant evaluation](../../glossary.md#constant-evaluation).

Inputs: clang-sema-init, gcc-cp-typeck2, gcc-cp-init, edg-decl-inits,
clang-diag-sema (see `inputs/MANIFEST.md`).

## Rules

### REF-1 (required)

A non-`const` lvalue reference binds only to an lvalue of the same type,
or of the same type with fewer qualifiers. It binds directly, so the
reference and the lvalue are the same object. A program is rejected if the
initializer is an lvalue of another type or would lose a `const`.

    long l = 1;
    int& r = l;           // rejected
    const int c = 1;
    int& s = c;           // rejected

Tests: declarations/initialization/references/lvalue-ref-binds-lvalue.cpp, declarations/initialization/references/lvalue-ref-other-type.cpp, declarations/initialization/references/lvalue-ref-drops-const.cpp

### REF-2 (required)

A `const` lvalue reference also binds to an rvalue, and to an expression of
another type that converts to its type. In those cases a temporary object
holding the converted value is created, and the reference binds to that
temporary. Later changes to the original don't affect it.

    long l = 1;
    const int& r = l;     // r refers to a temporary int
    l = 2;                // r is still 1

Tests: declarations/initialization/references/const-ref-binds-rvalue.cpp, declarations/initialization/references/const-ref-temporary-other-type.cpp

### REF-3 (required)

An rvalue reference binds to an rvalue, including an xvalue such as
`static_cast<int&&>(x)`, which refers to `x` itself. It also binds to an
lvalue of another type through a temporary, as in REF-2. A program is
rejected if it binds an rvalue reference to an lvalue of the same type.

Tests: declarations/initialization/references/rvalue-ref-binds-rvalue.cpp, declarations/initialization/references/rvalue-ref-rejects-lvalue.cpp, declarations/initialization/references/rvalue-ref-other-type-lvalue.cpp

### REF-4 (required)

When a reference variable is bound to a temporary, or to a member of a
temporary, the temporary lives as long as the reference. It is destroyed
when the reference goes out of scope, in the reverse order of creation
along with the other objects there.

    {
      const Note& a = make('a');
      Note&& b = make('b');
    }   // ~b, then ~a

Tests: declarations/initialization/references/lifetime-extended.cpp, declarations/initialization/references/lifetime-member-extended.cpp

### REF-5 (undefined)

A temporary's lifetime is extended only by binding it directly to the
variable. Binding it to a function parameter, or returning a reference to
it or to a local variable, doesn't extend anything. Using such a reference
after the object is gone has undefined behavior.

    const int& pass(const int& v) { return v; }
    const int& r = pass(1);   // the temporary ends here; r dangles

Tests: declarations/initialization/references/lifetime-not-through-return.cpp, declarations/initialization/references/dangling-local-return.cpp

### REF-6 (required)

A reference may be initialized with `=`, parentheses, or braces holding
one expression. In braces, conversions that narrow are rejected as in
LIST-2. A program is rejected if the initializer holds more than one
expression.

Tests: declarations/initialization/references/braced-reference.cpp, declarations/initialization/references/braced-reference-narrowing.cpp, declarations/initialization/references/reference-multiple-values.cpp

## Evidence

All seven compilers agree on all 15 tests. Results are in
`tests/RESULTS.md`.

- **REF-5.** All compilers reject the dangling reads during constant
  evaluation ("read of object outside its lifetime", "accessing ...
  outside its lifetime", "attempt to access expired storage"). Clang, GCC,
  and EDG also warn about returning a reference to a local variable.

## Divergences

None.
