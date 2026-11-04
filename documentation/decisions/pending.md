# Decisions that wait on the maintainer

Each row is one item that waits on the maintainer (decision 0054). Tidal
keeps the answer in the third column until the maintainer decides. The
last column lists the questions that the decision closes or changes.

| # | Item | Options | Tidal keeps | Questions |
|---|---|---|---|---|
| 1 | The choices of decision 0054: settling by review, and program tests under `tests/programs/` | Confirm, change, or drop each choice | Both choices. A question settles by review only when at most one newest family differs from the answer in each form | Confirms the 128 questions settled under decision 0054 (34 by item SR in 0637b59, 1 by the repairs from the exceptions and names reviews in b00cbf2, 13 by the repairs from the object-model and program-structure reviews in 573d849, 36 by the question reviews of overloading and templates in e0d06f0, 40 by the question reviews of classes, statements, expressions, and declarations in 8c15b06, and 4 by phase 8 item RV, the repairs from the gap reviews of the text phase 7 wrote). `tools/question_index.py` writes the total and fails `--check` when the total is wrong. If the maintainer lets the rule settle a question where two families differ, these 15 settle too: Q-0017, Q-0021, Q-0121, Q-0133, Q-0148, Q-0153, Q-0159, Q-0216, Q-0217, Q-0227, Q-0234, Q-0301, Q-0320, Q-0339, Q-0341 |
| 2 | The choices of decision 0051: settling when the newest version of each family agrees, and keeping the implementation-only operations referenced | Confirm or change each choice | Both choices | Q-0322 |
| 3 | A report of EDG behavior to the EDG project ([groups below](#edg-report)) | Approve the report, or hold it | Tidal follows Clang 22 and GCC 16. Each test records EDG's result as an override | Q-0002, Q-0003, and the cluster 5 groups below |
| 4 | The matrix agrees, and a gap review reads the C++23 text the other way | Follow the review, against every compiler, or follow the matrix | The matrix's answer in each, except Q-0307, where Tidal follows the review | Q-0300, Q-0181, Q-0243 (and the deduced form of Q-0226), Q-0275, Q-0277 (first form), Q-0280 (the two guide templates), Q-0225 (`T::box<int>` as an alias target without `typename`), Q-0307, Q-0336 (the read of an enumeration value that a byte copy stored, where both sanitizers report the read). Rules with no question, from the gap reviews of the exceptions chapter and names: THR-3 (a usable copy constructor when no copy runs), TRY-2 (a handler of a reference to a pointer that is not `const`), UNW-3 (the global deallocation form for a single object with extended alignment, where UNW-3 allows both forms), NAM-40 and NAM-41 (a class name in the type of a conversion function). From the gap reviews of the object model and program structure: STG-9 (a new object in the place of a member of a base subobject), LSUP-11 (an `std::allocator` whose element type is not a literal type), TR-P6 with TR-13, TR-14, and TR-15 (rejection of a program past a translation limit). From the question reviews of overloading and templates: Q-0140 (a call that uses equal default arguments from two scopes), Q-0142 (the address of two plain functions with different parameter lists and no target, where only Clang 18 follows the review), Q-0246 (a diagnostic for a second lookup that finds one different member), Q-0205 (an explicit specialization, after a use, of an unscoped member enumeration that the class template only declares). From the question reviews of classes, statements, expressions, and declarations, where one newest family follows the review: Q-0087 (an explicit object member function with the name and parameters of a base virtual function, where EDG follows the C++23 text and Clang and GCC a later change), Q-0131 (a qualified redeclaration of a friend function that only a class declares, where Clang follows the review), Q-0190 (the cv-qualifiers and ref-qualifiers in the match of `!=` and `==`, where only GCC follows the review for ref-qualifiers and no compiler for cv-qualifiers), Q-0074 (language linkage in a function type, where EDG follows the review), Q-0077 (`throw()`, where EDG follows the review), Q-0123 (an attribute before the cv-qualifiers of a pointer, where Clang follows the review), Q-0191 (a dropped inner array bound without `const` above it, where Clang follows the review). From the gap reviews of the text phase 7 wrote, where no newest compiler that runs the program follows the review: UNW-1 (the result of a return statement ends after the local objects built before it, as GCC 16 does; Clang 22 never ends it), OBJ-11 (a use of a `thread_local` variable with constant initialization does not start the deferred construction of the other `thread_local` objects of its unit). Where one newest family follows the review: Q-0362 (a member of an anonymous union that shares its name with a class, where Clang follows the review), Q-0366 (`main` with `extern "C++"`, plain or braced, where EDG follows the review), Q-0369 (a C name in an unnamed namespace, where EDG follows the review), Q-0371 (a qualified definition inside the member's own namespace, where Clang follows the review), Q-0330 (an unknown escape such as `\q` in a string literal, where EDG follows the review) |
| 5 | The variable form of Q-0235 | Reject the unqualified name (GCC 16), or accept it (Clang 22, EDG 7.0, and the cluster 6 review, with doubt) | Reject. INST-21 keeps one answer for the class, function, and variable forms | Q-0235 |
| 6 | Q-0343 and Q-0041 | Follow the gap review of item O, or follow the compilers | The review's reading. CPY-19 says a class with a deleted destructor, or with every copy and move function deleted, is not trivially copyable, and every family's builtin query says it is. JMP-6 accepts the jump, as GCC does, and Clang and EDG reject it | Q-0343, Q-0041 |
| 7 | A probe of which C++23 rejections later language versions accept, as an index with no rules ([plan below](#later-version-probe)) | Run the probe as planned, or keep later versions out of scope | C++23 only. Newer features are rejected under strict mode | Q-0007. A choice to follow later changes also decides Q-0087, Q-0181, and Q-0188 |
| 8 | An outside reader for the Foundations chapter and two topic pages (decision 0024) | Ask the reviewer who prompted the principles-first change, or skip | No outside reading | None |
| 9 | Process questions that name no rule ([list below](#process-questions)) | Confirm or change each provisional answer | Each provisional answer | Q-0006, Q-0008, Q-0116, Q-0119, Q-0182, Q-0183, Q-0186, Q-0312 |

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
