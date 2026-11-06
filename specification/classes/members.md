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

A static data member can have the class's own type, because the
definition of the static data member comes after the class.

The class body is one scope, so each member name has one meaning in the
body ({principle}`NAM-P1`, {rule}`TYN-5`), and code outside the body
names a nested type through the class. A class defined inside a function
is a scope that can own nothing that would need a definition elsewhere.
:::

:::{principle} CLS-P2
:rules: CLS-3 CLS-4 CLS-7 CLS-20 CLS-21 CLS-23 CLS-24 CLS-27

A member belongs to each object of the class or to the class as a whole.
A static data member is one object for the whole program, so it needs
one definition, and a static member function has no object to act on. A
member function defined in the class is `inline`, because every unit
that includes the class sees the function body. A `const` static member
of integer type with an initializer in the class is a constant the class
can use directly, so the member cannot be `volatile`, and the
initializer must be a constant expression, and
the member still needs a definition if the program uses the member as an
object.
:::

:::{principle} CLS-P3
:rules: CLS-5 CLS-6 CLS-13 CLS-14 CLS-15 CLS-17 CLS-25 CLS-26

A non-static member function acts on an object. In the function, `this`
has the type of that object with the function's own cv-qualifiers. The
ref-qualifier of the function selects the function by the {term}`value category`
of the object. A function with no ref-qualifier takes an object of
either category, so the function would compete with each qualified
overload that has the same cv-qualifiers. Overloads with the same
parameters and the same cv-qualifiers therefore all have
ref-qualifiers, or none of them has one.

A member function can instead write that object out as its first
parameter. The object is then an ordinary argument, so the function:

- has no `this`,
- cannot be static or virtual or qualified,
- has an ordinary function pointer as its address.

The object of each call initializes that parameter, so a default
argument for the parameter could never take effect, and the parameter
cannot have one.

The call and subscript operators need no object at all, so they can be
static.
:::

:::{principle} CLS-P4
:rules: CLS-10 CLS-11 CLS-19 CLS-29 CLS-30 CLS-31 CLS-32 CLS-33 CLS-34 CLS-35

The members of an object lie in the object in the order of their
declarations, and each object has a size, even an empty one, so that two
objects of one type never share an address ({principle}`OBJ-P1`).

A *standard-layout* class promises a layout that a program can rely on.
The object starts with its first member, and its bases add no bytes
before that member:

```text
address of the object, of each base subobject, and of the first member
|
+--------------+--------------+---------+
| first member | other member | padding |
+--------------+--------------+---------+
```

Each condition of CLS-19 removes something that would break the
picture:

- a pointer for virtual calls, or a {term}`virtual base`, at the start,
- members of two classes, which would need an order between the classes,
- members in two accesses,
- two {term}`subobjects <subobject>` of one type, which need two addresses. This case
  covers a base whose type is the type of an object at the start of the
  first member (CLS-32).

An empty base whose own class is standard-layout takes no bytes, in any
derived class (CLS-33). The rest of a class layout is the
implementation's choice, in two grades:

| Grade | Examples |
|---|---|
| Implementation-defined (CLS-11, CLS-31) | the size of an empty class, the size of other empty bases and members, alignment, bit-fields |
| Unspecified (CLS-34) | the place of the pointer for virtual calls, padding, the use of the end padding of a base |

CLS-31, BF-9, and CLS-35 give the choices of the matrix targets.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   class_def: `class_head` "{" {`member_decl`} "}"
   class_name: `identifier`
   member_decl: {`attribute_spec` | `alignment_specifier`} [`decl_specifiers`]
              :    [`member_declarator` {"," `member_declarator`}] ";"
              : | `function_definition`
              : | `access_label`
              : | `friend_decl`
              : | `alias_decl`
              : | `using_decl`
              : | `static_assert_decl`
              : | `template_decl`
              : | `explicit_specialization`
              : | `deduction_guide`
   member_declarator: `declarator` [`virt_specifiers`] [`pure_specifier` | `member_init`]
                    : | `declarator` `requires_clause`
                    : | `bitfield`
   member_init: "=" `assignment_expr` | "=" `braced_init` | `braced_init`
