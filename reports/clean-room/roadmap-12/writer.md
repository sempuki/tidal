# Roadmap, round 12: what to do after phase 14, from a clean-room writer's view

Date: 2026-11-11. Decision 0029, with decisions 0033, 0039, 0050, 0051, 0054, 0055, 0056, 0057, 0058, 0059, 0060, 0061, and 0062. A clean-room agent in the role of a clean-room writer, who drafts rules, Evidence, and tests from compiler behavior, prepared this report. It read AGENTS.md, CLAUDE.md, the round 11 writer report `reports/clean-room/roadmap-11/writer.md` with its probes and scripts, `documentation/status.md`, `documentation/roadmap.md` (the phase 14 table), decisions 0060 to 0062, `documentation/decisions/pending.md`, `git log 45ee5e85..bd87b27c` with every commit body, the phase 14 reports (`reports/clean-room/I7/report.md`, `P3/report.md`, `rust-frontend/report-25.md`), the two phase 14 gap reviews in `reports/review/` (`2026-11-11-gap-review-phase-14-*`), the question files Q-0429 to Q-0434, `reports/clean-room/ledger.md`, `reports/clean-room/rust-frontend/ledger.md`, and the rules that its items name. It did not read the other round 12 reports, `inputs/`, `build/exercise-keys/`, or `frontend/`. Counts come from read-only `git`, `grep`, and the scripts in `writer-scratch/` beside this report (copied from round 11, with `make_grids.py` rewritten and `run_probes.py` added), and from read-only runs of `tools/check_specification.py`, `tools/question_index.py --check`, and `tools/question_index.py --share`. Comparisons with the end of phase 13 read commit 45ee5e85 through `git show`. Five probe grids ran with `tools/probe_grid.py`, and one run script built and ran programs, on the baseline compilers (`writer-probes/README.md`). Commit read: `bd87b27c`.

## 1. State from the writer's view

### 1.1 Counts

| Measure | End of phase 13 (45ee5e85) | Now (bd87b27c) |
|---|---|---|
| Rule entries | 1698 (15 withdrawn), 1683 live | 1700 (15 withdrawn), 1685 live; DEP-40 and LOOP-9 are new |
| Principles | 343 | 343 |
| Rules and principles whose text changed in the phase | 34 rules and 8 principles in phase 13 | 27 rules and 11 principles on 15 pages, and the two new rules; no class change |
| Rules and principles changed after RV7 read the tree (6afa79f9) | none | 13 rules and 7 principles on 10 pages |
| Words on the 85 pages, without index and glossary | 273,009 | 276,499 |
| Evidence words on those pages | 70,879 | 72,445 |
| `cpp` example blocks | 224 | 230 |
| Tests (`*.cpp`, excluding `*.tu.cpp`) | 6881, of which 126 are program tests | 6947, of which 138 are program tests |
| Test files with a per-compiler override | 716 (888 override lines) | 725 (897 override lines) |
| Test files with an override for a newest family | 619 | 626: EDG alone 220, GCC alone 205, Clang alone 107, EDG and GCC 65, Clang and EDG 18, Clang and GCC 10, all three 1 |
| Tests with `tidal-error` patterns | 3132 | 3149 |
| Question files | 428, 180 open | 434, 185 open |
| `check_specification.py` | 1698 rules, 0 problems | 1700 rules, 0 problems |
| `question_index.py --check` | 428 questions, 0 problems | 434 questions, 0 problems |
| Front end | 6721 of 6881 pass, 6745 pass their check | 6804 of 6947 pass, 6831 pass their check |
| Completion line (decision 0060) | 127 tests fail their check, 2 of 126 program tests do not run, 9 out of scope | 107 tests fail their check, 3 of 138 program tests do not run, 9 out of scope |
| Exercise ledger | 988 rows: 920 commit, 26 open, 27 text, 12 question, 3 out of scope | 1044 rows: 960 commit, 26 open, 33 text, 22 question, 3 out of scope |
| Front-end ledger | 163 rows: 152 commit, 2 open, 4 text, 1 question, 4 out of scope | 165 rows: 154 commit, 2 open (G164, G165), 4 text, 1 question, 4 out of scope |
| Committed similarity reports | 218 | 224; the phase added 6 reports and 27,836 lines |

Phase 14 added 66 tests and program tests and two rules. It opened 6 questions (Q-0429 to Q-0434) and settled 1 (Q-0422).

