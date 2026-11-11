# Gap review: phase 14 text on classes, declarations, foundations, and statements (RV7, part A)

Date: 2026-11-11

Role: source reader

Reference: the C++23 working draft sources.

Commit read: 6afa79f9f5b92813bcfec545d26220bce5572c47.

The review covers the rules and principles on these pages that changed after RV6 read the tree: DTOR-6, DTOR-8, CPY-15, UNI-P1, UNI-P3, UNI-6, UNI-9, UNI-14, CV-6, RET-6, REF-1, REF-2, CONV-1, NAM-21, VC-2, JMP-P3, and JMP-10. It records a read of the whole RV6 repair diff (commit 9c009448) on the classes, declarations, foundations, and statements pages and the glossary, hunk by hunk. It covers the text that phase 14 changed on these pages in commits d0597264 and 1130ceb8: DCL-19, DCMP-P1, IF-12, LOOP-9, LOOP-P1, LSUP-1, LSUP-6, STG-28, STG-29, STG-P2, and SW-1. It gives a first reading of Q-0422, Q-0424, Q-0430, and Q-0431. Points carry IDs of the form RV7-A-n.

## Result

Most of the text agrees with the reference. The RV6 repairs on these pages close RV6-A-1, A-2, A-5, A-9, A-10, A-11, A-14, A-15, A-16, and A-17, and the list part of A-6. The Evidence notes for A-6, A-8, A-13, and A-18 state the review's readings correctly.

One group of new text reads against the reference:

- STG-28, STG-29, and STG-P2 now say that a typed pointer made from a byte position can step by the size of its type, and that placement new can use the position it reaches. The reference makes that step itself undefined, because the pointer's type and the type of the byte it points to are not similar (RV7-A-10). Q-0430 reads against for the same reason.

CPY-15 keeps the known split of RV6-A-18 on the constraint check, and its Evidence note records it (RV7-A-2).

Two items add a condition:

- LOOP-9 names only the declaration in a `for` initializer. A declaration in a condition, and the hidden variables of a range-based `for`, are also skipped by a jump into the body (RV7-A-7).
- LSUP-6 says the program does not define the nothrow object. A declaration of that object at namespace scope is a definition unless the program marks it external (RV7-A-8).

LSUP-1 and Q-0431 concern a shape that the reference does not give (RV7-A-9). The doubts on DTOR-8, UNI-P3, UNI-14, REF-2, the glossary entry, and NAM-21 are small. Q-0422 is supported. Q-0424 is supported with a doubt on the wording of UNI-P3.

## Rules and principles changed after RV6

| Item | Verdict | Note |
|---|---|---|
| DTOR-6 | agrees | A decltype that gives the class type names the destructor. A decltype that gives a reference type does not. Closes RV6-A-11. |
| DTOR-8 | agrees with a doubt | RV7-A-1. The decltype sentence matches. A qualified name of a base class after the tilde is also valid. |
| CPY-15 | reads against | RV7-A-2. The cv-qualified class by value matches and closes that part of RV6-A-18. The constraint skip is the known split. |
| UNI-P1 | agrees | The trivial copy or move of the whole union carries the active member. The construction inside a member is now limited to constant evaluation. Closes RV6-A-15. |
| UNI-P3 | agrees with a doubt | RV7-A-3. The conclusion matches. The stated reason conflicts with the reference's own account of an anonymous union, and the principle omits the static assertion. |
| UNI-6 | agrees | The undefined construction inside a member that is not active is now limited to constant evaluation. Closes RV6-A-14. |
| UNI-9 | agrees | The nested anonymous union and the static assertion now match. Closes RV6-A-17. |
| UNI-14 | agrees with a doubt | RV7-A-4. The run-time sentence matches RV6-A-14. The copy sentence names only the copy forms. The doubt of RV6-A-13 stays, and the Evidence note records it. |
| CV-6 | agrees | A decltype is a third way to reach an array or function type without a declarator. Closes RV6-A-10. |
| RET-6 | agrees | The parameter case follows the implementation's choice of where a parameter ends, which VC-10 gives. A thread-local local gives an lvalue reference. Closes RV6-A-2. |
| REF-1 | agrees | The last bullet now names the right side of the binding, and the conversion function case is linked. Closes RV6-A-1. |
| REF-2 | agrees with a doubt | RV7-A-5. A non-class prvalue of a compatible type binds directly after materialization. Two other direct cases are not stated. |
| CONV-1 | agrees | The four wide character types now go on to the long types. Closes RV6-A-5. |
| NAM-21 | agrees with a doubt | RV7-A-6. The trailing requires clause matches. The splits of Q-0410 and Q-0415 stay. |
| VC-2 | agrees | Only a prvalue that is neither a class nor an array loses its cv-qualifiers. An array prvalue keeps those of its elements. |
| JMP-P3 | agrees | The move applies only to a variable with automatic storage. A static or thread-local local and a structured binding stay lvalues. |
| JMP-10 | agrees | The thread-local local now matches. Closes the rest of RV6-A-9. |

