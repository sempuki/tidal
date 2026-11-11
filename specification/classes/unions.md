# Unions and bit-fields

A union is a class whose non-static data members share storage. At most
one member is active at a time. A bit-field is a data member with a width
in bits. This page covers:

- union declarations and their special members,
- initialization and the active member,
- anonymous unions,
- bit-field declarations, values, and their limits.

Depends on: class definitions and members, constructors and destructors,
copy and move, list initialization and aggregates, standard conversions.

## Principles

:::{principle} UNI-P1
:rules: UNI-1 UNI-2 UNI-4 UNI-5 UNI-6 UNI-8 UNI-13 UNI-14
:exceptions: UNI-10 UNI-12

A union is one piece of storage with several views. All members of the
union start at its first byte, and at most one member is an object at a
time:

```text
first byte
|
+--------+
| a      |                  member a
+--------+--------------+
| b                     |   member b
+-----------------------+
| c  |                      member c
+----+
```

The initializer picks the first active member. These actions make
another member active instead:

- an assignment to the member through the built-in operator or a
  trivial assignment operator,
- the construction of an object of the member's type at the member's
  address,
- a trivial copy or move of the whole union, which makes active the
  member that is active in the source.

During constant evaluation, the construction of an object inside the
member, such as one element of an array member or a member of a member
union, does not make the member active.

The assignment can
also reach a part of the member, as in `u.a.tag = 7`. A user-provided
assignment operator is a call on the member, and the member is not an
object yet. So such a call does not make the member active, and the
call uses an object that is not there. Reading a member that
is not active reads an object that is not there, so the behavior is
undefined ({principle}`OBJ-P4`). A reference is not an object with
storage of its own, so a union cannot hold a reference. A union and
each of its members are {term}`pointer-interconvertible`, so a
`reinterpret_cast` turns a pointer to one into a pointer to the other
({rule}`STG-16`).
:::

:::{principle} UNI-P2
:rules: UNI-3 UNI-11

The union does not know which member is active, so the union cannot
build, copy, or destroy its members on its own. When a member would need
such an operation, the implementation deletes the union's {term}`special member functions <special member function>`
and leaves the job to the union's own code
({principle}`CPY-P3`).
:::

:::{principle} UNI-P3
:rules: UNI-7 UNI-9

An anonymous union adds its members to the scope around it, as if the
scope held the members, while the members still share one storage. An
anonymous union can therefore hold only plain public data members and
static assertions. The anonymous union is itself a type with no name.
No declaration inside it can introduce another type, so a member whose
declaration defines an unnamed class, and an anonymous union inside it,
are not valid. At
namespace scope outside an unnamed namespace, an anonymous union needs
`static` because its members have no name to link by (UNI-9).
:::

:::{principle} UNI-P4
:rules: BF-1 BF-2 BF-3 BF-4 BF-5 BF-6 BF-7 BF-11 BF-12

A bit-field is a member with a width in bits. The width and the
declared type each decide part of its behavior:

| Property | Decided by |
|---|---|
| The range of values the bit-field holds | The width |
| The way it promotes | The width |
| Its signedness | The declared type |
| Its treatment in initialization | The declared type |

The width is an integer constant. The width has an integral or unscoped
enumeration type, or a class type whose conversion function gives such
a type. The class must be a literal type, and its conversion functions
that are not `explicit` must give exactly one such target type. Two
functions with one target type, such as a `const` and a non-`const`
overload, are allowed. The function that the conversion selects must be
`constexpr`, because the conversion runs during constant evaluation.
A class whose destructor is user-provided and not `constexpr`, such as
`~W() {}`, is not a literal type, so it fails here even when its
conversion function is `constexpr`. A floating-point width is
rejected. A scoped enumeration
converts to an integer only with a cast ({principle}`ENUM-P2`), so an
enumerator of a scoped enumeration is rejected as a width.

A bit-field need not start on a byte, so a bit-field has no address. A
program cannot point to a bit-field or bind a plain reference to it. An
unnamed bit-field only reserves bits. The bits are padding, and no
operation on the class reads or writes them. So an unnamed bit-field
takes no {term}`cv-qualifier`, and a `const` or `volatile` on it is
rejected.
:::

:::{principle} UNI-P5
:rules: BF-8 BF-9 BF-10

The implementation decides how bit-fields are packed. It also decides
the value that a bit-field holds after a value too wide for the
bit-field is stored. An implementation should warn when a width exceeds
the declared type or a constant does not fit.
:::

:::{principle} UNI-P6
:rules: UNI-10 UNI-12 UNI-15 UNI-16

Two types are *layout-compatible* when they have the same layout. Two
{term}`standard-layout` structs that start with members of layout-compatible
types place those members at the same offsets:

```text
struct Circle { int tag; int radius; };
struct Square { int tag; int side; };

offset 0        4
+---------------+----------------+
| tag           | radius / side  |
+---------------+----------------+
 common initial
 sequence
```

The common initial sequence pairs the non-static data members and
bit-fields of the two structs in declaration order, from the first. An
unnamed bit-field takes part, though it is not a member, so it can end
the sequence. A pair stays in the sequence while
all of these hold:

- The two types are layout-compatible.
- Both members are bit-fields of one width, or neither is a bit-field.
- The two members have the same alignment.
- Both members carry `[[no_unique_address]]`, or neither does.

Two structs are layout-compatible when the sequence holds all their
members.

A union of such structs can therefore read the shared start through
either struct, so a tag at the start can say which struct is active.
The read is an exception to UNI-P1 at run time only. {term}`Constant evaluation <constant evaluation>`
tracks the active member and rejects the read.
:::

## Syntax

% tidal-grammar: shape anonymous_union
```{eval-rst}
.. productionlist:: tidal
   union_def: "union" {`attribute_spec` | `alignment_specifier`} [`class_name` ["final"]] "{" {`member_decl`} "}"
   anonymous_union: ["static"] "union" "{" {`member_decl`} "}" ";"
   bitfield: [`identifier`] ":" `constant_expr` [`member_init`]
```

