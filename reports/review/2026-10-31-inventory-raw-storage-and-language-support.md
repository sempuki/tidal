<!--
Behaviour inventory, 2026-10-31: objects in raw storage and object
representation (phase 5 item F1), and language-support types, round 2
(item F2).
Written by: a source reader under AGENTS.md that read the C++ working
draft sources and Tidal's own pages. It did not write or change any file
in this repository; the integrator copied this summary here.
Checked before use: the source reader ran tools/similarity.py on its
summary outside the repository (seven passages reworded); the
integrator's own check gives 0 flagged. No code, no quotes, no section
names. Each item is a behaviour to probe, not a rule.
-->

# Behaviour inventory for phase 5, items F1 and F2

Role: source reader. Each item below is a description of language
behaviour in the source reader's own words. Groups and labels are the
source reader's own. Items are sorted alphabetically by label within
each group. In this inventory, "byte buffer" means an array whose
element type is the unsigned narrow character type or the library's
byte enumeration type.

## Part F1: objects in raw storage and object representation

### F1 categories (probe axes)

1. Access type. The glvalue type used for a read or write: the same
   type as the object, the same type with other cv-qualifiers, the
   signed or unsigned partner of an integer type, a plain or unsigned
   narrow character type, the library byte type, the signed narrow
   character type, a base or derived class, or an unrelated type.
2. Creation route. A placement new expression, a call to a global or
   class allocation function, the start of a byte buffer's lifetime, a
   C allocation routine, a byte copy routine, the library's bit cast,
   the library's explicit lifetime start functions, or an assignment
   that switches the active member of a union.
3. Evaluation mode. A run-time program, constant evaluation, or a run
   under a sanitizer.
4. Handle to the new object. The old variable name, an old pointer or
   reference, the pointer that placement new returns, the pointer that
   the launder operation returns, or a pointer from a reinterpret cast.
5. Kind of created type. A scalar, an aggregate with no user-provided
   destructor, a class with a trivial eligible constructor and trivial
   destructor, a class with a non-trivial destructor, a polymorphic
   class, an array, or a const-qualified complete object.
6. Old occupant. None, an object of the same type that overlays the
   storage exactly, an object of another type, an object that overlaps
   only in part, or an object that encloses the new one.
7. Pointer arithmetic domain. Elements of a declared array, the
   position one past the end, a single object treated as an array of
   one, bytes of an object reached through a character pointer, rows
   of a multidimensional array, and an element type that differs from
   the pointer's type.
8. Storage source. A local byte buffer, a byte buffer that is a class
   member, a static byte buffer, storage from a global allocation
   function, storage from a C allocation routine, and the storage of
   an existing object whose type is not a byte type.

### F1 behaviours

#### Addresses, sizes, and layout

- ADDR-base-zero-size: when a standard-layout class has an empty base,
  that base takes no bytes, and its address can equal the
  address of another subobject of a different type.
- ADDR-bitfield-none: a bit-field has no address of its own, so the
  first-byte rule for addresses does not cover it.
- ADDR-contiguous: an object of a trivially copyable or standard-layout
  type occupies one unbroken run of bytes.
- ADDR-distinct-unless-nested: two live objects that are not
  bit-fields share an address only when one is nested inside the
  other, or when one of them is a zero-size subobject and their types
  differ.
- ADDR-first-byte: the address of an object is the address of the
  lowest byte that it occupies, except for bit-fields and zero-size
  subobjects.
- ADDR-zero-size-unspecified: the address of a zero-size subobject is
  some byte inside its complete object, and which byte is unspecified.
- ADDR-zero-size-when: an object has non-zero size unless it is a
  potentially overlapping subobject (a base, or a member marked as not
  needing a unique address) of class type with no virtual functions,
  no virtual bases, and no parts of non-zero size. When the rule
  allows zero size for a class that is not standard-layout, the
  outcome is implementation-defined.
