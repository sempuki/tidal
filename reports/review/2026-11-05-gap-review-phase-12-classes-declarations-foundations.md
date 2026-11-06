# Gap review: phase 12 text on classes, declarations, foundations, statements, and exceptions (RV5, part A)

Date: 2026-11-05

Role: source reader

Reference: the C++23 working draft sources. The review covers the present text of CLS-26, ALN-4, the decltype-auto row of AUTO-P3, RET-6, REF-1, SINIT-6, UNW-3, CONV-1, CONV-9, LSUP-11, NAM-21, OBJ-11, STG-5, STG-8, STG-P3, TR-4, TR-16, TR-17, TR-P1, and JMP-10. It gives a first reading of Q-0415, read against the present NAM-21 sentence, and of Q-0421. It also settles the doubt that the last review left on SW-P1 (ledger point RV4-3). Points carry IDs of the form RV5-A-n.

## Result

Most of the text agrees with the reference. REF-1 reads against it, because its list of direct bindings is narrower than the reference's test and REF-8 closes the list. SW-P1 adds a condition, so the doubt of RV4-3 stands. LSUP-11, OBJ-11, and NAM-21 each keep one known split from the reference that pending.md row 4 already holds (Q-0416 and Q-0410 and the literal type case). RET-6 claims undefined behavior in two cases where the reference gives a valid reference. TR-17 and TR-P1 rest on a reading of the main rule that the reference leaves open, because the linkage of main is implementation-defined. Both questions are supported with a doubt.

## Rules and principles

| Item | Verdict | Note |
|---|---|---|
| CLS-26 | agrees with a doubt | RV5-A-1. Two such overloads clash only if their trailing constraints are also equivalent. |
| ALN-4 | agrees with a doubt | RV5-A-2. The reference does not state what happens when a new expression builds an object in storage that is not aligned for it. |
| AUTO-P3 (decltype-auto row) | agrees with a doubt | RV5-A-3. Parameters and locals of rvalue reference type also become xvalues in a return statement. The row names only local objects. |
| RET-6 | agrees with a doubt | RV5-A-4. A use of the returned rvalue reference is not always undefined. |
| REF-1 | reads against | RV5-A-5. The reference binds directly in more cases than the list gives, and REF-8 then rejects valid programs. |
| SINIT-6 | agrees with a doubt | RV5-A-6. The reference gives only the latest point for a deferred start. |
| UNW-3 | agrees with a doubt | RV5-A-7. The rule does not give the choice between the sized and unsized forms in a class. |
| CONV-1 | agrees with a doubt | RV5-A-8. A bit-field of enumeration type promotes as its enumeration type does. |
| CONV-9 | agrees | Each listed conversion has no implicit form. The rule for added qualifiers at deeper levels matches. |
| LSUP-11 | reads against | RV5-A-9. Known split: the reference exempts the allocator members only for a literal element type. |
| NAM-21 | agrees with a doubt | RV5-A-10. The friend sentence says "after the declarator" where the reference means after the qualified name inside the declarator. The known split of Q-0410 remains. |
| OBJ-11 | reads against | RV5-A-11. Known split (Q-0416): the reference starts the deferred construction on a use of any thread-local variable of the unit. |
| STG-5 | agrees with a doubt | RV5-A-12. A class's own allocation function hides the non-allocating form unless the expression starts with the global scope prefix. |
| STG-8 | agrees | The three operations, the look-ahead choice of a set, the pointer to a suitable object at the start of the storage, and the alignment condition all match. |
| STG-P3 | agrees with a doubt | RV5-A-13. The last sentence is too strong, and elements of an implicit-lifetime type need no construction. |
| TR-4 | agrees | A function that is used but defined nowhere makes the program not valid with no diagnostic required. |
| TR-16 | agrees | The reference leaves unspecified whether a floating-point result during translation equals the run-time result. |
| TR-17 | agrees with a doubt | RV5-A-14. A missing main needs a diagnostic. Two definitions in two units may fall under the one definition rule instead, which needs none. |
| TR-P1 | agrees with a doubt | RV5-A-15. The opening paragraph is broader than the table, and the class for conditionally-supported constructs documents the wrong thing. |
| JMP-10 | agrees with a doubt | RV5-A-16. An rvalue reference to a volatile object does not count, and the variable must belong to the innermost function or lambda. |
| SW-P1 | adds a condition | RV5-A-18. RV4-3 stands. The reference requires no statement after the switch. |

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0415 | supports with a doubt | RV5-A-10. For a friend whose declarator names a member of a class or namespace with a qualifier, a plain name after the qualified name in that declarator is first searched in the named scope, then by ordinary lookup. The test's parameter type is such a name. The doubt concerns the wording of NAM-21. |
| Q-0421 | supports with a doubt | RV5-A-17. The allocate member starts the life of the array only, and no element. A change to an element outside its lifetime ends constant evaluation. The doubt is that the exemption for the allocator members names the library's own template, and the tests declare their own. |

