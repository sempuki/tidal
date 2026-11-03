# Virtual functions and abstract classes

A virtual function is a member function that a derived class can
override. A call through a pointer or reference to a base calls the
override for the class that the object was created as. An abstract class
has a pure virtual function, and a program cannot create an object of an
abstract class.
This page covers:

- declaring and overriding virtual functions,
- `override` and `final`,
- return types of overrides,
- calls during construction and destruction,
- virtual destructors,
- abstract classes,
- `dynamic_cast`.

Depends on: derived classes, constructors and destructors, access
control, `new` and `delete`.

## Principles

:::{principle} VIRT-P1
:rules: VIRT-1 VIRT-5 VIRT-11 VIRT-20

A virtual call goes to the function for the class the object was created
as, found through a pointer or reference to any base. A function in a
derived class overrides a base function when their signatures match,
with or without the keyword, and each virtual function must have exactly
one such final overrider in the complete class. The call is resolved at
run time, so every virtual function needs a definition, called or not.
:::

:::{principle} VIRT-P2
:rules: VIRT-2 VIRT-3 VIRT-4 VIRT-14 VIRT-15 VIRT-19 VIRT-21

An override replaces a function, so the override must fit where the
original stood:

- declared as a member in a class,
- with the same return type or a pointer or reference to a more derived
  class,
- with the same `consteval` and deleted status.

`override` and `final` let a class say what it means, and the
implementation holds the class to the stated meaning.
:::

:::{principle} VIRT-P3
:rules: VIRT-6 VIRT-7 VIRT-12 VIRT-13

While a constructor or destructor of class `C` runs, the object is a `C`
and nothing more ({principle}`OBJ-P1`), so a virtual call there reaches
the overrider for `C`, and a pure virtual function has nothing to reach.
For a class `D` derived from `B`:

```text
B constructor runs     the object is a B   calls reach B's overriders
D constructor runs     the object is a D   calls reach D's overriders
  ...
D destructor runs      the object is a D   calls reach D's overriders
B destructor runs      the object is a B   calls reach B's overriders
```

A virtual destructor lets `delete` through a base pointer find the whole
object. Without a virtual destructor, such a `delete` is undefined.
:::

:::{principle} VIRT-P4
:rules: VIRT-8 VIRT-9 VIRT-16 VIRT-17

A pure virtual function declares an obligation for derived classes. A
class with a pure virtual function is abstract and cannot be an object
on its own, only a base of a class that meets the obligation. A pure
function can still have a definition, and a pure virtual destructor
needs a definition, because derived destructors call the pure virtual
destructor.
:::

:::{principle} VIRT-P5
:rules: VIRT-10 VIRT-18

`dynamic_cast` checks at run time whether the object is of the requested
class, and gives a null pointer when the object is not. The check needs
a polymorphic class, except for the harmless direction toward a base.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   virtual_decl: "virtual" [`decl_specifiers`] `declarator`
               :    [`virt_specifiers`] [`pure_specifier`] ";"
   virt_specifiers: `virt_specifier` [`virt_specifier`]
   virt_specifier: "override" | "final"
   pure_specifier: "=" "0"
   dynamic_cast: "dynamic_cast" "<" `type_id` ">" "(" `expression` ")"
