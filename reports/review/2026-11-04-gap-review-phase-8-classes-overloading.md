# Gap review: the text phase 8 wrote on the classes and overloading chapters

Date: 2026-11-04
Role: source reader

Reference: the C++23 working draft in the local reference tree. The
review covers every rule and principle on the pages under
`specification/classes/` and `specification/overloading/` whose prose
changed between commit dc90663 and commit 1b16115, with `:tests:` lines
left out. Six rules changed only by glossary markup, and the review
records them as unchanged in meaning. Part B applies each principle that
item PQ widened to the program of each question that waits on that
principle. Part C reads Q-0381 to Q-0384. Q-0383 is included because
its Affects line names OPR-8 and OPR-9. The other questions from Q-0377
to Q-0386 name no rule on these pages. Later defect resolutions are
mentioned only where they may change the answer.

## Result

The principles hold up. In each of the 14 uses in Part B, the model of
the widened principle gives the provisional answer, and the reference
agrees with that answer. Five of the principles leave out a detail of
the reference, and the detail sections say which.

Of the four questions in Part C, Q-0382 agrees with the reference.
Q-0381 and Q-0384 are about choices that the reference leaves open, and
the reference is silent on them. Q-0383 contradicts the reference. The
reference makes the declaration of a literal operator with a reserved
suffix ill-formed, no diagnostic required. That supports option 2 in
the no-diagnostic form, and it makes OPR-9 wrongly classified.

The gaps in Part A are these:

- OPR-9 is a recommended rule. The reference makes the declaration
  itself ill-formed with no diagnostic needed.
- OVL-38 lets two equal default arguments from two scopes pass. The
  reference rejects the call whenever two declarations in different
  scopes both give the default argument that the call uses, equal or
  not. This is the known split of Q-0140. Phase 8 made it wider by
  defining when two default arguments are the same.
- VIRT-1 asks for the same ref-qualifier. The C++23 text decides
  overriding by its correspondence test, under which a member function
  with no ref-qualifier matches one with a ref-qualifier.
- ACC-20 does not cover a friend template declaration that names a
  member of a specialization of a class template.
- UNI-7 does not say that only one designator can name a member of one
  anonymous union.
- CTOR-17 still leaves one doubt from the phase 7 review, the
  aggregate that uses the default member initializer.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| ACC-P3 | agrees | The reference lets a class befriend a template, so that every specialization is a friend, or one specialization only. |
| ACC-20 | agrees, one missing case | See "ACC-20" below. |
| CTOR-12 | agrees | The reference makes it undefined to call a member function, or to use the object as the operand of typeid or of a dynamic cast, from the initializer list of a constructor or from any function that it calls, directly or not, before the initializers of all bases have finished. |
| CTOR-17 | agrees, one doubt | See "CTOR-17" below. |
| CTOR-21 | agrees | Glossary markup only. |
| CPY-P4 | agrees | In C++23 a returnable local name is an xvalue whatever the return type is, so a converting constructor of another class that takes an rvalue reference is chosen. |
| CPY-27 | agrees | The reference's conditions for a trivial copy or move function name no condition on volatile members. An older condition on volatile members was removed before C++23. |
| DER-1 | agrees | A cross-reference was added. |
| ILC-4 | agrees | Glossary markup only. |
| CLS-11, CLS-31, CLS-33 | agree | Glossary markup only. |
| CLS-35 | agrees within what the reference fixes | The reference says nothing on the matrix's choices. The new rows describe the platform ABI. The union row fits the reference: every union member starts at the union's address, and a complete object has nonzero size. The sizes of pointers to members and of reference members are left to the implementation. |
| OPR-8 | agrees | The reference treats a token that can be read both as a literal with a suffix and as a built-in literal as the built-in literal. A program that declares a literal operator for a built-in suffix also breaks the reservation (see OPR-9). |
| OPR-9 | wrong classification | See "OPR-9 and Q-0383" below. |
| UNI-2 | agrees | The class head grammar of the reference accepts the final marker with every class key, unions included. |
| UNI-7 | agrees, one missing condition | See "UNI-7" below. |
| BF-7 | agrees | The reference counts a bit-field source by its width when the target of a narrowing check is an integer type. The width exception does not apply when the target is a floating-point type. CONV-21 should keep that limit. |
| BF-9 | agrees | The reference makes allocation and alignment of bit-fields implementation-defined, and says that an unnamed bit-field is not a member. The new items describe the platform. |
| BF-11 | agrees | Glossary markup only. |
| VIRT-1 | missing case | See "VIRT-1" below. |
| VIRT-3 | agrees | The reference allows each of the two specifiers at most once, and only on the first declaration of the virtual function, which is the one in the class. |
| VIRT-7 | agrees | Two destructors always correspond in the reference, so a destructor of a derived class, declared or implicit, overrides a virtual destructor of a base and is virtual. |
| VIRT-13 | agrees, no basis | The reference has no recommended practice for any of the three warnings. The new item is Tidal's own. A warning does not change whether a program conforms. |
| OVL-38 | extra condition | See "OVL-38" below. |