### 1.2 What phase 14 took from round 11

| Round 11 item | Phase 14 result |
|---|---|
| 1. F4 | Done in 1130ceb8 and 6afa79f9. 19 new tests and 4 changed. I6-5, I6-6, I6-15, and P2-6 became Q-0434, Q-0432, Q-0433, and Q-0431. |
| 2. FE25 | Done in b035efd4 and 18bb9b3a. The count went from 144 at the start of the round to 104. G164 and G165 are `open`. |
| 3. RV7 | Done in 4dc57e81, b5d925e0, and 87b127cb. 27 points, each with a ledger row: 18 commit, 6 question, 3 text. Q-0422 settled. No file under `reports/review/` records a source reader's read of the RV7 repair diff (b5d925e0), which decision 0062 asks for. |
| 4. GL | Not adopted. Decision 0062 left the guides for a later phase. The P3 reader read only `specification/`, and P3-1, P3-2, and P3-3 ask for what a guide gives. |
| 5. I7 | Done in 411ef68e. 17 findings: 15 open, I7-4 `text`, I7-16 and I7-17 `commit`. |
| 6. HC | Done in R8 (f4109f3a). The I7 and P3 reports each list their host-only claims. This round's probes confirm every listed claim (section 1.3). |
| Process 2, a test of an event separates that event | Followed in F4. `class-point-static-constexpr-made-at-use.cpp` reads the member only after the later declaration. |
| Process 3, a save step for reader reports | Not done. The tool refused the writes of the I7, P3, and round 25 reports, and the integrator saved each from the returned text. |
| Process 4, rewrite marks | Partly done. Report 25 marks 30 of 104 tests "rewrite" in its section 7 table. `conformance.tsv` and the completion line do not carry the mark. |
| Process 5, a review uses the principle a question waits on | Partly done. RV7 used JMP-P3 and settled Q-0422. It did not use DEP-P5 or DEP-P2, which changed in the phase, so Q-0427 and Q-0432 now wait for a use. |

### 1.3 The 26 open ledger rows and G164, G165, with the probes of this round

Each row that depends on compiler behavior has a probe in `writer-probes/`. Every host-only claim in the I7 and P3 reports holds on the matrix. The grids found four splits that no report claims: I7-6, I7-13, I7-14, and the static members of P3-6.

