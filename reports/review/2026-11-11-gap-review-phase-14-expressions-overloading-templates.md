# Gap review: phase 14 text on expressions, overloading, and templates (RV7, part B)

Date: 2026-11-11

Role: source reader

Reference: the C++23 working draft sources

Commit read: 6afa79f9f5b92813bcfec545d26220bce5572c47

The review covers the rules and principles on these pages that changed after RV6 read the tree (NEW-22, DED-8, DED-24, DEP-P5, DEP-24). It reads every hunk of the RV6 repair commit 9c009448 on the expressions, overloading, and templates pages, including the Evidence notes. It reads the text that phase 14 changed on these pages in commits d0597264 and 1130ceb8: ADD-14, DEP-2, DEP-4, DEP-5, DEP-6, DEP-9, DEP-12, DEP-20, DEP-30, DEP-37, DEP-40, DEP-P1, DEP-P2, DEP-P5, DEP-P6, INST-1, INST-2, INST-5, INST-7, INST-33, INST-P1, INST-P2, and the new examples. It gives a first reading of Q-0429, Q-0432, Q-0433, and Q-0434, and reads Q-0427 again. Later defect resolutions are named only by subject.

## Result

Most of the text agrees with the reference. The RV6 repairs close RV6-B-3, RV6-B-8, and RV6-B-9, and Q-0427 is now supported with no doubt. Q-0429 and Q-0432 are supported. Q-0433 and Q-0434 are supported with a doubt.

Three items read against the reference:

- The new DEP-P5 row for a deduced class type after the keyword template. The C++23 grammar has no place for the keyword there, and the qualifier of a deduced class type must not be dependent (RV7-B-1). Q-0226 already records the dispute.
- The DEP-P6 row for CNS-13, which now says "rejects". The reference's own example calls that case ill-formed with no diagnostic required (RV7-B-2).
- The new paragraph of ADD-14. In the reference, any arithmetic on a pointer whose pointee type is not similar to the array's element type is undefined, so the move gives no address (RV7-B-3). The reading bears on Q-0430, which part A of the review covers.

Four items add a condition:

- NEW-22 and its Evidence note limit the sized aligned global form to the path after delete. The reference uses the same selection after a constructor throws (RV7-B-4).
- DEP-6 and the new DEP-30 bullet say that every independent name that lookup does not find is rejected. The reference gives that answer for an unqualified name only. It gives a qualified name into the current instantiation that finds nothing as a case that may be diagnosed, which DEP-17 also says (RV7-B-5).
- INST-1, INST-2, and INST-P1 exempt a call "inside" decltype. The reference exempts only a call that is the whole operand and is not an immediate invocation (RV7-B-6).
- DEP-40 states its lookup at the definition for any qualified call. The rule holds only when the qualifier is not dependent (RV7-B-7).

The other points are doubts. The recorded read of the RV6 repair diff on these pages finds that every hunk agrees with the reference, except the throw path in the NEW-22 rule and note (RV7-B-4).

## RV6 repair diff, commit 9c009448

The reviewer read the whole diff of 9c009448 on the expressions, overloading, and templates pages: lambdas, new-delete, conversion-sequences, constraints, deduction, and dependent-names. The commit's other files are outside part B.

| Hunk | Verdict | Note |
|---|---|---|
| lambdas, the LAM-6 note | agrees | The note gives the RV6-B-5 reading and the probe results. It makes no claim beyond them. |
| new-delete, NEW-22 rule | adds a condition | After delete, a complete single object with more alignment gets the sized aligned form, as the reference says. The reference reaches the same answer after a throw, and the rule leaves that path open (RV7-B-4). |
| new-delete, NEW-28 tests line, Evidence count, table row | agrees | Counts and compiler results only. |
| new-delete, the NEW-22 Evidence note | adds a condition | The delete half agrees and closes RV6-B-13 for that path. The throw half says that UNW-3 allows either aligned form, and the reference picks the sized one (RV7-B-4). |
| conversion-sequences, the ICS-11 and ICS-46 note | agrees | The note gives the RV6-B-4 reading and the compiler result, and says that ICS-46 follows the compilers. |
| constraints, the CNS-11 note | agrees | The note says that the reviews read C++23 as EDG and Clang 18 do. That matches RV6-B-1. |
| deduction, DED-8 | agrees | An array prvalue keeps the cv-qualifiers of its elements. RV6-B-8 is closed. |
| deduction, DED-24 | agrees with a doubt | An enumeration qualifier is now valid, and a member of the wrong kind fails. The doubt is the template case (RV7-B-8). |
| deduction, Evidence count | agrees | Counts only. |
| dependent-names, the DEP-P5 row for an object of the current instantiation | agrees | A base that is not dependent is searched, so its member template needs no keyword. |
| dependent-names, DEP-24 | agrees | Both bullets now match the reference (RV7-B-9). |
| dependent-names, Evidence count | agrees | Counts only. |
| dependent-names, the DEP-15 note | agrees | The note gives the RV6-B-2 reading: a member of the parameter's type whose initializer is the bare parameter name stands for the parameter, and the parenthesized form does not. |

