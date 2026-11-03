# Gap review: item O, class properties

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree. The
review compares the item O rules, the four new glossary entries, and
the seven questions with that draft only. Later defect resolutions are
mentioned only where the compilers may follow them.

## Result

All seven provisional answers agree with the reference or fit within
it. Q-0343 and Q-0041 both agree, against the compilers named in the
questions. Q-0342 concerns a layout choice that the reference leaves
open.

The rules mostly agree. The gaps are these:

- CLS-30 and CLS-31 tie the zero size of an empty base to the wrong
  class. The reference requires zero size for an empty base whose own
  class is standard-layout, whatever the derived class is. CLS-31 calls
  some of these cases implementation-defined.
- CLS-31 labels the whole layout beyond CLS-10 and CLS-30
  implementation-defined. The reference labels only a few layout facts
  that way and leaves the rest open with no label.
- CLS-32 counts every empty member with the no-unique-address attribute.
  The reference counts such a member only if it has zero size, and that
  size is the implementation's choice.
- The glossary entry for trivially copyable leaves out the two
  conditions that Q-0343 is about.
- LAM-20 leaves out the alignment of the closure type.
- UNI-10 does not mention volatile members, and UNI-15 rests on a
  reading about inherited members that the reference does not state.
- CPY-27 does not say that a member of array of class type counts.
- CPY-19 promises a byte copy into a second object, and for a class
  with a const member that copy writes a const object. The reference
  has the same tension and does not resolve it.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| CPY-P6 | agrees | The reference defines the property by the same three conditions. Access takes no part in eligibility. A reference member, or a const member of non-class type, deletes the implicit assignment, which then is not eligible, and the class can still be trivially copyable. The type table matches the reference. |
| CPY-19 | agrees, with one doubt | The reference needs at least one eligible copy or move function, each eligible one trivial, and a destructor that is trivial and not deleted. The second-object byte copy matches the reference, which also excludes subobjects that can overlap others. See "CPY-19 and const members" below. |
| CPY-27 | agrees, one missing detail | The reference makes a copy or move function trivial when it is not user-provided, the class has no virtual function and no virtual base, and the function selected for each direct base and each member of class type is trivial. The reference names members of array of class type as well. CPY-27 says only "a member of class type". Default member initializers do not affect copy triviality in the reference. A function defaulted after its first declaration is user-provided in the reference too. |
| CPY-28 | agrees | Scalars, trivially copyable classes, arrays of these, and their cv-qualified forms. |
| CLS-P4 | agrees | The principle matches the reference's intent. Its last paragraph inherits the classification problem of CLS-31. |
| CLS-19 | agrees | The seven conditions match the reference one for one. The reference counts bit-fields, named or not, when it asks that one class declare all the data members. |
| CLS-32 | agrees in part | See "CLS-32" below. |
| CLS-29 | agrees | Same pattern as CPY-28. |
| CLS-30 | agrees, with a narrower scope | The address facts and the pointer conversions to the first member and to each base match the reference. The zero size of an empty base is wider in the reference. See "Empty bases" below. |
| CLS-31 | wrong classification in part | See "Empty bases" and "Layout classification" below. |
| UNI-P6 | agrees | The reference makes a read through the shared start a read of a member that is not active, and constant evaluation rejects such a read. |
| UNI-3 | agrees | For a copy or move constructor, the reference deletes the union's function when the constructor chosen by overload resolution for a variant member is not trivial. For assignment the reference speaks of the matching assignment operator of the variant member, with less explicit wording about the choice. The default constructor and destructor cases match. |
| UNI-10 | agrees, one missing note | The reference allows the read for two struct members of a standard-layout union. The reference also notes that a volatile object read through a non-volatile glvalue is undefined. That can happen when the active struct has a volatile member and the other struct has a plain one, since cv-qualifiers do not stop the match. UNI-10 says nothing on this point. |
| UNI-15 | agrees, with one doubt | The four conditions match the reference. The reference does not say whether the members of a struct include those its base declares. UNI-15's last paragraph and the inherited test take a plausible reading that the reference does not state. |
| UNI-16 | agrees | Same type apart from cv-qualifiers, enumerations with one underlying type, structs whose common initial sequence covers every member and bit-field, and unions whose members pair up in any order. Arrays qualify only as the same type. |
| BF-9 | agrees | The reference makes bit-field allocation and alignment implementation-defined. A zero-width unnamed bit-field moves the next bit-field to an allocation unit boundary. The matrix choices are a valid instance. |
| JMP-P2 | agrees | Matches the reference's idea of a declaration whose default initialization does nothing. |
| JMP-6 | agrees | See "Q-0041" below. The reference also covers multidimensional arrays and cv-qualified forms. An empty class with a trivial default constructor can be declared const with no initializer, and a jump can skip it. JMP-6 says "class type", which covers that case if cv forms count as class types. |
| STG-2 | agrees | The reference requires contiguous bytes for objects of trivially copyable or standard-layout type. |
| STG-12 | agrees | The reference allows the round trip through plain char as well. The limit on plain char follows from the reference's rules for indeterminate values, which only unsigned char and the byte type may carry. The reference excludes subobjects that can overlap others, as STG-12 does through STG-25. |
| LAM-20 | agrees, one missing item | The reference lets an implementation change the closure's size and alignment, whether it is trivially copyable, and whether it is standard-layout. LAM-20 lists size but not alignment. The reference leaves the order of all members for copy captures unspecified, explicit ones included. LAM-20 names only implicit captures for the order of initialization. That bullet predates item O. |

