# Read of the RV9 repair diff (8d33fcb1)

Date: 2026-11-11

Role: source reader

Reference: the C++23 working draft sources.

Commit read: 8d33fcb17ac8e7c365be191103d54a2f30e94d17.

The read covers every hunk of commit 8d33fcb1 under `specification/`, with the review that the commit repairs (`reports/review/2026-11-11-gap-review-phase-16.md`). The reader also read the 11 new and 2 changed tests that the hunks cite, and the question records of commits b310bf11 and 8d33fcb1, with `pending.md`, where they state the review's reading. The read is item RV9, last part, of decision 0064. New points carry IDs of the form RV10-R-n.

## Result

Most hunks agree with the reference. The repairs close RV9-1, RV9-2 as text, RV9-8, RV9-10, RV9-11, and RV9-12. They close RV9-14 for a concept.

The text keeps the compilers' answer against the review for RV9-6 (PRIM-15 and OPR-8) and RV9-7 (LSUP-10 and UNI-14). Those hunks record the review with a Note and a row 4 item. The Notes and the row 4 items state the review's reading correctly, apart from one small gap in the LSUP-10 record (RV10-R-6).

One hunk adds a condition. OBJ-2's new item says that an assignment to a scalar that an explicit destructor call ended creates no new object. The reference makes an assignment through a union member's name create a new object of the member's type when the write would otherwise be undefined. So the item holds for an array element and for a variable, as both new tests show, and fails for a scalar union member (RV10-R-3).

The other new points are doubts:

- INST-30's exception covers two of the places where the reference counts the class as complete (RV10-R-1).
- FN-12's phrase on the outermost enclosing class does not fit a nested class defined outside its enclosing class (RV10-R-2).
- For the copy assignment, the reference does not say which union's lifetime counts in the limit on a `mutable` active member (RV10-R-4).
- CPT-28's new sentence reaches places where the reference's rule does not reach (RV10-R-5).
- OBJ-1's array row names any constructor that is not trivial, where the reference names the default constructor (RV10-R-7).
- An OPR Note names two different suffixes for one test (RV10-R-8).

## Hunks