## Principle uses

| Principle | Question | Test file | The model's answer | Matches the provisional answer? | Reference verdict |
|---|---|---|---|---|---|
| CLS-P3 | Q-0027 | `ref-qualified-mixed.cpp` | The two overloads have the same parameters, and only one has a ref-qualifier. The declarations are rejected. | yes | agrees, see "CLS-P3" below |
| CLS-P3 | Q-0317 | `explicit-object-default-argument.cpp` | The object of each call initializes the explicit object parameter, so the parameter cannot have a default argument. The declaration is rejected. | yes | agrees. The grammar of a parameter has no form that carries both the explicit object marker and a default argument. |
| CVF-P1 | Q-0059 | `deduced-auto.cpp` | The name of the conversion function can be the deduced placeholder, and the return statement gives the target type. The program runs. | yes | agrees, see "CVF-P1" below |
| OPR-P3 | Q-0060 | `literal-default-arg.cpp` | The literal supplies every argument, so the literal operator has no default argument. The declaration is rejected. | yes | agrees. The reference makes a default argument on any parameter of a literal operator ill-formed. |
| DCMP-P1 | Q-0061 | `declared-outside-then-defaulted.cpp` | The friend was declared outside the class first, so the class did not ask for it. Defaulting it is rejected. | yes | agrees. A defaulted comparison defined in a class must be the first declaration of that function. |
| DCMP-P2 | Q-0158 | `eq-reference-to-bool-result.cpp` | The reversed equality returns a reference to bool, which is not the type bool. The comparison is rejected. | yes | agrees. The chosen rewritten equality must return bool, with or without cv-qualifiers. |
| VIRT-P4 | Q-0086 | `call-returning-abstract.cpp` | A call makes its result as an object of the abstract class. The call is rejected, and the declaration stays valid. | yes | agrees, see "VIRT-P4" below |
| UNI-P1 | Q-0088 | `user-assignment-not-activating-constant.cpp` | The assignment uses a user-provided operator, so it is a call on a member that is not an object yet. The member does not become active, the behavior is undefined, and constant evaluation rejects it. | yes | agrees. Only the built-in or a trivial assignment can start the lifetime of the member, and a member call on an object outside its lifetime is undefined. |
| UNI-P4 | Q-0058 | `bitfield-unnamed-cv.cpp` | An unnamed bit-field takes no cv-qualifier. The declaration is rejected. | yes | agrees. The reference forbids a cv-qualified type on an unnamed bit-field. |
| UNI-P4 | Q-0313 | `bitfield-width-scoped-enumeration.cpp` | A scoped enumeration converts to an integer only with a cast, so its enumerator is rejected as a width. | yes | agrees. The width must be an integral constant expression, which allows only integral and unscoped enumeration types. |
| UNI-P4 | Q-0314 | `bitfield-width-class-converts.cpp` | The width has a class type whose conversion function gives int. The width is 3, and the program is accepted. | yes | agrees, see "UNI-P4" below |
| UNI-P6 | Q-0340 | `layout-compatible.json`, row "overlap mark on both" | Each pair of members has the same type, and both last members carry the attribute. The sequence holds every member, so the structs are layout-compatible. | yes | agrees, see "UNI-P6" below |
| OVL-P2 | Q-0145 | `init-copy-list-explicit-conversion.cpp` | Copy-list-initialization is a form of copy initialization, so the explicit conversion function cannot make the temporary for the copy constructor's parameter. No constructor is viable, and the program is rejected. | yes | agrees. The reference allows an explicit conversion function there only in a direct initialization of the class. |
| OVL-P4 | Q-0147 | `builtin-removed-by-rewritten.cpp` | The non-member three-way operator takes the parameter types of the built-in less-than candidate for the enumeration and takes part as a rewritten candidate, so it replaces the built-in candidate. The rewritten candidate is chosen. | yes | agrees. In the reference, a built-in candidate leaves the set when a plain non-member function with the same parameter types is a candidate, either directly or through a rewrite. Template specializations do not count. |