## Recorded read of the RV6 repair diff

The review read every hunk of commit 9c009448 on these files: `classes/constructors.md`, `classes/copy-move.md`, `classes/unions.md`, `declarations/declarators.md`, `declarations/functions/return-deduction.md`, `declarations/initialization/references.md`, `foundations/conversions.md`, `foundations/names.md`, `foundations/storage.md`, `foundations/translation.md`, `foundations/values.md`, `statements/jumps.md`, and `glossary.md`. The hunks on the expressions, overloading, and templates pages belong to part B.

| File | Hunk | Verdict |
|---|---|---|
| constructors.md | DTOR-8 tests and text | agrees with a doubt (RV7-A-1) |
| constructors.md | DTOR-6 text | agrees |
| constructors.md | Evidence count | no reference content |
| copy-move.md | CPY-15 tests and text | agrees on the cv-qualified class; the last sentence is unchanged and reads against (RV7-A-2) |
| copy-move.md | Evidence count | no reference content |
| copy-move.md | Divergence note on the constraint check | agrees. The note states the RV6 reading correctly and names the row that holds the split. |
| unions.md | UNI-P1 list of actions and the constant-evaluation limit | agrees |
| unions.md | UNI-P3 sentence on declared types | agrees with a doubt (RV7-A-3) |
| unions.md | UNI-9 tests, list, and static assertion | agrees |
| unions.md | UNI-14 tests, copy sentence, and run-time sentence | agrees with a doubt (RV7-A-4) |
| unions.md | UNI-6 tests and constant-evaluation limit | agrees |
| unions.md | Evidence count | no reference content |
| unions.md | Q-0424 divergence line on GCC and the nested anonymous union | agrees. GCC's acceptance is a missing diagnostic. |
| unions.md | Q-0425 divergence line on the run-time answer | agrees |
| unions.md | Split-test row for the nested anonymous union | agrees. The expected rejection matches. |
| unions.md | Note on UNI-14 and the library construction allowance | agrees. The note states RV6-A-13 correctly. |
| unions.md | Note on UNI-14 and UNI-6 at run time | agrees. The two outcomes match RV6-A-14, and both make the run test defined. |
| unions.md | Note on the trivial copy in constant evaluation | agrees. Constant evaluation treats a trivial copy or move of a union as a copy of its active member, so the target's old member is gone. |
| declarators.md | CV-6 tests and text | agrees |
| declarators.md | Evidence count | no reference content |
| return-deduction.md | RET-6 tests and text | agrees |
| return-deduction.md | Evidence count | no reference content |
| references.md | REF-1 last bullet | agrees |
| references.md | REF-1 link to the conversion function | agrees |
| references.md | REF-2 sentence on the prvalue | agrees with a doubt (RV7-A-5) |
| conversions.md | CONV-1 wide character types | agrees |
| names.md | NAM-21 tests and trailing requires clause | agrees |
| names.md | Evidence count | no reference content |
| names.md | Note on the friend and the inline namespace | agrees. The reference's own illustration rejects that form when nothing else in scope declares the name. |
| storage.md | STG-6 tests gain the union run test | agrees. The run-time outcome of RV6-A-14 reuses the union's storage in one of its two forms. |
| storage.md | STG-P2 table row for a union with an array member | agrees with a doubt. The union's initializer and an assignment match. The construction of the whole member through the library function shares the doubt of RV6-A-13. The last column now says that each form builds every element, which matches. |
| storage.md | Evidence count | no reference content |
| translation.md | Note on TR-P1 | agrees. The note states RV6-A-8 correctly and points to the page that records the choice to declare names in the standard namespace. |
| values.md | VC-2 tests and text | agrees |
| values.md | Evidence count | no reference content |
| glossary.md | Entry for a direct binding | agrees with a doubt (RV7-A-5) |
| jumps.md | JMP-P3 sentences on automatic storage | agrees |
| jumps.md | JMP-10 tests and the thread-local bullet | agrees |
| jumps.md | Evidence count | no reference content |

