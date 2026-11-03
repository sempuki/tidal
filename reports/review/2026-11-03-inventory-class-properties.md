<!--
Behaviour inventory, 2026-11-03: class properties (phase 6 item O,
roadmap-3 designer item 8).
Written by: a source reader under AGENTS.md that read the C++ working
draft sources and Tidal's own pages (classes/copy-move.md,
classes/members.md, classes/constructors.md, classes/unions.md,
declarations/alignment.md, declarations/attributes.md,
statements/jumps.md, foundations/storage.md, foundations/types.md, the
glossary). It did not write or change any file in this repository.
Checked before use: the source reader ran tools/similarity.py with
--spec on this summary outside the repository, reworded each flagged
passage, and reran until the tool exited 0. No code, no quotes, no
section names. Each item is a behaviour to probe, not a rule. Groups,
labels and order are the source reader's own.
-->

# Behaviour inventory for phase 6, item O

Role: source reader. Each item below describes language behaviour in
the source reader's own words. Groups and labels are the source
reader's own. Items are sorted alphabetically by label within each
group. In this inventory, "copy and move functions" means the copy
constructor, the move constructor, the copy assignment operator and
the move assignment operator. "Special members" adds the default
constructor and the destructor. "Empty-type member marked for overlap"
means a non-static data member that carries the attribute that lets
it share an address.

## Categories (probe axes)

1. Access. All non-static data members in one access, members split
   over two or three accesses, private or protected bases, and
   inaccessible special members.
2. Base shape. No base, one base, a chain of single bases, two
   different bases, the same empty base reached twice by two paths,
   a virtual base, an empty base whose type equals the type of the
   first member (or of something at offset zero inside it).
3. Where data members are declared. All in the most derived class,
   all in one base, split between a base and the derived class, an
   unnamed bit-field added in a derived class whose base has members.
4. Kind of each special member. Implicit, defaulted on its first
   declaration, defaulted after its first declaration, user-provided,
   deleted (explicitly or because a part cannot do it), constrained
   with a satisfied or unsatisfied constraint, two overloads of one
   kind where one is more constrained, two copy constructors that take
   different reference kinds, a constructor template that wins over
   the copy constructor for some source.
5. Member types. Scalars (including enumerations, pointers, pointers
   to members, the null pointer type), volatile scalars, const
   members, reference members (lvalue and rvalue), arrays, members of
   class type with each property on or off, bit-fields (named,
   unnamed, zero width, wider than the type), anonymous unions,
   empty-type members marked for overlap.
6. Virtual parts. None, a virtual function, a virtual destructor, a
   virtual base.
7. Default member initializers. None, on one member, on a variant
   member.
8. Class key. A class or struct versus a union, and a closure type of
   a lambda with or without captures.
9. Type form. A class, an array of it (known and unknown bound), a
   cv-qualified form, a scalar, a reference, void, a function type.
10. Observation channel. A requires expression or other check of
    validity, a union wrapper whose implicit special member is deleted
    or not, a jump past a declaration, constant evaluation (assignment
    to an inactive union member, pointer comparisons), the size and
    alignment operators, address comparison at run time, address
    identity of a by-value parameter, an argument passed through an
    ellipsis, a library trait (not usable in tests under decision
    0051), and a compiler builtin query (Evidence only).
11. Target and compiler family. The matrix targets, and the GCC,
    Clang and EDG families with their ABI choices.

## Trivially copyable classes

- TCC-all-deleted: a class whose four copy and move functions are all
  deleted is not trivially copyable, even if its destructor is
  trivial.
- TCC-at-least-one: a class with one eligible copy or move function
  (for example only a trivial move constructor, with the others
  deleted) is trivially copyable when that function is trivial and the
  destructor is trivial and not deleted.
- TCC-const-member: a class with a const scalar member has a deleted
  copy assignment, but its copy constructor stays trivial, so the class
  is still trivially copyable.
- TCC-constrained-overloads: in a class template with two copy
  constructors of one kind, a constrained defaulted one and an
  unconstrained user-provided one, only the more constrained one is
  eligible when its constraint holds. The class is then trivially
  copyable. When the constraint fails, the user-provided one is the
  eligible one and the class is not trivially copyable. Probe each
  compiler family.
