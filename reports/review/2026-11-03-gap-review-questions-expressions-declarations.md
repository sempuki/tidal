# Gap review: open questions on expressions, declarations, and the other foundations pages

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree (the
final C++23 draft). The review compares each answer with that draft
only. The cluster holds the 83 provisional questions whose first
Affects rule is on a page under expressions or declarations (other
than the pages on reference and list initialization), or on the
foundations pages for constant evaluation, conversions, evaluation
order, and values, and that had no gap review. Later changes are
mentioned only where they matter, and they are not in the reference.

## Result

Twelve of the 83 answers conflict with the reference in some form.

- Q-0025 contradicts the reference in part. The rule agrees with
  option 1, but no diagnostic is required when the specifier first
  appears after the initializing declaration.
- Q-0045 contradicts the reference. Static is required only in a
  namespace with external linkage, so the draft supports option 2.
- Q-0063 contradicts the reference, with a doubt about the range of a
  type that has infinities. The draft supports option 2.
- Q-0074 contradicts the reference. The draft treats two function types
  that differ only in linkage as distinct, so it supports option 2.
- Q-0077 contradicts the reference. The empty throw form is gone from
  the grammar, so the draft supports option 2.
- Q-0080 contradicts the reference. A const volatile static member
  that is not inline cannot have an initializer in the class, so the
  draft supports option 2.
- Q-0092 contradicts the reference in part. Option 1 holds for the
  trailing return type, and option 2 holds for the parameter list.
- Q-0123 contradicts the reference. The attribute goes before the
  cv-qualifiers, so the draft supports option 2.
- Q-0191 contradicts the reference. A dropped inner bound needs const
  at the levels above it, so the draft supports option 2.
- Q-0300 contradicts the reference. The draft has the reverse
  alignment step, so it supports option 2. No compiler in the table
  implements the step.
- Q-0330 contradicts the reference. An unknown escape of a basic
  character is conditionally supported, so the draft supports option
  3, and every compiler in the table conforms.
- Q-0335 contradicts the reference. The draft has no literal type
  limit for a temporary in constant evaluation, so it supports option
  2. The variable item of CE-17 has the same problem.

Three answers agree with a doubt (Q-0033, Q-0124, Q-0222). For
Q-0033, a later defect resolution supports option 1. Clang follows
that change, and GCC and EDG do not.

The draft is unclear for Q-0032. The draft is silent for Q-0020 and
Q-0184 (process questions) and for Q-0079 (a specification that
depends on itself).

The other 64 answers agree with the reference.

## Questions in the cluster