```

## Constraints

:::{rule} CLS-1
:kind: reject
:tests: classes/members/member-of-own-type.cpp classes/members/pointer-to-own-type.cpp classes/members/incomplete-member.cpp

A non-static data member must have a complete type. A class is not
complete inside its own definition, so the class cannot contain a member
of its own type. A pointer to the class type is valid.
:::

:::{rule} CLS-2
:kind: required
:tests: classes/members/complete-in-member-bodies.cpp classes/members/complete-in-default-member-init.cpp classes/members/complete-in-default-argument.cpp classes/members/noexcept-complete-class.cpp expressions/postfix/default-template-argument-incomplete-class.cpp

The class is complete inside:

- member function bodies,
- default arguments,
- default member initializers,
- the operands of `noexcept` specifiers.

A default template argument of a member template is not among these
places ({question}`Q-0406`).

These places can use members that are declared later. At other places
inside the definition, the class is not complete.
:::

:::{rule} CLS-22
:kind: reject
:tests: classes/members/sizeof-in-class-incomplete.cpp expressions/postfix/member-access-incomplete-class.cpp

At the places inside a class definition where CLS-2 says the class is
not complete, a use that needs a complete class type is not valid.
:::

:::{rule} CLS-3
:kind: required
:tests: classes/members/static-data-member.cpp classes/members/static-member-not-defined.cpp classes/members/inline-static-member.cpp classes/members/static-const-integral.cpp classes/members/static-const-enum.cpp

A static data member is one object for the program. These rules apply:

- A static data member that is not `inline` needs one definition outside
  the class (LNK-6, LNK-7).
- An `inline` or `constexpr` static data member is defined in the class.
- A `const` static data member of integer or enumeration type that is
  not `volatile` can have an initializer in the class.
:::

:::{rule} CLS-23
:kind: reject
:tests: classes/members/static-non-const-in-class-init.cpp classes/members/static-const-double-in-class-init.cpp declarations/volatile/static-const-volatile-in-class.cpp declarations/volatile/static-inline-const-volatile-in-class.cpp

CLS-3 lists the static data members that can have an initializer in the
class. Other static data members must not have an initializer in the
class. A `static const volatile` integer member that is not `inline` is
one of them, because constant evaluation cannot read a volatile object
({principle}`VOL-P1`, {question}`Q-0080`).
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
not `const` on a `const` object, or a member function that is not
`volatile` on a `volatile` object.
:::

:::{rule} CLS-6
:kind: required
:tests: classes/members/ref-qualified.cpp classes/members/ref-qualified-const-and-xvalue.cpp classes/members/ref-qualified-mixed-different-cv.cpp

A member function can have the ref-qualifier `&` or `&&`. A call on an
lvalue selects `&`, and a call on an rvalue selects `&&`.
:::

:::{rule} CLS-26
:kind: reject
:tests: classes/members/ref-qualified-mixed.cpp classes/members/ref-qualified-mixed-const.cpp classes/members/ref-qualified-mixed-call.cpp classes/members/ref-qualified-mixed-different-cv.cpp classes/members/explicit-object-and-unqualified-clash.cpp

The overloads of a member function that CLS-6 describes must all have
ref-qualifiers, or none, if they have the same parameters and the same
cv-qualifiers ({question}`Q-0027`). Overloads whose cv-qualifiers
differ can mix a function with a ref-qualifier and one without. A
member function without a ref-qualifier and one with an explicit object
parameter of reference to the same class type, with the same other
parameters, also clash.
:::

:::{rule} CLS-7
:kind: required
:tests: classes/members/static-member-function.cpp classes/members/static-member-function-pointer.cpp

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
:tests: classes/members/nested-types.cpp classes/members/nested-types-out-of-class.cpp

A class can declare these members:

- nested classes,
- enumerations,
- type aliases.

Code outside the class names these members as `S::name`. A nested class
or enumeration can share its name with a data member, member functions,
or an enumerator ({rule}`NAM-39`).
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
the function. The member function can also read the value of a
`constexpr` variable of any {term}`literal type`, or of another constant
variable, without taking the address of the variable.
:::

:::{rule} CLS-28
:kind: reject
:tests: classes/members/local-class-static-member.cpp classes/members/local-class-uses-local.cpp classes/members/local-class-member-defined-outside.cpp classes/members/local-class-member-template.cpp classes/members/local-class-friend-definition.cpp

A local class (CLS-12) must not have:

- a static data member,
- a {term}`member template`,
- a friend function defined in the local class.

Each member function of a local class is defined inside the class. In an
evaluated operand, such a member function must not name the automatic
variables of the function, except as CLS-12 allows.
:::

:::{rule} CLS-13
:kind: required
:tests: classes/members/explicit-object-parameter.cpp classes/members/explicit-object-from-prvalue.cpp foundations/evaluation-order/explicit-object-order-unspecified.cpp classes/members/explicit-object-deduced-forwarding.cpp classes/members/explicit-object-deduced-lvalue.cpp classes/members/explicit-object-deduced-constant.cpp classes/operators/subscript-explicit-object-two-args.cpp

A member function can declare its first parameter as an explicit object
parameter, written `this T self`. The object of the call initializes that
parameter. If the parameter has class type and the object is a prvalue
of that type, the prvalue initializes the parameter directly, with no
temporary ({rule}`VC-4`, {question}`Q-0181`). The parameter ends
when any other parameter would ({rule}`FN-10`).

The parameter can have a type that the call deduces, such as
`this Self&& self` in a member function template or `this auto& self`.
The object of the call is then the argument that deduction uses
({rule}`DED-8`). One such function serves objects of every value
category and cv-qualification:

| Object of the call | `Self` for `this Self&& self` | `Self` for `this Self& self` |
|---|---|---|
| a non-`const` lvalue `g` of type `G` | `G&` | `G` |
| a `const` lvalue of type `G` | `const G&` | `const G` |
| an rvalue of type `G` | `G` | no match |
:::

:::{rule} CLS-14
:kind: reject
:tests: classes/members/explicit-object-no-this.cpp classes/members/explicit-object-qualified.cpp classes/members/explicit-object-static.cpp classes/members/explicit-object-virtual.cpp classes/members/explicit-object-not-first.cpp classes/members/explicit-object-default-argument.cpp classes/members/explicit-object-non-member.cpp

A member function with an explicit object parameter has no `this`. It
must not:

- be static,
- be virtual,
- have a cv-qualifier or a ref-qualifier.

The parameter declared with `this` must be the first parameter of a
member function or a lambda, and it must not have a default argument
({question}`Q-0317`).
:::

:::{rule} CLS-15
:kind: required
:tests: classes/members/static-call-operator.cpp classes/members/static-subscript-no-parameters.cpp classes/operators/subscript-explicit-object-two-args.cpp classes/operators/static-call-operator-template.cpp overloading/overload-resolution/address-static-member-template.cpp

`operator()` and `operator[]` can be static member functions. `operator[]`
can have any number of parameters ({rule}`OPR-3`).
:::

:::{rule} CLS-16
:kind: required
:tests: classes/members/static-member-own-type.cpp classes/members/static-member-own-type-constant.cpp classes/members/thread-local-static-member.cpp

A static data member can have the type of its own class, which is still
incomplete at the declaration. The definition comes after the class, and
it can be `inline constexpr` when the declaration in the class is
`const`. A static data member can be `thread_local`.
:::

:::{rule} CLS-17
:kind: required
:tests: classes/members/explicit-object-address-ordinary.cpp classes/members/explicit-object-address-by-value.cpp

The address of a member function with an explicit object parameter is an
ordinary pointer to function. The address is not a pointer to member. A
call through the pointer passes the object as the first argument.
:::

:::{rule} CLS-20
:kind: reject
:tests: classes/members/static-const-init-not-constant.cpp classes/members/static-const-enum-init-not-constant.cpp

The initializer of a `const` static data member of integer or
enumeration type in its class must be a constant expression.
:::

:::{rule} CLS-19
:kind: required
:tests: classes/members/standard-layout-reinterpret.cpp classes/members/standard-layout-members-in-base.cpp classes/members/standard-layout-static-member-access.cpp classes/members/standard-layout-empty-bases.cpp classes/members/standard-layout-base-type-first-member.cpp classes/members/standard-layout-repeated-base.cpp classes/unions/common-initial-sequence-inherited.cpp classes/members/empty-base-shares-address.cpp classes/members/standard-layout-bases-same-address.cpp

A class is *standard-layout* if all of these conditions hold:

- It has no virtual function and no virtual base.
- All its non-static data members have the same access.
- No non-static data member is a reference, and each has a
  {term}`standard-layout` type (CLS-29).
- Each base class is standard-layout.
- One class declares all the non-static data members and bit-fields of
  the class and its bases. Unnamed bit-fields count.
- No two base class subobjects have the same type.
- No base class has the type of an object that CLS-32 lists.

A standard-layout class declared with `union` is a *standard-layout
union*. Static members and member functions play no part.
:::

:::{rule} CLS-32
:kind: required
:tests: classes/members/standard-layout-base-type-first-member.cpp classes/members/standard-layout-base-type-inside-member.cpp classes/members/standard-layout-base-type-anonymous-union.cpp

For CLS-19, these objects lie at the start of a class `X`
({question}`Q-0341`):

- the data member of `X` that is declared first among those that are
  not static, unless that member is a bit-field,
- if that member has class type, the objects at the start of that class,
- if that member is an array, its first element and the objects at the
  start of the element,
- if that member is a union, each member of the union and the objects at
  the start of each.

An anonymous union counts as a member of union type. Each member of `X`
that has zero size also counts, in any position, together with the
objects at its start. Only a `[[no_unique_address]]` member can have
zero size, and the choice is implementation-defined (CLS-31).
:::

:::{rule} CLS-29
:kind: required
:tests: classes/members/standard-layout-types.cpp classes/members/standard-layout-nested-array.cpp

These types are {term}`standard-layout` types:

- {term}`scalar types <scalar type>`,
- standard-layout classes (CLS-19),
- arrays of standard-layout types,
- the cv-qualified forms of these types.

A reference type, `void`, and a function type are not standard-layout.
:::

## Runtime Behavior

:::{rule} CLS-10
:kind: required
:tests: classes/members/member-order.cpp classes/members/empty-class-size.cpp classes/members/distinct-addresses.cpp classes/members/member-order-mixed-access.cpp classes/members/first-member-same-address.cpp classes/members/empty-base-shares-address.cpp classes/members/layout-base-type-first-member.cpp classes/members/layout-empty-base-not-standard-layout.cpp classes/members/layout-overlap-member.cpp classes/members/standard-layout-base-type-first-member.cpp classes/members/standard-layout-base-type-inside-member.cpp classes/members/standard-layout-bit-field-first.cpp classes/members/standard-layout-repeated-base.cpp classes/members/standard-layout-base-type-anonymous-union.cpp classes/members/layout-empty-bases-depths.cpp classes/members/layout-empty-member-after-empty-base.cpp

Non-static data members of nonzero size have increasing addresses in the
order of their declarations. Members of an anonymous union are not in
this order. An object of an empty class has a size greater than zero. Two
distinct complete objects have distinct addresses. The first data member
of an object and a base subobject of an empty class can have the address
of the object.
:::

:::{rule} CLS-30
:kind: required
:tests: classes/members/first-member-same-address.cpp classes/members/standard-layout-bases-same-address.cpp classes/members/empty-base-shares-address.cpp classes/members/standard-layout-reinterpret.cpp classes/members/standard-layout-members-in-base.cpp classes/members/standard-layout-static-member-access.cpp classes/members/standard-layout-empty-bases.cpp classes/members/standard-layout-bit-field-first.cpp classes/members/standard-layout-types.cpp classes/members/standard-layout-address-constant.cpp classes/members/standard-layout-nested-array.cpp

An object of a standard-layout class has these properties:

- Its address equals the address of each of its base subobjects.
- If the earliest declared data member that is not static is not a
  bit-field, the object and that member start at one address.

A `reinterpret_cast` between a pointer to the object and a pointer to
that member or base gives a usable pointer ({rule}`STG-16`). The same
holds during {term}`constant evaluation`
({question}`Q-0338`).
:::

:::{rule} CLS-33
:kind: required
:tests: classes/members/empty-standard-layout-base-zero-size.cpp classes/members/empty-standard-layout-base-zero-size-ilp32.cpp classes/members/standard-layout-empty-bases.cpp classes/members/layout-empty-base-not-standard-layout.cpp classes/members/layout-base-type-first-member.cpp classes/members/empty-class-cases.cpp

A base subobject takes no bytes if its class is an {term}`empty class`
and standard-layout. The derived class need not be standard-layout. The
offset of such a base is unspecified (CLS-34).
:::

## Implementation Requirements

:::{rule} CLS-11
:kind: impl-defined
:tests: classes/members/empty-class-size-one.cpp classes/members/empty-class-array-size.cpp classes/members/empty-class-size-one-ilp32.cpp classes/members/empty-class-cases.cpp

The size of an {term}`empty class` is implementation-defined. Every compiler in
the matrix gives size 1, on x86-64 and with `-m32`.
:::

:::{rule} CLS-31
:kind: impl-defined
:tests: classes/members/layout-empty-base-not-standard-layout.cpp classes/members/layout-overlap-member.cpp classes/members/layout-overlap-member-ilp32.cpp declarations/attributes/no-unique-address.cpp classes/members/empty-class-cases.cpp classes/members/layout-empty-member-placement.cpp classes/members/layout-empty-member-placement-ilp32.cpp

These parts of a class layout are implementation-defined:

- whether an object of an {term}`empty class` takes no bytes, if the object is
  a base subobject outside CLS-33, or a `[[no_unique_address]]` member,
- the allocation and alignment of bit-fields ({rule}`BF-9`),
- alignment requirements ({rule}`ALN-5`).

Every compiler in the matrix makes these choices, on x86-64 and with
`-m32`:

| Case | Choice |
|---|---|
| An empty base whose class is not standard-layout | The base takes no bytes. |
| A `[[no_unique_address]]` member of empty class type | The member takes no bytes. Two such members of one type get different addresses ({rule}`ATTR-9`). |
:::

:::{rule} CLS-34
:kind: unspecified
:tests: classes/members/layout-base-type-first-member.cpp classes/members/layout-virtual-first-member.cpp classes/members/layout-virtual-first-member-ilp32.cpp classes/members/layout-base-tail-padding.cpp classes/members/layout-base-tail-padding-ilp32.cpp classes/members/layout-base-tail-padding-defaulted.cpp classes/members/layout-overlap-member.cpp classes/members/layout-overlap-member-ilp32.cpp classes/members/layout-order-and-pointer.cpp classes/members/layout-order-and-pointer-ilp32.cpp classes/members/layout-tail-padding-conditions.cpp classes/members/layout-tail-padding-conditions-ilp32.cpp classes/members/layout-tail-padding-deleted-constructor.cpp classes/members/layout-tail-padding-deleted-constructor-ilp32.cpp classes/members/layout-tail-padding-move-assignment.cpp classes/members/layout-tail-padding-move-assignment-ilp32.cpp classes/members/layout-empty-bases-depths.cpp classes/members/layout-empty-member-after-empty-base.cpp classes/members/layout-tail-padding-assignment-forms.cpp classes/members/layout-virtual-base-order.cpp classes/members/layout-virtual-base-part-ilp32.cpp classes/members/layout-virtual-base-part.cpp classes/members/layout-empty-member-placement.cpp classes/members/layout-empty-member-placement-ilp32.cpp classes/members/layout-virtual-base-alignment.cpp classes/members/layout-virtual-base-nested-order.cpp classes/members/layout-empty-virtual-base.cpp classes/members/layout-no-unique-address-end-padding.cpp classes/members/layout-no-unique-address-end-padding-ilp32.cpp classes/members/layout-union-size.cpp classes/members/layout-union-size-ilp32.cpp classes/members/layout-union-end-padding.cpp classes/members/layout-union-end-padding-ilp32.cpp classes/members/layout-member-sizes.cpp classes/members/layout-member-sizes-ilp32.cpp classes/members/layout-empty-base-offset-one.cpp

These parts of a class layout are unspecified, within CLS-10, CLS-30,
CLS-31, and CLS-33:

- the offset of a base subobject of zero size,
- the place of the pointer for virtual calls,
- the padding between members and at the end of the class,
- whether later members use the padding at the end of a base or of a
  `[[no_unique_address]]` member.

{rule}`CLS-35` gives the choices of the matrix targets.
:::

:::{rule} CLS-35
:kind: unspecified
:tests: classes/members/layout-order-and-pointer.cpp classes/members/layout-order-and-pointer-ilp32.cpp classes/members/layout-virtual-base-part.cpp classes/members/layout-virtual-base-part-ilp32.cpp classes/members/layout-virtual-base-order.cpp classes/members/layout-empty-member-after-empty-base.cpp classes/members/layout-empty-bases-depths.cpp classes/members/layout-empty-base-offset-one.cpp classes/members/layout-tail-padding-assignment-forms.cpp classes/members/layout-virtual-first-member.cpp classes/members/layout-virtual-first-member-ilp32.cpp classes/members/layout-base-type-first-member.cpp classes/members/layout-base-tail-padding.cpp classes/members/layout-base-tail-padding-ilp32.cpp classes/members/layout-tail-padding-conditions.cpp classes/members/layout-tail-padding-conditions-ilp32.cpp classes/members/layout-base-tail-padding-defaulted.cpp classes/members/layout-tail-padding-deleted-constructor.cpp classes/members/layout-tail-padding-deleted-constructor-ilp32.cpp classes/members/layout-tail-padding-move-assignment.cpp classes/members/layout-tail-padding-move-assignment-ilp32.cpp classes/members/layout-overlap-member.cpp classes/members/layout-overlap-member-ilp32.cpp foundations/storage/empty-base-takes-no-bytes.cpp foundations/storage/base-bytes-shared-with-member.cpp foundations/storage/no-unique-address-member-shares-bytes.cpp classes/members/layout-empty-member-placement.cpp classes/members/layout-empty-member-placement-ilp32.cpp classes/members/layout-virtual-base-alignment.cpp classes/members/layout-virtual-base-nested-order.cpp classes/members/layout-empty-virtual-base.cpp classes/members/layout-no-unique-address-end-padding.cpp classes/members/layout-no-unique-address-end-padding-ilp32.cpp classes/members/layout-union-size.cpp classes/members/layout-union-size-ilp32.cpp classes/members/layout-union-end-padding.cpp classes/members/layout-union-end-padding-ilp32.cpp classes/members/layout-member-sizes.cpp classes/members/layout-member-sizes-ilp32.cpp

The matrix targets, x86-64 and `-m32`, make the choices of CLS-34 as
the table gives. The *data size* of a class is the offset just past
the last byte that its pointer, bases, members, and bit-fields hold.
An empty subobject adds nothing to the data size, at offset 0 or at any
other offset. The class still ends after each empty subobject, so its
size is at least the offset of the empty subobject plus the subobject's
`sizeof`. The *data size so
far* is the data size of the parts placed before the current one.

| Part of the layout | Choice |
|---|---|
| Order | The pointer for virtual calls comes first, if the class has its own. The bases that are not virtual follow in the order of the base list. Then come the members, and then the virtual bases. The virtual bases come in the order that a depth-first walk of the base lists, from left to right, first meets them. The walk records a virtual base before it walks the base list of that base. |
| Pointer for virtual calls | A class with a virtual function or a {term}`virtual base` has the pointer at offset 0. If a base that is not virtual has such a pointer, the first such base goes to offset 0, and the class uses the pointer of that base. The pointer has the size and alignment of `void*` ({rule}`TYP-1`, {rule}`TYP-2`). |
| Padding | Each base and member starts at the data size so far, rounded up to a multiple of its alignment ({rule}`TYP-2`). The size of the class is at least its data size and the end of each empty subobject, and is a multiple of the largest alignment of its pointer, bases, and members. |
| An empty base, virtual or not | The base goes to offset 0 and adds nothing to the data size, so a later member can start at the same offset. |
| An empty `[[no_unique_address]]` member | The member goes to offset 0 and adds nothing to the data size, as an empty base does. |
| Two empty subobjects of one type | No base or member goes where it would put a subobject of an empty class type at the address of another subobject of that type. The check covers every empty subobject that the base or member holds. On a conflict, the base or member tries the data size so far, rounded up to its alignment, and then each next multiple of its alignment. |
| A base, or a `[[no_unique_address]]` member, of class type `C` | If `C` keeps its end padding, the subobject takes `sizeof(C)` bytes. If not, the subobject adds the data size of `C`, and the next bases and members can start in the padding at the end of `C`. |
| A base of class type `C` with virtual bases | The base subobject holds only the pointer of `C`, the bases of `C` that are not virtual, and the members of `C`. The base starts at a multiple of the largest alignment of these parts, which can be less than `alignof(C)`. The complete object holds each virtual base once. |
| A union | Every member starts at offset 0. The data size is the largest `sizeof` of a member. The size is the data size rounded up to the largest alignment of the members, and at least 1. |
| A reference member, or a pointer to data member | The member takes the size and alignment of `void*`. |
| A pointer to member function | The member takes twice the size of `void*`, with the alignment of `void*`. |
| Any other member | The next members start after its `sizeof` bytes. |

On GCC 16, Clang 22, and Clang 18, a class or union keeps its end
padding if all of these conditions hold:

- The class has no base and no virtual function.
- No data member that is not static is `private` or `protected`, or has
  a default member initializer.
- The class declares no constructor template, and no {term}`user-provided`
  constructor, destructor, or copy assignment operator. The copy
  assignment operator has one of the parameter forms of {rule}`CPY-3`,
  and an `operator=` template or a move assignment operator does not
  count.
- Each data member that is not static has a scalar type, a class or
  union type that keeps its end padding, or an array of such a type.

The compilers differ from this list in these cases:

| Compiler | Difference |
|---|---|
| GCC 14 | A class with a constructor, also a defaulted or deleted one, lends its end padding ({question}`Q-0342`). |
| GCC 14, GCC 16 | A class with a `[[no_unique_address]]` member lends its end padding ({question}`Q-0381`). |
| EDG 7.0 | A class with a deleted constructor, or a user-provided move assignment operator, lends its end padding ({question}`Q-0361`). A union always keeps its end padding ({question}`Q-0384`). |
:::

:::{rule} CLS-21
:kind: no-diagnostic
:tests: classes/members/static-const-odr-use-no-definition.cpp classes/members/static-const-address-no-definition.cpp

A `const` static data member with an initializer in its class, and not
`constexpr` or `inline`, needs a definition outside the class if the
program uses the address of the member, for example by binding a
reference to the member. An implementation need not diagnose a missing
definition before linking.
:::

## Examples

A deduced object parameter gives one accessor for a non-`const` and a
`const` object (CLS-13):

% tidal-example: fragment
```cpp
struct Grid {
  int cells[4]{};
  template <class Self>
  auto&& operator[](this Self&& self, int r, int c) {
    return static_cast<Self&&>(self).cells[r * 2 + c];
  }
};
Grid g;
g[1, 1] = 7;              // Self is Grid&, the result is int&
const Grid& c = g;
int x = c[1, 1];          // Self is const Grid&, the result is const int&
```

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

The layout choices of CLS-35 on the matrix targets. Each comment gives
the offsets on x86-64, then on `-m32` where they differ.

```cpp
struct Empty {};

