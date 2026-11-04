# Constructors and destructors

A constructor initializes an object of a class. A destructor ends the
life of an object of a class. This page covers:

- how to declare constructors and destructors,
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
:rules: CTOR-2 CTOR-3 CTOR-4 CTOR-24 DTOR-2 DTOR-3 DTOR-4 CTOR-19 CTOR-20 CTOR-23 DTOR-9

The implementation writes a default constructor and a destructor for a
class that writes none, because building and ending the parts is right
for most classes. Declaring any constructor says that default
construction needs thought, so the declaration removes the implicit
default constructor. When a part cannot be default-constructed or
destroyed, the synthesized function is deleted and only a use fails.
Building several parts needs the destructor of each part, because the
parts already built are destroyed when a later part fails. This holds
for the elements of an array `new` too. A
`const` object must state its value where the object is defined,
because nothing can change the object later. So a `const` object with
no initializer needs a class that gives every part a value. The zero
that static storage gets first ({rule}`STO-3`) does not count, so
CTOR-4 holds at namespace scope too. The `noexcept`
of a destructor follows from the parts of the class. A synthesized
function that does nothing beyond the parts' own trivial work is
*trivial*, and some rules permit more for a trivial function, such as a
jump past a declaration (JMP-6).
:::

:::{principle} CTOR-P3
:rules: CTOR-5 CTOR-6 CTOR-11 CTOR-12 CTOR-13 CTOR-18 CTOR-21

A constructor builds the parts first and then runs its body. The parts
are built in declaration order, bases before members, whatever order the
member initializer list has, because the destructor must undo them in
one fixed order. An implementation should warn when the list suggests
another order. Until the constructor completes, the object is not yet
`const`, and a part has no value until its turn, so reading the part
earlier is undefined. A nested class's default member initializer is
part of the enclosing class, so the initializer is usable only once the
enclosing class is complete.
:::

:::{principle} CTOR-P6
:rules: CTOR-7 CTOR-8 CTOR-14 CTOR-15 CTOR-16 CTOR-17

A member initializer names one part of the object and gives that part
its value. A member initializer cannot name anything else, and a
reference member that the initializer binds to a temporary is dead
before the body runs, so the implementation rejects such a binding. A delegating constructor hands the whole job to
one other constructor and adds only its own body, so its list names the
target constructor and nothing else. A delegating constructor cannot
hand the job back to itself, directly or through a cycle, and an
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

After the destructor runs, the object is gone, and using or destroying
the object again is undefined ({principle}`OBJ-P4`).
:::

:::{principle} CTOR-P5
:rules: CTOR-9 CTOR-10 CTOR-22 CTOR-25

An initialization of a class object calls a constructor with the
initializer's arguments, as a function call passes arguments to a
function. A constructor that takes one argument is a conversion from
that argument type, and an implicit conversion can use the constructor.
`explicit` withdraws that offer, so the constructor then runs only where
the program names the type.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   ctor_decl: {`ctor_specifier`} `class_name` "(" `parameters` ")" [`noexcept_specifier`]
   ctor_specifier: "constexpr" | "consteval" | "inline" | `explicit_spec`
   explicit_spec: "explicit" ["(" `constant_expr` ")"]
   ctor_def: `ctor_decl` [`ctor_init`] `function_body`
   ctor_init: ":" `mem_init` ["..."] {"," `mem_init` ["..."]}
   mem_init: `mem_init_id` ("(" [`expression_list`] ")" | `braced_init`)
   mem_init_id: `identifier` | [`nested_name`] `class_name`
   dtor_decl: {`dtor_specifier`} "~" `class_name` "(" ["void"] ")" [`noexcept_specifier`]
   dtor_specifier: "constexpr" | "inline" | "virtual"
   dtor_call: `postfix_expr` ("." | "->") [`nested_name`] "~" `identifier` "(" ")"
