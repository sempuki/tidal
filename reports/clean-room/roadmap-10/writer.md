# Roadmap, round 10: what to do after phase 12, from a clean-room writer's view

Date: 2026-11-06. Decision 0029, with decisions 0033, 0039, 0050, 0051, 0054, 0055, 0056, 0057, 0058, 0059, and 0060. A clean-room agent in the role of a clean-room writer, who drafts rules, Evidence, and tests from compiler behavior, prepared this report. It read AGENTS.md, CLAUDE.md, its own round 9 report `reports/clean-room/roadmap-9/writer.md` with the probes and scripts beside it, `documentation/status.md`, `documentation/roadmap.md` (the phase 12 table), decisions 0059 and 0060, `documentation/decisions/pending.md` with its packet, `git log a15a877d..ebc109b9` with every commit body, the phase 12 reports (`reports/clean-room/W20/report.md` and its reductions, `I5/report.md`, `ledger.md`, `rust-frontend/report-23.md`, `rust-frontend/ledger.md`, `rust-frontend/conformance.tsv`), the two phase 12 gap reviews in `reports/review/` (`2026-11-05-gap-review-phase-12-*`), the question files Q-0417 to Q-0422, and the rules and tests that its items name. It did not read the other round 10 reports, `inputs/`, `build/exercise-keys/`, or `frontend/`. Counts come from read-only `git`, `grep`, and the scripts in `writer-scratch/` beside this report (copied from round 9), and from read-only runs of `tools/check_specification.py` and `tools/question_index.py --check`. Comparisons with round 9 read commit a15a877d through `git show`. Thirteen probe grids ran with `tools/probe_grid.py` on the baseline compilers, and one probe ran on the host compilers (`writer-probes/README.md`). Commit read: `ebc109b9`.

## 1. State from the writer's view

### 1.1 Counts

| Measure | End of phase 11 (a15a877d) | Now (ebc109b9) |
|---|---|---|
| Rule entries | 1697 (15 withdrawn), 1682 live | 1698 (15 withdrawn), 1683 live; TR-17 is new |
| Principles | 343 | 343 |
| Rules and principles whose text changed in the phase | 17 rules and 9 principles on 20 pages | 32 rules and 4 principles on 23 pages; no class change |
| Rules and principles changed after RV5 read the tree (df4b5539) | none | 16 rules and 4 principles on 17 pages, all in 3bed9cb5 |
| Words on the 85 pages, without index and glossary | 266,614 | 269,253 |
| Evidence words on those pages | 68,087 | 68,855 |
| `cpp` example blocks | 220 | 222 |
| Tests (`*.cpp`, excluding `*.tu.cpp`) | 6711, of which 103 are program tests | 6790, of which 115 are program tests |
| Test files with a per-compiler override | 692 (854 override lines) | 703 (869 override lines) |
| Test files with an override for a newest family | 597 | 608: EDG alone 210, GCC alone 200, Clang alone 107, EDG and GCC 62, Clang and EDG 18, Clang and GCC 10, all three 1 |
| Tests with `tidal-error` patterns | 3070 | 3094 |
| Question files | 418, 174 open | 422, 176 open |
| `check_specification.py` | 0 problems, 28 override-only warnings | 1698 rules, 0 problems, 0 warnings (QR recorded a reason for each of the 28) |
| `question_index.py --check` | 0 problems, 5 family-check warnings | 422 questions, 0 problems, 0 warnings |
| Front end (`conformance.tsv`) | round 22 baseline: 6459 pass of 6711 | 6586 of 6790 pass, 6626 pass their check |
| Tests that fail their check | 182: 116 missed rejects, 60 unsupported, 4 false rejects, 2 EDG-AGREES | 164: 100 missed rejects, 58 unsupported, 4 false rejects, 2 EDG-AGREES |
| Tests that pass their check and do not run | 40 run-unsupported | 40 run-unsupported |
| Exercise ledger | 867 rows: 762 commit, 68 open, 26 planned, 5 text, 3 question, 3 out of scope | 925 rows: 878 commit, 17 open, 0 planned, 17 text, 10 question, 3 out of scope |
| Front-end ledger | 152 rows, 0 open | 157 rows: 146 commit, 5 open (G153 to G157), 4 text, 1 question, 1 out of scope |
| Committed similarity reports | 193 (27 MB, 363,607 lines) | 210 (34 MB, 448,994 lines) |

Phase 12 added 79 tests and one rule. It opened 4 questions (Q-0419 to Q-0422) and settled 2 (Q-0323, Q-0383).