// Order and padding: pointer 0, c 8, i 12, size 16 (-m32: 0, 4, 8, 12).
struct Shape { virtual void draw() {} char c; int i; };

// An empty base: Empty 0, n 0, size 4.
struct Counted : Empty { int n; };

// An empty [[no_unique_address]] member: n 0, tag 0, size 4.
struct Tagged { int n; [[no_unique_address]] Empty tag; };

// Two empty subobjects of one type: first 0, n 0, data size 4,
// second 4, size 8.
struct Twice {
  [[no_unique_address]] Empty first;
  int n;
  [[no_unique_address]] Empty second;
};

// End padding: Plain keeps it, so z goes to 8 and the size is 12.
// Built has a user-provided constructor and lends it: z 5, size 8.
struct Plain { int i; char c; };
struct Built { Built() {} int i; char c; };
struct AfterPlain : Plain { char z; };
struct AfterBuilt : Built { char z; };

// A virtual base comes last: pointer 0, y 8, Shared 12, size 16
// (-m32: 0, 4, 8, 12).
struct Shared { int v; };
struct Middle : virtual Shared { char y; };

// A union: every member at 0, data size 5, size 8.
union Bytes { char c[5]; int i; };

// Members of pointer size: r 8, p 16, f 24, size 40
// (-m32: 4, 8, 12, 20). A pointer to member function takes two pointers.
struct Pointers { char c; int& r; int Plain::* p; void (Shape::* f)(); };
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 155
of 171 tests.

