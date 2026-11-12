# Second review: the open questions held only by a doubt (DQ)

Date: 2026-11-11

Role: source reader

Reference: the C++23 working draft sources.

Commit read: 9c409044e63f295e353c0c92de1bd74579069310.

The review is item DQ of decision 0066. It covers each question whose status is provisional and whose only wait is a review with a doubt. A read-only script found 44 such questions. For each one the review read the provisional answer, the earlier Opinions entries, and the rules and tests that the question names. It then read the reference on the point that the doubt names.

Each question gets one reading:

- supports: the doubt is resolved for the answer,
- supports with a doubt that remains: the point names the doubt more narrowly,
- reads against: the point says what the reference does,
- silent: the reference does not settle the point.

Points carry IDs of the form DQ-n. A point exists only for a reading that reads against or that narrows a doubt.

## Result

Of the 44 questions, 14 are supported, 25 are supported with a doubt that remains, 4 read against the reference, and 1 finds the reference silent.

The four readings against:

- Q-0155, the literal that is too long for the array. The list rule for a braced string literal checks the kind of the characters and nothing about the bound. So the identity exists, and the pointer candidate ties with it, as Clang finds (DQ-7).
- Q-0189. The result tie-breaker compares the conversions from each candidate's return type. A constructor has no return type, and nothing in the reference supplies one. The reading leans to option 2, Clang's result (DQ-10).
- Q-0269. The rules that decide whether two function template declarations correspond treat the placeholder form and the written-out template as one template, in the unconstrained form and in the constrained form. Only an example in the reference says the two forms differ (DQ-11).
- Q-0274. The call in the test names a member whose using-declarator has a dependent final name, so the call depends on the parameters. The rule on a hypothetical instantiation covers only constructs that do not depend on them. In the empty instantiation the lookup finds nothing in the class and goes on to the namespace function, as GCC and EDG do (DQ-12).

Two points touch more than one question:

- Q-0001 and Q-0028 rest on one doubt: whether an infinity counts as a value of a floating type. The reference leaves the representation of floating types to the implementation. Q-0001 answers as if an infinity is not a value, and Q-0028 answers as if it is. The two answers cannot both follow from one reading (DQ-1, DQ-2).
- Q-0304, Q-0327, and Q-0359 all depend on the step of a byte pointer through an object. The letter of the reference still gives that step no defined result (DQ-16, DQ-19, DQ-21).

Several doubts are resolved. Q-0117, Q-0222, Q-0345, Q-0411, and Q-0415 each have a passage in the reference that settles the earlier doubt. For Q-0221 the reference says plainly that a new expression names the single matching deallocation function of a class.

## Questions

