# Roadmap, round 13: what to do after phase 15, from a clean-room writer's view

Date: 2026-11-11. Decision 0029, with decisions 0033, 0039, 0050, 0051, 0054, 0055, 0056, 0057, 0058, 0059, 0060, 0061, 0062, and 0063. A clean-room agent in the role of a clean-room writer, who drafts rules, Evidence, and tests from compiler behavior, prepared this report. It read AGENTS.md, CLAUDE.md, the round 12 writer report `reports/clean-room/roadmap-12/writer.md` with its probes and scripts, `documentation/status.md`, `documentation/roadmap.md` (the phase 15 table), decisions 0060 to 0063, `documentation/decisions/pending.md`, `git log 51bd5060..a6a2b16c` with every commit body, the phase 15 reports (`reports/clean-room/X25/report.md` with its probes, `P4/report.md` with its reductions, `rust-frontend/report-26.md`), the RV8 reviews and reads in `reports/review/` dated 2026-11-11, the question files Q-0435 to Q-0446, `reports/clean-room/ledger.md`, `reports/clean-room/rust-frontend/ledger.md`, `reports/clean-room/rust-frontend/rewrite-marks.tsv`, and the rules that its items name. It did not read the other round 13 reports, `inputs/`, `build/exercise-keys/`, or `frontend/`. Counts come from read-only `git`, `grep`, and the scripts in `writer-scratch/` beside this report (`changed.py`, `counts.py`, `rule.py`, and `waits.py` copied from round 12, `make_grids.py` and `run_probes.py` rewritten, and `doubt_only.py` new), and from read-only runs of `tools/check_specification.py`, `tools/question_index.py --check`, and `tools/question_index.py --share`. Comparisons with the end of phase 14 read commit 51bd5060 through `git show`. Six probe grids ran with `tools/probe_grid.py`, and one run script built and ran programs, on the baseline compilers (`writer-probes/README.md`). Commit read: `a6a2b16c`.

## 1. State from the writer's view

### 1.1 Counts

| Measure | End of phase 14 (51bd5060) | Now (a6a2b16c) |
|---|---|---|
| Rule entries | 1700 (15 withdrawn), 1685 live | 1700 (15 withdrawn), 1685 live; no rule is new |
| Principles | 343 | 343 |
| Rules and principles whose text changed in the phase | 27 rules and 11 principles in phase 14 | 28 rules and 8 principles on 15 pages; no class change |
| Rules and principles changed after the phase's reviews read the tree | 13 rules and 7 principles (after RV7) | 17 rules and 3 principles on 11 pages (after RV8, e7e972d6) |
| Words on the 85 pages, without index and glossary | 276,499 | 280,463 |
| Evidence words on those pages | 72,445 | 74,548 |
| `cpp` example blocks | 230 | 238 |
| Tests (`*.cpp`, excluding `*.tu.cpp`) | 6947, of which 138 are program tests | 7015, of which 152 are program tests |
| Test files with a per-compiler override | 725 (897 override lines) | 743 (919 override lines) |
| Test files with an override for a newest family | 626 | 643: EDG alone 225, GCC alone 213, Clang alone 107, EDG and GCC 69, Clang and EDG 18, Clang and GCC 10, all three 1 |
| Tests with `tidal-error` patterns | 3149 | 3177 |
| Question files | 434, 185 open | 446, 197 open |
| `check_specification.py` | 1700 rules, 0 problems | 1700 rules, 0 problems |
| `question_index.py --check` | 434 questions, 0 problems | 446 questions, 0 problems |
| Front end | 6804 of 6947 pass, 6831 pass their check | 6895 of 7015 pass, 6926 pass their check |
| Completion line (decisions 0060, 0063) | 107 tests fail their check, 3 of 138 program tests do not run, 9 out of scope | 80 tests fail their check (32 marked, 48 toward the gate), 5 of 152 program tests do not run, 9 out of scope |
| Exercise ledger | 1044 rows: 960 commit, 26 open, 33 text, 22 question, 3 out of scope | 1097 rows: 1004 commit, 16 open, 44 text, 30 question, 3 out of scope |
| Front-end ledger | 165 rows: 154 commit, 2 open, 4 text, 1 question, 4 out of scope | 165 rows: 156 commit, 0 open, 4 text, 1 question, 4 out of scope |
| Committed similarity reports | 226 | 241 |