## RV5-A-1

The reference compares two member function declarations by their parameter types after the object parameter, by their object parameters, and also by their trailing constraints. A function with no ref-qualifier and one with a ref-qualifier, with the same cv-qualifiers and the same other parameters, clash only when their trailing constraints are equivalent or both absent. In a class template, two such overloads with different constraints are two functions. CLS-26 does not say so. The rest of CLS-26 matches, including the clash between an unqualified function and an explicit object parameter of the class type by value or by either reference kind.

## RV5-A-2

The reference requires an allocation function without an alignment parameter to return storage aligned only for types without new-extended alignment. Overload resolution in a new expression falls back to a form without the alignment when the class offers no aligned form. The reference does not then say what happens when the object is built at an address that does not suit the type. Undefined behavior follows by omission, since no object of the type can sit there. ALN-4 reaches the same end through STG-8, which concerns objects that the call creates implicitly. The object that the new expression itself builds is a different object, so the citation is a weak fit.

## RV5-A-3

In a return statement, a name, with or without parentheses, is an xvalue when it names an automatic variable whose type is an object type without volatile, or an rvalue reference whose referred type lacks volatile, and the variable belongs to the body or the parameter list of the innermost function or lambda. So a parameter also counts, and so does a local or parameter of rvalue reference type. The row names only a local object. A name without parentheses still gives its declared type, as the row says.

## RV5-A-4

RET-6 makes every use of the returned rvalue reference undefined. Two cases give a valid reference:

- a local or parameter of rvalue reference type that refers to an object which outlives the function. The return type is an rvalue reference to that object, which still exists.
- a parameter of object type. Whether a parameter's lifetime ends when the function returns or at the end of the full expression that holds the call is implementation-defined. On an implementation that chooses the second, a use inside that full expression is defined.

RET-6 also names only local variables, so parameters have no stated result.

## RV5-A-5

The reference binds an lvalue reference directly to an lvalue that is not a bit-field when a pointer to the lvalue's type converts to a pointer to the referenced type by a standard conversion. That test covers more than REF-1 lists:

- a function lvalue with a non-throwing exception specification and a reference to the same function type without it,
- a pointer to member, where the referenced type adds qualifiers below the top level,
- an array whose element type is a pointer, where the referenced array type adds qualifiers at deeper levels, as a qualification conversion allows.

REF-8 makes the REF-1 list closed for a reference that is not const, so REF-8 would reject the first case. In a probe by this reader, the local Clang and GCC accept the three bindings. EDG was not run. The direct binding to a base class subobject matches, and the reference makes a binding to an ambiguous or inaccessible base not valid, as REF-8 gives.

## RV5-A-6

The reference says that a deferred dynamic initialization of a thread-local variable happens before the first use, by the thread, of any non-inline thread-local variable of the same unit. The point and the threads are implementation-defined. SINIT-6 offers two choices, the thread's first statement or the thread's first use of the variable. The second choice is narrower than the reference, which allows any point before that use, and which can start the variable on the use of another variable. SINIT-6 points at OBJ-11 for these details, so the doubt is small.

## RV5-A-7

For the cleanup of a non-placement new expression, the reference uses the same selection as a delete expression. When the class declares both a sized and an unsized usual deallocation function, and the alignment preference leaves both, the reference picks the unsized one because the functions belong to a class. UNW-3 states the choice only for the global functions. The global part matches. A complete single object picks the sized form, an array picks the sized form only when the element class has a non-trivial destructor, and other cases are unspecified.

## RV5-A-8

The reference treats a bit-field of enumeration type as a value of that enumeration for promotion. The width does not decide its promotion. CONV-1 and BF-6 both say that a bit-field promotes by width. A second, smaller gap is the wide character types. Their list continues past unsigned int to the long types. CONV-1 gives only int and unsigned int, which is enough on the matrix targets.

## RV5-A-9

The reference ignores the bodies of the allocator members during constant evaluation only when the element type is a literal type. For a type that is not literal, the bodies count, and their calls of the global allocation and deallocation functions end constant evaluation. LSUP-11 says the element type need not be literal. pending.md row 4 already holds this split, and the matrix accepts. A second doubt: the reference's exemption names the library's own template. The tests declare a class template of that name themselves, which the reference does not treat as the library's template.

## RV5-A-10

For a friend declaration whose declarator names a member of a class or namespace through a qualifier, the reference searches the named scope first for an unqualified name that follows the qualified name inside the declarator. If that search finds nothing, ordinary lookup runs. The search of the named scope includes its inline namespaces and, for a class, its bases. The search does not climb to the enclosing scopes of the named scope. Ordinary lookup then starts from the friend declaration.