| Question | Reading | Note |
|---|---|---|
| Q-0001 | supports with a doubt that remains | DQ-1. |
| Q-0018 | supports | The enumeration is not valid when no integral type of the implementation can hold every value, and the choice among the types that can is the implementation's. Whether GCC's 128-bit type is an extended type is a fact about GCC. The answer holds either way. |
| Q-0028 | supports with a doubt that remains | DQ-2. |
| Q-0032 | supports with a doubt that remains | DQ-3. |
| Q-0117 | supports | An object comes into being for a union member only when that member becomes active. So a union with no active member has no member object for the check on parts to visit. The rule on equal template arguments also names two unions that both lack an active member. |
| Q-0124 | supports with a doubt that remains | DQ-4. |
| Q-0136 | supports with a doubt that remains | DQ-5. |
| Q-0137 | supports | The comparison of two standard sequences leaves the lvalue transformation out. What remains of each sequence is a conversion of a pointer to the derived class, which is the source that the pointer tie-breakers describe. |
| Q-0154 | supports with a doubt that remains | DQ-6. |
| Q-0155 | reads against, for the literal that does not fit | DQ-7. The UTF-8 case and the known bound are supported. |
| Q-0175 | supports with a doubt that remains | DQ-8. |
| Q-0187 | supports with a doubt that remains | DQ-9. |
| Q-0189 | reads against | DQ-10. |
| Q-0200 | silent | The reference gives each instantiation of an alias template its own closure type. It does not say whether two uses with the same arguments are one instantiation or two. Its account of an alias specialization as a substitution at each use fits Clang. Its treatment of one argument list as one specialization fits option 1. |
| Q-0220 | supports | The narrowing by partial ordering covers only the set of templates. Nothing gives a plain function a preference over a matching specialization, so a plain function and a specialization are two matches, and no deallocation function is called. |
| Q-0221 | supports | The reference says that a new expression names the single matching deallocation function of a class, whether or not the initialization can throw. A named function is used, and a use of a deleted function makes the program not valid. This settles the doubt of the 2026-11-04 entry. |
| Q-0222 | supports | The reference does describe deduction and partial ordering for this match. When more than one template is left, deduction fails, and only the declaration contexts make that failure an error. The new expression then has no single match, so no deallocation function is called and no error results. EDG's rejection reads against. |
| Q-0233 | supports | No storage class but `thread_local` and no attribute can belong to an explicit instantiation. The side question, whether a `thread_local` variable template must repeat the keyword, does not touch the tests, which all repeat it. |
| Q-0269 | reads against | DQ-11. |
| Q-0274 | reads against | DQ-12. |
| Q-0287 | supports | The normative steps substitute the whole return-type requirement after the expression and the `noexcept` test, so an unused extra argument is substituted and can fail. Only the example that equates the form with a nested requirement points the other way, and an example does not bind. |
| Q-0290 | supports | The parameter list of a requires expression is a function parameter clause, and the reference sends the reader to the function rules for it. Those rules give a lone unnamed parameter of a non-dependent `void` type the meaning of an empty list. |
| Q-0293 | supports with a doubt that remains | DQ-13. |
| Q-0295 | supports with a doubt that remains | DQ-14. |
| Q-0299 | supports with a doubt that remains | DQ-15. |
| Q-0304 | supports with a doubt that remains | DQ-16. |
| Q-0318 | supports with a doubt that remains | DQ-17. |
| Q-0326 | supports with a doubt that remains | DQ-18. |
| Q-0327 | supports with a doubt that remains | DQ-19. |
| Q-0345 | supports | A standard-layout class has all its data members first declared in one class, either itself or one base. So the order across a base and a derived class never arises for the classes where a common initial sequence exists, and the members of a base count as members of the derived class. |
| Q-0350 | supports with a doubt that remains | DQ-20. |
| Q-0359 | supports with a doubt that remains | DQ-21. |
| Q-0375 | supports with a doubt that remains | DQ-22. |
| Q-0378 | supports with a doubt that remains | DQ-23. |
| Q-0396 | supports with a doubt that remains | DQ-24. |
| Q-0402 | supports with a doubt that remains | DQ-25. |
| Q-0411 | supports | In C++23 a fold that a type constraint on a pack makes is itself one atom. Each declaration writes its own type constraint, so the two folds come from two separate places in the source and are not identical. The later change that EDG may follow is not part of C++23. |
| Q-0414 | supports with a doubt that remains | DQ-26. |
| Q-0415 | supports | The reference has a rule for exactly this case. In a friend declaration whose name is qualified, a plain name after the qualified name is searched first in the named scope and then by ordinary lookup. NAM-21 now states the parts after the qualified name in that way. |
| Q-0417 | supports | A deleted or private aligned form takes part in overload resolution and is chosen, and the checks for deletion and access come after the choice. So a function is found, the fallback search does not happen, and the program is not valid. The undefined phrase for a failed first search matters only when overload resolution finds nothing, which is not this case. |
| Q-0433 | supports with a doubt that remains | DQ-27. |
| Q-0434 | supports | A local class of a function template is not on the list of dependent types, so `this` in its member is not type-dependent. The general rule that a dependent part makes the whole expression dependent yields to the specific rule for a member access, which applies here. The other two doubts of RV7-B-11 concern cases that the tests do not hold. |
| Q-0445 | supports with a doubt that remains | DQ-28. |
| Q-0448 | supports with a doubt that remains | DQ-29. |

## DQ-1

Q-0001, floating overflow in a constant.