Phase 15 added 68 tests and program tests and no rule. It opened 12 questions (Q-0435 to Q-0446) and settled none.

### 1.2 What phase 15 took from round 12

| Round 12 item | Phase 15 result |
|---|---|
| 1. F5 | Done in 232ac618 and e7e972d6. 20 new tests and 2 changed. I7-6, I7-13, I7-14, and P3-6 became Q-0438, Q-0439, Q-0437, and Q-0441, and the neighbor case of Q-0436 became Q-0440. P3-7 to P3-10 went to the front end. |
| 2. FE26 | Done in a91ccc77. The completion count went from 127 at the start of the round to 77, and the gate count from 97 to 45. P3-7 and P3-8 are `commit`. P3-9 and P3-10 stay `open`, because their tests pass their check and stop in the run. |
| 3. RV8 | Done in 6a73ea89, 29e8ab25, 9f3bceb4, d6fd9cea, 76f9f670, and bdc27d44, with a recorded read of each repair diff. 29 points (RV8-R-1 to R-10, RV8-A-1 to A-9, RV8-B-1 to B-10) and 5 more (RV9-R-1 to R-5). The read of the RV8 repair diff withdrew RV8-R-1 with RV8-A-6 and RV8-R-4, so bdc27d44 removed the two row 4 items that d6fd9cea had added. No question settled. The reviews did not use DEP-P2, DEP-P5, or UNI-P3, so Q-0432, Q-0427, and Q-0424 still wait for a use. |
| 4. GL | Not adopted. Decision 0063 left the guides for a later phase. The P4 reader read only `specification/`, and P4-3 and P4-4 ask for what a guide gives, as P3-1 to P3-3 did. |
| 5. I8 | Not adopted. |
| 6. RC | Partly done. GC (17077248) made the marks file and the completion line counts the marked tests apart. R9 gave the `forward_class` finding a ledger row (RM12-2). The save step for reader reports is not done. The tool refused the writes of the X25, P4, and round 26 reports, and the integrator saved each from the returned text. |
| Process 1, a probe row per unclear finding | Partly followed. X25 kept its probe files and listed its host-only claims, and this round turned the files into a grid at once. P4 kept two reductions and no grid. |
| Process 2, the read of a repair diff is a file | Done. R9 (1f248814) added the check to `check_specification.py`, and RV8 recorded both reads. |
| Process 3, rewrite marks in a file | Done (GC). |
| Process 4, a finding in a commit body gets a ledger row | Done for the three findings of round 12 (RM12-1 to RM12-3). Section 5 of report 26 names a front-end fault with no test and no ledger row. |
| Process 5, a save step for reader reports | Not done. Three more reports came back as messages. |

### 1.3 The 16 open ledger rows, with the probes of this round

Each row that depends on compiler behavior has a probe in `writer-probes/`. Every host-only claim of the X25 and P4 reports holds on the matrix. The grids found four splits that no report claims. EDG alone accepts a read of an element never built (`a2` case 1) and a read of a part with no value in the copy of a union (`a3c`). EDG rejects the transient use of `a7` case 2, which GCC 16 accepts. GCC 14 warns under `-Wall` on the null-pointer form that P4 used for P4-4.