- ADDR-too-large-type: a type whose size in bytes exceeds the largest
  value of the size type is ill-formed.

#### Implicit object creation

- IMP-allocation-call: each call of an allocation function (single or
  array form), whether the program writes the call or a new expression
  makes it, creates objects implicitly in the storage it returns
  and yields a pointer to a fitting one of them.
- IMP-byte-buffer-start: when the lifetime of a byte buffer begins,
  objects are created implicitly inside it, and the buffer hosts them.
- IMP-choice-defined: the implicitly created set is whatever set of
  objects of implicit-lifetime types makes the program's behaviour
  defined. The set can be empty.
- IMP-choice-none: if no set of implicitly created objects would make
  the behaviour defined, the behaviour is undefined.
- IMP-choice-unspecified: if several sets would each make the
  behaviour defined, which set is created is unspecified.
- IMP-implicit-lifetime-class: a class counts as implicit-lifetime if
  it is an aggregate whose destructor the user did not provide. A class
  also counts if its destructor is trivial and not deleted, and one or
  more of its eligible constructors are trivial.
- IMP-implicit-lifetime-types: scalars, implicit-lifetime classes,
  every array type, and cv-qualified forms of these are the types that
  implicit creation can produce. An array of a class with a
  non-trivial constructor is still such a type, but its elements are
  not created.
- IMP-library-sources: some library facilities create objects
  implicitly. These include the C allocation routines, the byte copy
  and byte move routines, the bit cast facility, and the allocator
  traits.
- IMP-lifetime-start-array: the library's explicit lifetime start for
  arrays with a count of zero has no effect and returns a pointer equal
  to its argument. With a positive count it acts as the single-object
  form for an array of that many elements.
- IMP-lifetime-start-explicit: the library's explicit lifetime start
  function creates an object of the named implicit-lifetime type at
  the given address, keeps the bytes already in the storage as its
  object representation, and gives each trivially copyable part the
  value those bytes encode. Other parts get unspecified values, which
  may be indeterminate.
- IMP-malloc-struct: storage from a C allocation routine, cast to a
  pointer to a simple aggregate and then written through member
  access, has defined behaviour because an aggregate object is created
  implicitly.
- IMP-nontrivial-subobjects: implicit creation does not start the
  lifetime of a subobject whose own type is not implicit-lifetime.
- IMP-pointer-choice: when an operation must yield a pointer to a
  created object, it picks one whose address is the start of the
  region and which makes behaviour defined. If none works, the
  behaviour is undefined. If several work, the pick is unspecified.

#### Lifetime boundaries and storage reuse

- LIFE-begin-conditions: an object's lifetime begins once storage of
  the right size and alignment exists and its initialization, if any,
  has finished. Default initialization with a trivial default
  constructor counts as finished at once.
- LIFE-const-static-reuse: a complete object that is const and is not
  dynamic (so its duration is automatic, static, or thread) must never
  have another object built in its bytes, during or after its life.
  Doing so is undefined.
- LIFE-end-class: the lifetime of a class object ends when its
  destructor call starts.
- LIFE-end-nonclass: the lifetime of a non-class object ends when the
  object is destroyed.
- LIFE-end-release: any object's lifetime ends when its storage is
  released.
- LIFE-end-reuse: any object's lifetime ends when another object that
  is not nested in it takes over its storage. A byte buffer that
  hosts a new object therefore keeps living.
- LIFE-no-destructor-call: ending a class object's lifetime by reusing
  or releasing its storage does not call its destructor. A program
  that relies on the destructor's effects then misses them.
- LIFE-reference: a reference's lifetime begins when its
  initialization finishes and ends as a scalar object's lifetime
  would.
- LIFE-scoped-type-mismatch: if a program ends an automatic, static,
  or thread object whose type has a non-trivial destructor, and an
  object of that same type is not in the storage when the implicit
  destructor call happens, the behaviour is undefined. Leaving the
  scope by an exception counts.
