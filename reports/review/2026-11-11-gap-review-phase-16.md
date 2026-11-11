# Gap review: phase 16 text that no source reader has read (RV9)

Date: 2026-11-11

Role: source reader

Reference: the C++23 working draft sources.

Commit read: 1cc1696e380ebcdcc68b878515cd4e3ab66836e7.

The review is item RV9 of decision 0064. It covers four groups of text:

- FN-12's new sentence and its Evidence note, and INST-1's new clause that cites FN-12 (commit 4431852f), with the two tests the hunks cite,
- every hunk of commit bdc27d44 under `specification/`, and the three test notes that the commit changed,
- the text that item F6 changed in commit b3f17197: TYN-6, TYN-10, the shape `specialization_class`, OPR-8, LSUP-10, LSUP-P3, UNI-14, CE-21, CE-22, CE-P3, OBJ-1's array row, STG-P2 and the label of the storage-form table, the glossary entry for lifetime, and the examples on the range-for, concepts, and storage pages,
- first readings of Q-0447 and Q-0448, and a new reading of Q-0445 after F6 changed TYN-10.

The read of the RV8 repair diff used points of the form RV9-R-n. This review does not repeat those points. New points carry IDs of the form RV9-n.

## Result

Most of the text agrees with the reference. The bdc27d44 hunks agree, and each now states the reading of the read of the RV8 repair diff correctly.

Two items read against the reference. Both are known splits that the text records:

- OPR-8's citation of PRIM-15, the Q-0063 line on the operators page, and `literal-float-form-out-of-range.cpp`. The reference makes a floating literal outside its type's range not valid. A literal with a suffix that calls the `long double` form is treated as a call with the plain `long double` literal as its argument, so that literal must also be in range (RV9-6). This is the reading that the earlier review of Q-0063 gave.
- LSUP-10's new third place, a member of a union that is not active. The reference allows the library construction function in constant evaluation only on storage that the library allocator gave or on an object that started to live during the evaluation. A member that is not active has not begun (RV9-7). The Evidence note says so.

Three items add a condition:

- INST-1's new clause. Tidal's own INST-30 and the reference put the point of instantiation of a class specialization before the namespace-scope declaration that needs it. For the run test, that point is before the enclosing class, where the class is still incomplete. The clause and INST-30 now give two places for the same case (RV9-2).
- OBJ-1's array row and the glossary entry for lifetime. An array's lifetime also waits for its own initialization to complete, and it also ends when the array is destroyed (RV9-11).
- CE-22's new sentence leaves out one limit that the reference puts on a union's implicit copy: a union whose active member is `mutable` (RV9-10).

FN-12 agrees with two small doubts (RV9-1). TYN-6, TYN-10, and the new shape agree with a doubt on the qualified form (RV9-4). UNI-14's new sentences agree (RV9-8). CE-21 agrees with a doubt on how the reference treats a part whose lifetime ended (RV9-9). The examples agree, two with a small doubt (RV9-12, RV9-13, RV9-14).

Q-0445, Q-0447, and Q-0448 are each supported with a doubt.

## FN-12 and INST-1 (commit 4431852f)

| Item | Verdict | Note |
|---|---|---|
| FN-12, the sentence on a member function defined in its class | agrees with a doubt | RV9-1. The reference checks the types where the body is, and the body is a place where the class counts as complete. |
| FN-12 Evidence note | agrees | The grid's cases match. The `sizeof` case falls in a member declaration, where the class is incomplete (CLS-22). |
| INST-1, the clause that cites FN-12 | adds a condition | RV9-2. The clause disagrees with INST-30 as written. |
| `member-in-class-specialization-over-class.cpp` | agrees with a doubt | RV9-2. The expected run follows every compiler. The letter of the point of instantiation reads against it. |
| `member-in-class-type-completed-after-class.cpp` | agrees | The type is incomplete at the end of the class and inside the body, so the definition is not valid. |

## Commit bdc27d44