| Row | What the probe shows | What the text needs | Question |
|---|---|---|---|
| P3-9 | None needed. `begin-end-from-base-over-derived.cpp` passes on every baseline compiler. | None. The front end stops in the run (RFOR-2). | none |
| P3-10 | None needed. `assignment-operator-class-parameter.cpp` passes on every baseline compiler. | None. The front end stops at a class assignment through a user `operator=` (OVL-7). | none |
| P4-1 | `p4-points`: every compiler accepts a member function defined in its class with a return type `Box<Money>` or `List<Money, 16>`, or a parameter of type `Box<Money>`, and rejects a return type completed only after the class. | None. 4431852f added the FN-12 sentence and two tests. The row waits for the front end (one false reject, one missed reject, and `money-first-try.cpp`). | none |
| P4-2 | `p4-points`: every compiler accepts a member `operator==` with an `std::initializer_list<int>` parameter, with a named operand and with `std::initializer_list<int>{1, 2}`. A braced list as the right operand of `==` is rejected everywhere, and as the operand of `operator[]` is accepted. | None. The row waits for the front end (`initializer-list-parameter.cpp` and `sequences-first-try.cpp`). | none |
| P4-3 | `p4-points`: every compiler accepts an adaptor over any sequence whose closure type is a template parameter, deduced from the aggregate, in a range-based `for` over an end type that has only `==`. | An informative example under RFOR-2, after the computed sequence of P3-1: a filtering adaptor over any range, citing LAM-P4, CTAD-9, RFOR-2, and RFOR-3. | none |
| P4-4 | `p4-points`: every compiler names the iterator type through a null pointer and through a declared function, and accepts a requires-expression parameter of that type. `p4-4-warnings`: GCC 14 warns "'this' pointer is null" on the null-pointer form under `-Wall`. The declared-function form gives no warning anywhere. | CPT-28 links the declared-function example on the decltype page (P3-3) and gains an example of a requires-expression parameter of the iterator type. An Evidence note records the GCC 14 warning, so the example uses the declared function. | none |
| P4-5 | `p4-points`: every compiler rejects `return last;` for a `Maybe` with deleted copy functions and no move constructor, and accepts it with a move constructor. | The `Maybe` example on the storage page gains its move constructor. The sentence after it cites CPY-5 and CPY-P2 beside CPY-8. | none |
| P4-7 | `p4-points`: every compiler passes the `long double` form the value of `19.99L`, `0.1L`, and `1e300L`, which differs from the `double` value of `19.99`, and gives `19.99_usd` the value 1999. The integer form receives `18446744073709551615` and `0x10` as their values. | OPR-8: the numeric form receives the value that the literal without its suffix has as a literal of type `unsigned long long` or `long double`, and PRIM-21 gives the rounding. One accept test, no override. | none |
| X25-1 | `x25-rules`: every compiler accepts `std::construct_at` of a whole member that is not active during constant evaluation, with another member active and with none. | LSUP-10 adds a third place: a member of a union whose lifetime began in the same evaluation, citing UNI-14. LSUP-10's tests line gains the existing UNI-14 tests of the form. | none |
| X25-2 | None possible. No program shows when an array's own lifetime begins. | OBJ-1 adds a row for an array: its lifetime begins with its storage or when STG-8 creates it, and its elements begin as their own types say (STG-28). | none |
| X25-3 | None needed. | The glossary entry "lifetime" follows OBJ-1: a class object's lifetime ends when its destructor call starts, and a scalar with an initializer begins when the initializer completes. | none |
| X25-4 | `run-probes`: on each of the six compilers that build, a read through the member name gives the new element (7), and a read of the old member gives its value (3), with no sanitizer report. No run separates the readings. In constant evaluation, GCC 16 and EDG accept the construction and Clang rejects it (`x25-constant`, Q-0425). | UNI-14 says that a run cannot tell whether the member name reaches the new element, and that Q-0425 holds the case. No test. | Q-0425 (dated note) |
| X25-5 | `x25-rules`: every compiler accepts a trivial copy and a trivial copy assignment of a union whose active member has a part with no value, for an `int` array member and for a member of class type. Clang and GCC reject a read of that part in the copy, and EDG accepts. | CE-22 and CE-P3: a trivial copy or move of a union copies its active member without reading its parts, and a part with no value has none in the target. One accept test, no override, and one reject test with an EDG override. | New: a read of a part with no value in the copy of a union (EDG alone accepts) |
| X25-6 | None needed. | STG-P2 cites the storage-form table of the storage page's Examples, and UNI-14 and LSUP-P3 link it. | none |
| X25-7 | `x25-rules`: every newest compiler rejects a `constexpr` variable of a union or a struct with an `int` array element ended by a pseudo-destructor call. GCC 14 accepts both. | CE-21: an element that has ended is a part with no value. An example, and two reject tests with a GCC 14 override. | none |
| X25-9 | None needed. | None. No rule rests on the outside knowledge. The row becomes `text`. | none |

