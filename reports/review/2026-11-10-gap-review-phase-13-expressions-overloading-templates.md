# Gap review: phase 13 text on expressions, overloading, and templates (RV6, part B)

Date: 2026-11-10

Role: source reader

Reference: the C++23 working draft sources

Commit read: 98efc40f37190ddba31c5548cacfd49061c9a8f0

The review covers the rules and principles that the RV5 repair changed (LAM-5, LAM-6, LAM-7, ICS-11, ORD-13) and the text that item F3 changed: CAST-1, CNS-11, COND-16, DED-7, DED-8, DED-11, DED-14, DED-15, DED-24, DED-29, DED-30, DED-P4, DED-P5, the new example under DED-20, DEP-15, DEP-24, DEP-P4, DEP-P5, INST-1, the INST-11 Evidence, NEW-25, and NEW-29. It gives a first reading of Q-0426, Q-0427, and Q-0428. It reads again the RV5 points that the RV5 repair kept with an Evidence note: RV5-B-2 and RV5-B-3 (COND-16, COND-3), RV5-B-5 (NEW-22), and RV5-B-7 (ICS-17). Later defect resolutions are named only by subject.

## Result

Most of the text agrees with the reference. The RV5 repair closed the doubts on ORD-13 and on the capture terms of LAM-5 to LAM-7. ICS-11 now follows the reference on direct binding. One new item reads against the reference: the second paragraph of CNS-11, with Q-0426. DEP-15 and DEP-P4 read against it on a static constant member that equals the parameter, which the Evidence already records. DED-24 adds a condition for an enumeration in a qualified name. Q-0428 is supported. Q-0427 is supported with a doubt. Q-0426 reads against the C++23 text.

The four RV5 points kept with an Evidence note still stand. RV5-B-2 and RV5-B-3 are doubts in the reference, and no Tidal text can remove them. RV5-B-5 and RV5-B-7 still add a condition. RV5-B-1 (step 1 of COND-16, Q-0418) is unchanged and still reads against.

Gaps found in part B:

- CNS-11 skips the constraint check of function template candidates when a candidate that is not a template takes every argument with no conversion. The C++23 text checks the constraints of every template candidate during deduction. Its own example makes a call not valid for an error in that check, even though a plain function is the better match (RV6-B-1, Q-0426).
- DEP-15 and DEP-P4 say that a static constant member whose value is a constant parameter does not stand for the parameter. In the reference, such a member stands for the parameter when its type matches and its initializer is the parameter's bare name. The argument list then names the current instantiation (RV6-B-2).
- DED-24 lists a qualified name whose qualifier is not a class as a substitution failure. The reference accepts an enumeration there, so an enumeration with the named enumerator is valid (RV6-B-3).
- Step 1 of COND-16 still needs both operands to be lvalues (RV5-B-1, Q-0418).
- NEW-22 still limits the sized global form to types that are not over-aligned (RV5-B-5).
- ICS-17 still requires that a known bound is not less than the number of elements (RV5-B-7).

## Rules and principles

