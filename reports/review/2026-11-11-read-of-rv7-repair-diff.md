# Read of the RV7 repair diff (b5d925e0)

Date: 2026-11-11

Role: source reader

Reference: the C++23 working draft sources.

Commit read: b5d925e0ec9b36f89ba7306defa145d86c4c4cc3.

The read covers every hunk of commit b5d925e0 under `specification/`, with the two RV7 reviews that the commit repairs (`reports/review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md` and `reports/review/2026-11-11-gap-review-phase-14-expressions-overloading-templates.md`). The reader also read the new and changed tests that the hunks cite. The read is item RV8, first part, of decision 0063. New points carry IDs of the form RV8-R-n.

## Result

Most hunks agree with the reference. The repairs close RV7-A-1, A-3, A-4, A-5, A-7, A-8, B-2, B-5, B-6, B-7, B-12, B-13, B-15, and B-16. RV7-B-10 is closed as a conflict inside the text. The reading of Q-0225 that RV7-B-10 named stays in `pending.md` row 4.

The text keeps the compilers' answer, with an Evidence note, for RV7-A-2, A-10, B-1, B-3, and B-4. Those notes state the reviews' readings correctly, except one. The Q-0049 note and the row 4 item it names carry a reading of RV7-B-3 that is too wide (RV8-R-9). For a pointer to a base class subobject, the reference agrees with ADD-14 and the compilers.

One hunk adds a condition. INST-1, INST-2, and INST-P1 still leave out the call that is an immediate invocation (RV8-R-4).

The other new points are doubts.

## Hunks