## Text that phase 14 changed

| Item | Verdict | Note |
|---|---|---|
| DCL-19 | agrees | A constant integral variable with a constant initializer can serve in a bound, a static member among them. |
| DCMP-P1 | agrees | Only special member functions and comparison operators can be defaulted. A hash has no defaulted form, so the class writes it. |
| IF-12 | agrees | The link to LOOP-9 adds no claim. |
| LOOP-9 | adds a condition | RV7-A-7. The flow after the jump matches. Other declarations that a jump into the body skips are not named. |
| LOOP-P1 | agrees | The diagram follows LOOP-9. The variable of the loop must already hold a value at the jump, as the diagram assumes. |
| LSUP-1 | reference silent | RV7-A-9. The reference gives no layout, and it needs a library declaration of the template to be reachable. |
| LSUP-6 | adds a condition | RV7-A-8. The library defines the object, which matches. The program's own declaration must not define it. |
| STG-28 | reads against | RV7-A-10. The new sentence on a typed pointer made from a byte offset. |
| STG-29 | reads against | RV7-A-10. The new sentence on the position one past the byte. |
| STG-P2 | reads against | RV7-A-10. The new sentence on the typed step. The rest of the principle agrees. |
| SW-1 | agrees | The link to LOOP-9 adds no claim. |

The F4 commit adds no example to the comparisons page or to the storage page. Its text there is the DCMP-P1 sentence and the STG-P2 sentence above. The F4 examples near these pages sit on expression pages: the comparison of two character pointers after PRIM-9, the wrapping hash, and the two owners in the new-delete page. Each agrees with the reference in a quick read. Equal literals may or may not share one object, unsigned arithmetic wraps, and the conversion back to a signed type keeps the value modulo the width. Part B reads those pages in full.

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0422 | supports | RV7-A-11. JMP-P3 now decides the question. A structured binding is no variable, so its name stays an lvalue. |
| Q-0424 | supports with a doubt | RV7-A-3. UNI-P3 now decides the question in the reference's direction. Its stated reason needs care. |
| Q-0430 | reads against | RV7-A-10. The reference makes the step undefined. Option 3 is closest. |
| Q-0431 | silent | RV7-A-9. The reference gives no shape for the template. It requires the library's own declaration. |

## RV7-A-1

DTOR-8 now says that a decltype which gives a reference type does not denote the class. That matches. The reference requires the name after the tilde to denote the class whose destructor runs. A reference type is a different type.

DTOR-8 still says that the name must denote the type of the object. The reference also allows a call that names the destructor of a base class of the object's type. With a qualifier, the call runs the base destructor alone and does not dispatch. The reference shows such calls on a derived object. Read strictly, DTOR-8 rejects them. No other rule on the page states the base case.

## RV7-A-2

CPY-15 now covers a first parameter of the class type with any cv-qualifiers. That matches the reference and closes the first part of RV6-A-18.