| Item | Verdict | Note |
|---|---|---|
| constructors.md, the Q-0444 line | agrees | RV9-3. |
| `dtor-call-unqualified-base.cpp`, the note | agrees | RV9-3. |
| instantiation.md, the INST-1, INST-2, INST-P1 note | agrees | RV9-3. |
| `class-decltype-consteval-call-not-instantiated.cpp`, the note | agrees | RV9-3. |
| `integer-counterpart-one-past.cpp`, the note | agrees | RV9-3. The expected run stays the split of RV8-R-9 and RV9-R-3. |

## Text that item F6 changed

| Item | Verdict | Note |
|---|---|---|
| shape `specialization_class` and its paragraph | agrees with a doubt | RV9-4. The optional qualifier has no clear support in the reference. |
| TYN-6, the specialization sentence | agrees with a doubt | RV9-4. The same doubt on the qualified form. |
| TYN-10, the attribute item | agrees | RV9-4. The reference forbids an attribute on an explicit instantiation outright. |
| TYN Evidence note on a template-id with no template head | agrees | RV9-4. The reference rejects the form. No TYN rule lists it. |
| OPR-8, the value of the numeric form | agrees | RV9-5. |
| OPR-8, the call of the template by its name | agrees | RV9-5. |
| OPR-8, the citation of PRIM-15, and the Q-0063 line | reads against | RV9-6. |
| OPR example, the two new declarations | agrees | RV9-5. |
| LSUP-10, the third place | reads against | RV9-7. The known split of RV6-A-13 and RV8-A-8. |
| LSUP-P3, the link to the table | agrees | No claim of its own. |
| UNI-14, the run-time sentences | agrees | RV9-8. |
| unions page, the FixedStack paragraph link | agrees | No claim of its own. |
| CE-21, the ended part | agrees with a doubt | RV9-9. |
| CE-22 and CE-P3, the union copy | agrees with a doubt | RV9-10. CE-22 also misses the `mutable` limit. |
| CE examples, the union copy and the ended element | agrees | RV9-9, RV9-10. |
| OBJ-1, the array row | adds a condition | RV9-11. |
| OBJ Evidence note on the array row | agrees | No program observes the start of the array apart from its elements in the cases that STG-28 gives. |
| glossary, lifetime | adds a condition | RV9-11. The class and scalar sentences agree. The array sentence has the same gap as OBJ-1. |
| STG-P2, the link to the table | agrees | The sentence on the typed step from a byte is unchanged and keeps the split of RV7-A-10 and Q-0430. |
| storage-form table label | agrees | A label has no reference content. |
| storage page, the Maybe example and the sentence after it | agrees with a doubt | RV9-12. |
| range-for page, the adaptor example | agrees | RV9-13. |
| concepts page, the iterator-type example | agrees with a doubt | RV9-14. |

## Questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0445 | supports with a doubt | RV9-4. Only a declaration that holds the elaborated type alone may carry the attribute. The template-id form inside a specialization is one such declaration, and an explicit instantiation may carry none. The doubt is the qualified form. |
| Q-0447 | supports with a doubt | RV9-9. Option 1 matches the reference's intent. The reference does not say outright that a scalar whose lifetime ended has no value. The table's last row also holds a case the reference decides. |
| Q-0448 | supports with a doubt | RV9-10. Option 1 matches. The reference does not say whether the copy of the active member reads its parts. |

## RV9-1

FN-12, the new sentence on a member function defined in its class.

The reference forbids an incomplete class as a parameter type or the return type of a function definition, with the check made inside the function body. A deleted definition is exempt. A member function's body is one of the places where its class counts as complete. So for a member function defined in its class, the types need to be complete only where the class is complete, and FN-12's sentence states the effect. The reject test has a type that is still incomplete there, so the definition is not valid.

Two small doubts:

- A function body inside a nested class that is defined inside its enclosing class also counts as a place where the enclosing class is complete. So for a member function of such a nested class, the check falls at the end of the outermost enclosing class, which the sentence does not say. The second case of the Evidence grid is consistent with this.
- A friend function defined inside the class has its body inside the class too, so the same rule reaches it. The sentence names only a member function.

FN-12 does not name an abstract class. The reference forbids that too in a definition, and VIRT already holds the case with its own tests.

## RV9-2

INST-1, the clause that the definition of a member function defined in its class instantiates the class at the end of that class, and `member-in-class-specialization-over-class.cpp`.

