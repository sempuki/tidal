# Reference binding

The initializer of a reference binds the reference to an object or a
function. This page covers which expressions each kind of reference
accepts, when a temporary object is created, and how long it lives.

## Syntax

```{eval-rst}
.. productionlist:: init_references
   ref_init: "=" `expression`
           : | "(" `expression` ")"
           : | "{" `expression` "}"
```

## Legality Rules

:::{rule} REF-1
:kind: required
:tests: declarations/initialization/references/lvalue-ref-binds-lvalue.cpp declarations/initialization/references/lvalue-ref-other-type.cpp declarations/initialization/references/lvalue-ref-drops-const.cpp

A non-`const` lvalue reference binds directly to an lvalue. The reference
and the lvalue are then the same object. The lvalue must have the same
type, or the same type with fewer {term}`cv-qualifiers <cv-qualifier>`.
:::

:::{rule} REF-2
:kind: required
:tests: declarations/initialization/references/const-ref-binds-rvalue.cpp declarations/initialization/references/const-ref-temporary-other-type.cpp

A `const` lvalue reference can also bind to an rvalue. It can also bind to
an expression of a different type that converts to its type. In these
cases, the reference binds to a temporary object that holds the
converted value.
:::

:::{rule} REF-3
:kind: required
:tests: declarations/initialization/references/rvalue-ref-binds-rvalue.cpp declarations/initialization/references/rvalue-ref-rejects-lvalue.cpp declarations/initialization/references/rvalue-ref-other-type-lvalue.cpp

An rvalue reference binds to an rvalue, including an xvalue. It can bind
to an lvalue of a different type through a temporary object, as REF-2
gives. It must not bind to an lvalue of the same type.
:::

:::{rule} REF-6
:kind: required
:tests: declarations/initialization/references/braced-reference.cpp declarations/initialization/references/braced-reference-narrowing.cpp declarations/initialization/references/reference-multiple-values.cpp

A reference can be initialized with `=`, parentheses, or braces. The
initializer must hold exactly one expression. In braces, a narrowing
conversion must not occur (LIST-2).
:::

## Dynamic Semantics

:::{rule} REF-4
:kind: required
:tests: declarations/initialization/references/lifetime-extended.cpp declarations/initialization/references/lifetime-member-extended.cpp

If a reference variable binds to a temporary object, or to a member of
one, the temporary lives as long as the reference. The temporary is
destroyed when the reference goes out of scope. It is destroyed in order
with the other objects of that scope.
:::

## Undefined Behavior

:::{rule} REF-5
:kind: undefined
:tests: declarations/initialization/references/lifetime-not-through-return.cpp declarations/initialization/references/dangling-local-return.cpp

Only a direct binding to a variable extends the life of a temporary. A
binding to a function parameter or a returned reference does not extend
it. Use of a reference after the life of its object ends is undefined.
:::

## Examples

```cpp
long l = 1;
int& r = l;                  // rejected: different type
const int c = 1;
int& s = c;                  // rejected: loses const

const int& t = l;            // t refers to a temporary int
l = 2;                       // t is still 1

int x = 0;
int&& u = static_cast<int&&>(x);   // u refers to x
int&& v = x;                 // rejected: lvalue of the same type

{
  const Note& a = make('a');
  Note&& b = make('b');
}                            // ~b, then ~a

const int& pass(const int& v) { return v; }
const int& w = pass(1);      // the temporary ends here; w dangles
```

## Evidence

All seven compilers agree on all 15 tests. The results are in
`tests/RESULTS.md`.

- **REF-5.** All compilers reject the dangling reads during
  {term}`constant evaluation` ("read of object outside its lifetime",
  "accessing ... outside its lifetime", "attempt to access expired
  storage"). Clang, GCC, and EDG also warn when a function returns a
  reference to a local variable.

Inputs: clang-sema-init, gcc-cp-typeck2, gcc-cp-init, edg-decl-inits,
clang-diag-sema (see `inputs/MANIFEST.md`).
