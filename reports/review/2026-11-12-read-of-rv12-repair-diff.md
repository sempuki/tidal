# Read of the RV12 repair diff

Date: 2026-11-12

Role: source reader

Reference: the C++23 working draft sources.

Commit read: 0c826a296553748af9c819c9b6156b66215c3e22.

The read covers every hunk of commit 0c826a29 under `specification/` and `tests/`, with the commit message and the review that the commit repairs (`reports/review/2026-11-12-gap-review-phase-18.md`, points RV12-1 to RV12-15 and its Part B, which the ledger numbers RV12-16 to RV12-22). The reader also read Q-0456, Q-0457, and Q-0458, the `pending.md` hunk of the commit, and ACC-20 on the access page, which holds the one-head form of the friend that TPL-4 limits. The read is item RV12, last part, of decision 0066. New points carry IDs of the form RV12-R-n.

## Result

One repair keeps an answer that reads against the reference. TPL-4's second condition, `friend-two-heads-pointer-pattern.cpp`, and the provisional answer of Q-0458 read against (RV12-R-1). The reference lets the friend's qualifier give a pattern of the first head's parameter, so the friend in the test is valid, the member reads the private member, and the assertion holds. Clang's acceptance is the reference's result. The repair does not change the rule. It records RV12-4 correctly and opens Q-0458 for it, so RV12-4 is answered as a question.

The other repairs agree with the reference:

- RV12-1. The Syntax list and the glossary entry now limit the comma item to the built-in operator, and the new sentence sends the overloaded case to COMMA-7. UNEV-2's list now has the reference's full closed set of forms, the loose sentence on an operand of an operator is gone, and UNEV-6 and the UNEV-P2 bullet follow. The three new reject tests agree. Both doubts of RV12-1 are closed. UNEV-2 still leaves three small details open (RV12-R-2).
- RV12-15. DEP-P2's new table gives the right thing for `noexcept` and names `typeid`, which closes the doubt on DEP-P2's reason. OPR-P3 now states the choice of the numeric form, the call that is not valid when the raw form and the template both exist, and the other heads. So the principle now covers the whole of Q-0435's option 1. Two small doubts remain (RV12-R-3, RV12-R-4).
- RV12-16. The TARG lines and the three test notes name Q-0456. TARG-16 and the braced arm still read against, as RV10-6 and RV12-10 said, and the lines state that correctly.
- RV12-18. The CTAD lines, the split row, and the test note name Q-0457 in place of Q-0020. CTAD-17's item still reads against, as RV8-B-5 said, and the lines state that correctly.

## Items