### Divergences

- {question}`Q-0080` (CLS-3, CLS-23; open): whether a `static const
  volatile` integer member can have an initializer in its class. It
  cannot, as Clang says. GCC and EDG accept the initializer.
- {question}`Q-0027` (CLS-6, CLS-26; settled on CLS-P3): whether a
  member function with a ref-qualifier can be overloaded with one
  without, when both have the same cv-qualifiers. It cannot. GCC 16
  accepts the declarations and reports the ambiguity only at a call.
- {question}`Q-0341` (CLS-19, CLS-32; open): whether a base whose type
  sits at the start of the first member through a member, an array, or
  a union, or in a later `[[no_unique_address]]` member, makes a class
  not standard-layout. It does, as Clang's query and the
  class-properties inventory (item SL-first-member-type) say. GCC's
  query differs in all four forms, and EDG's in three.
- {question}`Q-0338` (CLS-30; settled on CLS-P4): whether constant
  evaluation finds a standard-layout object at the address of its
  first member and of its base. It does, as at run time. EDG gives
  `false`.
- {question}`Q-0342` (CLS-34, CLS-35; settled): whether a base with
  a defaulted default constructor keeps its end padding. It keeps it,
  and CLS-35 lists the other choice for GCC 14. GCC 14 lends the
  padding.
