# Gap review: open questions on classes and statements

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree (the
final C++23 draft). The review compares each provisional answer with
that draft only. The cluster holds the 35 open or provisional questions
whose first Affects rule is on a page under the classes or statements
chapter and that no gap review has read. Later changes to the language
are mentioned only where they matter, and they are not in the
reference.

## Result

Ten of the 35 answers conflict with the reference in some form.

- Q-0062 contradicts the reference. A comparison that is defaulted
  after its first declaration and would be deleted makes the program
  invalid with a diagnostic required. The draft supports option 2.
- Q-0082 contradicts the reference. Creating an array of class objects
  with a new expression makes the destructor needed, so a deleted
  destructor makes the program invalid. The draft supports option 2.
- Q-0083 contradicts the reference, with doubt. The draft has no rule
  that makes the program invalid. The draft supports option 2.
- Q-0084 contradicts the reference. Both forms are invalid with a
  diagnostic required. The draft supports option 2.
- Q-0087 contradicts the reference text, and a later change agrees with
  the answer. The draft supports option 2. Clang and GCC follow the
  later change.
- Q-0126 contradicts the reference for the fifth row. The draft
  supports option 2.
- Q-0131 contradicts the reference. The draft supports option 2.
- Q-0181 contradicts the reference text, as the cluster 1 review found.
  A later change gives option 1, and every compiler in the table except
  Clang 18 follows it.
- Q-0190 contradicts the reference. The draft supports option 2 and goes
  further than any compiler for cv-qualifiers.
- Q-0344 contradicts the reference in part, with doubt. For a copy made
  by the library's byte copy functions, the draft supports option 2.

Two questions fall outside what the reference decides.

- Q-0185 is a question about Tidal's notation.
- Q-0361 concerns a layout choice that the draft leaves to the
  implementation.

The other 23 answers agree with the reference. Q-0085 carries a side
note on a later change, and Q-0345 carries a small doubt.

## Questions in the cluster

