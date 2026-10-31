# Derived classes

A class can derive from one or more base classes. An object of the
derived class contains a subobject for each base. This page covers the
base list, name lookup in derived classes, conversions to a base, virtual
bases, the order of construction with virtual bases, and inherited
constructors.

Access to bases is with access control. Virtual functions have their own
page.

Depends on: class definitions and members, access control, constructors
and destructors, copy and move.

## Syntax

```{eval-rst}
.. productionlist:: derived
   base_clause: ":" `base_spec` {"," `base_spec`}
   base_spec: [`access_specifier`] ["virtual"] `class_name`
            : "virtual" `access_specifier` `class_name`
   class_head: ("class" | "struct") `class_name` ["final"] [`base_clause`]
   inheriting_decl: "using" `class_name` "::" `class_name` ";"
```

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
separate subobjects of it. An aggregate can have bases, and its
initializer list gives the bases first.
:::

:::{rule} DER-3
:kind: required
:tests: classes/derived/data-member-hides-function.cpp classes/derived/derived-hides-base-overloads.cpp classes/derived/hiding-type-by-member.cpp classes/derived/qualified-name-reaches-base.cpp classes/derived/using-brings-overloads.cpp classes/derived/using-derived-wins.cpp

A name declared in a derived class hides every member with that name in
its bases. This includes all overloads of a function, and data members
and types. A qualified name such as `d.B::f()` finds the base member. A
using-declaration of a base member makes its overloads visible in the
derived class. A derived member with the same parameters hides the one
that the using-declaration brings.
:::

:::{rule} DER-9
:kind: reject
:tests: classes/derived/inherited-ctor-not-direct-base.cpp classes/derived/using-not-a-base.cpp

A using-declaration in a class must name a member of a base. A
using-declaration that inherits constructors must name a direct base.
:::

:::{rule} DER-4
:kind: reject
:tests: classes/derived/ambiguous-member-unused.cpp classes/derived/ambiguous-member.cpp classes/derived/ambiguous-overloads-different-bases.cpp classes/derived/ambiguous-qualified-ok.cpp classes/derived/ambiguous-repeated-base-member.cpp classes/derived/ambiguous-resolved-by-using.cpp classes/derived/dominance-derived-hides.cpp classes/derived/dominance-nonvirtual-ambiguous.cpp classes/derived/repeated-base-static-ok.cpp

If a name lookup in a derived class finds the name in more than one base
subobject, the use of the name is ambiguous. This applies to overloads
from different bases too. These exceptions apply:

- A static member, a type, or an enumerator found through two paths to
  the same class is not ambiguous.
- A declaration in a class hides a declaration with the same name in its
  virtual bases, on every path.

A qualified name or a using-declaration can remove the ambiguity. A name
that is ambiguous but not used is valid.
:::

:::{rule} DER-5
:kind: required
:tests: classes/derived/ambiguous-base-via-path.cpp classes/derived/derived-to-base-constant.cpp classes/derived/derived-to-base-pointer.cpp classes/derived/null-derived-to-base.cpp classes/derived/second-base-address.cpp classes/derived/slicing.cpp

A pointer or reference to a derived class converts implicitly to a
pointer or reference to an unambiguous base. The result refers to the
base subobject, which can have a different address. A null pointer
converts to a null pointer. An object of the derived class can initialize
a base object, which copies only the base part. Constant evaluation can
do this conversion.
:::

:::{rule} DER-10
:kind: reject
:tests: classes/derived/ambiguous-base-conversion.cpp classes/derived/base-to-derived-implicit.cpp classes/derived/virtual-and-nonvirtual-base.cpp

A pointer or reference to a base does not convert implicitly to a derived
class. A conversion to a base that occurs as more than one subobject is
not valid. A path through one direct base can select one of them.
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
:tests: classes/derived/virtual-base-downcast.cpp

`static_cast` must not convert a pointer or reference to a virtual base
to a derived class.
:::

:::{rule} DER-12
:kind: reject
:tests: classes/derived/constexpr-ctor-virtual-base.cpp classes/derived/constexpr-object-virtual-base.cpp

A class with a virtual base must not have a `constexpr` constructor. An
object of such a class cannot be a `constexpr` variable.
:::

:::{rule} DER-13
:kind: reject
:tests: classes/derived/most-derived-must-init-virtual-base.cpp

Each constructor of the most derived class must be able to initialize
every virtual base. If a virtual base has no default constructor, each
such constructor must name the virtual base in its member initializer
list.
:::

:::{rule} DER-14
:kind: required
:tests: classes/derived/base-private-ctor.cpp classes/derived/base-private-dtor.cpp classes/derived/base-protected-ctor.cpp classes/derived/base-protected-dtor-direct.cpp classes/derived/base-protected-dtor.cpp classes/derived/virtual-base-private-dtor.cpp

A derived class needs access to the constructor and destructor of each
base that it uses. This includes virtual bases that are not direct bases.
A protected constructor or destructor gives this access. A private one
does not, so the special member of the derived class is deleted.
:::

:::{rule} DER-8
:kind: required
:tests: classes/derived/inherited-ctor-access.cpp classes/derived/inherited-ctor-default-args.cpp classes/derived/inherited-ctor-explicit.cpp classes/derived/inherited-ctor-hidden-by-derived.cpp classes/derived/inherited-ctor-no-default.cpp classes/derived/inherited-ctor-not-copy.cpp classes/derived/inherited-ctor.cpp

`using B::B;` in a class `D` *inherits* the constructors of the direct
base `B`. A call with matching arguments initializes the `B` subobject
with the constructor of `B`. The other bases and members of `D` are
default-initialized. These rules apply:

- Default, copy, and move constructors are not inherited.
- An inherited constructor keeps its access, its `explicit`, and its
  default arguments.
- A constructor of `D` with the same parameters hides the inherited one.
:::

:::{rule} DER-15
:kind: reject
:tests: classes/derived/inherited-ctor-two-bases-ambiguous.cpp classes/derived/inherited-ctor-two-bases-unused.cpp

A call must not select a constructor signature that two bases both
provide through inherited constructors.
:::

## Dynamic Semantics

:::{rule} DER-7
:kind: required
:tests: classes/derived/most-derived-inits-virtual-base.cpp classes/derived/virtual-base-constructed-first.cpp classes/derived/virtual-base-constructed-once.cpp classes/derived/virtual-base-init-indirect.cpp

The constructor of the most derived class initializes each virtual base
once, before the other bases. The member initializers for a virtual base
in other classes have no effect. Each virtual base is destroyed after the
other bases.
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
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on all
67 tests.

Inputs: clang-sema-declcxx, clang-sema-access, clang-diag-sema,
edg-class-decl.
