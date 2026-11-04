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
overload. Overloads with the same parameters therefore all have
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
:tests: classes/members/complete-in-member-bodies.cpp classes/members/complete-in-default-member-init.cpp classes/members/complete-in-default-argument.cpp classes/members/noexcept-complete-class.cpp

The class is complete inside:

- member function bodies,
- default arguments,
- default member initializers,
- the operands of `noexcept` specifiers.

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
:tests: classes/members/ref-qualified.cpp classes/members/ref-qualified-const-and-xvalue.cpp

A member function can have the ref-qualifier `&` or `&&`. A call on an
lvalue selects `&`, and a call on an rvalue selects `&&`.
:::

:::{rule} CLS-26
:kind: reject
:tests: classes/members/ref-qualified-mixed.cpp classes/members/ref-qualified-mixed-const.cpp classes/members/ref-qualified-mixed-call.cpp

The overloads of a member function that CLS-6 describes must all have
ref-qualifiers, or none, if they have the same parameters
({question}`Q-0027`).
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
:tests: classes/members/explicit-object-parameter.cpp classes/members/explicit-object-from-prvalue.cpp expressions/sequencing/explicit-object-order-unspecified.cpp

A member function can declare its first parameter as an explicit object
parameter, written `this T self`. The object of the call initializes that
parameter. If the parameter has class type and the object is a prvalue
of that type, the prvalue initializes the parameter directly, with no
temporary ({rule}`VC-4`, {question}`Q-0181`). The parameter ends
when any other parameter would ({rule}`FN-10`).
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
:tests: classes/members/static-call-operator.cpp classes/members/static-subscript-no-parameters.cpp

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
:tests: classes/members/empty-standard-layout-base-zero-size.cpp classes/members/empty-standard-layout-base-zero-size-ilp32.cpp classes/members/standard-layout-empty-bases.cpp classes/members/layout-empty-base-not-standard-layout.cpp classes/members/layout-base-type-first-member.cpp

A base subobject takes no bytes if its class is empty and
standard-layout. The derived class need not be standard-layout. The
offset of such a base is unspecified (CLS-34).
:::

## Implementation Requirements

:::{rule} CLS-11
:kind: impl-defined
:tests: classes/members/empty-class-size-one.cpp classes/members/empty-class-array-size.cpp classes/members/empty-class-size-one-ilp32.cpp

The size of an empty class is implementation-defined. Every compiler in
the matrix gives size 1, on x86-64 and with `-m32`.
:::

:::{rule} CLS-31
:kind: impl-defined
:tests: classes/members/layout-empty-base-not-standard-layout.cpp classes/members/layout-overlap-member.cpp classes/members/layout-overlap-member-ilp32.cpp declarations/attributes/no-unique-address.cpp

These parts of a class layout are implementation-defined:

- whether an object of an empty class takes no bytes, if the object is
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
:tests: classes/members/layout-base-type-first-member.cpp classes/members/layout-virtual-first-member.cpp classes/members/layout-virtual-first-member-ilp32.cpp classes/members/layout-base-tail-padding.cpp classes/members/layout-base-tail-padding-ilp32.cpp classes/members/layout-base-tail-padding-defaulted.cpp classes/members/layout-overlap-member.cpp classes/members/layout-overlap-member-ilp32.cpp classes/members/layout-order-and-pointer.cpp classes/members/layout-order-and-pointer-ilp32.cpp classes/members/layout-tail-padding-conditions.cpp classes/members/layout-tail-padding-conditions-ilp32.cpp classes/members/layout-tail-padding-deleted-constructor.cpp classes/members/layout-tail-padding-deleted-constructor-ilp32.cpp classes/members/layout-tail-padding-move-assignment.cpp classes/members/layout-tail-padding-move-assignment-ilp32.cpp classes/members/layout-empty-bases-depths.cpp classes/members/layout-empty-member-after-empty-base.cpp classes/members/layout-tail-padding-assignment-forms.cpp classes/members/layout-virtual-base-order.cpp classes/members/layout-virtual-base-part-ilp32.cpp classes/members/layout-virtual-base-part.cpp

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
:tests: classes/members/layout-order-and-pointer.cpp classes/members/layout-order-and-pointer-ilp32.cpp classes/members/layout-virtual-base-part.cpp classes/members/layout-virtual-base-part-ilp32.cpp classes/members/layout-virtual-base-order.cpp classes/members/layout-empty-member-after-empty-base.cpp classes/members/layout-empty-bases-depths.cpp classes/members/layout-tail-padding-assignment-forms.cpp classes/members/layout-virtual-first-member.cpp classes/members/layout-virtual-first-member-ilp32.cpp classes/members/layout-base-type-first-member.cpp classes/members/layout-base-tail-padding.cpp classes/members/layout-base-tail-padding-ilp32.cpp classes/members/layout-tail-padding-conditions.cpp classes/members/layout-tail-padding-conditions-ilp32.cpp classes/members/layout-base-tail-padding-defaulted.cpp classes/members/layout-tail-padding-deleted-constructor.cpp classes/members/layout-tail-padding-deleted-constructor-ilp32.cpp classes/members/layout-tail-padding-move-assignment.cpp classes/members/layout-tail-padding-move-assignment-ilp32.cpp classes/members/layout-overlap-member.cpp classes/members/layout-overlap-member-ilp32.cpp foundations/storage/empty-base-takes-no-bytes.cpp foundations/storage/base-bytes-shared-with-member.cpp foundations/storage/no-unique-address-member-shares-bytes.cpp