| Question | Answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0027 | The declarations are rejected (option 1). | agrees | option 1 | In the draft, two member functions with the same parameters, one with a ref-qualifier and one without, have matching object parameters when the object types agree once references are removed. The two declarations then clash. GCC 16's acceptance is a defect against the draft. I know of no later change that allows the pair. |
| Q-0052 | The using-declaration sets the access (option 1). | agrees | option 1 | The draft checks access on the using-declaration that the name lookup finds, with the access of the place where the class declares it. A private using-declaration in the derived class makes the use from outside invalid. |
| Q-0053 | Invalid, no diagnostic required (option 1). | agrees | option 1 | The draft makes a constructor that delegates to itself, directly or through others, invalid with no diagnostic required. |
| Q-0054 | Undefined, so constant evaluation rejects it (option 1). | agrees | option 1 | The draft makes a second destructor call on an object whose lifetime has ended undefined, and it names the case of an explicit call followed by the implicit one at block exit. Undefined behavior of the core language makes an expression not constant. |
| Q-0055 | The function is deleted (option 1). | agrees | option 1 | The draft lists the ways a defaulted special member may differ from the implicit one. A parameter type outside that list makes a function that is defaulted on its first declaration deleted. A copy constructor with a reference to const volatile class and a move constructor with a const rvalue reference are still copy and move constructors. |
| Q-0056 | Undefined, so constant evaluation rejects it (option 1). | agrees | option 1 | The draft makes a single-object delete through a base without a virtual destructor undefined when the static and dynamic types differ. |
| Q-0058 | A cv-qualifier is not allowed (option 1). | agrees | option 1 | The draft forbids a cv-qualified type on an unnamed bit-field. GCC's acceptance is a defect. |
| Q-0059 | A deduced return type is allowed (option 1). | agrees | option 1 | The draft forbids a deduced return type only on a conversion function template. A plain conversion function may deduce its type. GCC 16's pedantic error is a defect. |
| Q-0060 | No default argument (option 1). | agrees | option 1 | The draft states directly that a default argument on any parameter of a literal operator makes the program invalid. |
| Q-0061 | The first declaration must be in the class (option 1). | agrees | option 1 | In the draft, a comparison may be defaulted inside a class only where the class declares it for the first time. A friend that defaults a function already declared at namespace scope breaks the rule. |
| Q-0062 | Invalid, no diagnostic required (option 1). | contradicts | option 2 | See below. |
| Q-0081 | The initializer-list constructor is used (option 1). | agrees | option 1 | The draft's special case for a braced list with one element of the same class applies only to aggregates. The class in the question is not an aggregate, so the initializer-list constructor is tried first. |
| Q-0082 | The array form is valid (option 1). | contradicts | option 2 | See below. |
| Q-0083 | The program is not valid (option 1). | contradicts, with doubt | option 2 | See below. |
| Q-0084 | Invalid, no diagnostic required (option 1). | contradicts | option 2 | See below. |
| Q-0085 | Not valid (option 1). | agrees | option 1 | The draft requires a non-template constexpr constructor or destructor to belong to a class with no virtual base. As a side note, I understand that a change for the next standard lifts this limit. Clang and GCC in the table still reject, so they do not follow that change. |
| Q-0086 | The call is not valid (option 1). | agrees | option 1 | The draft lets a function be declared with an abstract return type. A call yields a prvalue of that type, and the draft forbids a prvalue of abstract class type except in the operand of decltype. Clang's acceptance is a defect against the draft. |
| Q-0087 | The declaration is not valid (option 1). | contradicts the text; a later change agrees | option 2 in the draft text | See below. |
| Q-0088 | The member does not become active (option 1). | agrees | option 1 | The draft changes the active member only for an assignment that uses the built-in operator or a trivial assignment operator. A user-provided operator is a member call on an object whose lifetime has not begun, which is undefined. |
| Q-0096 | Any label can end a block (option 1). | agrees | option 1 | The grammar of a block allows a sequence of labels after the last statement, and labels include case and default labels. Such a label behaves as if a null statement followed it. GCC's rejection is a defect. |
| Q-0098 | Any member named begin or end counts (option 1). | agrees | option 1 | The draft takes the member form when a search in the class finds at least one declaration for each name. The kind of member does not matter. |
| Q-0121 | Constant evaluation can write the variable (option 1). | agrees | option 1 | In the draft, a jump into the scope of a variable with vacuous initialization initializes that variable as part of the transfer. The variable's lifetime then begins during the evaluation, so a write is allowed. |
| Q-0122 | An empty init statement is allowed (option 1). | agrees | option 1 | The draft uses one init-statement production for the range-based loop and for the selection statements. That production includes an expression statement with no expression. GCC's rejection is a defect. |
| Q-0126 | Follow the majority; the fifth row is valid (option 1). | contradicts for the fifth row | option 2 | See below. |
| Q-0131 | The declaration is rejected (option 1). | contradicts | option 2 | See below. |
| Q-0158 | The result must be cv bool (option 1). | agrees | option 1 | The draft requires a selected rewritten or reversed equality candidate to have a return type of possibly cv-qualified bool. A reference to bool is not that type. GCC's acceptance is a defect. |
| Q-0181 | The prvalue initializes the parameter directly (option 1). | contradicts the text; a later change agrees | option 2 in the draft text | See below. |
| Q-0185 | A change table now, version field later (option 1). | outside the reference | none | The question is about Tidal's own notation. The draft has nothing to say about it. |
| Q-0190 | Qualifiers of members take no part (option 1). | contradicts | option 2, and further | See below. |
| Q-0317 | The default argument is not valid (option 1). | agrees | option 1 | The draft's grammar for a parameter has no form that carries both the explicit object marker and a default argument. EDG's acceptance is a defect. |
| Q-0329 | The initializer runs at run time (option 1). | agrees | option 1 | A local reference bound to a temporary refers to an object of automatic storage, which is not a valid result of a constant expression. The reference is then neither usable in constant expressions nor constant-initialized, so its initializer is not manifestly constant-evaluated. GCC's result is a defect. |
| Q-0344 | The byte copy is undefined (option 1). | contradicts in part, with doubt | option 2 for the library copy | See below. |
| Q-0345 | Members from a base count (option 1). | agrees, with doubt | option 1 | The draft states, outside any note, that a derived class has its bases' members among its own. Read with that statement, the common initial sequence takes the base's members. The doubt is that the sequence is defined in declaration order, and the draft does not say how that order runs across a base and a derived class. With all members in one base, the question does not arise. |
| Q-0355 | The call reaches the final overrider for the complete object (option 1). | agrees | option 1 | The draft resolves a virtual call by the final overrider in the most derived object. Nothing in the draft makes a call from inside an intermediate class's function differ, outside construction and destruction. EDG's result is a defect. |
| Q-0361 | The base keeps its padding (option 1). | outside the reference | none | The draft does not fix whether a derived class reuses the end padding of a base. The choice belongs to the implementation and its ABI, so every column in the table conforms. |

