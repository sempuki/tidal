# Constructors and destructors

A constructor initializes an object of a class. A destructor ends the
life of an object of a class. This page covers how to declare them, the
default constructor and destructor that a class gets implicitly, member
initializer lists, delegating and `explicit` constructors, and the order
of construction and destruction.

Copy and move constructors are in a separate page. Inherited constructors
are with derived classes. Calls to virtual functions during construction
are with virtual functions.

Depends on: class definitions and members, access control, initialization,
`new` and `delete`.

## Syntax

```{eval-rst}
.. productionlist:: ctor
   ctor_decl: {`ctor_specifier`} `class_name` "(" [`parameters`] ")" [`noexcept`]
   ctor_specifier: "constexpr" | "consteval" | "inline" | `explicit_spec`
   explicit_spec: "explicit" ["(" `constant_expression` ")"]
   ctor_def: `ctor_decl` [":" `mem_init` {"," `mem_init`}] `function_body`
   mem_init: `mem_init_id` ("(" [`expression_list`] ")" | `braced_init`)
   mem_init_id: `identifier` | `class_name`
   dtor_decl: {`dtor_specifier`} "~" `class_name` "(" ["void"] ")" [`noexcept`]
   dtor_specifier: "constexpr" | "inline" | "virtual"
   dtor_call: `postfix_expression` ("." | "->") "~" `type_name` "(" ")"
```

A *member initializer list* is the list of `mem_init` after the colon. A
*delegating constructor* has a `mem_init` that names its own class.

## Static Semantics

:::{rule} CTOR-1
:kind: reject
:tests: classes/constructors/ctor-address.cpp classes/constructors/ctor-const-qualified.cpp classes/constructors/ctor-explicit-object.cpp classes/constructors/ctor-out-of-line.cpp classes/constructors/ctor-redeclared.cpp classes/constructors/ctor-ref-qualified.cpp classes/constructors/ctor-return-type.cpp classes/constructors/ctor-specifiers.cpp classes/constructors/ctor-static.cpp classes/constructors/ctor-virtual.cpp classes/constructors/ctor-volatile.cpp

A constructor has the name of its class and no return type. A constructor
must not:

- be `static` or `virtual`,
- have a cv-qualifier or a ref-qualifier,
- have an explicit object parameter,
- be declared two times in its class.

A program cannot take the address of a constructor.
:::

:::{rule} CTOR-2
:kind: required
:tests: classes/constructors/defaulted-default-restores.cpp classes/constructors/implicit-default.cpp classes/constructors/no-implicit-default-with-copy-ctor.cpp classes/constructors/no-implicit-default-with-ctor.cpp

If a class declares no constructor, it has an implicit public default
constructor. A declaration of any other constructor, including a copy
constructor, removes the implicit one. `= default` declares it again.
:::

:::{rule} CTOR-3
:kind: required
:tests: classes/constructors/deleted-default-base.cpp classes/constructors/deleted-default-const-class-member.cpp classes/constructors/deleted-default-const-member.cpp classes/constructors/deleted-default-const-aggregate-member.cpp classes/constructors/deleted-default-member-no-default.cpp classes/constructors/deleted-default-member-private.cpp classes/constructors/deleted-default-only-if-used.cpp classes/constructors/deleted-default-reference-init.cpp classes/constructors/deleted-default-reference.cpp classes/constructors/explicitly-defaulted-but-deleted.cpp

The implicit or defaulted default constructor is deleted if one of these
has no default member initializer:

- a reference member,
- a `const` member whose class has no user-provided default constructor,
  or a `const` member of a type that is not a class.

It is also deleted if a base or member has no default constructor that is
accessible and not deleted. A deleted default constructor makes only its
use invalid.
:::

:::{rule} CTOR-4
:kind: reject
:tests: classes/constructors/const-object-defaulted-in-class.cpp classes/constructors/const-object-empty.cpp classes/constructors/const-object-member-init.cpp classes/constructors/const-object-no-init.cpp classes/constructors/const-object-user-ctor.cpp

A `const` object without an initializer must have a class type that
gives each member a value. One of these must be true:

- The class has a user-provided default constructor.
- Each data member has a default member initializer or is itself such a
  class.
- The class has no data members.

A default constructor defaulted on its first declaration is not
user-provided.
:::

:::{rule} CTOR-5
:kind: required
:tests: classes/constructors/mem-init-base-and-member.cpp classes/constructors/mem-init-braces-narrowing.cpp classes/constructors/mem-init-braces.cpp classes/constructors/mem-init-overrides-default.cpp classes/constructors/mem-init-type-alias-base.cpp classes/constructors/mem-init-uses-param.cpp

