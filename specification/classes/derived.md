# Derived classes

A class can derive from one or more base classes. An object of the
derived class contains a subobject for each base. This page covers:

- the base list,
- name lookup in derived classes,
- conversions to a base,
- virtual bases,
- the order of construction with virtual bases,
- inherited constructors.

Access to bases is with access control. Virtual functions have their own
page.

Depends on: class definitions and members, access control, constructors
and destructors, copy and move.

## Principles

:::{principle} DER-P1
:rules: DER-1 DER-2 DER-18 DER-22

A derived object contains one {term}`subobject` for each base in its base list,
and the members of each base become members of the derived class
({principle}`OBJ-P1`). A base must therefore be a complete class that
allows derivation. The same class reached by two non-virtual paths gives
two separate subobjects. The two subobjects are valid, and a member
reached through both is ambiguous.
:::

:::{principle} DER-P2
:rules: DER-3 DER-4 DER-9 DER-19

Lookup in a derived class searches the class and then its bases, as
scopes nested outward ({principle}`NAM-P2`). A name in the derived class
hides the same name in every base, overloads included, and a
using-declaration brings a base name back. A name found in two base
subobjects is ambiguous until the program picks a path.
:::

:::{principle} DER-P3
:rules: DER-5 DER-10 DER-11 DER-16 DER-17

A derived object is a base object with more parts, so a pointer or
reference to the derived object converts to the base implicitly and
lands on the base subobject. The other direction is a claim about what
the object really is, so the conversion:

- needs `static_cast`,
- is undefined when the claim is false,
- is impossible from a {term}`virtual base`, whose location depends on the most
  derived class.
:::

:::{principle} DER-P4
:rules: DER-6 DER-7 DER-12 DER-13

A virtual base is shared by every path, so an object has one subobject
of that base. Compare a base `A` that `D` reaches through `B` and
through `C`:

```text
A is a non-virtual base           A is a virtual base

  A         A                            A
  |         |                           / \
  B         C                          B   C
   \       /                            \ /
       D                                 D

a D holds two A subobjects        a D holds one A subobject
```

Only the {term}`most derived class` knows the whole object, so its constructor
initializes the virtual base, first and once. Every constructor of that
class must be able to initialize the virtual base. The position of the
virtual base is decided at run time, so such a class has no `constexpr`
constructor or destructor.
:::

:::{principle} DER-P5
:rules: DER-8 DER-14 DER-15 DER-20 DER-21

Constructing a derived object constructs its bases, so the derived class
needs access to the constructors and destructors of the bases. A derived
class can inherit the constructors of a direct base. An inherited
constructor builds the base subobject and leaves the rest to {term}`default initialization`. A signature that two bases both offer is ambiguous.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   base_clause: ":" `base_spec` ["..."] {"," `base_spec` ["..."]}
   base_spec: [`access_specifier`] ["virtual"] [`nested_name`] `class_name`
            : | "virtual" `access_specifier` [`nested_name`] `class_name`
   class_head: ("class" | "struct") {`attribute_spec` | `alignment_specifier`}
             :    [[`nested_name`] (`class_name` | `template_id`) ["final"]] [`base_clause`]
   inheriting_decl: "using" [`nested_name`] `class_name` "::" `class_name` ["..."] ";"
