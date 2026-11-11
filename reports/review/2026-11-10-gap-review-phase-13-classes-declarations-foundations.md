# Gap review: phase 13 text on classes, declarations, foundations, and statements (RV6, part A)

Date: 2026-11-10

Role: source reader

Reference: the C++23 working draft sources.

Commit read: 98efc40f37190ddba31c5548cacfd49061c9a8f0.

The review covers the rules and principles that changed after RV5 read the tree: CLS-26, BF-6, ALN-4, AUTO-P3, RET-6, REF-1, SINIT-6, UNW-3, CONV-1, NAM-21, STG-5, STG-P3, TR-P1, JMP-10, and SW-P1. It covers the text that item F3 changed: REF-1 again, CV-6, DTOR-6, LSUP-11, STG-10, STG-P2 with the FixedVec example and the table of storage forms, UNI-14, UNI-6, UNI-9, UNI-P1, and CPY-15. It gives a first reading of Q-0422, Q-0424, and Q-0425. It reads TR-17 again for the RV5 point A-14. Points carry IDs of the form RV6-A-n.

## Result

Most of the text agrees with the reference. The RV5 repairs close RV5-A-1 to A-8, A-10, A-12, A-13, A-15, A-16, and A-18. Three items read against the reference:

- UNI-6 makes a construction inside a member that is not active undefined. The reference gives that construction defined behavior at run time (RV6-A-14).
- CPY-15 cites a skip of the constraint check that the reference does not have (RV6-A-18).
- LSUP-11 keeps the known split on element types that are not literal (RV6-A-12).

UNI-9 adds a condition, because the reference also forbids an anonymous union inside an anonymous union (RV6-A-17). The doubts on UNI-14 and STG-P2 come from the allowance that constant evaluation gives the library's construction function. That allowance names objects whose lifetime began, and a member that is not active has not begun (RV6-A-13). TR-17 still agrees with a doubt. Q-0422 and Q-0424 are supported. Q-0425 is supported with a doubt.

## Rules and principles changed after RV5

| Item | Verdict | Note |
|---|---|---|
| CLS-26 | agrees | The condition on trailing constraints closes RV5-A-1. |
| BF-6 | agrees | A bit-field of an unscoped enumeration type promotes as its enumeration does. A bit-field too wide for both int and unsigned int gets no promotion. |
| ALN-4 | agrees | The citation of STG-23 fits. An allocation function without an alignment parameter owes only fundamental alignment. |
| AUTO-P3 (decltype-auto row) | agrees | The row now matches JMP-10 for parameters, rvalue references, volatile locals, and structured bindings. |
| RET-6 | agrees with a doubt | RV6-A-2. A parameter of object type has no stated result. |
| REF-1 | agrees with a doubt | RV6-A-1. The list now matches the reference's test. The last bullet names the wrong type as its subject. |
| SINIT-6 | agrees with a doubt | RV6-A-3. The other variable that starts the initialization must also be non-inline. |
| UNW-3 | agrees with a doubt | RV6-A-4. An over-aligned single object of complete type also takes the sized form. |
| CONV-1 | agrees with a doubt | RV6-A-5. The bit-field repair matches. The wide character types still have a longer list of targets. |
| NAM-21 | agrees with a doubt | RV6-A-6. A trailing requires clause is missing from the list. The search for a friend does not enter inline namespaces. Q-0410's split remains. |
| STG-5 | agrees with a doubt | RV6-A-7. The hiding repair matches. The reference reserves the non-allocating forms to the library. |
| STG-P3 | agrees | The sentence on implicit-lifetime elements and the sentence on alignment by chance close RV5-A-13. |
| TR-P1 | agrees with a doubt | RV6-A-8. Some undefined behavior in the reference has no run that reaches it. The main paragraph shares RV6-A-19. |
| JMP-10 | agrees with a doubt | RV6-A-9. A thread-local variable at block scope also does not count. |
| SW-P1 | agrees | The statement after the switch is now advice, and the program is valid without it. RV4-3 and RV5-A-18 are closed. The advice to warn is Tidal's own; the reference is silent there. |

## Text that item F3 changed