A member initializer list initializes direct bases and non-static data
members. Each initializer uses parentheses or braces. Braces use the
rules of list initialization, so a narrowing conversion is not valid. An
initializer for a member replaces its default member initializer.
:::

:::{rule} CTOR-15
:kind: reject
:tests: classes/constructors/mem-init-duplicate.cpp classes/constructors/mem-init-in-declaration-only.cpp classes/constructors/mem-init-indirect-base.cpp classes/constructors/mem-init-not-member.cpp classes/constructors/mem-init-pack-of-reference.cpp classes/constructors/mem-init-static-member.cpp

Each `mem_init` must name a non-static data member, a direct base, or a
virtual base *(referenced)* of the class. A name can be a type alias of a
base. A `mem_init` must not:

- name the same member or base two times,
- occur in a constructor declaration that is not a definition,
- value-initialize a reference member.
:::

:::{rule} CTOR-8
:kind: reject
:tests: classes/constructors/delegating-self.cpp

A constructor must not delegate to itself.
:::

:::{rule} CTOR-16
:kind: reject
:tests: classes/constructors/delegating-with-member.cpp

The `mem_init` of a delegating constructor must be the only one in its
list.
:::

:::{rule} CTOR-9
:kind: reject
:tests: classes/constructors/explicit-argument.cpp classes/constructors/explicit-bool-false.cpp classes/constructors/explicit-bool-not-constant.cpp classes/constructors/explicit-bool-true.cpp classes/constructors/explicit-copy-init.cpp classes/constructors/explicit-copy-list-init.cpp classes/constructors/explicit-default-copy-list.cpp classes/constructors/explicit-direct-init.cpp classes/constructors/explicit-non-ctor.cpp classes/constructors/explicit-out-of-line.cpp classes/constructors/explicit-return.cpp classes/constructors/explicit-static-cast.cpp

An `explicit` constructor is not used in copy-initialization. This
includes:

- `T x = v;` and `T x = {v};`,
- passing an argument,
- returning a value, including `return {};`.

Direct initialization and casts can use it. `explicit(c)` makes the
constructor `explicit` if the constant expression `c` is `true`. Only a
constructor or conversion function *(referenced)* declared in its class
can have `explicit`.
:::

:::{rule} CTOR-10
:kind: required
:tests: classes/constructors/converting-ctor-multi-arg-list.cpp classes/constructors/converting-ctor.cpp classes/constructors/one-user-conversion-plus-standard.cpp classes/constructors/one-user-conversion.cpp

A constructor that is not `explicit` is a *converting constructor*. An
implicit conversion can use it to make an object from its arguments,
including from a braced list. An implicit conversion uses at most one
converting constructor. Standard conversions can come before it.
:::

:::{rule} DTOR-1
:kind: reject
:tests: classes/constructors/dtor-const.cpp classes/constructors/dtor-explicit-object.cpp classes/constructors/dtor-out-of-line.cpp classes/constructors/dtor-params.cpp classes/constructors/dtor-ref-qualified.cpp classes/constructors/dtor-return-type.cpp classes/constructors/dtor-specifiers.cpp classes/constructors/dtor-static.cpp classes/constructors/dtor-two.cpp classes/constructors/dtor-variadic.cpp classes/constructors/dtor-void-param.cpp classes/constructors/dtor-wrong-name.cpp

A destructor has the name of its class after `~`, no parameters, and no
return type. A parameter list of `(void)` is valid. A destructor must
not:

- be `static`,
- have a cv-qualifier or a ref-qualifier,
- be variadic or have an explicit object parameter,
- be declared two times in its class.
:::

:::{rule} DTOR-2
:kind: required
:tests: classes/constructors/dtor-on-const-object.cpp

If a class declares no destructor, it has an implicit public destructor.
A destructor can run on a `const` object.
:::

:::{rule} DTOR-3
:kind: reject
:tests: classes/constructors/deleted-dtor-delete.cpp classes/constructors/deleted-dtor-new.cpp classes/constructors/deleted-dtor-temporary.cpp classes/constructors/deleted-dtor-variable.cpp classes/constructors/implicitly-deleted-dtor-member.cpp classes/constructors/private-dtor-member.cpp classes/constructors/private-dtor-variable.cpp

If the destructor of a class is deleted or not accessible, a program must
not:

- declare a variable of the class with automatic, static, or thread
  storage duration,
- create a temporary of the class,
- apply `delete` to an object of the class.

A `new` expression for the class is valid. The implicit destructor is
deleted if a base or member has a destructor that is deleted or not
accessible.
:::

:::{rule} DTOR-4
:kind: required
:tests: classes/constructors/dtor-member-noexcept-false.cpp classes/constructors/dtor-noexcept-false.cpp classes/constructors/implicit-dtor-noexcept.cpp classes/constructors/user-dtor-noexcept.cpp