- {question}`Q-0361` (CLS-34, CLS-35; open): whether a base with a
  deleted constructor or a user-provided move assignment operator keeps
  its end padding. It keeps it, as GCC 16 and Clang do, on x86-64 and
  with `-m32`. EDG lends the padding in both cases, and GCC 14 for a
  deleted constructor.
- {question}`Q-0381` (CLS-34, CLS-35; open): whether a
  `[[no_unique_address]]` member makes a class lend its end padding. It
  does not, as Clang and EDG say. GCC lends the padding.
- {question}`Q-0384` (CLS-34, CLS-35; open): whether a union follows
  the end-padding conditions of a class. It does, as Clang and GCC say.
  EDG keeps the end padding of every union.
- {question}`Q-0181` (CLS-13; open): whether a class prvalue
  initializes a by-value explicit object parameter without a copy or
  move. It does, as every compiler but Clang 18 says. Clang 18 rejects
  the call when the class has no usable copy or move constructor.
- {question}`Q-0317` (CLS-14; settled on CLS-P3): whether an explicit
  object parameter can have a default argument. It cannot. EDG accepts
  the default argument.

### Split tests

EDG runs no programs, so its cell for a run test gives the result of
the compile.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `declarations/volatile/static-const-volatile-in-class.cpp` | CLS-23 | reject | reject | accept | accept | {question}`Q-0080` |
| `explicit-object-default-argument.cpp` | CLS-14 | reject | reject | reject | accept | {question}`Q-0317` |
| `explicit-object-from-prvalue.cpp` | CLS-13 | run | Clang 22: run; Clang 18: reject | run | accept | {question}`Q-0181` |
| `layout-base-tail-padding-defaulted.cpp` | CLS-34, CLS-35 | accept | accept | GCC 16: accept; GCC 14: reject | accept | {question}`Q-0342` |
| `layout-no-unique-address-end-padding-ilp32.cpp` | CLS-35, CLS-34 | accept | accept | reject | accept | {question}`Q-0381` |
| `layout-no-unique-address-end-padding.cpp` | CLS-35, CLS-34 | accept | accept | reject | accept | {question}`Q-0381` |
| `layout-tail-padding-deleted-constructor-ilp32.cpp` | CLS-35, CLS-34 | accept | accept | GCC 14: reject; host GCC: accept | reject | {question}`Q-0361` |
| `layout-tail-padding-deleted-constructor.cpp` | CLS-35, CLS-34 | accept | accept | GCC 16: accept; GCC 14: reject | reject | {question}`Q-0361` |
| `layout-tail-padding-move-assignment-ilp32.cpp` | CLS-35, CLS-34 | accept | accept | accept | reject | {question}`Q-0361` |
| `layout-tail-padding-move-assignment.cpp` | CLS-35, CLS-34 | accept | accept | accept | reject | {question}`Q-0361` |
| `layout-union-end-padding-ilp32.cpp` | CLS-35, CLS-34 | accept | accept | accept | reject | {question}`Q-0384` |
| `layout-union-end-padding.cpp` | CLS-35, CLS-34 | accept | accept | accept | reject | {question}`Q-0384` |
| `ref-qualified-mixed-const.cpp` | CLS-26 | reject | reject | GCC 16: accept; GCC 14: reject | reject | {question}`Q-0027` |
| `ref-qualified-mixed-different-cv.cpp` | CLS-26, CLS-6 | run | run | GCC 16: run; GCC 14: reject | accept | {question}`Q-0027` |
| `ref-qualified-mixed.cpp` | CLS-26 | reject | reject | GCC 16: accept; GCC 14: reject | reject | {question}`Q-0027` |
| `standard-layout-address-constant.cpp` | CLS-30 | accept | accept | accept | reject | {question}`Q-0338` |