| Row | What the probe shows | What the text needs | Question |
|---|---|---|---|
| I7-1 | `i7-ctad`: every compiler elides the braces of a member `Plain p` and of `int v[2]`, neither dependent. EDG rejects `Named b{"ab", 1}` (Q-0277). | CTAD-10 gains a bullet: a member or base whose type is not dependent follows LIST-4. `aggregate-elision.cpp` and `aggregate-string-fixed-array.cpp` hold both cases. One accept test for the `int v[2]` member, no override. | none |
| I7-2 | None needed. `result-then-initialize.cpp` holds the case. | CTAD-25 says that step 5 chooses the constructor of the deduced type again, and that constructor can differ from the one that gave the guide. An example under CTAD-26 with the tie. | none |
| I7-3 | `i7-ctad`: every compiler reports a class template that inherits constructors as not an aggregate (DER-8). | A note under CTAD-16: the inherited copy candidate can give a type whose initialization fails (DER-8, CTAD-26). The note of `inherited-copy-candidate-user-guide.cpp` drops the aggregate candidate, and the test cites CTAD-26. | none |
| I7-5 | No probe can separate the readings. A guide from an implicit copy or move constructor gives the same type as the copy candidate. | CTAD-5: only the constructors that the class declares give guides. | none |
| I7-6 | `i7-ctad`: a user guide `(std::initializer_list<int>, int = 0)` wins a braced list of two on every compiler. For `C c{}` with a guide with no parameters and an initializer-list guide, Clang 18 and 22 take the guide with no parameters, and GCC 14 and 16 and EDG take the initializer-list guide. | CTAD-18 defines an initializer-list guide: its first parameter is `std::initializer_list<E>`, and each later parameter has a default. An accept test for the list of two, no override. The empty list needs a question; the provisional answer follows ILC-3 for constructors, which CTAD-17 cites (Clang). One accept test with GCC and EDG overrides. | New: an empty list and the initializer-list guides (Clang against GCC and EDG) |
| I7-7 | `i7-ctad`: `IntPair p(1, 2.0)` gives `Pair<int, double>` on every compiler but Clang 18 (Q-0240). EDG rejects a conversion to the fixed parameter. `alias-template-fixed-argument.cpp` holds both overrides. | An example under CTAD-14 with the steps written out. The text keeps both readings of when a guide whose result cannot match is dropped, since no program can tell them apart. | none |
| I7-8 | `i7-ctad`: every compiler applies the constraint of an inner alias through an outer alias, and Clang 18 rejects the satisfied case too (Q-0240). | CTAD-14: the steps run once per alias, inner first, and each alias's constraints apply (CTAD-28). One reject test, no override. | none |
| I7-9 | `i7-ctad`: GCC and EDG give a derived class the guides from the base's constructors, and no compiler gives it the base's guide with no parameters or the base's aggregate candidate. Clang gives no inherited guide (Q-0239). | CTAD-16 lists what a derived class takes: the guides from the base's constructors, its user guides, and its copy candidate. The guide with no parameters (CTAD-6) and the aggregate candidate (CTAD-9) stay with the base. Two reject tests, no override. | none |
| I7-10 | None needed. | CTAD-P1: a user guide with no template head is a function, and OVL-27 applies to it. CTAD-P3: CTAD-22 follows OVL-27 for two guides from constructors. | none |
| I7-11 | `i7-ctad`: in copy initialization, a guide whose parameter needs a user-defined conversion is viable on Clang 22, GCC, and EDG. Clang 18 rejects. | CTAD-17: the limit of OVL-14 and ICS-10 applies to the constructors of step 5 and does not apply to the guides. One accept test with a Clang 18 override. | none |
| I7-12 | None needed. | CTAD-18 adds "unlike ILC-3 for constructors" and links Q-0081. | none |
| I7-13 | `i7-ctad`: every compiler accepts `new Box` with no initializer. A static data member of the deduced type with no initializer, `inline` or not: GCC accepts, Clang and EDG reject. | CTAD-1 lists a `new` expression with or without an initializer. CTAD-2 lists a static data member with no initializer. One accept test, no override; one reject test with a GCC override. | New: a static data member with no initializer (GCC alone accepts) |
| I7-14 | `i7-ctad`: in copy initialization from `'c'`, Clang and EDG leave out the guide that is explicit for `char` and choose the user guide `N(int) -> N<int>`. GCC 14 and 16 choose the explicit guide and then reject the initialization. Copy-list initialization is rejected everywhere (CTAD-23). | CTAD-17: each guide's condition is evaluated for its own deduced arguments before the choice. One accept test with a GCC override. | New: a conditional `explicit` and the choice (GCC alone differs) |
| I7-15 | None needed. | CTAD-9 links ICS-18, which gives the sequence of a braced list to an aggregate. | none |
| P3-1 | None needed. | An informative example under RFOR-2: an iterator over a sequence computed on demand, with a separate end type. A sentence under RFOR-P1 that a lazy sequence is a class with its own state, because coroutines are out of scope (KW-1). | none |
| P3-2 | None needed. | An informative example under LAM-P4: an abstract callable class template with a virtual call operator and a virtual destructor, and a derived template that owns the closure (VIRT-P3, NEW-P2, CPY-P3). | none |
| P3-3 | `p3-copy-init`: every compiler accepts a function template that is declared only and named in `decltype`. `pipeline.cpp` links and runs on the matrix. | An example on the decltype page that names a call's result type through a declared function, citing ODR-3 and INST-7. | none |
| P3-4 | None needed. Round 11's `p2-points` grid holds the split on equal literals (Q-0033). | A note under the equality rules of the comparison page: `==` on two `const char*` compares addresses, and PRIM-9 gives the case of two literals. | none |
| P3-5 | `run-probes`: every compiler that builds reports `stack-use-after-scope` under `-fsanitize=address`. | An informative example under OBJ-9: a class that keeps a reference to a temporary argument, which dangles after the declaration. | none |
| P3-6 | `p3-points`: every compiler accepts an in-class body, a declaration with the body outside, a `virtual` body, a parameter of type `Cursor<D>`, and a deduced return type over an incomplete derived class, and rejects a return type written `typename D::value_type`. A `static` member of type `Cursor<D>` declared only: GCC rejects. A `static inline` member of that type: Clang accepts, GCC and EDG reject. | INST-5: the instantiation of a class makes each member function declaration, and the parameter and return types need to be complete only when the definition is made (INST-7). INST-1's bullet on a function definition names the time the definition is made. Two accept tests, no override. The static members need a question. | New: when a static member whose type names an incomplete class argument needs that type complete (GCC on the declaration; GCC and EDG on the `inline` member) |
| P3-7 to P3-10 | The four tests of 8b10f65f pass on every baseline compiler. | None. Each row moves to the front-end round. | none |
| P3-11 | `p3-copy-init`: every compiler accepts `Action<int> a = closure;` with deleted copy operations and a converting constructor template, and rejects it with an `explicit` constructor. | A sentence under CTOR-P5 or VC-4: copy initialization from another type through a converting constructor initializes the object directly, with no copy. One accept test, no override. | none |
| P3-12 | None needed. | Glossary entries for "closure type" (LAM-2) and "range" (RFOR-P1). | none |
| G164 | No probe. EDG runs no program, and no sanitizer stops the step (Q-0430). | STG-28 says that a `T*` cast from a pointer to the byte at the start of element `k` of that array points to element `k`, and cites Q-0430. Part A of RV7 reads against the allocated case and part B allows it, so the clause sits under `pending.md` row 4. | Q-0430 (row 4) |
| G165 | `g165-wall`, `g165-woverloaded`: under `-Wall`, neither GCC nor Clang warns when one base overload is overridden and another is hidden. Both warn when no derived function of the name overrides. Under `-Woverloaded-virtual` alone, GCC also warns in the first case and Clang does not. | VIRT-13: the warning concerns a derived name none of whose functions overrides a base function. An Evidence note gives GCC's wider form under its explicit flag. One test with `-Wall` that accepts the first case, no override. | none |