| Question | Answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0005 | Operand order of the built-in and overloaded forms is unspecified (option 1). | agrees | option 1 | For a built-in operator of this kind the draft leaves the two operands unsequenced, which is stronger than an unspecified order, since the two evaluations may also interleave. An overloaded operator written in operator form is a function call here, and the draft gives such calls no fixed argument order, because these operators carry no special ordering. The lack of variation in the built-in forms does not matter to the draft. |
| Q-0011 | No diagnostic is required (option 1). | agrees | option 1 | The draft requires a definition of each non-inline function or variable that the program uses in the sense of the one-definition rule, and it exempts a breach from diagnosis. Whether a use counts is decided from the source, so the answer's point about the optimizer is not the draft's reason. The classification is the same. |
| Q-0014 | Early initialization is allowed, and the value seen is unspecified (option 1). | agrees | option 1 | The draft lets an implementation turn a dynamic initialization into a static one under two conditions about other variables and the final value. When an earlier initializer in the same unit reads such a variable, the draft makes it unspecified which of the two values the reader sees. Both values in the test fit. |
| Q-0017 | Undefined outside the range of the enumeration's values (option 1). | agrees | option 1 | For an enumeration without a fixed underlying type, the draft defines the conversion only for a value inside the range of the enumeration's values, which comes from the smallest bit width that holds every enumerator. Outside that range the behavior is undefined. The draft also bars undefined operations from constant evaluation, so when the test performs the cast during constant evaluation, a diagnostic is required. GCC's and EDG's acceptance there is a defect. |
| Q-0018 | The underlying type is the implementation's choice, which may be a wider type of its own or a rejection (option 1). | agrees | option 1 | The draft makes the enumeration ill-formed when no integral type holds every enumerator value. Integral types include any extended integer types that an implementation provides, and their existence is the implementation's choice. So acceptance is valid only if the implementation counts its wide type as an extended integer type. GCC does not document its 128-bit type as one, so its acceptance under strict options is doubtful. |
| Q-0019 | A repeated using declaration in a block is allowed (option 1). | agrees | option 1 | The draft forbids naming one declaration twice through using declarations only when both stand in the same class scope. Two using declarations in a block that name the same entity do not conflict under the draft's scope rules, since both refer to the same entity. The older rule that rejected the block form was removed by the rewrite of scope and lookup that the draft contains. |
| Q-0020 | A per-compiler expectation with a note (option 1). | silent | none | A process question about the test format. The draft says nothing about it. |
| Q-0021 | A misplaced standard attribute is rejected (option 1). | agrees | option 1 | The draft makes a program ill-formed when an attribute appears where its own rules do not allow it. The function-only attribute, the discard attribute, and the overlap attribute each name the entities they may apply to, and a plain variable or a bit-field is not among them. A warning counts as a diagnostic, so GCC's and Clang's warnings conform, though they then accept the program as an extension. A later change that lets an implementation ignore standard attributes keeps this placement rule. |
| Q-0022 | Rejected outside a function (option 1). | agrees | option 1 | The draft defines the predefined function name only inside a function body, as if a local variable were declared there. Outside a function the name is undeclared, so the use is ill-formed and needs a diagnostic. Clang's warning is a diagnostic, and its acceptance is an extension. |
| Q-0025 | When any declaration has the specifier, the initializing declaration must have it, and both programs are rejected (option 1). | contradicts, in part | option 1 for the rule, with no diagnostic needed for the first program | See below. |
| Q-0029 | A chained comparison is valid, and a warning is recommended (option 1). | agrees | option 1 | The draft groups the relational operators from left to right and converts the bool result to the other operand's type by the usual arithmetic conversions. Nothing in the draft makes the form ill-formed or deprecated. Clang 22's default rejection does not conform, though a flag restores conforming behavior. |
| Q-0030 | Rejected (option 1). | agrees | option 1 | For the relational operators, the draft converts operands to a composite pointer type only when both are pointers. A pointer and an integer zero reach neither the arithmetic path nor the pointer path, and the operands then differ in type, which is ill-formed. The null pointer type is neither arithmetic nor a pointer type, so that operand is ill-formed too. The three-way operator requires an object pointer or an array on both sides. EDG's acceptance is a defect. |
| Q-0031 | Forming the pointer is undefined (option 1). | agrees | option 1 | The draft defines adding an integer to a pointer into an array only when the result lands on an element or one past the last element. Any other result is undefined at the moment of the addition, with no read needed. The draft bars undefined operations from constant evaluation, so a diagnostic is required there, and GCC's acceptance is a defect. |
| Q-0032 | A braced list is allowed only after simple assignment (option 1). | unclear | none | See below. |
| Q-0033 | Rejected during constant evaluation (option 1). | agrees, with doubt | option 1 | See below. |
| Q-0034 | An xvalue (option 1). | agrees | option 1 | The draft makes a subscript of an array operand an lvalue when the array is an lvalue, and an xvalue in every other case. An array prvalue is first materialized into a temporary, which gives an xvalue. EDG's lvalue is a defect. |
| Q-0035 | Rejected (option 1). | agrees | option 1 | The draft requires the operand of a delete expression to be a pointer to an object type or a class type that converts to one. A pointer to void is not a pointer to an object type, so a diagnostic is required. GCC's warning is a diagnostic, and its acceptance is an extension. |
| Q-0036 | Accepted, with the size converted to the size type (option 1). | agrees | option 1 | The draft converts the first array size in a new expression implicitly to the unsigned size type. A floating value converts implicitly by the ordinary floating-integral conversion, so the form is valid. An older rule limited the size to integral or enumeration types, and GCC still applies it. |
| Q-0037 | Rejected (option 1). | agrees | option 1 | The draft allows a capture default or a simple capture only when the lambda's innermost enclosing scope is a block, or when it sits in a default member initializer and the innermost scope is that class. A lambda at namespace scope fits neither case. The rule does not depend on whether anything would be captured. EDG's acceptance is a defect. LAM-6 already states the member initializer exception. |
| Q-0038 | Valid, since the ban covers only evaluated uses (option 1). | agrees | option 1 | The draft bans a parameter in a default argument only where the parameter would be potentially evaluated. The operand of the size operator is unevaluated, so the use is valid. EDG's rejection is a defect. |
| Q-0039 | Not undefined at run time for unsigned char and byte, and rejected during constant evaluation (option 1). | agrees | option 1 | The draft makes producing an indeterminate value undefined, with exceptions for the unsigned ordinary character type and the byte type when the value is copied into an object of such a type. The copy then holds an indeterminate value. Constant evaluation forbids reading an object whose value is indeterminate, so the constant form needs a diagnostic. A later change replaces many of these undefined reads of uninitialized automatic variables with erroneous behavior, which matters for INIT-1 as a whole and not for this exception. |
| Q-0045 | Static is needed at every namespace scope (option 1). | contradicts | option 2 | See below. |
| Q-0046 | The pair is rejected (option 1). | agrees | option 1 | The draft forbids the two attributes in one sequence of attribute specifiers. Two adjacent bracketed specifiers form one such sequence. The rule is not exempt from diagnosis, so GCC's warning alone falls short. |
| Q-0047 | The enumeration must be complete (option 1). | agrees | option 1 | The draft requires the named enumeration to have a reachable definition with its enumerator list. An opaque declaration gives no such list, so EDG's acceptance is a defect. |
| Q-0048 | The bindings fall back to the data members (option 1). | agrees | option 1 | The draft uses the tuple protocol only when the trait class is complete and has a member with the value name. Without that member, the draft moves on to the data member case. |
| Q-0049 | The subobject acts as an array of one element (option 2). | agrees | option 2 | The draft treats an object outside any array as the only element of a one-element array for pointer arithmetic. A note says that adding anything other than zero or one to a pointer to a base subobject is undefined. A difference needs two elements of one array, and the two base subobjects are not. The position one past the end points to no object, so a read through it is undefined. |
| Q-0050 | Rejected (option 1). | agrees | option 1 | The draft applies the arithmetic conversions to the three-way operator only when both operands are arithmetic, or when one is integral and the other an unscoped enumeration. An enumeration with a floating type fits neither case and no other case, so the program is not valid. |
| Q-0051 | Rejected during constant evaluation, unspecified at run time (option 1). | agrees | option 1 | The draft leaves the result unspecified when one pointer is one past one complete object and the other points to a different complete object. An equality comparison with an unspecified result is not allowed in a constant expression. EDG's acceptance is a defect. |
| Q-0063 | The value is implementation-defined, and an implementation can reject the literal (option 1). | contradicts, with doubt | option 2 | See below. |
| Q-0064 | The program is not valid (option 1). | agrees | option 1 | For a decimal literal without a suffix, the draft lists only signed types. If none of them holds the value, the literal may take a signed extended integer type, and otherwise the program is not valid. An unsigned type is never allowed. EDG's acceptance in strict mode is a defect unless EDG has a signed extended type wide enough. |
| Q-0065 | The decltype form must not occur in the qualified name of a declaration (option 1). | agrees | option 1 | The draft forbids that form in the qualifier of a name being declared. The rule does not touch qualifiers in expressions. |
| Q-0066 | Not valid, with no diagnostic required (option 1). | agrees | option 1 | The draft makes a program not valid, with no diagnostic required, when a search in a class gives a different result from the same search in the completed class. A name used before a later member declares the same name fits this case. GCC's diagnostic is allowed. |
| Q-0067 | The class must be C-like (option 1). | agrees | option 1 | The draft limits an unnamed class that gets a typedef name for linkage to data members, member classes, and member enumerations. It allows no base and no default member initializer, and the limits apply to nested classes too. The rule entered with C++20, so GCC 14's acceptance is a defect. |
| Q-0068 | A function must not be declared inline after its definition (option 1). | agrees | option 1 | The draft makes the program not valid when a definition is reachable at the first inline declaration. A diagnostic is required. |
| Q-0069 | Not valid (option 1). | agrees | option 1 | The draft forbids alignment specifiers whose combined effect is weaker than the alignment the entity would have without them. A diagnostic is required. |
| Q-0070 | The definition must have the same alignment (option 1). | agrees | option 1 | The draft requires every definition to state an equivalent alignment when any declaration states one. A non-defining declaration may leave the specifier out. A diagnostic is required within one translation unit. The rule covers variables and classes alike. |
| Q-0071 | The strictest alignment applies (option 1). | agrees | option 1 | The draft takes the strictest of all alignment specifiers on an entity, with no exception for classes. GCC's result on a class is a defect. |
| Q-0073 | The specifier must not apply to an enumeration (option 1). | agrees | option 1 | The draft lists variables, data members other than bit-fields, and classes as the entities that accept an alignment specifier. An enumeration is not a class and is not in the list. |
| Q-0074 | Implementation-defined (option 1). | contradicts | option 2 | See below. |
| Q-0075 | Not valid (option 1). | agrees | option 1 | The draft makes a declaration with C language linkage and a global variable of the same name declare one entity. The two declarations then give that entity different language linkages, which makes the program not valid. A diagnostic is required when one declaration is reachable from the other. |
| Q-0076 | Not valid in both cases (option 1). | agrees | option 1 | The draft forbids a linkage specification on the main function. It also makes a program not valid when it gives C language linkage to anything called main, whatever namespace holds it. |
| Q-0077 | Implementation-defined; where accepted, the form means non-throwing (option 1). | contradicts | option 2 | See below. |
| Q-0078 | The operand must convert to the boolean type without narrowing (option 1). | agrees | option 1 | The draft requires the operand to be a constant expression contextually converted to the boolean type, with only a short list of conversions. Conversions from an integer or a pointer to the boolean type are not on that list. The C++23 change that relaxed this for static assertions and constexpr if did not cover the exception specifier. |
| Q-0079 | The program is not valid, and no diagnostic is required (option 1). | silent | none | See below. |
| Q-0080 | The initializer in the class is valid (option 1). | contradicts | option 2 | See below. |
| Q-0089 | The declared-type specifier can name the class in an explicit destructor call (option 1). | agrees | option 1 | The grammar of an unqualified name has a form with a tilde and a declared-type specifier, and that form names the destructor of the type it denotes. After a dot or an arrow the tilde can only start a destructor name. GCC's rejection is a defect. |
| Q-0091 | The behavior is undefined, so constant evaluation rejects the deletion (option 1). | agrees | option 1 | For the array form of delete, the draft makes the behavior undefined when the dynamic type of the deleted object is not similar to the static type. A virtual destructor does not help the array form. Undefined behavior makes the evaluation fail as a constant expression, so a diagnostic is required. GCC's acceptance is a defect. |
| Q-0092 | Captures are visible in the parameter list and the trailing return type (option 1). | contradicts, in part | option 1 for the trailing return type, option 2 for the parameter list | See below. |
| Q-0093 | The declaration is not valid (option 1). | agrees | option 1 | A declaration made of only a class key and a name is valid only in a few listed forms, and each listed form has a plain identifier or a template name with arguments. A qualified name is not one of them, unless the declaration is an explicit specialization or an explicit instantiation. A diagnostic is required. |
| Q-0094 | A deallocation function is non-throwing without the keyword (option 1). | agrees | option 1 | The draft gives every deallocation function with no explicit exception specifier a non-throwing specification. The rule names no exception for a class member or for a form with a size parameter. GCC's result is a defect. |
| Q-0095 | The defaulted comparison is potentially throwing (option 1). | agrees | option 1 | For a comparison operator defaulted on its first declaration with no exception specifier, the draft makes the specification potentially throwing exactly when an expression in its implicit definition is potentially throwing. A call of a member's equality operator without a non-throwing specifier is such an expression. EDG's result is a defect. |
| Q-0100 | Only an integer literal with value zero is a null pointer constant (option 1). | agrees | option 1 | The draft defines a null pointer constant as an integer literal of value zero or a prvalue of the null pointer type. A character literal is a separate kind of literal, so it does not qualify. EDG follows the older rule, in which any integral constant expression of value zero counted. |
| Q-0102 | The result has the type of the null pointer literal (option 1). | agrees | option 1 | The last case of the conditional operator covers a pair of operands in which one is of the null pointer type and the second is a null pointer constant. That case gives the result the null pointer type. The integer literal zero is a null pointer constant. GCC's rejection is a defect. |
| Q-0103 | The comparison is not valid (option 1). | agrees | option 1 | The relational operators require arithmetic, enumeration, or pointer operands after the usual conversions. The null pointer type is none of these. The equality operators list the null pointer type, and the relational operators do not. A diagnostic is required, so EDG's acceptance is a defect. |
| Q-0104 | The pointers are equal (option 1). | agrees | option 1 | The class in the test is standard-layout, so the object and its first member are pointer-interconvertible and have the same address. Two pointers that represent the same address compare equal, and the result is specified, so constant evaluation must give equality. EDG's result is a defect. |
| Q-0105 | The declaration redeclares the namespace-scope entity and takes its internal linkage (option 1). | agrees | option 1 | The draft lets a block-scope declaration with the extern specifier redeclare the earlier entity even when a local declaration hides the earlier one, and the redeclaration gets internal linkage. An example in the draft shows this case. EDG follows the older rule, under which the hidden entity was not found and the result was a linkage conflict. |
| Q-0106 | Not valid (option 1). | agrees | option 1 | The draft lists the only places where a function type with a cv-qualifier or a ref-qualifier can appear. The list covers a non-static member function, the target of a pointer to member, the type in a function alias, and two template cases. A reference to such a type is not on the list, just as a plain pointer is not. |
| Q-0107 | Not valid; only static, thread-local, and the cv-qualifiers can join the placeholder (option 1). | agrees | option 1 | The draft allows only four kinds of declaration specifier in a structured binding declaration, which are static, thread-local, the deduced placeholder, and the cv-qualifiers. The constant-initialization specifier is not allowed. A later change for the next standard allows the constant-expression specifier on a structured binding. I have not checked whether that change also allows the constant-initialization specifier. |
| Q-0108 | Not valid (option 1). | agrees | option 1 | The draft requires the first declaration of a function to carry the dependency attribute if any declaration does. Within one translation unit the rule is not exempt from diagnosis. A later change removes the attribute's requirements for the next standard, so the attribute there has no effect, which matches GCC. |
| Q-0111 | The default argument is usable through the using declaration (option 1). | agrees | option 1 | A call through an overload set uses the default arguments of every reachable declaration in the function's own scope. A note in the draft says such a call can find the function through a using declaration from which the declaration with the default argument is not reachable. Clang's rejection is a defect. |
| Q-0114 | The type is unspecified, any integral type that holds the value (option 1). | agrees | option 1 | Without a fixed underlying type, an enumerator with no initializer keeps the type of the one before it unless the value no longer fits. In that case the draft gives it an unspecified integral type large enough for the value, and the program is not valid if no such type exists. |
| Q-0123 | The attribute comes after the cv-qualifiers (option 1). | contradicts | option 2 | See below. |
| Q-0124 | The step is done in the promoted type, and the result converts back to the operand's type (option 1). | agrees, with doubt | option 1 | See below. |
| Q-0178 | A call through a non-throwing pointer type to a function that may throw is undefined (option 1). | agrees | option 1 | The draft makes a call undefined when the type used for the call differs from the type of the function's definition. The one exception covers a pointer type that a function pointer conversion could produce from the function's own pointer type. That conversion only removes the non-throwing property, so the question's direction falls outside the exception. |
| Q-0184 | Evidence can name a vendor option when a test runs the compiler with that option (option 2). | silent | none | A process question about Tidal's Evidence. The draft has nothing to say about it. |
| Q-0191 | A dropped inner array bound needs no const at the levels above (option 1). | contradicts | option 2 | See below. |
| Q-0194 | An operand of a fold needs parentheses around a binary operator (option 1). | agrees | option 1 | The draft's grammar makes each operand of a fold a cast expression. An operand that holds an unparenthesized binary operator does not fit, so the fold is not valid. EDG's acceptance is a defect. |
| Q-0221 | A new expression is not valid when its matching deallocation function is deleted (option 1). | agrees | option 1 | The draft counts the single matching deallocation function for a class as named by the new expression, so it is used. Any use of a deleted function, apart from its declarations, makes the program not valid. The draft does not depend on whether the initialization can throw. GCC's acceptance is a defect. |
| Q-0222 | Valid, and no deallocation function is called when two templates match with no preference (option 1). | agrees, with doubt | option 1 | See below. |
| Q-0300 | No reverse step, and the program is rejected (option 1). | contradicts | option 2 | See below. |
| Q-0301 | No allocation call, and an exception that a handler for the bad array length class catches (option 1). | agrees | option 1 | The draft lists the three erroneous cases and says that no allocation function is called. With a potentially throwing allocation function, the expression throws an exception that matches a handler for the bad array length class. In a constant expression the program is not valid. The draft measures a bound that is too large against an implementation-defined maximum object size. |
| Q-0302 | A null pointer when the allocation function is non-throwing (option 1). | agrees | option 1 | When the allocation function chosen for the expression cannot throw, the draft makes the result a null pointer of the result type. No allocation function is called. GCC's exception is a defect. |
| Q-0313 | A scoped enumerator is not valid in any of the three places (option 1). | agrees | option 1 | The alignment operand and the bit-field width must be integral constant expressions, which the draft limits to integral and unscoped enumeration types. The size in an array new is implicitly converted to the size type, and a scoped enumeration has no implicit conversion to an integer. EDG's and GCC's acceptances are defects. |
| Q-0314 | A constant of class type converts by its conversion function (option 1). | agrees | option 1 | Where the draft requires an integral constant expression, the draft converts an expression of literal class type contextually to an integer or unscoped enumeration type. The chosen conversion function must be constexpr. The alignment operand and the bit-field width are such places. GCC's rejection is a defect. |
| Q-0316 | A member access inside the class definition sees the earlier members (option 1). | agrees | option 1 | The draft requires a complete class for a member access except inside the definition of that class. The program is not valid if the result would differ once the class is complete. A member declared later is not found at the point of the access. Clang's rejection is a defect. |
| Q-0319 | The copy into the explicit object parameter takes an unspecified place among the arguments (option 1). | agrees | option 1 | The draft puts the implied object argument first in the list that matches arguments to parameters, so the explicit object parameter is initialized like any other parameter. The postfix expression is sequenced before the arguments. Each parameter's initialization, with its side effects, is indeterminately sequenced with the others. Interleaving is not allowed, but either order is. |
| Q-0326 | The size of a volatile access is implementation-defined (option 1). | agrees | option 1 | The draft makes the meaning of a volatile access implementation-defined. A conforming implementation must still carry out volatile accesses as the abstract machine does. The draft does not say directly whether two accesses may merge into one. Separately, the draft does not apply volatile semantics to an object while its constructor runs, which bears on the open part about an initializer. |
| Q-0330 | An unknown escape is invalid and is rejected (option 1). | contradicts | option 3 | See below. |
| Q-0335 | Constant evaluation rejects a temporary of a type that is not literal (option 1). | contradicts | option 2 | See below. |
| Q-0347 | The likelihood attribute can apply to an identifier label (option 1). | agrees | option 1 | The draft lets the two likelihood attributes apply to labels or statements, and its label grammar allows an attribute before each of the three kinds of label. Clang's rejection is a defect. |
| Q-0348 | The pointer to member keeps the ref-qualifier (option 1). | agrees | option 1 | The draft lets a function type with a ref-qualifier, including one named through an alias, be the target type of a pointer to member. The qualifier is part of the function type, and nothing removes it. GCC's rejection is a defect. |
| Q-0353 | A converted member pointer names the same member wherever its value goes (option 1). | agrees | option 1 | The draft's conversion of a pointer to a member of a base to a pointer to a member of a derived class keeps the member it refers to. A constexpr function gives the same result as an equivalent ordinary function, apart from copy elision. Nothing in the draft lets the value change when it passes through an object or a return. EDG's results are a defect. |
| Q-0360 | The implementation rejects an alignment it does not support (option 1). | agrees | option 1 | The draft makes the program not valid when the operand gives an extended alignment that the implementation does not support in that context. Which extended alignments are supported is implementation-defined. A diagnostic is required, so Clang's silent acceptance is a defect. |