The *active member* of a union is the member that holds a value. A
*variant member* is a non-static data member of a union, or of an
anonymous union in a class. An *anonymous union* has no name and declares
no variable.

## Constraints

:::{rule} UNI-1
:kind: required
:tests: classes/unions/alignment-strictest-not-largest.cpp classes/unions/member-addresses-equal.cpp classes/unions/shared-storage.cpp classes/unions/union-array-member.cpp classes/unions/union-member-functions.cpp classes/unions/union-static-member.cpp classes/unions/bitfield-union-size.cpp

Each data member of a union that is not static begins at the first byte
of the union. A union is at least as large as its largest member, and at
least as aligned as its most strictly aligned member. A union can have:

- member functions,
- constructors,
- a destructor,
- static data members,
- array members.

The members of a union are public unless an access label says otherwise.
:::

:::{rule} UNI-2
:kind: reject
:tests: classes/unions/union-reference-member.cpp classes/unions/union-rvalue-reference-member.cpp classes/unions/union-final.cpp classes/unions/union-final-not-base.cpp

A union must not have a member of reference type. Rules DER-1 and VIRT-2
give the limits on bases and virtual functions. A union can carry
`final` after its name, and the marker changes nothing, because DER-1
already forbids a union as a base.
:::

:::{rule} UNI-3
:kind: required
:tests: classes/unions/union-default-member-init-enables-ctor.cpp classes/unions/union-copy-two-copy-constructors.cpp classes/unions/union-nontrivial-member-user-ctor-dtor.cpp classes/unions/union-trivial-copy.cpp classes/unions/union-deleted-move-uses-copy.cpp classes/unions/union-member-deleted-copy.cpp classes/copy-move/trivially-copyable-at-least-one.cpp classes/copy-move/trivially-copyable-const-member.cpp classes/copy-move/trivially-copyable-constraint-fails.cpp classes/copy-move/trivially-copyable-defaulted-later.cpp classes/copy-move/trivially-copyable-deleted-destructor.cpp classes/copy-move/trivially-copyable-member-selected.cpp templates/constraints/destructor-trivial-by-concept.cpp

If a variant member has one of these non-trivial functions (CTOR-23,
DTOR-9, CPY-27), the matching special member of the union is deleted.
For a copy or move function, the function that the union's own copy or
move selects for the member decides
({question}`Q-0339`):

- a {term}`default constructor`,
- a copy or move function,
- a destructor.

The same rule applies to a class with an anonymous union member. A
default member initializer on a variant member gives the union a usable
default constructor. {rule}`CPY-7` also applies to variant members, so a
deleted or inaccessible copy function of a member deletes that of the
union.
:::

:::{rule} UNI-11
:kind: reject
:tests: classes/unions/anonymous-union-nontrivial-default.cpp classes/unions/union-nontrivial-copy-deleted.cpp classes/unions/union-nontrivial-dtor-only.cpp classes/unions/union-nontrivial-member-deleted-ctor.cpp classes/unions/union-member-deleted-copy.cpp classes/constructors/trivial-destructor-defaulted.cpp classes/constructors/trivial-destructor-member-nontrivial.cpp classes/constructors/trivial-destructor-user-provided.cpp classes/constructors/trivial-destructor-virtual-destructor.cpp classes/copy-move/trivial-assign-implicit.cpp classes/copy-move/trivial-assign-user-provided.cpp classes/copy-move/trivial-copy-defaulted.cpp classes/copy-move/trivial-copy-implicit.cpp classes/copy-move/trivial-copy-member-initializer.cpp classes/copy-move/trivial-copy-member-nontrivial.cpp classes/copy-move/trivial-copy-user-provided.cpp classes/copy-move/trivial-copy-virtual-base.cpp classes/copy-move/trivial-copy-virtual-function.cpp classes/copy-move/trivial-move-user-provided.cpp classes/copy-move/trivially-copyable-member-template-selected.cpp classes/copy-move/trivial-copy-array-member-nontrivial.cpp

A program must not use a special member function that UNI-3 deletes. The
union must then declare that function itself.
:::

:::{rule} UNI-4
:kind: required
:tests: classes/unions/aggregate-first-member.cpp classes/unions/default-member-init-active.cpp classes/unions/designated-member.cpp classes/unions/value-init-zero.cpp classes/unions/default-init-no-active-member.cpp classes/unions/anonymous-union-mem-init-constructor.cpp classes/unions/anonymous-union-designated-member.cpp

A union is initialized as one of its members:

- A braced list without designators initializes the first member.
- A designator such as `{.f = 2.0f}` selects a member.
- A member with a default member initializer is used if there is no
  other initializer.
- Value initialization `{}` initializes the first member to zero, if no
  member has a default member initializer.
- If no member has a default member initializer,
  {term}`default initialization` initializes no member. The union then
  has no active member until an assignment makes one active (UNI-5).

The initialized member is then active. Default initialization also
applies to a union member that a constructor does not initialize
({rule}`INIT-9`).
:::

:::{rule} UNI-8
:kind: reject
:tests: classes/unions/mem-init-two-members.cpp classes/unions/narrowing-first-member.cpp classes/unions/two-default-member-inits.cpp classes/unions/two-designators.cpp classes/unions/two-initializers.cpp

Initialization of a union must name at most one member. A program must
not give:

- two initializers or two designators in a braced list,
- default member initializers to two variant members,
- `mem_init` entries for two variant members.

A braced initializer for the first member must not narrow.
:::

:::{rule} UNI-7
:kind: required
:tests: classes/unions/anonymous-union-default-member-init.cpp classes/unions/anonymous-union-mem-init-constructor.cpp classes/unions/anonymous-union-member-order.cpp classes/unions/anonymous-union-init.cpp classes/unions/anonymous-union-local.cpp classes/unions/anonymous-union-member.cpp classes/unions/anonymous-union-namespace-static.cpp classes/unions/anonymous-union-brace-elision.cpp classes/unions/anonymous-union-designated-member.cpp classes/unions/anonymous-union-two-designators.cpp