Eleven rows need text (P4-3, P4-4, P4-5, P4-7, X25-1 to X25-7). Four rows wait only for the front end (P3-9, P3-10, P4-1, P4-2), and X25-9 needs no change. Three rows need tests, about 5 in all. One row needs a new question (X25-5).

Two points outside the 16 rows also need text, each with tests:

- **RV9-R-4 (Q-0445).** `q0445-template-id`: every compiler accepts an attribute after the class key in an explicit specialization and in a partial specialization that are declared only. TYN-10 rejects an attribute outside `forward_class`, which ends in an identifier, so the text rejects two forms that every compiler accepts. TYN-6 and the `forward_class` production take the template-id form in such a declaration. Two accept tests, no override.
- **Report 26, section 5.** `fe26-named-call`: every compiler accepts `operator""_w<'1', '2'>()` and gives the value of `12_w`. One accept test (OPR-8, OPR-10), so that the front-end fault has a test and a ledger row.

### 1.4 The questions phase 15 opened

| Question | **Waits for:** | What a writer can do now |
|---|---|---|
| Q-0435 (OPR-8, OPR-10, OPR-16, the char pack template) | a use of OPR-P3 | Nothing. A principle-use batch can use OPR-P3. |
| Q-0436 (PRIM-2, a literal whose digits no integer type holds) | a principle (PRIM-P1) | Add a PRIM-P1 sentence: a literal that calls a literal operator has no type from the literal's list, and only the numeric form needs a value that `unsigned long long` holds. |
| Q-0437 (CTAD-17, a conditional `explicit`) | a principle (CTAD-P1) | Add a CTAD-P1 sentence: in copy initialization, each guide's `explicit` condition is evaluated for its own deduced arguments before the choice. |
| Q-0438 (CTAD-18, an empty list) | a review (doubt); newest families that agree | Nothing. GCC and EDG differ. |
| Q-0439 (CTAD-2, a static member with no initializer) | the maintainer (row 4) | Nothing. |
| Q-0440 (PRIM-2, the numeric form's value) | a principle (PRIM-P1) | The same PRIM-P1 sentence as Q-0436. |
| Q-0441 (INST-5, a static member's type) | a review (doubt); newest families that agree | Nothing. |
| Q-0442 (ADD-14, the matching integer type) | the maintainer (row 4) | Nothing. |
| Q-0443 (CTAD-18, a reference to the list) | a principle (CTAD-P3) | Add a CTAD-P3 sentence: CTAD-18 alone defines an initializer-list guide, as ILC-3 does for constructors, including a first parameter that is a reference to the list. |
| Q-0444 (DTOR-8, the base name with no qualifier) | a principle (CTOR-P1) | Add a CTOR-P1 sentence: the name after the tilde in a member access names the type of the object. |
| Q-0445 (TYN-10, an attribute after the class key) | a review (doubt) | The template-id form of section 1.3 answers RV9-R-4 by the matrix. The question then waits for a review of GCC's acceptance alone. |
| Q-0446 (OPR-8, the class-type template) | newest families that agree; a principle (OPR-P3) | Nothing for the families. An OPR-P3 sentence can say that a template that takes the string literal comes first. |

Seven open questions now wait for a principle that a writer can add: Q-0196 (TPL-P3), Q-0429 (DED-P2), Q-0436 and Q-0440 (PRIM-P1), Q-0437 (CTAD-P1), Q-0443 (CTAD-P3), and Q-0444 (CTOR-P1). Q-0446 waits for one too, and also for the families. Four wait for a use of a principle: Q-0424 (UNI-P3, with a doubt), Q-0427 (DEP-P5), Q-0432 (DEP-P2), and Q-0435 (OPR-P3). The last principle-use batch was PU3 in phase 10. Under decision 0055, a question does not settle on a principle written in the same phase, so a phase 16 batch can settle Q-0427, Q-0432, and Q-0435, and the seven others in the phase after.

43 open questions wait only for a review (doubt), 15 for a review (silent), 7 for a review (against), and 1 for a review (part) (`writer-scratch/doubt_only.py`). Decisions 0058, 0059, 0061, 0062, and 0063 each deferred a narrow second review of the doubt-held questions. Under decision 0054, a review that supports one of the 43 with no doubt settles it, since at most one newest family differs on each.

### 1.5 The pending.md row 4 items that phase 15 added

| Item | Families | What it waits for |
|---|---|---|
| STG-28 and STG-16, Q-0430 (element `k` through a byte pointer) | Clang 22 and GCC keep the matrix; EDG runs no program | Row 4a. STG-16 now names the case as an exception that the reference does not have. |
| ADD-14, Q-0442 (the matching integer type) | Clang 22 and GCC keep the matrix; EDG runs no program | Row 4a. It replaced the item "ADD-14, ADD-P6". |
| CTAD-2, Q-0439 (the `inline` static member) | Clang 22 and EDG keep the matrix; GCC follows the review | Row 4b |
| CTAD-17 (a guide reached through a conversion function) | Every family keeps the matrix; Clang 18 follows the review | Row 4a, a rule item |
| CTAD-16 (the base's whole guide set) | GCC and EDG keep the matrix; Clang makes no inherited guides | Row 4a, a rule item |

The items "DTOR-8, Q-0444" and "INST-1, INST-2" came and went in the phase. Row 4a holds 20 questions and 17 rule items. The request to the maintainer of 2026-11-10 has no answer. `question_index.py --share` reports "Rows 1 to 9 hold 119 of the 197 open questions (60%). With row 10, the rows hold 134 (68%)."

Q-0425 is in row 1. Its array form has two newest families against the answer Tidal keeps. GCC 16 and EDG accept the construction at an element of an array member that is not active, and Clang 22 rejects it (`x25-constant`). Row 1's option would settle the question with Clang's answer. The X25 design takes the other answer.

### 1.6 The 20 rules and principles that changed after RV8 read the tree

The close lists DTOR-8, OPR-P3, OPR-8, OPR-16, OPR-10, UNI-P3, FN-12, REF-2, TYN-10, TYN-6, ADD-11, ADD-14, STG-16, STG-28, CTAD-18, DEP-P1, DEP-40, INST-1, INST-2, and INST-5. `changed.py` from e7e972d6 to a6a2b16c gives the same 17 rules and 3 principles. From d6fd9cea to a6a2b16c it gives only FN-12 and INST-1. So the read of the RV8 repair diff (76f9f670), which read every hunk of d6fd9cea, has read 18 of the 20. No source reader has read the P4 sentence of FN-12 or INST-1's citation of it (4431852f), or the hunks of bdc27d44 on the constructors and instantiation pages.

Ranked by the writer's need for a second reading:

1. FN-12 and INST-1. No reader has read the P4 change. FN-12 now sets when a member function defined in its class needs its types complete, which CLS-2 and INST-5 also touch.
2. TYN-6 and TYN-10. RV9-R-4 is a doubt, and the matrix now shows two forms that TYN-10 rejects and every compiler accepts (section 1.3). F6 changes both.
3. OPR-8, OPR-10, OPR-16, OPR-P3. The read agrees, with the RV9-R-5 doubt. F6 changes OPR-8 for P4-7.
4. STG-16, STG-28, ADD-14, ADD-11. The read reads STG-16, STG-28, and ADD-14 against, and row 4a holds each. A second reading adds nothing until the maintainer decides row 4a.
5. CTAD-18, DEP-P1, DEP-40, INST-2, INST-5, DTOR-8, REF-2, UNI-P3. The read agrees.

F6 changes LSUP-10, OBJ-1, UNI-14, CE-21, CE-22, CE-P3, STG-P2, OPR-8, TYN-6, TYN-10, CPT-28, the glossary entry "lifetime", and the examples on the range-for and storage pages. PQ2 adds sentences to PRIM-P1, CTAD-P1, CTAD-P3, CTOR-P1, OPR-P3, DED-P2, and TPL-P3. RV9 reads those as well.

### 1.7 Distance to decision 0060's completion test

The completion line reads "80 tests fail their check, of which 32 need the template model (marks file) and 48 count toward the gate of decisions 0062 and 0063; 5/152 program tests do not run to their result; 9 tests out of scope". The gate held at the end of front-end round 26 with 45, and the phase 16 decision adds the rewrite as an item. P4 added three tests that count (two FN-12 tests and `initializer-list-parameter.cpp`), so the count stands at 48, two under the limit of 50.

The five program tests that do not run are `r10-tuple-second-try.cpp` (CTAD-29, unsupported), the P3 first tries `pipeline-first-try.cpp` and `events-first-try.cpp` (the run stops of P3-9 and P3-10), and the P4 first tries `money-first-try.cpp` and `sequences-first-try.cpp` (P4-1 and P4-2). The four open front-end rows account for four of the five program tests and three of the 48 tests.

Writer work touches the count in two ways:

- F6 adds about 8 tests. The union tests of X25-5 and X25-7 read parts with no value in constant evaluation. Round 26 made an attribute after the class key an error on the front end when the specifier does not declare the class alone, and the TYN-6 tests hold such an attribute in a specialization. Some of the 8 can fail on the front end, and the margin is two.
- No test is a new candidate for `out of scope`.

## 2. Proposed items, ranked

| Rank | Item | Evidence | Cost (writer sessions) | Similarity risk | Unblocks |
|---|---|---|---|---|---|
| 1 | F6. Answer the 16 open rows, RV9-R-4's template-id form, and the report 26 fault | section 1.3; six grids and one run script | 1.25 | low | FE27 targets; text that RV9 reads |
| 2 | DQ. A narrow second review of the 43 doubt-held questions | section 1.4; five deferrals | 0.75 of records and repairs | medium | settlement of up to 43 questions with no maintainer row |
| 3 | RW. The rewrite's first stages, with front-end round 27 on the current design | decisions 0060, 0062, 0063; section 1.7 | 0.25 for ledger rows and grammar findings | low | the completion test |
| 4 | PQ2. Principles for the 7 questions that wait for one, and principle-use batch PU4 | section 1.4 | 0.5 | medium | settlement of Q-0427, Q-0432, and Q-0435 in the phase, and 7 more in the next |
| 5 | RV9. Gap review of FN-12, INST-1, the bdc27d44 hunks, F6's text, and PQ2's sentences, with the read of its repair diff | section 1.6 | 0.5 of repairs | medium | a reviewed tree for the rewrite's rule comments |
| 6 | RC2. Records: a save step for reader reports, a check before each commit that touches `questions/` or `reports/review/`, and a ledger row for a front-end fault with no test | sections 1.2 and 4 | 0.2 | low | fewer repair commits |

Items 1, 2, 4, 5, and 6 cost about 3.2 writer sessions. With RW, about 3.45.

### Item 1. F6. Answer the open rows with the probes of this round

- **What.** Section 1.3, row by row. In order:
  - Constant evaluation and unions (X25-1, X25-4, X25-5, X25-7): sentences in LSUP-10, UNI-14, CE-21, CE-22, and CE-P3; a CE-21 example; about 4 tests; one new question (X25-5); a dated note on Q-0425.
  - Lifetime (X25-2, X25-3, X25-6): an OBJ-1 row for an array, the glossary entry "lifetime", and links to the storage-form table from STG-P2, UNI-14, and LSUP-P3.
  - Port findings (P4-3, P4-4, P4-5, P4-7): the adaptor example under RFOR-2, the CPT-28 link and example with its Evidence note, the `Maybe` move constructor with the CPY-5 and CPY-P2 citations, and the OPR-8 sentence with 1 test.
  - Elaborated types (RV9-R-4, Q-0445): TYN-6, TYN-10, and `forward_class` take the template-id form in a declared-only specialization; 2 tests; a dated note on Q-0445.
  - Report 26, section 5: 1 test of a literal operator template called by name, and a front-end ledger row.
  - X25-9 becomes `text`. P3-9, P3-10, P4-1, and P4-2 name item RW's front-end round.
- **Needs first.** Nothing. The probes decide every row that depends on behavior.
- **Evidence.** Section 1.3 and `writer-probes/README.md`.
- **Cost.** 1.25 sessions.
- **Risk.** Low. Each test records the families that differ as overrides. X25-2 and X25-3 change wording on lifetime, where a writer can recall the reference's phrasing, so the similarity run of F6 reads OBJ-1 and the glossary first.
- **Unblocks.** FE27's targets. The X25 and P4 rows close.

### Item 2. DQ. A narrow second review of the doubt-held questions

- **What.** A source reader reads the 43 questions whose **Waits for:** line names only a review (doubt), against the current text and its tests, and says for each whether the doubt stands. The list comes from `writer-scratch/doubt_only.py`. By the first rule of each Affects line, ICS (4), NEW (4), CPT (3), and STG (3) hold the most. The writer records each reading, settles each question with no doubt left under decision 0054, and repairs the text where the reader reads against. The review can also read the 15 silent questions for row 10.
- **Needs first.** Nothing. F6 changes Q-0445 and no other question of the list.
- **Evidence.** Section 1.4. The question reviews of 2026-11-03 settled 76 questions in two commits (e0d06f0 and 8c15b06). Phase 15 settled none, and 119 of the 197 open questions wait on the maintainer's rows.
- **Cost.** One source-reader session in two or three parts. 0.75 writer session for the records, the settlements, and the repairs.
- **Risk.** Medium. The round 12 editor counted 20 of the doubt-held questions as last read on 2026-11-03, and their rules have changed since.
- **Unblocks.** The only large drop in the open count that does not wait on the maintainer.

### Item 3. RW. The rewrite's first stages, and front-end round 27

- **What.** Decision 0062 has the phase 16 decision add the rewrite as an item, and status step 2 says it starts from the rewrite design. The rewrite takes the 32 marked tests over. The current front end takes P3-9, P3-10, P4-1, and P4-2, F6's tests, and the 48 tests with named causes, in a front-end round 27. The writer answers the grammar findings that a parser built from the Syntax segments gives (decision 0060), as RM12-2 was.
- **Needs first.** F6 for its tests.
- **Cost.** 0.25 writer session for the ledger rows and grammar answers.
- **Risk.** Low for the writer. Each stage keeps `frontend_check.py` at 0 worse (decision 0060).
- **Unblocks.** The completion test. The four open front-end rows close four of the five program tests that do not run.

### Item 4. PQ2 and PU4. Principles, then their use

- **What.** PQ2 adds one sentence to each of PRIM-P1, CTAD-P1, CTAD-P3, CTOR-P1, OPR-P3, DED-P2, and TPL-P3, as section 1.4 gives, and moves each question's **Waits for:** line to a use of the principle. PU4 is a principle-use batch (`tools/exercises.py principles`) on the questions that wait for a use now: Q-0424, Q-0427, Q-0432, and Q-0435.
- **Needs first.** Nothing for PU4. PQ2 can run beside F6, and its OPR-P3 sentence comes after PU4, so that the batch uses the OPR-P3 that phase 15 wrote.
- **Evidence.** Section 1.4. PU1 to PU3 settled 29 questions (926831c, f2ac11c, 75e4fab).
- **Cost.** 0.5 session for the sentences and the batch's records.
- **Risk.** Medium. A principle sentence states a model, and the model can echo the reference's organization. Each sentence cites the rule that already gives the case.
- **Unblocks.** Q-0427, Q-0432, and Q-0435 in phase 16 if the reader reaches their answers. Q-0424 also needs its doubt resolved. The seven PQ2 questions in phase 17 (decision 0055).

### Item 5. RV9. Gap review of the changed text

- **What.** A source-reader review of FN-12 and INST-1, the bdc27d44 hunks, F6's text, and PQ2's sentences, then the repairs and a recorded read of their diff. The prompt names the doubts left as `text` that F6 or PQ2 touch (RV9-R-4, RV9-R-5, RV8-B-9).
- **Needs first.** F6 and PQ2.
- **Cost.** 0.5 writer session of repairs.
- **Risk.** Medium. The union and constant-evaluation sentences of F6 sit near Q-0117, Q-0425, and Q-0115.
- **Unblocks.** A tree whose rule text a reviewer has read, for the rule IDs that the rewrite's code cites.

### Item 6. RC2. Records

- **What.** Three records. The reader prompts name a path in the scratchpad for the report, and the integrator copies the file, so that a refused write does not route the report through a message. The integrator runs `question_index.py --check` and `check_specification.py` with `&&` before each commit that adds a file under `reports/review/` or `questions/`. A front-end round report that names a fault with no test gives it a ledger row.
- **Needs first.** Nothing.
- **Cost.** 0.2 session.
- **Risk.** Low.

### What the rules need before front-end round 27 and the next exercises

- **Before FE27.** None of the four open front-end rows needs text. F6's tests come first, so that the round takes them.
- **Before the rewrite's parser.** The TYN-6 and `forward_class` change of F6, so that the grammar takes the template-id form that every compiler accepts.
- **Before a fifth Port.** The P4-3 and P4-4 examples, or the guides that decision 0063 deferred.
- **Before the next Propose.** The X25-1 and X25-5 sentences, so that a designer of a union feature finds LSUP-10 and CE-22 in agreement with UNI-14.
- **Propose and Diagnose.** Unchanged. F6 changes text by sentences, rows, and examples.

## 3. Out of scope

1. **No new topic.** Phase 15 added no rule and no page.
2. **Coroutines, modules, threads and the memory model, extended floating-point types, the preprocessor beyond `#include "file"`, and the general library stay out**, as the charter and decision 0051 say. P4-3 asks for lazy stages, which the text answers with iterator classes (RFOR-P1).
3. **The maintainer's rows.** 119 of 197 open questions wait on rows 1 to 9 (60%), 134 with row 10 (68%). The request of 2026-11-10 has no answer. The writer does not choose for the maintainer, and that includes Q-0425's array form.
4. **P27 itself.** X25 is a Propose exercise. F6 answers its findings on the current text and adopts none of its new rules.
5. **The rewrite's code.** The writer's items add no test that needs the template model.
6. **MSVC and later versions** stay deferred (row 7). X25-9 names later versions, and no rule rests on them.

## 4. Process changes

Ranked by writer time saved.

1. **A reader keeps its probes as files with defines for each case.** X25's probe files took one script to turn into a grid, and every claim was checked in one run. P4's claims needed the writer to rebuild each case from the prose. A probe file per claim, with a `CASE` define for each reading, is enough for `make_grids.py`.
2. **A read of a repair diff comes before row 4 items.** d6fd9cea added two row 4 items from RV8-R-1 and RV8-R-4, and bdc27d44 removed them after the read of the diff withdrew both points. When a point reads against an answer that two newest families give, the writer can record the point as a Note and wait for the read of the repair diff before it adds a row 4 item.
3. **A check runs before each commit that adds a review.** 76f9f670 committed the read of the RV8 repair diff without its question records, and `question_index.py --check` failed until bdc27d44. The shell habit of `&&` covers the case if the commit command chains the check (item RC2).
4. **A front-end fault with no test gets a test and a ledger row.** Report 26 named a fault in section 5, and neither ledger holds it. This round's grid gives the test.
5. **A save step for reader reports** (item RC2). Six reports in two phases came back as messages.

General knowledge used in this report: none. Every count comes from Tidal's files, its commit messages, scripts run on commits 51bd5060, e7e972d6, d6fd9cea, and a6a2b16c, and the writer probes in `writer-probes/`.