### Q-0025 in more detail

- The draft states the rule of option 1. If any declaration of a variable carries the specifier, the initializing declaration must carry it too.
- The draft exempts a breach from diagnosis when the initializing declaration cannot reach any declaration that has the specifier.
- In the first program the specifier first appears after the initializing declaration, so no declaration with it is reachable there. That program is ill-formed, but no diagnostic is required. GCC's and EDG's acceptance conforms, and CINIT-2's reject classification is too strong for this case.
- In the second program the declaration with the specifier comes first and is reachable from the initializing declaration, so a diagnostic is required. GCC's acceptance is a defect, and Clang's and EDG's rejection is right.

### Q-0032 in more detail

- The grammar of the draft lets any assignment operator, compound forms included, take a braced list as its right operand.
- The draft then says where such a list may appear. For a scalar target it gives a meaning only for simple assignment. For a class target it passes the list to the selected operator function, and an example in the draft uses a compound operator on a class.
- The draft does not say clearly whether a compound assignment to a scalar counts as an assignment to a scalar for this purpose, or what such a form would mean. Implementations split, and the question is a known open issue for the committee.
- The draft gives no clear support to either option. Option 1 is the reading that needs no invented meaning, while Clang's reading is also possible.

### Q-0033 in more detail

- The draft leaves it unspecified whether two string literals with the same text share storage, and whether two evaluations of one literal give the same object.
- The draft bars an equality comparison whose result is unspecified from constant evaluation. The draft does not say plainly that a comparison between two literal objects falls under that bar, because the equality rules themselves define the result by whether the pointers point to the same object.
- A later defect resolution states that a comparison whose result depends on whether such literal objects overlap is not a constant expression. That change supports option 1. Clang follows it, and GCC and EDG do not.
- The comparison in the test applies the equality operator to two array operands, which the draft marks as deprecated.