The last sentence still lets CNS-11 skip the constraint check of a constructor template. The reference has no such skip, as RV6-A-18 said. The Divergences note now records that the reviews read the C++23 text as EDG and Clang 18 do, and that pending.md row 4 holds the case. The note is correct. The verdict stays reads against for that sentence.

## RV7-A-3

UNI-P3 now says that an anonymous union declares no type, and draws from that the rejection of an unnamed class defined by a member declaration and of a nested anonymous union. Both conclusions match the reference. Nested types, anonymous unions, and functions may not be declared inside an anonymous union, and a diagnostic is required.

The reason given conflicts with the reference's own account. In the reference, the anonymous union itself defines a type with no name, and an object of that type. The restriction falls on declarations inside the union. A reader who takes the principle's sentence literally may conclude that the anonymous union's own type does not exist. The meaning the principle wants is that nothing inside the union may introduce a type.

UNI-P3 also says that an anonymous union holds only plain public data members. The reference also allows a static assertion there, as UNI-9 now says.

## RV7-A-4

UNI-14's run-time sentence matches RV6-A-14. The construction is defined, and the program can use the new object through the pointer that the construction returns.

The new copy sentence names the trivial copy constructor and copy assignment. The reference treats the move constructor and the move assignment of a union the same way. At run time, the implicit copy or move of a union copies its bytes and creates in the target an object for each object nested in the source. During constant evaluation, a trivial copy or move counts as a copy or move of the active member. UNI-P1 names the move, and UNI-14 does not. A copy assignment to the union itself creates nothing new, which neither rule needs to say.

The doubt of RV6-A-13 stays. The allowance that lets constant evaluation reduce the library's construction function names storage from the library allocator or an object whose lifetime began in the evaluation. A member that is not active has not begun. The Evidence note records this reading and the compilers' wider one.

## RV7-A-5

REF-2 now says that a prvalue of a type that REF-1 admits becomes a temporary, and that the reference binds directly to it. That matches the reference. A non-class prvalue of a compatible type is adjusted to the reference's cv-qualifiers, materialized, and bound, and the binding counts as direct.

Two direct cases remain unstated in REF-2:

- An xvalue binds directly whenever its type is compatible in the sense of REF-1. REF-2 says only an xvalue of the same type. An xvalue of a derived class, or of a pointer type that a qualification adjustment reaches, also binds directly.
- A class rvalue whose class is not related to the referenced type can convert through a conversion function to an rvalue of a compatible type. The reference binds directly to that result. REF-2 says only "a class rvalue", and its last sentence would place this case among the temporaries that hold a converted value. ICS-11 covers the case, and REF-2 does not link it.

The glossary entry now matches for the materialized prvalue. The reference defines a direct binding by exclusion: every case except the one that converts the initializer to the referenced type and binds to a new temporary. The entry's first sentence speaks of the object that the initializer names. The result of a conversion function is not named by the initializer, yet the binding to it is direct. The entry's list of rules does not include that case.

## RV7-A-6

NAM-21 now lists the trailing requires clause. The reference places every part of a redeclaration after the qualified name inside the named scope, and the requires clause is such a part. This closes the list part of RV6-A-6.

The splits stay. For a qualified enumeration definition, the reference's rule covers the part after the head name, which includes the underlying type. That is Q-0410. The friend note on inline namespaces states the reading of RV6-A-6 correctly. A search in the named scope does not enter its inline namespaces, and the reference's illustration rejects the form when no other declaration is in scope.

## RV7-A-7

LOOP-9 says that a jump into a loop body runs the rest of the body, then the iteration expression of a `for`, then the condition. That matches. The reference describes each loop by an equivalent form in which the body is followed by the step and a return to the test. A jump to a label inside the body joins that flow. The initializer of a `for` lies before the loop and does not run.

LOOP-9 names only a declaration in the `for` initializer as subject to JMP-6. The reference's check covers every automatic variable that is in scope at the target and not at the source. Two more kinds are in scope inside a body:

- A variable declared in the condition of a `while` or a `for`. Such a declaration always has an initializer, so a jump past it is not valid.
- The hidden variables that a range-based `for` declares for its range and its iterators. They are initialized, so a jump into the body of a range-based `for` is not valid.