The reference makes an evaluation undefined when its result lies outside what the type can represent, and an undefined operation keeps an expression from being constant. The reference leaves the representation of each floating type to the implementation. So the doubt is now one question: whether an implementation whose floating types have an infinity counts the range of the type as reaching it. If it does, the exact result of an overflow lies inside the range and rounds, which is Clang's result. If it does not, the overflow is undefined, which is option 1. Nothing else in the reference decides it. Q-0028 takes the other side of the same question (DQ-2).

## DQ-2

Q-0028, a floating conversion above the finite range.

The reference gives three cases for a conversion between floating types: an exact value, a value between two neighbors of the target, and every other value, which is undefined. The answer holds only if an infinity counts as a neighbor. That is the same doubt as DQ-1, with the same silence. Q-0001 counts the overflow as outside the range, and Q-0028 counts an infinity as a value of the target. One reading of the doubt should serve both answers.

## DQ-3

Q-0032, a braced list after a compound assignment.

The grammar admits a braced list after every assignment operator. The reference then allows the list in two places: an assignment to a scalar, for which it gives a meaning only to the plain form, and an assignment to a class, where the list goes to the chosen operator function. The doubt is now only whether the scalar case covers the compound operators. If it does, the reference gives the form no meaning. Option 1 needs no such meaning, and nothing in the reference contradicts it.

## DQ-4

Q-0124, increment and decrement of a narrow signed operand.

The prefix form is defined by the compound assignment, which supports the answer. The postfix form says only that the operand gains one, and it points the reader to the rules of addition and assignment. The postfix text also expects an increment whose new value does not fit a bit-field, and gives that case an implementation-defined value. The doubt is now only whether the pointers to addition and assignment bring in the promotion and the conversion back for the postfix form.

## DQ-5

Q-0136, empty braces and a designated list to a reference to an array.

The designated list is settled. A designated list converts only to an aggregate that aggregate initialization accepts, and a designator cannot name an element of an array.

For empty braces and an array of unknown bound, the list rule asks for a sequence from each element, and there are none. The rule then takes the worst of those sequences, and the reference does not say what the worst of no sequences is. The reference also forbids empty braces as the initializer of an array of unknown bound, so the temporary could never be made. That fits option 1. A sequence can still exist for a call whose initialization later fails, so the doubt now rests only on the empty set of element sequences.

## DQ-6

Q-0154, the nearest-base tie-breaker for a braced list of one derived element.

For a class that is not an aggregate, the list rule gives the sequence only a rank, conversion rank for a derived element. The reference has three kinds of implicit sequence, and this one is neither user-defined nor an ellipsis, so it counts as a standard sequence, and the rules for comparing standard sequences apply to it. The doubt is now only whether a sequence that the reference describes by its rank alone counts as the conversion from the derived class to a base that the tie-breaker names.

## DQ-7

Q-0155, a braced string literal to a reference to a character array.

The list rule gives the identity when the single element is a string literal of a suitable kind for the array. The suitable kinds come from the rule on character arrays, which matches the literal's kind to the element type and allows a UTF-8 literal for a `char` array. The bound check sits in a separate sentence of that rule. So a literal too long for the array still has the identity. The pointer candidate has an array-to-pointer step, which the comparison leaves out, so the two sequences tie. That is Clang's result and reads against option 1 for `list-string-literal-too-short.cpp`. The UTF-8 case agrees with the answer. The known bound wins by the list tie-breaker for arrays of one element type, so the third case agrees too.

## DQ-8

Q-0175, a dynamic initializer that reads a member of a later class variable.

For a class with a non-trivial constructor, the reference makes any reference to a member before the constructor starts undefined. Its example gives a class with a trivial constructor as the case where the reference is fine. That settles the case of the probe, a class with a constructor, for option 1. The doubt is now only for a class whose default constructor is trivial and which gets a dynamic initializer. For that case the reference's note on static initialization lets an earlier initializer see the zero, and the lifetime rule points the other way.

## DQ-9

Q-0187, the object parameter in a call with no object.

