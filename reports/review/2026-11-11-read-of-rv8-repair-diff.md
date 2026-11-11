# Read of the RV8 repair diff (d6fd9cea)

Date: 2026-11-11

Role: source reader

Reference: the C++23 working draft sources.

Commit read: d6fd9ceab8bb2ff7d0975f4377d32928e6c95b39.

The read covers every hunk of commit d6fd9cea under `specification/`, with the three RV8 summaries that the commit repairs (`reports/review/2026-11-11-read-of-rv7-repair-diff.md`, `reports/review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md`, and `reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`). The reader also read the 18 new and 2 changed tests that the hunks cite, and the question records of commits 9f3bceb4 and d6fd9cea, with `pending.md`, where they state a review's reading. The read is item RV8, last part, of decision 0063. New points carry IDs of the form RV9-R-n.

## Result

Most hunks agree with the reference. The repairs close RV8-R-6, R-7, R-9, A-2, A-3, A-4, and B-8. They close the first part of RV8-R-2, A-7, A-9, and B-9.

Two earlier points of the read of the RV7 repair diff do not hold, and this read withdraws them:

- RV8-R-1, with RV8-A-6, on the unqualified base name in an explicit destructor call. The reference has a lookup rule for a name after a tilde in a member access. The rule makes that name refer to the type of the object, so the call on a derived object with only the base's name is not valid. DTOR-8's new sentence, Clang, and EDG agree with the reference (RV9-R-1).
- RV8-R-4, on a call to a consteval function as the whole operand of decltype. The reference counts a call as an immediate invocation only when the call can be evaluated, and the operand of decltype never is. So the exclusion that the decltype rule names cannot reach such a call. INST-2's new sentence and every compiler agree with the reference (RV9-R-2).

The records of both cases state the earlier readings correctly. But the Q-0444 line, Q-0444, the INST Note, two test notes, and two `pending.md` row 4 items ("DTOR-8, Q-0444" and "INST-1, INST-2") now record disagreements that the reference does not have.

The text keeps the compilers' answer against a review, with a Note and a row 4 item, for RV8-A-1 with R-10 (STG-28, STG-16), R-9 (the integer case of ADD-14), B-4 (CTAD-2), B-5 (CTAD-17), and B-6 (CTAD-16). Those hunks read against the reference, as the reviews said, and each Note states its review's reading correctly. The new CTAD-16 test separates the two readings, and the reference gives the result that the compilers reject (see the RV8 points table).

The other new points are doubts.

## Hunks