LOOP-9 does not say whether it covers the range-based form.

## RV7-A-8

LSUP-6 now says that the library defines the nothrow forms and the nothrow object, and that the program does not define that object. The first part matches. The library declares the object as an external constant and defines it.

The table gives the object's shape as an object of the constant tag type. A declaration of a variable at namespace scope is a definition unless it is marked external and has no initializer. A constant object at namespace scope without that mark also has internal linkage. So a declaration in the table's shape, written without the external mark, defines a separate object of the program's own. LSUP-6 needs the program's declaration to be external and non-defining for its last sentence to hold. The test for LSUP-6 already uses that form.

## RV7-A-9

LSUP-1 gives a fixed shape for a program's own declaration of the initializer list template: a pointer, then a count of the size type, and a constructor of any access. Q-0431 asks whether another order or count type must be rejected.

The reference gives no layout. Its library description says only that an object of the template gives access to an array of constant elements, and it mentions two obvious representations as a note. The language rule requires that a declaration of the template from the standard library be reachable at each use, and it makes a program without one not valid. A program that declares the template itself, in any shape, adds to the standard namespace, which the reference does not allow. So the reference is silent on which shapes work. The rejection of other shapes by Clang and GCC, and the acceptance by EDG, are both outside what the reference decides. LSUP-1 and Q-0431 record Tidal's own choice, as the language-support page already says for names in the standard namespace.

## RV7-A-10

The new sentences of STG-28, STG-29, and STG-P2, with the provisional answer of Q-0430, say three things:

- A typed pointer cast from the address of a byte steps by the size of its type to the position one past the byte.
- Placement new can build an object at that position.
- A typed pointer made from the result of an allocation function plus a multiple of the type's size in bytes points to the element at that address.

The reference reads against all three.

A cast through a void pointer changes the pointer value only when an object of the target type is pointer-interconvertible with the object pointed to. A byte of a byte array and an object nested in the array are not pointer-interconvertible. So the cast gives a typed pointer that still points to the byte. STG-29 already says this.

For addition and subtraction on a pointer, the reference makes the behavior undefined when the pointer's type and the element type of the array that holds the pointed-to object are not similar. A byte of a byte array is an element of that array, of a byte type. A typed pointer to it fails that test. So any addition to it is undefined, including a step of one. The rule that counts a lone object as an array of one does not help, since the byte is already an array element. The reference gives the step no address. Q-0430's option 3 is the closest of its options. The reference makes the computation itself undefined, and with it every later use of the result.

The same holds after an allocation function. The function creates objects implicitly and returns a pointer to one created object at the start of the storage. A program that does byte arithmetic needs that object to be a byte array. The typed pointer cast from a byte position then points to a byte, and a step on it is undefined. If the returned object is instead an array of the element type, the byte pointer points to that array and its byte arithmetic is undefined. No choice of created objects gives both operations defined behavior. STG-28's earlier sentence, that a typed pointer cast straight from the result points to element 0, is not affected.

The reference gives a defined path to the same address. Arithmetic on the byte pointer itself is defined inside the byte array or the allocated bytes. Placement new at the address that this arithmetic reaches is defined, given size and alignment. The pointer that placement new returns reaches the new object. The compilers accept the typed step and no sanitizer stops it, which explains the matrix results. ADD-14's base, which counts a byte as an array of one for a pointer of another type, is on an expression page, and part B reads it.

## RV7-A-11

Q-0422: JMP-P3 now says that the move applies only to a variable with automatic storage and that a structured binding is no variable. The reference treats a structured binding as its own kind of entity, and the rule that makes a returned name an xvalue names only such variables. So JMP-P3 decides the question for option 1, and the reference supports that answer with no doubt. The same holds for a binding to data members and for the tuple-like form.

Q-0424: UNI-P3 now rules out option 2, as RV7-A-3 reads. The reference supports option 1. The doubt concerns only the principle's stated reason, which a reader might misread as a claim about the anonymous union's own type.