| File | Item | Rule, principle, or term | Review point | Verdict |
|---|---|---|---|---|
| decltype.md | Syntax list, the comma item limited to the built-in operator | discarded-value places, COMMA-1 | RV12-1 | agrees. The reference makes the left operand a discarded value only for the built-in operator. |
| decltype.md | sentence on the left operand of an overloaded comma | COMMA-7 | RV12-1 | agrees. With an overloaded operator the operand is an argument of a call, and nothing discards it. |
| glossary.md | discarded-value expression, the comma item and the volatile sentence | glossary | RV12-1 | agrees |
| decltype.md | UNEV-P2, the volatile bullet | UNEV-P2 | RV12-1 | agrees. The bullet sends the reader to UNEV-2's list. |
| decltype.md | UNEV-2, the list of forms that read a volatile object | UNEV-2 | RV12-1 | agrees with a doubt (RV12-R-2). The eight forms are the reference's set. |
| decltype.md | UNEV-6, the read in a discarded value and the read by an operator | UNEV-6, CE-12 | RV12-1 | agrees. Constant evaluation rejects any read of a volatile object, wherever it comes from. |
| decltype.md | Divergences line on UNEV-2 and UNEV-6 | UNEV-2, UNEV-6 | RV12-1 | agrees as a record. The line states RV12-1 correctly. |
| decltype.md | Evidence count | none | none | no reference content |
| declarations.md | TPL-4's second condition cites Q-0458 | TPL-4 | RV12-4 | reads against (RV12-R-1) |
| declarations.md | TPL Note on the pattern and on a local class | TPL-4, TPL-2 | RV12-4 | agrees as a record. The Note states RV12-4 correctly. The sentence on a local class agrees: the reference forbids a friend template there, whatever the number of heads. |
| declarations.md | Q-0458 Divergences line and split row | TPL-4 | RV12-4 | agrees as a record. The answer it gives reads against (RV12-R-1). |
| declarations.md | Evidence count | none | none | no reference content |
| dependent-names.md | DEP-P2, the sentence and the table | DEP-P2, DEP-4 | RV12-15 | agrees with a doubt (RV12-R-4). The four forms are value-dependent in the reference only when the operand is type-dependent or the type is dependent. |
| operators.md | OPR-P3, the sentence on other heads | OPR-P3, OPR-10 | RV12-15 | agrees with a doubt (RV12-R-3) |
| operators.md | OPR-P3, the table of competing forms | OPR-P3, OPR-8, OPR-16 | RV12-15 | agrees. Each row gives the reference's result. The table leaves out the case with no usable form (RV12-R-3). |
| arguments.md | TARG-16's paragraph cites Q-0456 | TARG-16 | RV12-16 | reads against, as before. The reference's template argument has no braced form. The citation records the point correctly. |
| arguments.md | Q-0456 Divergences line | TARG-16 | RV12-16 | agrees as a record |
| arguments.md | TARG Notes name Q-0456 and RV12-10 | TARG-13, TARG-16 | RV12-16 | agrees as a record |
| class-template-deduction.md | CTAD-17's item on the limit cites Q-0457 | CTAD-17 | RV12-18 | reads against, as before. In copy initialization from an object of another class, the reference does not let a user-defined conversion reach the first parameter of a guide, so Clang 18's rejection is the reference's result. The citation records the point correctly. |
| class-template-deduction.md | Q-0457 Divergences line, split row, and CTAD-17 Note | CTAD-17 | RV12-18 | agrees as a record |

## Tests

| Test | Verdict |
|---|---|
| `discarded-volatile-conditional-read.cpp` | agrees. Both arms are names of volatile objects, so the discarded conditional reads the result, and constant evaluation rejects the read. |
| `discarded-volatile-comma-read.cpp` | agrees. The right operand of the built-in comma is a name of a volatile object. |
| `discarded-volatile-parenthesized-member-read.cpp` | agrees. A pointer-to-member access in parentheses is in the set, and the member of a volatile object is volatile. |
| `friend-two-heads-pointer-pattern.cpp` | reads against (RV12-R-1). The program is valid in the reference, and the assertion holds. Clang's acceptance conforms. |
| `class-arguments.cpp`, `same-specialization-class.cpp`, `braced-scalar-arguments.cpp` (notes only) | the notes record Q-0456 correctly. The braced forms still read against, as RV10-6 and RV12-10 said. |
| `copy-init-guide-user-conversion.cpp` (note only) | the note records Q-0457 correctly. The expected acceptance still reads against, as RV8-B-5 said. |

## Review points

| Point | Answered | Note |
|---|---|---|
| RV12-1 | yes | Both doubts closed. RV12-R-2 is a small remainder. |
| RV12-4 | as a question | TPL-4 keeps the answer of GCC and EDG, which reads against. Q-0458 holds the point (RV12-R-1). |
| RV12-15 | yes | DEP-P2's reason and `typeid`, and OPR-P3's missing parts of option 1, are in. RV12-R-3 and RV12-R-4 are small remainders. |
| RV12-16 | yes | Part B asked for a question on RV10-6. Q-0456 holds it, and every line that rests on the point names it. |
| RV12-18 | yes | Part B asked for a question of its own in place of Q-0020. Q-0457 holds it. |

## Q-0458, first reading

The question and its context state the case correctly. The table of compiler results matches the test and the two older friend tests.