The members of an anonymous union are members of the enclosing scope. In
a class, the anonymous union is an unnamed data member in its place in
declaration order, and its members are variant members of the class. A
`mem_init` or a default member initializer of a variant member
initializes the union with that member active (UNI-4). In a braced
list for the class, the anonymous union is one element. Its initializer
can have its own braces, or the braces can be left out, so `{1, 2, 3}`
gives the union the value 2 for its first member. A designator can also
name a member of the anonymous union directly, as in `{.level = 2.0f}`,
and the union is then initialized with that member active. Because the
union is one element, one list can name only one of its members with a
designator. A second designator for a member of the same anonymous
union is not valid ({rule}`LIST-14`). An anonymous union can be:

- local,
- a class member,
- `static` at namespace scope.
:::

:::{rule} UNI-9
:kind: reject
:tests: classes/unions/anonymous-union-function.cpp classes/unions/anonymous-union-name-conflict.cpp classes/unions/anonymous-union-namespace-nonstatic.cpp classes/unions/anonymous-union-nested-type.cpp classes/unions/anonymous-union-private.cpp classes/unions/anonymous-union-static-member.cpp classes/unions/anonymous-union-unnamed-namespace.cpp classes/unions/anonymous-union-unnamed-struct-member.cpp classes/unions/anonymous-union-nested-anonymous-union.cpp classes/unions/anonymous-union-static-assertion.cpp

At namespace scope outside an unnamed namespace, an anonymous union
needs `static`, as {rule}`STO-11` gives ({question}`Q-0045`). The members of the anonymous
union must all be public data members that are not static. An anonymous
union must not have:

- member functions,
- static data members,
- nested types, also an unnamed class that a member declaration
  defines, as in `struct { int key; } pair;`, and an anonymous union
  ({question}`Q-0424`),
- a member with a name that the enclosing scope has.

An anonymous union can contain a `static_assert` declaration.
:::

:::{rule} BF-1
:kind: required
:tests: classes/unions/bitfield-basic.cpp classes/unions/bitfield-bool.cpp classes/unions/bitfield-char-types.cpp classes/unions/bitfield-constant-width.cpp classes/unions/bitfield-enum.cpp classes/unions/bitfield-in-union.cpp classes/unions/bitfield-zero-unnamed.cpp classes/unions/bitfield-width-unscoped-enumeration.cpp classes/unions/bitfield-width-class-converts.cpp classes/unions/bitfield-width-class-explicit-conversion.cpp classes/unions/bitfield-width-class-two-conversions-same-type.cpp

A bit-field has an integral or enumeration type and a width. The width
is a constant expression of integral or unscoped enumeration type, or of
a class type that converts to such a type ({question}`Q-0314`). Such a
class is a literal type, and its conversion functions that are not
`explicit` give exactly one integral or unscoped enumeration type as
their target. Several functions can give that one type. The function
that the conversion selects is `constexpr`. An `explicit` conversion
function does not count. A
bit-field can be unnamed. An unnamed bit-field of width 0 makes the next
bit-field start at a new allocation unit. A union can have bit-fields.
:::

:::{rule} BF-2
:kind: reject
:tests: classes/unions/bitfield-float.cpp classes/unions/bitfield-negative-width.cpp classes/unions/bitfield-nonconstant-width.cpp classes/unions/bitfield-pointer.cpp classes/unions/bitfield-static.cpp classes/unions/bitfield-typedef.cpp classes/unions/bitfield-unnamed-cv.cpp classes/unions/bitfield-zero-named.cpp classes/unions/bitfield-width-floating.cpp classes/unions/bitfield-width-scoped-enumeration.cpp classes/unions/bitfield-width-class-not-constexpr.cpp classes/unions/bitfield-width-class-two-conversions.cpp classes/unions/bitfield-width-class-not-literal.cpp

A bit-field must not:

- have a type that is not integral or an enumeration,
- be `static` or a `typedef`,
- have a negative width or a width that is not a constant,
- have a width of a type that BF-1 does not allow, such as a
  floating-point type or a scoped enumeration ({question}`Q-0313`),
- have width 0 and a name.

An unnamed bit-field must not have a {term}`cv-qualifier` ({question}`Q-0058`).
:::

:::{rule} BF-3
:kind: required
:tests: classes/unions/bitfield-bool-wide.cpp classes/unions/bitfield-very-wide.cpp classes/unions/bitfield-wider-than-type.cpp classes/unions/bitfield-wide-layout-ilp32.cpp classes/unions/bitfield-wide-layout.cpp

A width can be larger than the number of bits in the type, including for
`bool`. The extra bits are padding. The value range stays that of the
type.
:::

:::{rule} BF-4
:kind: reject
:tests: classes/unions/bitfield-address.cpp classes/unions/bitfield-alignof-type-ok.cpp classes/unions/bitfield-const-ref-temp.cpp classes/unions/bitfield-nonconst-ref.cpp classes/unions/bitfield-sizeof.cpp classes/unions/bitfield-const-volatile-ref.cpp

A program must not:

- take the address of a bit-field,
- apply `sizeof` to it,
- bind a non-const reference to it.

A `const` reference that is not `volatile` binds to a temporary that
holds a copy of the value ({rule}`REF-2`). A `const volatile` reference
does not bind to a bit-field ({rule}`VOL-10`).
:::

:::{rule} BF-6
:kind: required
:tests: classes/unions/bitfield-auto-type.cpp classes/unions/bitfield-promotes-to-int.cpp classes/unions/bitfield-promotes-unsigned-int.cpp classes/unions/bitfield-wide-unsigned-promotion.cpp foundations/conversions/bit-field-enumeration-promotes-as-enumeration.cpp

A bit-field of integer type promotes as follows. A bit-field of an
unscoped enumeration type promotes as its enumeration does
({rule}`CONV-1`).