```

A `base_spec` or an `inheriting_decl` with `...` expands a pack of bases
({rule}`PACK-14`, {rule}`PACK-21`).

A *direct base* is a class in the base list. An *indirect base* is a base
of a base. A *virtual base* is a base named with `virtual`. The *most
derived class* of an object is the class that the object was created as.

## Constraints

:::{rule} DER-1
:kind: reject
:tests: classes/derived/base-alias.cpp classes/derived/base-duplicate.cpp classes/derived/base-enum.cpp classes/derived/base-final.cpp classes/derived/base-incomplete.cpp classes/derived/base-not-class.cpp classes/derived/base-qualified.cpp classes/derived/base-self.cpp classes/derived/base-union.cpp classes/derived/final-not-keyword.cpp classes/derived/union-as-derived.cpp

Each class in a base list must be:

- a complete class type that is not a union,
- not the class being defined,
- not marked `final`,
- named only once in the list.

The name can be a type alias or a qualified name. A union must not have a
base list. `final` after the class name is a marker only in that position.
:::

:::{rule} DER-2
:kind: required
:tests: classes/derived/aggregate-with-base.cpp classes/derived/inherits-members.cpp classes/derived/repeated-base-two-subobjects.cpp classes/derived/sizeof-at-least-base.cpp

An object of a derived class contains one subobject of each non-virtual
direct base. The members of a base are members of the derived class. If
the same class is a non-virtual base on two paths, the object contains two
separate subobjects of that class. An aggregate can have bases, and its
initializer list gives the bases first.
:::

:::{rule} DER-3
:kind: required
:tests: classes/derived/hiding-type-by-member.cpp classes/derived/qualified-name-reaches-base.cpp classes/derived/using-brings-overloads.cpp classes/derived/using-derived-wins.cpp foundations/names/injected-member-hides-base-name.cpp

A name declared in a derived class hides every member with that name in
its bases. The hidden members include:

- all overloads of a function,
- data members,
- types.

A qualified name such as `d.B::f()` finds the base member. A
using-declaration of a base member makes the overloads of that member
visible in the derived class. A derived member with the same parameters
hides the base function that the using-declaration brings ({rule}`OVL-1`).
:::

:::{rule} DER-19
:kind: reject
:tests: classes/derived/data-member-hides-function.cpp classes/derived/derived-hides-base-overloads.cpp

A program must not use an unqualified name to reach a base member that
DER-3 hides.
:::

:::{rule} DER-9
:kind: reject
:tests: classes/derived/inherited-ctor-not-direct-base.cpp classes/derived/using-not-a-base.cpp

A using-declaration in a class must name a member of a base. A
using-declaration that inherits constructors must name a direct base.
:::

:::{rule} DER-4
:kind: reject
:tests: classes/derived/ambiguous-member-unused.cpp classes/derived/ambiguous-member.cpp classes/derived/ambiguous-overloads-different-bases.cpp classes/derived/ambiguous-qualified-ok.cpp classes/derived/ambiguous-repeated-base-member.cpp classes/derived/ambiguous-resolved-by-using.cpp classes/derived/dominance-derived-hides.cpp classes/derived/dominance-nonvirtual-ambiguous.cpp classes/derived/repeated-base-static-ok.cpp foundations/names/injected-two-bases-ambiguous.cpp foundations/names/injected-two-paths-virtual.cpp foundations/names/injected-two-paths.cpp

A name lookup finds no declaration, a set of declarations, or an
ambiguity. If a name lookup in a derived class finds the name in more
than one base subobject, the result is an ambiguity. This rule also
applies to overloads from different bases. These exceptions apply:

- A static member, a type, or an enumerator found through two paths to
  the same class is not ambiguous. The name of a base that a class
  inherits ({rule}`NAM-35`) is such a type.
- A declaration in a class hides a declaration with the same name in the
  virtual bases of that class, on every path.

A qualified name or a using-declaration can remove the ambiguity. An
ambiguity makes the program not valid only where the program uses the
name. Two rules name specific cases of this rule (decision 0042):
{rule}`NAM-18` for a qualified name `C::x`, and {rule}`NAM-36` for an
{term}`injected class name` inherited from two different bases.
:::

:::{rule} DER-5
:kind: required
:tests: classes/derived/ambiguous-base-via-path.cpp classes/derived/derived-to-base-constant.cpp classes/derived/derived-to-base-pointer.cpp classes/derived/null-derived-to-base.cpp classes/derived/second-base-address.cpp classes/derived/slicing.cpp

A pointer or reference to a derived class converts implicitly to a
pointer or reference to an unambiguous base. The result refers to the
base subobject, which can have a different address. A null pointer
converts to a null pointer. An object of the derived class can
initialize a base object. The initialization copies only the base part.
{term}`Constant evaluation <constant evaluation>` can do this conversion.
:::

:::{rule} DER-10
:kind: reject
:tests: classes/derived/ambiguous-base-conversion.cpp classes/derived/base-to-derived-implicit.cpp classes/derived/virtual-and-nonvirtual-base.cpp

A pointer or reference to a base does not convert implicitly to a derived
class. A conversion to a base that occurs as more than one subobject is
not valid. A path through one direct base can select one of the
subobjects.
:::

:::{rule} DER-6
:kind: required
:tests: classes/derived/virtual-base-conversion-unambiguous.cpp classes/derived/virtual-base-shared.cpp classes/derived/virtual-keyword-order.cpp

All virtual bases of the same class in an object share one subobject.
A conversion to that base is not ambiguous. `virtual` and the access
specifier can be in either order.
:::

:::{rule} DER-11
:kind: reject
:tests: classes/derived/virtual-base-downcast.cpp classes/derived/virtual-base-downcast-reference.cpp

`static_cast` must not convert a pointer or reference to a virtual base
to a derived class.
:::

:::{rule} DER-12
:kind: reject
:tests: classes/derived/constexpr-ctor-virtual-base.cpp classes/derived/constexpr-dtor-virtual-base.cpp classes/derived/constexpr-object-virtual-base.cpp expressions/constant-expressions/implicit-constructor-virtual-base.cpp expressions/constant-expressions/defaulted-constructor-virtual-base.cpp expressions/constant-expressions/virtual-base-path-not-reached.cpp expressions/constant-expressions/implicit-copy-virtual-base.cpp

A class with a virtual base must not have a `constexpr` constructor or a
`constexpr` destructor. Its implicit and defaulted constructors are not
`constexpr` either ({rule}`CE-2`). An object of such a class cannot be a
`constexpr` variable.
:::

:::{rule} DER-13
:kind: reject
:tests: classes/derived/abstract-skips-virtual-base-init.cpp classes/derived/most-derived-must-init-virtual-base.cpp

Each constructor of the most derived class must be able to initialize
every virtual base. If a virtual base has no {term}`default constructor`, each
such constructor must name the virtual base in its member initializer
list. An {term}`abstract class` is never the most derived class, so its
constructors need not initialize its virtual bases.
:::

:::{rule} DER-14
:kind: required
:tests: classes/derived/base-protected-ctor.cpp classes/derived/base-protected-dtor.cpp

A protected constructor or destructor of a base gives a derived class
access to that constructor or destructor.
:::

:::{rule} DER-21
:kind: reject
:tests: classes/derived/base-private-ctor.cpp classes/derived/base-private-dtor.cpp classes/derived/virtual-base-private-dtor.cpp

A derived class needs access to the constructor and destructor of each
base that the derived class uses. This requirement includes virtual
bases that are not direct bases. A private constructor or destructor
does not give this access, so the special member of the derived class is
deleted.
:::

:::{rule} DER-8
:kind: required
:tests: classes/derived/inherited-ctor-default-args.cpp classes/derived/inherited-ctor-hidden-by-derived.cpp classes/derived/inherited-ctor.cpp classes/derived/inherited-ctor-access-in-friend.cpp classes/derived/inherited-ctor-not-copy.cpp

`using B::B;` in a class `D` *inherits* the constructors of the direct
base `B`. A call with matching arguments initializes the `B` subobject
with the constructor of `B`. The other bases and members of `D` are
default-initialized. These rules apply:

- Default, copy, and move constructors are not inherited.
- An inherited constructor keeps its access, its `explicit`, and its
  default arguments. The access check happens where `D` is
  initialized, as for the constructor of `B` named there.
- A constructor of `D` with the same parameters hides the inherited
  constructor.

A class that inherits constructors is not an aggregate
({doc}`/declarations/initialization/list`).
:::

:::{rule} DER-20
:kind: reject
:tests: classes/derived/inherited-ctor-access.cpp classes/derived/inherited-ctor-access-in-friend.cpp classes/derived/inherited-ctor-explicit.cpp classes/derived/inherited-ctor-no-default.cpp classes/derived/inherited-ctor-not-copy.cpp

DER-8 sets limits on inherited constructors. Through `D`, a program must
not use:

- a constructor of `B` that DER-8 does not inherit,
- an inherited constructor that is not accessible,
- an inherited `explicit` constructor in {term}`copy-initialization`.
:::

:::{rule} DER-15
:kind: reject
:tests: classes/derived/inherited-ctor-two-bases-ambiguous.cpp classes/derived/inherited-ctor-two-bases-unused.cpp

A call must not select a constructor signature that two bases both
provide through inherited constructors.
:::

:::{rule} DER-16
:kind: required
:tests: classes/derived/static-cast-downcast.cpp classes/derived/static-cast-downcast-two-levels.cpp

`static_cast` can convert a pointer or reference to a base `B` into a
pointer or reference to a derived class `D`. `B` must be a base of `D`
that is:

- non-virtual,
- accessible,
- unambiguous.
:::

:::{rule} DER-18
:kind: required
:tests: classes/derived/direct-and-indirect-base.cpp classes/derived/direct-and-indirect-base-deep.cpp

A class can be both a direct base and an indirect base of the same
derived class. The declaration is valid.
:::

:::{rule} DER-22
:kind: reject
:tests: classes/derived/direct-and-indirect-base-use.cpp classes/derived/direct-and-indirect-base-call.cpp

If a class is both a direct and an indirect base (DER-18), a use of a
member of that base through the derived class is ambiguous.
:::

## Runtime Behavior

:::{rule} DER-7
:kind: required
:tests: classes/derived/most-derived-inits-virtual-base.cpp classes/derived/virtual-base-constructed-first.cpp classes/derived/virtual-base-constructed-once.cpp classes/derived/virtual-base-init-indirect.cpp

The constructor of the most derived class initializes each virtual base
once, before the other bases. The member initializers for a virtual base
in other classes have no effect. Each virtual base is destroyed after the
other bases.
:::

## Undefined Behavior

:::{rule} DER-17
:kind: undefined
:tests: classes/derived/static-cast-downcast-wrong-object-constant.cpp classes/derived/static-cast-downcast-wrong-object-sanitizer.cpp

If a `static_cast` to a derived class `D` applies to an object that is
not a `D` subobject, the behavior is undefined. Constant evaluation
rejects such a cast.
:::

## Examples

```cpp
struct Shape { int id; };
struct Square : Shape { int side; };
Square s{{7}, 2};                // base part first
Shape* p = &s;                   // valid: derived to base
Square* q = p;                   // rejected: base to derived