### Q-0045 in more detail

- The draft requires the static specifier only for an anonymous union in a namespace that has external linkage.
- An unnamed namespace has internal linkage, and so does any namespace nested inside one. An anonymous union declared there falls outside the requirement.
- So the draft accepts the form without static, as Clang does. GCC's and EDG's rejection is a defect. The draft supports option 2, and STO-11 and UNI-9 apply the requirement too widely.

### Q-0063 in more detail

- The draft makes a program not valid when the scaled value of a floating literal lies outside the range of representable values of its type. A value inside the range that is not exact is rounded up or down, and the choice is implementation-defined.
- The draft gives no permission to accept an out-of-range literal with a warning. On that reading, option 2 holds, and Clang and GCC miss a required diagnostic.
- The doubt is the meaning of the range for a type that has infinities. If the range of such a type is taken to cover all real numbers, every finite literal is in range and rounds to infinity, which is Clang's and GCC's result. The draft does not settle that point clearly.

### Q-0074 in more detail

- In the draft, a function type with C linkage and one with C++ linkage count as separate types, even when their parameters and return type match. This is option 2, and EDG follows it.
- The draft gives no permission for an implementation to merge the two types. So Clang and GCC do not conform here, and they have not conformed for many years.
- I know of no adopted change after C++23 that removes the linkage from the function type. The behavior of Clang and GCC is a long-standing gap between the draft and practice.