## Rules and principles

| Item | Verdict | Note |
|---|---|---|
| NEW-22 | adds a condition | See the table above and RV7-B-4. |
| DED-8 | agrees | Closes RV6-B-8. |
| DED-24 | agrees with a doubt | RV7-B-8. |
| DEP-24 | agrees | RV7-B-9. |
| DEP-P5 | reads against | The row for an object of the current instantiation agrees. The new row for the keyword template before a deduced class type reads against (RV7-B-1). The new sentence on the middle column agrees with a doubt, because the row for Q-0225 puts a rejection in that column (RV7-B-10). |
| ADD-14 | reads against | The new paragraph gives an address to a move that the reference leaves undefined (RV7-B-3). |
| DEP-2 | agrees with a doubt | The exception for a member access through the object of the current instantiation follows the reference. The doubts concern local classes, the names that are always dependent, and members of a base that is not dependent (RV7-B-11). |
| DEP-4 | agrees | The operand of sizeof, alignof, and noexcept makes the result value-dependent only through a dependent type or a type-dependent operand. The reference treats typeid the same way, and DEP-4 does not name it (RV7-B-12). |
| DEP-5 | agrees | A call with no dependent argument is bound at the definition, with both kinds of lookup. |
| DEP-6 | adds a condition | RV7-B-5. |
| DEP-9 | agrees | The link to INST-32 matches: a specialization of a function can have several points, and points that differ in meaning make the program not valid with no diagnostic required. |
| DEP-12 | agrees | An unqualified name that only a dependent base declares finds nothing at the definition. The reference treats such a name as any undeclared name. |
| DEP-20 | agrees | The return type of a member function, and of a friend declared in the class, is a position that allows only a type. |
| DEP-30 | adds a condition | The new bullet relies on DEP-6 and has the same gap (RV7-B-5). |
| DEP-37 | agrees | The link to OPR-2 adds no claim. The reference's own example rejects such an operator at the definition. |
| DEP-40 | adds a condition | RV7-B-7. |
| DEP-P1 | agrees with a doubt | The table rows agree. The word "Only" in the lead sentence leaves out the other dependent calls of DEP-8 (RV7-B-13). |
| DEP-P2 | agrees | The new sentence matches DEP-4. |
| DEP-P6 | reads against | The rows for DEP-6 and DEP-12 agree, with the RV7-B-5 condition. The CNS-13 row reads against (RV7-B-2). |
| INST-1 | adds a condition | A call that passes the class by value needs a complete class, which agrees. The decltype exemption is too wide (RV7-B-6). |
| INST-2 | adds a condition | RV7-B-6. |
| INST-5 | agrees with a doubt | The bodies of friend functions wait, and the initializers of inline and constexpr static members wait. The in-class initializer of a non-inline const static member is made with the class. That reading has a doubt (RV7-B-14). |
| INST-7 | agrees | A friend function defined in a class specialization is made when a use needs its definition. |
| INST-33 | agrees | A class has one point. A static data member that waits has the points of a function. |
| INST-P1 | adds a condition | The decltype cell has the same width as INST-2 (RV7-B-6). |
| INST-P2 | agrees with a doubt | The new rows match INST-5 and INST-7, with the RV7-B-14 doubt. |