1. If `int` can hold all values of the bit-field's width, the bit-field
   promotes to `int`.
2. If not, the bit-field promotes to `unsigned int` if `unsigned int`
   can hold those values.
3. If neither can, the promotion uses the declared type of the
   bit-field, as for other integers.

The type of the bit-field for `auto` {term}`deduction` is the declared type.
:::

:::{rule} BF-7
:kind: required
:tests: classes/unions/bitfield-aggregate-skips-unnamed.cpp classes/unions/bitfield-constant-fits.cpp classes/unions/bitfield-constant-too-wide.cpp classes/unions/bitfield-init-default-member.cpp classes/unions/bitfield-unnamed-not-member.cpp

A named bit-field can have a default member initializer, with `=` or
braces. Aggregate initialization skips unnamed bit-fields. For a
bit-field target, the test for a {term}`narrowing conversion` uses the
declared type, not the width. A bit-field source counts by its width
({rule}`CONV-21`).
:::

:::{rule} BF-11
:kind: reject
:tests: classes/unions/bitfield-narrowing-init.cpp classes/unions/bitfield-narrowing-double.cpp

A braced initializer for a bit-field must not need a
{term}`narrowing conversion` to the declared type of the bit-field.
:::

:::{rule} BF-12
:kind: required
:tests: classes/unions/bitfield-unnamed-not-member.cpp classes/unions/bitfield-unnamed-value-init-copy.cpp classes/unions/bitfield-aggregate-skips-unnamed.cpp

An unnamed bit-field reserves bits of padding ({rule}`STG-11`) and
declares no member. These operations skip it:

- aggregate initialization (BF-7) and structured bindings,
- the implicit copy and move functions ({rule}`CPY-8`),
- a defaulted comparison.
:::

:::{rule} UNI-12
:kind: reject
:tests: classes/unions/common-initial-sequence-constant.cpp classes/unions/common-initial-sequence-second-member-constant.cpp

Constant evaluation does not permit a read through the other struct that
UNI-10 describes.
:::

## Runtime Behavior

:::{rule} UNI-5
:kind: required
:tests: classes/unions/assign-changes-active.cpp classes/unions/nontrivial-member-needs-construction.cpp classes/unions/union-copy-preserves-active.cpp classes/unions/default-init-no-active-member.cpp foundations/storage/implicit-union-member-assignment.cpp

An assignment to a member makes that member active if the assignment
uses the built-in operator or a trivial assignment operator
({rule}`CPY-27`). A copy of a union copies its active member. Constant
evaluation follows these rules.
:::

:::{rule} UNI-13
:kind: required
:tests: classes/unions/chain-assign-activates.cpp classes/unions/common-initial-sequence.cpp classes/unions/chain-assign-other-part-no-value-constant.cpp classes/unions/chain-assign-nontrivial-constant.cpp classes/unions/chain-assign-member-function.cpp classes/unions/chain-assign-member-function-constant.cpp classes/unions/chain-assign-array-element-nontrivial-constant.cpp

The left operand of an assignment can name its object through a chain
of member accesses and built-in subscripts, such as `s.u.a.tag` or
`u.arr[1]`. If the assignment uses the built-in operator or a trivial
assignment operator, each union member in the chain that is not active
becomes active. These conditions
apply:

- A member becomes active only if its default constructor is trivial
  ({rule}`CTOR-23`). A default member initializer in the member's class
  makes the default constructor non-trivial.
- The parts of a new active member that the assignment does not write
  have no value. A read of such a part is undefined ({rule}`OBJ-2`).

Constant evaluation follows this rule.
:::

:::{rule} UNI-14
:kind: required
:tests: classes/unions/construct-activates-constant.cpp classes/unions/construct-switches-active-constant.cpp classes/unions/placement-new-activates.cpp classes/unions/slot-array-constant.cpp classes/unions/destroyed-member-read-constant.cpp classes/unions/array-member-element-address-no-active-member.cpp classes/unions/construct-in-member-of-inactive-member-union-constant.cpp classes/unions/placement-into-union-member-of-new-array-element.cpp classes/unions/array-member-element-steps-no-active-member-constant.cpp classes/unions/construct-at-array-member-element-constant.cpp classes/unions/construct-in-member-of-inactive-member-union-run.cpp classes/unions/trivial-copy-changes-active-member-constant.cpp classes/unions/trivial-move-changes-active-member-constant.cpp

If placement `new` ({rule}`STG-5`, with a form that {rule}`LSUP-7`
gives) or `std::construct_at` ({rule}`LSUP-10`) creates an object of a member's type at the address of
the member, the new object is that member, and the member becomes
active. The rule holds when the union has no active member, and when
another member is active. An explicit destructor call on the active
member ({rule}`DTOR-6`) leaves the union with no active member. A
trivial copy or move constructor, or a trivial copy or move assignment,
of the whole union makes active in the target the member that is active
in the source.

During {term}`constant evaluation`, a new object inside a member that
is not active, such as an element of an array member or a member of a
member union, does not make the member active ({question}`Q-0425`). At
run time, the construction is valid, and the program can use the new
object through the pointer that the construction gives. A program can
still form pointers to the elements of an array member that is not
active, step between them, and compare them ({rule}`STG-10`).
:::

:::{rule} BF-5
:kind: required
:tests: classes/unions/bitfield-plain-int-signed.cpp classes/unions/bitfield-signed-long-long-negative.cpp

A bit-field of a signed type, including plain `int`, holds negative
values that fit in its width.
:::

:::{rule} UNI-10
:kind: required
:tests: classes/unions/common-initial-sequence.cpp classes/unions/common-initial-sequence-cv.cpp classes/unions/common-initial-sequence-enum.cpp classes/unions/common-initial-sequence-bit-field.cpp classes/unions/common-initial-sequence-nested.cpp classes/unions/common-initial-sequence-inherited.cpp