### Q-0077 in more detail

- The draft's grammar for an exception specification has only the noexcept forms. The empty throw form was removed in C++20, and the draft lists the removal among its changes from C++17.
- So the empty throw form is a syntax error, and a diagnostic is required. The draft supports option 2, which is EDG's result.
- Clang and GCC accept the form without a diagnostic under their strictest mode, which is a defect. A warning would be enough to conform.

### Q-0079 in more detail

- The draft lists when an exception specification is needed. One case is a function chosen by overload resolution in an expression. The operand of the member's own specifier calls the member, so evaluating the specification needs the specification itself.
- The draft states no rule for a specification that depends on itself. No rule makes such a program invalid, with or without a diagnostic, and no rule gives the specification a fixed value.
- The draft's deferred handling of these specifiers covers template specializations, and its limit on recursive instantiation does not reach a member of an ordinary class.
- So the draft supports none of the three options. Option 1 is a reasonable reading of the gap, and Clang's error is consistent with it.

### Q-0080 in more detail

- The draft lets a static data member declared in its class have an initializer there only in two cases. One case is a const member of integral or enumeration type that is also non-volatile and non-inline. The other is an inline member, which includes a member with the constant-expression specifier.
- Every other static data member declared in a class is barred from having an initializer there. The bar is not exempt from diagnosis.
- A const volatile member of integral type that is not inline fits neither case. So the in-class initializer is not valid and must be diagnosed, which is Clang's result and option 2. GCC's and EDG's acceptance is a defect.
- An inline member with the same type and initializer would be valid.