struct Logger { void write(int); };
struct Report : Logger {
  using Logger::write;           // keeps write(int) visible
  void write(const char*);
};

struct Device { Device(int port); };
struct Disk : virtual Device { Disk() : Device(1) {} };
struct Tape : virtual Device { Tape() : Device(2) {} };
struct Drive : Disk, Tape {
  Drive() : Device(3) {}         // the only Device initializer that runs
};

class Socket {
  Socket(int fd);                // private
  friend struct Server;
};
struct Server : Socket {
  using Socket::Socket;
  static Server open() { return Server(3); }  // valid: Server is a friend
};
Server s(3);                     // rejected: Socket(int) is private here
```

A base name reached through two paths is ambiguous only if the paths
lead to different classes (DER-4):

```cpp
struct Node { };
struct Left : Node { };
struct Right : Node { };
struct Pair : Left, Right {
  Node* first();                 // valid: one class, Node, on both paths
};

namespace a { struct Item { }; }
namespace b { struct Item { }; }
struct Both : a::Item, b::Item {
  Item* get();                   // rejected: two different classes
};
```

A derived visitor that overrides one `visit` hides the others, and
Clang and GCC do not warn under `-Wall` (DER-3, {rule}`VIRT-13`):

```cpp
struct Num; struct Add;
struct Walker {
  virtual void visit(const Num&) {}
  virtual void visit(const Add&) {}
};
struct Counter : Walker {
  using Walker::visit;           // keeps visit(const Add&) visible
  void visit(const Num&) override;
};
```

A mixin casts itself to the class that derives from it (DER-16). The
protected constructor keeps other code from making a mixin that is not
part of a `Self`, which DER-17 makes undefined:

```cpp
template <class Self> struct Doubling {
  int twice() const { return 2 * static_cast<const Self&>(*this).value(); }
protected:
  Doubling() = default;
};
struct Price : Doubling<Price> {
  int cents = 5;
  int value() const { return cents; }
};
Price p;
int t = p.twice();               // 10
Doubling<Price> d;               // rejected: the constructor is protected
```

A class with a virtual base has no `constexpr` constructor, so constant
evaluation cannot create one (DER-12, {rule}`CE-2`). A `constexpr`
function can still create one on a path that runs only at run time:

```cpp
struct Base { int n = 3; };
struct Left : virtual Base { };
constexpr int make(bool build) {
  if (build) { Left x; return x.n; }
  return 0;
}
static_assert(make(false) == 0); // fine: the object is not created
static_assert(make(true) == 3);  // rejected: Left() is not constexpr
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 85
of 86 tests.

- DER-12: EDG accepts a `constexpr` destructor in a class with a virtual
  base. Clang and GCC reject the destructor ({question}`Q-0085`).

Inputs: clang-sema-declcxx, clang-sema-access, clang-diag-sema,
edg-class-decl.