A standard-layout union ({rule}`CLS-19`) can have two members that are
standard-layout classes and not unions. While one of them is active, a
program can read a member of the other one if the member lies in the
{term}`common initial sequence` of the two classes (UNI-15). The member
can also be a member of such a member, at any depth, when the two
enclosing members are of layout-compatible types (UNI-16). The read
gives the value of the matching member of the active class. If that
matching member is volatile and the member read is not, the read
accesses a volatile object through a glvalue that is not volatile, and
the behavior is undefined ({rule}`VOL-6`).
:::

:::{rule} UNI-15
:kind: required
:tests: classes/unions/common-initial-sequence.cpp classes/unions/common-initial-sequence-cv.cpp classes/unions/common-initial-sequence-bit-field.cpp classes/unions/common-initial-sequence-inherited.cpp classes/unions/common-initial-sequence-nested.cpp classes/unions/common-initial-sequence-enum.cpp

Take two standard-layout classes that are not unions. Their *common
initial sequence* is the longest run of their non-static data members
and bit-fields, from the first in declaration order, in which each
matching pair meets all of these conditions:

- The two types are {term}`layout-compatible` (UNI-16).
- Either both are bit-fields with equal widths, or both are ordinary
  members.
- Both have the same alignment requirement.
- Both have `[[no_unique_address]]`, or neither has it
  ({question}`Q-0340`).

The members of a class are the members of the one class that declares
them ({rule}`CLS-19`), so they can come from a base. This reading is
provisional ({question}`Q-0345`).
:::

:::{rule} UNI-16
:kind: required
:tests: classes/unions/common-initial-sequence-nested.cpp classes/unions/common-initial-sequence-enum.cpp classes/unions/common-initial-sequence-cv.cpp

Two types are {term}`layout-compatible` in these cases:

- They are the same type, apart from cv-qualifiers.
- They are enumerations with the same underlying type.
- They are standard-layout classes that are not unions, and their common
  initial sequence (UNI-15) holds all their members and bit-fields.
- They are standard-layout unions with the same number of members, and
  their member types pair up as layout-compatible in some order.

Two array types are layout-compatible only if they are the same type.
:::

## Undefined Behavior

:::{rule} UNI-6
:kind: undefined
:tests: classes/unions/chain-assign-nontrivial-constant.cpp classes/unions/compound-assign-inactive-constant.cpp classes/unions/read-inactive-after-switch-constant.cpp classes/unions/read-inactive-constant.cpp classes/unions/user-assignment-not-activating-constant.cpp classes/unions/default-init-read-constant.cpp classes/unions/constructor-leaves-union-read-constant.cpp classes/unions/destroyed-member-read-constant.cpp classes/unions/read-inactive-bool-sanitizer.cpp classes/unions/construct-at-array-member-element-constant.cpp classes/unions/chain-assign-array-element-nontrivial-constant.cpp classes/unions/construct-in-member-of-inactive-member-union-constant.cpp

If a program reads a union member that is not active, the behavior is
undefined. During constant evaluation, {rule}`CE-6` rejects the read.
A compound assignment to a member that is not active reads
that member. An assignment to a member that is not active, through an
assignment operator that is user-provided, uses an object that does not
exist yet ({question}`Q-0088`). An assignment through a chain of member
accesses ({rule}`UNI-13`) to a union member whose default constructor is
not trivial also uses an object that does not exist
({question}`Q-0213`). During constant evaluation, a construction of an
object inside a member that is not active ({rule}`UNI-14`) uses a
member that does not exist ({question}`Q-0425`).
:::

## Implementation Requirements

:::{rule} BF-9
:kind: impl-defined
:tests: classes/unions/bitfield-layout-lp64.cpp classes/unions/bitfield-wide-layout.cpp classes/unions/bitfield-wide-layout-ilp32.cpp classes/unions/bitfield-wide-alignment-128.cpp classes/unions/bitfield-union-size.cpp classes/unions/bitfield-unnamed-alignment.cpp classes/unions/bitfield-unnamed-alignment-ilp32.cpp classes/unions/bitfield-zero-width-splits.cpp classes/unions/bitfield-low-order-first.cpp classes/unions/bitfield-next-unit-lp64.cpp classes/unions/bitfield-next-unit-ilp32.cpp

The allocation of bit-fields in an object is implementation-defined. The
matrix targets, x86-64 and `-m32`, place a bit-field of type `T` in this
way:

- Bit-fields fill the bits from the lowest-order bit up, in the order
  of their declarations, and other members share the bytes.
- The bit-field takes the next free bits if they lie in one run of
  `sizeof(T)` bytes that starts at a multiple of `alignof(T)`
  ({rule}`TYP-2`, {question}`Q-0120`).
- If not, the bit-field starts at the next multiple of `alignof(T)`.
- A bit-field whose width `N` is more than `8 * sizeof(T)` starts at
  the next multiple of `alignof(U)`. `U` is the largest integer type
  with at most `N` bits, and the class gets the alignment of `U`. On
  x86-64, GCC also counts its 128-bit integer type as `U`.
- The first `8 * sizeof(T)` bits of such a bit-field hold the value,
  and the later bits are padding (BF-3).
- In a union, a bit-field starts at byte 0 and takes the bytes that its
  width needs. The union gets the alignment of `T`, or of `U`.
- A bit that no named bit-field or other member covers is a padding
  bit ({rule}`STG-11`, {question}`Q-0372`). An unnamed bit-field is not
  a member, so its bits are padding bits.
- A named bit-field of type `T` gives the class at least the alignment
  of `T`, as a member of type `T` does ({rule}`ALN-1`).
- An unnamed bit-field does not change the alignment of the class. An
  unnamed bit-field of width 0 and type `T` moves the next member to the
  next multiple of `alignof(T)`, and the class covers that position.
:::

:::{rule} BF-10
:kind: impl-defined
:tests: classes/unions/bitfield-unsigned-wraps.cpp classes/unions/bitfield-signed-wraps.cpp classes/unions/bitfield-constant-wraps.cpp classes/unions/bitfield-constant-wraps-ilp32.cpp

