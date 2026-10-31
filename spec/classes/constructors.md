# Constructors and destructors

A constructor initializes an object of a class. A destructor ends the
life of an object of a class. This page covers:

- how to declare them,
- the default constructor and destructor that a class gets implicitly,
- member initializer lists,
- delegating and `explicit` constructors,
- the order of construction and destruction.

Copy and move constructors are in a separate page. Inherited constructors
are with derived classes. Calls to virtual functions during construction
are with virtual functions.

Depends on: class definitions and members, access control, initialization,
`new` and `delete`.

## Principles

:::{principle} CTOR-P1
:rules: CTOR-1 DTOR-1 DTOR-6 DTOR-8

A constructor begins the life of an object and a destructor ends it
({principle}`OBJ-P1`). Both are known by the class name, return nothing,
and act on the object in front of them, so neither can be static or
qualified, and a constructor cannot be virtual because no object exists
yet to dispatch on. A program can end an object by calling its
destructor by name, with the call syntax. The storage stays, and only
the object is gone.
:::

:::{principle} CTOR-P2
:rules: CTOR-2 CTOR-3 CTOR-4 DTOR-2 DTOR-3 DTOR-4 CTOR-19 CTOR-20 CTOR-23 DTOR-9

The implementation writes a default constructor and a destructor for a
class that writes none, because building and ending the parts is right
for most classes. Declaring any constructor says that default
construction needs thought, so it removes the implicit default
constructor. When a part cannot be default-constructed or destroyed, the
synthesized function is deleted and only a use fails. A `const` object
with no initializer needs a class that gives every part a value, because
nothing can give it one later. The `noexcept` of a destructor follows
from its parts. A synthesized function that does nothing beyond the
parts' own trivial work is *trivial*, and some rules permit more for
it, such as a jump past a declaration (JMP-6).
:::

:::{principle} CTOR-P3
:rules: CTOR-5 CTOR-6 CTOR-11 CTOR-12 CTOR-13 CTOR-18 CTOR-21

A constructor builds the parts first and then runs its body. The parts
are built in declaration order, bases before members, whatever order the
member initializer list has, because the destructor must undo them in
one fixed order. An implementation should warn when the list suggests
another order. Until the constructor completes, the object is not yet
`const`, and a part has no value until its turn, so reading it earlier
is undefined. A nested class's default member initializer is part of
the enclosing class, so it is usable only once that class is complete.
:::

:::{principle} CTOR-P6
:rules: CTOR-7 CTOR-8 CTOR-14 CTOR-15 CTOR-16 CTOR-17

A member initializer names one part of the object and gives it its
value. It cannot name anything else, and a reference member that it
binds to a temporary is dead before the body runs. A delegating
constructor hands the whole job to one other constructor and adds only
its own body, so its list names that constructor and nothing else. It
cannot hand the job back to itself, directly or through a cycle, and an
implementation need not find a cycle that runs through several
constructors.
:::

:::{principle} CTOR-P4
:rules: DTOR-5 DTOR-7

A destructor runs its body first and then ends the parts in the reverse
of their construction order ({principle}`OBJ-P2`):

```text
constructor                         destructor
  1. bases, in declaration order      1. destructor body
  2. members, in declaration order    2. members, last first
  3. constructor body                 3. bases, last first
```

After it runs, the object is gone, and using it or destroying it again
is undefined ({principle}`OBJ-P4`).
:::

:::{principle} CTOR-P5
:rules: CTOR-9 CTOR-10 CTOR-22

A constructor that takes one argument is a conversion from that argument
type, and an implicit conversion can use it. `explicit` withdraws that
offer, so the constructor then runs only where the program names the
type.
:::

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

## Constraints

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
:tests: classes/constructors/defaulted-default-restores.cpp classes/constructors/implicit-default.cpp classes/constructors/inherited-ctors-keep-default.cpp

If a class declares no constructor, it has an implicit public default
constructor. A declaration of any other constructor, including a copy
constructor, removes the implicit one. `= default` declares it again. A
using-declaration that inherits constructors does not declare a
constructor, so the implicit default constructor stays.
:::

:::{rule} CTOR-19
:kind: reject
:tests: classes/constructors/no-implicit-default-with-copy-ctor.cpp classes/constructors/no-implicit-default-with-ctor.cpp

If CTOR-2 gives a class no default constructor, an initialization that
needs one is not valid.
:::

:::{rule} CTOR-3
:kind: required
:tests: classes/constructors/deleted-default-const-class-member.cpp classes/constructors/deleted-default-only-if-used.cpp classes/constructors/deleted-default-reference-init.cpp classes/constructors/const-member-all-default-inits.cpp