| Item | Verdict | Note |
|---|---|---|
| REF-1 | agrees with a doubt | RV6-A-1. The base relation alone decides direct binding, and an ambiguous or inaccessible base makes the binding not valid. That part matches. |
| CV-6 | agrees with a doubt | RV6-A-10. A decltype specifier is a third way to apply the qualifier. |
| DTOR-6 | agrees with a doubt | RV6-A-11. The decltype after the tilde must give the class type. A decltype that gives a reference type does not. |
| LSUP-11 | reads against | RV6-A-12. Known split. The rejection of an assignment before construction matches for every element type. |
| STG-10 | agrees | The reference lists forbidden uses of a pointer to an object outside its lifetime. Pointer arithmetic is not among them, so stepping inside an array member that is not active is allowed. |
| STG-P2 | agrees with a doubt | RV6-A-16. The FixedVec claims match. Two rows of the table carry doubts. |
| UNI-14 | agrees with a doubt | RV6-A-13. The constant cases rest on a reading of the allowance for the library's construction function that the reference does not state. At run time, a construction inside a member can still start the member's life. |
| UNI-6 | reads against | RV6-A-14. The last sentence makes a defined construction undefined. |
| UNI-9 | adds a condition | RV6-A-17. The reference also forbids an anonymous union inside an anonymous union and allows a static assertion. |
| UNI-P1 | agrees with a doubt | RV6-A-15. Two more events change the active member. |
| CPY-15 | reads against | RV6-A-18. The reference checks the constraints of every template candidate. It has no skip for an exact match of a candidate that is not a template. |

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0422 | supports | RV6-A-9. A structured binding is an entity of its own kind and is no variable. Its name in a return statement is an lvalue. |
| Q-0424 | supports | RV6-A-17. A member declaration that defines an unnamed class declares a type inside the anonymous union, and the reference forbids that with a diagnostic. |
| Q-0425 | supports with a doubt | RV6-A-13 and RV6-A-14. Constant evaluation rejects the construction. At run time the construction is defined, and the member can become active when a whole object of the member's type fits in the storage that the allocation call returns. |

## RV5 point kept with a note

| Item | Verdict | Note |
|---|---|---|
| TR-17 (RV5-A-14) | agrees with a doubt | RV6-A-19. The reference still does not settle which rule governs two definitions of main in two units. The Evidence note states the open reading. The freestanding case is still unstated. |

## RV6-A-1

REF-1 now lists every case that the reference's test admits for an lvalue that is not a bit-field. These cases cover deeper qualification of pointers, pointers to members, arrays of pointers, the removal of the non-throwing property from a function type, and an array of known bound for a reference to an array of unknown bound. The list also matches for a base class.

The doubt is in the last bullet. The list opens with the types that the lvalue has. The last bullet then describes the type that a pointer to the lvalue's type reaches. That type is the referenced type, and the lvalue does not have it. Read strictly, the bullet names the wrong side of the binding. The meaning is clear from the examples.

A class lvalue that converts to an lvalue through a conversion function also binds directly to the result. REF-1 does not cover that case. Another rule may cover it.

## RV6-A-2

RET-6 now limits the undefined use to a local object, and it gives the case of a variable of rvalue reference type. A parameter of object type that JMP-10 counts also yields an rvalue reference. The reference makes it implementation-defined whether a parameter's lifetime ends when the function returns or at the end of the full expression that holds the call. On an implementation that chooses the second, a use of the returned reference inside that full expression is defined. RET-6 gives no result for a parameter.

## RV6-A-3

SINIT-6 now allows a later point before the thread's first use of the variable or of another thread-local variable of the unit. The reference counts a use of any non-inline thread-local variable defined in the same unit. A use of an inline thread-local variable does not start the initialization. A use inside another initialization also does not count. SINIT-6 points to OBJ-11 for the uses. OBJ-11 limits the other variable to one with dynamic initialization, which is the known split of Q-0416. The reference has no such limit.

## RV6-A-4

UNW-3 now defers the class case to NEW-22. NEW-22 picks the unsized form when a class declares both, which matches the reference.

For the global functions, UNW-3 gives the sized form only for a single object whose alignment is not more than the default new alignment. The reference first keeps the forms with or without the alignment parameter, as the type's alignment decides. Then it picks the sized form for any single object of complete type. So an over-aligned single object of complete type takes the sized aligned form. UNW-3 says only that it takes a form with the alignment.

The rest matches. A placement form uses the one matching deallocation function, and no function runs when none matches or several match.

## RV6-A-5

CONV-1 and BF-6 now promote a bit-field of an unscoped enumeration type as the enumeration promotes, which closes the first part of RV5-A-8. The second part stands. For the four wide character types, the reference tries int, unsigned int, and then the long types in order. CONV-1 names only int and unsigned int. The result on both matrix targets is the same.

## RV6-A-6

The friend sentence now matches the reference's behavior. A name after the qualified name inside the declarator is searched in the named scope, then by ordinary lookup.