If a value stored in a bit-field of width `n` does not fit, the value
that the bit-field holds is implementation-defined. On the x86-64 targets
in the matrix, the bit-field holds the value in its range that differs
from the stored value by a multiple of 2^n. The same holds during
constant evaluation.
:::

## Recommended Practice

:::{rule} BF-8
:kind: recommended
:tests: classes/unions/bitfield-constant-truncation-warns.cpp classes/unions/bitfield-fits-no-warning.cpp classes/unions/bitfield-wider-warns.cpp classes/unions/bitfield-assign-too-wide-warns.cpp classes/unions/bitfield-wider-warns-default.cpp

An implementation should warn if a bit-field width is larger than the
declared type, or if a constant assignment changes value because of the
width.
:::

## Examples

```cpp
struct Token {
  enum class Kind : unsigned char { number, text } kind;
  union {                        // anonymous: members of Token
    long number;
    const char* text;
  };
};
Token t{Token::Kind::number, {42}};

union Bits { unsigned u; float f; };
Bits b{.f = 1.0f};               // f is active
unsigned x = b.u;                // undefined: u is not active

struct Pixel {
  unsigned red : 5;
  unsigned green : 6;
  unsigned blue : 5;             // one 32-bit unit on x86-64
};
Pixel p{31, 70, 0};              // 70 is reduced modulo 64

struct Level { int value : 3; }; // holds -4 to 3
int store(int v) { Level l{}; l.value = v; return l.value; }
                                 // store(5) is -3, which is 5 - 8 (BF-10)

struct Header {
  unsigned version : 4;          // bits 0-3: the low half of byte 0
  unsigned length : 4;           // bits 4-7
  unsigned short id : 12;        // bits 16-27: bits 8-19 would cross
};                               // a 2-byte run of unsigned short
Header h{3, 5, 1};               // byte 0 is 0x53, and byte 2 is 0x01

struct Status {
  unsigned ready : 1;
  unsigned : 3;                  // three bits of padding
  unsigned mode : 4;
};
Status s{1, 2};                  // ready = 1, mode = 2 (BF-12)
auto [ready, mode] = s;          // two names for two members
```

An assignment through a chain makes a member active only if the
member's default constructor is trivial (UNI-13):

```cpp
struct Pair { int key; int value; };
struct Counter { int n = 0; };   // the default constructor is not trivial
union Cell {
  Pair pair;
  Counter counter;
  int raw;
  constexpr Cell() : raw(0) {}
};
constexpr int use() {
  Cell cell;
  cell.pair.key = 1;             // pair becomes active
  int k = cell.pair.key;         // 1
  int v = cell.pair.value;       // undefined: value has no value
  cell.counter.n = 2;            // undefined: counter does not become active
  return k + v;
}
```

A union slot holds an object of a type that has no default
constructor. The slot starts with no active member, `std::construct_at`
fills it, and an explicit destructor call empties it (UNI-14):

% tidal-example: fragment
```cpp
template <class T, int Capacity>
class FixedStack {
  union Slot {                   // T can lack a default constructor
    T value;
    constexpr Slot() {}          // no active member
    constexpr ~Slot() {}         // does not destroy value
  };
  Slot slots[Capacity];
  int count = 0;
public:
  constexpr FixedStack() {}
  constexpr ~FixedStack() { while (count > 0) pop(); }
  template <class... Args>
  constexpr void push(Args&&... args) {
    std::construct_at(&slots[count].value, static_cast<Args&&>(args)...);
    ++count;                     // value is now the active member
  }
  constexpr void pop() { --count; slots[count].value.~T(); }
  constexpr T& top() { return slots[count - 1].value; }
};
```

`std::construct_at` is declared as {rule}`LSUP-10` gives. A plain array
`T items[Capacity]` would need a default constructor for `T`.

A member of a member can be read through the other class when the
enclosing members are layout-compatible (UNI-10):

```cpp
struct Point { int x; int y; };
struct Pair { int first; int second; };
struct Line { Point from; int width; };
struct Span { Pair range; int step; };
union Shape { Line line; Span span; };
Shape s{.line = {{1, 2}, 3}};
int y = s.span.range.second;     // 2: Point and Pair are layout-compatible
```

A class with two alternatives keeps a tag and an anonymous union, and
has one constructor for each alternative. A defaulted `==` is deleted
for a class with an anonymous union member ({rule}`DCMP-2`), so the
class writes `==` and reads only the active members:

```cpp
struct Error { int code; };
template <class T> class Result {
  bool ok_;
  union { T value_; Error error_; };
public:
  constexpr Result(T v) : ok_(true), value_(v) {}
  constexpr Result(Error e) : ok_(false), error_(e) {}
  constexpr bool ok() const { return ok_; }
  friend constexpr bool operator==(const Result& a, const Result& b) {
    if (a.ok_ != b.ok_) return false;
    return a.ok_ ? a.value_ == b.value_ : a.error_.code == b.error_.code;
  }
};
static_assert(Result<int>(3) == Result<int>(3));
static_assert(!(Result<int>(3) == Result<int>(Error{3})));
```

For a `T` with a destructor or a copy function that is not trivial, the
matching special member of the class is deleted ({rule}`UNI-3`). The
class then writes that member, such as a destructor that destroys the
active member.

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 159
of 179 tests.

### Divergences

Each settled question below keeps its test's override for the family
that still differs.

- {question}`Q-0339` (UNI-3, CPY-27; open): whether a member with a
  trivial and a user-provided copy constructor makes the copy of the
  union non-trivial when the copy selects the trivial one. The selected
  constructor decides, so the union's copy constructor stays, as Clang
  says. GCC and EDG delete it.
- {question}`Q-0088` (UNI-6; settled on UNI-P1): whether an assignment
  through a user-provided operator makes an inactive member active
  during constant evaluation. It does not, so constant evaluation
  rejects it, as Clang and GCC say. EDG 7.0 accepts the assignment.
