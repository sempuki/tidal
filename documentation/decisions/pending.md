# Decisions that wait on the maintainer

Each row is one item that waits on the maintainer (decision 0054). Tidal
keeps the answer in the third column until the maintainer decides. The
last column lists the questions that the decision closes or changes.

Rows 1, 4a, and 10 come first (decision 0059, item PK3). Each has an
option under which no test's expectation changes, and together they
hold 92 open questions and the 17 rule items of row 4a. The
[decision packet](#decision-packet) gives the questions and tests of
each option.

| Date | Request to the maintainer | Answer |
|---|---|---|
| 2026-11-10 | Decide rows 1, 4a, and 10 first (decision 0061, item PK4) | none yet |

| # | Item | Options | Tidal keeps | Questions |
|---|---|---|---|---|
| 1 | The choices of decision 0054: settling by review, and program tests under `tests/programs/` | Confirm, change, or drop each choice | Both choices. A question settles by review only when at most one newest family differs from the answer in each form | Confirms the 203 questions settled under decision 0054 ([by item below](#settlements-under-decision-0054)). `tools/question_index.py` writes the total and fails `--check` when the total is wrong. If the maintainer lets the rule settle a question where two families differ, the questions of the [decision packet](#decision-packet) row 1 settle too |
| 2 | The choices of decision 0051: settling when the newest version of each family agrees, and keeping the implementation-only operations referenced | Confirm or change each choice | Both choices | Q-0322 |
| 3 | A report of EDG behavior to the EDG project ([groups below](#edg-report)) | Approve the report, or hold it | Tidal follows Clang 22 and GCC 16. Each test records EDG's result as an override | Q-0002, Q-0003, and the cluster 5 groups below |
| 4 | A gap review reads the C++23 text against the matrix's answer, and at most one newest family follows the review ([items below](#matrix-against-review)) | Follow the review, against the compilers that differ from it, or follow the matrix | The matrix's answer in each, except Q-0307, where Tidal follows the review | Each question and rule in the section below |
| 5 | The variable form of Q-0235 | Reject the unqualified name (GCC 16), or accept it (Clang 22, EDG 7.0, the cluster 6 review with doubt, and the gap review of the text phase 8 wrote on expressions, foundations, and templates, which also reads the class form as valid and only the function form as not valid) | Reject. INST-21 keeps one answer for the class, function, and variable forms | Q-0235 |
| 6 | Q-0343 and Q-0041 | Follow the gap review of item O, or follow the compilers | The review's reading. CPY-19 says a class with a deleted destructor, or with every copy and move function deleted, is not trivially copyable, and every family's builtin query says it is. JMP-6 accepts the jump, as GCC does, and Clang and EDG reject it | Q-0343, Q-0041 |
| 7 | A probe of which C++23 rejections later language versions accept, as an index with no rules ([plan below](#later-version-probe)) | Run the probe as planned, or keep later versions out of scope | C++23 only. Newer features are rejected under strict mode | Q-0007. A choice to follow later changes also decides Q-0087, Q-0181, and Q-0188 |
| 8 | An outside reader for the Foundations chapter and two topic pages (decision 0024) | Ask the reviewer who prompted the principles-first change, or skip | No outside reading | None |
| 9 | Process questions that name no rule ([list below](#process-questions)) | Confirm or change each provisional answer | Each provisional answer | Q-0006, Q-0008, Q-0116, Q-0119, Q-0182, Q-0183, Q-0186, Q-0312, Q-0412 |
| 10 | A review finds the reference silent on a question, so its answer is Tidal's own choice ([decision packet](#decision-packet), row 10) | Record each answer as Tidal's choice in the page's Evidence and settle it, or keep it open | Kept open | The questions of the packet's row 10 |

## Settlements under decision 0054

| Questions | Item | Commit |
|---|---|---|
| 34 | item SR | 0637b59 |
| 1 | the repairs from the exceptions and names reviews | b00cbf2 |
| 13 | the repairs from the object-model and program-structure reviews | 573d849 |
| 36 | the question reviews of overloading and templates | e0d06f0 |
| 40 | the question reviews of classes, statements, expressions, and declarations | 8c15b06 |
| 4 | phase 8 item RV, the repairs from the gap reviews of the text phase 7 wrote | 7a68692 |
| 35 | phase 9 item RV2, the repairs from the gap reviews of the text phase 8 wrote | 1bf7812 |
| 20 | phase 9 item PU, the principle-use batch PU1 | 926831c |
| 7 | the principle-use batch PU2 | f2ac11c |
| 10 | phase 10 item RV3, the gap reviews of the text phase 9 wrote: Q-0092, Q-0247, Q-0340, Q-0358, Q-0390, Q-0392, Q-0397, Q-0398, Q-0399, Q-0400 | b1d89e7 |
| 2 | phase 10 item PU3, the principle-use batch PU3: Q-0370, Q-0376 | 75e4fab |
| 4 | phase 11 item RV4, the gap reviews of the text phase 10 wrote: Q-0263, Q-0386, Q-0395, Q-0413 | 1e51073 |
| 2 | phase 12 item QR, question records: Q-0323, Q-0383 | da51ae9 |
| 1 | phase 13 item QR2, question records: Q-0419 | 4045bbd |
| 1 | phase 13 item RV6, the gap reviews of the text phase 13 wrote: Q-0428 | 7dd6d7c |
| 1 | phase 14 item RV7, the gap reviews of the text phase 14 wrote: Q-0422 | 4dc57e8 |
| -8 | phase 8 item PQ reopened these under the principle test of the style guide: Q-0064, Q-0067, Q-0111, Q-0242, Q-0259, Q-0319, Q-0329, Q-0373. Item PU settled Q-0067 and Q-0329 again | |

## Decision packet

`tools/question_index.py` writes this section from the `**Waits for:**`
line of each open question and from the row 4 table below. For each
row and option, it gives the open questions that the option settles
and the number of tests whose notes name them. "Answers stay" means
that no test's expectation changes. For the other options, the last
column gives an upper bound: the tests that name the questions, some of
which hold a case that the option leaves as it is. The tool cannot tell
which case each test holds. Row 4a holds the questions where every
newest family keeps the matrix's answer, and row 4b those where one
family follows the review. The rows marked "rule items" hold the
items of the row 4 table that name rules and no question; their
column of tests counts the tests whose `tidal-rules` line names one
of the item's rules, so the bound is wide. Row 10 holds the questions
whose last review finds the reference silent.

| Row | Option | Open questions it settles | Tests that name them | Tests whose expectation changes |
|---|---|---|---|---|
| 1 | Widen decision 0054 to settle while two newest families differ; answers stay | 58: Q-0017, Q-0021, Q-0033, Q-0041, Q-0045, Q-0062, Q-0080, Q-0082, Q-0084, Q-0121, Q-0126, Q-0129, Q-0132, Q-0133, Q-0134, Q-0140, Q-0143, Q-0148, Q-0152, Q-0153, Q-0156, Q-0159, Q-0161, Q-0165, Q-0168, Q-0174, Q-0192, Q-0193, Q-0195, Q-0197, Q-0216, Q-0217, Q-0218, Q-0227, Q-0234, Q-0235, Q-0241, Q-0248, Q-0251, Q-0301, Q-0307, Q-0320, Q-0331, Q-0335, Q-0337, Q-0339, Q-0341, Q-0346, Q-0349, Q-0368, Q-0374, Q-0403, Q-0420, Q-0421, Q-0425, Q-0438, Q-0441, Q-0446 | 138 | 0 |
| 2 | Decide the row; answers may change | 1: Q-0322 | 0 | 0 |
| 3 | Decide the row; answers may change | 12: Q-0195, Q-0197, Q-0198, Q-0201, Q-0203, Q-0235, Q-0266, Q-0272, Q-0277, Q-0278, Q-0280, Q-0282 | 23 | at most 23 |
| 4a | Follow the compilers; answers stay | 20: Q-0140, Q-0142, Q-0181, Q-0225, Q-0243, Q-0246, Q-0275, Q-0277, Q-0280, Q-0300, Q-0336, Q-0394, Q-0405, Q-0406, Q-0407, Q-0409, Q-0410, Q-0416, Q-0430, Q-0442 | 36 | 0 |
| 4a | Follow the review; answers change | 20: Q-0140, Q-0142, Q-0181, Q-0225, Q-0243, Q-0246, Q-0275, Q-0277, Q-0280, Q-0300, Q-0336, Q-0394, Q-0405, Q-0406, Q-0407, Q-0409, Q-0410, Q-0416, Q-0430, Q-0442 | 36 | at most 36 |
| 4b | Follow the compilers; answers stay | 23: Q-0074, Q-0077, Q-0087, Q-0123, Q-0131, Q-0190, Q-0191, Q-0205, Q-0226, Q-0307, Q-0330, Q-0362, Q-0366, Q-0369, Q-0371, Q-0389, Q-0393, Q-0401, Q-0404, Q-0408, Q-0418, Q-0426, Q-0439 | 50 | 0 |
| 4b | Follow the review; answers change | 23: Q-0074, Q-0077, Q-0087, Q-0123, Q-0131, Q-0190, Q-0191, Q-0205, Q-0226, Q-0307, Q-0330, Q-0362, Q-0366, Q-0369, Q-0371, Q-0389, Q-0393, Q-0401, Q-0404, Q-0408, Q-0418, Q-0426, Q-0439 | 50 | at most 50 |
| 4a | Follow the compilers, rule items; answers stay | 17: THR-3; TRY-2; UNW-3; NAM-40, NAM-41; STG-9; LSUP-11; TR-P6, TR-13, TR-14, TR-15; UNW-1; OBJ-11; ENUM-10, ENUM-P1; NS-12; NEW-P5; ICS-45; DEP-15, DEP-P4; CTAD-17; CTAD-16; CMP-8 | 204 | 0 |
| 4a | Follow the review, rule items; answers change | 17: THR-3; TRY-2; UNW-3; NAM-40, NAM-41; STG-9; LSUP-11; TR-P6, TR-13, TR-14, TR-15; UNW-1; OBJ-11; ENUM-10, ENUM-P1; NS-12; NEW-P5; ICS-45; DEP-15, DEP-P4; CTAD-17; CTAD-16; CMP-8 | 204 | at most 204 |
| 5 | Decide the row; answers may change | 1: Q-0235 | 3 | at most 3 |
| 6 | Decide the row; answers may change | 2: Q-0041, Q-0343 | 5 | at most 5 |
| 7 | Decide the row; answers may change | 5: Q-0007, Q-0087, Q-0116, Q-0181, Q-0188 | 5 | at most 5 |
| 8 | Decide the row; answers may change | none | 0 | 0 |
| 9 | Decide the row; answers may change | 9: Q-0006, Q-0008, Q-0116, Q-0119, Q-0182, Q-0183, Q-0186, Q-0312, Q-0412 | 1 | at most 1 |
| 10 | Record each answer as Tidal's own choice; answers stay | 17: Q-0020, Q-0079, Q-0165, Q-0176, Q-0179, Q-0180, Q-0184, Q-0185, Q-0321, Q-0325, Q-0349, Q-0361, Q-0372, Q-0381, Q-0384, Q-0423, Q-0431 | 32 | 0 |

Rows 1 to 9 hold 119 of the 199 open questions (60%). With row 10, the rows hold 134 (67%).

83 open questions wait for a review that supports the answer with no doubt. By what the last review found: 56 doubt, 17 silent, 9 against, 1 part.

## Matrix against review

Row 4 holds these items. In each, a source-reader gap review reads the
C++23 text against the answer that Tidal keeps. A family cell says
"matrix" when the newest version of the family gives the answer Tidal
keeps, and "review" when it gives the review's reading. EDG runs no
program, so it has no result for a run-time case. The review column
names files in `reports/review/`. Tidal keeps the matrix's answer in
each item, except Q-0307, where Tidal follows the review.

| Item | Case | Clang 22 | GCC 16 | EDG 7.0 | Review |
|---|---|---|---|---|---|
| Q-0300 | the aligned allocation form for a type without extended alignment | matrix | matrix | matrix | `2026-11-01-gap-review-question-cluster-1` |
| Q-0181 | the explicit object parameter initialized from a prvalue | matrix (a later change) | matrix (a later change) | matrix (a later change) | `2026-11-01-gap-review-question-cluster-1` |
| Q-0243 | a deduced class template name after a dependent qualifier | matrix | matrix | matrix | `2026-10-31-gap-review-packs-and-ctad`, `2026-11-01-gap-review-question-cluster-1` |
| Q-0226 | the deduced form, which waits with Q-0243 | review | matrix | matrix | `2026-11-03-gap-review-question-cluster-6`, `2026-11-11-gap-review-phase-14-expressions-overloading-templates`, `2026-11-11-gap-review-phase-15-expressions-overloading-templates` |
| Q-0275 | deduction of a template template parameter | matrix | matrix | matrix | `2026-11-01-gap-review-question-cluster-1` |
| Q-0277 | the first form, a string literal for an array member of dependent type | matrix | matrix | matrix | `2026-11-03-gap-review-question-cluster-5` |
| Q-0280 | the two guide templates with the same parameters | matrix | matrix | matrix | `2026-11-03-gap-review-question-cluster-5` |
| Q-0225 | `T::box<int>` as an alias target without `typename` | matrix | matrix | matrix | `2026-11-03-gap-review-question-cluster-6`, `2026-11-11-gap-review-phase-14-expressions-overloading-templates`, `2026-11-11-gap-review-phase-15-expressions-overloading-templates` |
| Q-0307 | the read through an old base pointer after a replacement, in constant evaluation | matrix | matrix | matrix | `2026-11-03-gap-review-question-cluster-4`, `2026-11-03-gap-review-object-model` |
| Q-0307 | the same read for a `const` object | review | matrix | matrix | `2026-11-03-gap-review-question-cluster-4`, `2026-11-03-gap-review-object-model` |
| Q-0336 | the read of an enumeration value that a byte copy stored | matrix (the sanitizer stops the read) | matrix (the sanitizer stops the read) | runs no program | `2026-11-03-gap-review-object-model` |
| THR-3 | a usable copy constructor when no copy runs | matrix | matrix | matrix | `2026-11-03-gap-review-exceptions` |
| TRY-2 | a handler of a reference to a pointer that is not `const` | matrix | matrix | runs no program | `2026-11-03-gap-review-exceptions` |
| UNW-3 | the global deallocation form for a single object with extended alignment after a throw; UNW-3 allows both forms | matrix | matrix | runs no program | `2026-11-03-gap-review-exceptions`, `2026-11-10-gap-review-phase-13-classes-declarations-foundations`, `2026-11-11-gap-review-phase-14-expressions-overloading-templates` |
| NAM-40, NAM-41 | a class name in the type of a conversion function | matrix | matrix | matrix | `2026-11-03-gap-review-names` |
| STG-9 | a new object in the place of a member of a base subobject | matrix | matrix | matrix | `2026-11-03-gap-review-object-model` |
| LSUP-11 | an `std::allocator` whose element type is not a literal type | matrix | matrix | matrix | `2026-11-03-gap-review-object-model`, `2026-11-10-gap-review-phase-13-classes-declarations-foundations` |
| TR-P6, TR-13, TR-14, TR-15 | rejection of a program past a translation limit | matrix | matrix | matrix | `2026-11-03-gap-review-program-structure` |
| Q-0140 | a call that uses equal default arguments from two scopes; OVL-38's test of when two default arguments are the same is Tidal's own | matrix | matrix | matrix | `2026-10-31-gap-review-overload-resolution`, `2026-11-03-gap-review-questions-overloading`, `2026-11-04-gap-review-phase-8-classes-overloading` |
| Q-0142 | the address of two plain functions with different parameter lists and no target; only Clang 18 follows the review | matrix | matrix | matrix | `2026-11-03-gap-review-questions-overloading` |
| Q-0246 | a diagnostic for a second lookup that finds one different member | matrix | matrix | matrix | `2026-11-03-gap-review-questions-templates` |
| Q-0205 | an explicit specialization, after a use, of an unscoped member enumeration that the class template only declares | matrix | review | matrix | `2026-11-03-gap-review-questions-templates` |
| Q-0087 | an explicit object member function with the name and parameters of a base virtual function | matrix (a later change) | matrix (a later change) | review | `2026-11-03-gap-review-questions-classes-statements` |
| Q-0131 | a qualified redeclaration of a friend function that only a class declares | review | matrix | matrix | `2026-11-03-gap-review-questions-classes-statements` |
| Q-0190 | the ref-qualifiers in the match of `!=` and `==` | matrix | review | matrix | `2026-11-03-gap-review-questions-classes-statements` |
| Q-0190 | the cv-qualifiers in the same match | matrix | matrix | matrix | `2026-11-03-gap-review-questions-classes-statements` |
| Q-0074 | language linkage in a function type | matrix | matrix | review | `2026-11-03-gap-review-questions-expressions-declarations` |
| Q-0077 | `throw()` | matrix | matrix | review | `2026-11-03-gap-review-questions-expressions-declarations` |
| Q-0123 | an attribute before the cv-qualifiers of a pointer | review | matrix | matrix | `2026-11-03-gap-review-questions-expressions-declarations` |
| Q-0191 | a dropped inner array bound without `const` above it | review | matrix | matrix | `2026-11-03-gap-review-questions-expressions-declarations` |
| Q-0330 | an unknown escape such as `\q` in a string literal | matrix | matrix | review | `2026-11-03-gap-review-questions-expressions-declarations` |
| UNW-1 | the place of a return statement's result in the order of destruction; UNW-1 keeps GCC's order | never destroys the result | matrix | runs no program | `2026-11-04-gap-review-phase-7-repairs` |
| OBJ-11 | a use of a `thread_local` variable with constant initialization starts no deferred construction | matrix | matrix | runs no program | `2026-11-04-gap-review-phase-7-additions` |
| Q-0362 | a member of an anonymous union that shares its name with a class | review | matrix | matrix | `2026-11-03-gap-review-names`, `2026-11-04-gap-review-phase-7-additions` |
| Q-0366 | `main` with `extern "C++"`, plain or braced | matrix | matrix | review | `2026-11-04-gap-review-phase-7-additions` |
| Q-0369 | a C name in an unnamed namespace | matrix | matrix | review | `2026-11-04-gap-review-phase-7-repairs` |
| Q-0371 | a qualified definition inside the member's own namespace | review | matrix | matrix | `2026-11-04-gap-review-phase-7-repairs` |
| VIRT-1, Q-0394 | a derived member function with no ref-qualifier against a base virtual function with one; VIRT-1 says it does not override, and Clang 18 follows the review | matrix | matrix | matrix | `2026-11-04-gap-review-phase-8-classes-overloading` |
| ACC-20, Q-0404 | a friend declaration that names a member through a qualifier such as `A<T*>::`, used by the member of a specialization that does not match; ACC-20 gives access | matrix | matrix | review | `2026-11-05-gap-review-phase-9-classes-overloading-exceptions` |
| COND-16, Q-0401 | a `Derived` prvalue against a `Base` xvalue in a conditional expression; COND-16 says the reference does not bind directly, so the result is a prvalue | matrix | review | matrix | `2026-11-05-gap-review-phase-9-expressions-templates` |
| ENUM-10, Q-0407 | an enumerator whose initializer is an enumerator of another unscoped enumeration; ENUM-10 gives the enumeration type | matrix | matrix | matrix | `2026-11-05-gap-review-phase-9-declarations-statements-foundations` |
| NAM-21, Q-0410 | the underlying type in the definition of an enumeration with a qualified name; NAM-21 does not look it up in the named scope | matrix | matrix | matrix | `2026-11-05-gap-review-phase-9-declarations-statements-foundations` |
| PFX-P3, CLS-2, Q-0406 | a default template argument of a member template; the class is not complete there | matrix | matrix | matrix | `2026-11-05-gap-review-phase-9-expressions-templates` |
| TARG-27, Q-0409 | an injected class name from two bases as a template template argument; TARG-27 rejects it as ambiguous | matrix | matrix | matrix | `2026-11-05-gap-review-phase-9-expressions-templates` |
| ORD-P1, Q-0405 | the address of an overload set with explicit template arguments and no target type; the address is ambiguous | matrix | matrix | matrix | `2026-11-05-gap-review-phase-9-expressions-templates` |
| CTAD-16, Q-0408 | an inherited guide with a derived parameter that the base does not use and that has a default; the default rescues the guide | review | matrix | matrix | `2026-11-05-gap-review-phase-9-expressions-templates` |
| OBJ-11, Q-0416 | a use of a constant-initialized `thread_local` and the deferred construction of the other `thread_local` variables of the unit; OBJ-11 says the use does not start it | matrix | matrix | runs no program | `2026-11-05-gap-review-phase-10-classes-declarations-foundations` |
| ENUM-10, ENUM-P1 | inside the list, an enumerator whose initializer has an unscoped enumeration type keeps that enumeration type; the review reads the underlying type | matrix | matrix | matrix | `2026-11-04-gap-review-phase-8-declarations-statements` |
| NS-12 | a class in an inline namespace and a variable of the same name in the enclosing namespace are ambiguous; the review sets the class aside | matrix | matrix | matrix | `2026-11-04-gap-review-phase-8-declarations-statements` |
| Q-0393 | the null pointer literal as the operand of `noexcept` | review | matrix | matrix | `2026-11-04-gap-review-phase-8-declarations-statements` |
| Q-0389 | a default argument that reads the value of a local constant | review | matrix | matrix | `2026-11-04-gap-review-phase-8-declarations-statements` |
| COND-16, Q-0418 | step 1 for a `Derived` xvalue against a `const Base` lvalue; COND-16 needs an lvalue, the review lets a reference to `const` bind the xvalue | review | matrix | matrix | `2026-11-05-gap-review-phase-12-expressions-overloading-templates` |
| COND-16, Q-0418 | step 1 for an `A` xvalue against a `const A` lvalue | matrix | matrix | matrix | `2026-11-05-gap-review-phase-12-expressions-overloading-templates` |
| NEW-P5 | `new T` for a class that declares only an aligned allocation form and a `T` that is not over-aligned; the matrix rejects, the review adds the alignment on a second search | matrix | matrix | matrix | `2026-11-05-gap-review-phase-12-expressions-overloading-templates` |
| ICS-45 | an `explicit` conversion function whose result is a class derived from `C`, for the copy or move constructor of `C` in direct initialization | matrix | matrix | matrix | `2026-11-05-gap-review-phase-12-expressions-overloading-templates` |
| CNS-11, CPY-15, Q-0426 | the constraint check of a function template candidate when a candidate that is not a template takes every argument with no conversion; CNS-11 skips the check, the review checks every template candidate | matrix | matrix | review | `2026-11-10-gap-review-phase-13-expressions-overloading-templates`, `2026-11-10-gap-review-phase-13-classes-declarations-foundations`, `2026-11-11-gap-review-phase-14-classes-declarations-foundations` |
| CPY-15, Q-0426 | the W20-3 copy of a `Holder` beside a constructor template whose constraint needs its own result; the program runs, the review checks the constraint for `Holder` | matrix | matrix | review (checks, then accepts) | `2026-11-10-gap-review-phase-13-expressions-overloading-templates` |
| DEP-15, DEP-P4 | `Box<M>` inside `Box<N>`, where `M` is a `static` constant member of the parameter's type initialized with `N` alone; DEP-15 names another specialization, the review the current instantiation | matrix | matrix | matrix | `2026-11-10-gap-review-phase-13-expressions-overloading-templates` |
| STG-29, ADD-14, Q-0430 | `p + 1` for a `T*` cast through `void*` from the address of a byte of a byte array, and placement `new` at that position; the matrix gives the address `sizeof(T)` bytes on, the reviews make the step undefined | matrix | matrix | runs no program | `2026-11-11-gap-review-phase-14-classes-declarations-foundations`, `2026-11-11-gap-review-phase-14-expressions-overloading-templates`, `2026-11-11-read-of-rv7-repair-diff` |
| STG-28, Q-0430 | a `T*` cast from the result of an allocation function plus a multiple of `sizeof(T)` bytes, also through `unsigned char*`, and placement `new` at that address; STG-28 gives element `k` of a `T` array and lets the `T*` reach the new object; part A of the RV7 review gives a pointer to a byte, part B lets the allocation create the `T` array, the read of the RV7 repair leans toward part A, and the phase 15 review reads the case against for each choice of created objects; element 0 with no byte arithmetic agrees | matrix | matrix | runs no program | `2026-11-11-gap-review-phase-14-classes-declarations-foundations`, `2026-11-11-read-of-rv7-repair-diff`, `2026-11-11-gap-review-phase-15-classes-declarations-foundations` |
| ADD-14, Q-0442 | `p + 1` with no read, for a pointer to the signed or unsigned integer type that matches the element type of an array, such as an `unsigned*` to an element of an `int` array; ADD-14 gives the position one past, the reviews make any move undefined. For a `B*` to a base class subobject (Q-0049, settled), the read of the RV7 repair agrees with ADD-14 | matrix | matrix | runs no program | `2026-11-11-gap-review-phase-14-expressions-overloading-templates`, `2026-11-11-read-of-rv7-repair-diff` |
| CTAD-2, Q-0439 | `static inline Box b;` in a class, with no initializer, for a class template `Box` whose parameter has a default; CTAD-2 rejects, the review deduces with no arguments, as for a variable | matrix | review | matrix | `2026-11-11-gap-review-phase-15-expressions-overloading-templates` |
| CTAD-17 | copy initialization from a class whose conversion function gives the parameter type of a user guide; CTAD-17 keeps the guide viable, the review applies the limit on a user-defined conversion for the first parameter, which is Clang 18's answer | matrix | matrix | matrix | `2026-11-11-gap-review-phase-15-expressions-overloading-templates` |
| CTAD-16 | the guide with no parameters and the aggregate candidate of a base, for a class template that inherits its constructors; CTAD-16 keeps them with the base, the review gives the derived class the base's whole guide set | matrix (Clang makes no inherited guides) | matrix | matrix | `2026-11-11-gap-review-phase-15-expressions-overloading-templates` |
| CMP-8 | a relational comparison of pointers to two members of a union, or to a member that occupies no storage, during constant evaluation; Tidal keeps a result | matrix | matrix | matrix | `2026-11-04-gap-review-phase-8-expressions-foundations-templates` |

## EDG report

Clang 22 and GCC 16 agree, and EDG 7.0 differs, in these questions of
cluster 5. Questions marked "settled" settled under decision 0054, and
EDG's result stays a recorded deviation.

| Group | Questions |
|---|---|
| EDG rejects a program that Tidal accepts (13) | Q-0198, Q-0201, Q-0203, Q-0208 (settled), Q-0229 (settled), Q-0231 (settled), Q-0240 (settled), Q-0256 (settled), Q-0260 (settled), Q-0266, Q-0268 (settled), Q-0277, Q-0281 (settled) |
| EDG accepts a program that Tidal rejects (13) | Q-0146 (settled), Q-0170 (settled), Q-0172 (settled), Q-0224 (settled), Q-0237 (settled), Q-0245 (settled), Q-0265 (settled), Q-0267 (settled), Q-0272, Q-0278, Q-0280, Q-0286 (settled), Q-0292 (settled) |
| EDG follows another order or model of a step (3) | Q-0215 (settled), Q-0279 (settled), Q-0284 (settled) |
| EDG diagnoses a program that needs no diagnostic (1) | Q-0283 (settled) |
| Of the same kind, from cluster 3 (2) | Q-0214, Q-0285 (settled) |

The report leaves out four questions where the cluster 5 review reads
the C++23 text as EDG does:

- Q-0195 and Q-0197: TPL-35 and TPL-38 require a diagnostic, as EDG
  gives one. Clang 22 and GCC 16 accept the programs.
- Q-0235 and Q-0282: the review reads them as EDG does, with doubt.
  Tidal keeps the answer of Clang 22 and GCC 16. The maintainer can
  decide either question.

## Later-version probe

The plan below is what row 7 asks the maintainer to approve. Nothing in
it runs until then.

1. **A runner option.** `tools/run_tests.py` gains an option that
   replaces each compiler's C++23 flag with its newest language mode
   (`-std=c++2c` on Clang 22 and GCC 16, and EDG 7.0's newest mode) and
   runs only the tests whose expectation for that compiler is `reject`.
   The option writes its own results file and leaves `tests/RESULTS.md`
   and the cache of C++23 results alone. Cost: one session, with a test
   of the option on one directory.
2. **One matrix run.** The run covers about 2640 tests that expect
   `reject`, on the three newest compilers, so about 7900 compilations.
   No test runs a program. Cost: one run of the matrix at its usual
   speed.
3. **Triage.** For each test that a newest compiler accepts in its
   newest mode, a clean-room writer records the test, the compilers that
   accept it, and the rule the test cites. The writer gives no reason
   for a flip, because a reason would need later-version material
   (Q-0116 keeps the source-reader designer as the channel for one).
   Cost: one session per few hundred flips. The count of flips is not
   known before the run.
4. **An index with no rules.** The flips go in one generated index
   under `reports/`, sorted by page and rule. No rule, class, or test
   expectation changes, and no page gains a `:since:` field. A Propose
   report can cite the index in its version line.

The index gives data for Q-0007, and for Q-0087, Q-0181, and Q-0188,
where a newest family follows a change after C++23. The risk is in
step 3, where a writer can recall later-version material. The index
names compiler behavior only.

## Process questions

| Question | Tidal keeps |
|---|---|
| Q-0006: when a report compiler becomes a baseline compiler | After one clean review of its results |
| Q-0008: the similarity thresholds | An 8-word run and 0.25 overlap |
| Q-0116: a designer persona that is a source reader | The clean-room designer only |
| Q-0119: a diagram renderer | Plain text |
| Q-0182: reasons in Evidence | Only when a clean input states the reason |
| Q-0183: a cost model for proposals | None. A proposal states cost in words |
| Q-0186: judgments from outside Tidal in a proposal | Allowed when marked as outside knowledge |
| Q-0312: a list of token sequences that no production accepts | TR-12 gives the grammar its effect, and no rule lists such sequences. The maintainer decides whether the roadmap also keeps a list (option 3) |
| Q-0412: which effects a standard attribute may have (X22-6) | Placement rules, warnings, promises, and layout changes only; a new attribute adds no required rejection |