RV5-A-10 said that the search of a named namespace includes its inline namespaces. That reading was wrong. The search in the named scope is a plain search of that scope. For a class it also covers the bases. For a namespace it does not enter inline namespaces. The reference's own illustration rejects a friend whose parameter type is declared only in an inline namespace of the named namespace. NAM-21 does not state this limit. A test of the limit could follow.

For a declaration that is not a friend, the list in NAM-21 still omits a trailing requires clause. The reference places every part of the declaration after the qualified name in the named scope. The underlying type of an enumeration is the known split of Q-0410.

## RV6-A-7

STG-5 now says that a class's own allocation function hides the global non-allocating form, and that the global scope prefix finds it. That repair matches.

A second doubt concerns the program's own non-allocating forms (LSUP-7). The reference reserves these forms to the library and forbids a program to define replacements. Its promise that an array placement adds no overhead names the library's function. STG-5 and LSUP-7 treat the program's own definitions as those functions. This reading underlies every placement test, and pending.md may already hold it.

## RV6-A-8

TR-P1 now says that an undefined rule constrains only a run that reaches the operation. That statement holds for operations at run time. The reference also has undefined behavior that arises during translation, with no run at all. Examples are some misuses of the preprocessor and some lexical forms. The library rules add another kind: a program that adds declarations to the standard namespace has undefined behavior unless a rule allows it. Several Tidal tests declare names in that namespace. TR-P1's statement does not cover these kinds.

The row for conditionally-supported constructs now matches. The paragraph on main shares RV6-A-19.

## RV6-A-9

JMP-10 now matches the reference on rvalue references, volatile objects, lvalue references, the innermost function or lambda, and parentheses. The rule counts a variable with automatic storage. A thread-local variable at block scope has thread storage, so it does not count either. JMP-10 names only a static local.

Q-0422: the reference treats a structured binding as an entity of its own kind, apart from variables. The rule that turns a name into an xvalue in a return statement applies only to variables with automatic storage. So a structured binding's name stays an lvalue, and a parenthesized name gives an lvalue reference under decltype. This holds for a binding to a class's data members and for the tuple-like case. In the tuple-like case, the hidden reference variables exist, but the name denotes the binding. The provisional answer follows the reference.

## RV6-A-10

CV-6 now covers an alias and a template parameter. The reference applies the same results to any way that a qualifier reaches an array type or a function type without a declarator. A decltype specifier is one more such way. A qualifier on a function type is ignored, and a qualifier on an array type applies to the elements, for any of these ways.

## RV6-A-11

DTOR-6 now allows decltype of the object after the tilde. The reference requires the type name or decltype after the tilde to denote the class type of the destructor. A decltype of a variable of class type gives that class type, as the DTY-3 test shows. A decltype of an expression that is an lvalue other than a plain name gives a reference type. The same holds for a variable of reference type. Neither form denotes the class type. DTOR-6 does not say which forms of decltype work.

## RV6-A-12

LSUP-11 now rejects an assignment to an element before its construction for every element type. That conclusion matches. During constant evaluation, a change to an object is allowed only when its lifetime began in the same evaluation. The element's lifetime has not begun.

The known split of RV5-A-9 remains. The reference ignores the bodies of the allocator members only for a literal element type. For any other element type, the call of the global allocation function inside the body ends constant evaluation before any assignment. LSUP-11 still says the element type need not be literal. pending.md row 4 holds this split.

## RV6-A-13

UNI-14's first paragraph matches the reference at run time. An object of the member's type at the member's address, inside a union whose lifetime has begun, becomes that member. The new object ends the life of the member that was active before.

The constant cases carry a doubt. The reference lets constant evaluation reduce the library's construction function to its constructor call only for two kinds of target. One is storage that the library allocator gave. The other is an object that started to live during the same evaluation. A member that is not active has no lifetime yet. Read strictly, the allowance does not cover the activation of a member by that function. The function's body then contains a placement new, and constant evaluation stops. Clang and GCC accept the activation. They read the union's own lifetime as enough. The reference does not state that reading.

The second paragraph, new in F3, matches in constant evaluation. An element of an array member, or a member of a member union, has an enclosing member whose lifetime has not begun. The allowance does not apply, and the construction ends constant evaluation. So option 1 of Q-0425 holds for the constant forms.

