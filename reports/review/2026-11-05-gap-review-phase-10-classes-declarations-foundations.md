# Gap review: the text phase 10 wrote on classes, declarations, foundations, overloading, and statements

Date: 2026-11-05

Role: source reader

Reference: the C++23 working draft sources. The review covers part A of item RV4 of decision 0058. Part A reads each rule and principle whose prose changed between commit d962fc7 and the present on the pages for access, comparisons, conversion functions, members, operators, unions, alignment, placeholder types, constant functions, enumerations, default arguments, return type deduction, reference initialization, static initialization, structured bindings, conversions, language support, names, objects, translation, conversion sequences, the if statement, and the switch statement. Part B applies LSUP-P2 to the programs of Q-0299 and SB-P2 to the programs of Q-0395. Part C reads questions Q-0403, Q-0413, Q-0414, Q-0152, Q-0156, Q-0337, Q-0383, Q-0386, and Q-0402. The review describes language behavior in the reviewer's own words.

## Result

Most of the changed text agrees with the reference. Five rules contradict it, and four rules miss a case. Most of the contradictions are places where Tidal keeps the matrix's answer against an earlier review (Q-0404, Q-0406, Q-0407, Q-0410). Two contradictions are new. They are the friend sentence of NAM-21 and the constant-initialization sentence of OBJ-11. Both principle uses give the provisional answer. Every question in part C has support from the reference, and four of them carry a doubt.

Gaps found in part A:

