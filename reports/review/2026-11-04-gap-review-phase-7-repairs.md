# Gap review: rules that the phase 7 front-end and Propose repairs rewrote

Date: 2026-11-04
Role: source reader

Reference: the C++23 working draft in the local reference tree. The
review compares the rules that commits 2eb7c8d and 966aa1b rewrote,
read at HEAD in their pages, and the eight listed questions with that
draft only. COND-16 is new in commit 2eb7c8d and is reviewed with
COND-11. For layout facts that the reference leaves to the
implementation, the review checks only what the reference fixes. Later
defect resolutions are mentioned only where the compilers may follow
them.

## Result

Five of the eight provisional answers agree with the reference
(Q-0370, Q-0373, Q-0374, Q-0376, and the main answer of Q-0369).
Q-0371 contradicts the reference, which supports option 2. Q-0375
agrees in its result for the case in dispute, but the reference does
not use the test that NS-15 states. Q-0372 is not settled by the
reference, and what the reference does say leans toward option 2.

The rules mostly agree. The gaps are these:

- UNW-1 destroys the result of a call after the local objects that the
  exception leaves. The reference destroys the result before the local
  objects that were built before it.
- NS-15 forbids a qualified definition inside the namespace that
  declares the member. The reference allows it (Q-0371, Q-0375).
- LSP-4 and LNK-5 treat a C function or variable in an unnamed
  namespace as a C name with external linkage. In the reference such a
  name has internal linkage and so no language linkage at all.
- SB-5 leaves out a condition on the member lookup of the get function.
- COND-16 covers only an operand of class type. The reference also
  converts an operand of non-class type to the class type of the other
  operand, and it treats two operands of one class type, or of a class
  and its base, in a special way.
- NEW-22 says either global form can be called. The reference fixes
  the sized global form for a single object of complete type, as UNW-3
  already says.
- CTOR-12 leaves out the type-identification operator, the dynamic
  cast, and calls made indirectly from the base initializer.
- DEFARG-2 does not exclude member functions of class templates, whose
  default arguments the reference allows only on the first declaration
  in the class template.
- ALN-1 counts every variable declaration. The reference excludes the
  declaration of a handler's exception variable.
- FN-7 and CMP-14 are recommendations that the reference does not make.
  Both are Tidal's own advice, which their pages may already intend.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| UNI-7 | agrees, one doubt | The reference makes an anonymous union in a class an unnamed object that is a non-static data member, and its members are variant members. A member initializer or a default member initializer of a variant member makes that member the initialized one. See "UNI-7 and braces" below. |