Twelve rows need only a sentence, an example, a link, or a glossary entry (I7-2, I7-5, I7-7, I7-10, I7-12, I7-15, P3-1 to P3-5, P3-12). I7-3 changes a test's note, and G164 needs one clause. Four rows go to the front end (P3-7 to P3-10). Ten rows need tests, about 16 in all. Four new questions come from I7-6, I7-13, I7-14, and P3-6. One row changes a rule against the current text. CTAD-1 limits a `new` expression to one with an initializer, and every compiler accepts one with none.

### 1.4 The questions phase 14 opened

| Question | **Waits for:** | What a writer can do now |
|---|---|---|
| Q-0429 (DED-20, a member template's pack over the class's pack) | a principle (DED-P2) | Add a DED-P2 sentence: the instantiation of the class expands a pack whose pattern names only the class's pack, and deduction pairs only the parameters that hold the template's own parameters. The question then waits for a use of DED-P2. |
| Q-0430 (ADD-14, STG-28, STG-29) | the maintainer (row 4a) | Nothing. Both reviews read against, and every compiler that runs the programs gives the matrix's answer. G164 adds one clause under the same row. |
| Q-0431 (LSUP-1, the shape) | a review (silent), row 10 | Nothing. |
| Q-0432 (DEP-4, `sizeof` of a value-dependent operand) | a use of DEP-P2 | DEP-P2 has not changed since RV7 read the tree. A review or principle-use batch that uses it can settle the question, since EDG alone differs. |
| Q-0433 (INST-5, a `static const` member) | a review (doubt) | Nothing a probe adds. The doubt is a sentence of the reference on delayed initialization. |
| Q-0434 (DEP-2, `this->n`) | a review (doubt) | RV8 reads the three doubts of RV7-B-11. |

Q-0424 waits for a use of UNI-P3 and a review (doubt), and Q-0427 for a use of DEP-P5. Both principles changed in the RV7 repairs, so RV8 can use them.

### 1.5 The pending.md row 4 items that phase 14 added

