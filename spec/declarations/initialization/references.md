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

## Constraints

:::{rule} REF-1
:kind: required
:tests: declarations/initialization/references/lvalue-ref-binds-lvalue.cpp declarations/initialization/references/lvalue-ref-other-type.cpp declarations/initialization/references/lvalue-ref-drops-const.cpp declarations/initialization/references/bit-field-nonconst-ref.cpp

A non-`const` lvalue reference binds directly to an lvalue. The reference
and the lvalue are then the same object. The lvalue must have the same
type, or the same type with fewer {term}`cv-qualifiers <cv-qualifier>`. A
non-`const` lvalue reference must not bind to a bit-field.
:::

:::{rule} REF-2
:kind: required
:tests: declarations/initialization/references/const-ref-binds-rvalue.cpp declarations/initialization/references/const-volatile-ref-rvalue.cpp declarations/initialization/references/bit-field-const-ref.cpp

A `const` lvalue reference that is not `volatile` can also bind to an
rvalue. It can also bind to an expression of a different type that
converts to its type, and to a bit-field. In these cases, the reference
binds to a temporary object that holds a copy of the value.
:::

:::{rule} REF-3
:kind: required
:tests: declarations/initialization/references/rvalue-ref-binds-rvalue.cpp declarations/initialization/references/rvalue-ref-rejects-lvalue.cpp declarations/initialization/references/rvalue-ref-to-function.cpp declarations/initialization/references/rvalue-ref-base-from-derived-lvalue.cpp

An rvalue reference to an object type binds to an rvalue, including an
xvalue. It can bind to an lvalue of a different type through a temporary
object, as REF-2 gives. It must not bind to an lvalue of the same type, or
of a class derived from that type. An rvalue reference to a function type
binds to a function.
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
:tests: declarations/initialization/references/lifetime-extended.cpp declarations/initialization/references/lifetime-member-extended.cpp declarations/initialization/references/aggregate-member-extends.cpp declarations/initialization/references/lifetime-through-parens-and-conditional.cpp declarations/initialization/references/static-reference-to-temporary.cpp

If a reference variable binds to a temporary object, or to a member of
one, the temporary lives as long as the reference. This also applies to a
reference member of an aggregate initialized with braces, and through
parentheses, the selected operand of `?:`, and the right operand of a
comma. The temporary is destroyed when the reference goes out of scope,
in order with the other objects of that scope. A reference with static
storage keeps its temporary until the program ends.
:::

:::{rule} REF-7
:kind: required
:tests: declarations/initialization/references/const-ref-temporary-other-type.cpp declarations/initialization/references/rvalue-ref-other-type-lvalue.cpp

A temporary object that a reference binds to holds the converted value.
A later change to the source expression does not change the temporary.
:::

## Undefined Behavior

:::{rule} REF-5
:kind: undefined
:tests: declarations/initialization/references/lifetime-not-through-return.cpp declarations/initialization/references/dangling-local-return.cpp declarations/initialization/references/aggregate-member-paren-not-extended.cpp

Only the bindings in REF-4 extend the life of a temporary. A binding to a
function parameter, a returned reference, or a reference member of an
aggregate initialized with parentheses does not extend it
({question}`Q-0040`). Use of a reference after the life of its object ends
is undefined.
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

Inputs: clang-sema-init, edg-decl-inits, clang-diag-sema (see
`inputs/MANIFEST.md`).