- LIFE-thread-ordering: before and after in the lifetime rules follow
  the happens-before order, so using an object that another thread is
  still constructing, without synchronization, is undefined.
- LIFE-union-member: a union member's lifetime starts only when that
  member becomes the active one.

#### Object representation and value representation

- REP-byte-round-trip: the bytes of a trivially copyable object that
  is not a potentially overlapping subobject can be copied into a
  buffer of plain char, unsigned narrow character type, or the library
  byte type. Copying them back restores the original value, even when
  the object did not hold a valid value.
- REP-copy-between-objects: copying all bytes of one trivially
  copyable object into a second distinct object of the same type gives
  the second object the same value, provided neither is a potentially
  overlapping subobject.
- REP-indeterminate-byte-copy: an indeterminate value of unsigned
  narrow character type or library byte type can be copied through a
  conditional operand, a comma right operand, a cast to such a type, a
  discarded expression, a simple assignment to such an lvalue, or an
  initialization of such an object. The result is still indeterminate.
  Any other use of an indeterminate value is undefined.
- REP-indeterminate-on-obtain: an object with automatic or dynamic
  storage has an indeterminate value from the moment its storage is
  obtained until something initializes or assigns it.
- REP-object-bytes: the object representation of a type is the run of
  bytes, as many as the type's size, that an object of the type takes
  up, viewed as unsigned narrow characters.
- REP-padding: bits of the object representation that do not take
  part in the value are padding bits.
- REP-trivially-copyable-values: for a trivially copyable type, the
  value bits select one element of an implementation-defined set of
  values.
- REP-value-bits: the value representation is the subset of bits that
  carry the value of the type.

#### Pointer arithmetic over arrays and storage

- ARITH-element-type-mismatch: addition or subtraction is undefined
  when the pointer's pointee type is not similar to the element type
  of the array it points into. Reading through a reinterpreted pointer
  to the signed or unsigned partner type is allowed, but stepping it
  is not.
- ARITH-hypothetical-end: one past the last element acts as a
  hypothetical element at index n, and arithmetic may reach it from
  inside the array or start from it.
- ARITH-in-range: adding j to a pointer at element i gives element
  i plus j when the result lies from 0 to n inclusive. Subtraction
  works the same way in the other direction. Any other result is
  undefined.
- ARITH-null-zero: adding or subtracting zero to a null pointer gives
  a null pointer. Subtracting two null pointers gives zero.
- ARITH-single-object: a non-array object acts as an array of one
  element, so only offsets 0 and 1 are defined from a pointer to a
  complete object, a member, or a base.
- ARITH-subtract-range: subtracting pointers to elements i and j of
  one array gives i minus j. The behaviour is undefined if the two
  pointers point into different arrays, or if the result does not fit
  in the pointer difference type.
- ARITH-unknown-bound: arithmetic on a pointer to an array of unknown
  bound, or to an incomplete class, is ill-formed. A pointer to a
  pointer to such a type can still be stepped.

#### Pointer values, interconvertibility, and reachability

- PTR-array-not-first-element: an array and its first element have
  the same address, but a pointer to one cannot be reinterpreted into
  a usable pointer to the other.
- PTR-four-kinds: a pointer value either points to an object or
  function, points one past an object, is the null value of its type,
  or is invalid.
- PTR-interconvertible-pairs: two objects count as interconvertible
  (same address, and a reinterpret cast of a pointer to one gives a
  usable pointer to the other) when they are the same object, when one
  is a union and the other one of its members, when one is a
  standard-layout object and the other its first member or any of its
  base subobjects, or by a chain of such links.
- PTR-invalid-after-storage-end: once the storage that a pointer
  designates is gone (its duration over), the pointer value is
  invalid.
- PTR-layout-compatible: pointers to layout-compatible types share
  value representation and alignment.