The normative text gives the made-up object a class type and no value category. It says the call is not valid when a non-static member wins. The statement that the made-up object never makes a candidate viable or not viable sits in a footnote, and the footnote gives as its reason that all the members share one object parameter. With ref-qualifiers and explicit object parameters that reason no longer holds. The reference's own example of a static member function that calls a member with an explicit object parameter treats that member as chosen and the call as not valid, which fits option 1. The doubt is now that the support rests on a footnote whose stated reason is out of date.

## DQ-10

Q-0189, a constructor against a conversion function in the result tie-breaker.

The tie-breaker compares the standard conversion from the return type of each candidate to the type being initialized. A constructor has no return type, and the rule on copy initialization of a class from another class gives the constructor's result no stand-in type. Of the three contexts that the tie-breaker names, two hold only conversion functions. The third is initialization in general, which does include this case. So the text reads toward option 2: the tie-breaker cannot rank the constructor, and the two candidates are ambiguous, as Clang says. The doubt is the general context. A reader who takes the constructor's result as the target type reaches option 1, and the reference does not forbid that reading. The reference also does not state it.

## DQ-11

Q-0269, a placeholder parameter and a written-out template.

The reference turns each placeholder parameter into an invented type parameter. In the constrained form, the invented parameter carries the same type constraint as the written one. Two function template declarations correspond when their template parameter lists match in length and kind, their type constraints are equivalent, their parameter types are equivalent, and their template heads are equivalent. By those rules both pairs in the tests correspond:

- In `redeclare-placeholder-unconstrained.cpp`, each template has one unconstrained type parameter, and each function parameter is that parameter.
- In `redeclare-placeholder.cpp`, each template has one type parameter with the same constraint.

So the normative text makes the placeholder definition a definition of the earlier template, and the call is valid with one candidate. That is Clang's result. The only text for two templates is the example on abbreviated templates, which calls the two forms functionally equivalent and not equivalent. An example does not bind. CNS-27 and CNS-40 rest on that example. The rule that makes a program with two such templates not valid with no diagnostic required applies only if they are two templates in the first place.

## DQ-12

Q-0274, a name that finds an expanded using-declarator when the pack is empty.

The using-declarator in the test has a final name that depends on the pack. The reference counts a name that lookup finds through such a using-declarator as a dependent member of the class being defined. A member access whose name is dependent is type-dependent, so the call in `g` depends on the parameters. The rule on a hypothetical instantiation right after the definition covers only constructs that do not depend on the parameters, so it does not reach this call.

A note in the reference says a using-declarator is never dependent in a specialization and is replaced during the lookup for it. With an empty pack nothing replaces it, the search of the class finds nothing, and the lookup goes on to the namespace function. That is option 2, the result of GCC and EDG. The doubt is the rewrite of a class member name into a member access, which the definition's lookup triggers. The reference does not say whether that rewrite holds when the specialization's lookup finds no member. The reference also has a rule that a changed result of a second lookup is an ambiguity, and that rule names only qualified names.

## DQ-13

Q-0293, two templates that differ only in an ellipsis.

The constraint step gives up when the template parameter lists or the function parameter lists differ in length. The reference has no definition of a function parameter list. Elsewhere it speaks of a function with an ellipsis in its parameter list, which a reader could take as counting the ellipsis in the length. The rules on the number of parameters count only declared parameters. The doubt is now that one passage, against the plain sense of a length of parameters.

## DQ-14

Q-0295, the type of a constant argument for a partial specialization.

Deduction fails when the type of a constant differs from the type of the matching parameter of the template named. When the primary template's parameter has a placeholder type, the reference does not say whether that type means the placeholder as written or the type deduced for the given argument. The natural reading takes the deduced type, which gives option 1 for `partial-constant-type-exact.cpp`. For the `const auto*` form, the deduction of the placeholder from the argument gives a pointer to const, which differs from the argument's own pointer type. The doubt is now only the meaning of the type of a placeholder parameter in that rule.

## DQ-15

Q-0299, a declaration of `std::align_val_t` with another shape.