### 1.2 What phase 12 took from round 9

| Round 9 item | Phase 12 result |
|---|---|
| 1. FE23 | Done in 1017b19c. 18 better, 105 new tests pass, the COND procedure is one component. Five new findings, G153 to G157, all `open`. |
| 2. F2, the 20 planned rows | Done in 03e578c4, 3b83788d, 0f3f522f. Q-0419 holds GCC's ambiguity of I3-13. |
| 3. RD, the RV4 points a probe decides | Done in 6c6f69f8, with TR-17. |
| 4. QR, question records | Done in da51ae9e. Q-0323 settled; Q-0421 holds its assignment form. The family check reports nothing. |
| 5. RV5, gap review of phase 11 and 12 text | Done in 1cd74878, 3bed9cb5, f7641eb3. 33 review points, each with a ledger row. |
| 6. OV, reasons for the 28 override-only rules | Done inside QR. The warning list is empty. |
| 7. LO, triage of the 68 open rows | Done in L2 (13c03237, f10c661a). |
| Process 1, a deferred row carries its probe | Adopted by decision 0059. No row was deferred in phase 12. |
| Process 2, review points get ledger rows | Adopted. RV5-A-1 to RV5-A-18 and RV5-B-1 to RV5-B-15 have rows. |
| Process 3, the commit is chained to `checks.py` | Adopted. |
| Process 4, the family check reads grids | Done in QR. |
| Process 5, a save step for reader reports | Not done. The tool refused the writes of the FE23, W20, and I5 reports, and the integrator saved each from the returned text. |
| Process 6, smaller similarity reports | Not done. Phase 12 added 17 reports and 85,387 lines, twice the lines of phase 11. |

### 1.3 The 17 open ledger rows and G153 to G157, with the probes of this round

Decision 0059 says a front-end finding gets a test in the commit that records it. The W20 and I5 commits (1056cca2, 2bb7d26c) and the round 23 commit (1017b19c) add no test for W20-1, W20-2, W20-3, I5-9, I5-10, or G153 to G157. The W20 commit adds the four programs as program tests.