- PTR-past-end-not-neighbour: a pointer one past an object does not
  point to an unrelated object that happens to start at that address.
- PTR-reach-bytes: the bytes reachable through a pointer to an object
  are the bytes of any object interconvertible with it. If such an
  object is an array element, the bytes of the whole enclosing array
  are reachable.
- PTR-represents-address: a pointer to or one past an object stands
  for the address of the object's first byte or of the byte after the
  object. For an object outside its lifetime, the address is where it
  will be or was.
- PTR-void-representation: a pointer to void can hold any object
  pointer and has the same representation and alignment as a pointer
  to plain char.

#### Storage hosting and nesting

- HOST-byte-buffer-conditions: a complete object created inside a live
  byte buffer is hosted by that buffer when it fits entirely inside the
  buffer and no smaller byte buffer inside the outer one also meets
  these conditions.
- HOST-chain: a host relation and a subobject relation can be chained.
  An object hosted in a byte buffer that is a member of an object
  hosted in another buffer is nested in every outer object in the
  chain.
- HOST-complete-object-of: every object has one complete object. A
  complete object is its own complete object, and a subobject takes
  the complete object of the object that contains it.
- HOST-ends-previous: placing a new object in part of a byte buffer
  that already hosted another object ends the old object's lifetime.
  The buffer itself keeps living.
- HOST-element-overlay: an object created exactly over a member or an
  array element of a live object, with the same type ignoring
  cv-qualifiers, becomes that member or element of the containing
  object.
- HOST-most-derived: a complete object, member, or array element of
  class type has that class as its most derived class. An object of
  such a class, or of a non-class type, is a most derived object.
- HOST-nested: an object is nested in another when it is a subobject
  of the other, when the other hosts it, or through a chain of these
  two relations.
- HOST-non-byte-element: only arrays of unsigned narrow characters or
  of the library byte type can host other objects. An array of plain
  char or signed char does not host.

#### Transparent replacement and use outside lifetime

- OUT-glvalue-limits: before an object's lifetime starts (with its
  storage present) or after it ends (with its storage not yet reused),
  a glvalue for it may be used only for properties that do not depend
  on its value. Reading or writing through it, calling a non-static
  member function, binding it to a reference to a virtual base, or
  using it as an operand of dynamic cast or typeid is undefined.
- OUT-pointer-limits: in the same window, a pointer to the storage
  acts like a pointer to void. Deleting through it, member access or a
  member function call through it, converting it to a pointer to a
  virtual base, a static cast to anything except a pointer to void
  (optionally followed by a cast to a character or byte pointer), or a
  dynamic cast on it is undefined. Indirection alone is allowed.
- REPL-auto-rebind: if a new object is created where an ended object
  was, and the old object is replaceable by the new one, the old name,
  pointers, and references refer to the new object once its lifetime
  starts.
- REPL-conditions: replacement is seamless only if the new object
  exactly covers the old storage, has the same type ignoring
  top-level cv-qualifiers, the old object is not a const complete
  object, neither is a base or a member marked as not needing a unique
  address, and either both are complete objects or both are direct
  subobjects of enclosing objects that are themselves seamlessly
  replaced.
- REPL-launder-fallback: when replacement is not seamless, the launder
  operation, applied to an old pointer that holds the right address,
  yields a usable pointer to the new object.
- REPL-self-assignment-idiom: an assignment operator that destroys
  the object and then constructs a new one of the same type in place
  leaves the original name usable when the object is a complete object
  that is not const.

#### Type-access rules

- ALIAS-allowed-types: a read or write of an object's stored value is
  defined only through a glvalue whose type is similar to the object's
  dynamic type, to the signed or unsigned partner of that type, or to
  plain char, unsigned narrow character, or the library byte type.
- ALIAS-class-no-access: copying or assigning a class object is not an
  access of the class object as a whole. Only the scalar parts are
  accessed.