The reference makes undefined any declaration that a program adds to namespace `std` unless a rule allows it, and no rule allows this one. The implicit declarations of the allocation functions do not declare the type. So no shape needs a diagnostic, and the answer holds. The doubt is now only the class of the right shape. In the reference the right shape is undefined too. LSUP-5 and LSUP-P2 treat it as valid, which is Tidal's choice under decision 0041.

## DQ-16

Q-0304, a byte pointer that steps through an object.

The letter of the reference has not changed since the earlier reviews. A cast to a pointer to `unsigned char` still points to the object, because no byte object is pointer-interconvertible with it. Arithmetic needs a pointer into an array whose element type is similar to the pointer's type, so the step is undefined. The support for option 1 comes from the description of the object representation as a sequence of `unsigned char` objects and from the promise of byte copies. The doubt is the gap between that letter and that intent, and nothing in C++23 closes it.

## DQ-17

Q-0318, a member call through a null pointer in constant evaluation.

C++23 has no rule that names indirection through a null pointer, or a member call through one, as undefined. The arrow form is defined by indirection and then the dot form. Indirection gives the object that the pointer points to, and a null pointer points to none. The rule for a member access makes it undefined when the left side gives an object of an unrelated type, and that rule assumes an object exists. The doubt is now only that missing rule. The answer rests on the lack of any object for the access and the call.

## DQ-18

Q-0326, the size of a volatile access and merging.

The meaning of an access through a volatile glvalue is implementation-defined, and an implementation must still carry out volatile accesses as the abstract machine does. That supports option 1 for the size. The doubt is now only the open part of the answer, the merging of accesses. Two accesses of the abstract machine must both happen, but what counts as one access is the implementation's to define. For the VOL-12 initializer, the reference does not apply volatile semantics to an object while it is under construction.

## DQ-19

Q-0327, a byte pointer from a member that moves over the whole object.

The reasons of DQ-16 apply. The step from a member's bytes into the rest of the object has still less support, because the description of the object representation is per object, and a member's own representation covers only the member. The intent of byte copies of the whole object still needs such steps. The doubt is the same gap between the letter and the intent.

## DQ-20

Q-0350, Clang and a class size it cannot represent.

The 32-bit case is settled. The reference makes a type ill-formed when its object representation needs more bytes than `std::size_t` can count, and an ill-formed program needs a diagnostic. Clang's acceptance there is a defect.

The doubt is now only the 64-bit case of Clang 18. That class fits `std::size_t`. The reference asks an implementation to run a valid program correctly only within its resource limits. It asks the implementation to document those limits where they are known, and allows it to say that they are unknown. The implementation quantities do not decide conformance. So Clang 18 conforms if the class lies past one of its limits, and the reference requires no diagnostic there.

## DQ-21

Q-0359, a byte copy of a volatile member through plain bytes.

The reference makes undefined an access to an object defined with a volatile type through a glvalue that is not volatile, and it gives no exception for byte types. By the letter of DQ-16, a byte glvalue made by a cast still refers to the object, so the read is an access to the object itself. The doubt is now narrower. It is only whether a member declared volatile inside a class whose objects are not volatile counts as an object defined with a volatile type. The byte steps in the test also rest on the answer of Q-0304.

## DQ-22

Q-0375, a qualified definition of a member of an inline namespace.

The reference requires a declaration with a qualified name to be a friend declaration or to lie in a scope that contains the entity. The row in dispute, the definition inside the named namespace, is valid under any reading of that phrase, because the named namespace contains the inline namespace where the member is declared. So the answer's result for that row holds. The doubt is now only NS-15's own test, which excludes the declaring namespace itself. The reference has no such exclusion, and the row inside the inline namespace may be valid in the reference although every compiler rejects it. The reference does not define what it means for a scope to contain an entity.

## DQ-23

Q-0378, class operands of the conditional operator.

Form 1 is settled. The reference sets deleted functions aside when it asks whether a conversion sequence can be formed, so both operands convert, and the expression is not valid.