### Q-0092 in more detail

- The draft gives a lambda its own scope that starts right after the capture list and runs to the end of the body.
- A name that would refer to a variable captured by copy gets the type of the matching closure member when the name appears in the lambda's function parameter scope outside the parameter declaration clause. The trailing return type is in that region. So the parenthesized declared-type form there gives a reference to const in a lambda without the mutable keyword. GCC 16 and EDG agree. Clang and GCC 14 follow the earlier rule.
- A name in the parameter declaration clause itself, such as in a default argument or a parameter type, gets the type of the outer variable. The draft states that exception directly. So option 1 is right for the trailing return type and option 2 is right for the parameter list.
- The test in the question uses only the trailing return type, so its expected result agrees with the draft. The rule's mention of the parameter list does not.

### Q-0123 in more detail

- The draft's grammar for a pointer declarator operator puts an optional attribute list right after the asterisk and before an optional cv-qualifier sequence. The same order holds for a pointer to member.
- So an attribute between the asterisk and a cv-qualifier is valid, and an attribute after the cv-qualifier is outside the grammar of that operator. This is Clang's result and option 2.
- An attribute after the cv-qualifiers might still parse as an attribute at another position, such as one that follows the declarator name. I found no such position in the draft for the place in the test, so GCC's and EDG's acceptance looks like an extension.

