# Roadmap, round 11: what to do after phase 13, from a clean-room writer's view

Date: 2026-11-11. Decision 0029, with decisions 0033, 0039, 0050, 0051, 0054, 0055, 0056, 0057, 0058, 0059, 0060, and 0061. A clean-room agent in the role of a clean-room writer, who drafts rules, Evidence, and tests from compiler behavior, prepared this report. It read AGENTS.md, CLAUDE.md, the round 10 writer report `reports/clean-room/roadmap-10/writer.md` with its probes and scripts, `documentation/status.md`, `documentation/roadmap.md` (the phase 13 table), decisions 0060 and 0061, `documentation/decisions/pending.md`, `git log c67b349b..3bd790ae` with every commit body, the phase 13 reports (`reports/clean-room/I6/report.md`, `P2/report.md`, `rust-frontend/report-24.md`), the two phase 13 gap reviews in `reports/review/` (`2026-11-10-gap-review-phase-13-*`), the question files Q-0423 to Q-0428, `reports/clean-room/ledger.md`, `reports/clean-room/rust-frontend/ledger.md`, the guides index, and the rules that its items name. It did not read the other round 11 reports, `inputs/`, `build/exercise-keys/`, or `frontend/`. Counts come from read-only `git`, `grep`, and the scripts in `writer-scratch/` beside this report (copied from round 10, with `rule.py` and `make_grids.py` added), and from read-only runs of `tools/check_specification.py` and `tools/question_index.py --check`. Comparisons with the end of phase 12 read commit c67b349b through `git show`. Five probe grids ran with `tools/probe_grid.py` on the baseline compilers (`writer-probes/README.md`). Commit read: `3bd790ae`.

## 1. State from the writer's view

### 1.1 Counts