```

A `mem_init` with `...` expands a pack of bases ({rule}`PACK-14`).

A *member initializer list* is the list of `mem_init` after the colon. A
*delegating constructor* has a `mem_init` that names its own class. A
{term}`default constructor` is a constructor that a call with no
arguments can call. Its parameters all have default arguments, or it
has none.

## Constraints

:::{rule} CTOR-1
:kind: reject
:tests: classes/constructors/ctor-address.cpp classes/constructors/ctor-const-qualified.cpp classes/constructors/ctor-explicit-object.cpp classes/constructors/ctor-out-of-line.cpp classes/constructors/ctor-redeclared.cpp classes/constructors/ctor-ref-qualified.cpp classes/constructors/ctor-return-type.cpp classes/constructors/ctor-specifiers.cpp classes/constructors/ctor-static.cpp classes/constructors/ctor-virtual.cpp classes/constructors/ctor-volatile.cpp

A constructor has the name of its class and no return type. A constructor
must not:

- be `static` or `virtual`,
- have a {term}`cv-qualifier` or a ref-qualifier,
- have an explicit object parameter,
- be declared two times in its class.

A program cannot take the address of a constructor.
:::

:::{rule} CTOR-2
:kind: required
:tests: classes/constructors/defaulted-default-restores.cpp classes/constructors/implicit-default.cpp classes/constructors/inherited-ctors-keep-default.cpp classes/constructors/default-constructor-default-arguments.cpp classes/derived/inherited-ctor-keeps-implicit.cpp

If a class declares no constructor, it has an implicit public default
constructor. A declaration of any other constructor, including a copy
constructor, removes the implicit one. `= default` declares the default
constructor again. A using-declaration that inherits constructors does
not declare a constructor, so the implicit default constructor stays.
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

The default constructor is also deleted if a base or member has no
default constructor that is accessible and not deleted, or has a
destructor that is deleted or not accessible. A deleted default
constructor makes only a use of the constructor not valid.
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

:::{rule} CTOR-24
:kind: reject
:tests: classes/constructors/const-object-union-no-init.cpp classes/constructors/const-object-union-member-init.cpp classes/constructors/const-object-anonymous-union-init.cpp classes/constructors/const-object-anonymous-union-no-init.cpp classes/constructors/const-object-only-anonymous-unions.cpp

For CTOR-4, a union gives its members a value if the union declares a
default member initializer for at least one of its members. An
anonymous union member of a class meets the condition of CTOR-4 in the
same way, so a class whose data members are all in anonymous unions
without default member initializers does not meet the condition
({question}`Q-0126`).
:::

:::{rule} CTOR-5
:kind: required
:tests: classes/constructors/mem-init-base-and-member.cpp classes/constructors/mem-init-braces.cpp classes/constructors/mem-init-overrides-default.cpp classes/constructors/mem-init-type-alias-base.cpp classes/constructors/mem-init-uses-param.cpp

A member initializer list initializes direct bases and non-static data
members. Each initializer uses parentheses or braces. Braces use the
rules of list initialization. An initializer for a member replaces the
default member initializer of that member. Names inside the parentheses
or braces are looked up as in the constructor body, so a parameter hides
a member with the same name.
:::

:::{rule} CTOR-21
:kind: reject
:tests: classes/constructors/mem-init-braces-narrowing.cpp classes/constructors/mem-init-braces-narrowing-array.cpp

A member initializer with braces must not need a narrowing conversion.
:::

:::{rule} CTOR-15
:kind: reject
:tests: classes/constructors/mem-init-duplicate.cpp classes/constructors/mem-init-in-declaration-only.cpp classes/constructors/mem-init-indirect-base.cpp classes/constructors/mem-init-not-member.cpp classes/constructors/mem-init-pack-of-reference.cpp classes/constructors/mem-init-static-member.cpp

Each `mem_init` must name one of these:

- a non-static data member of the class,
- a direct base of the class,
- a {term}`virtual base` ({rule}`DER-6`) of the class.

A name can be a type alias of a base. A `mem_init` must not:

- name the same member or base two times,
- occur in a constructor declaration that is not a definition,
- value-initialize a reference member.
:::

:::{rule} CTOR-8
:kind: reject
:tests: classes/constructors/delegating-self.cpp classes/constructors/delegating-self-with-argument.cpp

A constructor must not delegate to itself.
:::

:::{rule} CTOR-16
:kind: reject
:tests: classes/constructors/delegating-with-member.cpp classes/constructors/delegating-with-base.cpp

The `mem_init` of a delegating constructor must be the only one in its
list.
:::

:::{rule} CTOR-9
:kind: reject
:tests: classes/constructors/explicit-argument.cpp classes/constructors/explicit-bool-false.cpp classes/constructors/explicit-bool-not-constant.cpp classes/constructors/explicit-bool-true.cpp classes/constructors/explicit-copy-init.cpp classes/constructors/explicit-copy-list-init.cpp classes/constructors/explicit-default-copy-list.cpp classes/constructors/explicit-direct-init.cpp classes/constructors/explicit-non-ctor.cpp classes/constructors/explicit-out-of-line.cpp classes/constructors/explicit-return.cpp classes/constructors/explicit-static-cast.cpp

An `explicit` constructor is not used in {term}`copy-initialization`.
Copy-initialization includes:

- `T x = v;` and `T x = {v};`,
- passing an argument,
- returning a value, including `return {};`.

Direct initialization and casts can use an `explicit` constructor.
`explicit(c)` makes the constructor `explicit` if the constant
expression `c` is `true`. Only a constructor or conversion function
({rule}`CVF-3`) declared in its class can have `explicit`.
{rule}`OVL-14` lists the candidates of each form of initialization.
:::

:::{rule} CTOR-10
:kind: required
:tests: classes/constructors/converting-ctor-multi-arg-list.cpp classes/constructors/converting-ctor.cpp classes/constructors/one-user-conversion-plus-standard.cpp classes/constructors/return-owner-converts-to-base-owner.cpp

A constructor that is not `explicit` is a *{term}`converting constructor`*. An
implicit conversion can use a converting constructor to make an object
from the constructor's arguments, including from a braced list. Standard
conversions can come before the converting constructor. {rule}`ICS-3`
gives the parts of such a conversion.
:::

:::{rule} CTOR-22
:kind: reject
:tests: classes/constructors/one-user-conversion.cpp classes/constructors/one-user-conversion-copy-init.cpp

An implicit conversion uses at most one converting constructor.
{rule}`ICS-9` gives the same limit for a conversion sequence.
:::

:::{rule} CTOR-25
:kind: reject
:tests: classes/constructors/ctor-no-match-direct.cpp classes/constructors/ctor-no-match-braces.cpp classes/constructors/ctor-no-match-functional-cast.cpp classes/constructors/ctor-no-match-mem-init.cpp classes/constructors/ctor-no-match-new.cpp foundations/names/adl-suppressed-by-type.cpp classes/constructors/ctor-no-match-list-ctor-not-viable.cpp classes/constructors/ctor-no-match-other-class.cpp classes/constructors/ctor-no-match-nested-braces.cpp classes/constructors/ctor-implicit-copy-move.cpp classes/constructors/ctor-ambiguous-braces.cpp classes/constructors/ctor-ambiguous-copy-list.cpp classes/constructors/ctor-ambiguous-argument-braces.cpp foundations/conversion-sequences/chosen-list-ambiguous.cpp classes/derived/inherited-nothing-not-aggregate.cpp

An initialization of a class object by a constructor must find one best
constructor for its arguments ({rule}`OVL-34`). A constructor accepts
the arguments if these conditions are true:

- The number of arguments fits its parameters ({rule}`PFX-5`).
- Each argument converts implicitly to its parameter type.

The candidates include the implicit copy and move constructors. In list
initialization, {rule}`ILC-2` and {rule}`ILC-3` first try the
initializer-list constructors. If those rules choose none, the arguments are
the elements of the list.

A call whose name finds a class, such as `probe(x)`, initializes an
object of that class. {term}`Argument-dependent lookup <argument-dependent lookup>` does not run for such a
call ({rule}`NAM-32`).

The program is not valid in each of these cases:

- No constructor accepts the arguments ({rule}`OVL-35`).
- More than one constructor accepts the arguments, and none is best
  ({rule}`OVL-36`).
- A braced list initializes a parameter of class type, and no
  constructor is best for the list. {rule}`ICS-19` gives the ambiguous
  sequence, and {rule}`ICS-8` rejects the chosen function.

CTOR-25 is a specific case of {rule}`OVL-35` and {rule}`OVL-36`. These
rules name specific cases of CTOR-25 (decision 0042):

- {rule}`CTOR-19`, an initialization that needs a default constructor
  that the class does not have,
- {rule}`CTOR-9`, an `explicit` constructor in copy-initialization,
- {rule}`CTOR-22`, a conversion through two converting constructors.
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
:tests: classes/constructors/dtor-on-const-object.cpp classes/constructors/dtor-implicit-const-block.cpp

If a class declares no destructor, it has an implicit public destructor.
A destructor can run on a `const` object.
:::

:::{rule} DTOR-3
:kind: reject
:tests: classes/constructors/deleted-dtor-delete.cpp classes/constructors/deleted-dtor-new.cpp classes/constructors/deleted-dtor-temporary.cpp classes/constructors/deleted-dtor-return-prvalue.cpp classes/constructors/deleted-dtor-copy-init-prvalue.cpp classes/constructors/deleted-dtor-new-from-prvalue.cpp classes/constructors/deleted-dtor-decltype-call.cpp classes/constructors/deleted-dtor-variable.cpp classes/constructors/implicitly-deleted-dtor-member.cpp classes/constructors/private-dtor-member.cpp classes/constructors/private-dtor-variable.cpp classes/constructors/delete-array-deleted-dtor.cpp classes/constructors/deleted-dtor-sizeof.cpp classes/constructors/deleted-dtor-sizeof-call.cpp classes/constructors/deleted-dtor-noexcept.cpp classes/constructors/deleted-dtor-decltype-braces.cpp classes/constructors/deleted-dtor-decltype-call-argument.cpp classes/constructors/deleted-dtor-decltype-comma.cpp classes/constructors/deleted-dtor-decltype-comma-left.cpp classes/constructors/private-dtor-sizeof.cpp classes/constructors/deleted-dtor-new-aggregate-braces.cpp classes/constructors/default-ctor-deleted-by-member-dtor.cpp classes/constructors/deleted-dtor-variant-member.cpp classes/copy-move/trivially-copyable-deleted-destructor.cpp

If the destructor of a class is deleted or not accessible, a program
must not:

- declare a variable of the class with automatic, static, or thread
  {term}`storage duration`,
- write a prvalue of the class, also where the prvalue initializes an
  object directly, as in `return S{};`,
- apply `delete` to an object of the class,
- create an array of the class with `new` ({question}`Q-0082`).

The prvalue rule also covers the operands of `sizeof`, `noexcept`, and
`typeid`. The one exception is a function call that is the operand of
`decltype`, also in parentheses or as the right operand of a comma. A
prvalue of the class as the left operand of the comma is not exempt.

A `new` expression for one object of the class is valid. An array
`new` destroys the elements that it has built when the initialization
of a later element throws, so the array form needs the destructor. The
initializer of a `new` expression must not be a prvalue of the
class, as in `new S(make())`. Braces that initialize the members, as in
`new S{1}`, are valid.

The implicit destructor is deleted if a base or member has a destructor
that is deleted or not accessible. A variant member of a union
({doc}`unions`) counts as a member here. The `new` exception covers only the
destructor of the class. {rule}`CTOR-3` still deletes a default
constructor that `new S` needs.
:::

:::{rule} DTOR-4
:kind: required
:tests: classes/constructors/dtor-member-noexcept-false.cpp classes/constructors/dtor-noexcept-false.cpp classes/constructors/implicit-dtor-noexcept.cpp classes/constructors/user-dtor-noexcept.cpp exceptions/unwinding/leaves-destructor-member-throwing-caught.cpp

A destructor without a `noexcept` specifier is `noexcept` if the
destructors of all bases and members are `noexcept`. The rule applies to
implicit and user-declared destructors.
:::

:::{rule} DTOR-8
:kind: reject
:tests: classes/constructors/dtor-address.cpp classes/constructors/dtor-call-wrong-type.cpp classes/constructors/dtor-without-call.cpp classes/constructors/pseudo-dtor-wrong-type.cpp

A program must call a destructor with `()` and cannot take the address
of a destructor. In an explicit destructor call, the name after `~` must
denote the type of the object, also for a {term}`scalar type`.
:::

:::{rule} CTOR-18
:kind: reject
:tests: classes/constructors/nested-default-member-init-before-complete.cpp classes/constructors/nested-default-member-init-read-in-default-argument.cpp

A default member initializer of a nested class can only be used after
the enclosing class is complete. A default argument of a constructor of
the enclosing class must not use that initializer, for example with
empty braces ({question}`Q-0083`).
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

With none of these conditions, the default constructor is *trivial*.
:::

:::{rule} DTOR-9
:kind: required
:tests: classes/constructors/trivial-destructor-defaulted.cpp classes/constructors/trivial-destructor-member-nontrivial.cpp classes/constructors/trivial-destructor-user-provided.cpp classes/constructors/trivial-destructor-virtual-destructor.cpp

Each of these makes a destructor non-trivial:

- The program provides its body. A function defaulted after its first
  declaration counts.
- The destructor is virtual.
- A base or a member of class type is ended by a non-trivial one.

With none of these conditions, the destructor is *trivial*.
:::

:::{rule} CTOR-17
:kind: reject
:tests: classes/constructors/reference-member-binds-temporary.cpp classes/constructors/reference-default-member-binds-temporary.cpp classes/constructors/reference-member-binds-converted-parameter.cpp classes/constructors/initializer-list-default-member-from-braces.cpp classes/constructors/initializer-list-member-from-braces.cpp

A constructor must not bind a reference member to a {term}`temporary object`,
through a `mem_init` or a default member initializer. The temporary ends
with the constructor. The binding is visible in the constructor or the
class definition, so an implementation rejects it ({question}`Q-0084`).

The same holds for a member of type `std::initializer_list<E>` *(library
referenced)* that a `mem_init` or a default member initializer sets
from a braced list. The array that holds the elements is a temporary
bound as if to a reference member.
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
Then the body of the delegating constructor executes. Default member
initializers apply only through the target constructor.
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

1. the destructor body executes,
2. the data members are destroyed, last declared first,
3. the direct bases are destroyed, last listed first.

{rule}`OBJ-3` gives the order of the variables of a block. Array
elements are destroyed from the last element to the first, also for
`delete[]`.
:::

:::{rule} DTOR-6
:kind: required
:tests: classes/constructors/dtor-call-via-alias.cpp classes/constructors/explicit-dtor-call.cpp classes/constructors/pseudo-dtor-scalar.cpp

An explicit destructor call runs the destructor and ends the life of the
object. The storage of the object is not released. The name after `~`
can be a type alias. A program can also call a destructor on an object
of scalar type through a type name.
:::

## Undefined Behavior

:::{rule} CTOR-12
:kind: undefined
:tests: classes/constructors/defaulted-leaves-members.cpp classes/constructors/base-init-calls-member-through-pointer-sanitizer.cpp classes/constructors/base-init-calls-member-of-member-constant.cpp classes/constructors/own-member-call-in-own-constructor.cpp classes/constructors/init-order-reads-later.cpp classes/constructors/base-init-calls-member-early-constant.cpp classes/constructors/init-order-reads-later-run.cpp classes/constructors/base-init-calls-member-early-sanitizer.cpp classes/constructors/base-init-calls-member-indirectly-sanitizer.cpp classes/constructors/base-init-dynamic-cast-early-constant.cpp

If a member is read before it has a value, the behavior is undefined.
This includes a member that a later `mem_init` initializes, and a member
that {term}`default initialization` leaves without a value. These calls
of a non-static member function are also undefined:

- a call on the object under construction, through `this` or a pointer
  or reference to it, from the `mem_init` of a base before all bases
  are initialized, also a call that a function called from there makes,
- a call on a member subobject before its initialization, which is a
  use outside its lifetime ({rule}`OBJ-2`,
  {question}`Q-0374`).

The same holds when the object under construction, before all bases
are initialized, is the operand of `typeid` or of a `dynamic_cast`.

A base's own constructor can call the member functions of that base.
:::

:::{rule} DTOR-7
:kind: undefined
:tests: classes/constructors/double-destroy-constant.cpp foundations/storage/member-call-after-end-constant.cpp classes/constructors/double-destroy-sanitizer.cpp classes/constructors/double-destroy-trivial-constant.cpp classes/constructors/parameter-destroyed-by-callee-constant.cpp

If a program destroys an object a second time, or calls a member
function of the object, after its destructor runs, the behavior is
undefined. The rule holds for a destructor of any kind, trivial or
not. A second destruction happens, for example, in these cases:

- at the end of the block after an explicit destructor call
  ({question}`Q-0054`),
- when the caller ends a parameter object that the called function
  already destroyed ({rule}`VC-10`).

These rules cover the other uses:

- {rule}`OBJ-2` for a read or write of the object,
- {rule}`NEW-5` for a second `delete` of a dynamic object.
:::

## Implementation Requirements

:::{rule} CTOR-14
:kind: no-diagnostic
:tests: classes/constructors/delegating-cycle.cpp classes/constructors/delegating-cycle-three.cpp

Constructors must not delegate to each other in a cycle. An
implementation need not diagnose a cycle that does not delegate directly
to itself ({question}`Q-0053`).
:::

## Recommended Practice

:::{rule} CTOR-13
:kind: recommended
:tests: classes/constructors/init-order-same-order.cpp classes/constructors/init-order-warns.cpp classes/constructors/init-order-no-warning-default.cpp

An implementation should warn if the member initializer list has an
order different from the order of initialization.
:::

## Examples

% tidal-example: reject
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

struct Range {
  int low, high;
  Range(int low, int high) : low(low), high(high + 1) {}
};                               // low(low): the member from the parameter
```