```

`virt_specifiers` and `pure_specifier` come after the declarator of a
member function. `override` and `final` are identifiers with a special
meaning only in that position. A *polymorphic class* declares or inherits
a virtual function.

## Constraints

:::{rule} VIRT-2
:kind: reject
:tests: classes/virtual/explicit-object-virtual.cpp classes/virtual/virtual-auto-return.cpp classes/virtual/virtual-data-member.cpp classes/virtual/virtual-in-union.cpp classes/virtual/virtual-nonmember.cpp classes/virtual/virtual-out-of-class.cpp classes/virtual/virtual-static.cpp classes/virtual/virtual-template.cpp classes/virtual/virtual-trailing-return.cpp

`virtual` can only occur on a declaration of a non-static member function
in its class definition. A program must not declare these as virtual:

- a static member function,
- a function outside a class, or a definition outside its class,
- a member function template,
- a function with a deduced return type,
- a member of a union,
- a member function with an explicit object parameter.

A trailing return type is valid. {rule}`VIRT-21` gives the limit on a
static member function in a derived class.
:::

:::{rule} VIRT-21
:kind: reject
:tests: classes/virtual/static-overrides-virtual.cpp

A static member function must not have the name and parameters of a
virtual function of a base. The rule applies also when the static
function does not use the keyword `virtual`.
:::

:::{rule} VIRT-3
:kind: reject
:tests: classes/virtual/final-different-params-ok.cpp classes/virtual/final-function.cpp classes/virtual/final-nonvirtual.cpp classes/virtual/final-on-override.cpp classes/virtual/override-final-order.cpp classes/virtual/override-identifier.cpp classes/virtual/override-nonvirtual.cpp classes/virtual/override-nothing.cpp classes/virtual/override-out-of-class.cpp classes/virtual/ref-qualifier-must-match.cpp

A function marked `override` must override a virtual function of a base.
A function marked `final` must be virtual, and a derived class must not
override it. `override` and `final` can occur in either order. The two
specifiers occur only on the declaration in the class.
:::

:::{rule} VIRT-14
:kind: reject
:tests: classes/virtual/consteval-overrides-virtual.cpp classes/virtual/consteval-virtual-pair.cpp

A `consteval` function and a function that is not `consteval` must not
override each other.
:::

:::{rule} VIRT-4
:kind: required
:tests: classes/virtual/covariant-less-cv-ok.cpp classes/virtual/covariant-own-class.cpp classes/virtual/covariant-pointer.cpp classes/virtual/covariant-private-base.cpp classes/virtual/covariant-reference.cpp classes/virtual/covariant-through-base-pointer.cpp

An override has the same return type as the function that it overrides,
or a *covariant* return type. A covariant return type is a pointer or
reference to a class `D` where the overridden function returns a pointer
or reference to a class `B`, with these conditions:

- `B` is `D` or an unambiguous base of `D`,
- `B` is accessible in the class of the override,
- the target of the new type has the same or fewer cv-qualifiers.

A call through the base converts the result to the type of the
overridden function.
:::

:::{rule} VIRT-15
:kind: reject
:tests: classes/virtual/covariant-ambiguous-base.cpp classes/virtual/covariant-by-value-rejected.cpp classes/virtual/covariant-incomplete.cpp classes/virtual/covariant-more-cv-rejected.cpp classes/virtual/covariant-private-base-other.cpp classes/virtual/covariant-unrelated.cpp classes/virtual/return-type-mismatch.cpp

An override must not have a return type that is different and not
covariant ({rule}`VIRT-4`). A return by value of a derived class is not covariant. The
class in a covariant pointer or reference must be complete at the
declaration of the override, unless that class declares the override.
:::

:::{rule} VIRT-5
:kind: reject
:tests: classes/virtual/no-unique-final-overrider.cpp classes/virtual/nonvirtual-base-two-overriders.cpp classes/virtual/unique-final-overrider-resolved.cpp

Each virtual function of a class must have one *final overrider*, the
override that a call selects. If two classes override the same function
of a shared virtual base, and neither overrides the other, the class
that derives from both must override the function too. With a
non-virtual base on two paths, each subobject has its own final
overrider.
:::

:::{rule} VIRT-8
:kind: reject
:tests: classes/virtual/abstract-array.cpp classes/virtual/abstract-capture-by-reference.cpp classes/virtual/abstract-capture-by-value.cpp classes/virtual/abstract-cast-temporary.cpp classes/virtual/abstract-function-declaration-ok.cpp classes/virtual/abstract-member.cpp classes/virtual/abstract-new.cpp classes/virtual/abstract-param-definition.cpp classes/virtual/abstract-pointer-ref-ok.cpp classes/virtual/abstract-return-definition.cpp classes/virtual/abstract-sizeof.cpp classes/virtual/abstract-variable.cpp classes/virtual/call-returning-abstract.cpp classes/virtual/derived-concrete.cpp classes/virtual/derived-still-abstract.cpp classes/virtual/pure-override-makes-abstract.cpp

A class with at least one pure virtual function, declared or inherited
without a final overrider, is *abstract*. A program must not create an
object of an abstract class. This rule covers:

- a variable, an array, a temporary, or a member,
- a `new` expression,
- a parameter or a return value in a function definition,
- a call to a function that returns the class ({question}`Q-0086`),
- a lambda capture by copy.

Pointers and references to an abstract class are valid. A function
declaration that is not a definition can use the class as a parameter or
return type. An override can make a function pure again.
:::

:::{rule} VIRT-9
:kind: reject
:tests: classes/virtual/pure-friend.cpp classes/virtual/pure-inline-definition.cpp classes/virtual/pure-nonvirtual.cpp classes/virtual/pure-not-zero.cpp

A pure specifier is `= 0` on a virtual function. A pure specifier must
not occur on:

- a function that is not virtual,
- a friend,
- a function defined in the class.
:::

:::{rule} VIRT-16
:kind: required
:tests: classes/virtual/pure-defined-out-of-line.cpp classes/virtual/pure-dtor-defined.cpp

A pure virtual function can have a definition outside its class. A
qualified call calls that definition. A derived class calls a pure
virtual destructor of its base, so that destructor needs a definition.
:::

:::{rule} VIRT-18
:kind: reject
:tests: classes/virtual/dynamic-cast-ambiguous-base.cpp classes/virtual/dynamic-cast-casts-away-const.cpp classes/virtual/dynamic-cast-from-nonpointer.cpp classes/virtual/dynamic-cast-inaccessible-base.cpp classes/virtual/dynamic-cast-incomplete.cpp classes/virtual/dynamic-cast-nonpolymorphic.cpp classes/virtual/dynamic-cast-not-pointer.cpp classes/virtual/dynamic-cast-to-nonclass.cpp classes/virtual/dynamic-cast-upcast-nonpolymorphic.cpp

A `dynamic_cast` must meet these conditions:

- The target is `void*`, a class pointer, or a class reference. The
  class must be complete.
- A pointer target needs a pointer operand. A reference target needs an
  lvalue or xvalue operand.
- The target keeps every cv-qualifier of the operand.
- A cast that does not go to a base needs an operand of polymorphic
  class type, through a pointer or reference.
- A cast that goes to a base needs a base that is accessible and
  unambiguous.
:::

:::{rule} VIRT-19
:kind: reject
:tests: classes/virtual/explicit-object-overrides.cpp classes/virtual/override-both-deleted.cpp classes/virtual/override-deleted-mismatch.cpp classes/virtual/override-of-deleted.cpp

An override and the function that it overrides must both be deleted or
both be not deleted. A member function with an explicit object parameter
must not have the name and parameters of a virtual function of a base
({question}`Q-0087`).
:::

## Runtime Behavior

:::{rule} VIRT-1
:kind: required
:tests: classes/virtual/const-qualifier-must-match.cpp classes/virtual/default-args-static-type.cpp classes/virtual/different-params-no-override.cpp classes/virtual/dispatch.cpp classes/virtual/object-call-no-dispatch.cpp classes/virtual/override-through-intermediate.cpp classes/virtual/override-without-virtual-keyword.cpp classes/virtual/private-override.cpp classes/virtual/qualified-call-no-dispatch.cpp classes/virtual/virtual-in-constexpr.cpp classes/virtual/virtual-call-override-not-constexpr.cpp classes/virtual/pure-virtual-constexpr-order.cpp

A member function overrides a virtual function of a base if the member
function has the same:

- name,
- parameter types,
- cv-qualifiers,
- ref-qualifier.

The member function is then virtual, with or without the keyword. A call
through a pointer or reference calls the final overrider for the class
of the object. These calls do not use the final overrider:

- a qualified call such as `p->B::f()`,
- a call on an object, not a pointer or reference.

Default arguments come from the function that the call names. Constant
evaluation follows the same rules, and the final overrider that it calls
must be `constexpr` ({rule}`CE-2`).
:::

:::{rule} VIRT-11
:kind: required
:tests: classes/virtual/polymorphic-class.cpp

A class that declares or inherits a virtual function is polymorphic.
:::

:::{rule} VIRT-6
:kind: required
:tests: classes/virtual/call-during-construction-constant.cpp classes/virtual/call-during-construction.cpp classes/virtual/call-during-destruction.cpp classes/virtual/call-during-member-init.cpp

During the constructor or destructor of a class `C`, a virtual call on
the object calls the final overrider for `C`. Overrides in classes
derived from `C` are not used. After all constructors complete, calls use
the most derived class.
:::

:::{rule} VIRT-7
:kind: required
:tests: classes/virtual/virtual-dtor-delete-constant.cpp classes/virtual/virtual-dtor-delete.cpp classes/virtual/virtual-dtor-inherited.cpp

If the destructor of a base is virtual, the destructors of all derived
classes are virtual. `delete` through a pointer to that base then calls
the destructor of the most derived class.
:::

:::{rule} VIRT-10
:kind: required
:tests: classes/virtual/dynamic-cast-ambiguous-target.cpp classes/virtual/dynamic-cast-constant.cpp classes/virtual/dynamic-cast-cross.cpp classes/virtual/dynamic-cast-down.cpp classes/virtual/dynamic-cast-during-construction.cpp classes/virtual/dynamic-cast-null.cpp classes/virtual/dynamic-cast-private-base.cpp classes/virtual/dynamic-cast-reference.cpp classes/virtual/dynamic-cast-void.cpp classes/virtual/dynamic-cast-down-inside-private-base.cpp classes/virtual/dynamic-cast-cross-from-private-base.cpp classes/virtual/dynamic-cast-cross-to-private-base.cpp classes/virtual/dynamic-cast-private-base-constant.cpp classes/virtual/dynamic-cast-reference-fails-throws.cpp classes/virtual/dynamic-cast-reference-fails-constant.cpp foundations/language-support/exception-classes-bad-cast.cpp

`dynamic_cast` converts with a check at run time. The operand refers to
a subobject `s`. A cast to a class `T` takes the first of these steps
that applies:

1. If `s` is a public base subobject of exactly one `T` object, the
   result refers to that `T` object.
2. If the most derived object reaches `s` through public bases only,
   and holds one unambiguous public base subobject of class `T`, the
   result refers to that `T` subobject. This step includes
   the cast to the most derived class and the *cross cast* to another
   base.
3. Otherwise the check fails.

A cast to `void*` gives a pointer to the most derived object. If the
check fails for a pointer, the result is a null pointer. If the check
fails for a reference, the cast throws `std::bad_cast`
({rule}`LSUP-8`). A null pointer converts to a null pointer.
During a constructor or destructor of class `C`, the object counts as a
`C`, so a cast to a class derived from `C` fails. Constant evaluation
can do these casts. {rule}`CE-8` rejects a failed cast of a reference
during constant evaluation.
:::

## Undefined Behavior

:::{rule} VIRT-12
:kind: undefined
:tests: classes/virtual/delete-array-through-base-constant.cpp classes/virtual/nonvirtual-dtor-delete-constant.cpp classes/virtual/pure-call-in-ctor-constant.cpp classes/virtual/pure-call-in-ctor-sanitizer.cpp classes/virtual/nonvirtual-dtor-delete-sanitizer.cpp classes/virtual/delete-array-through-base-sanitizer.cpp

The behavior is undefined in these cases:

- A constructor or destructor calls a pure virtual function of its class
  through a virtual call.
- `delete` destroys a derived object through a pointer to a base whose
  destructor is not virtual ({question}`Q-0056`).
- `delete[]` destroys an array of derived objects through a pointer to a
  base, also when the destructor is virtual.
:::

## Implementation Requirements

:::{rule} VIRT-17
:kind: no-diagnostic
:tests: classes/virtual/pure-dtor-needs-definition.cpp

A pure virtual destructor that a derived class uses must have a
definition. An implementation need not diagnose a missing definition
before linking.
:::

:::{rule} VIRT-20
:kind: no-diagnostic
:tests: classes/virtual/virtual-not-defined.cpp

A virtual function that is not pure must have a definition, even if no
code calls it. An implementation need not diagnose a missing definition
before linking.
:::

## Recommended Practice

:::{rule} VIRT-13
:kind: recommended
:tests: classes/virtual/delete-nonvirtual-warns.cpp classes/virtual/delete-nonvirtual-abstract-warns.cpp classes/virtual/delete-nonvirtual-final-no-warning.cpp classes/virtual/hidden-virtual-warns.cpp

An implementation should warn about these cases:

- `delete` through a pointer to a polymorphic class whose destructor is
  not virtual, unless the class is `final`,
- a derived function that hides a virtual function of a base and does not
  override it.
:::

## Examples

```cpp
struct Shape {
  virtual ~Shape() = default;
  virtual double area() const = 0;   // Shape is abstract
  virtual Shape* clone() const = 0;
};
struct Circle final : Shape {
  double r = 1;
  double area() const override { return 3.14 * r * r; }
  Circle* clone() const override { return new Circle(*this); }  // covariant
};

