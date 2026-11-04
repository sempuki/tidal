# Gap review: the object model

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree. The
review compares the live rules and principles of three pages with that
draft: objects, storage, and lifetime (OBJ), raw storage and object
representation (STG), and language support (LSUP). It also reads the
eighteen provisional questions whose Affects line names one of these
rules and that have no gap-review reading. Q-0307 already has one and
is left out. Later defect resolutions are mentioned only where they
matter.

## Result

The storage page mostly agrees with the reference, and so does most of
the objects page. The language-support page describes a practice that
the reference does not cover at all (see "Names that the program
declares in the standard namespace" below), so its rules are Tidal
choices with no counterpart.

The gaps are these:

- OBJ-1 ends the lifetime of a class object when its destructor
  completes. The reference ends it when the destructor call begins.
  OBJ-1 starts the lifetime of a scalar with its storage, and the
  reference waits for the scalar's initialization to finish. OBJ-1 ties
  the start of a class object's lifetime to a constructor, and
  aggregate initialization has none.
- OBJ-4 requires namespace-scope static objects to be built before the
  main function runs and in definition order. The reference makes
  early construction implementation-defined (as SINIT-5 says) and
  leaves template instantiations unordered.
- OBJ-11 calls the start of a thread-local object unspecified. The
  reference calls it implementation-defined, as SINIT-6 does.
- OBJ-6 does not put virtual bases first.
- OBJ-7 gives an empty class size 1. The reference asks only for a
  size above zero.
- STG-9 accepts any member of one common enclosing object. The
  reference also asks that the enclosing object itself could be
  replaced, which a base subobject and a const complete object cannot.
- STG-12 forbids a byte copy through plain char when a byte has no
  value. The reference names plain char arrays for byte copies and
  makes the guarantee hold even when the object has no valid value.
- LSUP-10 and LSUP-11 leave out the reference's conditions for
  constant evaluation: the object or storage that the construct-at
  function targets, and a literal element type for the allocator.
- LSUP-12 asks for equality with zero. The reference tests a defaulted
  three-way comparison with inequality against zero.

Of the eighteen questions, four agree, one agrees in part (Q-0350),
three contradict the provisional answer (Q-0299, Q-0334, and Q-0336),
and ten are unclear or silent.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| OBJ-P1 | agrees | The table of events matches. The lifetime-end point of a class object inherits the OBJ-1 gap. |
| OBJ-P2 | agrees in part | The static row shares the OBJ-4 gaps. Static objects also end when the program calls the exit function, not only after main returns. |
| OBJ-P3 | agrees | A reference is not an object in the reference either. |
| OBJ-P4 | agrees | Constant evaluation rejects these cases because each is undefined. |
| OBJ-1 | missing conditions | See "Lifetime bounds" below. |
| OBJ-2 | agrees | Access through a glvalue outside the lifetime is undefined, and so is the production of an indeterminate value other than through unsigned char or the byte type. A defaulted copy reads each scalar member, and an unnamed bit-field is not a member, so it is not copied. |
| OBJ-3 | agrees | Reverse order on every exit from the block. |
| OBJ-4 | wrong classification, missing conditions | See "Static and thread objects" below. |
| OBJ-5 | agrees | |
| OBJ-6 | missing condition | The reference builds virtual bases first, in a depth-first, left-to-right walk of the base graph, then the direct bases in base-list order, then the members. The derived classes page covers virtual bases, but OBJ-6 states the order with no exception. |
| OBJ-7 | agrees, one extra claim, one exception | The address rules match. The reference requires only a nonzero size for a complete object of an empty class, so size 1 is the implementation's choice (every matrix compiler picks 1). String literal objects may overlap in the reference, an exception that the primary-expressions page states and OBJ-7 does not mention. |
| OBJ-8 | agrees | REF-4 carries the detail. The reference has four exceptions to extension: a reference parameter in a call, a reference member of an aggregate built from a parenthesized list, a returned reference, and a reference in the initializer of a new expression. |
| OBJ-9 | agrees, missing cases | The reference also delays the end of a temporary in a default argument of a constructor that initializes or copies array elements, and, from C++23, extends temporaries in the range expression of a range-based for loop. OBJ-9 says "no longer" without these cases. |
| OBJ-10 | agrees | |
| OBJ-11 | wrong classification | The reference makes the choice implementation-defined. SINIT-6 already says so, and the two rules disagree. |
| STG-P1 to STG-P8 | agree | Each principle follows its rules. STG-P8's row for an invalid pointer value includes an integer conversion, which shares the STG-15 doubt. |
| STG-1 | agrees | The reference makes a type whose object representation needs more bytes than the size type can count ill-formed. |
| STG-2 | agrees | |
| STG-3 | agrees | Bit-fields are outside the address rule in the reference too. |
| STG-4 | agrees | Only unsigned char and byte-type arrays provide storage, and the nesting relation matches. The reference states hosting for a complete object created in the array. |
| STG-5 | agrees | The reference forbids any array overhead for the library's non-allocating forms. |
| STG-6 | agrees | Reuse by an object that is not nested ends the old object, with no destructor call. |
| STG-7 | agrees | The four kinds of type, including the aggregate with a destructor that is not user-provided, match. |
| STG-8 | agrees | The reference also uses a later use to pick the pointer that an allocation function returns, and the choice is unspecified when several work. |
| STG-9 | missing condition | See "Transparent replacement" below. |
| STG-10 | agrees | |
| STG-11 | agrees, one doubt | The definitions match. The last sentence is Tidal's answer to Q-0328, which the reference does not state. |
| STG-12 | extra condition | See "Byte copies" below. |
| STG-13 | agrees with the intent | See "Byte copies" below. |
| STG-14 | agrees | The reference asks for a type similar to one of three kinds, which covers cv-qualifiers at every level. Similarity also covers an array of unknown bound against an array of known bound. |
| STG-15 | agrees, one doubt | The four kinds match. The reference makes the result of converting an integer to a pointer implementation-defined, apart from the round trip. The reference never says that such a pointer is an invalid pointer value when no object lies there. |
| STG-16 | agrees | Same four cases, and an array and its first element are not interconvertible. |
| STG-17 | agrees | The reference starts a lifetime only in storage of the right size and alignment and gives no meaning to a construction elsewhere. The behavior is undefined by omission. |
| STG-18 | agrees | |
| STG-19 | agrees | |
| STG-20 | agrees | Automatic, static, and thread storage, during or after the lifetime. |
| STG-21 | agrees | Exit by an exception counts in the reference too. |
| STG-22 | agrees | The reference keeps separate lists for a pointer and for a glvalue. The pointer list has the cast and the delete. The glvalue list has the bind to a virtual base and the type query. STG-22 merges them, which is harmless. |
| STG-23 | agrees | |
| STG-24 | agrees | |
| STG-25 | classification doubt | The reference gives no label to the reuse of a base's end padding, as the item O review found for CLS-31. The byte-copy sentence goes beyond the reference (Q-0357). |
| STG-26 | agrees in part | The reference makes the value representation of pointers and of floating types implementation-defined. For integers it fixes the value of each bit but gives no label to the order of the bytes. |
| STG-27 | agrees, one doubt | Indirection and deallocation are undefined, and every other use is implementation-defined. The reference speaks of pointers to any part of the ended storage. It is not explicit about a pointer just past the end, which STG-27 includes. |
| STG-28 | agrees | See Q-0323. |
| STG-29 | agrees | See Q-0324. |
| LSUP-P1 to LSUP-P3 | outside the reference | See "Names that the program declares in the standard namespace". |
| LSUP-1 | outside the reference, one shape doubt | The reference fixes only the public interface of the initializer list template, not a pointer member and a count member. See Q-0321. |
| LSUP-2 | outside the reference, one extra condition | The member names of the three ordering classes match the library. The reference needs a library declaration only of the class that a use involves. LSUP-2 asks for all three, which is EDG's behavior. |
| LSUP-3 | agrees in substance | The reference asks for a complete specialization with a member named value that is an integral constant, of any integral type. The type that the protocol looks at keeps its cv-qualifiers. Specializing the library's templates is allowed in the reference, and declaring the primary templates is not. |
| LSUP-4 | agrees | The three names have these meanings in the reference. |
| LSUP-5 | outside the reference | The library's enumeration has no enumerators. The reference also says that the implicit declarations of the allocation functions do not bring in the enumeration's name, which matches the last sentence. See Q-0299. |
| LSUP-6 | agrees | The shape matches the library. The nothrow forms are replaceable global allocation functions, so a new expression can use them in constant evaluation. |
| LSUP-7 | contradicts in part | The reference reserves the non-allocating placement forms to the library and forbids a program to define replacements. A program that defines them is outside the reference. The array placement form matches. |
| LSUP-8 | agrees, one missing class | The hierarchy matches. The reference also throws a bad-typeid object for a type query on a null pointer to a polymorphic class. The type query is referenced in Tidal, so the gap is small. |
| LSUP-9 | agrees | The library's byte type is a scoped enumeration over unsigned char with no enumerators. |
| LSUP-10 | missing condition | See "Constant evaluation of the construct-at function and the allocator" below. |
| LSUP-11 | missing condition | See the same section. |
| LSUP-12 | contradicts in one row | See Q-0334. The rows for the rewritten relational operators and the reversed three-way comparison agree. |
| LSUP-13 | agrees | The reference converts each part's result to the return type with an explicit cast, and the library gives the stronger class conversion functions. |

## Questions

| Question | Provisional answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0175 | A read of a member before the constructor runs is undefined (option 1). | unclear | option 1 by the normative rules | See below. |
| Q-0299 | A mismatched shape is a reject rule (option 1). | contradicts | option 2 | Adding a declaration to the standard namespace is undefined in the reference, so no diagnostic is required. |
| Q-0304 | The bytes act as an array (option 1). | unclear | option 1 by intent | See "Byte copies". |
| Q-0305 | A plain char array hosts no object (option 1). | agrees | option 1 | Only unsigned char and byte-type arrays provide storage. |
| Q-0306 | Signed char is not an allowed type (option 1). | agrees | option 1 | The list has plain char, unsigned char, and the byte type only. |
| Q-0321 | The need arises at a braced initialization or a deduction (option 1). | silent | none | The reference makes every use, named or implicit, need the library declaration. That trigger is wider than option 1 and closest to GCC. A program's own declaration is outside the reference. |
| Q-0322 | Keep the operations referenced (option 1). | silent | either | A question of Tidal's test method. The reference puts each of these operations in the library. |
| Q-0323 | A pointer from the allocation reaches element 0, and arithmetic is valid before construction (option 1). | agrees | option 1 | See below. |
| Q-0324 | A cast of the byte array's address points to a byte (option 1). | agrees | option 1 | An element of a byte array is not interconvertible with an object that the array hosts. So the cast keeps pointing to the element, and the reference points to the launder operation for this case. |
| Q-0325 | The platform decides (option 1). | silent | option 1 or 2 | The integer-to-pointer mapping is implementation-defined apart from the round trip. Nothing makes an object exist at such an address, and nothing calls the result invalid. |
| Q-0327 | A byte pointer from a member can move over the whole object (option 1). | unclear | option 1 by intent | See "Byte copies". |
| Q-0328 | A padding byte has no value until written, and a copy through plain char needs a value (option 1). | unclear | leans to option 2 or 3 | See "Byte copies". |
| Q-0334 | The class must declare equality with zero (option 1). | contradicts | option 2 | See below. |
| Q-0336 | A read of an enumeration whose bytes hold a value outside its range is undefined (option 1). | contradicts | option 2 | See below. |
| Q-0349 | Only the conversion to a pointer to the element type is valid (option 1). | silent | either | In constant evaluation the reference ignores the body of the library allocator's members. A program's own body has no standing. The library's allocate returns a pointer to the first element of an array of the element type, which fits option 1. |
| Q-0350 | Clang has a defect in both cases (option 1). | agrees in part | option 1 for the 32-bit case | See below. |
| Q-0357 | The derived member takes the value of its new bytes (option 1). | unclear | leans to option 1 | The reference's byte-copy guarantees leave out potentially-overlapping subobjects, so it promises nothing for the base. A write through unsigned char to the member's bytes is an allowed access, and the reference does not say what the member then holds. Nothing makes the copy undefined as a whole. |
| Q-0359 | The plain byte copy of a volatile member is undefined (option 1). | unclear | leans to option 1 | The reference makes undefined an access to an object defined with a volatile type through a glvalue that is not volatile, and it has no exception for byte types. Whether a member counts as defined with a volatile type, and whether the bytes of the object representation count as volatile, is not stated. |

### Lifetime bounds

- In the reference, the lifetime of a class object ends when its
  destructor call begins. The rules for the period of destruction then
  allow the destructor body to use the members and bases. OBJ-1 and
  the OBJ-P1 diagram place the end after the destructor completes. The
  difference shows in what a pointer to the object may do from outside
  the destructor while the destructor runs.
- In the reference, every lifetime begins when storage is obtained and
  the initialization, if any, is complete. Default initialization that
  does nothing counts as complete at once. So a scalar declared with an
  initializer is outside its lifetime while its initializer runs. OBJ-1
  starts every scalar's lifetime with its storage. A write to the
  scalar through a pointer during its own initializer is undefined in
  the reference and valid under OBJ-1.
- OBJ-1 starts a class object's lifetime when its constructor
  completes. The reference speaks of initialization. An aggregate
  initialized from a braced list, and an object created implicitly,
  have no constructor call.
- OBJ-1 starts a delegating constructor's object when the target
  constructor completes. The reference does not tie the lifetime to
  that point. It only runs the destructor if the delegating body exits
  by an exception. The lifetime itself begins when the whole
  initialization, delegating body included, is complete.
- The reference ends a scalar's lifetime when the scalar is destroyed,
  which includes a pseudo-destructor call. OBJ-1's table names only the
  release and reuse of storage. The OBJ-P1 table and DTOR-6 cover the
  explicit call, so the gap is in OBJ-1's table only.
- The reference also has union rules: a member's lifetime begins only
  when it becomes the active member. UNI rules cover them elsewhere.

### Static and thread objects

- The reference makes it implementation-defined whether the dynamic
  initialization of a namespace-scope static variable runs before main
  begins or later. A late one must happen before the first use of a function or variable from its
  translation unit. SINIT-5 states this. OBJ-4 states early
  construction as a requirement.
- The reference orders initialization by definition only for variables
  that are neither inline nor template instantiations. An inline
  variable is partly ordered, and an instantiated template variable,
  such as a class template's static member once instantiated, is
  unordered. Between translation units the order is indeterminate. OBJ-4 and
  OBJ-P2 give definition order with no exception.
- A variable that is initialized statically is destroyed in the place
  that a dynamic initialization would have given it. OBJ-4 orders
  destruction by the completion of construction, which leaves this case
  unclear.
- All thread-local objects of a thread are destroyed before any static
  object. OBJ-4 does not say so.
- The reference makes undefined the flow of control through the
  definition of a block-scope static object that termination has
  already destroyed, as can happen from the destructor of another
  static object. No rule on the page covers this case.
- An exception that leaves the initialization or the destruction of a
  static or thread-local object calls the terminate function. The
  unwinding page may cover this. OBJ-4 does not.

### Transparent replacement

- STG-9's last condition accepts two direct subobjects "of one object".
  In the reference the containing objects must be replaceable in their
  turn. The reference applies the test to the containing object even
  when the old and new members share it. So the containing object
  must be complete or itself a replaceable subobject. It must not be a
  const complete object, and it must not be a base subobject or a
  member with the no-unique-address attribute.
- So in the reference a new object in the place of a member of a base
  subobject does not take over the old member's names. STG-9 lets it.
  The same holds for a mutable member of a const complete object.
- The reference also asks, for a new object to become a member or
  element, that the containing object be within its lifetime. STG-9's
  last paragraph does not say so.

### Byte copies

- The reference allows the round trip through an array of plain char,
  unsigned char, or the byte type. The guarantee holds even for an
  object whose value is not valid. Its examples use library copy
  functions.
- The reference's rule for indeterminate values allows only unsigned
  char and the byte type to carry such a value. A hand-written copy
  loop through plain char that meets an indeterminate byte is therefore
  undefined. STG-12 takes this reading for all copies. For a copy by
  the library functions, which Tidal references, the reference allows
  plain char.
- The reference never says that a padding byte of a class has an
  indeterminate value. An automatic object with no initialization has
  one as a whole. For an initialized object, the padding is not
  addressed. A note on integer types gives padding bits an unspecified
  value that cannot trap. These points lean toward option 2 or 3 of
  Q-0328 more than option 1. No rule decides the case.
- For Q-0304 and Q-0327, the letter of the reference does not support
  the step. A cast from a pointer to an object to a pointer to unsigned
  char leaves the pointer pointing to the original object, because no
  byte object is interconvertible with it. Pointer arithmetic is then
  undefined, because the pointer's type and the element type are not
  similar. The reference still calls the object representation a
  sequence of unsigned char objects and promises byte copies, which
  need the step. A proposal to fix access to object representations
  was in committee work during the C++23 cycle and was not adopted for
  C++23, as far as I know. Option 1 matches the intent and every
  implementation. The reference does not settle the matter.

### Constant evaluation of the construct-at function and the allocator

- The reference treats a call of the construct-at function in constant
  evaluation as only the constructor call. It does so only when the
  pointer argument targets allocator storage, or an object that started
  to live during that same evaluation. LSUP-10 has no such
  condition. The ranges form of the function counts as well.
- The reference ignores the bodies of the allocator's members during
  constant evaluation only when the element type is a literal type.
  LSUP-11 has no such condition. The storage must also be freed within
  the same evaluation, which NEW-37 may already state.
- The reference's allocate starts the lifetime of the array but of
  none of its elements. This matches STG-28. An assignment to an int
  element before construction is then a write outside the element's
  lifetime, so Clang's rejection, noted in Q-0323, agrees with the
  reference.

### Names that the program declares in the standard namespace

- The reference makes it undefined for a program to add a declaration
  or definition to the standard namespace. It allows specializations
  of library templates only in stated cases. It reserves the
  non-allocating placement forms to the library.
- For the initializer list template and the three ordering classes,
  the reference makes a program ill-formed when no declaration from
  the library precedes a use. Implicit uses count, such as a deduction
  or a built-in three-way comparison. A declaration that the program
  writes is not a library declaration.
- So LSUP-P1 to LSUP-P3 and the shape rules describe programs that the
  reference does not define. The reject classification of LSUP-1,
  LSUP-2, LSUP-5, LSUP-12, and LSUP-13 has no counterpart. Decision
  0041 makes this a Tidal choice. Where the rules state a library
  shape, the shapes mostly match. The exceptions are the private shape
  of the initializer list template (LSUP-1), the enumerators that
  LSUP-5 allows, and the inline definitions of LSUP-7.
- The destroying form of operator delete needs a library tag type.
  LSUP does not list that type, though the constructors page has the
  destroying form.

### Q-0175 in more detail

- By the normative lifetime rule, a class object with a dynamic
  initializer has not begun its lifetime before that initializer
  completes. A footnote of the lifetime rules gives exactly this case
  as an example of storage that exists before its object. A read of a
  member then is an access outside the lifetime, which supports
  option 1.
- A note in the rules for static initialization says that an earlier
  initializer that reads a later object may see the zero-initialized
  value. The note speaks of objects in general, and its example uses a
  scalar. That note fits option 2, and it is the only support for the
  zero read that SINIT-2 gives for a scalar.
- Read strictly, the lifetime rule also makes SINIT-2's scalar read
  undefined. The reference is inconsistent here. A core issue on the
  lifetime of zero-initialized objects before their dynamic
  initialization was open after C++23, as far as I know.

### Q-0323 in more detail

- An allocation by name implicitly creates an array of the element
  type. That array is an implicit-lifetime type whatever the element
  type is. Its elements exist, and their lifetimes do not start when
  the element type is not implicit-lifetime.
- The pointer limits outside a lifetime do not include pointer
  arithmetic, so stepping over unbuilt elements is valid.
- The allocation gives back a pointer to one of the created objects
  that starts at the first byte of the storage. Element 0 qualifies.
  A conversion from the untyped pointer to a pointer to the element type then reaches
  element 0. The reference says this through its intent rather than in
  a single rule, because an array and its first element are not
  interconvertible. The library's allocator says outright that it
  returns a pointer to the first element.

### Q-0334 in more detail

- The reference finds the first differing part of a defaulted
  three-way comparison by applying inequality against zero to each
  part's result, converted to a truth value. Clang and EDG follow
  this.
- With only equality against zero, inequality still works through the
  rewritten form. With only inequality, it works directly. So the
  reference accepts a class with either operator, which is option 2.
  GCC's use of equality is a departure that the library's classes hide,
  because they have both operators.
- The question is about a program-declared ordering class, which the
  reference does not cover. Within the reference's own logic, the
  answer is option 2.

### Q-0336 in more detail

- The reference gives an enumeration the object and value
  representation of its underlying type, and a value is stored with the
  same bits in either type. A footnote to the definition of the
  enumeration's values says that the range is for promotions and
  conversions. It does not stop an expression of the enumeration type
  from holding a value outside the range.
- The undefined case in the reference is the conversion of an
  out-of-range value to an enumeration with no fixed underlying type.
  No rule makes the later read undefined. A value placed by a byte copy
  reads back as that value, which is option 2.
- The sanitizers check the read because that is where they can see the
  value. Their report does not show that the reference makes the read
  undefined.

### Q-0350 in more detail

- The reference makes a type ill-formed when its object representation
  needs more bytes than the size type can count. The 32-bit case is
  such a type. Clang accepts it and gives a reduced size, which
  conflicts with the reference. Option 1 holds for that case.
- The 64-bit case of Clang 18 is a class that fits the size type but
  passes Clang's own limit, and its size comes out wrong. The reference
  asks for correct execution only within the implementation's resource
  limits. It does not say what an implementation must do with a program
  beyond them. So this case is less clear. A diagnostic is the better
  behavior, but the reference does not plainly require it.