At run time the second paragraph needs a condition. Every call of a function named operator new creates objects implicitly in the storage it returns, when that gives the program defined behavior. The non-allocating form is such a function. If a whole object of the enclosing member's type fits in that storage at the member's address, the implicit creation can start that member's life inside the live union. The new object then becomes a subobject of it, and the member is active. In the member union form of Q-0425, the inner union and the new double have the same size, so this can happen. In the array form, the storage starts at a later element or is smaller than the array, so it cannot. The paragraph holds only when no such object fits.

## RV6-A-14

The last sentence of UNI-6 says that a construction inside a member that is not active uses a member that does not exist. The class of the rule is undefined. At run time the reference gives the construction defined behavior. The construction reads nothing and calls nothing on the member. Forming the address of an element or of a member of a member that is not active is allowed, because it does not access an object.

Two outcomes follow, and both are defined:

- If no implicit creation applies (RV6-A-13), the new object is not nested in the union. It reuses the union's storage and ends the life of the union and of each object that contains the union. Undefined behavior comes only from a later use of those objects through their old names. A union with a trivial destructor at automatic storage needs nothing at the end of its scope.
- If implicit creation applies, the enclosing member becomes active and the union stays alive.

In Q-0425's run test, the program reads the new double through the pointer that placement new returns. That read is defined. So the reference expects the program to run and return zero. The front end's expectation that the run stops on UNI-6 reads against the reference.

In constant evaluation the rejection matches, through the reason in RV6-A-13.

## RV6-A-15

UNI-P1 names two actions that change the active member. The reference has two more:

- A trivial copy or move of a whole union, by its constructor or its assignment operator, copies the active member of the source. The member of the same name becomes active in the target.
- At run time, an operation that creates objects implicitly in the union's storage can start a member's life (RV6-A-13).

The new sentence on constructions inside a member shares the run-time condition of RV6-A-13.

## RV6-A-16

The FixedVec claims match. Each row of bytes is the smallest array of bytes that holds its object, so the row hosts it. A cast of a row's address points to a byte. A pointer one past the object in one row has the address of the next row's object, yet it does not point to that object. A read through it is undefined.

Two rows of the table carry doubts:

- The row for a union slot for each element relies on the library's construction function in constant evaluation. It shares the doubt of RV6-A-13.
- The row for a union with an array member says that constant evaluation works only when an assignment makes the member active. The union's own initializer can also make the member active, and the library's construction function may do so for the whole member under the reading of RV6-A-13. Both build every element and need the element's default constructor. The table's conclusion, that no form inside the object gives all three properties, still holds.

## RV6-A-17

The reference forbids nested types, functions, and anonymous unions inside an anonymous union. It allows public non-static data members and static assertion declarations. UNI-9 does not name the nested anonymous union, and it does not say that a static assertion is allowed. An anonymous union inside an anonymous union is not valid, and a diagnostic is required.

Q-0424: a member declaration whose type is an unnamed class defined in that declaration declares a class inside the anonymous union. The reference forbids any nested type there. Nothing exempts an unnamed one. So the program is not valid, and a diagnostic is required. GCC's acceptance in strict mode is a missing diagnostic. The provisional answer follows the reference.

## RV6-A-18

The main part of CPY-15 matches. A constructor whose first parameter is the class type, with or without cv-qualifiers, and whose other parameters all have default arguments, is not valid. A constructor template never yields such a constructor, and the copy uses the copy constructor. CPY-15 names only the plain class type. A const or volatile class type by value is also covered.

In a class template, every specialization of such a declaration is not valid. Before instantiation the reference allows, and does not require, a diagnostic. CPY-15's claim that the implementation rejects at the definition describes the compilers.

The last sentence reads against the reference. The reference checks the constraints of a function template candidate during deduction, after the template arguments are known and before the function type is formed. Every template candidate goes through deduction. The reference has no rule that skips this check because a candidate that is not a template matches every argument exactly. So the constraint of the constructor template is checked for the class type itself in every copy. The rule that a template never yields a copy constructor applies after that check. The CNS-11 skip follows Clang 22 and GCC. It has no basis in this reference. The first review of Q-0426 reads CNS-11 itself.

## RV6-A-19

TR-17 keeps the compilers' answer with an Evidence note. The reading of RV5-A-14 stands. The reference requires exactly one function named main in the global scope, and a program with none needs a diagnostic. The linkage of main is implementation-defined. With external linkage, two definitions in two units can count as one function defined twice. That break falls under the one definition rule, which needs no diagnostic across units. The reference does not say which rule governs. TR-17's class is one reading, and it is not against the reference. The note in the Evidence segment records the open reading. The reference also lets a freestanding implementation decide whether a program must define main. TR-17 and TR-P1 still do not mention it. The verdict stays agrees with a doubt.
