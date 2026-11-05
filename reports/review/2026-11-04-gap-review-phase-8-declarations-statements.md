# Gap review: the text phase 8 wrote on the declarations and statements chapters

Date: 2026-11-04
Role: source reader

Reference: the C++23 working draft in the local reference tree. The
review covers every page under the declarations and statements
chapters. For each page it reads the change from the commit before
dc90663 to commit 1b16115 and the rule text at HEAD. Part A compares
each rule and principle whose prose phase 8 added or changed with the
draft. Rules whose only change is a list of tests, an example marker,
or the Evidence section are left out. Part B applies each widened
principle to the program of each open question that waits on it, and
then checks the question against the draft. Part C reads Q-0377,
Q-0379, and Q-0385. No other question from Q-0377 to Q-0386 names a
rule on these pages. Later defect resolutions are mentioned only where
they matter.

## Result

Most of the new text agrees with the reference. All 25 held questions
on these pages get the provisional answer from their principle's
model, and the reference agrees with each of them. The three questions
of Part C also agree with the reference.

The gaps are these:

- ENUM-P1 leaves out two cases of an enumerator's type inside the
  list: an initializer whose type is an unscoped enumeration, and a
  first enumerator with no initializer.
- ENUM-P3 says the implementation can reject an enumeration that no
  type holds. The reference makes rejection required in that case.
- EXS-P1 and EXS-7 state the limit on the operand only as "no
  narrowing". The reference allows a fixed set of conversions, and one
  conversion outside the set is not a narrowing conversion.
- DEFARG-P1 lets a default argument name a local only in an operand
  that is not evaluated. The reference also allows a local whose use is
  not an odr-use, such as reading the value of a constant.
- IF-P4 leaves out local references among the variables whose
  initializer is tried as a constant, and its last sentence does not
  allow for an enclosing constant evaluation.
- INIT-P2 does not require that the copied indeterminate value itself
  have the byte type.
- TYN-P1 lists three things a class with a name for linkage must not
  have. The reference forbids more.
- TYN-P2 does not except explicit specializations and explicit
  instantiations.
- LNK-1 gives external linkage to every class and enumeration at
  namespace scope. It leaves out unnamed ones without a name for
  linkage, and module linkage.
- NS-P2 and NS-12 call every name declared both in an inline namespace
  and in the enclosing namespace ambiguous. Functions and a class
  hidden by another declaration are exceptions.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| ALN-P1 | agrees | The table of places matches the reference, the handler's variable included. The operand rules match. For an operand of class type, the reference also asks that the chosen conversion function be a constant function and that the conversion be unambiguous. |