- TCC-defaulted-later: a copy or move function defaulted after its
  first declaration counts as user-provided and is not trivial, so the
  class is not trivially copyable. Tidal's CPY-19 lists "implicit or
  defaulted" without this split; CPY-27 has the split.
- TCC-deleted-destructor: a deleted destructor makes the class not
  trivially copyable, even with trivial copy and move functions.
- TCC-every-eligible: each eligible copy or move function must be
  trivial. A class with a trivial defaulted copy constructor taking a
  const reference and a second, user-provided copy constructor taking
  a non-const reference is not trivially copyable, because both are
  eligible.
- TCC-inaccessible-ok: access plays no part. A trivial copy
  constructor that is private still counts as trivial and eligible.
- TCC-member-selected: triviality of a class's own copy constructor
  depends on the function that overload resolution picks to copy each
  base and member. A member whose class is not trivially copyable (two
  copy constructors as in TCC-every-eligible) can still be copied by a
  trivial one, and the enclosing class can then be trivially copyable.
  Tidal's CPY-19 requires each member to be trivially copyable, which
  differs here.
- TCC-member-template-wins: a constructor template in a member's class
  can be a better match than its copy constructor for some source
  kinds. Probe whether the enclosing class's copy constructor then
  counts as non-trivial and whether the enclosing class stays
  trivially copyable.
- TCC-reference-member: a class with a reference member has a deleted
  copy assignment but a trivial copy constructor, so it is trivially
  copyable.
- TCC-union: a union whose members are all trivially copyable is
  trivially copyable. A union with a member that has a non-trivial
  copy function has that function deleted unless the program provides
  one, and a provided one is non-trivial.
- TCC-virtual-destructor: a virtual destructor is never trivial, so a
  class with one is not trivially copyable.
- TCC-virtual-parts: a virtual function or a virtual base makes every
  copy and move function non-trivial, so the class is never trivially
  copyable.
- TCC-volatile-member: a volatile scalar member does not make the copy
  constructor non-trivial, so the class stays trivially copyable.
  Older compilers disagreed. Probe each family.

## Trivially copyable and other type categories

- TYPE-array: an array of a trivially copyable type is trivially
  copyable, and the same holds for the standard-layout and trivial
  categories. Probe arrays of unknown bound, which are incomplete.
- TYPE-closure: whether a closure type is trivially copyable, and
  whether it is standard-layout, is left to the implementation, along
  with its size and alignment. Probe lambdas with no capture, with
  copy captures and with reference captures.
- TYPE-cv: a cv-qualified form of a trivially copyable type is still
  trivially copyable, including const and volatile class types and
  volatile scalars. The same holds for standard-layout.
- TYPE-non-object: references, void and function types belong to none
  of these categories.
- TYPE-scalar: every scalar type (arithmetic, enumeration, pointer,
  pointer to member, the null pointer type) is trivially copyable,
  standard-layout, trivial, and implicit-lifetime.
- TYPE-trivial-class: a trivial class is trivially copyable and also
  has at least one eligible default constructor, and every eligible
  default constructor is trivial. A default member initializer or a
  user-provided default constructor leaves the class trivially
  copyable but not trivial.
- TYPE-trivial-vs-standard-layout: the two properties are
  independent. Members in two accesses can give a trivial class that
  is not standard-layout. A user-provided destructor can give a
  standard-layout class that is not trivially copyable.

## Trivial and eligible special members