Form 2 has a narrower doubt. Neither reference binding works between a const and a volatile lvalue of one class. The case for one class type needs the target to carry every cv-qualifier of the source, and it fails both ways. The text then falls through to the last case, which converts to the other operand's type after the lvalue-to-rvalue step. A class keeps its cv-qualifiers there, and a difference in top-level qualifiers does not stop a sequence from being formed. Under that reading both sequences form, and the expression is not valid for that reason. Under the answer's reading neither forms, and the later step finds no candidate. Both readings reject the program, so the tests stand. The doubt is only which step rejects it.

## DQ-24

Q-0396, references inside the function types of ORD-9.

For the address of an overload set, the whole function type is the one type that takes part. The removal of references and qualifiers in partial ordering acts only on the top level of that type, and a function type is not a reference. That supports option 1 for both tests in Affects. In `address-reference-against-value.cpp`, the template with the reference parameter does not deduce from the invented function type with a value parameter, and the other direction succeeds. In `address-value-against-forwarding.cpp`, the reasoning is the same.

The doubt is now limited to `address-reference-kinds.cpp`, which is not in Affects. Option 1 gives that test's result only if a forwarding parameter inside a function type deduces from an invented lvalue reference. The reference gives that adjustment for call deduction and for top-level ordering. It does not give it for a part of a function type.

## DQ-25

Q-0402, the ordering class and a defaulted `<=>`.

The reference counts the deduction of an `auto` return type in a defaulted three-way comparison as a use of the comparison category type, and the library declaration must precede that use. It does not say where the use takes place. In the reference, the program of `ordering-declared-after-class.cpp` is not valid whatever the place. A declaration that the program writes in namespace `std` is undefined, and it does not count as the library declaration. So no library declaration ever precedes the use. The doubt about the place exists only under Tidal's model, in which a program can supply a library declaration (LSUP-P2, decision 0041). Within that model the reference gives no further guidance.

## DQ-26

Q-0414, a friend defined in a nested class.

A private member is accessible at a point that lies in a direct member or a friend of the class. Inner classes count among the members of the enclosing one and share the access that any member has. Each member of a class also gets every access that its class has. The friend function is a member of neither class and a friend only of the nested class, and the reference says friendship does not pass along. The doubt is now one phrase. The test holds only if the body of the friend, which lies within the nested class's definition, counts as lying in a direct member of the enclosing class. The reference does not say whether that phrase means the place in the text or the entity.

## DQ-27

Q-0433, the initializer of a `static const` member in a class template.

The instantiation of a class makes the declarations of its static data members and not their definitions. For a `static const` member of integral type that is not `inline` or `constexpr`, the declaration in the class is not a definition and holds the initializer. So the class makes the initializer, which is option 1. The doubt is now only one sentence. A member is made when a use needs its definition, and the initialization of a static data member, with its side effects, waits for such a use. That sentence is tied to the definition, and for this member the initializer sits in the declaration. GCC can still read the sentence as delaying the initializer, and the reference does not rule that reading out.

## DQ-28

Q-0445, an attribute after the class key outside a forward declaration.

An attribute after the class key is allowed only when the elaborated type is the whole declaration and has one of two forms: a class key with an identifier, or a class key with a template-id, the second only in an explicit specialization or a partial specialization. Every other elaborated type can carry no attribute. So the variable declaration of `elaborated-attribute-not-forward-declaration.cpp` and the explicit instantiation are not valid, and GCC reads against. The doubt is now only the qualified template-id form, such as the last declaration of `explicit-specialization-attributes.cpp`. The allowed forms have no qualifier. The grammar of an elaborated type gives an attribute no place before any template-id, qualified or not, so the reference is not consistent on that form. Every compiler accepts it.

## DQ-29

Q-0448, a union copy in constant evaluation.

For constant evaluation, the reference treats a trivial copy or move of a union as a copy of its active member, and that copy is trivial. It does not say whether a trivial copy of the active member copies its representation or each part. The doubt now touches only the acceptance test `union-copy-part-with-no-value.cpp`. If the copy went part by part, it would read the element or member with no value, and the copy itself would not be constant. Every compiler reads against that. The read test `union-copy-part-with-no-value-read.cpp` is rejected under either reading, at the copy or at the read, so it stands as written.