| STG-8 | agrees | The reference creates objects at the operation, as many as give the program defined behavior, and leaves the choice among several such sets unspecified. Each created object has an implicit-lifetime type, and the creation does not start the lifetime of subobjects of other types. The pointer that an allocation function returns refers to one of the new objects, chosen among those that start where the storage starts. |
| STG-18 | agrees, one doubt | Both listed cases leave no set that works. The reference has a third undefined case, when no choice of the pointer that an allocation function returns gives defined behavior. See "STG-18 exceptions" below. |
| SB-5 | missing condition | See "SB-5" below. The index type agrees with the reference (Q-0376). |
| NEW-22 | agrees for classes, too loose for global forms | See "NEW-22" below. |
| COND-11 | agrees | The reference makes the step apply when the operands still differ in type after the conversion attempts and one is a class. The candidates are the pairs of floating-point and promoted integral types, with the result type from the usual arithmetic conversions, and one candidate per pointer, pointer-to-member, and scoped enumeration type. The operator cannot be overloaded, so only these candidates take part. |
| COND-16 | missing cases | See "COND-16" below. The order of the three attempts, the direct-binding limit on the two reference attempts, and the ambiguous case match the reference. |
| BF-9 | agrees, one doubt | The reference makes allocation and alignment of bit-fields implementation-defined, packs them into some addressable allocation unit, makes the bits beyond the width of the type padding bits, and makes an unnamed bit-field of width zero push the following bit-field to the start of a new allocation unit. Every other row is a matrix choice that the reference allows. See "BF-9 and unnamed bit-fields" below. |
| FN-7 | not in the reference | The reference makes flowing off the end of such a function undefined and recommends no warning for it. Its only nearby recommendation is a warning for a function marked as never returning that might return. FN-7 also has no exception for the main function or for a coroutine, where reaching the end is defined. |
| CTOR-12 | agrees, missing cases | The reference makes a call of a member function on the object under construction undefined when the call happens in the constructor's initializer list before the base initializers finish. The reference also covers calls made indirectly from there, and the same holds for the object as operand of the type-identification operator or of a dynamic cast. For a member not yet initialized, the reference's lifetime rule makes a call of a non-static member function undefined, so the second bullet agrees (Q-0374). |
| UNW-1 | wrong order | See "UNW-1" below. The first part agrees. |
| CE-17 | agrees | The reference forbids evaluating a goto statement and passing through the declaration of a static or thread-local variable that is not usable in constant expressions. A non-literal variable is not itself forbidden. Its destructor call must be a call of a constexpr function, so a user-provided destructor that is not constexpr ends constant evaluation even with an empty body. |
| LSP-7 | agrees, one missing classification | The reference makes a C function or variable and a variable of the global scope with one name the same entity, so the kinds or language linkages clash. A function of the global scope is not drawn in. When neither declaration is reachable from the other, as across translation units, the reference requires no diagnostic for a clash of language linkage. LSP-7 is all reject. The other bullets agree, including the ban on a C name for the main function. |
| NS-15 | contradicts in its second bullet | See "NS-15" below. The first bullet agrees, because the reference counts only declarations whose target is the namespace or a member of its inline namespace set, at any depth. |
| DEFARG-2 | missing condition, one doubt | See "DEFARG-2" below. The block rule agrees. |
| DEFARG-5 | agrees | The reference allows default arguments only in the parameter list of a function declaration, a lambda, or a template parameter, and never inside the declarator of a parameter. So pointers, references, and type aliases to functions have none. The lambda case is on another page. |
| DEFARG-9 | agrees | The reference forbids giving a default argument again in the same scope, also with the same value, and gives each scope its own set. |
| ALN-1 | agrees, one missing exclusion | See "ALN-1" below. |
| ALN-4 | agrees | The reference ties the addresses at which an object can be allocated to its alignment, for every storage duration. A conversion to a pointer type whose alignment the address does not meet gives an unspecified pointer value, so no object of that type is reached. |
| STG-23 | agrees | The reference makes an access through a type that cannot access the object undefined, and the access types are the object's own type and its similar and signed or unsigned forms, the plain and unsigned character types, and the byte type. The signed character type is not one of them. The new misaligned case follows from ALN-4. |
| NS-1 | agrees | The listed forms match the ways the reference lets a name of another namespace be used. |
| NS-P2 | agrees, one doubt | The reference treats an unnamed namespace as one that a using directive names, so a qualified use finds the direct member first, and an unqualified use in the enclosing namespace sees both. An inline namespace's members join qualified lookup through the enclosing namespace. An unqualified use in the enclosing namespace also reaches both for an inline namespace, which the table does not say. For functions, two declarations form an overload set and are not ambiguous. |
| LSP-4 | contradicts in its second bullet | See "C names in an unnamed namespace" below. The third bullet agrees (Q-0369). |
| CMP-14 | not in the reference | The reference has no recommendation about chained comparisons. The fold rule that CMP-14 cites agrees with the reference. |
| PRIM-27 | agrees, one doubt | The reference expands a fold with the parts grouped as PRIM-P6's table shows. The words "at the outer end" for the extra operand could be read as the outermost nesting level. In the reference the extra operand joins the innermost pair, at the far end of the sequence. The sentence also sits with the left fold, though a right fold can have an extra operand too. |

## Questions

| Question | Provisional answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0369 | Two entities (option 1). A C name in an unnamed namespace stays one entity with the other C names. | agrees in the main answer, contradicts the last sentence | option 1 | A name with internal linkage has no language linkage in the reference, so the rule that joins C names across namespaces does not apply. The reference's own example of such a function says the same. See "C names in an unnamed namespace" for the last sentence. |
| Q-0370 | Not valid (option 1). | agrees | option 1 | The reference makes a program ill-formed when a default argument added in a member definition outside the class makes a default, copy, or move constructor. It names copy and move assignment operators too, which cannot have default arguments anyway. |
| Q-0371 | Not valid (option 1). | contradicts | option 2 | See "NS-15" below. |
| Q-0372 | Padding bits with unspecified values, and the read is defined (option 1). | unclear, leans to option 2 | none clearly | See "Q-0372" below. |
| Q-0373 | The candidates of option 1. | agrees | option 1 | The arithmetic candidates take every floating-point type, the single-precision type included, so two operands that convert to it give that type. Each scoped enumeration type has its own candidate. GCC and EDG each conflict with the reference in one place. |
| Q-0374 | Undefined (option 1). | agrees | option 1 | The member's lifetime has not begun, because its initialization, even a vacuous one, has not happened yet. The reference lists a call of a non-static member function through a glvalue to such an object as undefined, and the object is not under construction. Undefined behavior ends constant evaluation, as Clang finds. |
| Q-0375 | The namespace that declares the member (option 1). | agrees in result | option 1 for the outcome | See "NS-15" below. The definition inside the named namespace is valid in the reference. The reference's test also makes the definition inside the inline namespace valid, which every compiler rejects. |
| Q-0376 | The standard size type (option 1). | agrees | option 1 | The reference defines the index as a prvalue of the standard size type. |