Shape s;                           // rejected: abstract
Shape* p = new Circle;
double a = p->area();              // calls Circle::area
Circle* c = dynamic_cast<Circle*>(p);  // points to the Circle
delete p;                          // calls ~Circle, then ~Shape
```

A virtual call during constant evaluation reaches a `constexpr` final
overrider (VIRT-1):

```cpp
struct Tile {
  constexpr virtual int cost() const = 0;
  constexpr virtual ~Tile() = default;
};
struct Road : Tile {
  constexpr int cost() const override { return 1; }
};
struct Swamp : Tile {
  int cost() const override { return 5; }   // not constexpr
};
constexpr int price(const Tile& t) { return t.cost(); }
static_assert(price(Road{}) == 1);   // fine
static_assert(price(Swamp{}) == 5);  // rejected: Swamp::cost is not constexpr
```

A static member function cannot take the place of a virtual function
(VIRT-21):

```cpp
struct Base { virtual void reset(); };
struct Derived : Base {
  static void reset();             // rejected: Base::reset is virtual
};
```

Each failed condition of VIRT-4 makes the return type of an override
not covariant (VIRT-15):

```cpp
struct Node { };
struct Leaf : private Node { };
struct Left : Node { };
struct Right : Node { };
struct Twice : Left, Right { };