- ACC-20 makes the named member of every specialization a friend. The reference makes it a friend only when the specialization matches the qualifier's pattern and the member's declaration matches the friend declaration after substitution.
- CLS-2 excludes default template arguments of member templates from the places where the class is complete. The reference lists them among those places.
- CLS-26 misses a case. An explicit object parameter taken by value also clashes with a member function that has no ref-qualifier when the class types match.
- OPR-P3 says each built-in suffix is all lowercase or all uppercase. The reference lets the unsigned part and the long or size part of an integer suffix take their case separately.
- ENUM-10 gives an enumerator whose initializer has an unscoped enumeration type that enumeration type. The reference gives it the enumeration's underlying type. The reference also leaves the type of a first enumerator with no initializer to the implementation as a signed integer type.
- CONV-9 asks for constants at the outer levels only when an inner level adds a constant. The reference also asks for this when an inner level adds a volatile qualifier or drops an array bound.
- NAM-21 excludes the underlying type of an enumeration from the qualified scope, and the reference includes it. NAM-21 also says a friend declaration with a qualified name gets no lookup in the named scope. The reference searches that scope first for names after the declarator in a friend declaration and then falls back to ordinary lookup.
- OBJ-11 says a use of a constant-initialized thread-local variable does not start the deferred construction. The reference counts a use of any thread-local variable of the unit that is not inline.
- ICS-45 lists direct-initialization contexts. The list leaves out member initializers, parenthesized initializers of a new expression, and the other places that convert an operand to a boolean value, such as conditions of selection and iteration statements.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| ACC-20 | contradicts | The reference befriends the member of a specialization only when deduction from the qualifier succeeds and the declaration then matches the member. Specializations outside the pattern and members whose declarations differ are not friends. |
| ACC-10 | agrees with a doubt | The first part agrees. Friendship does not pass along in the reference. The access of a friend of a nested class rests on reading the friend's definition as text inside a member of the enclosing class. |
| DCMP-1 | agrees | The reference flattens array members of any depth into their elements before the comparison. |
| CVF-7 | agrees | A conversion function template must not have a deduced return type. A target that contains a placeholder under a pointer or reference counts as deduced. |
| CLS-2 | contradicts | The reference counts a default template argument inside the class's member list as a place where the class is complete. Every baseline compiler rejects the program (Q-0406). |
| CLS-26 | missing case | The reference also pairs a function with no ref-qualifier with one whose explicit object parameter is the class type by value, because it compares the types after it removes references. |
| CLS-9 | agrees | Two explicit object parameters of different types give two functions. Two declarations that differ only in being static clash. |
| OPR-P3 | missing case | The integer suffixes combine a separately cased unsigned letter with a long or size suffix, so mixed forms such as an upper unsigned letter with a lower long letter are built-in suffixes too. Only the double long suffix must keep one case. |
| OPR-9 | agrees | The reference treats the suffix in the spaced form as an identifier, so the reservation of a leading underscore with an uppercase letter applies there and not in the joined form. |
| UNI-P4 | agrees | The class must be a literal type, its non-explicit conversion functions must give exactly one allowed target type, and the selected function must be constexpr. |
| BF-1 | agrees | Same reading as UNI-P4. |
| ALN-8 | agrees | The alignment operand uses the same rule for class types as a bit-field width. |
| AUTO-P3 | agrees with a doubt | The new sentence agrees for a local object of automatic storage. The reference excludes volatile objects and local lvalue references from this rule, so for those the type stays an lvalue reference. |
| AUTO-19 | agrees | Deduction against a pointer pattern fails for the null pointer literal and for a zero integer. A conditional that yields a pointer type deduces. |
| CEV-1 | agrees | An implicit conversion that calls an immediate constructor is an immediate invocation, also when it initializes a parameter. |
| ENUM-10 | contradicts | For an initializer of unscoped enumeration type the reference gives the underlying type of that enumeration. For a first enumerator with no initializer it gives a signed type that the implementation picks (Q-0407). |
| DEFARG-P2 | agrees | The reference forbids a later default outside the class that makes a default, copy, or move constructor. It also lists the copy and move assignment operators, which cannot take defaults anyway. |
| RET-2 | agrees with a doubt | The new row agrees. For a prvalue the result is an rvalue reference, so the call gives an xvalue. The value category changes from prvalue to xvalue in that case. |
| REF-8 | agrees | A reference to a bit-field binds only through a temporary, and only an lvalue reference to a const type that is not volatile, or an rvalue reference, can do that. |
| SINIT-4 | agrees | The reference adds the two conditions that the new text states, and it allows the same choice for thread storage. |
| SINIT-5 | agrees | The deferral, its trigger, the exclusion of uses that come from other initializations, and the separate trigger for inline variables match the reference. |
| SINIT-6 | agrees with a doubt | The reference ties a deferred thread-local initialization to the first use of any non-inline thread-local variable of the same unit. SINIT-6 names only the variable itself, and OBJ-11 states the wider trigger. |
| SB-P2 | agrees | The reference takes the member form only when the class search finds a function template whose first parameter is a constant. Otherwise it uses argument-dependent lookup alone. |
| CONV-P3 | agrees | Object pointers convert to a pointer to a virtual base. Pointers to members cannot convert through a virtual base or a base of one. |
| CONV-9 | missing case | The reference asks for constants at every outer level whenever an inner level gains any qualifier or loses an array bound. Adding volatile alone at an inner level also needs them. |
| LSUP-P2 | agrees with a doubt | The library declares the alignment type as a scoped enumeration over the size type. The reference makes a program's own declaration in the standard namespace undefined, which is a weaker class than not valid with no diagnostic. Both classes allow every result of the matrix. |
| LSUP-2 | agrees with a doubt | The value for all-equal parts is the strong equal value cast to the return type, so the strong class is used. The reference does not fix where the implicit use by a deduced return type takes place (Q-0402). |
| LSUP-11 | agrees | A read of storage where no object lives is outside a lifetime and fails in constant evaluation. |
| NAM-21 | contradicts | The scope of the qualifier covers everything after the enumeration's head name, the underlying type included (Q-0410). A friend declaration with a qualified name does get a first search in the qualifier's scope. |
| OBJ-1 | agrees | The reference runs the destructor when the body of a delegating constructor exits by an exception. |
| OBJ-11 | contradicts | The address part agrees. The reference requires the deferred construction before the first use of any non-inline thread-local variable of the unit, constant-initialized ones included. |
| TR-P1 | agrees with a doubt | The reference states the rule that a program has exactly one main with no waiver. Two definitions of main in two units then also seem to need a diagnostic, and TR-P1 names only the missing main. |
| TR-4 | agrees with a doubt | The content agrees. TR-4 is a required rule, and it now holds a rule on the program that needs a diagnostic, which TR-P1's table gives to the reject class. |
| ICS-P1 | agrees | Promotion of a bit-field, null pointer constants, string literals, and overload sets do depend on the expression. Access, deletion, and narrowing are judged after the choice. |
| ICS-45 | missing case | The parts on copy initialization of a bound temporary agree. The list of direct-initialization contexts leaves out member initializers, parenthesized new initializers, and the other places that convert an operand to a boolean value. |
| ICS-28 | agrees | A reference bound to a materialized temporary binds an rvalue. The rank step with its same-rank tie-breakers comes before the rvalue reference rule. |
| IF-25 | agrees | A const variable of any enumeration type that is not volatile is usable in constant expressions when its initializer is constant. |
| SW-P1 | agrees with a doubt | An enumeration can hold values that no enumerator names. The reference makes flowing off the end undefined only when it happens and does not require a statement. An enumeration with no fixed type whose enumerators cover its whole range has no other values. |