| File | Hunk | Rule or principle | RV8 point | Verdict |
|---|---|---|---|---|
| constructors.md | tests line and the sentence on a base name with no qualifier | DTOR-8 | R-1, A-6, RM12-3 | agrees (RV9-R-1) |
| constructors.md | Evidence count | none | none | no reference content |
| constructors.md | Q-0444 divergence line | DTOR-8 | R-1 | agrees as a record. The line states R-1 correctly. R-1 does not hold (RV9-R-1). |
| constructors.md | split test row for `dtor-call-unqualified-base.cpp` | DTOR-8 | R-1 | agrees. The expected rejection matches the reference. |
| operators.md | OPR-P3, the sentence on a string literal and a template | OPR-P3 | A-3 | agrees |
| operators.md | OPR-8 tests line, the string row of the table, the two kinds of template, and the numeric form of the literal's kind | OPR-8 | A-3, A-4 | agrees with a doubt (RV9-R-5) |
| operators.md | OPR-16 tests line and the two declarations alone | OPR-16 | A-3, A-4 | agrees |
| operators.md | OPR-10 tests line and the two kinds | OPR-10 | A-3 | agrees |
| operators.md | Evidence count | none | none | no reference content |
| operators.md | Q-0435 divergence line on the two declarations | OPR-16 | A-4 | agrees |
| operators.md | Q-0446 divergence line | OPR-8, OPR-16 | A-3 | agrees. The template comes first when the string fits it, as Clang does. |
| operators.md | three split test rows | OPR-8, OPR-16 | A-3, A-4 | agrees |
| operators.md | Note on the class-type template | OPR-8, OPR-10 | A-3 | agrees |
| operators.md | Note on the numeric form and the place of the conflict | OPR-8, OPR-16 | A-4 | agrees |
| unions.md | the namespace sentence names the unnamed namespace | UNI-P3 | A-7 | agrees |
| references.md | the first direct case leaves out a bit-field | REF-2 | R-2, A-9 | agrees |
| type-names.md | `elaborated_type` gains attributes after the class key | grammar, TYN-4 | RM12-2 | agrees with a doubt (RV9-R-4) |
| type-names.md | TYN-10 tests line and the bullet on attributes outside a forward declaration | TYN-10 | RM12-2 | agrees with a doubt (RV9-R-4) |
| type-names.md | TYN-6 tests line and the bullet on attributes in a forward declaration | TYN-6 | RM12-2 | agrees |
| type-names.md | Evidence count | none | none | no reference content |
| type-names.md | Q-0445 divergence line and split row | TYN-10 | RM12-2 | agrees |
| additive.md | ADD-11 tests line and the sentences on a byte pointer into an object | ADD-11 | RM12-1 | agrees. The two pointers do not point into one array, so the difference is undefined. |
| additive.md | ADD-14 tests line and the Q-0442 citation | ADD-14 | R-9 | reads against (RV9-R-3) |
| additive.md | Evidence count | none | none | no reference content |
| additive.md | Q-0049 line | ADD-14 | R-9 | agrees. The line states R-9 correctly. |
| additive.md | Q-0442 divergence line | ADD-14 | R-9 | reads against as an answer. The line states R-9 correctly. |
| additive.md | Evidence note on the array of one, limited to the element cases | ADD-14 | R-9, B-3 | agrees. The note states R-9 correctly. |
| additive.md | Note on ADD-11 | ADD-11 | RM12-1 | agrees |
| storage.md | STG-13 tests line | STG-13 | RM12-1 | agrees |
| storage.md | STG-14 tests line | STG-14 | R-9 | agrees for the read through the pointer. The step in the same test reads against (RV9-R-3). |
| storage.md | STG-16 names STG-28 as an exception | STG-16 | A-1 | reads against. The reference has no such exception. The sentence now says plainly that it departs from the rule. |
| storage.md | STG-28 tests line and the cast to element `k` as an exception to STG-16 | STG-28 | A-1, R-10 | reads against for element `k` above 0, and agrees for element 0. The new wording no longer claims a consequence, which closes the second half of A-1. |
| storage.md | STG-28 example of a type that is not an implicit-lifetime type | STG-28 | A-2 | agrees |
| storage.md | Evidence count | none | none | no reference content |
| storage.md | Q-0430 line names the phase 15 review | STG-28 | A-1 | agrees as a record |
| storage.md | Note on A-1 and R-10 | STG-28, STG-16 | A-1, R-10 | agrees as a record. The note states both readings correctly. |
| storage.md | Note on A-2 | STG-28, STG-7 | A-2 | agrees. The `Cell` with a user-provided copy constructor has no trivial eligible constructor and is no aggregate, so it is not an implicit-lifetime type. |
| class-template-deduction.md | CTAD-2 tests line | CTAD-2 | B-4 | reads against for the inline form, as B-4 said. The split of the two tests is right. |
| class-template-deduction.md | CTAD-16 tests line | CTAD-16 | B-6 | reads against |
| class-template-deduction.md | CTAD-18, a reference to the list and a guide whose parameters all have defaults | CTAD-18 | B-8 | agrees |
| class-template-deduction.md | CTAD-24 tests line | CTAD-24 | B-6 | reads against for `inherited-aggregate-candidate-own-constructor.cpp` |
| class-template-deduction.md | Evidence count | none | none | no reference content |
| class-template-deduction.md | Q-0020 line | CTAD-17 | B-5 | agrees as a record. The line states B-5 correctly. |
| class-template-deduction.md | Q-0439 line | CTAD-2 | B-4 | agrees as a record. The line states B-4 correctly. |
| class-template-deduction.md | Q-0438 line on a guide with defaults | CTAD-18 | B-8 | agrees with the doubt that B-8 gave Q-0438 |
| class-template-deduction.md | Q-0443 divergence line | CTAD-18 | B-8 | agrees |
| class-template-deduction.md | three split test rows | CTAD-2, CTAD-18 | B-4, B-8 | agrees for the CTAD-18 rows. The inline CTAD-2 row reads against, as recorded. |
| class-template-deduction.md | Note on CTAD-16 | CTAD-16 | B-6 | agrees as a record. The note states B-6 correctly. |
| class-template-deduction.md | Note on CTAD-17 | CTAD-17 | B-5 | agrees as a record. The note states B-5 correctly. |
| class-template-deduction.md | Note on CTAD-1 and CTAD-2 | CTAD-1, CTAD-2 | B-4 | agrees as a record. The note states B-4 correctly. |
| dependent-names.md | lead sentence of the call table names operator expressions | DEP-P1 | R-7 | agrees |
| dependent-names.md | DEP-40 sends the current instantiation to DEP-17 and DEP-18 | DEP-40 | R-6 | agrees |
| instantiation.md | INST-1 tests line | INST-1 | R-4 | agrees (RV9-R-2) |
| instantiation.md | INST-2 tests line and the consteval sentence | INST-2 | R-4 | agrees (RV9-R-2) |
| instantiation.md | INST-5, the types of a member function | INST-5 | B-9 | agrees |
| instantiation.md | Evidence count | none | none | no reference content |
| instantiation.md | Note on the consteval call | INST-1, INST-2, INST-P1 | R-4 | agrees as a record of R-4. R-4 does not hold (RV9-R-2). |