A destructor without a `noexcept` specifier is `noexcept` if the
destructors of all bases and members are `noexcept`. This applies to
implicit and user-declared destructors.
:::

:::{rule} DTOR-8
:kind: reject
:tests: classes/constructors/dtor-address.cpp classes/constructors/dtor-call-wrong-type.cpp classes/constructors/dtor-without-call.cpp classes/constructors/pseudo-dtor-wrong-type.cpp

A program must call a destructor with `()`. It cannot take the address of
a destructor. In an explicit destructor call, the name after `~` must
denote the type of the object. This also applies to a scalar type.
:::

## Dynamic Semantics

:::{rule} CTOR-6
:kind: required
:tests: classes/constructors/body-after-members.cpp classes/constructors/init-order-declaration.cpp classes/constructors/init-order-uses-earlier.cpp

A constructor initializes in this order:

1. direct bases, in the order of the base list,
2. non-static data members, in the order of their declarations,
3. the constructor body executes.

The order of the member initializer list has no effect.
:::

:::{rule} CTOR-7
:kind: required
:tests: classes/constructors/delegating-ignores-default-member-init.cpp classes/constructors/delegating.cpp

A delegating constructor first executes the constructor that it names.
Then its own body executes. Default member initializers apply only
through the target constructor.
:::

:::{rule} CTOR-11
:kind: required
:tests: classes/constructors/const-object-modified-in-ctor.cpp classes/constructors/ctor-on-const-this-type.cpp

A constructor can change the members of a `const` object. The object is
`const` only after its constructor completes.
:::

:::{rule} DTOR-5
:kind: required
:tests: classes/constructors/dtor-order-array.cpp classes/constructors/dtor-order-automatic.cpp classes/constructors/dtor-order-delete-array.cpp classes/constructors/dtor-order.cpp classes/constructors/implicit-dtor-runs-members.cpp

A destructor works in the opposite order to a constructor:

1. its body executes,
2. the data members are destroyed, last declared first,
3. the direct bases are destroyed, last listed first.

In a block, the variable constructed last is destroyed first. Array
elements are destroyed from the last element to the first. This applies
to `delete[]` too.
:::

:::{rule} DTOR-6
:kind: required
:tests: classes/constructors/dtor-call-via-alias.cpp classes/constructors/explicit-dtor-call.cpp classes/constructors/pseudo-dtor-scalar.cpp

An explicit destructor call runs the destructor and ends the life of the
object. Its storage is not released. The name after `~` can be a type
alias. A program can also call a destructor on an object of scalar type
through a type name.
:::

## Undefined Behavior

:::{rule} CTOR-12
:kind: undefined
:tests: classes/constructors/defaulted-leaves-members.cpp classes/constructors/init-order-reads-later.cpp

If a member is read before it has a value, the behavior is undefined.
This includes a member that a later `mem_init` initializes, and a member
that default initialization leaves without a value.
:::

:::{rule} DTOR-7
:kind: undefined
:tests: classes/constructors/double-destroy-constant.cpp classes/constructors/use-after-dtor-constant.cpp

If a program uses an object after its destructor runs, the behavior is
undefined. This includes a second destruction, for example at the end of
the block after an explicit call ({question}`Q-0054`).
:::

## Implementation Requirements

:::{rule} CTOR-14
:kind: no-diagnostic
:tests: classes/constructors/delegating-cycle.cpp

Constructors must not delegate to each other in a cycle. An
implementation need not diagnose a cycle that does not delegate directly
to itself ({question}`Q-0053`).
:::

## Recommended Practice

:::{rule} CTOR-13
:kind: recommended
:tests: classes/constructors/init-order-same-order.cpp classes/constructors/init-order-warns.cpp

An implementation should warn if the member initializer list has an
order different from the order of initialization.
:::

## Examples

```cpp
struct Buffer {
  int size;
  int* data;
  Buffer(int n) : data(new int[n]), size(n) {}  // size is initialized first
  Buffer() : Buffer(16) {}                        // delegating
  ~Buffer() { delete[] data; }
};

struct Meters { explicit Meters(double v) : v(v) {} double v; };
Meters a(2.0);                   // valid
Meters b = 2.0;                  // rejected: explicit

struct Ref { int& r; };
Ref r;                           // rejected: default constructor deleted

struct Counter { int n; };
const Counter c;                 // rejected: n has no value
const Counter d{};               // valid
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 108 of
110 tests.

- CTOR-14: Clang and EDG reject an indirect delegation cycle. GCC accepts
  it ({question}`Q-0053`).
- DTOR-7: EDG 7.0 accepts a second destruction during constant evaluation
  ({question}`Q-0054`).

Inputs: clang-sema-declcxx, clang-sema-init, clang-diag-sema,
edg-class-decl, edg-decl-inits.