## Principle uses

| Principle | Question | Test file | The model's answer | Matches the provisional answer? | Reference verdict |
|---|---|---|---|---|---|
| LSUP-P2 | Q-0299 | `alignment-enumeration-unscoped.cpp` | The type is declared unscoped, which is another kind. The program is not valid and needs no diagnostic. An implementation can reject the declaration where it stands, so the GCC and EDG rejection and the Clang acceptance both conform. | yes | supports with a doubt |
| LSUP-P2 | Q-0299 | `alignment-enumeration-wrong-underlying-type.cpp` | The type is scoped but has a one-byte underlying type, which is another underlying type. The program is not valid and needs no diagnostic, and rejection at the declaration is allowed. | yes | supports with a doubt |
| LSUP-P2 | Q-0299 | `alignment-enumeration-class.cpp` | The type is declared as a class, which is another kind. The program is not valid and needs no diagnostic, and rejection at the declaration is allowed. | yes | supports with a doubt |
| SB-P2 | Q-0395 | `tuple-member-variable-template-get-uses-free.cpp` | The class has the tuple protocol. The member named get is a variable template, which does not count, so each element comes from the free get found by argument-dependent lookup. The bindings are 1 and 2, the assertion holds, and the program is accepted. | yes | supports with no doubt |
| SB-P2 | Q-0395 | `tuple-member-class-template-get-uses-free.cpp` | The member named get is a class template, which does not count, so each element comes from the free get. The bindings are 1 and 2, and the program is accepted. | yes | supports with no doubt |

The doubt for the three LSUP-P2 rows: the reference makes any program declaration in the standard namespace undefined, and it does not describe a program-supplied alignment type at all. Undefined behavior allows the same results as a rule with no diagnostic, so the outcome is the same, but the class differs.

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0403 | supports with no doubt | The defaulted equality flattens nested arrays down to their scalar elements, and constant evaluation gives the same result as run time. The GCC and EDG results do not follow the reference. |
| Q-0413 | supports with no doubt | The conversion of the argument calls an immediate constructor outside an immediate context, so it is an immediate invocation. The call is not constant for the value given, and main cannot escalate, so the program is not valid. |
| Q-0414 | supports with a doubt | A private member is reachable from code inside a direct member of the class, and the friend's body is written inside the nested class, which is a member. The reference also says friendship does not pass along, and it does not say whether a friend defined inside a nested class counts as code inside that member. |
| Q-0152 | supports with no doubt | The reference compares ranks first and resolves same-rank ties inside that step. The rvalue reference preference comes next, and the qualification step after it. Every row of the question follows from that order. |
| Q-0156 | supports with no doubt | For an rvalue reference, the candidate set for direct binding admits functions that return a plain value, and in direct initialization the explicit ones count when only a qualification adjustment remains. |
| Q-0337 | supports with a doubt | The candidate set for direct binding gives an lvalue reference only functions that return an lvalue reference. The fallback then copy-initializes a temporary, which excludes explicit functions. An example in the reference binds an lvalue reference to const directly to a value result of a conversion function, which does not fit that candidate set. |
| Q-0383 | supports with no doubt | Declaring a literal operator whose suffix does not start with an underscore is not valid, and no diagnostic is required, so every compiler result conforms. In `literal-builtin-suffix-wins.cpp` each literal matches the grammar of a built-in literal and is read as one, so no operator runs. The test's expectation of a clean run records what compilers do, because the declarations in the test are themselves not valid with no diagnostic required. |
| Q-0386 | supports with no doubt | The reference requires a declaration only of the ordering type that a use involves, and it names each type separately. |
| Q-0402 | supports with a doubt | The reference counts the deduced return type of a defaulted three-way comparison as an implicit use that the declaration must precede. It does not say where that use takes place: at the defaulted declaration, at the end of the class, or at the first use of the operator. |

## ACC-20