## RV8 points

| Point | Closed | Note |
|---|---|---|
| RV8-R-1 | yes, withdrawn | DTOR-8 now rejects the form, and the reference agrees (RV9-R-1). |
| RV8-R-2 | partly | The bit-field is left out. The doubt on a converting constructor stays. |
| RV8-R-3 | no change | The doubt stays as text. |
| RV8-R-4 | yes, withdrawn | INST-2 keeps the compilers' answer, and the reference agrees (RV9-R-2). |
| RV8-R-5 | no change | The doubt stays as text. |
| RV8-R-6 | yes | |
| RV8-R-7 | yes | |
| RV8-R-8 | no change | The doubt stays as text. |
| RV8-R-9 | yes | The Q-0049 line, the ADD-14 note, and row 4 now hold only the element cases. Q-0442 holds the integer case. |
| RV8-R-10 | no | Recorded in the storage Note with A-1. STG-28 keeps the compilers' answer. |
| RV8-A-1 | no | STG-28 now states the case as an exception, which removes the false "So". The case still reads against, and row 4 holds it. |
| RV8-A-2 | yes | |
| RV8-A-3 | yes | RV9-R-5 is a small remainder. |
| RV8-A-4 | yes | |
| RV8-A-5 | no change | A-5 supported the answers. |
| RV8-A-6 | yes, withdrawn | As RV8-R-1 (RV9-R-1). |
| RV8-A-7 | yes | The doubt on the word "plain" stays and is small. |
| RV8-A-8 | no change | The doubts stay as text. |
| RV8-A-9 | partly | As RV8-R-2. |
| RV8-B-1 | no change | The doubt stays as text. |
| RV8-B-2 | no change | Q-0225 and Q-0226 hold the two rows. The DEP-25 doubt stays. |
| RV8-B-3 | no change | The doubt stays as text. |
| RV8-B-4 | no | The text keeps the compilers' answer. The new test isolates the inline form, which the reference allows. |
| RV8-B-5 | no | The text keeps the compilers' answer. The Note and row 4 record the review. |
| RV8-B-6 | no | The text keeps the compilers' answer. The new test has the shape that B-6 proposed. In the reference, the alias built from the base takes the base's aggregate candidate, which deduces the derived type from the one integer. The derived class's own constructor template then initializes the object. So the reference accepts the program that every compiler rejects, and B-6 stands. |
| RV8-B-7 | no change | The doubt stays as text. |
| RV8-B-8 | yes | Both CTAD-18 doubts are repaired. The doubt on the notional default constructor stays with Q-0438. |
| RV8-B-9 | partly | The word "only" is repaired. The inline member stays with Q-0441. |
| RV8-B-10 | no change | B-10 supported the answers. |

## Question records

The records of commits 9f3bceb4 and d6fd9cea state each review's reading correctly: Q-0020, Q-0049, Q-0225, Q-0226, Q-0231, Q-0243, Q-0277, Q-0304, Q-0429, Q-0430, Q-0431, Q-0435, Q-0436, Q-0437, Q-0438, Q-0439, Q-0440, Q-0441, Q-0442, and Q-0444, with the row 4 items "STG-28, Q-0430", "ADD-14, Q-0442", "CTAD-2, Q-0439", "CTAD-17", "CTAD-16", "DTOR-8, Q-0444", and "INST-1, INST-2". The Q-0304 note states correctly where RV8-A-1 differs from RV8-R-10.

Two records state a reading that this read withdraws. Q-0444's opinions and provisional answer, and the row 4 item "DTOR-8, Q-0444", carry RV8-R-1 and RV8-A-6 (RV9-R-1). The row 4 item "INST-1, INST-2" carries RV8-R-4 (RV9-R-2). Each states the earlier reading as it was written.

Q-0442's option 2 is narrower than the reading it reports (RV9-R-3).

## New questions