| ALN-1, ALN-2 | agrees | The reference forbids the alignment specifier on the variable of a handler. |
| ALN-P3 | agrees | The reference makes the program ill-formed when an extended alignment is not supported. The support can depend on the context of the declaration, which ALN-P3 does not say. |
| ATTR-P1 | agrees, two small points | See "ATTR-P1" below. |
| ATTR-6 | agrees, doubt on the class | See "ATTR-6" below. |
| ENUM-P1 | missing cases | See "ENUM-P1" below. |
| ENUM-P2 | agrees | The reference asks that the enumeration named by the declaration have a reachable full definition with its list. An opaque declaration does not count. |
| ENUM-P3 | wrong classification for the last case | See "ENUM-P3 and ENUM-5" below. |
| ENUM-3 | agrees | The functional cast form is an explicit conversion and is valid for a scoped enumeration. |
| ENUM-16 | agrees | Direct initialization from an integer in parentheses needs an implicit conversion to the scoped type, and none exists. |
| ENUM-17 | agrees | The reference asks for a converted constant of the underlying type, which excludes narrowing conversions. |
| ENUM-19 | agrees | |
| ENUM-5 | agrees, doubt | See "ENUM-P3 and ENUM-5" below. |
| ENUM-7 | agrees | |
| EXS-P1, EXS-7 | missing condition | See "EXS-P1 and EXS-7" below. |
| DEFARG-P1 | missing case | See "DEFARG-P1" below. |
| DEFARG-P2 | agrees | The reference grants the right to add defaults in a later declaration only to functions that are not templates, and it asks that a member of a class template have all its defaults on the declaration in the class. A call sees the defaults of every reachable declaration in the function's own scope, also through a using declaration made earlier. |
| DEFARG-2 | agrees | |
| DEFARG-13 | agrees | The reference states the class template case outright. For a function template it gives the right to add defaults only to functions that are not templates, so the limit follows from that. Members of a class nested in a class template are templated members, so "lies inside one" fits. |
| DEFARG-10 | agrees, one narrow point | The reference has both exceptions. The delay of the check covers every default argument that is a complete-class context, so it also covers a friend function defined in the class and the members of a nested class. DEFARG-10 names only member functions. The immediate call exception applies to a call that is potentially evaluated. |
| FN-P5 | agrees, small points | The reference defines the variable with static storage duration, in the function's parameter scope. So the variable is visible in the member initializers of a constructor. Whether two such variables have distinct addresses is unspecified. FN-P5 calls it a local variable and says neither point. |
| FN-7 | agrees | The reference has no such recommendation. Reaching the end of the main function returns zero. FN-7 is Tidal's own advice. |
| INIT-P2 | missing condition | See "INIT-P2" below. |
| INIT-4, INIT-5 | agrees | Copy initialization and a parenthesized list allow any implicit conversion, narrowing ones included. |
| LIST-P1, LIST-2 | agrees | LIST-2 now points to CONV-21, which is on another page. The bool rows in the Evidence match the reference, where bool is an integer type for this rule and a value that does not fit narrows. |
| REF-P2 | agrees | The reference ends the temporary at the end of the full expression for an aggregate initialized from a parenthesized list, and for a new initializer. |
| LSP-P2 | agrees | |
| LSP-8 | agrees | The reference treats a C name and a global variable of the same spelling as one entity. Two declarations that give one entity different language linkages make the program ill-formed, and no diagnostic is required when neither declaration is reachable from the other. "In different translation units" is a looser test than reachability when modules are used. |
| NS-P2, NS-12 | agrees, missing exceptions | See "NS-P2 and NS-12" below. |
| NS-P3 | agrees | The reference forbids a repeated using declarator of the same entity only in a class scope. Two using declarations of one variable do not conflict, because they name the same entity. |
| SA-3 | agrees | The reference converts the condition to bool and asks only that the result be a constant, so a narrowing conversion is allowed. |
| LNK-1 | missing conditions | See "LNK-1" below. |
| SB-P2 | agrees | The reference uses the tuple protocol only when the trait class is complete and has a member named value. |
| SB-5 | agrees, one narrow point | In the reference, the member form needs a lookup in the class that finds a function template, and that template's first parameter must take a value. SB-5 says "member template". A member class template or variable template of that name does not count in the reference. |
| SB-P3 | agrees | The reference allows the storage class specifiers static and thread_local, the placeholder, and cv-qualifiers, and nothing else. It deprecates the volatile qualifier there, which SB-P3 does not mention. |
| TYN-P1 | missing conditions | See "TYN-P1" below. |
| TYN-P2 | missing exception | See "TYN-P2" below. |
| VOL-2, VOL-6 | agrees | The reference does not apply const or volatile semantics while the object is being built or destroyed. They take effect when the constructor of the most derived object ends, and stop when its destructor starts. |
| STMT-8 | agrees | The reference puts the parameters in a parameter scope that encloses the outermost block. A separate rule makes a declaration in that block ill-formed when it conflicts with a parameter. A class and a variable of one name do not conflict there, the same as in one scope. Tidal's model of one scope gives the same results. |
| IF-P4 | missing case, doubt | See "IF-P4" below. |
| IF-16 | agrees | The reference allows a narrowing conversion in this condition, and its own example says so. |
| RFOR-P1 | agrees | The four statements share one form of initializer statement, and the empty expression statement is one of its forms. |
| RFOR-7 | agrees | The reference takes the member form when the search in the class finds both names, whatever kind of member they are. The loop then needs valid member calls. |
| SW-3 | agrees | The reference asks for a converted constant of the adjusted type of the condition, which excludes narrowing conversions. |

## Principle uses

Each row applies the principle at HEAD to the question's program
without the rule text. Each row is a use of the principle under
decision 0056.