In the reference, the point of instantiation of a class specialization that a use outside any template needs is right before the namespace-scope declaration that holds the use. INST-30 says the same. For the run test, the use is in a body inside the class, and the namespace-scope declaration that holds it is the enclosing class itself. So by that rule the specialization's point is before the class, where the class is incomplete. A member of the class's own type in the specialization would then make the instantiation fail.

Every compiler instead instantiates the specialization when it processes the body, after the class is complete. The reference does not reconcile its rule on complete classes in a body with its point of instantiation for a class specialization, so it is not consistent on this case. FN-12's sentence follows the first rule, and the compilers follow it too.

The condition is in Tidal's own text. INST-1's clause and INST-30 now give two different places for the same specialization. One of them needs to state the exception, or the clause stands against INST-30.

## RV9-3

The hunks of bdc27d44.

- The Q-0444 line and the note of `dtor-call-unqualified-base.cpp`. The reference looks up a name after a tilde in a member access with the object's type as its context, and requires the name to refer to that type, cv-qualifiers aside. On a derived object, the base's name fails this, so the call is not valid, as Clang and EDG say. The line and the note now state the reading of RV9-R-1 and no longer lean toward GCC.
- The INST note and the note of `class-decltype-consteval-call-not-instantiated.cpp`. The reference counts an invocation of an immediate function as an immediate invocation only when the invocation is potentially evaluated. The operand of `decltype` is not evaluated, so the call is no immediate invocation. The special case of `decltype` for a call that returns a class then needs no complete class. The `sizeof` control needs a complete type for a separate reason. Both notes agree.
- The note of `integer-counterpart-one-past.cpp`. The reference makes addition and subtraction undefined when the pointer's type and the array's element type are not similar. Subtraction of the two pointers falls under the same rule, so the difference is undefined too, as the note now says. The read through the original pointer stays valid. The test's expected run is the known split.

## RV9-4

TYN-6, TYN-10, the shape `specialization_class`, the TYN Evidence notes, the three new tests, and Q-0445.

The reference allows a declaration that consists only of an elaborated type in three cases: an explicit specialization, an explicit instantiation, and two listed forms. The first listed form is the forward declaration that ends in an identifier. The second ends in a template-id with no qualifier, and the reference permits it only inside an explicit specialization or a partial specialization. Both listed forms show an attribute after the class key. In every other use, an elaborated type may carry no attribute. An alignment specifier is a kind of attribute specifier in the reference, so it follows the same rules.

- TYN-10's item now allows the attribute in `forward_class` and in `specialization_class`, and rejects it elsewhere. This matches, and it covers the friend form of Q-0445's table, since the reference's friend forms have no attribute place.
- The reference states separately that no attribute may belong to an explicit instantiation. So TYN-10's last sentence and `explicit-instantiation-attribute.cpp` agree, and GCC reads against.
- A class key with a template-id and no template head is not valid in the reference, because the second form is allowed only inside a specialization. The Evidence note agrees. No TYN rule lists the case, and the shape paragraph says only that the declaration follows a template head.

The doubt is the qualified form. The shape allows a qualifier before the template-id, and TYN-6, the Evidence note, and `explicit-specialization-attributes.cpp` accept an attribute with a qualified name. The reference exempts an explicit specialization as a whole, so a qualified explicit specialization with no body is valid. But the reference's grammar for an elaborated type gives an attribute place only before a plain identifier, with or without a qualifier, and never before a template-id. Its second listed form shows the attribute only with an unqualified template-id. A qualified partial specialization with no body fits none of the listed forms, with or without an attribute. Every compiler accepts the qualified forms that the tests hold. The read of the RV8 repair diff found the grammar gap (RV9-R-4), and this review finds that the qualified case is the part with least support. The doubt is small, since the reference is not consistent and all compilers agree.

Q-0445 is supported with this doubt.

## RV9-5

OPR-8, the value that the numeric form receives, the call by name, and the example.

For an integer literal with a suffix, the reference calls the `unsigned long long` form with an argument that is the same characters, without the suffix, written as a plain literal of that type. For a floating literal it does the same with a plain `long double` literal. So the numeric form receives the value that the literal without its suffix has as a literal of that type, as the new sentence says. The rounding of a floating value is the implementation's choice between the two nearest values, which PRIM-21 gives. The values in the Evidence note and `literal-numeric-form-value.cpp` follow.