| Row | What the probe shows | What the text needs | Question |
|---|---|---|---|
| W20-1 | No probe. The front end reports PFX-4 on `rows-not-contiguous.cpp`; GCC and Clang with sanitizers report nothing. | A sentence beside the `FixedVec` example in STG-P2: a pointer to one row's object does not reach the next. A `run-fail` test from the reduction with `tidal-interpreter: run-fail` (the compilers cannot see the fault). | none |
| W20-2 | `w20-2-union-array-member`: `construct_at` on an element of an array member of a union in constant evaluation. Clang 18 and 22 and GCC 14 reject every form; GCC 16 and EDG accept. An assignment to an `int` element (UNI-13) is accepted everywhere. | A rule on the unions page for an element of an array member: what building one element does to the member, and whether a pointer to element 0 reaches element 1. Two constant tests with a Clang override (two of three newest families accept). An EDG override on the class-element assignment, where EDG alone accepts. | New: Clang differs from GCC 16 and EDG. |
| W20-3 | `w20-3-self-by-value-constraint`: every newest compiler accepts `Boxable<Box>` for `Box(T)` and for `Box(const T&)`. Clang 18 alone reports that the constraint depends on itself. | A sentence in CPY-15 or CNS-11: the by-value specialization leaves the candidates before its constraints are checked, and the `Box(const T&)` case needs no recursive check either. One accept test with a `clang-18` override. The front end then has a false reject to fix. | none (newest families agree) |
| W20-4 | `w20-4-tag-address-*`: with `-fsanitize=undefined` or `-fsanitize=null`, GCC 14 and 16 reject `!=` on the addresses of two variable template specializations, two static members of a class template, and two `inline constexpr` variables, in a `static_assert`. Plain `constexpr` variables pass. Clang accepts all; every compiler accepts all without the flags. | An Evidence note in CMP-15. A test only if the matrix builds constant tests with a sanitizer flag; otherwise the note alone. | none |
| W20-5 | `w20-5-void-pointer-round-trip-constant`: every compiler rejects `static_cast` from `void*` to an object pointer in constant evaluation, also back to the original type. | One CAST-1 sentence for the round trip that links STG-16 and the constant-evaluation page's existing sentence on the cast. One reject constant test with no override. | none |
| W20-6 | No probe. | A table of the storage forms beside the `FixedVec` and `FixedStack` examples: constant evaluation, a contiguous `T*`, a default constructor. The W20-2 rule fills the union-array row. | none |
| I5-1 | Q-0420 holds the behavior. | A DED-30 sentence that points to DED-29's paragraph on an address with a target type. | Q-0420 |
| I5-2 | No probe; the reader's programs agree on all three implementations. | The first list of DED-29 names "a body that the function type needs". | none |
| I5-3 | The `call-types` grid already shows it. | A DED-7 or DED-14 sentence: `const` and `volatile` on a function type are dropped, so `const T` matches a function type. | none |
| I5-4 | `i5-deduction-boundaries`: every compiler, EDG included, rejects two equal `Base<int>` bases, a private base, and a protected base, by reference and by pointer, with a fallback present. | A DED-15 sentence pointing to OVL-37. Three reject tests with no override. | none |
| I5-5 | The same grid: every compiler selects the fallback for `T::type` with an incomplete `T`. | The DED-24 list names an incomplete class. INST-1's list names a qualified name. One accept test with no override, which also serves I5-9. | none |
| I5-6 | `i5-6-const-prvalue`: every compiler gives `T = int` for a `const int` prvalue and `T = const C` for a `const C` class prvalue. | A DED-8 sentence on the cv-qualifiers of a non-class prvalue. Two tests with no override. | none |
| I5-7 | The reader's programs agree on GCC, Clang, and the front end. | An example under DED-20. | none |
| I5-8 | The same grid: every compiler selects the fallback for `f(const T**)` with `int**`, and the template for `f(const T* const*)`. | DED-P4 table rows for both cases, and a sentence that names the rule that removes the template. One accept test. | none |
| I5-9 | Front end. | The I5-5 test is its target. | none |
| I5-10 | Front end. The reader found GCC and Clang reject both reductions. | Two reject tests (a reference to a private base, a reference to an ambiguous base) outside the conditional operator, citing REF-1, ACC-12, CONV-17. | none |
| I5-11 | The reader's `p05` agrees on GCC, Clang, and the front end. | DED-24 names an array of functions and an array of `void`. DED-11 names the size type as `decltype(sizeof 0)`. | none |
| G153 | `g153-cond-private-base`: in a friend, where the base is accessible, every compiler gives a `Root` lvalue; outside, the expression is not valid for a private or an ambiguous base everywhere. The probe cannot tell the two readings apart, because both reject. | A COND-16 sentence: steps 1 and 2 ask only for the base relation, and ACC-12 and CONV-17 apply to the conversion made. This is the front end's reading and the test notes' reading. RV5-B-1 and Q-0418 touch the same step, so the sentence must leave Q-0418's xvalue case as it is. | none |
| G154 | `g154-private-copy-selected`: every compiler rejects six forms (copy-initialization, by-value argument, return of a parameter, return of a local, and two private-move forms) and accepts two prvalue controls. | Six reject tests and two accept tests that cite ACC-3 and the copy-move rule that selects the constructor. No text change. | none |
| G155 | No probe needed; the two tests exist. | A reject rule, or a wider NEW-25, for an allocation function that a `new` expression selects and that is deleted or not accessible. NEW-29 keeps the selection. A new reject rule is a class change (decision 0034). Q-0417 holds the fallback. | Q-0417 |
| G156 | `g156-assign-before-construction-types`: the Q-0421 split holds for `double`, a pointer, an enumeration, `char`, a whole simple-class element, and a class with its own assignment operator. Clang rejects each; GCC and EDG accept each. | LSUP-11 states the rule for every element type. Q-0421's **Affects** names one or two more tests. | Q-0421 |
| G157 | `g157-virtual-instantiation-events`: every compiler instantiates an unnamed virtual function for an object at namespace or block scope, a `new` expression, a temporary, and an object of a derived class; none does for `sizeof`, a pointer, a derived class with no object, or a nonvirtual call. A call of the virtual function through a pointer: GCC and Clang instantiate, EDG does not. An explicit instantiation declaration before an object: GCC and Clang do not instantiate, EDG does. | An Evidence list in INST-11 with the agreed uses. Tests for `new` and a temporary, as G157 asks, with no override. The named call belongs to INST-7 (a call needs the definition), so a test of it has an EDG override. | New, only if the named-call case or the explicit instantiation declaration case leads to a rule change. An EDG override needs none. |

Seven rows need only a sentence or an example (W20-6, I5-1, I5-2, I5-3, I5-7, I5-11, and the W20-1 sentence). Ten need tests. Two need a new question (W20-2; G157 if the writer changes INST-7). The probes decide every row that depends on compiler behavior.