- ALIAS-signed-char-absent: the signed narrow character type is not in
  the allowed list. Reading another object's bytes through it is
  undefined by the rule.
- ALIAS-union-copy: invoking a defaulted copy or move constructor or
  assignment of a union with an argument that does not refer to a live
  object of that union type is undefined.

### F1 gaps and doubts

- Byte-wise pointer arithmetic: the arithmetic rules speak only of
  array elements. A character pointer obtained from an object that is
  not a byte array has no array to step through, yet byte copies and
  byte inspection need it. Every compiler allows such stepping in
  practice, and constant evaluation may reject it. Tidal will need a
  provisional position.
- Constant evaluation: Clang and GCC reject reinterpret casts in
  constant expressions, so the type-access rules and implicit creation
  can be probed only at run time. Recent compilers accept placement
  new and a cast from a pointer to void in constant evaluation. They
  may differ on placement into storage of another type and on reuse
  of a byte buffer.
- Implicit creation detection: no compiler reports a violation of the
  implicit-creation rules. Probes can show only that programs run as
  expected, and an optimizer may still exploit the rule.
- Library lifetime start functions: support for the explicit lifetime
  start functions arrived late in the standard libraries. GCC 14 and
  older libraries may lack them. Probes should check for the feature
  test macro.
- Optimizer exploitation of type-access rules: GCC and Clang both
  assume the rules at higher optimization levels, and the outcome of a
  forbidden access changes with the optimization level. Sanitizer
  support for type access is new in Clang (the type sanitizer) and
  absent in GCC.
- Plain char hosting: the hosting rule names only the unsigned byte
  types, while the type-access rule includes plain char. Compilers do
  not distinguish these cases in code generation.
- Signed char access: GCC and Clang treat signed char like the other
  character types for type access. The rule excludes it.
- Transparent replacement of const members: older drafts forbade
  rebinding when a class had const or reference members. The current
  rule does not. Compilers may still treat const members as constant
  across placement new.
- Zero-size subobjects: the size rule for empty classes that are not
  standard-layout is implementation-defined. The member attribute for
  no unique address is ignored by MSVC and honoured by GCC and Clang.

### F1 already covered

- ADD-6, ADD-7, ADD-8, ADD-10, ADD-11, ADD-14 cover ARITH-in-range,
  ARITH-hypothetical-end, ARITH-single-object, ARITH-subtract-range,
  ARITH-null-zero, and ARITH-element-type-mismatch.
- ADD-3 covers ARITH-unknown-bound in part (complete pointee type).
- ADD-12 and LSUP-4 cover the pointer difference type.
- CAST-6 covers the reinterpret cast to a reference without the
  type-access limits.
- CLS-10 and the attribute page cover ADDR-base-zero-size and member
  order in part.
- CLS-19 covers PTR-interconvertible-pairs for the first member of a
  standard-layout class only.
- CPY-19 covers REP-copy-between-objects, without the restriction on
  potentially overlapping subobjects and without the round trip
  through a byte buffer.
- INIT-1 covers REP-indeterminate-byte-copy for a copy into an
  unsigned narrow character or byte object.
- NEW-3 and OBJ-2 cover reading an object before it has a value.
- OBJ-1 covers LIFE-begin-conditions and LIFE-end-class.
- OBJ-7 covers ADDR-distinct-unless-nested for complete objects and
  array elements.
- OBJ-10 covers LIFE-end-reuse for a read through an old reference.
- TYP-1, TYP-2, TYP-12 cover sizes, alignments, and padding on the
  matrix targets.
- UNI-1 and UNI-5 cover union member addresses and LIFE-union-member.
- Not covered: hosting by byte buffers, nesting, implicit object
  creation, implicit-lifetime types, seamless replacement, the
  launder fallback, the limits on use outside a lifetime, the
  type-access rules, object and value representation, padding bits,
  reachability, and pointer value kinds.

## Part F2: language-support types, round 2