The reference grants friendship through a member named with a dependent qualifier only after two checks. Deduction of the friend's template parameters from the specialization must succeed, and the friend declaration with the deduced arguments substituted must match the member's declaration. A specialization that does not match a pointer pattern fails the first check. An explicit specialization whose member has a different return type fails the second check, even when it matches the pattern. ACC-20 says the member of every specialization is a friend, explicit specializations included, and that the pattern does not limit this (Q-0404). The text has no condition on the member's declaration. Clang and GCC agree with ACC-20 on the pattern case, and EDG agrees with the reference.

## ACC-10

The reference gives a nested class the access of any member. A friend function declared in the nested class gets the access of the nested class's members. The reference states that friendship does not pass along. So ACC-10's chain, from friend to nested class to enclosing class, has no direct support. The access can still hold on a textual reading, because the friend's definition is written inside the nested class, and the nested class is a direct member of the enclosing class. The reference does not settle which reading applies. This is the same doubt as for Q-0414.

## CLS-2

The reference's list of places where a class counts as complete includes default template arguments in the class's member list, next to function bodies, default arguments, exception specifications, and default member initializers. CLS-2 says a default template argument of a member template is not among these places. Every baseline compiler agrees with CLS-2 (Q-0406), so implementations and the reference differ here.

## CLS-26

The reference decides whether two non-static member functions clash by their object parameters. When exactly one of them is an implicit object function with no ref-qualifier, the two match if their object parameter types are the same after references are removed. An explicit object parameter that takes the class by value therefore matches a function with no ref-qualifier and the same cv-qualification. CLS-26 names only an explicit object parameter of reference type. The match also depends on cv-qualification: a function with a const qualifier pairs with an explicit object parameter of the const class.

## OPR-P3

The integer suffix grammar of the reference builds a suffix from an unsigned letter and an optional long, double long, or size suffix, in either order. Each piece has its own lowercase and uppercase forms. So mixed spellings, such as an uppercase unsigned letter with a lowercase long letter, are built-in suffixes. Only the double long suffix must use one case for both letters. OPR-P3 says each suffix is all lowercase or all uppercase, which leaves out these mixed forms. A literal with a mixed suffix keeps its built-in meaning as well.

## ENUM-10

The reference gives two rows of ENUM-10's table different answers. Take an enumeration with no fixed underlying type, and an enumerator whose initializer has an unscoped enumeration type. Inside the list, that enumerator takes the initializer enumeration's underlying type. When the first enumerator has no initializer, its type is a signed integer type that the reference leaves unspecified. Tidal states the int type as a fact. Every baseline compiler gives the enumeration type and int (Q-0407).

## CONV-9

The reference builds a combined type level by level. Whenever any inner level gains a cv-qualifier, or changes from an array of known bound to one of unknown bound, every level above it, down to the outermost pointer, must be const in the target. CONV-9's last item covers only the case where an inner level gains const. Adding volatile alone at an inner level, or dropping an inner array bound, also needs const at every level above. Without it the conversion is not implicit.

## NAM-21

There are two differences. First, the reference brings the scope of the qualifier into everything after the head name of an enumeration, and the underlying type comes after that name. NAM-21 says the underlying type is not part of the scope (Q-0410). Every baseline compiler rejects the program that would find the name there. Second, the reference has a separate rule for friend declarations whose declarator names a qualified member. Names written after the declarator are first searched in the class or namespace that the qualifier names. If that search finds nothing, ordinary lookup runs. NAM-21 says a friend declaration gets no such lookup, which contradicts this rule. The friend search is a single-scope search, so it differs in detail from the scope inclusion that NAM-21 lists for other declarations.

## OBJ-11

The reference sequences a deferred thread-local initialization before the first use, by the thread, of any non-inline thread-local variable defined in the same unit. That set includes variables with constant initialization. OBJ-11 says a use of a constant-initialized variable does not start the construction. That sentence contradicts the reference. Implementations may skip the initialization step for a variable they know to be constant-initialized, so the matrix may follow OBJ-11. A test would show which. The new statement that taking the address is a use agrees.

## ICS-45

The reference makes these forms direct initialization: an initializer in parentheses, a braced list without an equals sign, a cast operand, a member initializer, a parenthesized initializer of a new expression, and every context that converts an operand to a boolean value. The boolean contexts include the conditions of selection and iteration statements, the first operand of the conditional operator, the operand of a static assertion, and the operands of the logical operators. ICS-45 lists the cast operand, the variable, the copy or move constructor parameter, and the operands of the three logical operators. The list does not say that it is complete, but it reads that way. For the logical operators, the reference reaches the explicit conversion through the built-in operator's boolean conversion of the operand. Overload resolution among built-in candidates does not reach it. The statement that a bound temporary is copy-initialized agrees.