Q-0190 and Q-0374 also carry notes from 2026-11-04. They wait on the
maintainer, or on newest families that differ, and on no widened
principle on these pages, so Part B does not use them. Q-0055 is
settled.

## Questions

| Question | Provisional answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0381 | The attribute does not change the end padding (option 1). | silent | neither | The reference leaves the use of end padding to the implementation. The attribute only allows a member to share storage. Both options conform, and the question records a property of the platform ABIs. |
| Q-0382 | The word after the class name is an identifier (option 1). | agrees | option 1 | The class head accepts only the final marker after the class name. The reference reads a word with special meaning as a plain identifier whenever the special reading does not fit. So the declaration names a variable of the complete class, with a braced initializer. GCC 14 is wrong. |
| Q-0383 | The literal calls the literal operator (option 1). | contradicts | option 2, with no diagnostic required | See "OPR-9 and Q-0383" below. |
| Q-0384 | The conditions of a class apply to a union (option 1). | silent | neither | The reference leaves end padding to the implementation for unions too. Both options conform. |

### ACC-20

- The reference has a further form of template friend. A friend
  declaration with a template head can name a member function or a
  member class of a specialization of a class template, written with a
  qualifier that ends in a template name with arguments. The template
  parameters must be deducible from that qualifier. Every matching
  member of every specialization, explicit specializations included,
  then becomes a friend.
- ACC-20 says that a friend declaration with a template head names a
  class template or a function template. The member form falls outside
  that list. The template pages of Tidal do not state it either, as far
  as a search of them shows.
- The other limits of the reference fit what Tidal says elsewhere: no
  friend template in a local class, no partial specialization as a
  friend, and no definition of a friend class template inside a class.

### CTOR-17

- The reference makes the binding ill-formed from a member initializer
  in a constructor, and from a default member initializer. Its own
  example shows that the second case applies only where a constructor
  that uses the initializer is used. CTOR-17 now says the same.
- The reference handles a member of the standard initializer-list type
  set from a braced list in a member initializer as a dangling binding,
  in an example of its own. CTOR-17's added paragraph agrees. The
  default member initializer form follows from the same lifetime rule.
- The reference states the default member initializer case with no
  limit to constructors. An aggregate initialization that falls back on
  the default member initializer of a reference member is not named.
  CTOR-17 says only that an aggregate initialization that gives the
  member a value is valid. The case where the aggregate initialization
  uses the default stays open, as in the phase 7 review.

### OPR-9 and Q-0383

- The reference reserves every literal suffix that does not start with
  an underscore for later versions of the language. It also reserves
  suffixes that contain a double underscore for implementations.
- A declaration of a literal operator that uses a reserved suffix is
  ill-formed, and no diagnostic is required. The program is not valid
  from the declaration on, whether or not a literal uses the operator.
- So OPR-9's class, recommended, is wrong. The declaration belongs to
  the no-diagnostic class, and a warning is a fair diagnostic for it.
  OPR-P3's sentence that an implementation should warn has the same
  problem. OPR-9 also leaves out the double-underscore reservation.
- For Q-0383, every result in the table conforms: calling the
  operator, rejecting the literal, and rejecting the declaration.
  Option 1 promises a call that the reference does not give. Option 2
  fits the reference once "not valid" is read as no diagnostic
  required.
- The reference reads the digits before a suffix as the longest
  sequence that fits. For the u8 test, as far as I can tell, the
  reference reads one suffix made of both characters, which is GCC's
  reading. EDG's reading as a built-in suffix with extra text after it
  looks wrong. The point does not change the verdict, because the
  declaration is already outside the language.
- The reference also deprecates the form of literal operator name with
  a space between the quotes and the suffix. In that form the suffix
  is an ordinary identifier, and the usual reservation of identifiers
  applies too.

### OVL-38

- The reference makes the program ill-formed when the chosen function
  was found through declarations in two different scopes, and two of
  them each give a default argument that the call needs in order to be
  viable. The reference's own example uses equal default arguments in
  the two scopes and calls the call that uses them an error.