A deleted destructor permits `new`, but not every initializer of it
(DTOR-3):

% tidal-example: reject
```cpp
struct Pinned { int v; ~Pinned() = delete; };
Pinned make();
Pinned* a = new Pinned{1};       // valid: the braces initialize v
Pinned* b = new Pinned(make());  // rejected: a prvalue of Pinned

struct Holder { Pinned p; };
Holder* h = new Holder;          // rejected: CTOR-3 deletes Holder()
```

A braced element of one value can reach a constructor in two ways. The
way without a user-defined conversion wins (CTOR-25):

```cpp
struct Span { Span(int); };
Span g{{1}};   // Span(int): {1} gives an int directly (ICS-15). The
               // copy constructor would need Span(int) first (ICS-21)
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 173
of 194 tests.

Two questions on this page settled on 2026-11-03 under decision 0054 after the source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`). For each, the review supports the answer with no doubt, and a principle gives the same answer. Each test keeps its override for the family that still differs:

| Question | Principle | Family that still differs |
|---|---|---|
| {question}`Q-0053` | CTOR-P6 | none; CTOR-14 needs no diagnostic |
| {question}`Q-0054` | CTOR-P4 | EDG 7.0 |

- CTOR-23 and DTOR-9: the tests observe triviality through JMP-6 and
  UNI-3. A jump past a declaration is valid only for a trivial default
  constructor (JMP-6), and a union's special member is deleted when a
  member's is not trivial (UNI-3). The destructor tests use the union,
  because JMP-6 does not look at the destructor ({question}`Q-0041`).