## AUTO-P3

The reference turns a parenthesized name in a return statement into an xvalue only when the name denotes an automatic variable declared in the function's body or parameter list. That variable must be an object that is not volatile, or a reference of the rvalue kind to such an object. A local lvalue reference stays an lvalue, so a deduced type from it is an lvalue reference. A volatile local object also stays an lvalue. AUTO-P3 says "a local variable" without these limits. JMP-10 has the same breadth.

## RET-2

The new row for a forwarding placeholder agrees with the deduction rule. A returned prvalue gives an rvalue reference return type, so the call expression is an xvalue. The row's claim that the value category stays holds for lvalues and xvalues only. A returned name of a local object is an xvalue in a return statement (JMP-10). With this placeholder that gives an rvalue reference to a local object, and with an lvalue reference placeholder it fails to deduce. RET-2 does not mention either effect.

## SINIT-6

The reference sequences a deferred thread-local initialization before the first use, by that thread, of any non-inline thread-local variable of the same unit. SINIT-6 says the variable is initialized when the thread first uses it. The reference's trigger is wider, and OBJ-11 states it. The reference gives no deferral rule for inline thread-local variables, and SINIT-6 now excludes them. That gap is in the reference.

## LSUP-P2

The library declares the alignment type as a scoped enumeration whose underlying type is the size type, and LSUP-P2 agrees. The reference requires a library declaration before a program names the type, and it makes any program declaration in the standard namespace undefined. A rule with no diagnostic is a stronger class than undefined behavior, because it says the program is not valid. Both classes allow rejection at the declaration and acceptance. The class choice is Tidal's and has no direct base in the reference.

## LSUP-2

The new sentence agrees. The value returned when every part compares equal is the strong equal value converted to the declared return type, so a defaulted comparison with a weak or partial return type uses the strong class. The reference counts the deduced return type of a defaulted comparison as an implicit use of the class, and it requires the declaration to come before the use. It does not say where in the text that use happens. See Q-0402.

## TR-P1

TR-P1 now names one requirement that needs a diagnostic at link time, that a program defines main. The reference's rule is that a program has exactly one function named main at global scope, with no waiver of the diagnostic. A program with two definitions of main in two units breaks the same rule. The rule against multiple definitions across units waives the diagnostic for other functions, so it is unclear which rule governs a second main. TR-P1 does not cover that case. The reference also makes main optional in a freestanding environment, which Tidal may treat as outside its scope.

## TR-4

The prose agrees with the reference. TR-4 has the required class, which TR-P1 gives to facts about the implementation. TR-4 now also holds a rule on the program, that a program defines exactly one main, and a break of that rule needs a diagnostic. TR-P1's table gives such rules the reject class. The class of TR-4 and the class of the main requirement differ.

## SW-P1

An enumeration with a fixed underlying type can hold any value of that type. One with no fixed type can hold every value in the smallest bit-field range that covers its enumerators. So a switch with a case for each enumerator can still complete in most cases. When the enumerators cover the whole range of an enumeration with no fixed type, no other value exists, and the switch cannot complete. The reference does not require a statement after the switch. It makes flowing off the end of a value-returning function undefined when it happens. FN-7 is a recommended rule, which fits, but "needs a statement" reads stronger than the reference.

## Q-0414

See ACC-10. The textual reading supports the provisional answer. The reference's statement that friendship does not pass along leaves a doubt.

## Q-0337

The literal candidate set for direct binding supports option 1. An lvalue reference gets only conversion functions that return an lvalue reference. Without one, the reference falls back to copy initialization of a temporary, which takes no explicit function. The binding rule for a const lvalue reference also allows a direct bind to an rvalue result of a conversion, and an example in the reference binds a const lvalue reference to part of a class value that a non-explicit conversion function returns. That example does not fit the candidate set as written. If the candidate set for a const lvalue reference were read to include value results, explicit functions in direct initialization would qualify too, which is option 2.

## Q-0402

The reference names the deduced return type of a defaulted three-way comparison as an implicit use of the ordering type, and the library declaration must come before that use. The reference does not fix the point of the use. For a member defaulted inside the class, deduction needs the member types, so it can happen at the declaration or later. Option 1 is a reasonable reading, but the reference does not decide between the options.