NAM-21 says "a name after the declarator". The parameter list is part of the declarator, so the sentence, read strictly, misses the case that Q-0415 tests. The reference's case is a name that follows the qualified name and is still part of the declarator.

For a declaration that is not a friend, the reference places all of the declaration after the qualified name in the scope that the qualifier names. The list in NAM-21 is then a set of examples. A trailing requires clause is one more part. The underlying type of an enumeration comes after the qualified name, and Q-0410 keeps the matrix's answer against the reference.

## RV5-A-11

The reference starts a deferred construction before the first use, by the thread, of any non-inline thread-local variable of the same unit. The other variable need not have dynamic initialization. OBJ-11 limits the trigger to variables with dynamic initialization and says that a use of a constant-initialized variable starts nothing. Q-0416 and pending.md row 4 already hold this split. A read of a const integral thread-local variable with constant initialization is likely not a use in the reference's sense, so it may start nothing in either reading. The rest of OBJ-11 matches: the point and the threads are implementation-defined, and a thread-local static data member defined at namespace scope is such a variable.

## RV5-A-12

The search for the allocation function of a new expression with a class type looks in the class first. If the class declares any allocation function, the global non-allocating form is hidden, and a placement new on a pointer then fails unless the expression starts with the global scope prefix. STG-5 does not say so. NEW-13 may cover the search. The rest of STG-5 matches. With new-extended alignment, the call falls back to the form without the alignment, because the library has no aligned non-allocating form. The library's non-allocating array form adds no array overhead, so the array starts at the pointer. A null pointer from the non-allocating form is undefined.

## RV5-A-13

The last sentence of STG-P3 says that, without the alignment argument, the storage holds no array of an over-aligned type. The reference promises only fundamental alignment in that case. The storage can still happen to sit at a suitable address, and then an array can exist there. STG-8 states the condition by address, which matches.

The diagram says that each element begins its lifetime when the program constructs it. For an element of implicit-lifetime type, the implicit creation can start the element's lifetime itself, as STG-8 allows at run time. The sentence is right for elements whose type is not implicit-lifetime. The note in the reference says that implicit creation does not start such subobjects.

## RV5-A-14

The reference requires exactly one function named main in the global scope, and that rule needs a diagnostic. A program with no main breaks it. The linkage of main is implementation-defined. With external linkage, as on every baseline compiler, two definitions in two units define one function, which keeps the count at one. The break is then of the one definition rule for a non-inline function, and that rule needs no diagnostic across units outside named modules. The reference does not settle which rule governs. TR-17's class of reject for the two-unit case is one reading. The reference also lets a freestanding implementation decide whether main is required at all, which TR-17 does not mention.

## RV5-A-15

The opening paragraph says that a program which breaks a rule on the program is not valid and must be diagnosed, whatever the reach of the broken part. The table then lists the no-diagnostic and undefined classes as rules on the program. The reference requires a diagnostic only for a diagnosable rule. For undefined behavior, it places no requirement only on an execution that contains the operation. A program whose undefined operation never runs is valid and must run correctly.

For conditionally-supported constructs, the reference requires a diagnostic when the implementation does not support the construct. It also requires the implementation's documentation to list the constructs it does not support. TR-P1 says that an implementation documents the behavior of a construct it accepts. Several such constructs carry implementation-defined meaning, which is documented, but the class itself asks for the list of unsupported constructs.

The main sentence follows TR-17 and shares RV5-A-14.

## RV5-A-16

JMP-10 matches on parentheses, on static locals, on volatile objects, and on lvalue references. Two limits are missing:

- a variable of rvalue reference type counts only when the referred type is not volatile.
- the variable must belong to the body or parameter list of the innermost function or lambda. A name of an enclosing function's variable inside a lambda does not count.

A structured binding is not a variable in the reference's sense, so its name stays an lvalue. JMP-10 does not say so.

## RV5-A-17

In Q-0421, the allocate member is the program's own declaration in the standard namespace. The reference's exemption, and its statement that allocate starts the life of the array but of no element, belong to the library template. If the program's template counted as the library's, the reading is clear: the assignment changes an element outside its lifetime, and constant evaluation stops. During constant evaluation, the reference lets a program change only objects that started to live during that same evaluation. Implicit object creation does not rescue the element during constant evaluation, because the exemption treats the member bodies as absent. The provisional answer follows. The same reasoning covers the member of a simple class element.

## RV5-A-18

The reference makes flowing off the end of a value-returning function, other than main or a coroutine, undefined when an execution does it. Nothing makes the program not valid. So a function with a switch that covers every enumerator, and no statement after it, is a valid program. Its behavior is undefined only for a value that reaches the end. SW-P1's "needs a statement" is stronger than this. FN-7 is a recommendation to warn, which fits the reference, but it does not turn the statement into a need. An enumeration with no fixed underlying type whose enumerators fill its range has no other values, so such a switch cannot reach the end there, and FN-7's warning still fires.