### Q-0062 in more detail

- In the draft, a function that a class declares and that is defaulted
  later is user-provided. Such a function is defined at the place where
  it is defaulted.
- If that definition turns out to be deleted, the program is invalid.
  The draft gives this rule with no exemption from diagnosis, so a
  diagnostic is required.
- The rule covers comparison operators as well as special members. A
  member without equality makes the defaulted equality deleted, so the
  definition after the class must be rejected even when nothing uses
  the function.
- Clang's result matches the draft. GCC and EDG accept, which is a
  defect against the draft. This matches the treatment of copy and move
  constructors that the question cites from CPY-17.

### Q-0082 in more detail

- The draft says that a new expression that creates an array of class
  objects makes the destructor potentially invoked. The reason is that
  a failed construction part way through must destroy the elements
  already built.
- The draft makes a program invalid when a potentially invoked
  destructor is deleted or not accessible. A diagnostic is required.
- The single-object form has no such rule, so it stays valid.
- Clang's result matches the draft. GCC and EDG accept, which is a
  defect against the draft.

### Q-0083 in more detail

- In the draft, a default argument and a default member initializer
  are both contexts in which the class is regarded as complete. A
  context of that kind inside a nested class is also one for the
  enclosing class.
- So the default argument of the enclosing constructor and the nested
  class's default member initializer are both handled as if the
  enclosing class were complete. I find no rule that orders them or
  makes the use of the nested class's initializer invalid at that
  point.
- The draft does forbid a default member initializer that leads,
  directly or indirectly, to the implicit definition of the default
  constructor of its own class, and an aggregate initialization inside
  a default member initializer that would use that same initializer.
  Neither rule fits the test, since the use is in a default argument
  of the enclosing class.
- So the draft reads the program as valid, which is EDG's result. The
  doubt is that the compilers that reject do so because the order of
  these delayed contexts is a known unsettled area. The draft does not
  settle it in favour of rejection, but it does not describe the order
  in detail either.

### Q-0084 in more detail

- The draft states two rules. A constructor's member initializer that
  binds a reference member to a temporary makes the program invalid. A
  default member initializer that binds a reference member to a
  temporary also makes it invalid.
- Neither rule is exempt from diagnosis, so an implementation must
  diagnose both forms in the question.
- Clang's result matches the draft. GCC's warning and EDG's acceptance
  are defects against the draft.
- Option 3 does not fit. The draft does not leave the binding valid and
  move the problem to a later use.

### Q-0087 in more detail

- In the draft, a function in a derived class overrides a base virtual
  function when the two declarations would correspond. For member
  functions, correspondence compares the object parameters. In the
  test, the base function's object parameter refers to the base class
  and the derived function's explicit object parameter refers to the
  derived class. The types differ, so the declarations do not
  correspond.
- The derived function then does not override. The draft forbids
  declaring a function with an explicit object parameter virtual, but
  that rule is not reached, because the function is not virtual. The
  declaration is valid and hides the base function. This is EDG's
  result and option 2.
- I understand that a later defect resolution changed the test for
  overriding so that the object parameter is ignored when the derived
  function has an explicit object parameter. Under that change, the
  function would override and so be virtual, which is not allowed.
  That gives option 1.
- Clang and GCC follow the later change. Their diagnostics speak of an
  explicit object function that overrides or appears in a virtual
  function. EDG follows the draft.

### Q-0126 in more detail

- The draft lets a const object of class type go without an
  initializer only if the class meets a set of conditions. For a union
  with at least one data member, exactly one variant member must have
  a default member initializer. For a class that is not a union, each
  anonymous union member that has at least one data member must have
  exactly one member with a default member initializer.