### UNW-1

- The reference destroys, during unwinding, every automatic object
  built since the try block was entered, in reverse order of the
  completion of construction. When the exception comes from destroying
  the temporaries or local variables of a return statement, the
  reference destroys the result of the call too.
- The result is built before the locals start to be destroyed, and
  after the locals of the function were built. So in reverse order the
  result goes first among the objects that remain. The reference's own
  example shows this order. After the throw, the result is destroyed,
  and then the local objects declared before the throwing one.
- UNW-1 places the destruction of the result after the objects of the
  blocks that the exception leaves. That order conflicts with the
  reference whenever a local object was built before the throwing one.
- The rest of UNW-1 agrees. A later return statement in a handler of
  the same function builds a new result, as the reference's example
  shows.

### NS-15

- The reference requires a declaration with a qualified declarator to
  inhabit a namespace scope that contains the entity it redeclares, and
  to match a declaration that is nominable in the namespace that the
  qualifier names. Nominable declarations include those of the inline
  namespace set.
- A namespace contains its own members in this sense. So the
  reference allows a member of a namespace to be defined with a
  qualified name inside that same namespace. A core defect resolution
  long before C++23 made this kind of redundant qualification valid.
  Clang's warning reflects that history. GCC and EDG conflict with the
  reference. Q-0371 should take option 2.
- For Q-0375, the global namespace, the named namespace, and the inline
  namespace all contain the member. As I read the reference, the
  definition through the named namespace is valid in all three places.
  The inside-the-named-namespace row matches option 1's result. The
  inside-the-inline-namespace row is valid in the reference, and every
  compiler rejects it. The compilers seem to keep an older rule, under
  which the definition had to be in a namespace that encloses the one
  the qualifier names. I hold this reading of the third row with some
  doubt, because the reference speaks of containing an entity without
  defining the phrase.
- The second bullet of NS-15 therefore adds a condition that the
  reference does not have.

### C names in an unnamed namespace

- In the reference, an unnamed namespace has internal linkage, and so
  do the names of variables and functions declared in it, at any
  depth. A linkage specification gives its language linkage only to
  names with external linkage.
- So a function or variable declared in a C linkage block inside an
  unnamed namespace has internal linkage and no language linkage. It is
  not one entity with C names of the same spelling elsewhere, and
  another translation unit cannot reach it.
- LSP-4's second bullet and the exception in LNK-5 follow Clang and
  GCC, which conflict with the reference here. Q-0369's last sentence
  repeats the same claim. I know of no later resolution that gives such
  names external linkage, though implementations have long differed on
  this point.

### SB-5

- The reference uses the member form of the get call only when a
  search for the name in the class finds at least one template whose
  leading parameter takes a value rather than a type. Otherwise it
  uses the free form with argument-dependent lookup only.
- SB-5 asks only for a member function template. A class with a member
  get template whose first parameter is a type falls to the free form
  in the reference, and to the member form under SB-5. The condition
  came in during the C++20 cycle.
- The index is a prvalue of the standard size type in the reference,
  as SB-5 and Q-0376 say.

### COND-16

- The reference tries to convert each operand to a type tied to the
  other operand when the types differ and either is a class. COND-16
  starts from an operand of class type. The reference also tries an
  operand of non-class type against a class operand, for example
  through a converting constructor of the class. If the class also
  converts to the non-class type, both attempts succeed and the
  expression is ill-formed. COND-3 and COND-5 do not cover either case.
- In the third attempt, the reference's target type depends on how the
  two classes relate. For one class type, ignoring cv-qualifiers, the
  conversion goes only toward the operand that is at least as
  cv-qualified. For a base class, the target is the base with the
  derived operand's cv-qualifiers. Otherwise the target is the other
  operand's type after the lvalue, array, and function conversions.
  COND-16 forms a plain implicit conversion to the other type, which
  could succeed in both directions for one class type with different
  cv-qualifiers and so make COND-5 reject a valid expression.