A literal operator template is a function template, and a program can call it by its name with explicit template arguments. An empty argument list gives an empty pack. The sentence and `literal-operator-template-called-by-name-value.cpp` agree.

The example's first operator rounds the product to the nearest whole number, and the result is 1999 whichever way the implementation rounds the literal. The second counts its characters, so the suffixed literal and the call by name give the same count.

## RV9-6

OPR-8's citation of PRIM-15, the Q-0063 line on the operators page, the new split row, and `literal-float-form-out-of-range.cpp`.

The reference makes a floating literal whose scaled value is outside the range of its type not valid. It is not conditionally supported. A suffixed floating literal that calls the `long double` form is treated as a call whose argument is the plain `long double` literal, so the same rule applies to that argument, and the program is not valid. This matches the reasoning that RV8-A-5 gave for an integer value too large for its type.

So the test's expected acceptance, PRIM-15's class, and the Q-0063 line read against the reference. A compiler that accepts such a literal with a warning has given a diagnostic, and the reference asks for no more. The line's sentence that each result conforms therefore holds only for an accepting compiler that warns. The raw form and the template need no value, and the Evidence note says that every compiler accepts them, which agrees. The earlier gap review of Q-0063 gave the same reading, and this review adds the suffixed case to it.

## RV9-7

LSUP-10, the third place for the placement `new` inside `std::construct_at` during constant evaluation, and the Evidence note.

The reference treats a call of the library construction function as only its constructor call when the pointer reaches storage that the library allocator gave, or an object that started to live during the evaluation. Otherwise the placement form inside it disqualifies the evaluation. A member of a union that is not active has not begun its lifetime, so the new bullet reads against the reference. The Evidence note states this correctly, and LSUP-10 follows the compilers, as UNI-14 does. The verdicts of RV6-A-13 and RV8-A-8 stand.

## RV9-8

UNI-14, the new sentences on the enclosing member at run time.

In the reference, an object created at the storage of an array element becomes a subobject of the array only if the array's lifetime has begun. An array member that is not active has not begun. So the new object is not part of the member, and the member stays inactive at run time as well. A placement call implicitly creates objects only in the region it returns, which is one element's storage, so no implicit creation can start the whole member. A read through the member's name then reaches an element of an array that is not alive, and that read is undefined, as the sentence says by way of UNI-6.

A member that was active ends only if the new object reuses storage that the member occupies, which STG-6 gives. Reuse of part of the member's storage also ends it. The sentence's word "takes" covers that if STG-6 is read so. The Evidence note reports the run probe as a fact and draws the right conclusion that no test can hold the undefined read.

## RV9-9

CE-21, the ended part, the example with the ended element, the two new tests, and Q-0447.

The reference requires each scalar subobject of a constant's value to have a value that is not indeterminate. It allows a pseudo-destructor call in constant evaluation on an object whose lifetime began in the evaluation, and such a call ends the scalar's lifetime. The reference does not say outright what value a scalar has after its lifetime ends. It does say that an object's properties apply only during its lifetime, so an ended scalar has no value that the constraint could accept. So CE-21's new item matches the reference's intent, and both tests agree. The doubt is that the reference reaches this result only through that general statement.

An element of class type that a destructor call ended follows the same reading, since the constraint applies to each scalar part.

The last row of Q-0447's table holds a case that the reference decides. An assignment to a scalar after its lifetime ended uses the object outside its lifetime, and the reference makes that undefined. No operation in that row creates a new object there. An evaluation with undefined behavior is not constant, so Clang's rejection matches. GCC 16 and EDG read against. No rule of the constant-evaluation page names the case, as the question says.

Q-0447 is supported with the doubt above.

## RV9-10

CE-22 and CE-P3, the trivial copy of a union, the example, the two new tests, and Q-0448.