| Item | Families | What it waits for |
|---|---|---|
| Q-0430, the typed step over bytes and placement `new` there | Clang 22 and GCC keep the matrix; EDG runs no program | Row 4a |
| ADD-14 with ADD-P6 (Q-0049's move to one past an array of one) | Every family accepts | Row 4a, a rule item. The packet now lists 16 rule items. |

The request to the maintainer of 2026-11-10 (rows 1, 4a, and 10 first) has no answer. Row 1 lists 55 questions, row 4a 19 questions and 16 rule items, and row 10 15 questions. `question_index.py --share` reports "Rows 1 to 9 hold 114 of the 185 open questions (62%). With row 10, the rows hold 129 (70%)."

### 1.6 The 20 rules and principles that changed after RV7 read the tree

The close lists DTOR-8, UNI-P3, UNI-14, REF-2, LSUP-6, STG-P2, LOOP-9, CNS-P3, CNS-13, DED-20, DEP-P1, DEP-P5, DEP-P6, DEP-4, DEP-6, DEP-40, DEP-30, INST-P1, INST-1, and INST-2. `changed.py` from 6afa79f9 to bd87b27c gives the same 13 rules and 7 principles, all from b5d925e0. Ranked by the writer's need for a second reading:

1. DEP-6, DEP-30, DEP-P6, CNS-13, CNS-P3. RV7-B-2 and RV7-B-5 moved the line between a required reject and a case with no diagnostic. DEP-17 has to agree with all five.
2. DEP-40, DEP-P1, DEP-P5, DEP-4. DEP-40 gained a limit to an independent qualifier. DEP-P5 carries Q-0427 and the Q-0225 row. DEP-4 names `typeid`, which no test checks.
3. INST-1, INST-2, INST-P1, DED-20. The `decltype` exemption now covers only the whole operand. DED-20 carries Q-0429. F5 changes INST-1 and INST-5 again for P3-6.
4. UNI-P3, UNI-14, REF-2, DTOR-8. UNI-P3 carries Q-0424. REF-2 lists three direct cases.
5. LOOP-9, LSUP-6, STG-P2. Each change is one clause.

F5 changes CTAD-1, CTAD-2, CTAD-5, CTAD-9, CTAD-10, CTAD-14, CTAD-16, CTAD-17, CTAD-18, CTAD-25, CTAD-P1, CTAD-P3, INST-1, INST-5, STG-28, VIRT-13, a sentence under VC-4 or CTOR-P5, and DED-P2, with examples on five pages and two glossary entries. RV8 reads those as well.

### 1.7 Distance to decision 0060's completion test

The completion line reads "107 tests fail their check, 3/138 program tests do not run to their result, 9 tests out of scope". Report 25 gives a cause for each of its 104 tests. 30 of them are marked "rewrite": 17 missed rejects, 7 unsupported, 3 false rejects, and 3 EDG-AGREES. The current design can take the other 74: 43 unsupported, 28 missed rejects, 2 EDG-AGREES, and 1 false reject. P3 added the other 3 failing tests and 2 of the program tests that do not run (P3-7 to P3-10). The pages with the most failures are `templates/dependent-names` (12), `templates/instantiation` (10), `templates/class-template-deduction` (8), and `templates/constraints` (7).

The gate of decision 0062 asks for at most 50. Its other two parts hold. The 30 rewrite tests alone use 30 of the 50, so the other 77 (report 25's 74 and P3's 3, which no report has marked) must come down to 20. Round 25 removed 40 in one round.

Writer work touches the count in two ways:

- F5 adds about 16 tests. The CTAD-2 reject for a static data member with no initializer joins `site-member.cpp` and `site-return-type.cpp`, which the front end misses now. The I7-6 and I7-14 tests choose among guides, and the front end has that step.
- No test is a new candidate for `out of scope`.

## 2. Proposed items, ranked

| Rank | Item | Evidence | Cost (writer sessions) | Similarity risk | Unblocks |
|---|---|---|---|---|---|
| 1 | F5. Answer the 26 open rows and G164, G165 | section 1.3; five grids and one run script | 1.5 | low | FE26 targets; text that RV8 reads |
| 2 | FE26. Front-end round 26 | P3-7 to P3-10, F5's tests, the 74 failures that report 25 gives to the current design | 0.25 for the ledger | low | the gate of decision 0062 |
| 3 | RV8. Gap review of the 20 changed rules and principles, F5's text, and the RV7 repair diff, with principle uses | section 1.6; Q-0424, Q-0427, Q-0432 | 0.5 of repairs | medium | settlement of up to three questions |
| 4 | GL. Guides for a programmer from another language, and reader access to `guides/` | P3-1, P3-2, P3-3; round 11 item 4 | 0.5 | low | a Port reader who finds the iterator and callable forms |
| 5 | I8. Implement on constraints | 7 failing tests; no blind build has read the page | 0.25 for triage | low | text findings on a page with 40 rules and 22 questions |
| 6 | RC. Records: rewrite marks in `conformance.tsv`, a ledger row for the grammar finding of RP, and a save step for reader reports | sections 1.2 and 4 | 0.2 | low | a gate count that reads its own file |

Items 1, 3, 4, and 6 cost about 2.7 writer sessions. With FE26 and I8 triage, about 3.2.

### Item 1. F5. Answer the open rows with the probes of this round

- **What.** Section 1.3, row by row. In order:
  - The CTAD page (I7-1 to I7-3, I7-5 to I7-15): sentences in CTAD-1, CTAD-2, CTAD-5, CTAD-10, CTAD-14, CTAD-16, CTAD-17, CTAD-18, and CTAD-25; CTAD-P1 and CTAD-P3; examples under CTAD-14 and CTAD-26; the CTAD-9 link to ICS-18; the note of `inherited-copy-candidate-user-guide.cpp`; about 10 tests; three new questions (I7-6, I7-13, I7-14).
  - The instantiation page (P3-6): INST-1 and INST-5 sentences; 2 tests; the new question on static members.
  - G164 and G165: an STG-28 clause and a VIRT-13 sentence with its Evidence note; 1 test.
  - P3-11: the VC-4 or CTOR-P5 sentence and 1 test.
  - Examples and glossary (P3-1 to P3-5, P3-12): RFOR-2, LAM-P4, the decltype page, the comparison page, OBJ-9, and two glossary entries.
  - Q-0429: the DED-P2 sentence.
- **Needs first.** Nothing. The probes decide every row that depends on behavior.
- **Evidence.** Section 1.3 and `writer-probes/README.md`.
- **Cost.** 1.5 sessions.
- **Risk.** Low. Each test records the families that differ as overrides. The I7-13 and I7-14 answers have one newest family against them. The I7-6 answer and the `inline` case of P3-6 follow Clang 22 alone, where the text already gives that reading (ILC-3, INST-5). Two newest families stand against each, so under decision 0054 they wait for the maintainer's row 1 even after a review.
- **Unblocks.** FE26's targets; the I7 and P3 rows close.

### Item 2. FE26. Front-end round 26

- **What.** The P3-7 to P3-10 tests (a member definition of a base over the derived class made early, and a class assignment through a user `operator=`); F5's tests; the 74 failures of report 25 that the current design can take, with the CTAD sites of CTAD-1 and CTAD-2 first; G164 and G165 after F5's text.
- **Needs first.** F5 for its tests.
- **Cost.** 0.25 writer session for the ledger rows that follow.
- **Risk.** Low.
- **Unblocks.** The gate. If the round ends with at most 20 failing tests outside the rewrite marks, the gate holds.

### Item 3. RV8. Gap review of the changed text

- **What.** A source-reader review of the 20 rules and principles of section 1.6 and of F5's text, with a recorded read of the RV7 repair diff (b5d925e0), which no file in `reports/review/` holds. The prompt names DEP-P2 for Q-0432, DEP-P5 for Q-0427, and UNI-P3 for Q-0424, and asks the reviewer to use each. A first reading of the four new questions. Then the repairs and a recorded read of RV8's repair diff.
- **Needs first.** F5.
- **Cost.** 0.5 writer session of repairs.
- **Risk.** Medium. The dependent-names page had 9 of RV7's 16 part B points.
- **Unblocks.** Settlement of Q-0427 and Q-0432 under decisions 0054 and 0055, if the review finds no doubt. Every compiler agrees on Q-0427, and EDG alone differs on Q-0432. Q-0424 also needs its doubt resolved.

### Item 4. GL. Guides for a programmer from another language

- **What.** The Port prompt in `exercises.md` lets the reader read `guides/`. A how-to guide covers three forms that P3 built from several pages: a lazy sequence as an iterator with an end type (RFOR-2, RFOR-P1, KW-1), a type-erased callable that owns its closure (LAM-P4, VIRT-P3, NEW-P2, CPY-P3), and the type of a call named through a declared function (ODR-3, INST-7). `resource-owner.md` gains the shared owner of P2-2 and P2-7, as round 11 proposed.
- **Needs first.** F5's examples, so that the guide cites them.
- **Evidence.** P3-1 to P3-3, and P2-1, P2-2, P2-4, P2-7 in phase 13. Decision 0046 keeps specification pages from linking a guide, so the prompt is the route.
- **Cost.** 0.5 session.
- **Risk.** Low. A guide cites rules and adds no fact.

### Item 5. I8. Implement on constraints

- **What.** An Implement exercise on `templates/constraints.md` (40 rules, 147 tests, 34 overrides, 22 questions): satisfaction, normal forms, and subsumption, with the I7 filter and scoring.
- **Needs first.** Nothing. FE26 can run beside it.
- **Evidence.** The page has 7 failing tests. No Implement exercise has read it. I6 and I7 gave 31 text findings on three template pages for two sessions of triage.
- **Cost.** 0.25 writer session to triage the report; the rows that follow come in the next phase.
- **Risk.** Low.

### Item 6. RC. Records

- **What.** Three records. `conformance.tsv` gains a column that names the TEMPLATE-MODEL section of each rewrite-marked test, and `frontend_check.py` prints the count apart in its completion line. RP's finding on `forward_class` (attributes after the class key, which `elaborated_type` does not allow) gets a ledger row. The reader prompts name a path in the scratchpad for the report, and the integrator copies the file, so that a refused write does not route the report through a message.
- **Needs first.** Nothing.
- **Cost.** 0.2 session.
- **Risk.** Low.

### What the rules need before front-end round 26 and the next exercises

- **Before FE26.** The CTAD-1 and CTAD-2 sentences, so that the site checks have a rule for `new` with no initializer and a static data member with none. The INST-5 sentence, so that P3-7 and P3-8 have a rule that names when the return type is made. The VIRT-13 sentence, so that G165 has an answer.
- **Before I8.** Nothing. The constraints page has no open ledger row.
- **Before a fourth Port.** GL, so that the reader can find the iterator and callable forms.
- **Propose and Diagnose.** Unchanged. F5 changes text by sentences, rows, and examples.

## 3. Out of scope

1. **No new topic.** Phase 14 added two rules and no page.
2. **Coroutines, modules, threads and the memory model, extended floating-point types, the preprocessor beyond `#include "file"`, and the general library stay out**, as the charter and decision 0051 say. P3-1 asks for a generator. The text answers it with an iterator class, and KW-1 keeps coroutines out.
3. **The maintainer's rows.** 114 of 185 open questions wait on rows 1 to 9 (62%), 129 with row 10 (70%). The request of 2026-11-10 has no answer. The writer does not choose for the maintainer.
4. **A large drop in the open count.** F5 opens four questions. RV8 can settle two or three.
5. **The front-end rewrite.** 30 tests need the template model of `frontend/TEMPLATE-MODEL.md`. The writer's items add no test that needs it.
6. **MSVC and later versions** stay deferred (row 7).
7. **The ADD-11 test that T3 did not write.** The test waits for Q-0430, which is in row 4a.

## 4. Process changes

Ranked by writer time saved.

1. **A reader writes a probe row for each "unclear" finding.** This round's grids decided 13 I7 and P3 rows and G165. Every host-only claim held on the matrix. The four new splits came from rows where the reader chose a reading and made no claim (I7-6, I7-13, I7-14, P3-6). A grid file in the reader's directory, with one row for each reading it chose, lets the writer run the matrix at once and saves the writer from rebuilding each case from the prose.
2. **The read of a repair diff is a file.** Decision 0062 asks for a recorded read of each repair diff before the item ends. RV7 recorded its read of 9c009448 inside the review files, and no file records a read of b5d925e0. A check in `check_specification.py` that each repair commit named in a gap review's ledger rows has a file under `reports/review/` that names its hash would catch the gap.
3. **Rewrite marks live in `conformance.tsv`** (item RC). The gate of decision 0062 counts tests, and the marks are in a report table. A column lets `frontend_check.py` give both counts, and the next decision can compare them with the gate.
4. **A finding in a commit body gets a ledger row.** RP's finding on `forward_class` sits only in 9359cc42's message. The ledger checks read reports, and they cannot see a commit body.
5. **A save step for reader reports.** Three more reports came back as messages in phase 14 (I7, P3, round 25), as three did in phase 13.

General knowledge used in this report: none. Every count comes from Tidal's files, its commit messages, scripts run on commits 45ee5e85, 6afa79f9, and bd87b27c, and the writer probes in `writer-probes/`.