### F2 categories (probe axes)

1. Alignment of the allocated type. Fundamental alignment, alignment
   above the default new alignment, and alignment above the largest
   fundamental alignment.
2. Declaration source. The library header, a declaration that the
   program writes itself in namespace std, or no declaration at all.
3. Entity. The aligned allocation and deallocation forms, the nothrow
   forms and the nothrow tag, the non-allocating placement forms, the
   alignment enumeration, the allocation failure exception, the array
   length exception, the failed dynamic cast exception, the failed
   typeid exception, the common exception base, the new handler
   functions, and the launder operation.
4. Evaluation mode. Run time and constant evaluation.
5. Replacement status. The library default, or a program-supplied
   replacement of a global function.
6. Use site. A new expression, a delete expression, a direct call by
   name, a catch handler, and a typeid or dynamic cast operand.

### F2 behaviours

#### Allocation forms with alignment

- ALGN-array-rule: an unaligned array allocation form returns storage
  aligned for any object of fundamental alignment that fits in the
  requested size.
- ALGN-chosen-for-new-extended: a new expression for a type whose
  alignment exceeds the default new alignment passes the type's
  alignment as a second argument of the alignment enumeration type,
  before any placement arguments.
- ALGN-default-new-alignment: the implementation reports the default
  new alignment through a predefined macro. A type with greater
  alignment is new-extended.
- ALGN-fallback-add: if no allocation function matches the argument
  list built for a type that is not new-extended, overload resolution
  runs again with the type's alignment inserted after the size.
- ALGN-fallback-remove: if no function matches the argument list
  built for a new-extended type, overload resolution runs again with
  the alignment argument removed.
- ALGN-implicit-declarations: besides the plain forms, every
  translation unit has implicit global declarations of the aligned
  allocation forms and of the aligned deallocation forms, with and
  without a size, for single objects and arrays.
- ALGN-invalid-value: passing an alignment that is not a valid
  alignment to a library allocation or deallocation function is
  undefined.
- ALGN-names-not-introduced: the implicit declarations introduce only
  the operator names. Naming the namespace std, the size type, or the
  alignment enumeration still needs a library declaration first.
- ALGN-result-aligned: an allocation function that takes an alignment
  argument returns storage aligned to that value.
- ALGN-single-rule: an unaligned single-object allocation form returns
  storage aligned for any object of fundamental alignment that has the
  requested size.

#### Alignment enumeration type

- AVT-scoped-sized: the alignment enumeration is a scoped enumeration
  with the size type as its fixed underlying type and no enumerators.
- AVT-usual-deallocation: a usual deallocation function may take,
  after its pointer, an optional destroying tag, then an optional size,
  then an optional alignment argument, in that order.
- AVT-value-construct: a program creates an alignment value by a cast
  or by direct initialization from an integer, because the enumeration
  has no enumerators and no implicit conversion from integers.

#### Allocation failure and array length exceptions

- BADA-base: the allocation failure exception derives publicly from
  the common exception base.
- BADA-handler-match: a throwing allocation function that fails
  throws an exception that a handler for the allocation failure type
  would catch, and never returns null.
- BADA-message: the message of the allocation failure exception is an
  implementation-defined string.
- BADL-base: the array length exception derives publicly from the
  allocation failure exception, so a handler for allocation failure
  catches it.
- BADL-constant-bound: a bad bound that the compiler can compute as a
  constant, in an expression that is evaluated, makes the program
  invalid at compile time.
- BADL-erroneous-bound: a new expression whose run-time bound is
  negative, too large for the size type, too large for the
  implementation, or smaller than the number of initializers, calls no
  allocation function. It throws an exception matching the array
  length type, or yields null when the chosen allocation function does
  not throw.
- BADL-message: the message of the array length exception is an
  implementation-defined string.

#### Dynamic cast and typeid exceptions

- BADC-base: the failed dynamic cast exception derives publicly from
  the common exception base.