- {question}`Q-0213` (UNI-6, UNI-13; compiler defect): whether an
  assignment through a chain makes active a member whose default
  constructor is not trivial. It does not, as Clang and GCC say. EDG
  accepts the assignment.
- {question}`Q-0045` (UNI-9; open): whether an anonymous union in an
  unnamed namespace must be declared `static`. It need not, as Clang
  says. GCC and EDG reject the declaration.
- {question}`Q-0309` (UNI-14; compiler defect): whether constant
  evaluation must reject a read of a union member after an explicit
  destructor call ends it. It must, as Clang and EDG say. GCC accepts
  the read and gives 0.
- {question}`Q-0340` (UNI-15, UNI-16; settled): whether two structs are
  layout-compatible when matching members both have
  `[[no_unique_address]]`. They are, as GCC's builtin query says.
  Clang 22's query says they are not. No test shows the property.
- {question}`Q-0345` (UNI-15, UNI-16; open): whether the members that
  a base declares count in the common initial sequence. They count, as
  the builtin queries of Clang 22 and GCC say. The gap review of item O
  (`reports/review/2026-11-03-gap-review-item-o.md`) finds the reference
  silent. No family differs in the run test.
- {question}`Q-0314` (BF-1; settled on UNI-P4): whether a constant of
  class type with a `constexpr` conversion to `int` gives a bit-field
  width. It does, as Clang and EDG say. GCC 16 rejects it.
- {question}`Q-0313` (BF-2; settled on UNI-P4): whether an enumerator of
  a scoped enumeration gives a bit-field width. It does not, as Clang
  and GCC say. EDG 7.0 converts it.
- {question}`Q-0058` (BF-2; settled on UNI-P4): whether an unnamed
  bit-field can have a cv-qualifier. It cannot, as Clang and EDG say.
  GCC 16 accepts it.
- {question}`Q-0120` (BF-9; settled on {principle}`TYP-P3`): whether
  EDG's 8-byte alignment of `long long` with `-m32` is valid. It is,
  since the alignment is implementation-defined. EDG with `-m32` starts
  a `long long` bit-field after 40 bits at bit 64, and Clang and GCC at
  bit 40.
- {question}`Q-0372` (BF-9; open): what a byte read gives for the bits
  of a byte that no bit-field covers. The bits are padding bits with
  unspecified values, and the read is defined. Clang and GCC at `-O0`
  leave the bits as the stack held them. The gap review of the rules
  that phase 7 rewrote
  (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`) reads the
  reference as leaving the value of the byte open.
- {question}`Q-0424` (UNI-9; open):
  whether a member whose type is an unnamed class defined in its own
  declaration counts as a nested type of an anonymous union. It does,
  as EDG and Clang under `-pedantic-errors` say. GCC accepts the
  program, and also an anonymous union inside an anonymous union.
- {question}`Q-0425` (UNI-14, UNI-6, CE-6; open):
  whether building an object inside a union member that is not
  active, as a member of a member union or an element of an array
  member, makes the member active. It does not, so constant evaluation
  rejects the construction, as Clang and GCC say for a member union and
  Clang and GCC 14 say for an array member. EDG accepts both forms, and
  GCC 16 accepts the array form. At run time the construction is
  valid, and Clang and GCC run the member-union form.

### Split tests

EDG runs no programs, so its cell for a run test gives the result of
the compile. A test with `-m32` runs on EDG only in its i686
configuration.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `anonymous-union-nested-anonymous-union.cpp` | UNI-9 | reject | reject | accept | reject | {question}`Q-0424` |
| `anonymous-union-unnamed-namespace.cpp` | UNI-9 | accept | accept | reject | reject | {question}`Q-0045` |
| `anonymous-union-unnamed-struct-member.cpp` | UNI-9 | reject | reject | accept | reject | {question}`Q-0424` |
| `bitfield-next-unit-ilp32.cpp` | BF-9 | accept | accept | accept | reject | {question}`Q-0120` |
| `bitfield-unnamed-cv.cpp` | BF-2 | reject | reject | accept | reject | {question}`Q-0058` |
| `bitfield-wide-alignment-128.cpp` | BF-9 | accept | accept | reject | accept | |
| `bitfield-wide-layout-ilp32.cpp` | BF-9, BF-3 | accept | accept | accept | reject | {question}`Q-0120` |
| `bitfield-width-class-converts.cpp` | BF-1 | accept | accept | reject | accept | {question}`Q-0314` |
| `bitfield-width-class-explicit-conversion.cpp` | BF-1 | accept | accept | reject | accept | {question}`Q-0314` |
| `bitfield-width-class-two-conversions-same-type.cpp` | BF-1 | accept | accept | reject | accept | {question}`Q-0314` |
| `bitfield-width-scoped-enumeration.cpp` | BF-2 | reject | reject | reject | accept | {question}`Q-0313` |
| `chain-assign-array-element-nontrivial-constant.cpp` | UNI-6, UNI-13 | reject | reject | reject | accept | {question}`Q-0213` |
| `chain-assign-nontrivial-constant.cpp` | UNI-6, UNI-13 | reject | reject | reject | accept | {question}`Q-0213` |
| `classes/copy-move/trivially-copyable-member-selected.cpp` | CPY-27, CPY-19, UNI-3 | accept | accept | reject | reject | {question}`Q-0339` |
| `classes/copy-move/trivially-copyable-member-template-selected.cpp` | CPY-27, UNI-11 | reject | reject | reject | accept | {question}`Q-0339` |
| `construct-at-array-member-element-constant.cpp` | UNI-14, UNI-6 | reject | reject | GCC 16: accept; GCC 14: reject | accept | {question}`Q-0425` |
| `construct-in-member-of-inactive-member-union-constant.cpp` | UNI-14, UNI-6, CE-6 | reject | reject | reject | accept | {question}`Q-0425` |
| `destroyed-member-read-constant.cpp` | UNI-14, UNI-6 | reject | reject | accept | reject | {question}`Q-0309` |
| `union-copy-two-copy-constructors.cpp` | UNI-3, CPY-27 | accept | accept | reject | reject | {question}`Q-0339` |
| `user-assignment-not-activating-constant.cpp` | UNI-6 | reject | reject | reject | accept | {question}`Q-0088` |

### Notes

- BF-1, BF-2: every compiler rejects a width of floating-point type.
- BF-8: Clang 18 and 22 and GCC 14 and 16 give both warnings by
  default, without `-Wall`, so `-Werror` alone turns them into errors.
  `-pedantic-errors` does not turn them into errors.
- BF-9: Clang and GCC give the same layouts on x86-64 and with `-m32`.
- BF-9, wide bit-fields: every compiler gives the same sizes for
  bit-fields of 40, 64, 65, 128, and 1000 bits, and the same
  alignments up to 64 bits. From 128 bits, GCC aligns the class to 16
  on x86-64, and Clang and EDG align it to 8. In a union, every
  compiler gives a bit-field the bytes that its width needs.
- BF-9, padding bits: the gap review of the rules that phase 7
  rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`)
  reads the reference as making the bits of an unnamed bit-field
  padding bits, as BF-9 now says ({question}`Q-0372`).