The text of DEP-P1, DEP-5, and DEP-9 and the Evidence notes for DEP-4, DEP-5, DEP-6, DEP-9, DEP-12, DEP-40, CNS-13, INST-1, INST-2, INST-5, and INST-7 record compiler results. They agree with the reference except where the points above say otherwise. For the DEP-4 note on alignof of a decltype, the reference sides with GCC. A constant parameter of a fixed type is not type-dependent, so the decltype of it is not a dependent type.

## Examples

| Example | Verdict | Note |
|---|---|---|
| DED-29 reject example | agrees | The return type needs the class to be complete. The failed assertion arises while the class is made, outside the substitution itself, so the error is hard and the fallback does not save the call. |
| INST-33 reject example, changed annotation | agrees with a doubt | The read at namespace scope sets a point before the later declaration, and the end of the unit is another point. Points that differ make the program not valid with no diagnostic required. Every compiler rejects, so the annotation follows the compilers (RV7-B-15). |
| Instantiation diverges example | agrees | Making the class makes only the declaration of the constexpr member. The read after the later declaration makes the initializer, and lookup there finds the function. |
| PRIM-9 example | agrees | Whether two equal literals give one object is unspecified. Equality of the two pointers compares addresses. |
| Multiplicative example | agrees with a doubt | Unsigned arithmetic wraps, and the conversion back to a signed type keeps the value modulo the width. The comment names 32 bits, which holds only where int has 32 bits. |
| New-delete examples | agrees | The array owner pairs array new with array delete and removes its copies. The counted handle adds before it releases, so assignment to itself keeps the object. The arrow operator chains through the returned pointer. |

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0427 | supports | The name after the dot is looked up in the class of the object at the definition when that class is the current instantiation. The lookup searches the bases that are not dependent. A found template opens an argument list. DEP-24 now says so, and the RV6-B-9 doubt is closed (RV7-B-9). |
| Q-0429 | supports | Making the class makes the declaration of the member template with the class's pack known. A function parameter pack whose pattern uses only the class's pack becomes one ordinary parameter per element. Those parameters hold no parameter of the member template, so deduction does not use them, and DED-20 does not apply (option 1, RV7-B-16). |
| Q-0432 | supports | In the reference, these operators give a value-dependent result only for a dependent type or a type-dependent operand. A value-dependent operand alone is not enough (option 1, RV7-B-12). |
| Q-0433 | supports with a doubt | The declaration in the class of a non-inline const static member carries the initializer, and making the class makes the declarations of static members. So the initializer is made with the class (option 1). The doubt is a general sentence on static members that delays their initialization until a use (RV7-B-14). |
| Q-0434 | supports with a doubt | A member access is type-dependent through the member's dependent type or a dependent name, and a name into a class with a dependent base that finds nothing is dependent. The list of dependent types does not name a local class, so this in a local class of a function template is not type-dependent (option 1). The doubts are in RV7-B-11. |

## RV7-B-1

DEP-P5, the new row for a member class template after the keyword template with no argument list, as the type of a declaration with deduced arguments. In the C++23 grammar, a type name that stands for a deduced class type is an optional typename keyword, an optional qualifier, and the template's name. The keyword template has no place in that form. The reference also requires that the qualifier of a deduced class type is not dependent, and a template parameter as qualifier is dependent. The form after typename that uses the keyword template must carry an argument list. So the reference rejects the row's form on two grounds. The row cites Q-0226, and that question's Opinions already record a cluster review with the same reading. The row is new text that states the form as valid, so it carries the dispute into the principle.

## RV7-B-2

DEP-P6, the CNS-13 row, and CNS-13's first bullet. The reference requires that a template-id whose arguments are all independent satisfies the constraints of its template. Its own example puts such a template-id inside a class template that is never made, and calls that ill-formed with no diagnostic required. It gives the same answer for a template template parameter used with an independent argument. The old row said "need not diagnose" and added that every baseline compiler rejects, which matched. The new row says "rejects", and CNS-13 says that the rule holds inside a template that is never made. Both read against the reference's example. The compilers' result can stay as Evidence.

## RV7-B-3