At run time, the reference makes the implicit copy of a union copy its object representation. For constant evaluation, the reference treats a trivial copy or move constructor or assignment of a union as a copy of its active member, if any, and notes that this copy is trivial. It says no more on whether the copy of the active member reads that member's scalar parts. If the copy of an active member that is an `int` array were read as a copy of each element, the read of the element with no value would make the copy itself not constant, and every compiler would read against. Read as a copy of the representation of the active member, the copy reads nothing, and a part with no value in the source has none in the target. CE-22's sentence and option 1 of Q-0448 take the second reading, and Clang and GCC agree. The doubt is that the reference does not choose between the two readings in words.

CE-22's sentence misses one condition. The reference also disqualifies a call of the implicit copy or move constructor or assignment of a union whose active member is `mutable`, unless the union's lifetime began within the evaluation. No rule of the page names the case.

The CE example agrees. An assignment to an element of the array member through the union's name starts the member's lifetime, and the other elements then exist with no value. The copy reads nothing, and the read of the second element of the copy is not constant.

Q-0448 is supported with the doubt above.

## RV9-11

OBJ-1's new row for an array, and the glossary entry for lifetime.

An array is not a class, so the reference's rule for an object that is not a class applies. The lifetime begins when storage of the right size and alignment is obtained and the array's initialization, if any, is complete. Vacuous initialization counts. The lifetime ends when the array is destroyed, or when its storage is released or reused by an object not nested in it.

- The start column names only the storage and the operations of STG-8. For an array with an initializer, or an array of a class with a constructor that does work, the array's own lifetime begins only when the initialization of its last element completes. So the array begins after its elements in those cases. The clause that an element can begin later than the array is right for the cases that STG-28 gives, where an operation creates the array and an element of a type without implicit lifetime is built later.
- The end column omits destruction. An array with automatic storage, or one that the array form of `delete` ends, is destroyed before its storage goes away.

The glossary entry's array sentence has the same gap. Its class sentence, which now ends the lifetime at the start of the destructor call, and its scalar sentence agree with OBJ-1 and with the reference.

## RV9-12

The Maybe example on the storage page and the sentence after it.

In the reference, a class with a user-declared copy constructor gets no implicit move constructor, and a deleted copy constructor counts as user-declared. An rvalue argument then selects the copy constructor, which binds the rvalue, and the selected function is deleted. So without its own move constructor, the class cannot be returned from a local variable, while a prvalue return needs no constructor at all. The sentence, the citations of CPY-5 and CPY-P2, and the Evidence note agree. The new move constructor builds the value in its own bytes and keeps the pointer that placement `new` returns, which matches STG-5.

The doubt is small. The sentence says that the class writes its own copy and move functions. The class deletes both copy functions, writes a move constructor, and declares no move assignment. A move assignment from an rvalue then selects the deleted copy assignment, so the class cannot be move-assigned. The sentence reads as if it could.

## RV9-13

The adaptor example under RFOR-2 and `filter-adaptor-any-range.cpp`.

The adaptor is an aggregate with two members of the template's two parameter types. The reference's deduction for an aggregate gives one parameter from the range object and one from the closure, which is a class type of its own. The nested iterator type is a member type, so the adaptor stays an aggregate. The aliases name the range's iterator and end types through a function that is only declared, inside an unevaluated operand, so the function needs no definition.

The loop binds the adaptor prvalue to a reference that extends its life until the loop ends, calls each of the member functions once, and compares the iterator with the end object through the iterator's member. With the range of the earlier example, the squares are 1, 4, 9, and 16, the even ones are 4 and 16, and the sum is 20. For the range whose iterator has only an equality member, the inequality test inside the adaptor uses the rewritten form of the equality member, which the reference allows. The test's values agree.

## RV9-14

The concepts example under CPT-28 and `requires-parameter-iterator-type.cpp`.

A parameter of a requires expression has no storage and no lifetime, so no iterator is made, as the example says. For a range whose alias is valid, each requirement is valid, and the concept holds.

The doubt is small. For a type whose alias is not valid, the failure lies in the parameter list. The reference's sentence that makes a requires expression false names a failure in its requirements, not in its parameter list. The concept's constraint is the requires expression itself, and the reference makes a constraint not satisfied when substitution into it forms a type that is not valid. So the concept is false for that type by another route, and the example and the test agree. The class in the test whose begin member returns an integer fails at the dereference requirement, which also makes the concept false.