The matrix targets, x86-64 and `-m32`, make the choices of CLS-34 as
the table gives:

| Part of the layout | Choice |
|---|---|
| Order | The pointer for virtual calls comes first, if the class has its own. The bases that are not virtual follow in the order of the base list. Then come the members, and then the virtual bases. The virtual bases come in the order that a depth-first walk of the base lists, from left to right, first meets them. |
| Pointer for virtual calls | A class with a virtual function or a {term}`virtual base` has the pointer at offset 0. If a base that is not virtual has such a pointer, the first such base goes to offset 0, and the class uses the pointer of that base. |
| Padding | Each base and member starts at the end of the data so far, rounded up to a multiple of its alignment ({rule}`TYP-2`). The size of the class covers every subobject and is a multiple of the largest alignment of its pointer, bases, and members. |
| An empty base | The base goes to offset 0 and adds nothing to the data so far, so a later member can start at the same offset. |
| Two empty subobjects of one type | No base or member goes where it would put a subobject of an empty class type at the address of another subobject of that type. The check covers every empty subobject that the base or member holds. On a conflict, the base or member tries the end of the data so far, rounded up to its alignment, and then each next multiple of its alignment. |
| A base, or a `[[no_unique_address]]` member, of class type `C` | If `C` keeps its end padding, the subobject takes `sizeof(C)` bytes. If not, the next bases and members can start in the padding at the end of `C`. |
| A base of class type `C` with virtual bases | The base subobject holds only the pointer of `C`, the bases of `C` that are not virtual, and the members of `C`. The next bases and members can start after the last member of `C`. The complete object holds each virtual base once. |
| Any other member | The next members start after its `sizeof` bytes. |

On GCC 16, Clang 22, and Clang 18, a class keeps its end padding if all
of these conditions hold:

- The class has no base and no virtual function.
- No data member that is not static is `private` or `protected`, or has
  a default member initializer.
- The class declares no constructor template, and no user-provided
  constructor, destructor, or copy assignment operator. The copy
  assignment operator has one of the parameter forms of {rule}`CPY-3`,
  and an `operator=` template or a move assignment operator does not
  count.
- Each data member that is not static has a scalar type, a class or
  union type that keeps its end padding, or an array of such a type.

Two compilers add conditions. A class does not keep its end padding if
it also declares the member that the table names:

| Compiler | Member |
|---|---|
| GCC 14 | a constructor, also a defaulted or deleted one ({question}`Q-0342`) |
| EDG 7.0 | a deleted constructor, or a user-provided move assignment operator ({question}`Q-0361`) |
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

The results are in `tests/RESULTS.md`. All eight compilers agree on 136
of 147 tests.

- CLS-23: Clang rejects an in-class initializer for a `static const
  volatile` integer member. GCC and EDG accept it ({question}`Q-0080`).
  CLS-23 follows Clang, and the test records GCC and EDG as overrides.
- CLS-3: a missing definition of a static data member compiles in each
  translation unit. The linker reports the missing definition.
- CLS-26: GCC 16 accepts mixed ref-qualified overloads and reports
  ambiguity only at a call ({question}`Q-0027`).
  Every compiler rejects a call of the two overloads
  (`ref-qualified-mixed-call.cpp`).
- CLS-9: Clang and GCC accept an anonymous struct member as an
  extension, and reject the member only under `-pedantic-errors`.