ADD-14, the new paragraph, and Q-0430. In the reference, adding to or subtracting from a pointer is undefined when the pointed-to type and the element type of the array are not similar. The reference's example uses a pointer to unsigned int that points into an array of int, and makes a move by one undefined. A pointer to some type that points to a byte of a byte array meets the same condition. So the move by one is undefined, and the reference gives it no address. The paragraph says the move lands one size of the pointer's type further on. The same reference rule also reads against ADD-14's older sentence that counts the object as an array of one for a pointer of another type, since that sentence allows the move to one past. That sentence predates this phase and has its own question (Q-0049). The allocated-storage case of STG-28 differs, because creating objects inside allocated storage can supply an array of the pointer's type. A declared byte array gets no new pointer value from a cast.

## RV7-B-4

NEW-22, its Evidence note, and UNW-3. For a delete of a single object, the reference keeps the aligned forms for a type with more alignment than the default. Then, with global functions and a complete type, it picks the form with the size. NEW-22 now says so after delete. For a new expression whose constructor throws, and with no placement arguments, the reference finds the deallocation function by the same procedure as a delete expression. So after a throw the reference also picks the sized aligned form for a complete single object. NEW-22 and the note leave that path to UNW-3, which allows either form. The note records that every compiler calls the form without the size after a throw. The text follows the compilers there, and the reference reads otherwise. RV6-B-13 is closed for the delete path only.

## RV7-B-5

DEP-6, DEP-30's new first bullet, and DEP-17. The reference's examples give two answers for a name in a template that lookup at the definition does not find. An undeclared unqualified name is an error at the definition. A qualified name into the current instantiation, in a class with no dependent base, that finds nothing may be diagnosed before any instantiation, which is the no-diagnostic case. DEP-6 says that any independent name that is not declared must be rejected, also with no specialization made. DEP-30's new bullet says the same through DEP-6. DEP-17 says the opposite for the qualified case and the member access through this, and points to DEP-30. So DEP-6 and the new bullet need a limit to names outside the current instantiation, or DEP-17's case conflicts with them. The tests for this-member-missing reject on newer compilers, and Clang 18 and GCC 14 accept, which fits the no-diagnostic reading.

## RV7-B-6

INST-1, INST-2, and the INST-P1 class row. In the reference, when the operand of decltype is itself a prvalue, no temporary is made, and the prvalue's class type can be incomplete. That is why a call that returns the class does not make the class. The exemption covers only the case where the call is the whole operand, and not a call that is an immediate invocation. A call nested deeper, such as an argument passed by value to another call or the object of a member access, still needs a complete class, even inside decltype. INST-2 says "inside", INST-1 says "outside", and INST-P1 says "of a call". INST-P1's phrase is close. The other two are wider than the reference.

## RV7-B-7

DEP-40. In the reference, a qualified name waits for each specialization in two cases. Its qualifier can be a dependent type other than the current instantiation. Or the qualifier can be the current instantiation, with a dependent base, and lookup at the definition finds nothing. DEP-40 says that a qualified call looks up its name at the definition, in the scope that the qualifier names, and so finds no later function. That holds for a qualifier that is a namespace or an independent class. It does not hold for a dependent qualifier, which DEP-19 and DEP-22 already treat as looked up at the instantiation. The DEP-P1 table row uses a namespace qualifier, which fits. The part on an unqualified name that finds a class member agrees. The reference suppresses argument-dependent lookup when ordinary lookup finds a class member.

## RV7-B-8

DED-24, the wrong-kind bullet. The reference also fails a member of the wrong kind in a position that needs a template. Its example uses a qualified name as an argument for a template template parameter, with no keyword. DED-24 names only the case after the keyword template. The test for the wrong kind may or may not hold the other position. The doubt is small. The enumeration bullet agrees, and RV6-B-3 is closed.

## RV7-B-9

DEP-24 and Q-0427. After the dot or arrow, the reference looks up the name in the class of the object. When that class is the current instantiation, the lookup runs at the definition and covers the bases that are not dependent. A found template makes the next angle bracket open an argument list. When the class has a dependent base and the lookup finds nothing, the name waits, and the angle bracket is the less-than operator. After the scope operator with the current instantiation, a name that finds nothing because of a dependent base also needs the keyword. DEP-24's two bullets now say all of this. RV6-B-9 is closed. A base that is the current instantiation of an enclosing class also counts as searched, as DEP-11 says. DEP-24's phrase "a base that is not dependent" covers it, since such a base is not a dependent base in the reference's sense.