- OVL-38 asks for different default arguments, and phase 8 added a
  test of sameness by expression. The reference has no such test.
  Under the reference, the programs `default-from-two-scopes-same.cpp`
  and `default-from-two-scopes-same-literal.cpp` are ill-formed, and
  their tests expect acceptance.
- This is the open choice of Q-0140, which waits on the maintainer.
  The new sameness test widens the distance between OVL-38 and the
  reference. It also adds a notion that the reference has nowhere
  else.

### UNI-7

- The reference treats the anonymous union as one element of the
  aggregate. Brace elision then lets the union's initializer appear
  without its own braces. UNI-7 agrees.
- A designator in the reference must name a direct non-static data
  member. The element that is initialized is the one that is or
  contains that member, so a designator can name a member of an
  anonymous union, and the union is initialized with that member
  active. UNI-7 agrees.
- The reference allows only one designator for the members of one
  anonymous union in one list. UNI-7 does not state the limit, so a
  list with two designators that name two members of one anonymous
  union has no rule that rejects it.

### VIRT-1

- In the C++23 text, a member function overrides a virtual function of
  a base when its declaration corresponds to the base's declaration,
  trailing constraints aside. Correspondence compares the object
  parameters. If one of the two functions has neither an explicit object parameter
  nor a ref-qualifier and the other has a ref-qualifier, the test drops
  the references and compares the object types.
- So a derived member with no ref-qualifier corresponds to a virtual
  base member with the same cv-qualifiers and a ref-qualifier. Read
  literally, the derived member overrides. VIRT-1 asks for the same
  ref-qualifier, so it says the derived member does not override.
- The compilers split. A local probe with GCC reports that a derived
  member marked as an override overrides nothing. Clang reports that a
  function without a ref-qualifier cannot overload one with a
  ref-qualifier.
- I believe a later core issue on overriding and explicit object
  parameters puts back a requirement of the same ref-qualifier. If it
  does, VIRT-1 matches the later text. I could not confirm the
  resolution or its date.

### CLS-P3

- In the reference, two member function declarations with one name
  correspond when their non-object parameter lists match, their
  trailing constraints are equivalent, and their object parameters
  correspond. With one function lacking a ref-qualifier, the object
  types must still match after references are removed, so the
  cv-qualifiers must be equal.
- Q-0027's program has no cv-qualifiers, so the model and the
  reference give the same answer. CLS-P3 says "the same parameters"
  without saying that the cv-qualifiers of the object count. A
  function with a const qualifier and a ref-qualifier can be
  overloaded with one that has neither, under the reference.

### CVF-P1

- The reference allows a deduced return type for a plain conversion
  function and forbids one for a conversion function template. CVF-P1
  says the deduction works "as for any function with a deduced return
  type", and neither CVF-P1 nor CVF-1 nor the deduction page states the
  limit on templates. The answer to Q-0059 is not affected.

### VIRT-P4

- The reference forbids a prvalue of abstract class type in every
  place but one unevaluated operand. A call that returns the class
  makes such a prvalue, so the call is ill-formed. The model agrees.
- The reference also names the abstract class as a parameter type of
  a function being called. VIRT-P4's table lists a parameter only in a
  function definition. A call to a declared function with a parameter
  of abstract class type makes an object as its argument, and neither
  VIRT-P4's table nor VIRT-8 names that row.

### UNI-P4

- The reference converts an expression of literal class type
  contextually to an integral or unscoped enumeration type where an
  integral constant expression is needed. The chosen conversion
  function must be constexpr, and the choice must be unique.
- UNI-P4 says that a class type works when its conversion function
  gives such a type. It leaves out that the class must be a literal
  type, that the conversion function must be constexpr, and that two
  candidate conversion functions make the width ambiguous. The test of
  Q-0314 meets all three conditions, so the answer stands.

### UNI-P6

- The reference builds the common initial sequence from non-static
  data members and bit-fields, unnamed bit-fields included. Its own
  example shows an unnamed bit-field of width zero ending the sequence.
- UNI-P6 pairs "the members" of the two structs, and UNI-P4 and BF-9
  say that an unnamed bit-field is not a member. A reader of UNI-P6
  alone could skip the unnamed bit-field. UNI-15 says "members and
  bit-fields" and agrees with the reference.
- For Q-0340 the attribute condition is the one that matters, and the
  reference supports option 1.
