# Class definitions and members

A class definition introduces a class type and declares its members. The
members are:

- data members,
- member functions,
- static members,
- nested types,
- type aliases.

## Principles

:::{principle} CLS-P1
:rules: CLS-1 CLS-2 CLS-8 CLS-9 CLS-12 CLS-16 CLS-22 CLS-28

A class definition is a scope and a type under construction. Inside its
body the class is incomplete, so a data member needs a type that is
already complete, and a class cannot hold an object of its own type. The
parts of the body that the implementation can read after the closing
brace see the class as complete. These parts include:

- member function bodies,
- default arguments,
- default member initializers,
- `noexcept` operands.

A static data member can have the class's own type for the same reason:
its definition comes after the class.

The class body is one scope, so each member name has one meaning in it
({principle}`NAM-P1`), and code outside the body names a nested type
through the class. A class defined inside a function is a scope that can
own nothing that would need a definition elsewhere.
:::

:::{principle} CLS-P2
:rules: CLS-3 CLS-4 CLS-7 CLS-20 CLS-21 CLS-23 CLS-24 CLS-27

A member belongs to each object of the class or to the class as a whole.
A static data member is one object for the whole program, so it needs
one definition, and a static member function has no object to act on.
A member function defined in the class is `inline`, because every unit
that includes the class sees its body. A `const` static member of
integer type with an initializer in the class is a constant the class
can use directly, so the initializer must be a constant expression, and
the member still needs a definition if the program uses it as an object.
:::

:::{principle} CLS-P3
:rules: CLS-5 CLS-6 CLS-13 CLS-14 CLS-15 CLS-17 CLS-25 CLS-26

A non-static member function acts on an object. Its `this` has the type
of that object with the function's own cv-qualifiers, and its
ref-qualifier selects the function by the value category of the object.
A member function can instead write that object out as its first
parameter. The object is then an ordinary argument, so the function:

- has no `this`,
- cannot be static or virtual or qualified,
- has an ordinary function pointer as its address.

The call and subscript operators can be static for the same reason:
they need no object at all.
:::

:::{principle} CLS-P4
:rules: CLS-10 CLS-11 CLS-19

The members of an object lie in it in the order of their declarations,
and each object has a size, even an empty one, so that two objects never
share an address ({principle}`OBJ-P1`). A class that keeps its members in
one access, with no virtual parts, has a layout a program can reason
about: its address is the address of its first member.
:::

## Syntax

```{eval-rst}
.. productionlist:: cls_members
   class_def: ("class" | "struct") `name` [`base_clause`] "{" {`member`} "}" ";"
   member: `member_declaration` | `function_definition` | `access_label`
   member_function_suffix: [`cv`] ["&" | "&&"]
```

## Constraints

:::{rule} CLS-1
:kind: reject
:tests: classes/members/member-of-own-type.cpp classes/members/pointer-to-own-type.cpp classes/members/incomplete-member.cpp

A non-static data member must have a complete type. A class is not
complete inside its own definition, so it cannot contain a member of its
own type. A pointer to its own type is valid.
:::

:::{rule} CLS-2
:kind: required
:tests: classes/members/complete-in-member-bodies.cpp classes/members/complete-in-default-member-init.cpp classes/members/complete-in-default-argument.cpp classes/members/noexcept-complete-class.cpp

The class is complete inside:

- member function bodies,
- default arguments,
- default member initializers,
- the operands of `noexcept` specifiers.

These can use members that are declared later. At other places inside
the definition, the class is not complete.
:::

:::{rule} CLS-22
:kind: reject
:tests: classes/members/sizeof-in-class-incomplete.cpp

At the places inside its definition where CLS-2 says a class is not
complete, a use that needs a complete class type is not valid.
:::

:::{rule} CLS-3
:kind: required
:tests: classes/members/static-data-member.cpp classes/members/static-member-not-defined.cpp classes/members/inline-static-member.cpp classes/members/static-const-integral.cpp classes/members/static-const-enum.cpp

A static data member is one object for the program. These rules apply:

- A static data member that is not `inline` needs one definition outside
  the class (LNK-6, LNK-7).
- An `inline` or `constexpr` static data member is defined in the class.
- A `const` static data member of integer or enumeration type can have an
  initializer in the class.
:::

:::{rule} CLS-23
:kind: reject
:tests: classes/members/static-non-const-in-class-init.cpp

CLS-3 lists the static data members that can have an initializer in the
class. Other static data members must not have an initializer in the
class.
:::

:::{rule} CLS-4
:kind: required
:tests: classes/members/member-function-inline.cpp classes/members/define-outside-class.cpp

