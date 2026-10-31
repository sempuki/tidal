# Reference binding

The initializer of a reference binds the reference to an object or a
function. This page covers which expressions each kind of reference
accepts, when a temporary object is created, and how long it lives.

## Principles

:::{principle} REF-P1
:rules: REF-1 REF-8 REF-2 REF-9 REF-3 REF-10 REF-6 REF-11

A reference is a name for an existing object or function
({principle}`OBJ-P3`), and the binding gives it that object. What a
reference accepts follows from what it may do with its referent. A
reference that can change the object binds only to an lvalue of that
type or a derived type. A `const` lvalue reference changes nothing, so
it also binds to rvalues and to values of other types, through a
temporary object when there is no object to bind to. An rvalue
reference exists to take from an object that is going away, so it binds
to rvalues and never to an lvalue of its own type. In every form the
initializer is one expression.
:::

:::{principle} REF-P2
:rules: REF-4 REF-7 REF-5

A temporary made for a reference holds the converted value and lives as
long as the reference variable, because a reference to nothing would be
useless. This extension belongs to the variable's binding, including
bindings through a member, a subscript, or a plain cast. It does not
reach a function parameter or a returned reference. A reference used
after its object has ended, or bound through a null pointer, has no
defined meaning.
:::

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
:tests: declarations/initialization/references/lvalue-ref-binds-lvalue.cpp declarations/initialization/references/base-ref-to-derived.cpp declarations/initialization/references/unknown-bound-ref.cpp

A non-`const` lvalue reference binds directly to an lvalue. The reference
and the lvalue are then the same object. The lvalue can have the same
type, the same type with fewer {term}`cv-qualifiers <cv-qualifier>`, or
a class derived from the type through an accessible base. A reference
to an array of unknown bound binds to an array of known bound. REF-8
lists what is not valid.
:::

:::{rule} REF-8
:kind: reject
:tests: declarations/initialization/references/lvalue-ref-other-type.cpp declarations/initialization/references/lvalue-ref-drops-const.cpp declarations/initialization/references/bit-field-nonconst-ref.cpp

The lvalue that a non-`const` lvalue reference binds to must have one of
the types that REF-1 lists. A non-`const` lvalue reference must not bind
to a bit-field.
:::

:::{rule} REF-2
:kind: required
:tests: declarations/initialization/references/const-ref-binds-rvalue.cpp declarations/initialization/references/bit-field-const-ref.cpp declarations/initialization/references/const-ref-xvalue-direct.cpp

A `const` lvalue reference that is not `volatile` can also bind to an
rvalue. It can also bind to an expression of a different type that
converts to its type, and to a bit-field. If the rvalue is an xvalue of the
same type, or a class rvalue, the reference binds to that object
directly. In the other cases, the reference binds to a temporary object
that holds a copy of the value. REF-9 lists what is not valid.
:::

:::{rule} REF-9
:kind: reject
:tests: declarations/initialization/references/const-volatile-ref-rvalue.cpp

An lvalue reference that is `volatile`, or that is not `const`, must not
bind to an rvalue.
:::

:::{rule} REF-3
:kind: required
:tests: declarations/initialization/references/rvalue-ref-binds-rvalue.cpp declarations/initialization/references/rvalue-ref-to-function.cpp

An rvalue reference to an object type binds to an rvalue, including an
xvalue. It can bind to an lvalue of a different type through a temporary
object, as REF-2 gives. An rvalue reference to a function type binds to
a function. REF-10 lists what is not valid.
:::

:::{rule} REF-10
:kind: reject
:tests: declarations/initialization/references/rvalue-ref-rejects-lvalue.cpp declarations/initialization/references/rvalue-ref-base-from-derived-lvalue.cpp declarations/initialization/references/rvalue-ref-drops-const.cpp

An rvalue reference to an object type must not bind to an lvalue of the
same type, or of a class derived from that type. It must not drop a
cv-qualifier of the object.
:::

:::{rule} REF-6
:kind: required
:tests: declarations/initialization/references/braced-reference.cpp

A reference can be initialized with `=`, parentheses, or braces. REF-11
lists what is not valid.
:::

:::{rule} REF-11
:kind: reject
:tests: declarations/initialization/references/braced-reference-narrowing.cpp declarations/initialization/references/reference-multiple-values.cpp

The initializer of a reference must hold exactly one expression. In
braces, a narrowing conversion must not occur (LIST-2).
:::

## Runtime Behavior

:::{rule} REF-4
:kind: required
:tests: declarations/initialization/references/lifetime-extended.cpp declarations/initialization/references/lifetime-member-extended.cpp declarations/initialization/references/aggregate-member-extends.cpp declarations/initialization/references/lifetime-through-parens-and-conditional.cpp declarations/initialization/references/static-reference-to-temporary.cpp declarations/initialization/references/extend-through-static-cast.cpp declarations/initialization/references/extend-through-array-member-subscript.cpp

If a reference variable binds to a temporary object, or to a member of
one, the temporary lives as long as the reference. This also applies to a
reference member of an aggregate initialized with braces, and through
parentheses, the selected operand of `?:`, the right operand of a
comma, a cast without a user-defined conversion such as `static_cast`,
and a subscript of an array member of the temporary ({question}`Q-0109`
for the subscript). The temporary is destroyed when the reference goes out of scope,
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
:tests: declarations/initialization/references/lifetime-not-through-return.cpp declarations/initialization/references/dangling-local-return.cpp declarations/initialization/references/aggregate-member-paren-not-extended.cpp declarations/initialization/references/null-dereference-bind-constant.cpp

Only the bindings in REF-4 extend the life of a temporary. A binding to a
function parameter, a returned reference, or a reference member of an
aggregate initialized with parentheses does not extend it
({question}`Q-0040`). Use of a reference after the life of its object ends
is undefined. Binding a reference to `*p` where `p` is a null pointer is
undefined, and constant evaluation rejects it ({question}`Q-0110`).
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

All seven compilers agree on 28 of 31 tests. The results are in
`tests/RESULTS.md`.

- **REF-5.** All compilers reject the dangling reads during
  {term}`constant evaluation` ("read of object outside its lifetime",
  "accessing ... outside its lifetime", "attempt to access expired
  storage"). Clang, GCC, and EDG also warn when a function returns a
  reference to a local variable.
- **REF-4.** EDG 7.0 rejects binding an rvalue reference to an element of
  an array member of a temporary. Clang and GCC accept it and extend the
  temporary ({question}`Q-0109`).
- **REF-5.** GCC and Clang 22 reject a reference bound through a null
  pointer during constant evaluation. EDG 7.0 and Clang 18 accept it
  ({question}`Q-0110`).

Inputs: clang-sema-init, edg-decl-inits, clang-diag-sema (see
`inputs/MANIFEST.md`).