| Measure | End of phase 12 (c67b349b) | Now (3bd790ae) |
|---|---|---|
| Rule entries | 1698 (15 withdrawn), 1683 live | 1698 (15 withdrawn), 1683 live; no rule is new |
| Principles | 343 | 343 |
| Rules and principles whose text changed in the phase | 32 rules and 4 principles in phase 12 | 34 rules and 8 principles on 20 pages; no class change |
| Rules and principles changed after RV6 read the tree (98efc40f) | none | 18 rules and 4 principles on 13 pages, all in 9c009448 |
| Words on the 85 pages, without index and glossary | 269,253 | 273,009 |
| Evidence words on those pages | 68,855 | 70,879 |
| `cpp` example blocks | 222 | 224 |
| Tests (`*.cpp`, excluding `*.tu.cpp`) | 6790, of which 115 are program tests | 6881, of which 126 are program tests |
| Test files with a per-compiler override | 703 (869 override lines) | 716 (888 override lines) |
| Test files with an override for a newest family | 608 | 619: EDG alone 215, GCC alone 203, Clang alone 107, EDG and GCC 65, Clang and EDG 18, Clang and GCC 10, all three 1 |
| Tests with `tidal-error` patterns | 3094 | 3132 |
| Question files | 422, 176 open | 428, 180 open |
| `check_specification.py` | 1698 rules, 0 problems | 1698 rules, 0 problems |
| `question_index.py --check` | 422 questions, 0 problems | 428 questions, 0 problems |
| Front end (`conformance.tsv`) | 6586 of 6790 pass, 6626 pass their check | 6721 of 6881 pass, 6745 pass their check |
| Completion line (decision 0060) | 165 tests fail their check, 3 of 123 program tests do not run (item CT's first count) | 127 tests fail their check, 2 of 126 program tests do not run, 9 tests out of scope |
| Tests that fail their check, by status | 164: 100 missed rejects, 58 unsupported, 4 false rejects, 2 EDG-AGREES | 127: 69 missed rejects, 51 unsupported, 5 false rejects, 2 EDG-AGREES |
| Tests that pass their check and do not run | 40 run-unsupported | 24 run-unsupported |
| Exercise ledger | 925 rows: 878 commit, 17 open, 17 text, 10 question, 3 out of scope | 988 rows: 920 commit, 26 open, 0 planned, 27 text, 12 question, 3 out of scope |
| Front-end ledger | 157 rows: 146 commit, 5 open, 4 text, 1 question, 1 out of scope | 163 rows: 152 commit, 2 open (G161, G162), 4 text, 1 question, 4 out of scope |
| Committed similarity reports | 211 (34 MB) | 217 (37 MB); the phase added 6 reports and 27,599 lines |

Phase 13 added 91 tests and no rule. The phase opened 6 questions (Q-0423 to Q-0428) and settled 2 (Q-0419, Q-0428).

### 1.2 What phase 13 took from round 10

| Round 10 item | Phase 13 result |
|---|---|
| 1. F3 | Done in 0ae4cdb5 and 98efc40f. 38 tests. G153 and G155 to G157 are `commit`. Q-0428 opened and settled in RV6. |
| 2. FE24 | Done in 53577f27 and bafaf6da. 54 tests better, the completion count went from 175 to 127, and two new rows (G161, G162) are `open`. |
| 3. RV6 | Done in 7dd6d7ca, 9c009448, 2eb7bf32. 34 review points, each with a ledger row. Q-0426 and the DEP-15 item joined `pending.md` row 4. |
| 4. QR2 | Done in 4045bbdd. Q-0419 settled; Q-0423 holds the mixed case (row 10). |
| 5. PK4 | Done in 86749b4a. The packet lists 15 rule items of row 4a with 185 tests. The request to the maintainer of 2026-11-10 has no answer yet. |
| 6. OS | Done inside CT (2e588e25). G158 to G160 mark 5 tests `out of scope`, and G42 marks 4 more. |
| Process 1, a front-end finding gets its test in its commit | Adopted, and R7 (45d5b9bd) added the check. I6-11 and I6-12 came with their tests in e2592575. G161 and G162 name existing tests. |
| Process 2, a reader report lists its host-only claims | Adopted as a decision 0061 choice. Neither the I6 report nor the P2 report has the list, and `exercises.md` does not ask for it (section 4, process 1). |
| Process 3, the packet counts rule items | Done in PK4. |
| Process 4, one completion count | Done in CT. |
| Process 5, a save step for reader reports | Not done. The tool refused the writes of the I6, P2, and round 24 reports, and the integrator saved each from the returned text. |
| Process 6, smaller similarity reports | Not adopted as a change. Phase 13 added 6 reports and 27,599 lines, a third of phase 12's lines. |

### 1.3 The 26 open ledger rows and G161, G162, with the probes of this round

Each row that depends on compiler behavior has a probe in `writer-probes/`. The I6 and P2 readers ran their programs on the host GCC and Clang only. The grids add EDG and the older versions.

| Row | What the probe shows | What the text needs | Question |
|---|---|---|---|
| I6-1 | `i6-call-binding`: every compiler finds `lib::probe` for an independent call by argument-dependent lookup at the definition and ignores a better overload declared later. With no declaration before the template, every compiler rejects at the definition. | DEP-5 says that an independent unqualified call uses ordinary and argument-dependent lookup, both at the definition. The DEP-P1 table gains a row for the independent call. One accept test with a later overload, no override. | none |
| I6-2 | None needed; a fault of the component. | Optional: a DEP-P5 row for `p->template ~T()` beside DEP-34. | none |
| I6-3 | `i6-instantiation-checks`: `typename T::template box b(1);` with no list is accepted by GCC and EDG and rejected by Clang 18 and 22. | A DEP-P5 row for the deduced form that cites DEP-26 and Q-0226. The test `template-keyword-no-list-class.cpp` already holds the case. | Q-0226 (row 4) |
| I6-4 | None needed. | A DEP-P5 sentence: the first phase only chooses a reading; the second phase reports an error from what the name finds. | none |
| I6-5 | `i6-call-binding`: `f(this->n, 0)` and `f(n, 0)`, with `n` of a type that does not depend on the parameters, bind at the definition on Clang 22, GCC, and EDG. Clang 18 alone treats the `this->n` form as dependent. | DEP-2: a member access through `this`, or through an object of the current instantiation, is type-dependent only when the member's type is dependent or when lookup finds nothing and a dependent base exists (DEP-18). The `this` bullet then needs the same limit. One accept test with a later overload and a Clang 18 override. | none |
| I6-6 | `i6-instantiation-checks`: `B<sizeof(N)>::type` and `B<noexcept(N + 1)>::type` with no `typename`: Clang and GCC accept, EDG rejects and asks for `typename`. A `static_assert` cannot tell the readings apart (`i6-6-15-clean`). | DEP-4's last bullet leaves out the operands of `sizeof`, `alignof`, and `noexcept`; the `sizeof` bullet keeps its own cases. One accept test with an EDG override. | none (EDG alone differs) |
| I6-7 | `i6-instantiation-checks`: every compiler rejects an undeclared name (DEP-6) and a name that only a dependent base declares (DEP-12) in a template with no specialization. | DEP-P6 gains rows for DEP-6 and DEP-12 with "rejects". DEP-30's "a name in the body" names a name that lookup finds, so that DEP-30 does not cover DEP-6 and DEP-12. | none |
| I6-8 | The same grid: every compiler rejects the CNS-13 case before instantiation. | CNS-13 is a reject rule and says "also inside a template that is never instantiated". The DEP-P6 row says "rejects". | none |
| I6-9 | `i6-call-binding`: a qualified call `lib::f(T{}, 0)` does not see a later overload in `lib`; a call whose name finds a member does not see a later overload in an associated namespace. Every compiler agrees. | DEP-9: a qualified call with a dependent argument looks up its name at the definition in the named scope, with no argument-dependent lookup. DEP-14: an unqualified call whose name finds a class member takes no argument-dependent lookup. Two accept tests, no override. | none |
| I6-10 | `i6-call-binding`: a specialization first used before a later overload and checked after it. GCC sees the later overload; Clang and EDG do not. | DEP-9 says "a point of instantiation (INST-31)". An Evidence note on the split under INST-32, which needs no diagnostic. No test. | none |
| I6-13 | `i6-instantiation-checks`: every compiler accepts `decltype(make())` for a class whose instantiation fails, and rejects a call that passes that class by value. | INST-1 lists an argument that a call passes by value. INST-2 lists a call inside `decltype` whose result is the class. One accept test and one reject test, no override. | none |
| I6-14 | The same grid: every compiler rejects. `substitution-hard-error.cpp` already tests it. | An example under DED-29 with a fallback `f(...)`. | none |
| I6-15 | `i6-6-15-clean`: a `static constexpr` member whose initializer in the class fails. Clang and GCC accept an object and `sizeof` of the class and reject only a read of the member. EDG rejects all three. A `static inline` member gives the same split. A `static const int` member: Clang and EDG reject the object, GCC rejects only the read. | INST-33's last sentence says that the initializer of a `static constexpr` member is made with the class. Clang 22 and GCC 16 make it at a use of the member, and only EDG makes it with the class. INST-33's two tests read the member, so they do not show the difference. INST-33 then says that a use of the member makes the initializer, and INST-5's list keeps `constexpr` members. One accept test with an EDG override. The `static const int` case needs a question, because INST-5 follows GCC alone there. | New: a `static const` integer member's initializer in the class (Clang and EDG against GCC) |
| I6-16 | The same grid: every compiler accepts a class specialization whose friend body fails when nothing calls the friend, and rejects a call. | The INST-P2 table gains a row: a friend function defined in the class is made at a use, as a member function body is. One accept test, no override. | none |
| I6-17 | None needed. | DEP-37 links OPR-2, which gives the number of parameters of each operator function. | none |
| I6-18 | The same grid: every compiler rejects both forms in a template with no specialization, and accepts `int p = 0; T::x * p;`. | Comments in the example "The keywords": the first line is a product with an undeclared `q` (DEP-6); a line `T::type value;` breaks the grammar (DEP-29). | none |
| I6-19 | None needed. | DEP-20 names "the return type of a member function". | none |
| I6-20 | `i6-call-binding`: `this` in a local class of a function template is not type-dependent on Clang 22, GCC, and EDG; Clang 18 differs. | DEP-2 says so in the `this` bullet. The ranking, the lookup scope, and the operator arities are on other pages: DEP-9 links OVL-P1 (the steps of resolution) and NAM-P1 (scopes), and I6-17 adds OPR-2. One accept test with a Clang 18 override. | none |
| P2-1 | No probe. The guides `resource-owner` and `safe-arithmetic` hold most of what the reader built. The Port prompt kept the reader to `specification/`, and decision 0046 keeps specification pages from linking a guide. | No link. The Port prompt lets the reader read `guides/` (item GL). A sentence in DCMP-P1 that the language defaults equality and ordering and has no defaulted hash. | none |
| P2-2 | No probe. | A guide step for an owner whose copies share the object and count it, built from CPY-P2, CPY-P4, NEW-P2, and OPR-5. The specification gains no rule. | none |
| P2-3 | `p2-points`: in a `static_assert`, Clang rejects a comparison of two pointers to equal literals; GCC and EDG accept (Q-0033). | An example under PRIM-9 that compares two `const char*` at run time and states that `==` compares addresses. The example must not use constant evaluation, because Q-0033 splits there. | none |
| P2-4 | No probe. `guides/safe-arithmetic.md` section 3 already ties ADD-9, MUL-8, and MUL-10. | An example on the multiplicative page that computes a wrapping hash in `unsigned` and converts it back with CONV-2. The guide gains the same line (item GL). | none |
| P2-6 | `p2-points`: every compiler accepts a private constructor and a count of type `decltype(sizeof 0)` with no declaration of `std::size_t`. GCC and Clang reject a count before the pointer and a count of type `unsigned int`; EDG accepts both. | LSUP-1 says that the members come in that order, that the count has the type of `sizeof`, and that the constructor's access does not matter. One reject test for the reversed order with an EDG override, and one accept test for the private constructor. | none (EDG alone differs) |
| P2-7 | No probe. | The guide step of P2-2 shows a lookup that returns a pointer or `nullptr`. | none |
| P2-8 | No probe. | CV-8 already links CE-3. DCL-19, the array bound rule, gains a link to CE-3 and CV-8. | none |
| P2-9 | Front end. | A front-end change to the first line when every explored order gives one result. No text change. | none |
| G161 | `g161-jump-into-loop`: every compiler, in constant evaluation, runs the rest of a `while`, `for`, or `do` body after a `case` label inside it, then the iteration expression, then the condition. A `for` initializer that is an expression does not run. A `for` initializer that declares a variable with an initializer makes the jump not valid (JMP-6). | LOOP-1 to LOOP-3, or one new sentence on the loops page: a jump into a body enters the loop there, and after the body the loop goes on as after a normal pass. IF-12 and SW-1 link it. Two constant tests (`for` with an expression initializer, `do`), no override. | none |
| G162 | No probe. GCC and Clang link `nothrow-tag-self-declared.cpp` because the library defines the object. | LSUP-6 says that the library defines `std::nothrow` too, so a program declares it and does not define it. | none |

Six rows need only a link or a sentence (I6-2, I6-4, I6-17, I6-19, P2-8, G162). Ten rows need tests, about 15 in all. One new question comes from I6-15. Two rows change a rule against the current text: INST-33's last sentence (I6-15) and DEP-4's last bullet (I6-6). In both, Clang 22 and GCC 16 agree, and EDG alone differs.

### 1.4 The questions phase 13 opened

| Question | **Waits for:** | What a writer can do now |
|---|---|---|
| Q-0423 (I3-13, the mixed list case) | a review (silent) | Nothing. Packet row 10 holds the question. |
| Q-0424 (UNI-9, an unnamed class in an anonymous union) | a use of UNI-P3 | RV7 reads UNI-P3. The review supports the answer and GCC alone differs, so a use of the principle can settle it under decision 0055. |
| Q-0425 (UNI-14, a construction inside a member that is not active) | a review (doubt); newest families that agree | Nothing a probe adds. The doubt is whether the run-time construction makes the enclosing member active, and no compile-time or sanitizer test shows it. GCC 16 against GCC 14 in the array form stays. |
| Q-0426 (CNS-11, CPY-15) | the maintainer (row 4) | Nothing. The review reads against, and Clang 22 and GCC 16 keep the matrix's answer. |
| Q-0427 (DEP-24, a member template through an object of the current instantiation) | a review (doubt) | RV7 reads the RV6 wording of DEP-24 and the DEP-P5 row. Every compiler agrees, so a review with no doubt settles it. |
| Q-0428 | settled | none |

Q-0422 (a structured binding in a return) also waits for a use of JMP-P3, which RV6 changed. RV7 can use JMP-P3 for it.

### 1.5 The pending.md row 4 items that phase 13 added

| Item | Families | What it waits for |
|---|---|---|
| Q-0426, the CNS-11 forms | Clang 22 and GCC keep the matrix; EDG follows the review. | Row 4b. |
| Q-0426, the W20-3 copy | Clang 22 and GCC keep the matrix; EDG checks the constraint and accepts. | Row 4b. |
| DEP-15, DEP-P4 (a `static` constant member initialized with the parameter's name) | Every family keeps the matrix. | Row 4a, a rule item. The packet now lists 15 rule items. |

The RV6 reviews also named UNW-3 and LSUP-11 again, and their row 4 cells cite the new review. No writer step reaches these items. The request to the maintainer of 2026-11-10 (rows 1, 4a, and 10 first) is still open. Row 1 now lists 55 questions, row 4a 18 questions and 15 rule items, and row 10 16 questions.

### 1.6 The 22 rules and principles that changed after RV6 read the tree

The close lists DTOR-6, DTOR-8, CPY-15, UNI-P1, UNI-P3, UNI-6, UNI-9, UNI-14, CV-6, RET-6, REF-1, REF-2, NEW-22, CONV-1, NAM-21, VC-2, JMP-P3, JMP-10, DED-8, DED-24, DEP-P5, and DEP-24. `changed.py` from 98efc40f to 3bd790ae gives the same 18 rules and 4 principles, all from 9c009448. Ranked by the writer's need for a second reading:

1. UNI-6, UNI-14, UNI-P1, UNI-P3, UNI-9. The run-time construction inside a member that is not active changed from undefined to valid, and Q-0425 keeps a doubt there. UNI-P3 carries Q-0424.
2. DEP-24, DEP-P5, DED-24. DEP-24 carries Q-0427, and F4 adds four DEP-P5 rows (I6-2, I6-3, I6-4) beside the RV6 row.
3. JMP-10, JMP-P3, RET-6. JMP-P3 carries Q-0422.
4. CPY-15, REF-1, REF-2, NEW-22, DED-8, VC-2. Each repair closed a review point. NEW-22 has a Clang 18 override; REF-2 and the glossary changed the meaning of a direct binding.
5. CONV-1, CV-6, DTOR-6, DTOR-8, NAM-21. Each change is one clause.

F4 changes DEP-2, DEP-4, DEP-5, DEP-9, DEP-14, DEP-20, DEP-30, DEP-37, DEP-P1, DEP-P5, DEP-P6, INST-1, INST-2, INST-33, the INST-P2 table, LSUP-1, LSUP-6, PRIM-9's example, and the loops page. RV7 reads those as well.

### 1.7 Distance to decision 0060's completion test

The completion line reads "127 tests fail their check, 2/126 program tests do not run to their result, 9 tests out of scope". Round 24 moved the count from 175 to 127, and 92 tests were new in the round. The 127 are 69 missed rejects, 51 unsupported, 5 false rejects, and 2 EDG-AGREES. The pages with the most are `templates/dependent-names` (10), `templates/class-template-deduction` (9), `templates/constraints` and `overloading/conversion-sequences` (7 each), and `templates/instantiation` (6).

Report 24 names seven tests that need a model of the current instantiation and of a specialization, and it calls that model rewrite work: the four DEP-24 tests of Q-0427, two DEP-15 tests with a constant argument, and `substitution-member-wrong-kind.cpp`. The two program tests that do not run are `r10-list-first-try` (a TR-13 stop with no reduction) and `r10-tuple-second-try` (class template argument deduction).

Writer work touches the count in two ways:

- F4 adds about 15 tests. Most cover behavior the front end has (lookup at the definition, `decltype`, loops). The DEP-2 test for `this->n`, the DEP-4 test with no `typename`, and the INST-33 test can fail at first.
- No test is a new candidate for `out of scope`.

At 48 fewer per round, the count reaches zero in about three rounds, if the rewrite-only tests are counted apart. With them, the front end cannot meet the first condition before the rewrite. Section 4, process 4, gives a record for that case.

## 2. Proposed items, ranked

| Rank | Item | Evidence | Cost (writer sessions) | Similarity risk | Unblocks |
|---|---|---|---|---|---|
| 1 | F4. Answer the 26 open rows and G161, G162 | section 1.3; five probe grids | 1.25 | low | FE25 targets; text that RV7 reads |
| 2 | FE25. Front-end round 25 | F4's tests, P2-9, G161, 69 missed rejects, 51 unsupported | 0.25 for the ledger | low | the completion test |
| 3 | RV7. Gap review of the 22 changed rules, F4's text, and the principle uses | section 1.6; Q-0422, Q-0424, Q-0427 | 0.5 of repairs | medium | settlement of up to three questions |
| 4 | GL. Port and Write readers may read the guides; a shared-owner step | P2-1, P2-2, P2-4, P2-7; decision 0046 | 0.5 | low | a Port reader who finds the guides |
| 5 | I7. Implement on class template argument deduction | 9 failing tests; `r10-tuple-second-try`; no blind build has read the page | 0.25 for triage | low | text findings for the page with the second most failures |
| 6 | HC. The host-only claim list in the reader prompts | section 1.2; P2-6 and I6-5 differ on EDG or Clang 18 | 0.1 | low | probes at the time of the report |

Items 1, 3, 4, and 6 cost about 2.35 writer sessions. With FE25 and I7 triage, about 2.85.

### Item 1. F4. Answer the open rows with the probes of this round

- **What.** Section 1.3, row by row. In order:
  - The dependent-names page (I6-1 to I6-10, I6-17 to I6-20): DEP-2, DEP-4, DEP-5, DEP-9, DEP-14, DEP-20, DEP-30, DEP-37 sentences; DEP-P1, DEP-P5, DEP-P6 rows; the "The keywords" example comments; about 8 tests.
  - The instantiation page (I6-13 to I6-16): INST-1, INST-2, INST-33, the INST-P2 row, a DED-29 example; 4 tests; the new question on a `static const` member.
  - The loops page and the language-support page (G161, G162, P2-6): LOOP sentences, IF-12 and SW-1 links, LSUP-1 and LSUP-6 sentences; 4 tests.
  - The PRIM-9 example (P2-3) and the DCL-19 link (P2-8).
- **Needs first.** Nothing. The probes decide every row that depends on behavior.
- **Evidence.** Section 1.3 and `writer-probes/README.md`.
- **Cost.** 1.25 sessions.
- **Risk.** Low. The rows that change a rule (DEP-4, INST-33) follow Clang 22 and GCC 16 and record EDG as an override.
- **Unblocks.** FE25's targets; the I6 and P2 rows close.

### Item 2. FE25. Front-end round 25

- **What.** F4's tests; P2-9 (a first line that says every explored order gives one result); the 69 missed rejects; the TR-13 stop of `r10-list-first-try`; a mark on each failing test that needs the rewrite's model, with the reason.
- **Needs first.** F4 for its tests. The G161 sentence before the loop change.
- **Cost.** 0.25 writer session for the ledger rows that follow.
- **Risk.** Low.
- **Unblocks.** The completion count without the rewrite-only tests.

### Item 3. RV7. Gap review of the changed text

- **What.** A source-reader review of the 22 rules and principles of section 1.6 and of F4's text, with a use of JMP-P3 for Q-0422 and of UNI-P3 for Q-0424, a second reading of Q-0427 on the RV6 wording, and a first reading of F4's new question. Then the repairs and a read of the repair diff.
- **Needs first.** F4.
- **Cost.** 0.5 writer session of repairs.
- **Risk.** Medium. The union and dependent-names pages are where RV6 found points.
- **Unblocks.** Settlement of Q-0422, Q-0424, and Q-0427 under decisions 0054 and 0055, if the review finds no doubt. Each has at most one newest family that differs.

### Item 4. GL. Readers may read the guides

- **What.** The Port and Write prompts in `exercises.md` let the reader read `guides/`. Decision 0046 keeps specification pages from linking a guide, so the prompt is the route. `resource-owner.md` gains a step for an owner whose copies share one object and count it, with a lookup that returns a pointer or `nullptr` (P2-2, P2-7). `safe-arithmetic.md` gains the wrapping hash of P2-4.
- **Needs first.** Nothing.
- **Evidence.** P2-1, P2-2, P2-4, P2-7. The reader built what two guides show, because the prompt kept the reader to `specification/`.
- **Cost.** 0.5 session.
- **Risk.** Low. A guide cites rules and adds no fact.
- **Unblocks.** The guides on containers and owners that decision 0061 left for later, in a smaller form.

### Item 5. I7. Implement on class template argument deduction

- **What.** An Implement exercise on `templates/class-template-deduction.md` (29 rules, 120 tests, 22 overrides, 14 questions), with the I6 filter and the same scoring.
- **Needs first.** Nothing. FE25 can run beside it.
- **Evidence.** The page has 9 failing tests, the second most. `r10-tuple-second-try` stops there. I6 gave 17 text findings on two pages for one session of triage, more than any Port or Write exercise.
- **Cost.** 0.25 writer session to triage the report; the F5 rows that follow come in the next phase.
- **Risk.** Low.

### Item 6. HC. Host-only claims in the reader prompts

- **What.** The Implement, Write, and Port prompts in `exercises.md` ask for a section "Host-only claims", which lists each claim that rests on the host GCC and Clang alone. `check_specification.py` warns when a new reader report has no such section.
- **Needs first.** Nothing.
- **Evidence.** Decision 0061 adopted the choice, and neither phase 13 reader report follows it. This round's grids found EDG or Clang 18 differing on P2-6, I6-5, I6-6, I6-15, and I6-20. The reports cite GCC and Clang alone for these rows, or no compiler.
- **Cost.** 0.1 session.
- **Risk.** Low.

### What the rules need before front-end round 25 and the next exercises

- **Before FE25.** The G161 sentence, so that the loop change has a rule. The DEP-2 and DEP-4 sentences and their tests, so that the front end binds `this->n` and `B<sizeof(N)>::type` as Clang 22 and GCC do. The INST-33 change, so that the front end does not make a `constexpr` initializer with the class.
- **Before I7.** Nothing. The CTAD page has no open ledger row.
- **Before a third Port.** GL, so that the reader can read the guides.
- **Propose and Diagnose.** Unchanged: F4 changes text by sentences and rows only.

## 3. Out of scope

1. **No new topic.** Phase 13 added no rule and no page.
2. **Coroutines, modules, threads and the memory model, extended floating-point types, the preprocessor beyond `#include "file"`, and the general library stay out**, as the charter and decision 0051 say.
3. **The maintainer's rows.** 113 of 180 open questions wait on rows 1 to 9 (63%), 127 with row 10 (71%). The request of 2026-11-10 has no answer. The writer does not choose for the maintainer.
4. **A large drop in the open count.** F4 opens one question. RV7 can settle three.
5. **The front-end rewrite.** 127 tests fail their check, and at least 7 need the rewrite's model.
6. **MSVC and later versions** stay deferred (row 7).
7. **The run-time doubt of Q-0425.** No test that the matrix runs can show whether the enclosing member becomes active.

## 4. Process changes

Ranked by writer time saved.

1. **A reader prompt asks for the host-only claims** (item 6). This round's five grids decided 17 rows. In five of them (P2-6, I6-5, I6-6, I6-15, I6-20), EDG or Clang 18 differs from the host GCC and Clang.
2. **A test of an event that a rule names separates that event.** INST-33's tests read the member, and the read alone makes every compiler instantiate the initializer. So the tests passed while the sentence "made with the class" held for EDG only. A rule that says when something happens needs one test where only that event can make the difference.
3. **A save step for reader reports.** Three more reports came back as messages in phase 13 (I6, P2, round 24).
4. **The front-end ledger marks rewrite-only tests.** A row of kind "rewrite" names each failing test that needs a model the current front end does not have, with the reason. `frontend_check.py` prints them apart in the completion line. Decision 0060 can then compare the remaining failures with the rewrite's scope, which its "few and stable" clause asks.
5. **A principle that changed gets a use in the next review.** Q-0422 and Q-0424 wait for a use of JMP-P3 and UNI-P3. The review prompt can name each principle that a question waits on, so that the review is the use.

General knowledge used in this report: none. Every count comes from Tidal's files, its commit messages, scripts run on commits c67b349b, 98efc40f, and 3bd790ae, and the writer probes in `writer-probes/`.