### 1.4 The questions phase 12 opened

| Question | **Waits for:** | What a writer can do now |
|---|---|---|
| Q-0419 (I3-13, list sequence element) | a review (doubt) | Nothing that a probe adds. RV5-B-8 reads the same-function case as supported and the mixed case as Tidal's own choice. The mixed case fits `pending.md` row 10 ("record as Tidal's own choice"), and the same-function case can settle under decision 0054 if GCC is the one family that differs. Splitting the question that way is a record change. |
| Q-0420 (DED-29 address) | newest families that agree | Nothing. GCC and EDG differ from the answer, and the review supports it with no doubt. Packet row 1 lists it. |
| Q-0421 (assign before construction) | newest families that agree; a review (doubt) | The review's doubt is that the tests declare their own `std::allocator`. `q-0421-library-allocator.cpp` shows the same split with `<memory>`'s own allocator on GCC 16 and Clang 22. A dated note in the question answers the doubt. G156 widens **Affects**. The family item stays, because GCC and EDG differ. |
| Q-0422 (structured binding in a return) | a first review | The next gap review reads it. `q-0422-binding-category` shows EDG's type is `int&&`, also for a binding declared `auto&`. The Context can name that type. |

### 1.5 The pending.md row 4 items that phase 12 added

| Item | Families | What it waits for |
|---|---|---|
| Q-0418 (COND-16 step 1, xvalue) | Derived case: Clang follows the review, GCC and EDG keep the matrix. One-class case: every family keeps the matrix. | Row 4b. The maintainer chooses the review or the matrix. |
| NEW-P5 (RV5-B-4, an added alignment argument) | Every family keeps the matrix (all reject `only-aligned-class-form-ordinary-type.cpp`). | Row 4. NEW-P5 has no question, so the packet's 4a count of 18 questions does not include it. |
| ICS-45 (RV5-B-9, an explicit conversion to a derived class) | Every family keeps the matrix (all reject `explicit-conversion-to-derived-direct-init.cpp`). | Row 4, the same as NEW-P5. |

No writer step reaches these three. Each has one test whose expectation changes if the maintainer follows the review. The packet's tool counts questions only, so its row 4a line does not show the two rule items.

### 1.6 The 20 rules and principles that changed after RV5 read the tree

The close lists CLS-26, BF-6, ALN-4, AUTO-P3, RET-6, REF-1, SINIT-6, UNW-3, LAM-5, LAM-6, LAM-7, CONV-1, NAM-21, STG-5, STG-P3, TR-P1, ICS-11, JMP-10, SW-P1, and ORD-13. `changed.py` from df4b5539 to ebc109b9 gives the same 16 rules and 4 principles, all from 3bed9cb5. The RV5 repairs also added Evidence notes to COND-16, COND-3, ICS-17, NEW-22, NEW-P5, ICS-45, and TR-17; rule bodies outside Evidence did not change for these. Ranked by the writer's need for a second reading:

1. REF-1 and ICS-11. REF-1 now covers every implicit pointer conversion of the lvalue's type, and ICS-11 adds direct binding after materialization. G153 and I5-10 rest on REF-1's base case, and COND-16 step 1 reads REF-1.
2. JMP-10, RET-6, AUTO-P3. They carry Q-0422, which waits for a first review.
3. STG-5, STG-P3, ALN-4, UNW-3. The storage page is where W20-1, W20-2, and W20-6 add text.
4. LAM-5, LAM-6, LAM-7. "Simple capture" replaces "named capture" on three rules.
5. CLS-26, CONV-1, BF-6, NAM-21, SINIT-6, ORD-13, TR-P1, SW-P1. Each change is one sentence or a clause.

### 1.7 Distance to decision 0060's completion test

The second condition holds: every program test runs (report-23 section 6). For the first, 164 tests fail their check, down from 182 at the round 22 baseline. The round 23 report counts 204, because it adds the 40 run-unsupported tests. Decision 0060 says "passes its check", and `frontend_check.py` counts run-unsupported tests as passing their check, so the two counts differ. No front-end ledger row marks a test `out of scope` except one older row.

Writer work touches the count in three ways:

- New tests from item F3 add front-end targets: about 6 that fail at first (W20-2 two, W20-3 one, I5-10 two, the I5-5 test while I5-9 stands), so the count rises before round 24 lowers it.
- Two false rejects (`ellipsis-argument-conversions.cpp`, `ellipsis-nullptr-becomes-void-pointer.cpp`) include `<cstdarg>`. The writer can make each header-free if a declared form exists, or the integrator can mark each `out of scope` with the reason. `include-missing-file-angle.cpp` tests an include itself.
- The two EDG-AGREES tests (`trivially-copyable-member-template-selected.cpp`, `friend-distinct-from-namespace.cpp`) keep the reasons of report 21.

At 18 fewer per round, the count reaches zero in about nine rounds. Decision 0060 allows a roadmap round to propose the rewrite earlier only when the remaining failures are few and stable over two rounds; 164 is neither.

## 2. Proposed items, ranked

| Rank | Item | Evidence | Cost (writer sessions) | Similarity risk | Unblocks |
|---|---|---|---|---|---|
| 1 | F3. Answer the 17 open rows and the text parts of G153 to G157 | section 1.3; 13 probe grids | 1 to 1.25 | low | FE24 targets; text that RV6 reads |
| 2 | FE24. Front-end round 24 | G153 to G157, I5-9, I5-10, W20-2, W20-3; 164 failing checks | 0.25 to 0.5 | low | the completion test |
| 3 | RV6. Gap review of the 20 changed rules, F3's text, Q-0422 | section 1.6; Q-0422 waits for a first review | 0.5 of repairs | medium | first review of Q-0422 and of the new W20-2 question |
| 4 | QR2. Question records the writer can repair | Q-0419 split, Q-0421 doubt, Q-0422 Context | 0.25 | low | a likely settlement of part of Q-0419 |
| 5 | PK4. The packet counts rule items of row 4 | NEW-P5, ICS-45, and the older rule-only rows (NS-12, CMP-8, others) | 0.1 | low | the maintainer sees every item that row 4 holds |
| 6 | OS. Front-end out-of-scope rows for header tests | the two `<cstdarg>` tests | 0.1 | low | two fewer failures that no round can fix |

Items 1, 4, 5, and 6 cost about 1.5 writer sessions. With FE24, about 2.

### Item 1. F3. Answer the open rows with the probes of this round

- **What.** Section 1.3, row by row. In order:
  - The deduction page (I5-1 to I5-8, I5-11): seven sentences, a DED-P4 row pair, one example, and eight tests (three for I5-4, one for I5-5, two for I5-6, one for I5-8, and one for I5-3 if the grid is not enough).
  - The storage and union pages (W20-1, W20-2, W20-6): a sentence, a rule, a table, a `run-fail` test, two constant tests, and the new question.
  - W20-3, W20-5: two sentences and two tests.
  - W20-4, G157: Evidence notes; two INST-11 tests and one INST-7 test with an EDG override.
  - G153: one COND-16 sentence. G155: a reject rule or a wider NEW-25. G156: a wider LSUP-11 and one or two tests in Q-0421. G154: eight tests with no text change.
  - I5-10: two reject tests.
- **Needs first.** Nothing. The probes in `writer-probes/` decide every behavior the rows depend on.
- **Evidence.** 17 ledger rows in state `open`, 5 front-end rows in state `open`, the probe table in section 1.3.
- **Cost.** 1 to 1.25 sessions. About 25 tests.
- **Risk.** Low. The deduction page is long, and seven of the changes land there. A new reject rule for G155 changes a class.
- **Unblocks.** FE24 gets a test for every front-end finding. RV6 reads the new text.

### Item 2. FE24. Front-end round 24

- **What.** G153 to G157, I5-9, I5-10, the W20-3 false reject, the W20-2 subscript of an array member, the F3 tests, then the 164 failing checks by group.
- **Needs first.** F3's tests and the G153 sentence. Decision 0059 says a front-end round runs in each phase that changes rules.
- **Evidence.** Section 1.7, report-23 sections 3 and 5.
- **Cost.** 0.25 to 0.5 writer session to answer findings.
- **Risk.** Low.
- **Unblocks.** The distance to decision 0060.

### Item 3. RV6. A gap review of the text phases 12 and 13 change

- **What.** One source-reader review of the 20 rules and principles of section 1.6, F3's text (the deduction page, the unions and storage pages, CPY-15 or CNS-11, NEW-25, LSUP-11, COND-16), and the first reading of Q-0422 and the new W20-2 question. Then one repair pass with point IDs and ledger rows.
- **Needs first.** F3.
- **Evidence.** The close's list; Q-0422's **Waits for:** line.
- **Cost.** 0.5 writer session of repairs, plus one source-reader session.
- **Risk.** Medium. The union array rule is new text in an area with a compiler split.
- **Unblocks.** The last first review that phase 12 left.