| File | Hunk | Rule or principle | RV7 point | Verdict |
|---|---|---|---|---|
| constructors.md | tests line and the sentence on a qualified base name | DTOR-8 | A-1 | agrees with a doubt (RV8-R-1) |
| constructors.md | Evidence count | none | none | no reference content |
| copy-move.md | divergence note names the part A review | CPY-15 | A-2 | agrees. The note states the reading of RV7-A-2 correctly. |
| unions.md | static assertions, the union as an unnamed type, no type declared inside | UNI-P3 | A-3 | agrees |
| unions.md | tests line and the move forms | UNI-14 | A-4 | agrees |
| unions.md | Evidence count | none | none | no reference content |
| references.md | tests line and the list of three direct cases | REF-2 | A-5 | agrees with a doubt (RV8-R-2) |
| references.md | Evidence count | none | none | no reference content |
| references.md | note on the two direct cases and the pointer form | REF-2 | A-5 | agrees. A pointer xvalue bound to a reference to a more qualified pointer also binds directly in the reference. The note says only that no test checks that form, which is a fact about the tests. |
| additive.md | Q-0049 line gains the review's reading | ADD-14 | B-3 | reads against (RV8-R-9) |
| additive.md | Q-0430 line names both reviews | ADD-14 | A-10, B-3 | agrees |
| additive.md | Evidence note on the typed step and the array of one | ADD-14 | B-3 | agrees with a doubt (RV8-R-9). The byte case is stated correctly. The array of one is read too widely. |
| new-delete.md | Evidence note on the throw path | NEW-22 | B-4 | agrees. The note states the reading of RV7-B-4 correctly. |
| language-support.md | the program's declaration is external with no initializer | LSUP-6 | A-8 | agrees |
| language-support.md | Q-0431 line on the silent reference | LSUP-1 | A-9 | agrees |
| storage.md | STG-P2 cites Q-0430 | STG-P2 | A-10 | agrees. The citation adds no claim. |
| storage.md | Q-0430 line names both reviews | STG-28, STG-29 | A-10 | agrees |
| storage.md | Evidence note on the typed step and the defined path | STG-28, STG-29, STG-P2 | A-10, B-3 | agrees with a doubt (RV8-R-10) |
| glossary.md | entry for a direct binding gains the conversion function result | glossary, ICS-11 | A-5 | agrees with a doubt (RV8-R-2) |
| conversion-sequences.md | tests line | ICS-11 | A-5 | agrees. The new test binds to the object that the conversion function returns, which is a direct binding. |
| conversion-sequences.md | Evidence count | none | none | no reference content |
| jumps.md | tests line | JMP-6 | A-7 | agrees |
| jumps.md | Evidence count | none | none | no reference content |
| loops.md | tests line and the list of declarations in scope in the body | LOOP-9 | A-7 | agrees with a doubt (RV8-R-3) |
| constraints.md | table row for an independent template-id inside a template | CNS-13, DEP-30 | B-2 | agrees |
| constraints.md | first bullet of CNS-13 | CNS-13 | B-2 | agrees |
| constraints.md | Evidence note on the case | CNS-13 | B-2 | agrees |
| deduction.md | tests line and the sentence on a pack of the enclosing class | DED-20 | B-16 | agrees |
| deduction.md | Q-0429 divergence line | DED-20 | B-16 | agrees |
| dependent-names.md | lead sentence of the call table | DEP-P1 | B-13 | agrees with a doubt (RV8-R-7) |
| dependent-names.md | sentence on the middle column | DEP-P5 | B-10 | agrees as text. The row that it now matches reads against C++23, as RV7-B-10 said. |
| dependent-names.md | DEP-P6 rows for DEP-6, DEP-17, and CNS-13 | DEP-P6 | B-2, B-5 | agrees with a doubt (RV8-R-8) |
| dependent-names.md | the value-dependent list gains typeid | DEP-4 | B-12 | agrees |
| dependent-names.md | the exception for DEP-17 | DEP-6 | B-5 | agrees with a doubt (RV8-R-8) |
| dependent-names.md | tests line | DEP-7 | B-7 | agrees |
| dependent-names.md | tests line and the split by kind of qualifier | DEP-40 | B-7 | agrees with a doubt (RV8-R-6) |
| dependent-names.md | tests line and two new bullets | DEP-30 | B-2, B-5 | agrees |
| dependent-names.md | Evidence count | none | none | no reference content |
| dependent-names.md | Q-0226 line names the review and row 4 | DEP-P5 | B-1 | agrees. The note states the reading of RV7-B-1 correctly. |
| dependent-names.md | Evidence note on the two cases now under DEP-30 | DEP-6, DEP-30, CNS-13 | B-2, B-5 | agrees |
| dependent-names.md | Evidence note on typeid | DEP-4 | B-12 | agrees |
| instantiation.md | INST-P1 class row | INST-P1 | B-6 | adds a condition (RV8-R-4) |
| instantiation.md | tests line and the decltype bullet | INST-1 | B-6 | adds a condition (RV8-R-4) |
| instantiation.md | tests line and the whole-operand sentence | INST-2 | B-6 | adds a condition (RV8-R-4), with a doubt (RV8-R-5) |
| instantiation.md | tests line | INST-6 | B-6 | agrees |
| instantiation.md | the INST-33 reject example annotation | INST-32, INST-33 | B-15 | agrees |
| instantiation.md | Evidence count | none | none | no reference content |

## RV7 points

| Point | Closed | Note |
|---|---|---|
| RV7-A-1 | yes | The qualified base form is now stated. The unqualified form is RV8-R-1. |
| RV7-A-2 | no | The text keeps the compilers' answer. The note and row 4 record the review. |
| RV7-A-3 | yes | The reason now matches the reference, and the static assertion is named. |
| RV7-A-4 | yes | The move forms are named. The doubt of RV6-A-13 stays, as RV7-A-4 said. |
| RV7-A-5 | yes | Both missing direct cases are listed. RV8-R-2 is a small remainder. |
| RV7-A-6 | no change | RV7-A-6 asked for no repair. The splits stay. |
| RV7-A-7 | yes | The condition declaration and the range-based form are named. RV8-R-3 is a small remainder. |
| RV7-A-8 | yes | |
| RV7-A-9 | recorded | The Q-0431 line now says the reference is silent. |
| RV7-A-10 | no | The text keeps the compilers' answer. The note and row 4 record both reviews. RV8-R-10 concerns the note's account of the allocated storage. |
| RV7-A-11 | no change | RV7-A-11 supported the answers. |
| RV7-B-1 | no | The row stays. The note and row 4 record the review. |
| RV7-B-2 | yes | |
| RV7-B-3 | no, and partly withdrawn | The byte case stays in row 4 under Q-0430. For a base class subobject the reference agrees with ADD-14 (RV8-R-9). |
| RV7-B-4 | no | The text keeps the compilers' answer. The note and row 4 record the review. |
| RV7-B-5 | yes | RV8-R-8 is a small remainder. |
| RV7-B-6 | partly | The nested call is fixed. The immediate invocation is still missing (RV8-R-4). |
| RV7-B-7 | yes | RV8-R-6 is a doubt on the current instantiation. |
| RV7-B-8 | no change | The commit makes no repair, and the doubt stays. |
| RV7-B-9 | no change | RV7-B-9 closed RV6-B-9. |
| RV7-B-10 | yes, as text | The sentence and the Q-0225 row now agree. The row's reading stays with Q-0225 in row 4. |
| RV7-B-11 | no change | The commit makes no repair, and the doubts stay. |
| RV7-B-12 | yes | |
| RV7-B-13 | yes | The template-argument case is named. RV8-R-7 is a small remainder. |
| RV7-B-14 | no change | The commit makes no repair, and the doubt stays. |
| RV7-B-15 | yes | |
| RV7-B-16 | yes | |