struct Maker {
  virtual Node* make();
  virtual const Node* view();
};
struct HiddenBase : Maker {
  Leaf* make() override;           // rejected: Node is a private base of Leaf
};
struct TwoBases : Maker {
  Twice* make() override;          // rejected: Twice has two Node bases
};
struct MoreConst : Maker {
  const volatile Left* view() override;  // rejected: adds volatile
};
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 117
of 121 tests.

- VIRT-12: GCC accepts `delete` through a base without a virtual
  destructor during constant evaluation. Clang and EDG reject the
  `delete` ({question}`Q-0056`).
- VIRT-17: Clang and GCC report the missing destructor at link time.
  EDG translates the program and links nothing, so the test expects
  EDG to accept it ({ref}`baseline compilers <baseline-compilers>`).
- VIRT-8: Clang accepts a call to a function that returns an abstract
  class. GCC and EDG reject the call ({question}`Q-0086`).
- VIRT-19: EDG accepts an explicit object member function with the name
  and parameters of a base virtual function. Clang and GCC reject the
  function ({question}`Q-0087`).
- VIRT-20: Clang and GCC report the missing definition at link time.
  At `-O2` they emit no reference to the virtual table, and the program
  links. VIRT-20 allows both results, because the rule needs no
  diagnostic.
- VIRT-13: Clang 18 and 22 and GCC 14 and 16 give both warnings under
  `-Wall`, also for an abstract class. Neither warns for a `final`
  class.