- TRIV-copy-conditions: a copy or move function is trivial when it is
  not user-provided, the class has no virtual function and no virtual
  base, and the function chosen for each direct base and each member
  of class type (or array of class type) is trivial. Default member
  initializers play no part (Tidal's CPY-27 agrees).
- TRIV-default-conditions: a default constructor is trivial when it is
  not user-provided, the class has no virtual parts, no default member
  initializer appears on any member, and the default constructors of
  the direct bases and of the class-type members (or arrays of them)
  are all trivial (Tidal's CTOR-23).
- TRIV-destructor-conditions: a destructor is trivial when it is not
  user-provided, is not virtual, and each direct base and each class
  member (or array of one) has a trivial destructor (Tidal's DTOR-9).
  Virtual parts other than a virtual destructor do not by themselves
  make the destructor non-trivial. Probe a class with a virtual
  function and a defaulted destructor.
- TRIV-eligible-constraints: a special member with an unsatisfied
  constraint is not eligible. Of two special members of one kind with
  satisfied constraints, the one that the other is more constrained
  than is not eligible. "Same kind" for copy and move functions means
  the same first parameter type, and for assignment also the same
  qualifiers on the object parameter.
- TRIV-eligible-deleted: a deleted special member is never eligible,
  so it neither helps nor blocks trivially copyable.
- TRIV-union-delete: a union's implicit default constructor, copy and
  move functions and destructor are deleted when a member's matching
  function is non-trivial, unless a default member initializer on a
  variant member saves the default constructor. The same holds for an
  anonymous union inside a class. This is the main core-language
  window on triviality (see OBS group).

## Standard-layout classes

- SL-access: all non-static data members of the class have one
  access. Static members and member functions do not count.
- SL-base-standard-layout: every base class is standard-layout.
- SL-bit-field-counts: unnamed bit-fields count as members for the
  single-declaring-class condition. An unnamed bit-field added in a
  derived class whose base already has a member breaks
  standard-layout.
- SL-first-member-type: no base class may have the type of the first
  non-static data member, nor the type of anything that sits at offset
  zero inside that member (its own first member, recursively, the
  element type of an array, and every member type of a union or
  anonymous union in that place). Zero-size members (empty-type
  members marked for overlap) also join this set wherever they are.
  Probe each route.
- SL-member-standard-layout: every non-static data member has a
  standard-layout type (arrays of such count). A reference member of
  any kind breaks standard-layout.
- SL-one-declaring-class: across the class and all its bases, the
  non-static data members and bit-fields are all first declared in one
  class. Empty bases are fine at any depth. Members split between a
  base and the derived class break standard-layout.
- SL-one-subobject-per-type: no base type appears twice as a base
  subobject, even when both copies are empty and reached by different
  paths.
- SL-struct-vs-union: a standard-layout class declared with the union
  key is a standard-layout union; with the class or struct key it is a
  standard-layout struct. Some guarantees (common initial sequence,
  layout-compatible classes) talk about structs only.
- SL-virtual: a virtual function or a virtual base breaks
  standard-layout.

## What standard-layout gives a program

- SLG-base-address: the address of a standard-layout object equals the
  address of each of its base subobjects.
- SLG-contiguous: an object of a standard-layout type (or a trivially
  copyable type) occupies one unbroken run of bytes (Tidal's STG-2).
- SLG-empty-base-zero: a base subobject whose type is an empty
  standard-layout class has zero size, except where two subobjects of
  one type would then share an address (see LAY-empty-base-same-type).
- SLG-first-member-address: when the class has at least one
  non-static data member and the first one is not a bit-field, the
  object's address equals the first member's address. A bit-field
  first member has no address, so this guarantee does not reach it.
- SLG-no-leading-padding: padding can appear between members and at
  the end, but never before the first member.
- SLG-offset-query: the library's member offset query is supported for
  standard-layout classes. Use with any other class is
  conditionally-supported. Library only, and tests use no header.
- SLG-pointer-interconvertible: a standard-layout object is
  pointer-interconvertible with its first non-static data member and
  with each base subobject, so a reinterpret cast between pointers to
  them gives a usable pointer (Tidal's STG-16 and CLS-19).
- SLG-pointer-same-representation: pointers to layout-compatible types
  have the same value representation and the same alignment.

## Common initial sequence and layout-compatible types

- LC-alignment: corresponding members must have the same alignment
  requirement. An alignment specifier on one member and not the other
  ends the common initial sequence there.
- LC-arrays: arrays are layout-compatible only by being the same type.
  Probe whether two arrays of layout-compatible structs count, and
  whether compilers agree.
- LC-bit-fields: corresponding entries must both be bit-fields of the
  same width, or both be ordinary members. An unnamed zero-width
  bit-field in one struct ends the sequence there.
- LC-classes: two standard-layout structs are layout-compatible when
  the common initial sequence covers every member and bit-field of
  both.
- LC-cv: cv-qualifiers do not matter. A const or volatile member
  matches an unqualified member of the same type.
- LC-enumerations: two enumerations are layout-compatible when their
  underlying types are the same. An enumeration and its underlying
  integer type are not layout-compatible. Probe enumerations with the
  same underlying type but different enumerators, and scoped versus
  unscoped.
- LC-inherited-members: a standard-layout struct can take all its
  members from a base. Probe whether such a struct and a struct that
  declares the same members directly have a full common initial
  sequence.
- LC-overlap-mark: corresponding members must agree on the overlap
  attribute (both marked or neither).
- LC-prefix: for a pair of standard-layout structs, the common initial
  sequence is the longest leading run of members, in declaration order,
  whose corresponding types are layout-compatible and that meet the
  other conditions in this group. A signed and an unsigned integer of
  one size do not match.
- LC-union-read: in a standard-layout union whose active member is a
  standard-layout struct, a program can read a member of another
  struct member, if that member lies in the common initial sequence of
  the two structs. The read acts as a read of the matching member of
  the active struct (Tidal's UNI-10).
- LC-union-read-constant: constant evaluation rejects the read of
  LC-union-read (Tidal's UNI-12). Probe whether each compiler agrees.
- LC-union-read-volatile: when the active struct's member is volatile
  and the other struct's matching member is not, the read goes through
  a non-volatile glvalue to a volatile object, which is undefined.
- LC-unions: two standard-layout unions are layout-compatible when they
  have the same number of members and their member types can be paired
  up, in any order, as layout-compatible.
- LC-write-not-covered: the common initial sequence allows reads only.
  A write through the inactive struct is a normal write to an inactive
  member, which switches the active member.

## Implicit-lifetime types and aggregates

- AGG-conditions: an aggregate class has no user-declared or inherited
  constructors, no private or protected direct non-static data
  members, no private or protected direct bases, and no virtual
  functions or virtual bases. Probe how this differs from trivially
  copyable and from standard-layout.
- AGG-not-closure: a closure type is never an aggregate.
- AGG-vs-standard-layout: an aggregate can have public bases with
  members in more than one class, so it need not be standard-layout.
- AGG-vs-trivial: an aggregate can have members with non-trivial
  special members, so it need not be trivially copyable. A class with
  private members can be trivial but is not an aggregate.
- IL-aggregate-branch: every aggregate whose destructor is not
  user-provided is implicit-lifetime, even when its members have
  non-trivial constructors or destructors (Tidal's storage.md list).
- IL-arrays-all: every array type is implicit-lifetime, whatever its
  element type.
- IL-copy-constructor-counts: the trivial eligible constructor of the
  second branch can be a copy or move constructor. A class with a
  user-provided default constructor but a trivial copy constructor and
  a trivial destructor is implicit-lifetime.
- IL-deleted-destructor: a deleted destructor keeps a class out of the
  second branch. Probe whether an aggregate with a deleted destructor
  counts through the first branch (the destructor is deleted, not
  user-provided).
- IL-trivial-branch: a class is implicit-lifetime when its destructor
  is trivial and not deleted and one or more of its eligible
  constructors is trivial.
- IL-user-provided-destructor: an aggregate with a user-provided
  destructor is not implicit-lifetime through the first branch, and a
  user-provided destructor is never trivial, so the class is not
  implicit-lifetime at all.

## Observation channels without the library

- OBS-aggregate-designated: designated initializers work only for
  aggregates. A requires expression that tests brace initialization
  with designators observes aggregate status for classes that have at
  least one named member.
- OBS-call-temporary: when each eligible copy or move constructor of a
  class is trivial (and there is one or more), and its destructor is
  trivial or deleted, the implementation may pass and return its
  objects through a temporary (in registers). The parameter's address
  can then differ from the address that a non-copy constructor saw for
  the argument. This channel is per target and per ABI, so it shows
  how one compiler classifies a class, not a portable answer. Note
  that this condition looks only at constructors and the destructor,
  not at assignment, and so differs from trivially copyable.
- OBS-constant-relational: a relational comparison between pointers
  whose order the language leaves open (for example a base subobject
  and a member, or two members of a union) is not a constant
  expression. Constant evaluation therefore shows which pairs have a
  fixed order. Pairs of members in one class, any access, have a fixed
  order unless one has zero size.
- OBS-ellipsis: passing a class with an eligible non-trivial copy or
  move constructor, or a non-trivial destructor, through an ellipsis is
  conditionally-supported. Compilers differ (some reject, some warn,
  some accept). Probe as a diagnostic channel only.
- OBS-jump-vacuous: a jump past an automatic variable's declaration is
  allowed only for vacuous initialization (default initialization of a
  scalar or of a class with a trivial default constructor). Tidal's
  JMP-6 also requires a trivial destructor. Probe whether each
  compiler rejects a jump past a class with a trivial default
  constructor and a non-trivial destructor.
- OBS-literal: whether a class is a literal type is observable through
  a constexpr variable or constant evaluation. Literal type does not
  depend on trivially copyable or standard-layout.
- OBS-no-core-implicit-lifetime: no core-language channel shows whether
  a type is implicit-lifetime. Implicit creation does not run in
  constant evaluation, and a wrong guess gives undefined behaviour that
  sanitizers do not report. Only the library trait answers.
- OBS-no-core-layout-compatible: layout-compatible, common initial
  sequence and pointer-interconvertible have no core channel. The union
  read is rejected in constant evaluation, and a reinterpret cast is
  not allowed there. Only library traits answer.
- OBS-no-core-standard-layout: no core channel gives a yes or no for
  standard-layout. Each guarantee (address of first member, zero-size
  empty base, no leading padding) is something non-standard-layout
  classes may also show, so a run-time probe can confirm a guarantee
  but cannot prove a class is not standard-layout. Only the library
  trait (or a builtin query used as Evidence) answers.
- OBS-no-core-trivial-whole: no single core check answers trivially
  copyable or trivial for a class. A combination of OBS-union-wrapper
  checks matches the definitions except where the class has several
  eligible functions of one kind (TCC-every-eligible) or where
  eligibility differs from what overload resolution picks.
- OBS-size-alignment: the size and alignment operators are core. They
  show zero-size empty bases, tail padding reuse, the effect of the
  overlap attribute, and bit-field packing, per target.
- OBS-union-activation-constant: in constant evaluation, assignment to
  an inactive union member starts its lifetime only when its default
  constructor is trivial (Tidal's UNI-13), so constant evaluation
  observes a trivial default constructor.
- OBS-union-wrapper: wrapping the type as a union member, with no
  default member initializer, makes each implicit special member of
  the union deleted when the matching one of the type is non-trivial.
  A requires expression can then test default construction, copy and
  move construction, copy and move assignment, and an explicit
  destructor call on the union. A deleted function in the type also
  deletes the union's one, so a second check on the type itself is
  needed to tell deleted from non-trivial. The copy and move checks
  see only the function chosen for one source kind, not every
  eligible one.

## Class layout that varies by target

- LAY-access-order: members in different accesses still get
  increasing addresses in declaration order (Tidal's CLS-10). Earlier
  language versions left this order open. Probe that no compiler
  reorders.
- LAY-alignment-class: a class's alignment is the strictest of its
  members, its bases, and its own alignment specifier, and its size is
  a multiple of that alignment (Tidal's ALN-1). Probe a class with a
  virtual pointer, and a class with a virtual base.
- LAY-alignment-complete-vs-sub: a class with a virtual base can need a
  different alignment as a base subobject than as a complete object.
  The alignment operator reports the complete-object value.
- LAY-alignment-extended: whether alignments beyond the largest
  fundamental alignment are supported, and where, is
  implementation-defined. An unsupported request makes the program
  ill-formed. Valid alignments are powers of two, and the narrow
  character types have the weakest one.
- LAY-bit-field-alignment: the alignment of bit-fields and their effect
  on the class's alignment are implementation-defined. Probe whether
  the declared type of an unnamed bit-field changes the class
  alignment.
- LAY-bit-field-allocation: how bit-fields are placed in allocation
  units is implementation-defined, including whether one can straddle
  two units and whether bits fill from the low or high end. Probe the
  bit order on each target through a byte view, and the size of
  classes with mixed declared types.
- LAY-bit-field-wide: a width larger than the width of the declared
  type is allowed; the extra bits are padding. Probe the size effect.
- LAY-bit-field-zero-width: an unnamed zero-width bit-field moves the
  next bit-field to the start of a new allocation unit. Probe its
  effect on a following ordinary member and on size.
- LAY-empty-base-non-standard-layout: whether an empty base whose type
  is not standard-layout has zero size is implementation-defined.
  Probe an empty class with members of zero size only, and an empty
  class that is not standard-layout because of a repeated empty base.
- LAY-empty-base-same-type: an empty base whose type equals the type of
  the first member, or another base, cannot share that subobject's
  address, so the class grows. Probe size and address equality.
- LAY-empty-class-size: the size of an empty class is at least one and
  otherwise implementation-defined (Tidal's CLS-11).
- LAY-first-member-non-standard-layout: in a class that is not
  standard-layout, the first member's address can differ from the
  object's address (for example after a virtual pointer). Probe each
  family.
- LAY-overlap-attribute-bit-field: the overlap attribute on a bit-field
  is ill-formed. Tidal's Q-0021 records that compilers differ.
- LAY-overlap-attribute-empty: an empty-type member marked for overlap
  can share an address with another member or base of a different
  type, and takes no bytes. Two marked members of one empty type keep
  distinct addresses. Probe size and addresses.
- LAY-overlap-attribute-tail: a marked member of non-empty type lets
  later members use its tail padding. Probe size on each family.
- LAY-overlap-byte-copy: a potentially overlapping subobject is outside
  the byte-copy guarantee even when its type is trivially copyable,
  because a copy of its full size can overwrite a neighbour in its tail
  padding (Tidal's STG-12).
- LAY-padding: padding between members and at the end is the
  implementation's choice within alignment needs. Probe sizes for
  members ordered from small to large and large to small.
- LAY-pointer-order: a later member compares greater than an earlier
  one in the same object unless one has zero size or the class is a
  union. No order is fixed between a base subobject and a member, or
  between two bases.
- LAY-tail-padding-base: whether a derived class places members in a
  base's tail padding is an ABI choice. The GCC and Clang ABI does so
  only for bases that are not plain old data in its own sense. Probe
  size with a base that has a user-provided constructor, a private
  member, or neither.
- LAY-virtual-placement: where the virtual pointer and virtual base
  subobjects sit is left to the implementation. A virtual base can sit
  at a different offset for each most derived type.

## Gaps relative to Tidal

- GAP-CLS-19-address: CLS-19 says the address equals the first data
  member's address without the bit-field exception and without the
  case of a class with no data members (SLG-first-member-address).
- GAP-CLS-19-list: CLS-19 lacks SL-base-standard-layout (it has the
  member form only), SL-bit-field-counts, SL-first-member-type,
  SL-member-standard-layout for references, SL-one-declaring-class
  and SL-one-subobject-per-type.
- GAP-CPY-19-list: CPY-19 counts copy and move functions that are not
  deleted, where the property counts eligible ones, which differ when
  constraints apply. CPY-19 also lacks TCC-constrained-overloads, TCC-deleted-destructor,
  TCC-every-eligible, TCC-defaulted-later and TCC-member-selected, and
  uses "implicit or defaulted" where the property needs "trivial".
- GAP-glossary: the glossary has no entries for trivially copyable,
  standard-layout, trivial class, layout-compatible, or common initial
  sequence. "trivial" is defined for special members only.
- GAP-JMP-6: JMP-6 requires a trivial destructor as well as a trivial
  default constructor. See OBS-jump-vacuous.
- GAP-types-categories: foundations/types.md defines no type-level
  categories (TYPE group). STG-2 and STG-12 apply "trivially copyable"
  and "standard-layout" to types, including scalars and arrays, which
  only CPY-19 and CLS-19 define, and only for classes.
- GAP-UNI-10: UNI-10 defines the common initial sequence as members of
  "the same types". It lacks LC-alignment, LC-bit-fields, LC-cv,
  LC-enumerations, LC-overlap-mark and nested layout-compatible
  structs, and has no term for layout-compatible.