| Item | Verdict | Note |
|---|---|---|
| LAM-5 | agrees | The mode restriction after a default reaches only simple captures, and an init capture can use either mode. The page now uses one meaning of "explicit capture". See RV6-B-6. |
| LAM-6 | agrees with a doubt | The bullets match the reference. The doubt is the phrase "not inside a block". The reference looks at the innermost enclosing scope, and a lambda inside a block can still have a parameter scope as its innermost scope (RV6-B-5). |
| LAM-7 | agrees | An init capture at namespace scope is valid. The lifetime of a temporary bound by a reference init capture follows the closure object. |
| ICS-11 | agrees with a doubt | A non-class prvalue whose type is compatible with the referenced type binds directly, and the sequence is the identity, as RV5 asked. The doubt is the example: the C++23 ranking rules cannot tell the two references apart. Also the glossary and REF-2 still describe this binding as one through a temporary (RV6-B-4). |
| ORD-13 | agrees | The swapped case now names the conditions it replaces. RV5-B-12 is closed. |
| CAST-1 | agrees | The round trip through a pointer to void gives a pointer to the original object, and C++23 constant evaluation does not allow the conversion from a pointer to void to a pointer to an object type. |
| CNS-11 | reads against | The first paragraph agrees. The second paragraph has no basis in C++23, and the reference's own example reads against it (RV6-B-1). |
| COND-16 | reads against | The new sentence on bases agrees: the reference asks whether the reference would bind directly, and that test counts an ambiguous or inaccessible base as compatible and then makes the program not valid. Step 1 is unchanged and still reads against (RV5-B-1). The table doubts stay (RV6-B-11). |
| DED-7 | agrees | A cv-qualifier added to a function type through a template parameter is ignored, so the parameter takes the function type. |
| DED-8 | agrees with a doubt | A non-class prvalue loses its top-level cv-qualifiers before deduction, and a class prvalue keeps them. The reference also excludes array prvalues from the loss, and the sentence says "not a class" (RV6-B-8). |
| DED-11 | agrees | A value deduced from an array bound has the size type, and the value in a noexcept specifier has type bool. |
| DED-14 | agrees | The near step runs only when no exact values exist. The pair with two levels of pointer fails because the needed qualification step is not a valid conversion. |
| DED-15 | agrees | Two base subobjects of one specialization give one deduced type, so deduction succeeds and the conversion then fails. A base derived from another matching base wins. Deduction asks only for the class relation. |
| DED-20 example | agrees with a doubt | The explicit elements fill the pack, and the trailing parameter is deduced. The call with only one argument relies on the pack being empty. The reference states that result only for a pack that ends the template parameter list (RV6-B-7). |
| DED-24 | adds a condition | The new array forms agree. The qualified-name bullet treats every non-class qualifier as a failure, and the reference also accepts an enumeration. The incomplete class case agrees with a doubt (RV6-B-3). |
| DED-29 | agrees with a doubt | The lists now name the body of another specialization that the function type needs, which matches the reference. The RV5-B-10 doubt on losing call candidates stays (RV6-B-15). |
| DED-30 | agrees | A placeholder result type deduces nothing, and the body of a successful match is instantiated to find the type. The forwarding reference rule for an lvalue reference target matches. |
| DED-P4 | agrees | The new table and the base paragraph follow DED-14 and DED-15. |
| DED-P5 | agrees | The row on another function template's body matches DED-29. |
| DEP-15 | reads against | The new sentence on an argument list with no dependent argument agrees. The older sentence on a static constant member reads against, and the Evidence records it (RV6-B-2). |
| DEP-24 | agrees with a doubt | The new exception for an object of the current instantiation agrees. The doubt is the phrase "in the class" (RV6-B-9). |
| DEP-P4 | reads against | The same point as DEP-15, in the table row for a constant parameter (RV6-B-2). |
| DEP-P5 | agrees with a doubt | The new row matches DEP-24, with the same doubt on where lookup finds the member template (RV6-B-9). |
| INST-1 | agrees | A qualified name that looks into a class specialization needs the class's members, so the use instantiates it. |
| INST-11 Evidence | agrees | The list records compiler behavior inside a choice that the reference leaves unspecified. A call that names the function is a separate case (RV6-B-10). |
| NEW-25 | agrees | The selected allocation function goes through the usual checks for deletion and access after overload resolution. |
| NEW-29 | agrees | Unchanged in substance. The other direction stays with NEW-P5 and its Evidence note. |

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0426 | reads against | In C++23, deduction for a function template candidate checks the constraints once all arguments are known, and a failure there that is not a plain false result makes the program not valid. Nothing lets the implementation skip a template candidate because a plain function matches exactly. The reference gives an example where the check is an error and the plain function would win, and it calls the program not valid. So the W20-3 copy checks the constraint for the class itself (option 2), as Clang 18 and EDG do (RV6-B-1). |
| Q-0427 | supports with a doubt | The name after the dot is looked up in the type of the object. The lookup runs at the definition when that type is the current instantiation. A found member template makes the following angle bracket open an argument list, so no keyword is needed. The doubt is the phrase "in the class" (RV6-B-9). |
| Q-0428 | supports | A call through the pointer names the virtual function and uses it. A use of that kind needs the definition, so the member is instantiated. The unspecified case covers only a virtual function that would not otherwise be instantiated, so EDG's result is outside it (RV6-B-10). |

## RV6-B-1