- VIRT-13: under `-Wall`, Clang and GCC warn about a hidden virtual
  function only when the derived function overrides none of the base
  functions with that name. A derived function that overrides one
  overload and hides the others draws no warning. GCC warns about that
  case under `-Woverloaded-virtual=2`. A `using` declaration of the base
  name keeps the other overloads visible ({rule}`DER-3`).
- VIRT-12: the address sanitizer reports `delete` through a base without
  a virtual destructor only when the base and the derived class differ
  in size. If the derived class adds no data, the run exits 0 on Clang
  and GCC with no report.
- VIRT-12: during constant evaluation, GCC reports a call of a pure
  virtual function from a constructor as a call to a function that is
  not `constexpr` (compare {rule}`CE-2`). The function that GCC names is
  the run-time support function that a pure call reaches. Clang names
  the pure virtual call.
- VIRT-12: during constant evaluation, GCC reports `delete[]` of an
  array of derived objects through a pointer to a base as a change to
  allocated storage from outside the evaluation (compare
  {rule}`CE-5`). Clang reports a `delete` of a subobject.
- VIRT-12: at run time, the address sanitizer of Clang and GCC
  (`-O0 -fsanitize=address -fsized-deallocation`) stops both `delete`
  cases and reports `new-delete-type-mismatch`. Clang 18 passes the
  size to `delete` only under `-fsized-deallocation`.
- VIRT-12: at run time, a virtual call of a pure virtual function
  reaches the run-time library of Clang and GCC. The library prints
  "pure virtual method called" and aborts the program, also without a
  sanitizer.
- VIRT-9: GCC calls a misplaced pure specifier `= 0` an initializer of
  the function. Clang calls the specifier a pure specifier.
- VIRT-21: GCC reports a static member function with the name and
  parameters of a base virtual function in two lines. The second line
  points at the virtual function in the base.
- VIRT-10: every Clang and GCC configuration gives a null pointer for a
  cast from a private base, to the derived class or across to another
  base. A cast from the same subobject to a class that has it as a
  public base succeeds. EDG 7.0 lets the cast from a private base
  succeed during constant evaluation ({question}`Q-0212`).
- VIRT-10: a failed cast of a reference throws at run time on every
  Clang and GCC configuration, and a handler `catch (...)` catches the
  exception. Constant evaluation rejects the failed cast on every
  compiler.

Inputs: clang-sema-declcxx, clang-sema-expr, clang-diag-sema,
edg-class-decl.