| File | Hunk | Rule or principle | RV9 point | Verdict |
|---|---|---|---|---|
| members.md | CLS-2 tests line | CLS-2 | RV9-1, RV9-2 | agrees. Each new test uses the class in a body or a default argument, where the class is complete. |
| members.md | Evidence count | none | none | no reference content |
| operators.md | Q-0063 divergence line, the warning and the review | OPR-8, PRIM-15 | RV9-6 | agrees as a record. The line states RV9-6 correctly. Each result conforms under both readings, since a warning is a diagnostic. |
| operators.md | Note on OPR-8 and PRIM-15 | OPR-8, PRIM-15 | RV9-6 | agrees as a record. OPR-8 and PRIM-15 still read against, as RV9-6 said. The Note names two suffixes (RV10-R-8). |
| unions.md | UNI-14, all or part of the storage | UNI-14, STG-6 | RV9-8 | agrees |
| unions.md | UNI-14 Note names RV9-7 and row 4 | UNI-14 | RV9-7 | agrees as a record. UNI-14 still reads against in constant evaluation. |
| definitions.md | FN-12 tests line and the sentence on a member or friend function | FN-12 | RV9-1 | agrees with a doubt (RV10-R-2) |
| definitions.md | Evidence count | none | none | no reference content |
| definitions.md | Note on the two cases of RV9-1 | FN-12 | RV9-1 | agrees. The nested class and the friend function follow the rule on complete classes. |
| primary.md | Q-0063 divergence line | PRIM-15 | RV9-6 | agrees as a record |
| primary.md | PRIM-15 Note | PRIM-15 | RV9-6 | agrees as a record. The reference makes the literal not valid. PRIM-15 reads against, and row 4 holds the case. |
| constant-evaluation.md | CE-22 tests line and the sentence on a `mutable` active member | CE-22 | RV9-10 | agrees with a doubt (RV10-R-4) |
| constant-evaluation.md | Evidence count | none | none | no reference content |
| constant-evaluation.md | Q-0449 divergence line | CE-22 | RV9-10 | agrees with the same doubt (RV10-R-4) |
| constant-evaluation.md | two split test rows | CE-22 | RV9-10 | agrees for the constructor row. The assignment row agrees with the doubt of RV10-R-4. |
| constant-evaluation.md | CE Note on the other `mutable` forms | CE-22 | RV9-10 | agrees. See RV10-R-4 for what the reference gives in those forms. |
| language-support.md | LSUP-10 Note names RV9-7 and row 4 | LSUP-10 | RV9-7 | agrees as a record, with a small gap (RV10-R-6) |
| objects.md | OBJ-1 array row | OBJ-1 | RV9-11 | agrees with a doubt (RV10-R-7) |
| objects.md | OBJ-2 tests line and the item on an assignment after an explicit destructor call | OBJ-2 | RV9-9 | adds a condition (RV10-R-3) |
| objects.md | Evidence count | none | none | no reference content |
| objects.md | Q-0447 divergence line | OBJ-2, CE-21 | RV9-9 | agrees for the two forms that the tests hold. The general claim has the gap of RV10-R-3. |
| objects.md | two split test rows | OBJ-2 | RV9-9 | agrees. In both tests the left side names no union member, so no object is created, and the write is undefined. |
| objects.md | OBJ Note on the array row | OBJ-1 | RV9-11 | agrees |
| storage.md | STG-6, all or part of the storage | STG-6 | RV9-8 | agrees. Any reuse by an object not nested in the old one ends the old one. |
| storage.md | the Maybe sentence | STG-5, CPY-5 | RV9-12 | agrees. A user-declared copy function stops the implicit move assignment, so an rvalue selects the deleted copy assignment. |
| storage.md | Note on the Maybe assignment | CPY-5 | RV9-12 | agrees |
| glossary.md | lifetime, the array sentence | OBJ-1 | RV9-11 | agrees |
| concepts.md | CPT-28 tests line and the sentence on a parameter type that is not valid | CPT-28 | RV9-14 | agrees for a concept. The reference is silent for the other places (RV10-R-5). |
| concepts.md | example comment | CPT-28 | RV9-14 | agrees |
| concepts.md | Evidence count | none | none | no reference content |
| concepts.md | CPT-28 Note | CPT-28 | RV9-14 | agrees as a record. The Note states RV9-14 correctly. |
| instantiation.md | INST-P4 diagram of the point after the class | INST-P4, INST-30 | RV9-2 | agrees with a doubt (RV10-R-1). The declaration with no body needs no complete type, as the diagram says. |
| instantiation.md | INST-1 tests line and the clause on a function defined in a class | INST-1 | RV9-2 | agrees with a doubt (RV10-R-1). The clause and INST-30 now agree. |
| instantiation.md | INST-30 tests line and the exception for a class definition | INST-30 | RV9-2 | agrees with a doubt (RV10-R-1) |
| instantiation.md | INST-33 tests line | INST-33 | RV9-2 | agrees. The test is not valid under both readings of the point. |
| instantiation.md | Evidence count | none | none | no reference content |
| instantiation.md | Note on INST-30, INST-1, and FN-12 | INST-30 | RV9-2 | agrees as a record. The Note states RV9-2 correctly. For the `static constexpr` member read in a default argument, the reference places the member's point after the class, so EDG's rejection reads against. |

## RV9 points