| Principle | Question | Test file | The model's answer | Matches the provisional answer? | Reference verdict |
|---|---|---|---|---|---|
| ENUM-P3 | Q-0018 | unscoped-value-too-large.cpp | The values need more bits than any standard type has. An implementation with a wider type can use it, and one without can reject (option 1). | yes | agrees, with the doubt under "ENUM-P3 and ENUM-5" |
| NS-P3 | Q-0019 | using-declaration-repeat-block.cpp | The second using declaration says the same thing again and does not conflict. The block is valid (option 1). | yes | agrees |
| FN-P5 | Q-0022 | func-outside-function.cpp | Nothing declares the variable at namespace scope, so the use is rejected (option 1). | yes | agrees |
| DEFARG-P1 | Q-0038 | parameter-in-sizeof.cpp | The size operand is not evaluated and needs only the parameter's type. The declaration is valid (option 1). | yes | agrees |
| INIT-P2 | Q-0039 | copy-indeterminate-unsigned-char.cpp, copy-indeterminate-unsigned-char-run.cpp | The copy into a byte object is defined at run time, and constant evaluation rejects it (option 1). | yes | agrees |
| REF-P2 | Q-0040 | aggregate-member-extends.cpp, aggregate-member-paren-not-extended.cpp | Braces extend the temporary. With parentheses the temporary ends with the full expression, so the later read is outside its lifetime and constant evaluation rejects it (option 1). | yes | agrees |
| ATTR-P1 | Q-0046 | likely-and-unlikely.cpp | The two hints on one statement contradict each other, so the implementation rejects the pair (option 1). | yes | agrees |
| ENUM-P2 | Q-0047 | using-enum-opaque.cpp | The opaque declaration names no enumerator, so the declaration that brings in the enumerators is rejected (option 1). | yes | agrees |
| SB-P2 | Q-0048 | tuple-size-without-value.cpp | The trait class is complete with no value member, so test 3 applies and the names bind to the data members (option 1). | yes | agrees |
| TYN-P1 | Q-0067 | typedef-unnamed-class-member-function.cpp | The class has a member function, so it cannot get a name for linkage from the alias. Rejected (option 1). | yes | agrees |
| ALN-P1 | Q-0069 | weaker-than-natural.cpp | An alignment weaker than the natural one is rejected (option 1). | yes | agrees |
| ALN-P1 | Q-0073 | enum.cpp | The table rejects the specifier on an enumeration (option 1). | yes | agrees |
| EXS-P1 | Q-0078 | narrowing-operand.cpp, pointer-operand.cpp | The value two and an address both narrow on the way to bool, so both are rejected (option 1). | yes | agrees |
| TYN-P2 | Q-0093 | elaborated-qualified-declaration-only.cpp | A class key with a qualified name alone declares nothing, so it is rejected (option 1). | yes | agrees |
| SB-P3 | Q-0107 | constinit-specifier.cpp | The constant-initialization specifier is not in the list of four, so it is rejected (option 1). | yes | agrees |
| ATTR-P1 | Q-0108 | carries-dependency-redeclared.cpp | A later declaration cannot add the attribute, so it is rejected (option 1). | yes | agrees |
| DEFARG-P2 | Q-0111 | using-sees-later-default.cpp | The call through the using declaration uses the defaults that the namespace has at the call, the later one included. Valid (option 1). | yes | agrees |
| ENUM-P1 | Q-0114 | enumerator-type-grows.cpp | The type of the enumerator before cannot hold the value, so the implementation picks some integer type that can (option 1). | yes | agrees |
| RFOR-P1 | Q-0122 | empty-init-statement.cpp | A lone semicolon is a form of the shared initializer statement. Valid (option 1). | yes | agrees |
| ALN-P1 | Q-0313 | operand-scoped-enumeration.cpp | A scoped enumerator converts to an integer only with a cast, so it is rejected (option 1). | yes | agrees |
| ALN-P1 | Q-0314 | operand-class-converts.cpp | The class converts to an integer by its conversion function, so the operand is valid (option 1). | yes | agrees |
| IF-P4 | Q-0329 | consteval-local-reference.cpp | The local reference binds a temporary, so its initializer runs at run time and the result is 2 (option 1). | yes | agrees |
| ATTR-P1 | Q-0347 | likely-identifier-label.cpp | The hint can go on a label of any kind, the target of a jump included. Valid (option 1). | yes | agrees |
| ALN-P3 | Q-0360 | extended-beyond-largest.cpp | The request is above the largest supported alignment, so it is rejected (option 1). | yes | agrees |

For Q-0313 and Q-0314, ALN-P1 speaks only to the alignment operand.
The reference treats the bit-field width the same way, as an integral
constant expression, so the same answers hold there. The array size of
a new expression is converted implicitly to the size type, so a scoped
enumerator is rejected there too.