### Q-0124 in more detail

- For the prefix form, the draft defines the operation as equivalent to a compound assignment that adds one. The addition runs in the promoted type, and the assignment converts the sum back to the operand's type. That conversion is defined for every value and wraps modulo the width. So the prefix step is defined, which supports option 1, and EDG's rejection in a constant expression is a defect.
- For the postfix form, the draft says only that the operand is changed by adding one, and points to addition and assignment for conversions only in a note on the prefix form. The draft does not state that the postfix form uses the same promotion and conversion. The two forms are treated alike in practice, and nothing in the draft makes the postfix step undefined on its own. The doubt is only in the wording of the postfix form.

### Q-0191 in more detail

- The draft builds a combined type from the two types level by level.
  At any level where the combined type differs from either input,
  whether in its qualifiers or by having an unknown bound where an input
  has a bound, the draft adds const to every level between the
  outermost and that level.
- The conversion is allowed only when the combined type equals the
  target type.
- For a pointer to a pointer to an array of three, with the bound
  dropped at the array level, the draft therefore requires const on the
  middle pointer level. A target without it is not the combined type,
  and the conversion is not valid.
- The draft gives the same treatment to a dropped bound as to added
  qualifiers. This is Clang's result, and option 2 states it. GCC's and
  EDG's acceptance is a defect.