A member function that is defined in its class is `inline`. A member
function can be defined outside the class with a qualified name.
:::

:::{rule} CLS-24
:kind: reject
:tests: classes/members/define-undeclared-member.cpp classes/members/bound-member-function.cpp

The class must declare a member function before a definition outside the
class. A program must call a member function through an object, or take
its address with `&S::f`.
:::

:::{rule} CLS-5
:kind: required
:tests: classes/members/this-type.cpp classes/members/mutable-member.cpp

In a member function of `S`, `this` has type `S*` with the member
function's {term}`cv-qualifiers <cv-qualifier>`. A `const` member
function can change a `mutable` data member.
:::

:::{rule} CLS-25
:kind: reject
:tests: classes/members/const-member-modifies.cpp classes/members/non-const-on-const-object.cpp classes/members/volatile-object-nonvolatile-call.cpp

A `const` member function must not change a data member, unless the
member is `mutable`. A program must not call a member function that is
not `const` on a `const` object, or one that is not `volatile` on a
`volatile` object.
:::

:::{rule} CLS-6
:kind: required
:tests: classes/members/ref-qualified.cpp

A member function can have the ref-qualifier `&` or `&&`. A call on an
lvalue selects `&`, and a call on an rvalue selects `&&`.
:::

:::{rule} CLS-26
:kind: reject
:tests: classes/members/ref-qualified-mixed.cpp

The overloads of a member function that CLS-6 describes must all have
ref-qualifiers, or none, if they have the same parameters
({question}`Q-0027`).
:::

:::{rule} CLS-7
:kind: required
:tests: classes/members/static-member-function.cpp

A static member function has no `this`. A program can call it through
the class name or through an object.
:::

:::{rule} CLS-27
:kind: reject
:tests: classes/members/static-uses-this.cpp classes/members/static-member-function-const.cpp classes/members/static-ref-qualified.cpp classes/members/static-virtual.cpp

A static member function must not:

- use `this`,
- have a cv-qualifier or a ref-qualifier,
- be virtual.
:::

:::{rule} CLS-8
:kind: required
:tests: classes/members/nested-types.cpp

A class can declare these members:

- nested classes,
- enumerations,
- type aliases.

Code outside the class names them as `S::name`.
:::

:::{rule} CLS-9
:kind: reject
:tests: classes/members/duplicate-member.cpp classes/members/member-named-as-class.cpp classes/members/member-named-as-class-with-constructor.cpp classes/members/auto-data-member.cpp classes/members/anonymous-struct.cpp classes/members/member-function-initialized.cpp classes/members/anonymous-union.cpp classes/members/static-member-named-as-class.cpp classes/members/member-function-declared-twice.cpp classes/members/mutable-const.cpp classes/members/mutable-reference.cpp classes/members/mutable-static.cpp classes/members/paren-default-member-init.cpp classes/members/data-and-function-same-name.cpp classes/members/class-name-nested-class.cpp classes/members/class-name-alias.cpp classes/members/class-name-enumerator.cpp classes/members/class-name-function.cpp classes/members/class-name-anon-union-member.cpp classes/members/static-and-nonstatic-same-params.cpp classes/members/thread-local-nonstatic.cpp classes/members/extern-member.cpp classes/members/register-member.cpp classes/members/unnamed-class-static-member.cpp classes/members/nested-in-unnamed-static-member.cpp

A class must not declare:

- two data members with the same name, or a data member and a member
  function with the same name,
- the same member function two times, also if only one is `static`,
- a static data member, nested class, member type alias, member
  function, or unscoped enumerator with the name of the class, or a
  member of an anonymous union with that name,
- a non-static data member that is `thread_local`, `extern`, or
  `register`,
- a static data member in a class with no name, or in a class nested in
  one,
- a non-static data member with the name of the class, if the class
  declares a constructor,
- a non-static data member with a {term}`placeholder type`,
- a default member initializer in parentheses,
- `mutable` on a `const`, reference, or static member,
- an unnamed member of class type, other than an anonymous union,
- `= 0` on a member function that is not virtual.
:::

:::{rule} CLS-12
:kind: required
:tests: classes/members/local-class.cpp classes/members/local-class-uses-static-local.cpp classes/members/local-class-const-local.cpp classes/members/local-class-constexpr-double.cpp

A function can define a class inside its body. In an evaluated operand,
a member function of that local class can name the static variables of
the function. It can also read the value of a `constexpr` variable of
any literal type, or another constant variable, when it does not take
the address.
:::

:::{rule} CLS-28
:kind: reject
:tests: classes/members/local-class-static-member.cpp classes/members/local-class-uses-local.cpp classes/members/local-class-member-defined-outside.cpp classes/members/local-class-member-template.cpp classes/members/local-class-friend-definition.cpp