## Glossary

| Entry | Verdict | Note |
|---|---|---|
| trivially copyable | missing conditions | The entry says the eligible copy and move functions and the destructor are trivial. Read alone, a class with no eligible copy or move function qualifies, and so does a class whose deleted destructor would otherwise be trivial. CPY-19 and Q-0343 exclude both. |
| standard-layout | agrees | A loose summary of CLS-19 and CLS-29. |
| layout-compatible | agrees | The union case is folded into "members match in full", which is loose but not wrong. |
| common initial sequence | agrees, loosely | "Match in type" is narrower than the reference, which asks for layout-compatible types. A const int and an int match, and so do two enumerations with one underlying type. |

## Questions

| Question | Provisional answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0041 | A jump can skip a class with a trivial default constructor, whatever its destructor (option 2). | agrees | option 2 | See below. |
| Q-0338 | The addresses compare equal in constant evaluation (option 1). | agrees | option 1 | The reference makes two pointers that hold one address compare equal. The only unspecified case is a pointer one past the end of one object against the address of another complete object, which is not this case. A comparison with an unspecified result is not a constant expression, and this one is specified. EDG's false result conflicts with the reference. |
| Q-0339 | The selected function decides (option 1). | agrees | option 1 | The reference tests the function chosen to copy each base and each member, not every copy function of the part's class. The union rule for copy and move constructors also names the function chosen by overload resolution. So the copy of the first holder is trivial, and the copy of the pair that selects the template is not, as Clang finds. |
| Q-0340 | Layout-compatible when both members carry the attribute (option 1). | agrees | option 1 | The reference asks that both members carry the attribute or neither does. It does not end the sequence at a marked member. Both structs are standard-layout, and the member types and alignments match, so the structs are layout-compatible. Clang 22's answer conflicts with the reference. |
| Q-0341 | Not standard-layout in all four cases (option 1). | agrees | option 1, with a condition on the fourth case | See "CLS-32" below. |
| Q-0342 | The base keeps its padding (option 1). | silent | either | The reference does not fix when a derived class reuses the end padding of a base. Both answers are allowed. Recording the newest matrix choice is Tidal's own decision. |
| Q-0343 | Neither class is trivially copyable (option 1). | agrees | option 1 | See below. |

### Q-0343 in more detail

- The reference states both limits outright. A trivially copyable class
  needs at least one eligible copy or move function, and its destructor
  must be trivial and also not deleted.
- In the reference, a deleted function is never eligible. A class that
  deletes all four copy and move functions has none, so it fails the
  first limit. A class whose destructor is deleted fails the third
  limit, even when that destructor would otherwise count as trivial.
- Both limits entered the definition through core defect work in the
  C++20 cycle, when eligible special members were introduced. The
  builtin queries of the three compiler families appear to keep the
  older definition, which looked only at the triviality of the
  functions. I do not know the vendors' reasons. I know of no later
  resolution that withdraws either limit. Committee discussion of what
  the trivial properties should mean has continued, but nothing I know
  of changes these two limits for C++23.
- CPY-19 and CPY-P6 match the reference. The compilers' queries
  conflict with it. The question is right that no core-language check
  shows the difference, because the union rules look at the functions
  and not at the class property.

### Q-0041 in more detail