### Q-0222 in more detail

- The draft says that a placement deallocation function is called only
  when the lookup finds a single matching one. Otherwise none is
  called, and an exception from the initialization leaves the storage
  in place. The draft names no error for a lookup that finds more than
  one match.
- So two matches give the same result as no match, which is option 1.
- The doubt is how templates take part. The draft defines a match by
  comparing the parameter types of a declaration with those of the
  allocation function, and does not describe deducing template
  arguments or ordering templates for this purpose. EDG's reading, in
  which the templates are compared as in overload resolution and the
  tie makes the expression ambiguous, is possible under that silence.
- The draft treats a single matching deallocation function as named by
  the expression. With no single match, nothing is named, so no access
  or deletion check applies.

### Q-0300 in more detail

- The draft builds the argument list for the allocation function. For
  a type with an extended new alignment it adds the alignment after the
  size.
- If no function matches, the draft takes one of two steps and tries
  overload resolution again. For a type with extended new alignment it
  removes the alignment. For any other type it inserts the alignment
  after the size.
- So the draft has the reverse step, and the program in the question is
  valid. The draft supports option 2, the same reading as the earlier
  cluster 1 review.
- Every compiler in the question's table rejects the program, so none
  of them implements the reverse step in C++23 mode. I have not checked
  when the step entered the working draft.

### Q-0330 in more detail

- The draft's escape grammar has a third kind of escape sequence beside
  the simple and numeric ones. The third kind is a backslash followed
  by a member of the basic character set that has no other use after a
  backslash. A letter such as q falls in this kind.
- A character literal made of such a sequence is conditionally
  supported, with an implementation-defined value. The draft treats the
  same sequence in a string literal the same way, with an
  implementation-defined code unit sequence.
- An implementation that supports the construct documents its meaning.
  One that does not must diagnose the program. EDG's warning with a
  translated program fits the first case. Clang's and GCC's errors
  under their strict options fit the second.
- So the draft supports option 3, and every compiler in the table
  conforms. A backslash followed by a character outside the basic
  character set matches none of the escape forms, and that program is
  not valid.

### Q-0335 in more detail

- The draft's list of constructs that stop an expression from being a
  core constant expression has no entry for a temporary, or for a local
  variable, of a type that is not literal.
- The draft requires a literal type only for an object declared
  constexpr.
- A constexpr constructor and a constexpr member function of a class
  that is not literal are allowed in the draft. The call in the test
  reads only a non-volatile member, so no volatile read happens.
- So the evaluation is a constant expression, and Clang 22's acceptance
  conforms. The draft supports option 2.
- Clang 18 predates Clang's support for the C++23 relaxation of literal
  types in constexpr functions, which the draft contains. GCC 14 and 16
  and EDG 7.0 do not follow it for this case.
- The same reasoning reaches the item of CE-17 about a variable of a
  type that is not literal. The draft has no such limit for a variable
  in a constexpr function's evaluation. Only a variable declared
  constexpr needs a literal type.