The implicit or defaulted default constructor is deleted if one of these
has no default member initializer:

- a reference member,
- a `const` member of a type that is not a class,
- a `const` member of a class that has no user-provided default
  constructor and leaves a member without a value.

It is also deleted if a base or member has no default constructor that
is accessible and not deleted, or has a destructor that is deleted or
not accessible. A deleted default constructor makes only its use
invalid.
:::

:::{rule} CTOR-20
:kind: reject
:tests: classes/constructors/deleted-default-base.cpp classes/constructors/deleted-default-const-member.cpp classes/constructors/deleted-default-const-aggregate-member.cpp classes/constructors/deleted-default-member-no-default.cpp classes/constructors/deleted-default-member-private.cpp classes/constructors/deleted-default-reference.cpp classes/constructors/explicitly-defaulted-but-deleted.cpp classes/constructors/default-ctor-deleted-by-member-dtor.cpp

A program must not use a default constructor that CTOR-3 deletes.
:::

:::{rule} CTOR-4
:kind: reject
:tests: classes/constructors/const-object-defaulted-in-class.cpp classes/constructors/const-object-empty.cpp classes/constructors/const-object-member-init.cpp classes/constructors/const-object-no-init.cpp classes/constructors/const-object-user-ctor.cpp classes/constructors/const-object-base-leaves-member.cpp classes/constructors/const-object-base-initialized.cpp declarations/constexpr/variable-implicit-constructor-member-unset.cpp

A `const` object without an initializer must have a class type that
gives each member a value. One of these must be true:

- The class has a user-provided default constructor.
- Each data member has a default member initializer or is itself such a
  class.
- The class has no data members.

Each base class must meet the same condition.

A default constructor defaulted on its first declaration is not
user-provided.
:::

:::{rule} CTOR-5
:kind: required
:tests: classes/constructors/mem-init-base-and-member.cpp classes/constructors/mem-init-braces.cpp classes/constructors/mem-init-overrides-default.cpp classes/constructors/mem-init-type-alias-base.cpp classes/constructors/mem-init-uses-param.cpp

A member initializer list initializes direct bases and non-static data
members. Each initializer uses parentheses or braces. Braces use the
rules of list initialization. An initializer for a member replaces its
default member initializer.
:::

:::{rule} CTOR-21
:kind: reject
:tests: classes/constructors/mem-init-braces-narrowing.cpp

A member initializer with braces must not need a narrowing conversion.
:::

:::{rule} CTOR-15
:kind: reject
:tests: classes/constructors/mem-init-duplicate.cpp classes/constructors/mem-init-in-declaration-only.cpp classes/constructors/mem-init-indirect-base.cpp classes/constructors/mem-init-not-member.cpp classes/constructors/mem-init-pack-of-reference.cpp classes/constructors/mem-init-static-member.cpp

Each `mem_init` must name one of these:

- a non-static data member of the class,
- a direct base of the class,
- a virtual base ({rule}`DER-6`) of the class.

A name can be a type alias of a base. A `mem_init` must not:

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
constructor or conversion function ({rule}`CVF-3`) declared in its class
can have `explicit`.
:::

:::{rule} CTOR-10
:kind: required
:tests: classes/constructors/converting-ctor-multi-arg-list.cpp classes/constructors/converting-ctor.cpp classes/constructors/one-user-conversion-plus-standard.cpp

A constructor that is not `explicit` is a *converting constructor*. An
implicit conversion can use it to make an object from its arguments,
including from a braced list. Standard conversions can come before it.
:::

:::{rule} CTOR-22
:kind: reject
:tests: classes/constructors/one-user-conversion.cpp

An implicit conversion uses at most one converting constructor.
:::

:::{rule} DTOR-1
:kind: reject
:tests: classes/constructors/dtor-const.cpp classes/constructors/dtor-explicit-object.cpp classes/constructors/dtor-out-of-line.cpp classes/constructors/dtor-params.cpp classes/constructors/dtor-ref-qualified.cpp classes/constructors/dtor-return-type.cpp classes/constructors/dtor-specifiers.cpp classes/constructors/dtor-static.cpp classes/constructors/dtor-two.cpp classes/constructors/dtor-variadic.cpp classes/constructors/dtor-void-param.cpp classes/constructors/dtor-wrong-name.cpp

A destructor has:

- the name of its class after `~`,
- no parameters,
- no return type.

A parameter list of `(void)` is valid. A destructor must
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
:tests: classes/constructors/deleted-dtor-delete.cpp classes/constructors/deleted-dtor-new.cpp classes/constructors/deleted-dtor-temporary.cpp classes/constructors/deleted-dtor-variable.cpp classes/constructors/implicitly-deleted-dtor-member.cpp classes/constructors/private-dtor-member.cpp classes/constructors/private-dtor-variable.cpp classes/constructors/delete-array-deleted-dtor.cpp