## RV7-B-10

DEP-P5, the new sentence after the table. The sentence says that the middle column is the parser's reading and that the choice reports no error. The row for Q-0225 has "not valid without template" in that column, which is an error. In C++23, a name in a position that allows only a type, other than the last name before a scope operator, opens an argument list at the angle bracket with no keyword. That is the older reading of Q-0225, which waits on the maintainer. The new sentence and that row now say different things about the middle column, whichever reading is chosen.

## RV7-B-11

DEP-2 and Q-0434. In the reference, this is type-dependent when the class around it is a dependent type. The list of dependent types names a template parameter, a dependent qualified name, a nested class of the current instantiation, and built types. It names no local class. So this in a member of a local class of a function template is not type-dependent, which agrees with the new bullet. Three doubts remain.

- The same reading also covers a local class inside a member function of a class template. DEP-2 excludes only a local class of a function template, and "a member of a class template" could be read to include the other case.
- A member access is also type-dependent when its name is dependent for another reason. Two examples are a conversion function name whose target type is dependent, and the copy assignment operator named through the current instantiation. The new exception names only a dependent member type and a failed lookup with a dependent base.
- The reference states the member-access case as a sufficient condition. It reads as a full condition only when taken as the exception to the rule on subexpressions. Every newest compiler takes it that way, and so does DEP-2. A member that lookup finds in a base that is not dependent is a further gray case in the reference's wording, and the compilers treat it as not type-dependent.

## RV7-B-12

DEP-4 and Q-0432. The reference gives sizeof of an expression, sizeof of a type, alignof, noexcept, and typeid a value-dependent result only when the type is dependent or the operand is type-dependent. These forms are taken out of the general rule on operands. DEP-4 now says so for three of the operators. Typeid is the fourth form, and it can appear in a constant expression through the address of its result. DEP-4's last bullet still lets a value-dependent operand of typeid make the result value-dependent. The gap is small.

## RV7-B-13

DEP-P1, the lead sentence. In the reference, a call is dependent when an argument is type-dependent, when an argument is a pack expansion, or when the name carries dependent template arguments. Operator expressions with a type-dependent operand follow the same two lookups. "Only an unqualified call with a dependent argument" leaves out the template-argument case, which DEP-8 lists. The table rows agree.

## RV7-B-14

INST-5, INST-P2, and Q-0433. The reference makes the declarations of static data members with the class, and not their definitions. For an inline or constexpr static member, the declaration in the class is a definition, so its initializer waits. For a non-inline const static member of integer or enumeration type, the declaration in the class is not a definition and carries the initializer, so the initializer comes with the declaration. That supports Clang and EDG. The doubt is a general sentence that delays the initialization of a static data member, with its side effects, until a use needs the member's definition. GCC can read that sentence as delaying the initializer itself. The reference's wording favors option 1, since it ties the initializer to the declaration, and the delayed initialization concerns run-time effects.

## RV7-B-15

The INST-33 reject example. The read of the member in a namespace-scope initializer needs the initializer for constant evaluation, so the member is made with a point right after that declaration. The end of the unit is another point. At the first point the dependent call finds no function. At the second it does. The reference makes points with different meanings ill-formed with no diagnostic required, which INST-32 says. The example marks the program rejected and names INST-32, which is a no-diagnostic rule. The compilers all reject. The doubt is only the word "rejected" beside a rule that does not require it.

## RV7-B-16

Q-0429, with DED-20, INST-5, and PACK-1. In the reference, making a class makes the declarations of its member templates with the class's arguments in place. A pack expansion whose pattern holds only the class's pack is expanded then. The reference says that each element of an expanded function parameter pack names one of the parameters made from the pack's declaration. Deduction compares only the parameter types that hold template parameters of the template being deduced. The parameters made from the class's pack hold none. So the rule on a pack before the end of the parameters does not reach them, and deduction finds only the member template's own parameter. The reference does not state this case in one place. The reading follows from those rules, and every compiler agrees.