- CLS-19, CLS-29, CLS-32: no core-language check gives a yes or no for
  standard-layout (inventory item OBS-no-core-standard-layout). The
  tests check the guarantees of CLS-30 for classes that meet the
  conditions in an unusual way, and the distinct addresses of CLS-10
  for classes that fail a condition. The probe grid
  `standard-layout.json` compares the conditions with each compiler's
  builtin query, which no test uses (decision 0051). The three
  compilers agree with each other and with CLS-19 on 34 of 38 rows. The
  other rows are routes of CLS-32:

  | Base type `E` sits at the start of the first member | Clang query | GCC query | EDG query | CLS-19 |
  |---|---|---|---|---|
  | as the first member of the first member | not standard-layout | standard-layout | standard-layout | not standard-layout |
  | as the element of an array first member | not standard-layout | standard-layout | not standard-layout | not standard-layout |
  | as a member of a union first member | not standard-layout | standard-layout | standard-layout | not standard-layout |
  | as a `[[no_unique_address]]` member in a later position | not standard-layout | standard-layout | standard-layout | not standard-layout |

  CLS-32 follows Clang and the class-properties inventory (item
  SL-first-member-type)
  ({question}`Q-0341`).
- CLS-30: during constant evaluation, Clang and GCC give `true` when
  they compare the address of a standard-layout object with the address
  of its first member or of its base, each converted to `const void*`.
  EDG gives `false`. All three give `true` for an array and its first
  element ({question}`Q-0338`). Q-0338 settled on 2026-11-03 under
  decision 0054. The source-reader gap review of item O supports its
  answer with no doubt, and CLS-P4 gives the same answer. The test
  keeps its override for EDG.
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
- CLS-35: the compilers split on three kinds of declared member. The
  table gives the size of a derived class that adds one `char` to such
  a base, where 12 means that the base keeps its end padding:

  | Member of the base | GCC 16, Clang 22, Clang 18 | GCC 14 | EDG 7.0 |
  |---|---|---|---|
  | a defaulted default constructor ({question}`Q-0342`) | 12 | 8 | 12 |
  | a deleted constructor | 12 | 8 | 8 |
  | a user-provided move assignment operator | 12 | 12 | 8 |

  The results are the same on x86-64 and with `-m32`
  ({question}`Q-0361`). Every
  compiler keeps the end padding for a defaulted or deleted destructor
  or assignment operator.
- CLS-35: every compiler gives the same sizes for classes with virtual
  bases, for an empty member after an empty base of the same type, and
  for two empty bases that hold one empty type at different depths.
  Clang and GCC give the same offsets at run time. Every compiler lends
  the end padding for each copy assignment operator form of CPY-3 and
  for a constructor template, and keeps it for an `operator=` template.
- CLS-11: the size is 1 on every compiler in the matrix, on both
  targets.
- Sources: CLS-19 and CLS-32 come from the grid `standard-layout.json`
  and the inventory items SL-access, SL-base-standard-layout,
  SL-bit-field-counts, SL-first-member-type, SL-member-standard-layout,
  SL-one-declaring-class, SL-one-subobject-per-type, SL-struct-vs-union,
  and SL-virtual. CLS-29 comes from items TYPE-array, TYPE-cv, and
  TYPE-scalar. CLS-30 comes from items SLG-base-address,
  SLG-first-member-address, and SLG-pointer-interconvertible, and from
  the tests. CLS-33 comes from item SLG-empty-base-zero, the tests, and
  the gap review of item O
  (`reports/review/2026-11-03-gap-review-item-o.md`). CLS-31, CLS-34,
  and CLS-35 come from the layout tests, the writer's probes of the
  conditions on every matrix compiler, the same gap review, and items LAY-empty-base-non-standard-layout,
  LAY-empty-base-same-type, LAY-overlap-attribute-empty,
  LAY-overlap-attribute-tail, LAY-tail-padding-base, and
  LAY-virtual-placement.
- CLS-21: when a reference binds to the member, Clang and GCC emit a
  reference to the member at `-O0` and at `-O2`, and the link fails.
  EDG translates the program and links nothing.
- CLS-13: every baseline compiler supports explicit object
  parameters, Clang 18 and GCC 14 included, and runs the tests that use
  them. Clang 18 rejects a class prvalue for a by-value explicit
  object parameter when the class has no usable copy or move
  constructor. The other compilers initialize the parameter directly
  ({question}`Q-0181`).
- CLS-14: every compiler rejects an explicit object parameter that is
  not first and one on a non-member function. Clang and GCC reject a
  default argument on an explicit object parameter, and EDG accepts it
  ({question}`Q-0317`).

Inputs: clang-sema-declcxx, clang-sema-decl, clang-diag-sema, edg-class-
decl.