- In the reference, a jump may not enter the scope of an automatic
  variable unless every such variable gets vacuous initialization. A
  variable gets vacuous initialization when it is default-initialized
  and, for a class type or an array of one, the class has a trivial
  default constructor. The destructor plays no part.
- The older wording, up to C++17, asked for a trivial default
  constructor and a trivial destructor. A core defect resolution in the
  C++20 cycle, about the lifetime of objects with trivial destructors,
  removed the destructor condition, and the committee treated the
  change as a fix to earlier standards.
- GCC follows the newer rule. Clang and EDG still apply the older
  rule. As far as I know, Clang lists that resolution as not done. No
  later resolution that I know of brings the destructor condition back.
- JMP-6 and the provisional answer match the reference. The end of the
  block destroys the skipped object as usual, which the reference
  shows in its rule for transfers of control, so JMP-P2 and JMP-16 fit
  too.

### CLS-32

- The reference builds the set of types to avoid as bases in this way.
  It takes the first data member and every data member of zero size.
  For each, it adds the member's type and, recursively, the set for that
  type. For an array it adds the element type and its set. For a union
  it adds every member type and each of their sets. A class with no
  data members adds nothing, and neither does a scalar type.
- The first three rows of Q-0341 follow from the first member, its
  array element, and its union members. The reference makes all three
  classes not standard-layout, as Clang finds. The rule that builds
  this set came from a core defect resolution before C++20, so GCC and
  EDG lag behind it there.
- The fourth row depends on the size of the marked member. A data
  member with the no-unique-address attribute and an empty class type
  may have zero size. The reference leaves that choice to the
  implementation, because the member is not a base. If the member has
  zero size, the class is not standard-layout. If it does not, the
  member is not first, so the class is standard-layout. CLS-31 records
  that every compiler in the matrix gives such a member zero size, so
  option 1 holds for the matrix. CLS-32 states the condition for every
  implementation, and the reference states it only for a member of zero
  size.
- The reference takes the set of a zero-size member recursively, as it
  does for the first member. CLS-32 adds the marked member's type but
  not the objects at its start. An empty class rarely has such
  objects, so this gap is small.
- The bit-field exclusion in CLS-32 has no counterpart in the
  reference. It does no harm, because a bit-field has integral or
  enumeration type and cannot be a base.
- The reference counts an anonymous union in first position as the
  first member. CLS-32 covers that case if Tidal treats an anonymous
  union as a member of union type. The rule does not say so.

### Empty bases

- The reference gives an object zero size when it is a base subobject
  whose class is standard-layout and has no data members, and when no
  other condition forces a nonzero size (virtual functions, virtual
  bases, or parts of nonzero size). The derived class does not have to
  be standard-layout.
- CLS-30 gives zero size to an empty base only inside a standard-layout
  class. The reference requires more. CLS-31's second row covers an
  empty base whose type is the type of the first member. That base's
  class is empty and standard-layout, so the reference requires zero
  size there, while CLS-31 calls the case implementation-defined.
  The same holds for an empty standard-layout base of a class with a
  virtual function.
- The reference gives a zero-size subobject the address of some byte
  of its complete object, with no rule for which byte. So the offset of
  such a base is unspecified in the reference, while its size is
  required.
- CLS-31's first row, an empty base whose class is not standard-layout,
  matches the reference. The reference makes zero size for such a base
  implementation-defined.

### Layout classification

- The reference labels these layout facts implementation-defined: which
  empty objects outside the required case have zero size, the
  allocation and alignment of bit-fields, and alignment requirements.
- The reference gives no label to the place of the virtual-call pointer,
  to the padding between members, or to the reuse of a base's end
  padding. It leaves them open, which is closer to unspecified than to
  implementation-defined.
- CLS-31 calls all of these implementation-defined. The tests use the
  matrix choices, so the classification changes no test. It does
  promise documentation that the reference does not require.
- The no-unique-address rows of CLS-31 match the reference. A marked
  member of empty class type may take no bytes at the implementation's
  choice, and two objects of one type always need distinct addresses.

### CPY-19 and const members

- CPY-P6 and CPY-19 say a class with a const member is trivially
  copyable, and the reference agrees.
- The reference's second-object byte copy then writes the bytes of a
  const member of the target. The reference also makes any change to a
  const object during its lifetime undefined. The reference does not
  say which rule wins.
- The tests copy into a buffer and back, or into a fresh object. A copy
  over a live object with a const member is the doubtful case. CPY-19
  could keep the two rules apart. A question on this point would record
  the doubt.