### Notes

- CLS-3: a missing definition of a static data member compiles in each
  translation unit. The linker reports the missing definition.
- CLS-26: every compiler rejects a call of two overloads that differ
  only in a ref-qualifier on one of them (`ref-qualified-mixed-call.cpp`).
- CLS-9: Clang and GCC accept an anonymous struct member as an
  extension, and reject the member only under `-pedantic-errors`.
- CLS-19, CLS-29, CLS-32: no core-language check gives a yes or no for
  standard-layout (inventory item OBS-no-core-standard-layout). The
  tests check the guarantees of CLS-30 for classes that meet the
  conditions in an unusual way, and the distinct addresses of CLS-10
  for classes that fail a condition.
- CLS-30: every compiler gives `true` in constant evaluation when it
  compares the address of an array with the address of its first
  element.
- CLS-33: an empty standard-layout base takes no bytes on every
  compiler, also in a class with a virtual function or with members in
  two accesses.
- CLS-31, CLS-34, CLS-35: the layout tests use `sizeof` and run-time
  address differences. The size and alignment come from the probes in
  each test. CLS-31 holds the choices that each implementation
  documents: the size of other empty objects, bit-fields, and
  alignment. The other layout choices are unspecified (CLS-34), because
  an implementation need not document them. CLS-35 gives the choices
  of the matrix compilers. The tests of CLS-35 check each condition of
  its list with a base whose last member leaves 3 bytes of padding.