- BF-9: Clang, GCC, and EDG give the same sizes and alignments for
  unnamed bit-fields on x86-64 and with `-m32`.
- BF-9, BF-10 with `-m32`: the matrix runs no program built with
  `-m32`, because the host compilers have no 32-bit C library. In the
  writer's probes, the run tests of the two rules give the same
  results with `-m32` in the GCC 14, Clang 18, and Clang 22 containers.
- BF-12: Clang, GCC, and EDG accept a constant evaluation that copies
  and compares a class with an unnamed bit-field, and a structured
  binding that has one name for each named bit-field.
- UNI-15, UNI-P6: the builtin query of Clang and GCC finds two structs
  not layout-compatible when one has an unnamed bit-field that the
  other lacks, of width 0 or not. EDG has no such query. No test checks
  the case, because tests observe class properties only through
  behavior (decision 0051), and a read of the other struct's member
  past the unnamed bit-field has no defined result to observe.
- UNI-7: every compiler accepts a braced list for a class that leaves
  out the braces of its anonymous union, and a designator that names a
  member of the anonymous union, with or without the union's braces. Every
  compiler rejects a list with two designators that name members of one
  anonymous union.
- UNI-6: for a read from a union with no active member, GCC and EDG
  call the member uninitialized. Clang says that the union has no
  active member. For an assignment through a chain that does not make
  the member active (UNI-13), GCC says that the program accesses the
  member instead of the member that is active.
- UNI-6: the undefined sanitizer of GCC and Clang stops a read of an
  inactive `bool` member whose bytes hold no valid `bool` value. No
  sanitizer stops a read whose bytes give a valid value.
- UNI-14: Clang and GCC accept the constant evaluations in which
  `std::construct_at` makes a member active, and run the placement
  `new` form with no sanitizer report. The gap review of the text phase
  13 wrote on classes
  (`reports/review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md`,
  RV6-A-13) reads the C++23 allowance for `std::construct_at` in
  constant evaluation as naming only objects whose lifetime began, and
  a member that is not active has not begun. UNI-14 follows the
  compilers, which read the lifetime of the union as enough.
- UNI-14, UNI-6: at run time, the same review reads a construction
  inside a member that is not active as valid. The new object reuses
  the storage of the union ({rule}`STG-6`), or, when a whole object of
  the member's type fits at the member's address, the member becomes
  active and holds the new object. No test tells the two outcomes
  apart. The run test reads the new object only through the pointer
  that `new` returns, which is valid in both.
- UNI-14, UNI-P1: every compiler keeps the active member through a
  trivial copy of the union in constant evaluation, and rejects a read
  of the member that was active in the target before a copy assignment.
- UNI-14, STG-10: every compiler accepts a constant evaluation that
  steps a pointer between the elements of an array member that is not
  active, subtracts two such pointers, and compares them
  (`array-member-element-steps-no-active-member-constant.cpp`).
- UNI-13: every compiler accepts the constant evaluations in which an
  assignment through `.` and `[]` makes members of one or two unions
  active. Every compiler rejects a later read of a part that the
  assignment did not write.
- UNI-15, UNI-16: no core-language check shows whether two types are
  layout-compatible (inventory item OBS-no-core-layout-compatible). The
  run tests read through the other struct for each kind of match. The
  probe grid `layout-compatible.json` (in `tests/classes/members/grids/`)
  compares the conditions with the builtin query of Clang 22 and GCC.
  Clang 18 and EDG have no such query. The two queries agree with
  UNI-15 and UNI-16 on 23 of 24 rows. They agree that a signed and an
  unsigned member, members with different alignment, bit-fields of
  different widths, a bit-field and a plain member, an enumeration and
  its underlying type, and two arrays of different struct types do not
  match. The one other row is {question}`Q-0340`.
- UNI-15: the run test `common-initial-sequence-inherited.cpp` reads
  through the other struct on every compiler, and the grid row
  "members from a base" matches ({question}`Q-0345`).
- UNI-10: the sentence on volatile members follows {rule}`VOL-6` and the
  gap review of item O. A `const` member matches a plain one
  (`common-initial-sequence-cv.cpp`). The grid row "cv on one member"
  matches a `volatile` member with a plain one.

### Sources

UNI-15 and UNI-16 come from the grid `layout-compatible.json` and the
inventory items LC-alignment, LC-arrays, LC-bit-fields, LC-classes,
LC-cv, LC-enumerations, LC-inherited-members, LC-overlap-mark,
LC-prefix, and LC-unions. The UNI-10 wording comes from item
LC-union-read. The BF-9 notes come from items LAY-bit-field-alignment
and LAY-bit-field-zero-width and the tests.

Inputs: clang-sema-decl, clang-sema-declcxx, clang-sema-init,
clang-diag-sema, edg-class-decl.