Option 2 is the reference's answer. The reference lets a friend template name a member of a dependent specialization when the qualifier ends in a template-id of a class template and the friend's parameters can be deduced from that template-id. A member of a specialization is then a friend when deduction against that specialization succeeds and the declaration, with the deduced arguments put in, matches the member. The reference's own example on friends uses a pointer pattern of exactly this kind, and there the class template has no partial specialization for the pattern. So no partial specialization of the pattern needs to exist. GCC's message suggests that GCC looks for one. In the test, deduction against the pointer specialization gives the element type, the member template matches, and the member may read the private member.

The second head belongs to the member template. The reference's example also has a friend with two heads, and the parameters of the second head need not be deducible from the qualifier. So the pattern rule applies to the first head's parameters only, as both options assume.

The provisional answer (option 1) reads against. The "Waits for" line and the place in `pending.md` row 4b fit a point where the newest families split and the review reads against the answer.

The question does not mention ACC-20. ACC-20 lets a friend with one template head name a member through a pointer pattern of its parameter, and its tests hold that form (Q-0404 records a further split there). The reference states one rule for both cases, with one head or with two. So under the provisional answer, Tidal accepts the pattern for a member function and rejects it for a member template. That difference has no ground in the reference. The point belongs in Q-0458's context.

## New questions

None.

## RV12-R-1

TPL-4's second condition, the TPL Note, `friend-two-heads-pointer-pattern.cpp`, and Q-0458.

TPL-4 asks that the qualifier give the first head's parameters themselves as the arguments of the class template. The reference asks only that those parameters can be deduced from the qualifier's template-id. A pointer pattern of the parameter meets the reference's condition. The friend then covers the member of each specialization that matches the pattern. So the friend in the new test is valid, the member of the pointer specialization has access, and the program is valid with a true assertion. TPL-4, the test's expected rejection, and Q-0458's provisional answer read against. Clang 18 and Clang 22 give the reference's result. ACC-20 already accepts the same pattern for a friend with one head (see the first reading of Q-0458).

## RV12-R-2

UNEV-2's list and the UNEV-P2 bullet.

The eight forms match the reference. Three details remain open:

- The reference keys the read on a glvalue whose type is volatile-qualified. UNEV-2 says an expression that refers to a volatile object. The two differ when a volatile-qualified glvalue denotes a non-volatile object. The reference still reads it.
- The reference's set holds only operators with their built-in meaning. With an overloaded subscript, indirection, arrow, conditional, or comma, the expression is a call, and the set does not apply to it. UNEV-2's subscript, member access, indirection, and comma items do not say "built-in", although the Syntax list now does for the comma.
- For a volatile object of class type, the reference's read copies the object into a temporary. That copy needs a constructor that accepts a volatile source, which an implicit copy constructor does not. So a discarded name of a volatile class object is not valid unless the class has such a constructor. UNEV-2 says only that the program reads the object, and no Tidal rule I found states the class case.

Each detail is small, and no test holds any of them.

## RV12-R-3

OPR-P3's new sentence and table.

The sentence says that a template with any other head is not a literal operator. In the reference, a literal operator template must have one of the two heads, so a declaration with any other head is not valid. OPR-10 says so, and the sentence can be read as the same thing. The words "is not a literal operator" also allow a reading where the declaration is valid and lookup skips it. The doubt is small.

The table covers the cases where at least one usable form exists. It has no row for a suffix where lookup finds only forms of the wrong kind, such as only the floating form for an integer literal. The reference makes that literal not valid, and OPR-16's first sentence states it. The table's second row calls the case of both forms ambiguous. The reference simply forbids both forms there, and the result is the same.

## RV12-R-4

DEP-P2's sentence and table.

The conclusion agrees: none of the four forms is value-dependent unless the operand is type-dependent or the type is dependent. The reason for `typeid` is loose. For a glvalue of polymorphic class type, the result names the dynamic type of the object, so it can depend on which object the operand refers to. A constant parameter of reference type to a polymorphic class is value-dependent and not type-dependent, and the reference still makes `typeid` of it not value-dependent. So the sentence that the operand's value takes no part in the result fails for that case, though the outcome DEP-P2 gives is the reference's. No test holds `typeid`. The doubt is small.