- CLS-35: every compiler keeps the end padding for a defaulted or
  deleted destructor or assignment operator.
- CLS-35: every compiler gives the same sizes for classes with virtual
  bases, for an empty member after an empty base of the same type, and
  for two empty bases that hold one empty type at different depths.
  Clang and GCC give the same offsets at run time. Every compiler lends
  the end padding for each copy assignment operator form of CPY-3 and
  for a constructor template, and keeps it for an `operator=` template.
- CLS-35: every compiler places an empty `[[no_unique_address]]`
  member as it places an empty base, and an empty virtual base as an
  empty base that is not virtual. Every compiler starts a base with
  virtual bases at the alignment of the part that it holds, and puts a
  virtual base before its own virtual bases. Every compiler gives a
  reference member and a pointer to data member the size of a pointer,
  and a pointer to member function twice that size.
- CLS-11, CLS-33: every compiler treats a class whose only member is an
  empty `[[no_unique_address]]` member, or an unnamed bit-field of
  width 0, as an {term}`empty class`. An unnamed bit-field of nonzero
  width makes the class take its byte as a base.
- CLS-11: the size is 1 on every compiler in the matrix, on both
  targets.
- CLS-21: when a reference binds to the member, Clang and GCC emit a
  reference to the member at `-O0` and at `-O2`, and the link fails.
  EDG translates the program and links nothing.