CNS-11, second paragraph, CPY-15, and Q-0426. In C++23, a function template becomes a candidate through deduction. Once every template argument is deduced or comes from a default, the template parameter list is substituted, and then the associated constraints are checked. Unsatisfied constraints make deduction fail. An error during the check itself is a hard error. The text gives no step that drops template candidates before deduction because a non-template candidate already matches exactly. The reference even includes an example of the opposite: a template whose constraint check is an error beside a plain function that is an exact match, and the call is not valid. So CNS-11's skip, and its definition of a parameter that takes the argument with no conversion, have no basis in C++23. Clang 22 and GCC follow a shortcut that, as far as the reviewer knows, a later proposal would allow. The reviewer does not know whether it was adopted. CPY-15's added sentence says that the constraints of a by-value constructor template are checked before its specialization is dropped. The reference says only that such a specialization is never produced, and does not say when. Since the check is part of deduction and comes before substitution into the function type, the order in CPY-15 is a fair reading. For the recursive constraint in Q-0426, C++23 has no rule that names a satisfaction check that depends on its own result, so the reference gives no direct answer to the Clang 18 rejection beyond the check itself.

## RV6-B-2

DEP-15 and DEP-P4. In the reference, a name of a variable can take the place of a constant parameter in an argument list that names the current instantiation. The variable must have the parameter's type, cv-qualifiers aside, and must be initialized from the parameter's name alone, directly or through a chain of such variables. Parentheses around the initializer, a different type, or any other initializer end the substitution. So a static constant member of the same type, initialized with the bare parameter name, names the current instantiation. DEP-15 says it never counts, and the DEP-P4 table lists it under another specialization. The Evidence records that every compiler reads it as another specialization, also with parentheses. The text follows the compilers. The parenthesized form agrees with the reference. The plain form reads against it.

## RV6-B-3

DED-24, the qualified-name bullet. The reference makes a qualifier a substitution failure when it is neither a class nor an enumeration. An enumeration qualifier is valid when the named member exists and has the needed kind, such as an enumerator where a value is needed. DED-24 says that a qualifier that is not a class fails, which covers enumerations too. The reference also lists a member of the wrong kind: a value where a type is needed, a non-template where a template is needed, and a type where a value is needed. DED-24's "has no such member" may be meant to cover those, but it does not say so. The incomplete class case has no item of its own in the reference. A search in an incomplete class finds no member, so the case falls under the missing member. The test and the compilers agree with that reading, which leaves a small doubt only.

## RV6-B-4

ICS-11 and its example. The reference treats the binding of a const or rvalue reference to a compatible non-class prvalue as direct. The prvalue first takes the cv-qualifiers of the referenced type and is then materialized. So the identity is right for both references in the example. In C++23 the ranking cannot then prefer one of the two. Both sequences are the identity. The rule on reference bindings that differ only in top-level cv-qualifiers does not apply, because the two referenced pointer types differ below the top level. No other rule separates them. The compilers prefer the less qualified one, and a later defect resolution on reference binding between similar types gives that answer, as far as the reviewer knows. The example therefore depends on ICS-46, which goes past the C++23 text. A second, smaller doubt concerns the terms. The glossary defines a direct binding as one with no temporary in between. REF-2 says that in the cases other than an xvalue of the same type or a class rvalue, the reference binds to a temporary that holds a copy. ICS-11 now calls the non-class prvalue case direct, while it still materializes a temporary. The three texts use the term differently.

## RV6-B-5

LAM-6, the bullet on lambdas outside a block. The reference allows a capture default or a simple capture only when the innermost enclosing scope is a block scope, or in a default member initializer whose innermost scope is the class scope. A lambda can lie inside a function body and still have a different innermost scope. Examples are the parameter list or the trailing result type of an enclosing lambda, the template parameter list of a generic lambda, and the body of a requires expression that declares parameters. The parameter scope of such a requires expression reaches to the end of its body. LAM-6 names default arguments and local class members only, so the text permits these other cases and the reference does not. The cases are rare, and the compilers may not follow the reference in them. A probe of a capture default inside the body of a requires expression with parameters would show the compilers' answer.

## RV6-B-6

LAM-5 and LAM-7, and a correction to RV5-B-14. RV5-B-14 said that only a simple capture explicitly captures an entity. That was not exact. The reference treats an init capture as if it declares a variable and explicitly captures it. The rules on modes after a default are stated on simple captures, though, so an init capture can still use either mode after either default. LAM-5 now defines its use of "explicit capture" as the simple captures, and LAM-6 and LAM-7 say the same thing about init captures. The outcome agrees with the reference, and the correction changes no Tidal text.

## RV6-B-7