- BADC-message: its message is an implementation-defined string.
- BADC-reference-cast: a dynamic cast to a reference type that fails
  its run-time check throws an exception that a handler for the failed
  dynamic cast type would catch.
- BADT-base: the failed typeid exception derives publicly from the
  common exception base.
- BADT-message: its message is an implementation-defined string.
- BADT-null-deref: typeid applied to the result of dereferencing a
  null pointer to a polymorphic class throws an exception that a
  handler for the failed typeid type would catch. Extra parentheses
  around the dereference do not change this.
- BADT-other-null: when the operand is not a dereference of a null
  pointer of polymorphic class type, typeid does not evaluate a null
  check and does not throw.

#### Common exception base

- EXC-copy-preserves-message: copying a library exception, by
  construction or assignment, keeps the message unchanged when both
  objects have the same dynamic type.
- EXC-members-nothrow: each library exception class has a public
  default constructor, copy constructor, and copy assignment, and none
  of them throws.
- EXC-virtual: the common base has a virtual destructor and a virtual
  message function that does not throw, so a handler for the base can
  read the message of any derived exception.

#### Launder operation

- LAUN-constant: a call of the launder operation may appear in a
  constant expression exactly when its argument could appear in its
  place.
- LAUN-ill-formed-types: the launder operation rejects a function type
  or void as the pointee type at compile time.
- LAUN-precondition-live: the argument must hold the address of a
  live object whose type is similar to the pointee type. Otherwise the
  behaviour is undefined.
- LAUN-precondition-reach: every byte reachable through the result
  must already be reachable through the argument. Laundering a pointer
  to one array element into a pointer to an enclosing object that is
  larger is undefined.
- LAUN-result: the result points to the live object at that address,
  with the argument's type.
- LAUN-use-base: after a base subobject is replaced in place, a
  laundered pointer reaches the new base while the old pointer does
  not.
- LAUN-use-const: after a const complete object is replaced in place,
  a laundered pointer reaches the new object while the old pointer
  does not.

#### New handler

- NH-get-set: setting the new handler returns the previous handler,
  and getting it returns the current one. The initial handler is null.
- NH-loop: the library's throwing allocation function retries in a
  loop. On failure it throws if the handler is null and calls the
  handler otherwise. The loop ends on success or when the handler does
  not return.
- NH-obligations: a new handler must make more storage available and
  return, throw an allocation failure exception or a class derived
  from it, or end the program.

#### Nothrow forms and tag

- NT-delete-on-throw: if the constructor in a nothrow new expression
  throws, the implementation calls the matching nothrow deallocation
  form, which forwards to the plain form.
- NT-forwarding: the default nothrow allocation forms call the
  corresponding throwing form, return its result, and return null if
  it throws.
- NT-null-result: when a nothrow allocation returns null, the new
  expression performs no initialization, calls no deallocation
  function, and yields a null pointer.
- NT-tag-type: the nothrow tag is an empty class with an explicit
  default constructor, and the library supplies one constant object of
  that class.

#### Non-allocating placement forms

- PLC-array-no-overhead: an array new expression that uses the
  library's array placement form adds no array allocation overhead.
  Every other array new expression, including one with a
  program-defined placement form, may add an unspecified
  non-negative overhead that can differ between calls.
- PLC-byte-array-offset: for an array new of plain char, unsigned
  narrow character, or the library byte type, the overhead is a
  multiple of the strictest fundamental alignment of any type no
  larger than the array.
- PLC-delete-forms: the placement deallocation forms do nothing. The
  implementation calls them only when initialization in a placement
  new expression that used the library form throws.
- PLC-no-dynamic-rules: the general rules for dynamic allocation
  functions, such as non-null distinct results and implicit object
  creation, do not apply to the non-allocating forms.
- PLC-not-replaceable: a program must not define replacements for the
  non-allocating placement forms.