### NEW-22

- After the destroying-form and alignment steps, the reference picks
  the one remaining function if there is one, and the form without the
  size in a class. For the global functions, it picks the sized form
  when the type is complete, except that for an array the sized form is
  required only for a class with a non-trivial destructor, or an array
  of one. Only the remaining cases are unspecified.
- The type is always complete in a new expression whose initialization
  throws. So for a single object, the reference requires the sized
  global form. UNW-3 says this. NEW-22's third bullet says either form
  can be called, which is looser than both the reference and UNW-3.

### DEFARG-2

- The reference lets later declarations add default arguments only for
  functions that are not templates. When the member belongs to a class
  template, its default arguments must be on its first declaration inside
  that template, and a definition outside the class cannot add one.
  DEFARG-2 states no such limit, and I found no other rule that does.
- The reference adds the default arguments of an outside-class member
  definition to the class's set. For a namespace member defined with a
  qualified name outside its namespace, the reference speaks only of
  declarations that inhabit the same scope, and such a definition
  inhabits the outer scope. The reference's rule on which defaults a
  call can use then makes the added default usable. The reference does
  not say plainly whether a default given in both places is a repeat
  under DEFARG-9. DEFARG-2's reading is plausible but not stated.

### ALN-1

- The reference allows the alignment specifier on a variable, a data
  member, and the declaration of a class or union, including a
  forward declaration. It forbids the specifier on a bit-field, a
  function parameter, and the variable that a handler declares.
- ALN-1 says any variable declaration, and ALN-2 does not list the
  handler's variable. The handler case is missing.
- ALN-1 makes a class's alignment the strictest of its own specifier,
  bases, and members. The reference leaves alignment values to the
  implementation. Each subobject must be suitably aligned in every
  object, so the class's alignment must be at least that strict, and
  the matrix uses exactly that value. The size rule follows from array
  layout in the reference.

### UNI-7 and braces

- In aggregate initialization, the reference treats an anonymous union
  member as one element. A designated initializer can name a member of
  the anonymous union directly, and the reference then initializes the
  union as if with braces around that one designator.
- With a plain initializer list, brace elision applies, so the braces
  around the union's initializer can be left out.
- UNI-7 says a braced list initializes the anonymous union member with
  its own braces. Read as a requirement, that conflicts with brace
  elision and with the designated form. Read as a description of one
  valid form, it agrees.

### STG-18 exceptions

- The second case names two exceptions, containment and a new creation
  between the uses. The reference also lets the active member of a
  union change through assignment to a member, and an implicitly
  created union can make a later use as another member type valid.
  Other operations that start a lifetime, such as library functions
  that create objects implicitly, count under STG-8 already.
- This gap is small, but STG-18 states the list as complete.

### BF-9 and unnamed bit-fields

- The reference says unnamed bit-fields are not members. Their bits
  take part in no member's value, so they are padding bits.
- BF-9 calls a bit padding only when no bit-field or other member
  covers it. An unnamed bit-field is a bit-field, so the text could be
  read to make its bits non-padding. The rule should leave no doubt for
  Q-0372.

### Q-0372

- The reference defines padding bits as the bits of the object
  representation outside the value representation, and the extra bits
  of a wide bit-field are padding bits. It says nothing direct about
  the value that a read through the unsigned character type gives for
  a byte with both kinds of bits.
- Three places lean toward option 2. The bit-cast facility makes a bit
  of the result indeterminate when it maps to a bit outside the
  source's value representation, and then the smallest object holding
  that bit has an indeterminate value, which is defined only for the
  unsigned character and byte types. A non-normative note on atomic
  compare-and-exchange says padding bits outside the active member's
  value representation have an indeterminate value. And the reference
  gives an uninitialized automatic object an indeterminate value as a
  whole.
- Zero-initialization sets padding bits to zero in the reference, for
  a class and for a union. So after static initialization, or value
  initialization of a class whose default constructor is not
  user-provided, the bits read as zero. The question does not mention
  this case. The probes used aggregate initialization, which does not
  zero padding.
- Option 1 allows a byte copy through plain char. Under option 2 a byte
  with an indeterminate value can be copied only through the unsigned
  character or byte type. The reference does not settle the choice, and
  committee work on access to object representations continues.