## Questions

| Question | Provisional answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0377 | The class body sees the members of the qualifier (option 1). | agrees | option 1 | See below. |
| Q-0379 | The immediate call is checked at each call that uses the default (option 1). | agrees | option 1 | The reference makes this exception in its rule on the checking of default arguments. The call is neither evaluated nor checked as a constant at the declaration. The exception came with the C++23 change that lets immediate calls escalate, which was also applied to C++20 as a defect resolution. EDG 7.0 seems not to implement it. |
| Q-0385 | The definition repeats the whole template head, requires clause included (option 1). | agrees | option 1 | See below. |

### Q-0377

- In the reference, a declaration that redeclares or specializes a
  member of a namespace or class with a qualified name puts the part
  after that name into the scope of the namespace or class. The rule
  names the class head name and the enumeration head name as well as
  the declarator name. Friend declarations are excluded.
- So the body of a class defined with a qualified name sees the
  members of the qualifier, and so does a specialization of a library
  template defined outside the standard namespace.
- NAM-21 now names the class body. The same rule also covers the base
  clause of such a class, and the base and enumerator list of an
  enumeration defined with a qualified name. NAM-21 names neither.

### Q-0385

- The reference says that a member defined outside its class template
  takes a template head equivalent to the class template's. The
  statement is in a note.
- Two template heads are equivalent in the reference only if both have
  a requires clause or neither does, with equivalent constraints, and
  only if their type constraints match. A head without the clause is
  therefore a different head, and the definition matches no member of
  the class template.
- The normative support comes from the equivalence rule and from the
  rules for matching a definition to its declaration, so the reading is
  clear even though the direct statement is a note.

### ATTR-P1

- The reference allows the fallthrough statement only where the next
  statement to run carries a case or default label of the same switch.
  The table does not say "of the same switch". ATTR-2 also does not.
  A program that reaches a label of another switch is a rare case.
- The table says nodiscard goes on "a type". The reference allows a
  class or an enumeration, and a lambda's call operator. A type alias
  is not a place for it.

### ATTR-6

- The reference does not recommend a warning for unused variables. Its
  recommendation is only about what not to warn for. For a structured
  binding declaration without the unused marker, it says to warn only
  when all of its names are unused.
- ATTR-6 says "reads none of its names". The reference says "unused",
  which is wider than "not read". A name that is only assigned or whose
  address is taken is used. So ATTR-6 allows a warning in some cases
  where the reference advises against one.
- The rest of ATTR-6 is Tidal's own advice, as FN-7 is.

### ENUM-P1

- In the reference, an enumerator with an initializer has the type of
  the initializer, except when that type is an unscoped enumeration.
  Then the enumerator takes that enumeration's underlying type.
  ENUM-P1 gives the type of the initializer in every case.
- A first enumerator with no initializer has a signed integral type
  that the reference leaves unspecified. ENUM-P1 says an enumerator
  without an initializer keeps the type of the one before it, and the
  first one has none before it.
- When no integral type can hold an enumerator's value, the program is
  ill-formed. ENUM-P1 does not say so. ENUM-P3 covers the same case for
  the underlying type, with the doubt below.

### ENUM-P3 and ENUM-5

- The reference lets the implementation choose any integral type that
  holds all the values, extended integer types included. When no
  integral type of the implementation holds them, the program is
  ill-formed, and a diagnostic is required.
- ENUM-P3 says the implementation "can reject" in that case. The
  reference requires the rejection. The choice that is really left to
  the implementation is which extended types it has.
- ENUM-5 uses Tidal's term for integer type, which covers only the
  standard types. Read that way, ENUM-5 matches the reference: the
  implementation can use a wider type of its own, and must reject when
  it has none. Q-0018's option 1 fits this reading.
- Whether a given compiler's 128-bit type counts as an extended
  integer type is that compiler's own statement. The reference says
  nothing about it.

### EXS-P1 and EXS-7

- The reference asks for a constant converted to bool by a short fixed
  list of conversions. The list holds user-defined conversions, the
  lvalue, array, and function conversions, qualification and function
  pointer conversions, integral promotions, and integral conversions
  that do not narrow. It also holds the null pointer conversions from
  the null pointer type to a pointer.
- An integer to bool is an integral conversion in the reference, so the
  value one is allowed and the value two is not. A pointer to bool is a
  boolean conversion, which is not in the list at all. It is also a
  narrowing conversion, so EXS-P1 gives the right result.