- CTOR-14: Clang and EDG reject an indirect delegation cycle. GCC accepts
  it ({question}`Q-0053`).
- DTOR-7: EDG 7.0 accepts a second destruction during constant evaluation
  ({question}`Q-0054`). Clang and GCC reject it for a trivial destructor
  too, and for a parameter object that the called function destroyed
  before the caller ends it.
- DTOR-3: Clang rejects an array `new` of a class with a deleted
  destructor. GCC and EDG accept it ({question}`Q-0082`). The rule
  follows Clang and the source-reader gap review of the open questions
  on classes and statements
  (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`),
  and the test records GCC and EDG as overrides.
- CTOR-18: EDG accepts the nested default member initializer. Clang and
  GCC reject it ({question}`Q-0083`).
- CTOR-17: Clang rejects the binding. GCC accepts it with a warning, and
  EDG accepts it ({question}`Q-0084`). CTOR-17 is a `reject` rule,
  because the binding is visible where the constructor or the class is
  defined. The tests record GCC and EDG as overrides. The gap review of
  the rules that phase 7 added
  (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) reads
  the reference as treating an `std::initializer_list` member set from
  a braced list in the same way. The compilers split in the same way:
  Clang rejects both forms, and GCC and EDG accept them.
- CTOR-24: the compilers differ for unions and anonymous unions
  ({question}`Q-0126`):

  | Case | CTOR-24 and the test | Clang 22 | Clang 18 | GCC | EDG |
  |---|---|---|---|---|---|
  | Union, one variant member with a default member initializer | accepts | accepts | rejects | rejects | accepts |
  | Anonymous union with such a member, beside another member | accepts | accepts | rejects | accepts | accepts |
  | Anonymous union without one, beside another member | rejects | rejects | rejects | accepts | rejects |
  | Only anonymous unions, none with a default member initializer | rejects | rejects | rejects | accepts | accepts |

  The second column gives the provisional answer of {question}`Q-0126`,
  which each test expects. Each test records the compilers that differ
  from that answer as overrides.
- DTOR-3: all compilers reject each prvalue of the class, also where no
  temporary is made, as in `return S{};`. All reject the prvalue in the
  operand of `sizeof`, `noexcept`, and `typeid`, and in `decltype(S{})`.
  All accept a function call as the operand of `decltype`, also as the
  right operand of a comma. All reject the call as the left operand of
  the comma.
- CTOR-25: all compilers reject a direct initialization, a functional
  cast, a member initializer, and a `new` initializer whose arguments
  fit no constructor. In list initialization, all compilers fall back to
  the elements when no initializer-list constructor is viable. All
  accept an element of the class, or of a class that converts to it,
  through the implicit copy or move constructor. All reject a braced
  list for which two constructors tie, also as the argument of another
  constructor.
- CTOR-13: Clang and GCC give the order warning under `-Wall` and not
  by default.
- CTOR-12: no sanitizer of GCC or Clang stops a read of a member that
  has no value yet, under `-fsanitize=undefined` or `-fsanitize=address`.
  The front end's interpreter stops the read and names OBJ-2
  (`init-order-reads-later-run.cpp`). The `vptr` sanitizer of GCC stops
  a call of a member function from the `mem_init` of a base, and the
  Clang sanitizer does not (`base-init-calls-member-early-sanitizer.cpp`).
  The same holds for a call through a pointer to the object under
  construction, and for a call that a function called from the
  `mem_init` makes (`base-init-calls-member-indirectly-sanitizer.cpp`).
  Every compiler rejects a constant evaluation that uses the object as
  the operand of a `dynamic_cast` before its base is initialized. EDG
  also rejects the `dynamic_cast` after the base is initialized. At run
  time neither `vptr` sanitizer reports the `dynamic_cast`. The GCC
  build crashes, and the Clang build runs to the end.
  In constant evaluation, no compiler rejects the early
  call of a member function that reads no member. For a call on a
  member subobject that is not yet initialized, Clang rejects the
  constant evaluation, and GCC and EDG accept it
  ({question}`Q-0374`).
- DTOR-7: the address sanitizer of GCC and Clang stops a second
  destruction whose destructor frees storage. The `vptr` sanitizer of
  GCC stops a member call after the destructor, and the Clang
  sanitizer does not.

Inputs: clang-sema-declcxx, clang-sema-init, clang-diag-sema,
edg-class-decl, edg-decl-inits.