## RV8-R-1

DTOR-8, the new sentence on a qualified base name. The sentence matches the reference. A qualifier that names a base class runs the base destructor alone, and the reference shows such calls on a derived object and through a pointer.

The rule's earlier sentence still says that the name after the tilde must denote the type of the object. The commit records a probe split for the unqualified form on a derived object, which GCC accepts and Clang and EDG reject. The reference does not state that form directly. Its rules read as allowing it. A tilde followed by a type name denotes the destructor of that type. The base's name is found in the derived class through the base. A member access may name a member of a base class. The reference also makes the call undefined only when the object is neither of the destructor's class nor of a class derived from it, which assumes calls on derived objects. So the reference leans toward GCC. Read strictly, DTOR-8 rejects the form, and no question records the split.

## RV8-R-2

REF-2's list and the glossary entry. The three cases match the reference. Two small doubts remain.

- The reference's rvalue case leaves out a bit-field. A bit-field rvalue binds to a temporary, as REF-2's lead implies. The first bullet does not say so, and a reader could place an xvalue bit-field among the direct cases.
- The glossary entry and REF-2 treat every binding through a converting constructor as a binding to a temporary. The reference defines a direct binding as every case except its last one, and its last case has two steps: a user-defined conversion, and an implicit conversion to the referenced type. The reference's own gloss of the last case names only the implicit conversion. Read literally, the user-defined step could count as direct. Tidal's reading is the common one, and the compilers rank the conversions that way. The reference is not explicit.

## RV8-R-3

LOOP-9's new list. The two new bullets match the reference. A condition declaration always has an initializer, and it is in scope in the body. The equivalent form of a range-based `for` declares its range and iterator variables before the inner loop, each with an initializer.

The list leaves out two more declarations that a jump into a range-based body skips: the loop's own variable, which the equivalent form declares at the start of the body with an initializer, and a declaration in the optional initializer statement of a range-based `for`. Neither changes the result, since the bullet already makes the jump not valid. The doubt is small.

## RV8-R-4

INST-1, INST-2, and the INST-P1 class row. The new text limits the exemption to a call that is the whole operand of decltype, with or without parentheses. That matches the reference, and the parenthesized test agrees.

The reference also leaves out a call that is an immediate invocation, with or without parentheses. Such a call is evaluated as a constant, it needs a result object, and its class type must be complete. So a call to a consteval function that returns the class instantiates the class even as the whole operand of decltype. RV7-B-6 named this case, and the repair does not state it. No test covers it.

## RV8-R-5

INST-2 and UNEV-1 at the comma operator. UNEV-1 says that the decltype exception also holds for a call that is the right operand of a comma operator. INST-2 now limits the exemption to a call that is the whole operand. The test note for the member access case says that every compiler instantiates the class when the call stands on either side of a comma.