- CLS-13: every baseline compiler supports explicit object
  parameters, Clang 18 and GCC 14 included, and runs the tests that use
  them.
- CLS-14: every compiler rejects an explicit object parameter that is
  not first and one on a non-member function.

### Probe grids

The tables are in [`tables.md`](https://github.com/sempuki/tidal/blob/main/tests/classes/members/grids/tables.md) beside the grid files.

The probe grid `tests/classes/members/grids/standard-layout.json`
compares the conditions of CLS-19 with each compiler's builtin query,
which no test uses (decision 0051). Each row is a class, and each
column a compiler's query. The three compilers agree with each other
and with CLS-19 on 34 of 38 rows. The other four rows are routes of
CLS-32, and {question}`Q-0341` gives them.

### Sources

| Rule | Source |
|---|---|
| CLS-19, CLS-32 | grid standard-layout; inventory items SL-access, SL-base-standard-layout, SL-bit-field-counts, SL-first-member-type, SL-member-standard-layout, SL-one-declaring-class, SL-one-subobject-per-type, SL-struct-vs-union, SL-virtual |
| CLS-29 | inventory items TYPE-array, TYPE-cv, TYPE-scalar |
| CLS-30 | inventory items SLG-base-address, SLG-first-member-address, SLG-pointer-interconvertible; tests |
| CLS-33 | inventory item SLG-empty-base-zero; tests; gap review of item O (`reports/review/2026-11-03-gap-review-item-o.md`) |
| CLS-31, CLS-34, CLS-35 | layout tests; the writer's probes of the conditions on every matrix compiler; gap review of item O; inventory items LAY-empty-base-non-standard-layout, LAY-empty-base-same-type, LAY-overlap-attribute-empty, LAY-overlap-attribute-tail, LAY-tail-padding-base, LAY-virtual-placement |

Inputs: clang-sema-declcxx, clang-sema-decl, clang-diag-sema, edg-class-
decl.