If the destructor of a class is deleted or not accessible, a program
must not:

- declare a variable of the class with automatic, static, or thread
  storage duration,
- create a temporary of the class,
- apply `delete` to an object of the class.

A `new` expression for the class is valid, also for an array
({question}`Q-0082`). The implicit destructor is deleted if a base or
member has a destructor that is deleted or not accessible.
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

:::{rule} CTOR-18
:kind: reject
:tests: classes/constructors/nested-default-member-init-before-complete.cpp

A default member initializer of a nested class can only be used after the
enclosing class is complete. A default argument of a constructor of the
enclosing class must not use it, for example with empty braces (question
pending).
:::

:::{rule} CTOR-23
:kind: required
:tests: classes/constructors/trivial-default-base-nontrivial.cpp classes/constructors/trivial-default-defaulted.cpp classes/constructors/trivial-default-implicit.cpp classes/constructors/trivial-default-member-initializer.cpp classes/constructors/trivial-default-member-nontrivial.cpp classes/constructors/trivial-default-user-provided.cpp classes/constructors/trivial-default-virtual-base.cpp classes/constructors/trivial-default-virtual-function.cpp classes/constructors/trivial-default-defaulted-later.cpp

Each of these makes a default constructor non-trivial:

- The program provides its body. A function defaulted after its first
  declaration counts.
- The class has a virtual function or a virtual base.
- A member gets its value from a default member initializer.
- A base or a member of class type is built by a non-trivial one.

With none of them, the default constructor is *trivial*.
:::

:::{rule} DTOR-9
:kind: required
:tests: classes/constructors/trivial-destructor-defaulted.cpp classes/constructors/trivial-destructor-member-nontrivial.cpp classes/constructors/trivial-destructor-user-provided.cpp classes/constructors/trivial-destructor-virtual-destructor.cpp

Each of these makes a destructor non-trivial:

- The program provides its body. A function defaulted after its first
  declaration counts.
- It is virtual.
- A base or a member of class type is ended by a non-trivial one.

With none of them, the destructor is *trivial*.
:::

## Runtime Behavior

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
:tests: classes/constructors/defaulted-leaves-members.cpp classes/constructors/init-order-reads-later.cpp classes/constructors/base-init-calls-member-early-constant.cpp

If a member is read before it has a value, the behavior is undefined.
This includes a member that a later `mem_init` initializes, and a member
that default initialization leaves without a value. A call of a member
function from the `mem_init` of a base, before all bases are initialized,
is also undefined.
:::

:::{rule} DTOR-7
:kind: undefined
:tests: classes/constructors/double-destroy-constant.cpp classes/constructors/use-after-dtor-constant.cpp classes/constructors/use-after-delete-sanitizer.cpp

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

:::{rule} CTOR-17
:kind: no-diagnostic
:tests: classes/constructors/reference-member-binds-temporary.cpp classes/constructors/reference-default-member-binds-temporary.cpp

A constructor must not bind a reference member to a temporary object,
through a `mem_init` or a default member initializer. The temporary ends
with the constructor. An implementation need not diagnose it (question
pending).
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
  Buffer(const Buffer&) = delete;             // an implicit copy would share
  Buffer& operator=(const Buffer&) = delete;  // data and delete it twice
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

The results are in `tests/RESULTS.md`. All seven compilers agree on 127
of 133 tests.

- CTOR-23 and DTOR-9: the tests observe triviality through two rules
  that depend on it. A jump past a declaration is valid only for a
  trivial default constructor and destructor (JMP-6), and a union's
  special member is deleted when a member's is not trivial (UNI-3). The
  destructor tests use the union, because GCC accepts the jump for any
  destructor ({question}`Q-0041`).

- CTOR-14: Clang and EDG reject an indirect delegation cycle. GCC accepts
  it ({question}`Q-0053`).
- DTOR-7: EDG 7.0 accepts a second destruction during constant evaluation
  ({question}`Q-0054`).
- DTOR-3: Clang rejects an array `new` of a class with a deleted
  destructor. GCC and EDG accept it ({question}`Q-0082`).
- CTOR-18: EDG accepts the nested default member initializer. Clang and
  GCC reject it ({question}`Q-0083`).
- CTOR-17: Clang rejects the binding. GCC accepts it with a warning, and
  EDG accepts it ({question}`Q-0084`).

Inputs: clang-sema-declcxx, clang-sema-init, clang-diag-sema,
edg-class-decl, edg-decl-inits.