- The null pointer literal converted to bool is outside the list, and
  it is not a narrowing conversion. The reference rejects it as the
  operand, and "must not narrow" allows it. EXS-7 and EXS-P1 miss this
  case. A floating value also falls outside the list, and EXS-P1
  rejects it by the narrowing test, so the result is the same.

### DEFARG-P1

- For a parameter, the reference bans any use in a default argument
  that could be evaluated. For a local of an enclosing function, the
  ban covers only an odr-use, which is a different test.
- So a default argument can read the value of a local constant of
  integral type, since such a read is not an odr-use. DEFARG-P1 and
  DEFARG-4 allow a local only in an operand that is not evaluated.
- The ban on the pointer to the object agrees with the reference,
  which allows it only after the cv-qualifiers of the member function's
  declarator. A default argument comes before that point, in every
  operand.

### INIT-P2

- The reference gives a defined result only when the indeterminate
  value itself has the type unsigned char or the standard byte type.
  The value is then copied into an object of one of those types by
  initialization or assignment, or passed through a cast to one of
  them, either branch of a conditional expression, the right side of a
  comma expression, or a discarded expression.
- INIT-P2 names only the type of the target. Copying an indeterminate
  value of type int into an unsigned char is undefined in the
  reference. Q-0039's program copies an unsigned char into an unsigned
  char and is covered.
- A later standard changes the reading of uninitialized values to a new
  kind of behavior. That change is not a defect resolution and does not
  apply to C++23.

### TYN-P1

The reference forbids these in an unnamed class that gets a name for
linkage from an alias. TYN-P1 and TYN-9 list only the first three.

- member functions, and any other member that is not a non-static data
  member, a member class, or a member enumeration, for example a static
  data member or a member alias,
- base classes,
- default member initializers,
- a lambda expression anywhere in the class,
- a member class that breaks any of these limits itself.

### TYN-P2

- The reference rejects a declaration made only of a class key and a
  qualified name. It excepts an explicit specialization and an explicit
  instantiation, which can have that shape.
- TYN-P2 says such a declaration declares nothing and is rejected, and
  states no exception. A friend declaration of that shape is valid, but
  the friend specifier makes it a different shape, so TYN-P2 does not
  cover it in any case.

### LNK-1

- In the reference, a class or enumeration at namespace scope has
  linkage only if it has a name. An unnamed one has linkage when an
  alias gives it a name for linkage, and an unnamed enumeration also
  when an enumerator does. Any other unnamed class or enumeration has
  no linkage. LNK-1 gives external linkage to every class and
  enumeration.
- A name declared in a named module and not exported has module
  linkage, not external linkage. LNK-1 does not mention module linkage.
- The reference also gives templates and named namespaces this
  linkage. LNK-1 does not name them.

### NS-P2 and NS-12

- The reference adds an implicit using directive for the inline
  namespace to the enclosing namespace. So an unqualified lookup in the
  enclosing namespace finds both declarations in one scope. A name of
  one kind is then ambiguous, as NS-12 says.
- When both declarations are functions, lookup gives an overload set,
  and overload resolution picks one or reports an ambiguity of its own.
  When one declaration is a class or enumeration and the other is not,
  the class or enumeration is set aside and the other is used. NS-12
  and NS-P2 call every such pair ambiguous, for qualified and for
  unqualified uses.

### IF-P4

- In the reference, a variable's initializer runs as a constant
  evaluation in two cases: the variable can be used in constant
  expressions, or its initialization is constant initialization. A
  trial run decides both. The variables that can be
  usable in constant expressions include references, as well as const
  integral and enumeration variables.
- A local reference that binds an object of static storage duration
  can have a constant initializer. Its initializer is then tried as a
  constant, and the branch for constant evaluation runs. IF-P4 lists
  only const integral and enumeration locals, and says every other
  initializer runs at run time.
- A local reference that binds a temporary is different. The temporary
  lives with the reference, so it has automatic storage duration. A
  constant expression cannot refer to such an object, so the
  trial fails and the initializer runs at run time. That is Q-0329's
  case, and IF-P4 gets it right.
- An initializer inside a function that is itself being evaluated as a
  constant also runs in constant evaluation. The reference's own
  example shows a plain local in such a function. "Every other
  initializer runs at run time" needs that exception.