### Item 4. QR2. Question records

- **What.**
  - Q-0419: split the same-function case from the mixed case. The same-function case has review support; GCC alone differs. The mixed case moves toward row 10.
  - Q-0421: a dated note that the split holds with the library's allocator (`q-0421-library-allocator.cpp`), so the doubt about the declared allocator has an answer from behavior. The review still decides whether the text exempts a declared template.
  - Q-0422: the Context names EDG's type, `int&&`.
- **Needs first.** Nothing.
- **Cost.** 0.25 session.
- **Risk.** Low.
- **Unblocks.** A settlement of Q-0419's first part under decision 0054.

### Item 5. PK4. The packet counts rule items of row 4

- **What.** `question_index.py` reads the row 4 table and lists rule items that hold no question (NEW-P5, ICS-45, and older ones), with the tests whose expectation changes.
- **Needs first.** Nothing.
- **Evidence.** Section 1.5.
- **Cost.** 0.1 session.
- **Risk.** Low.

### Item 6. OS. Out-of-scope rows for header tests

- **What.** For each failing test that includes a library header, either a header-free form or a front-end ledger row `out of scope` with the reason.
- **Needs first.** Nothing.
- **Evidence.** Section 1.7.
- **Cost.** 0.1 session.
- **Risk.** Low.

### What the rules need before front-end round 24 and the next exercises

- **Before FE24.** The G153 sentence in COND-16, so that the round implements one reading. Tests for I5-9, I5-10, W20-2, W20-3, and G154, so the harness sees each finding. The W20-2 rule, or at least its question, so that the front end knows whether a subscript of an inactive array member is valid in constant evaluation.
- **Before a second Write suite on library-shaped code.** W20-1, W20-2, and W20-6 together: a reader who wants a contiguous in-object buffer needs the table and the union-array rule.
- **Before a second Implement on deduction.** I5-3, I5-5, I5-6, and I5-8 add four steps that the I5 component took from outside knowledge or guessed. The I5 reader did not read the conversion-sequence page; an Implement prompt that includes it removes most of I5-11.
- **Diagnose.** Unchanged: it needs no rule change first.
- **Propose X24.** F3 changes DED and COND text only by sentences. A P26 proposal that touches neither waits on nothing here.

## 3. Out of scope

1. **No new topic.** Phase 12 added one rule (TR-17) and no page.
2. **Coroutines, modules, threads and the memory model, extended floating-point types, the preprocessor beyond `#include "file"`, and the general library stay out**, as the charter and decision 0051 say.
3. **The maintainer's rows.** 111 of 176 open questions wait on rows 1 to 9 (63%), 124 with row 10 (70%), the same share as at the start of phase 12. Q-0418, NEW-P5, and ICS-45 are among them. The writer does not choose for the maintainer.
4. **A large drop in the open count.** F3 opens one or two questions. QR2 can settle part of Q-0419.
5. **The front-end rewrite.** 164 tests fail their check.
6. **MSVC and later versions** stay deferred (row 7).
7. **A sanitizer constant test for W20-4**, unless the matrix already builds constant tests with sanitizer flags. The Evidence note carries the fact.

## 4. Process changes

Ranked by writer time saved.

1. **A front-end finding gets its test in the same commit, in exercise commits too.** Decision 0059 states the choice, and the W20, I5, and round 23 commits recorded ten front-end findings with no test (section 1.3). A check can compare front-end and exercise ledger rows of kind "front end" with the tests that name them.
2. **A reader report lists its compiler-only claims for a probe.** W20-3, W20-4, I5-4, I5-5, I5-6, and I5-8 each rest on GCC and Clang only. EDG and the older versions changed the answer for W20-3 (Clang 18) and W20-2 (GCC 14). A grid run at the time of the report would give the writer the matrix result with the finding.
3. **The packet counts rule items as well as questions** (item 5).
4. **The completion count names one number.** Decision 0060 counts failed checks (164); the round report counts failed checks and runs (204). One definition, printed by `frontend_check.py`, avoids two figures for the same test.
5. **A save step for reader reports.** Three more reports came back as messages in phase 12.
6. **Smaller per-commit similarity reports.** Phase 12 added 85,387 lines in 17 reports.

General knowledge used in this report: none. Every count comes from Tidal's files, its commit messages, scripts run on commits a15a877d, df4b5539, and ebc109b9, and the writer probes in `writer-probes/`.