- Rows one to four agree with the provisional answer. A union with no
  initialized variant member is rejected. A union with one is accepted.
  A class with an anonymous union that has one is accepted. A class
  with an anonymous union that has none is rejected.
- In the fifth row, every data member is in an anonymous union and no
  union has a default member initializer. The anonymous union condition
  fails, so the object is not allowed. The draft has no exception for a
  class with no other data members.
- The draft therefore rejects the fifth row, which is the Clang 22
  model. GCC and EDG accept it, which is a defect against the draft.
  The draft supports option 2, and the rule needs no extra sentence.
- A side point is that the draft says exactly one, not at least one.
  A union with two initialized members is already invalid on other
  grounds, so this does not change any row.

### Q-0131 in more detail

- In the draft, a declaration with a qualified name does not look up
  its last name. The declaration must instead match one or more earlier
  declarations that are nominable in the named namespace.
- The draft defines a nominable declaration by its target scope. A
  friend function first declared in a class has the enclosing namespace
  as its target scope, and it does not inhabit a block scope. A note
  with an example in the draft lists such a friend as nominable in its
  namespace.
- So the qualified declaration at global scope matches the friend and
  is valid. The global namespace encloses the target namespace, which
  the draft also requires. The draft no longer has the older rule that
  a qualified declaration outside the namespace must be a definition.
- The rule of ACC-7, that qualified lookup does not find the friend,
  agrees with the draft for lookup. It does not decide this case,
  because the declarator is matched without lookup.
- Clang's result matches the draft. GCC and EDG apply an older rule.
  The draft supports option 2.

### Q-0181 in more detail

- The cluster 1 review covered this question. In the draft, the object
  expression of a member call written with a dot must be a glvalue, so
  a class prvalue there is materialized as a temporary. The explicit
  object parameter is then initialized from that temporary, which needs
  the copy or move constructor. With both deleted, the call is invalid.
  This is option 2.
- A later defect resolution lets the prvalue initialize a by-value
  explicit object parameter directly. That gives option 1.
- Every compiler in the table except Clang 18 follows the later change.
  The choice between the draft and the change belongs to Q-0007.

### Q-0190 in more detail

- In the draft, an equality operator is not used reversed or rewritten
  when a search finds an inequality operator that would correspond to
  it under the same name. For two member functions, correspondence
  compares their object parameters.
- When both functions are implicit object functions with
  ref-qualifiers, their object parameter types must be the same. A
  reference to const lvalue and a reference to const rvalue differ, so
  the inequality does not match. The reversed equality stays a
  candidate. This is GCC's result in the first column and option 2.
- When both functions have no ref-qualifier, the object parameter types
  must also be the same. A const member and a const volatile member
  differ, so the third column has no match in the draft either. A
  const member and a member with no cv-qualifier also differ, which
  covers the test with a different result and qualifiers. Every
  compiler finds a match there, so no compiler applies the draft's rule
  to cv-qualifiers.
- When exactly one function has no ref-qualifier, the draft compares the
  object types after removing references. The second column matches if
  its two functions have the same cv-qualifiers and differ only in the
  ref-qualifier. It does not match if the cv-qualifiers differ. The
  question does not say which pair it tested.
- The draft therefore supports option 2 and goes further. Both cv- and
  ref-qualifiers take part. The doubt is that every compiler ignores
  cv-qualifiers in this check. I have not found a later change that
  would explain that.

### Q-0344 in more detail

- The item O review found that the draft has both rules, the byte copy
  rule for trivially copyable types and the rule that a change to a
  const object is undefined, without saying which one wins.
- The draft also says that the library's byte copy and byte move
  functions implicitly create objects in the destination. Implicit
  creation makes new objects whenever doing so gives the program
  defined behavior.
- A new object of the class created over the second object ends the
  old object's lifetime. The old object is not a const complete object,
  only its member is const, so the new object transparently replaces
  it and the old name refers to the new object. No const object is
  changed, and the second object then has the first object's value.
  For a copy through the library functions, this reading supports
  option 2.
- A copy made byte by byte through a character pointer does not create
  objects. For that copy, the change to the const member stays
  undefined, as option 1 says.
- The doubt is whether implicit creation applies when the destination
  already holds a live object of the same type. The draft does not
  exclude it, but it does not show the case either.