The reference gives both results. The comma's result has the value category of its right operand, so a comma expression with a class prvalue on the right is itself a prvalue operand, and no temporary is made. A class operand of an operator also starts a search for operator functions. For a class template specialization, the class's friends can add candidates, so the class is instantiated for that search. A plain incomplete class has nothing to instantiate. So INST-2 agrees for a class template, and UNEV-1 agrees for an incomplete class that is not a template. Neither rule names the reason, and a reader who compares them sees a conflict.

## RV8-R-6

DEP-40, the split by qualifier. A namespace or an independent class qualifier gives lookup at the definition, and a dependent qualifier waits. Both halves match, and the new test agrees.

In the reference the current instantiation is a dependent type. A qualified call whose qualifier is the current instantiation looks up its name at the definition, in the class and its bases that are not dependent, and waits only when a dependent base leaves the lookup empty. DEP-17 and DEP-24 already say this. DEP-40's last sentence, read alone, places every dependent qualifier at the instantiation, and its first sentence does not name the current instantiation. The doubt is small.

## RV8-R-7

DEP-P1, the new lead sentence. The sentence now names the template-argument case through DEP-8, which closes RV7-B-13. The sentence still says that only an unqualified call uses both phases. An operator expression with a type-dependent operand also uses both kinds of lookup, as DEP-9 itself says. The table rows hold calls only, so the gap is in the word "Only". The doubt is small.

## RV8-R-8

DEP-6, DEP-30, and the DEP-P6 rows. The reference makes a qualified name into the current instantiation that finds nothing, in a class with no dependent base, a case that may be diagnosed before any instantiation. The same holds for a class member access whose object has the type of the current instantiation. The new rows and bullets give both cases to DEP-30, which matches. An undeclared unqualified name stays a required error, which also matches.

The text names the member access only through `this`. The reference's case is any member access whose object expression has the type of the current instantiation. A dereferenced `this` and a local object of the class's own type are two more forms. DEP-17 has the same limit. No test holds those forms. The doubt is small.

## RV8-R-9

The additive page: the Q-0049 line, the ADD-14 Evidence note, and the row 4 item "ADD-14, ADD-P6" that both name. RV7-B-3 said that the similarity rule also reads against ADD-14's older sentence on an array of one. That reading is too wide for one of the three cases that the older sentence covers.

- A pointer to a base class subobject. The subobject is not an element of an array. The reference counts such an object as the one element of an array of its own type, and that type is the pointer's type. So the move to one past is defined, and a move of two is undefined. A note next to the addition rule says the same for a base class subobject, a member subobject, and a complete object. ADD-14 and the compilers agree with the reference here. Q-0049's settled answer stands.
- A pointer of another type to an element of an array, where the element type is not similar to the pointer's type. Two Tidal cases fall here: a typed pointer to a byte of a byte array (Q-0430), and a pointer to the matching signed or unsigned integer type, which ADD-14's second paragraph covers. The reference makes any addition on such a pointer undefined, a move to one past included. Its own example uses the integer case. These cases read against ADD-14, as RV7-A-10 and RV7-B-3 said.

So the Q-0049 line's new sentence and the row 4 item, whose example is the base class subobject, record a disagreement that the reference does not have. The disagreement belongs to the element cases. The integer case has no row 4 item and no question of its own.

## RV8-R-10

The storage Evidence note, on a typed pointer at a multiple of the type's size into allocated storage. The note says that part A reads the pointer as pointing to a byte and that part B lets the allocation create an array of the type there. The note states both readings correctly.

The reference leans toward part A. The typed pointer comes from a cast of a byte position. A cast through a void pointer reaches another object only when the two objects are pointer-interconvertible. A byte and an object whose storage includes it are not, and neither are an array and its elements. So whatever objects the allocation creates, the cast gives a pointer that still points where the byte pointer pointed. Part B's reading needs the allocation to return a pointer to an array of the type, and then needs a character pointer to step through that array's bytes to the right element. The reference gives that byte step no defined result, which is the gap of Q-0304. Under Tidal's own answer to Q-0304 (ADD-6), part B's reading is the consistent one. Under the reference's text alone, part A's is. STG-28's sentence on a pointer cast straight from the allocation's result is not affected, as the note says.