| Point | Closed | Note |
|---|---|---|
| RV9-1 | yes | FN-12 names the friend function and the outermost enclosing class. RV10-R-2 is a small remainder. |
| RV9-2 | yes, as text | INST-30 now states the exception, and INST-1 cites it. The reference stays not consistent on the case (RV10-R-1). |
| RV9-3 | no change | RV9-3 agreed with the text. |
| RV9-4 | no change | The doubt stays with Q-0445. |
| RV9-5 | no change | RV9-5 agreed with the text. |
| RV9-6 | no | PRIM-15 and OPR-8 keep the compilers' answer. The Notes, the Q-0063 lines, and row 4 record the review correctly. |
| RV9-7 | no | LSUP-10 and UNI-14 keep the compilers' answer. Row 4 gains the item. |
| RV9-8 | yes | |
| RV9-9 | partly | The doubt on CE-21 stays as text. OBJ-2 now holds the last row of Q-0447's table, with a gap for a union member (RV10-R-3). |
| RV9-10 | yes | CE-22 names the `mutable` limit. The Q-0448 doubt stays. RV10-R-4 is a doubt on the new sentence. |
| RV9-11 | yes | RV10-R-7 is a small remainder. |
| RV9-12 | yes | |
| RV9-13 | no change | RV9-13 agreed with the text. |
| RV9-14 | yes for a concept | The new sentence also covers a variable template and an `if constexpr` condition (RV10-R-5). |

## Question records

The records of commits b310bf11 and 8d33fcb1 state the review's reading correctly:

- Q-0063, its dated notes, its Opinions entry, and the row 4 item "PRIM-15, Q-0063" state RV9-6 correctly. The wider Affects line of 8d33fcb1 matches the tests.
- Q-0430 states correctly that the review gives no new reading.
- Q-0444 states RV9-3 correctly.
- Q-0445 states RV9-4 correctly, with its doubt on the qualified form.
- Q-0447's Opinions entry and its first dated note state RV9-9 correctly. The second dated note says that OBJ-2 gives the RV9-9 reading. That holds for the forms in the table. OBJ-2's general claim goes past RV9-9, which read only the struct element (RV10-R-3). The new table row for a variable agrees with the reference.
- Q-0448's Opinions entry states RV9-10 correctly, with the `mutable` limit.
- The row 4 item "LSUP-10, UNI-14" states RV9-7 with one small gap (RV10-R-6).
- Q-0449's Opinions entry states RV9-10 correctly.

## New questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0449 | supports with a doubt | Option 1 matches the reference. The limit names the union's active member, and only when that member is `mutable`. So option 2, which counts any `mutable` member, reads against. The doubt is RV10-R-4. For the copy assignment, the reference does not say whether the union whose lifetime must have begun in the evaluation is the source or the target. |

The forms of Q-0449's table that no test holds:

- A union whose active member is not `mutable`, copied from a `constexpr` variable. The reference allows the copy, since the active member is readable. Clang and GCC read against, and EDG agrees.
- A struct with a `mutable` member, copied from a `constexpr` variable. The implicit copy reads each member, and the reference does not count a `mutable` part of a constant variable as usable in constant expressions. So the copy is not constant. Clang and GCC agree, and EDG reads against.
- A read of the `mutable` member of the constant union follows the same rule. Clang and GCC agree, and EDG reads against.

## RV10-R-1

INST-30's exception, INST-1's clause, the INST-P4 diagram, and the two new instantiation tests.

The repair does what RV9-2 asked. INST-30 now gives the point right after the class definition for a use in a member or friend function body and in a default argument, also inside a nested class. The reference's general point for a class specialization stays before the namespace-scope declaration, so the letter still reads against the exception. The reference's rule on complete classes reads with it. The compilers follow the exception.