The new example under DED-20. With explicit elements for the pack, the pack takes those elements and is not extended, because a function parameter pack before the end is never deduced. The parameter after the pack is then deduced from the remaining argument. That part agrees. For the call with one argument, the pack gets no explicit elements and is never deduced. The reference states that a pack that is not otherwise deduced becomes empty, but only for a pack that ends the template parameter list, and only in a note. Here a type parameter follows the pack. Read literally, the pack is neither deduced nor given, and deduction fails. The compilers accept the call with an empty pack, and DED-20 says so. The doubt is in the reference.

## RV6-B-8

DED-8. The reference drops the top-level cv-qualifiers of a prvalue whose type is neither a class nor an array. An array prvalue keeps the cv-qualifiers of its elements. DED-8 and VC-2 say "not a class". An array prvalue is rare, but it can arise from a braced list of an array alias. Deduction for a forwarding reference would then keep the const elements in the deduced type.

## RV6-B-9

DEP-24, DEP-P5, and Q-0427. In the reference, the name after the dot or arrow is looked up in the type of the object. The lookup waits when that type depends on a parameter and differs from the current instantiation. If it is the current instantiation, the lookup runs at the definition. When the lookup finds a template, the angle bracket opens an argument list. That lookup also searches the bases of the class that are not dependent. DEP-24 and the DEP-P5 row say that the class declares or contains the member template. A member template that a non-dependent base declares is also found, and the keyword is then not needed either. The bullet for the scope operator has a related gap. A qualifier that names the current instantiation, where lookup finds nothing because the member comes from a dependent base, is treated like a dependent qualifier that is not the current instantiation. DEP-24's first bullet leaves that case out. The test for a member template from a dependent base covers only the dot form.

## RV6-B-10

Q-0428 and the INST-11 Evidence. The reference instantiates a member of a class specialization when the program refers to it in a context that needs its definition. A call through a pointer names the virtual function and uses it, and such a use needs a definition. The reference also counts every virtual function that is not pure as used, which is why the unspecified rule applies only when no other need for the function exists. The call in Q-0428 creates that need, so the function is instantiated, as Clang and GCC do. The call does not stop being a use because it is in a function that the program never calls. The list in the INST-11 Evidence records compiler behavior in the unspecified case and makes no claim about the reference.

## RV6-B-11

COND-16. The new sentence on bases agrees. Steps 1 and 2 ask whether a reference would bind directly. That test uses compatibility through a pointer conversion. The base relation alone decides it, and the reference then makes the program not valid when the conversion is ambiguous or inaccessible. The note on the operator also says that access is ignored when the sequences are formed. The friend case in the Evidence fits that reading. Step 1 is unchanged. It still needs both operands to be lvalues, and the reference needs only the other operand to be an lvalue and a direct binding to a glvalue (RV5-B-1, Q-0418).

## RV6-B-12

RV5-B-2 and RV5-B-3 (COND-16, COND-3). Both still stand. In the C++23 text, a class operand against the same class with fewer cv-qualifiers falls to the last case of the class step. A base class operand against its derived class falls there too. A sequence can then often be formed, which the table does not allow. For a derived prvalue against a const base lvalue, the reference converts the derived operand to an unqualified base prvalue and leaves two operands that differ only in cv-qualifiers. The Evidence note now records that the table follows the compilers and the later reading. The doubt is in the reference, and the note answers it. Neither point needs a further repair.

## RV6-B-13

RV5-B-5 (NEW-22). The point still adds a condition. For a global deallocation function, the reference keeps the aligned forms for an over-aligned type, and then picks the sized form for any single object of complete type. NEW-22 and UNW-3 still give the sized form only to types that are not over-aligned, and they leave the over-aligned case to the implementation. The Evidence note records the review's reading. Unlike the NEW-P5 note, it gives no compiler result for the choice. A probe with a global sized aligned form that reports its call would show whether the compilers follow the reference.

## RV6-B-14

RV5-B-7 (ICS-17). The point still adds a condition. The reference forms the sequence from each element and from empty braces for the elements past the list. It says nothing about a list longer than a known bound. The Evidence note gives the probe in which every baseline compiler selects the larger bound. That supports the condition as the compilers' rule. The doubt remains in the reference, and no repair is needed.

## RV6-B-15

DED-29. The new wording on another specialization that the function type needs agrees with the reference, which says that any use of a specialization with a deduced result type causes its instantiation, and that errors there are hard errors. The RV5-B-10 doubt stays: the reference does not say whether being a call candidate is such a use. DED-29's paragraph on candidates that lose remains a fair reading.