- PLC-null-result-undefined: if a non-allocating placement form
  returns null, as it does when given a null argument, the behaviour
  is undefined.
- PLC-returns-argument: the non-allocating placement forms for single
  objects and arrays return their pointer argument and do nothing
  else.

#### Replaceable global functions

- REPLC-delete-null: a library deallocation function given null does
  nothing.
- REPLC-delete-throws: a deallocation function that ends by throwing
  has undefined behaviour.
- REPLC-sized-pairing: if a program replaces an unsized deallocation
  form, it should also replace the sized one. If it replaces a sized
  form, it must also replace the unsized one.
- REPLC-sized-swap: an implementation may call the unsized
  deallocation form in place of the sized one.
- REPLC-thread-safety: the library and replacement allocation and
  deallocation functions do not introduce data races. Calls that
  allocate or free a given unit of storage occur in one total order.
- REPLC-zero-size: a zero-size request may fail. If it succeeds, the
  result is a distinct non-null pointer, and indirection through it is
  undefined.

### F2 gaps and doubts

- Aligned allocation without a header: a program that declares the
  alignment enumeration itself may not satisfy every compiler. GCC
  checks other self-declared support types against its own header
  (see the LSUP-1 evidence). Probes should try a self-declared
  enumeration.
- Aligned new flags: GCC enables aligned allocation from C++17 with an
  option to turn it off. Clang may diagnose aligned allocation on
  targets whose C library lacks it. The default new alignment is 16 on
  x86-64 Linux and may be 8 or 16 with -m32.
- Array length exception: Clang and GCC throw at run time for a bad
  bound. Older GCC versions returned a null pointer or threw the
  allocation failure type. EDG does not run programs, so only the
  compile-time half is testable there.
- Constant evaluation: a failed reference dynamic cast and a typeid of
  a dereferenced null pointer cannot throw during constant evaluation,
  so compilers reject them. Clang and GCC accept the launder operation
  in constant evaluation.
- Exception classes without a header: catching the failed dynamic
  cast or failed typeid types needs the type information header. The
  core rules name the types but give the program no declaration.
  Compilers may reject typeid without that header.
- Fallback overload resolution: compilers may differ when a class
  declares only an aligned allocation form and allocates a type of
  ordinary alignment, or the reverse.
- Messages: the strings returned by the message functions differ by
  library and are implementation-defined.
- Placement array overhead: GCC and Clang add a cookie for an array of
  a class with a non-trivial destructor even with a program-defined
  placement form. With the library array placement form the rule
  allows no cookie. Probes should compare the returned pointer with
  the argument in both cases.
- Sized deallocation: GCC enables sized deallocation by default since
  C++14. Clang enabled it by default only in recent versions. A probe
  that counts calls of the sized form will see differences.

### F2 already covered

- LSUP-4 covers the size type and pointer difference type that the
  alignment enumeration and the placement forms use.
- NEW-2 covers a zero-length array new and part of REPLC-zero-size.
- NEW-4 covers deleting a null pointer.
- NEW-10 and NEW-20 cover placement arguments and the need for a
  declared placement form.
- NEW-14 covers the freedom to merge or omit allocation calls.
- NEW-17 covers a negative constant bound (BADL-constant-bound in
  part).
- NEW-19 covers the implicit global declarations of the unaligned
  forms only. The aligned forms are missing.
- NEW-21 to NEW-24 cover the choice of deallocation function after a
  throw, without the nothrow forms and without the alignment argument.
- NEW-P3 states that a failed global allocation throws a type that
  the library references.
- VIRT-10 covers BADC-reference-cast, with the exception type left to
  the library.
- Not covered: the aligned forms and the alignment enumeration, the
  nothrow forms and tag, the non-allocating placement forms and their
  overhead rule, the allocation failure and array length exceptions,
  the exception hierarchy, the failed typeid exception and typeid
  itself, the new handler, and the launder operation.