The doubt is the reach of the list. The reference counts the class as complete in five places inside its member list: a function body, a default argument, a default template argument, the operand of a `noexcept` specifier, and a default member initializer. INST-30 names the first two. For the other three, Tidal keeps the general point, where the class is incomplete. The INST Note gives the reason for the default member initializer (EDG's split). No Note names the default template argument or the `noexcept` operand, and no test holds them. CLS-2's list also leaves out the default template argument of a member template. The doubt is small.

## RV10-R-2

FN-12, the new sentence on a member or friend function defined in a class.

The reference checks the types inside the body, and counts the body of a nested class as a place where the enclosing class is complete only when the nested class is defined inside the enclosing class. A nested class can also be defined after its enclosing class ends, with a qualified name. For such a class, the check falls at the end of the nested class's own definition. FN-12's phrase on the outermost enclosing class could be read there as the enclosing class that ended earlier, which would reject the nested class's own type as a return type. No test holds the form. The doubt is small.

## RV10-R-3

OBJ-2's new item, the Q-0447 line, and the second dated note of Q-0447.

The reference has a rule for an assignment whose left side names a union member through member access, possibly under array subscripts. For each such member of a suitable type, if a write to it would be undefined because its lifetime has not begun or has ended, the assignment first creates a new object of that type at that place. So for a union whose scalar member an explicit destructor call ended, a later assignment through the member's name creates a new object and is valid. Tidal's own UNI-5 and UNI-14 give the same result for a union: the destructor call leaves no active member, and the assignment makes the member active.

OBJ-2's item says that the assignment creates no new object, with no exception. The item holds for both new tests, since the left side of each names an array element of a struct or a local variable, and neither is a union member. For an array element of a union member, the rule looks at the whole array member, which is still alive, so no object is created and the item holds there too. The condition is the scalar union member named directly. Without it, OBJ-2 and UNI-5 give two results for one program.

## RV10-R-4

CE-22's new sentence, the Q-0449 line, `union-copy-mutable-member-assignment-outside.cpp`, and Q-0449.

The reference forbids a call of a union's implicit copy or move constructor or assignment in constant evaluation when the union has an active member and that member is `mutable`. The limit does not apply if the union started to live during the same evaluation. For the constructor, the limit can only mean the source, since the target always begins within the evaluation. For the assignment, the reference names neither the source nor the target. CE-22 and the test read the source, which matches the purpose of the limit: the copy would read a part of a constant that the reference does not let constant evaluation read. GCC's acceptance of the assignment fits a reading of the target, which the local union satisfies. The doubt is that the reference does not choose in words.

The CE Note's other forms are decided by the reference, as the New questions section says. CE-22 states the union case only, as the Note says.

## RV10-R-5

CPT-28's new sentence and `requires-parameter-type-not-valid-false.cpp`.

The reference's rule that makes a requires expression false names a type or an expression that is not valid in its requirements. It does not name the parameter list. For a concept, the result is still false, because the concept's constraint is the requires expression, and a substitution failure in a constraint makes it not satisfied. RV9-14 gave that route. For a variable template and an `if constexpr` condition, no constraint is checked, and the reference has no sentence that turns a failure in the parameter list into false. There the letter is silent, and the general rule on a type that is not valid in an instantiation could make the program not valid. CPT-28's sentence covers all three places, and every compiler gives false in each. The test's second and third assertions rest on CPT-28 alone. The doubt is small.

## RV10-R-6

The LSUP-10 Note, the UNI-14 Note, and the row 4 item "LSUP-10, UNI-14".

The reference's allowance for the library construction function in constant evaluation names an object that started to live during the same evaluation. The LSUP-10 Note and the row 4 item say only an object whose lifetime began, and leave out the evaluation. A member that is not active has not begun at all, so the conclusion for LSUP-10 and UNI-14 does not change. But the record states the reviews' reading more widely than RV9-7 gave it. The point is small.

## RV10-R-7

OBJ-1's array row.

The reference starts an object's lifetime when its initialization completes, and counts vacuous initialization as complete with the storage. For an array, initialization is vacuous when the array is default-initialized and its element type, if a class, has a trivial default constructor. The row's condition names an element type with any constructor that is not trivial. An element type with a trivial default constructor and a user-provided copy constructor is caught by the row's wording, though its default initialization does nothing. No program observes the difference, as the OBJ Note says. The point is small. The end column and the glossary entry agree.

## RV10-R-8

The OPR Note on OPR-8 and PRIM-15.

The Note says that Clang and GCC accept the literal with the suffix `_v`, and then that they reject the literal with the suffix `_ld` under the error option. The test `literal-float-form-out-of-range.cpp` declares the suffix `_ld`. The first name has no counterpart in the test. The point has no reference content and is small.