| Question | Verdict | Reason |
|---|---|---|
| Q-0442 | reads against | RV8-R-9 and RV9-R-3. The pointer's type and the array's element type are not similar, so every step and every difference on such a pointer is undefined. The reference's own example of the rule uses this integer case. Option 2 is closest, without its exception for zero. |
| Q-0443 | supports | The reference counts a constructor whose first parameter is a reference to the list type, with any cv-qualifiers, as an initializer-list constructor. The guides act as constructors of a notional class, so the same holds for a guide (option 1). |
| Q-0444 | supports | RV9-R-1. A base name with no qualifier after the tilde must refer to the type of the object, so the call on a derived object is not valid (option 1). |
| Q-0445 | supports with a doubt | An attribute after the class key is allowed only in a declaration that holds nothing but the elaborated type, so a variable, a function, or a friend declaration cannot carry it (option 1). The doubt is RV9-R-4. |
| Q-0446 | supports | When the set for the suffix has a template whose parameter can take the string literal, the literal calls the template. Otherwise it calls the form with a pointer and a length, and with no such form the program is not valid (option 1). GCC and EDG read against for the first form, and EDG reads against for the second. |

## RV9-R-1

DTOR-8, Q-0444, the Q-0444 line, the test note of `dtor-call-unqualified-base.cpp`, and the row 4 item "DTOR-8, Q-0444". RV8-R-1 and RV8-A-6 read the reference as leaning toward GCC. That reading missed a lookup rule.

In a class member access, the name that follows the dot or arrow counts as a qualified name, with the object's type as its context. The reference has a separate rule for such a name when it comes after a tilde. The name is looked up both ways, and the type it names must be the context type, cv-qualifiers aside. For a call on a derived object with only the base's name, the context is the derived class, and the name finds the base. So the call is not valid. For the qualified form, the context of the name after the tilde is the base that the qualifier names, so the base's destructor runs, as DTOR-8 says.

The points that RV8-R-1 listed (lookup through the base, a member access to a base member, and the undefined case for an object of an unrelated class) do not override this rule. DTOR-8's earlier sentence, its new sentence, Clang, and EDG agree with the reference. GCC reads against.

The records state RV8-R-1 as it was written. But the test note says that GCC follows the review's reading of the reference, and the row 4 item and Q-0444's provisional answer present a review that disagrees. With this read, no review disagrees.

## RV9-R-2

INST-2's consteval sentence, `class-decltype-consteval-call-not-instantiated.cpp`, the INST Note, and the row 4 item "INST-1, INST-2". RV8-R-4 read a call to a consteval function as an immediate invocation inside decltype. That reading does not hold.

The reference's decltype rule excludes an immediate invocation from the case with no temporary, with or without parentheses. But the reference defines an immediate invocation only for a call that is potentially evaluated, and the operand of decltype is unevaluated. So a call to a consteval function as the whole operand of decltype is not an immediate invocation, and the class result needs no completion. The exclusion in the decltype rule has no reach over such calls.

The reference has one form of immediate invocation with no such condition: an aggregate initialization whose default member initializers hold an escalating call. That form needs the class complete in any case, so it does not touch INST-2.

So INST-2's new sentence agrees, and so do the three compilers. The test's note says that the call is an immediate invocation, which is wrong by the reference. The INST Note and the row 4 item state RV8-R-4 correctly, but they hold a disagreement that the reference does not have. The `sizeof` control in the writer's probe instantiates the class for another reason, since that operator needs a complete type.

## RV9-R-3

ADD-14's integer paragraph, Q-0442, and `integer-counterpart-one-past.cpp`. RV8-R-9 stands. The reference's rule on a pointer whose type is not similar to the array's element type makes the behavior of every addition and subtraction undefined. The rule has no exception for the move to one past, and none for a step of zero.

Q-0442's option 2 says that any addition other than zero is undefined. Read by the reference, zero is not set apart. The test also takes the difference of the two pointers, which is undefined by the same rule. The read through the pointer is valid, as STG-14 says. The test passes because no compiler or sanitizer reports the step. The doubt on option 2 is small, and the verdict on the question does not change.

## RV9-R-4

The `elaborated_type` production, TYN-10, and Q-0445. TYN-10 allows an attribute after the class key only in `forward_class`, which ends in an identifier. The reference allows an attribute in one more declaration that holds only the elaborated type: a class key with a template-id, inside an explicit specialization or a partial specialization. The reference's own grammar for an elaborated type with a template-id has no attribute place, so the reference is not consistent on that form. No test holds it. The doubt is small.

## RV9-R-5

OPR-8 and `literal-operator-template-class-parameter.cpp`. The test's second operator template has a parameter whose type is a class template name with no arguments, deduced from the string literal. The reference asks for one constant parameter of class type. It does not say whether a parameter with a deduced class type counts before the deduction. The deduced type is a class, and every compiler accepts the form. OPR-8 does not name the form, and the test note says that it is valid. The reference is silent on the point, and the doubt is small.