Three things are not valid in a local class (CLS-12):

- a static data member,
- a member template,
- a friend function defined in it.

Each member function of a local class is defined inside the class. In an
evaluated operand, it must not name the automatic variables of the
function, except as CLS-12 allows.
:::

:::{rule} CLS-13
:kind: required
:tests: classes/members/explicit-object-parameter.cpp

A member function can declare its first parameter as an explicit object
parameter, written `this T self`. The object of the call initializes that
parameter.
:::

:::{rule} CLS-14
:kind: reject
:tests: classes/members/explicit-object-no-this.cpp classes/members/explicit-object-qualified.cpp classes/members/explicit-object-static.cpp classes/members/explicit-object-virtual.cpp

A member function with an explicit object parameter has no `this`. It
must not:

- be static,
- be virtual,
- have a cv-qualifier or a ref-qualifier.
:::

:::{rule} CLS-15
:kind: required
:tests: classes/members/static-call-operator.cpp

`operator()` and `operator[]` can be static member functions. `operator[]`
can have any number of parameters ({rule}`OPR-3`).
:::

:::{rule} CLS-16
:kind: required
:tests: classes/members/static-member-own-type.cpp classes/members/thread-local-static-member.cpp

A static data member can have the type of its own class, which is still
incomplete at the declaration. Its definition comes after the class. A
static data member can be `thread_local`.
:::

:::{rule} CLS-17
:kind: required
:tests: classes/members/explicit-object-address-ordinary.cpp

The address of a member function with an explicit object parameter is an
ordinary pointer to function. It is not a pointer to member. A call
through it passes the object as the first argument.
:::

:::{rule} CLS-20
:kind: reject
:tests: classes/members/static-const-init-not-constant.cpp

The initializer of a `const` static data member of integer or
enumeration type in its class must be a constant expression.
:::

:::{rule} CLS-19
:kind: required
:tests: classes/members/standard-layout-reinterpret.cpp classes/members/first-member-same-address.cpp

A *standard-layout* class meets these conditions, and others that this
page does not list yet *(class properties referenced)*:

- It has no virtual function and no virtual base.
- Its data members that are not static all have one access.
- Each base and each member is also standard-layout.

The address of such an object equals the address of its first data
member. A `reinterpret_cast` from one of these pointers to the other gives
a usable pointer.
:::

## Runtime Behavior

:::{rule} CLS-10
:kind: required
:tests: classes/members/member-order.cpp classes/members/empty-class-size.cpp classes/members/distinct-addresses.cpp classes/members/member-order-mixed-access.cpp classes/members/first-member-same-address.cpp classes/members/empty-base-shares-address.cpp

Non-static data members of nonzero size have increasing addresses in the
order of their declarations. Members of an anonymous union are not in
this order. An object of an empty class has a size greater than zero. Two
distinct complete objects have distinct addresses. The first data member
of an object and a base subobject of an empty class can have the address
of the object.
:::

## Implementation Requirements

:::{rule} CLS-11
:kind: impl-defined
:tests: classes/members/empty-class-size-one.cpp

The size of an empty class is implementation-defined.
:::

:::{rule} CLS-21
:kind: no-diagnostic
:tests: classes/members/static-const-odr-use-no-definition.cpp

A `const` static data member with an initializer in its class, and not
`constexpr` or `inline`, needs a definition outside the class if the
program uses its address, for example by binding a reference to it. An
implementation need not diagnose a missing definition before linking.
:::

## Examples

```cpp
struct Node {
  int value;
  Node* next;              // fine: a pointer to the incomplete type
  int get() const { return later; }   // fine: later is declared below
  int later = 7;
};

struct Counter {
  static int count;        // needs a definition outside the class
  inline static int level = 3;
  static const int size = 4;
};
int Counter::count = 0;

struct Text {
  int which() &  { return 1; }   // s.which()
  int which() && { return 2; }   // Text{}.which()
};
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 83
of 84 tests.

- CLS-3: a missing definition of a static data member compiles in each
  translation unit. The linker reports it.
- CLS-26: GCC 16 accepts mixed ref-qualified overloads and reports
  ambiguity only at a call ({question}`Q-0027`).
- CLS-9: Clang and GCC accept an anonymous struct member as an
  extension, and reject it only under `-pedantic-errors`.
- CLS-11: the size is 1 on every compiler in the matrix.
- CLS-19: the tests check the consequence for a simple standard-layout
  class. They do not check each condition of the definition.

Inputs: clang-sema-declcxx, clang-sema-decl, clang-diag-sema, edg-class-
decl.
