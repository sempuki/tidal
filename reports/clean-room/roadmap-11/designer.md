# Roadmap, round 11: what to do after phase 13, from a language designer's view

Date: 2026-11-11. Decision 0029, with decisions 0037, 0039, 0060 and 0061. Role: clean-room reader, language-designer persona (`designer`). Inputs: AGENTS.md and CLAUDE.md; my round 10 report `reports/clean-room/roadmap-10/designer.md` (the model for this form) and its scratch scripts; `reports/clean-room/roadmap-7/portfolio.md` (P22 to P26, with the P26 entry that phase 13 corrected); `documentation/` (decisions 0060 and 0061, `decisions/pending.md` with its decision packet and its table of requests, the status page, the roadmap's phase 13 and front-end sections, and sections 4, 8 and 9 of `exercises.md`); `git log c67b349b..3bd790ae` with every commit body; the I6 report (score and findings) and the P2 report; `reports/clean-room/rust-frontend/report-24.md` (sections 1, 5, 6, 8 and 9) and the front-end ledger rows G160 to G163; the two 2026-11-10 phase 13 gap reviews in `reports/review/` (result tables, and the RV6-A-1 and RV6-B-1 to B-15 sections); `reports/clean-room/ledger.md`; the change tables of X7 to X10, X14, X15, X17, X19, X20, X22 and X23 at the rows that phase 13 changed; targeted reads of CNS-11, CPY-15, NEW-22, NEW-29, DED-24, UNI-6, UNI-14, REF-1, LSUP-6, JMP-P3, STG-P2 and the table of storage forms in `foundations/storage.md` under `specification/`; the **Status** and **Waits for:** lines of Q-0196, Q-0300, Q-0411, Q-0412 and Q-0422 to Q-0427; `tests/` headers for the counts; and `tools/propose_check.py` plain, with `--corpus`, with `--strict`, and with `--strict` on each report alone. Counts come from read-only Python scripts in `reports/clean-room/roadmap-11/designer-scratch/scripts/`, run on this tree and on copies of `c67b349b` (the round 10 roadmap commit) and `98efc40f` (the tree RV6 read) made with `git archive`. Their outputs are beside the scripts. I did not read the other round 11 reports, `inputs/`, `build/exercise-keys/` or `frontend/`. I did not run `tools/run_tests.py`, `tools/question_index.py` or a compiler, so this report makes no host-only claim (decision 0061). Statements from outside Tidal's files are marked "general knowledge". Commit read: `3bd790ae`.

## 1. The state from a designer's view

### 1.1 Counts at 3bd790ae

| Measure | Round 10 (ebc109b9) | Now (3bd790ae) |
|---|---|---|
| Pages that hold rules | 85 | 85 |
| Rule IDs (with withdrawn) | 1698 | 1698 |
| `required` / `reject` / `undefined` | 898 / 559 / 57 | 898 / 559 / 57 |
| `impl-defined` / `recommended` / `no-diagnostic` | 52 / 43 / 37 | 52 / 43 / 37 |
| `unspecified` / `conditionally-supported` / `withdrawn` | 32 / 5 / 15 | 32 / 5 / 15 |
| Rules and principles whose text changed in the phase (my diff, `:tests:` left out) | 36 (32 rules, 4 principles), and TR-17 added | 42 (34 rules, 8 principles) |
| Of these, changed after RV6 read the tree (98efc40f) | | 22 (18 rules, 4 principles), the list of the close commit |
| Test files with a `tidal-expect` line | 6790 | 6881 |
| Of these, `reject` | 2762 | 2797 |
| Of these, `link-fail` | 46 | 46 |
| Test files with `-m32` in `tidal-flags` | 76 | 76 |
| Program corpus (`propose_check.py --corpus`) | 115 programs (86 Write, 29 sketches), 8 multi-unit, 138 files, 13615 lines | 126 programs (89 Write, 37 sketches), 8 multi-unit, 149 files, 15442 lines |
| Questions | 422 | 428 |
| Provisional / settled / compiler defects | 176 / 233 / 13 | 180 / 235 / 13 |
| Open questions in the packet's rows 1 to 9 | 111 of 176 (63%) | 113 of 180 (63%) |
| Propose reports | 23 | 23 |
| `propose_check.py`: conflicts | 31 (10 withdrawn, 4 class, 17 new-id) | 31 (10 withdrawn, 4 class, 17 new-id) |
| `propose_check.py`: rows changed since their report's commit | 141 | 148 |
| Change-table rows whose rule text changed in the phase (my diff) | 24, in 14 reports | 22, in 11 reports |
| `required` rules whose text contains "not valid" | 74 | 74 |
| `reject` rules cited by an `accept` or `run` test whose text contains `requires` | 44 of 559 | 49 of 559 |
| Front end, full pass / passes its check | 6586 / 6626 of 6790 | 6721 / 6745 of 6881 |
| Completion line of decision 0060 | 164 tests (no line yet) | 127 tests fail their check, 2 of 126 program tests do not run, 9 tests out of scope |
| Exercise ledger rows | 925; 878 `commit`, 17 `open`, 17 `text`, 10 `question`, 3 `out of scope` | 988; 920 `commit`, 26 `open`, 27 `text`, 12 `question`, 3 `out of scope`, 0 `planned` |

Phase 13 added no rule ID and changed no class. It changed the text of 34 rules and 8 principles. The 7 new changed rows of `propose_check.py` are REF-2 in X14, CAST-1 and DEP-15 in X20, CNS-11 and DEP-P4 in X22, and NEW-25 and DED-24 in X23. X1 to X12 name no commit, so the tool counts no changed row for them. My diff finds 6 more rows there: CPY-15, DTOR-6 and STG-10 in X9, DED-30 and DED-24 in X7, DEP-24 in X8, and DED-24 in X10.

`propose_check.py --strict` exits 1 on the whole set, and on X1, X2, X4 to X12 and X19 alone. X3, X13 to X18 and X20 to X23 exit 0. Both results are the same as in rounds 9 and 10. `tools/propose_check.py` has not changed since d9c18bcc.

### 1.2 What phase 13 did with the round 10 designer items

Phase 13 took round 10 item 1 in part and round 10 process changes 1 and 3 in part. Decision 0061 deferred X24, the refresh notes, the shared scanner, P27, the P25 prototype, the pair exercise, the index of visibility and the design document for the rewrite by name.

| Round 10 item | Phase 13 | State |
|---|---|---|
| 1. The Propose prompt, P26's entry, X24, the stop-rule count | R7 (4e23b699) | prompt and entry done. The prompt names its read list, the portfolio, a version line and the `--strict` table form. P26's entry gives 5 test files and no program, and adds PRIM-27 and three cases. X24 deferred again, so the stop-rule count still waits |
| 2. Refresh notes and a refresh check | none | deferred. `X22/X19-refresh.md` is still the one note |
| 3. A shared impact scanner | none | deferred |
| 4. The packet rows that reach proposals | PK4 (86749b4a) | row 4's rule items are counted (now 15, with DEP-15 and DEP-P4). The request on the record asks for rows 1, 4a and 10, and has no answer. Row 9 is not in the request. Row 7's plan still says about 2640 tests and 7900 compilations. There are 2797 `reject` tests, so about 8391 compilations |
| 5. P27 from W20 | F3 (0ae4cdb5), FE | the text part is done. UNI-14 answers W20-2. The table of storage forms beside `FixedVec` answers W20-6, and STG-P2 answers W20-1. P27 is not in the portfolio |
| 6. The substitution boundary | F3 | I5-2 and I5-5 done (DED-29, DED-24). RV6-B-3 then added the enumeration qualifier and the member of the wrong kind to DED-24. P23b's rating was not read again |
| 7. Pair exercise X14 with X23 | none | deferred |
| 8. A prototype hook in FR | none | deferred with the design document |
| 9. An index of what a test of validity can see | none | deferred. 49 of 559 `reject` rules now |

Round 10's process changes:
- Taken: process change 1 (R7). Process change 3 in one case: NEW-29 now sends the rejection of a deleted or inaccessible allocation function to NEW-25, a `reject` rule (G155). No check lists the other `required` rules that say "not valid".
- Not taken: process change 2. Phase 13 changed 22 change-table rows in 11 reports, and no phase 13 commit body names X7, X8, X9, X10, X14, X15, X17, X19, X20, X22 or X23. Process change 4: section 4's prose still ends at "Round 7 ... with X22 on P23", and section 8 still lists X20, X23, X22, X21 in that order. Process change 6 waits for X24.

### 1.3 Phase 13 text that reaches a done design

My diff of `c67b349b` against `3bd790ae` finds 22 change-table rows in 11 reports whose rule text changed. These reach a design:

| Report | Rows | Effect on the design |
|---|---|---|
| X22 (P23a) | CNS-11, DEP-P4, DED-24 | CNS-11 has a new second paragraph. When a candidate that is not a template takes every argument with no conversion, the constraints of the function template candidates are not checked (Q-0426). RV6-B-1 reads the paragraph against the reference, EDG follows the review, and Q-0426 sits in row 4b. X22's row on CNS-11 still holds, but a checked template's guarantee now rests on a check that may not run. DEP-P4's row for a constant parameter reads against the reference (RV6-B-2), and DEP-15 with DEP-P4 is a new row 4a rule item. X22 exempts the current instantiation, so the reach of that exemption follows the item. I6-8 finds that the DEP-P6 row which X22-4 asked for gives CNS-13 a weaker strength than CNS-13's class |
| X9 (P8) | CPY-15, DTOR-6, STG-10 | CPY-15 now covers a cv-qualified `T` by value. It says a constructor template never gives a constructor `T(T)`, and that the template's constraints are checked before such a specialization is dropped, unless CNS-11 skips the check. X9 makes `T(T)` a relocation constructor, so its amendment must say whether a constructor template can give one, and whether CNS-11's skip applies. DTOR-6 and STG-10 do not reach the design |
| X23 (P25) | NEW-22, NEW-25, NEW-29, DED-24 | NEW-22 now gives the sized aligned form to an over-aligned single object of complete type after `delete` (RV6-B-13, with a Clang 18 override). X23's amendment keeps only the aligned forms as candidates. The two fit, and X23:new-3 should name the sized aligned form. NEW-25 now covers the allocation function that a `new` expression selects (G155), and X23 adds the deallocation cases to the same rule. NEW-29 points to NEW-25, and X23 withdraws NEW-29, so X23:new-1 and X23:new-2 must carry the pointer. G162 (who defines `std::nothrow`) sits beside X23's open case of placement forms |
| X14 (P20) | REF-1, REF-2 | REF-1 gained a bullet for pointer conversions and a sentence for the lvalue of a conversion function (OVL-15). X14's phrase "and not under-aligned" still attaches to the opening clause "an lvalue that is not a bit-field", so it applies to the new bullet too. REF-2 now calls the binding of a non-class prvalue direct (RV6-B-4). X14's REF-2 row is about a copy of an under-aligned glvalue and fits, but it should use REF-2's new terms |
| X7 (P12) | DED-30, DED-24 | DED-30 now says a match instantiates the body that gives the result type, and points to DED-29. X7 amends DED-30 for an invented constant parameter. The amendment should restate DED-30's new sentence |
| X20 (P17) | CAST-1, DEP-15, DED-24 | CAST-1 now states the `void*` round trip. DEP-15 now says a list with no dependent argument names another specialization. X20's amendments apply as written |

The other rows change cases that the designs do not use: DEP-24 in X8 (the new exception is for an object of the current instantiation, and X8's row is about a dependent object), NAM-21 in X15, CONV-1 in X19, and DED-24 in X10 and X17.

DED-24 is in six change tables: X7, X10, X17, X20, X22 and X23. Each row is "unchanged, state effect", and each says that the design's new rejection inside a function type is a substitution failure. DED-24's list is the one place where six designs meet.

### 1.4 The 22 rules and principles changed after RV6, ranked by design reach

| Rank | IDs | Done designs reached | Why a designer reads them |
|---|---|---|---|
| 1 | CPY-15 | X9 | The relocation constructor meets CNS-11's skip and Q-0426 |
| 2 | NEW-22, REF-1, REF-2 | X23, X14 | P25's sized aligned form, and P20's binding rows. These are the rules of the X14 and X23 pair |
| 3 | DED-24 | X7, X10, X17, X20, X22, X23 | The substitution boundary of six designs |
| 4 | DEP-24, DEP-P5 | X8; X22 through the current instantiation | Which names after `.` or `->` are looked up at the definition |
| 5 | UNI-14, UNI-6, UNI-9, UNI-P1, UNI-P3 | none | The union model that P27 starts from |
| 6 | NAM-21, CONV-1, DTOR-6 | X15, X19, X9 | Rebase notes only |
| 7 | DED-8, VC-2, RET-6, JMP-10, JMP-P3, CV-6, DTOR-8 | none in a change table. X5 and X8 name DED-8 and VC-2 in prose, and X9 names RET-6 | No design cites them as changed |

The next gap review reads all 22. For a designer, RV7 should also read CNS-11's second paragraph beside CPY-15, since X9 and X22 both rest on it.

### 1.5 Pages the Propose exercises have reached

All 85 pages are still reached by rule or principle IDs cited in X1 to X23. No Propose exercise ran in phase 13. My scan counts principle IDs too, so it finds two pages read by one exercise only:

| Page | Exercise | Commits to the page since that report | Proposal that aims at it |
|---|---|---|---|
| `declarations/namespaces.md` | X15 | 7 | none open |
| `expressions/comma.md` | X1 | 5 | P26 |

`statements/loops.md` (X11, and X6 through LOOP-P1 and LOOP-P2) and `expressions/bitwise.md` (X17, and X1 through BIT-P1) each have a second reader by this count. Neither second reader designed a change to the page.

`templates/dependent-names.md` had its first blind build in phase 13 (I6). Most designs that touch templates cite its rules: DEP-2 is in X5 to X8 and X16, DEP-9 in X5 to X8 and X15, and DEP-30 in X6, X12, X13, X17, X18 and X22.

### 1.6 What a designer still cannot state

**Whether a constraint check runs.** CNS-11's second paragraph skips the constraints of template candidates when a candidate that is not a template matches every argument with no conversion. Clang 22 and GCC 16 skip, and EDG and Clang 18 check. A design that adds a constraint, a checked marker (X22), or a constructor template (X9) depends on which reading holds. Q-0426 waits on the maintainer in row 4b. Four tests name it.

**Which calls in a template body bind at the definition.** I6-1 finds no statement that an independent call uses argument-dependent lookup at the definition. I6-9 finds no rule for a qualified call with a dependent argument, or for a call whose name finds a class member. I6-5 asks whether `this->n` is type-dependent when `n` is not. I6-10 asks which point of instantiation argument-dependent lookup uses. P23b must say which candidate a requirement stands for, and X8 (P16) adds a fallback lookup beside argument-dependent lookup. Both rest on these answers.

**Whether a new rejection in a template is required before instantiation.** I6-7 cannot tell "must" from "may" for DEP-6 and DEP-12 against DEP-30. I6-8 finds that CNS-13 and DEP-P6 give different strengths. Six designs cite DEP-30, and X22 put a row in DEP-P6.

**How a loop goes on after a jump into its body.** G161 is `open`. A labeled `continue` (P22) to an outer loop goes on as after a normal pass of that loop. The loops page states neither case.

**What a standard attribute may do.** Q-0412 still holds a provisional answer and waits on the maintainer (row 9). No test names it, and the request on the record does not ask for row 9.

**Whether a later version has taken the space.** 17 Propose reports name Q-0007. Row 7's count is stale (section 1.2).

**How much code a change touches.** The three kept scanners use text and an injected header. P26 needs a query on syntax. The front end's interpreter already explores unspecified orders and reports when they agree (P2-9). That output could count the programs that change meaning under X1 (P3, left-to-right evaluation everywhere).

**Which programs change meaning without a diagnostic.** 49 of 559 `reject` rules have an `accept` or `run` test that uses `requires`.

**How two changes compose.** No pair exercise has run. REF-1 and REF-2 changed again in phase 13, and X14 and X23 meet at ALN-4.

**What a change costs.** No change. Q-0183 keeps option 1.

### 1.7 The portfolio

| # | Proposal | Round 10 | Now | What moved it | What still holds it back |
|---|---|---|---|---|---|
| P26 | No overloaded comma, checked left operand | ready | ready | R7 corrected the entry | X24 has not run. By my scan of the 126 programs, none declares `operator,`, and 9 files hold a fold over the comma. Of these, 8 fold calls and 1 folds an assignment (`r7-registry/registry.h`). `r5-hashmap-kept-pointer.cpp` has a comma in a `for` increment with a left operand that is not a call, twice. So at least 2 of 126 programs become invalid as the entry states it. The scan reads `for` heads and folds only |
| P27 | An in-object array whose elements start their lives one at a time | suggested, partial | ready, not in the file | F3: UNI-14 answers W20-2, and the table of storage forms says that no form inside the object allows constant evaluation, one array of `T` for pointer arithmetic, and a `T` with no default constructor together | The entry. Q-0425 (row 1) keeps the constant-evaluation answer that P27's motivation rests on |
| P24 | Flag enumerations | ready | ready | | The marker under Q-0412 (row 9). ENUM-10 in row 4a (Q-0407, no test names it) |
| P22 | Labeled `break` and `continue` | ready | ready | | The marker. A version line (row 7). G161 for the model of a labeled `continue` |
| P23b | Expressions checked against requirements | partial | partial | I5-2 and I5-5 answered | No matching rule (X22-8, `out of scope`). Q-0411 doubt. Q-0412. I6-1, I6-5, I6-9 and I6-10. CNS-11's skip (Q-0426) |
| P8b | Relocation by bytes | partial | partial | | Row 2 (Q-0322). Row 6 (Q-0343, CPY-19) |

The portfolio file has drifted. Its table gives P25 as "ready" and its totals count 4 ready proposals. X23 took P25, and `exercises.md` section 8 lists X23 as done. Totals now: 23 done, 3 ready in the file (P22, P24, P26), 2 partial, 0 blocked. P26 is still the one ready proposal that changes existing behavior.

P2 suggests a further entry. P2-4 finds that a wrapping hash spreads over ADD-9, MUL-10, MUL-8 and CONV-2, and P2-3 finds that `==` on two `const char*` compares addresses. Other languages offer named wrapping operations (general knowledge). A designer would write that as P28, "arithmetic that wraps on request", my own invention. P28 adds a form and changes no existing behavior. No done design cites ADD-9, MUL-8 or MUL-10 in a change table.

### 1.8 Questions and principles

Phase 13 opened 6 questions (Q-0423 to Q-0428) and settled 2 (Q-0419, Q-0428). Two open questions now wait for a use of a principle: Q-0422 for JMP-P3 and Q-0424 for UNI-P3, because both principles changed in the phase (decision 0055). Q-0196 still waits for TPL-P3, a principle that does not exist. The share line is 63% for rows 1 to 9 and 71% with row 10.

Principles that a Propose finding met, now:

| Principle | Finding | State |
|---|---|---|
| ATTR-P1, ATTR-P2, ATTR-P4 | X14-7, X22-6 | Q-0412, row 9. Bears on P22, P23a, P23b and P24 |
| DEP-P6 | X22-4 | the CNS-13 row exists. I6-8 says its strength differs from CNS-13's class |
| DEP-P4 | X22 (the exemption of the current instantiation) | RV6-B-2 reads the row for a constant parameter against the reference. The row is a row 4a rule item |
| NEW-P5 | X23-1 | row 4a rule item |
| TR-P1 | X21-4 | RV6-A-8 holds a note. RV6-A-19 holds the doubt on TR-17's two-unit case |
| STG-P2 | W20-1, W20-6 (P27's input) | answered by F3. RV6-A-16 repaired one table row |

### 1.9 The 26 open ledger rows, ranked by design reach

| Rank | Rows | What they reach | Who acts |
|---|---|---|---|
| 1 | I6-8 | X22's own DEP-P6 row against CNS-13 | writer |
| 2 | I6-7 | "must" or "may" before instantiation. DEP-30 is in six designs | writer, with probes |
| 3 | I6-1, I6-9, I6-5, I6-10 | Which calls bind at the definition. P23b, X8, and X5 to X7 through DEP-2 and DEP-9 | writer, with probes |
| 4 | I6-13, I6-15, I6-16 | The lists of uses that instantiate (INST-1, INST-5 with INST-33, friends). X7 cites INST-1 and X18 cites INST-5 | writer |
| 5 | P2-3, P2-4 | Examples on CMP-15 and on the multiplicative page. The motivation for P28 | writer |
| 6 | I6-14, I6-3, I6-4, I6-18, I6-19, I6-17 | Examples and wording in deduction and dependent names. No design reaches them | writer |
| 7 | P2-1, P2-2, P2-7, P2-6, P2-8 | Guidance for a program with no library, LSUP-1, and a CE-3 link | writer |
| 8 | P2-9 | The interpreter's first line when every order agrees, and with it a measured count for X1 | front end |
| 9 | I6-2, I6-20 | A mistake in the component, and outside knowledge. I6-20's last item (`this` in a local class of a function template) can go to DEP-2 | writer |

The front-end rows G161 and G162 rank beside rank 3 for a designer. G161 is P22's model, and G162 is in X23's area.

### 1.10 The distance to decision 0060's completion test

The completion line at the close of phase 13 reads "127 tests fail their check, 2/126 program tests do not run to their result, 9 tests out of scope". By status, the 127 are 69 missed rejects, 51 unsupported, 5 false rejects and 2 `EDG-AGREES`. By chapter, 57 are on template pages, 18 on class pages, 15 on expression pages, 13 on declaration pages, and 11 on overloading pages. `templates/dependent-names` has the most (10). The two program tests are `r10-list-first-try` (TR-13, no small reduction) and `r10-tuple-second-try` (class template argument deduction).

The count went from 164 at round 23 to 127 at round 24, so it is not stable over two rounds. Report 24 says that the DEP-24 tests of Q-0427, the DEP-15 tests and `substitution-member-wrong-kind` need a model of the current instantiation and of a specialization, which it calls rewrite work. Some failures then wait for the rewrite that the completion test gates.

For a designer, the rewrite's design document matters more than the count. Decision 0060 asks for modules by page with rule IDs in comments. A prototype of a proposal then changes the module of each page that the proposal amends, behind a flag. The design document does not depend on the count, and decision 0061 deferred it with "an amendment to decision 0060".

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | Refresh the portfolio (P25 done, P27 and P28 added, P26's counts at 126 programs), add host-only claims to the Propose prompt, then X24 on P26, then the stop-rule count | 1.2, 1.5, 1.7 | 1 short designer step, 1 reader session, 1 repair session | low | the second reading of `comma.md`; the stop rule's first full count |
| 2 | The dependence rows that reach designs: I6-8, I6-7, I6-1, I6-9, I6-5, I6-10, and G161; then P23b's rating again | 1.6, 1.9 | 1 writer session with probes, a short designer step | low | P23b; X8's and X22's rows; P22's `continue` |
| 3 | Refresh notes for X22, X9, X23, X14 and X7, and a refresh check that reads them and names the commit that changed each row | 1.1: 148 rows; 1.3 | 1 reader session, a small tool change | low | reuse of the done designs; item 7 |
| 4 | The packet rows that reach proposals: row 4b's Q-0426, row 4a's DEP-15 and DEP-P4 item, row 9's Q-0412, and row 7 with its count from a tool | section 5 | maintainer decisions; 1 writer step for the count | low | the checked marker of X22 and P23b; X9's relocation constructor; the markers of P22 and P24 |
| 5 | Propose X25 on P27 | 1.7 | 1 reader session after item 1 | low | the first design in the union and storage area since X14; a ready proposal that changes no existing behavior |
| 6 | A shared impact scanner with a syntax query, and an order count from the interpreter for X1 | 1.6; 1.7: the comma scan is a lower bound; P2-9 | 1 front-end or writer session | low | section 4.4 of X24; section 4.3 of X1 with a measured count |
| 7 | Pair exercise X14 with X23 | 1.3, 1.4 | 1 reader session after item 3 | low | an interaction section that can be checked |
| 8 | The FR design document with a prototype hook, under an amendment of decision 0060 | 1.10 | a writer session for the document and the amendment | medium | the first prototype of a proposal |
| 9 | An index of what a test of validity can see | 1.1: 49 of 559 | 2 writer sessions | medium | section 4.3 of every proposal |

### Item 1. The portfolio, the prompt, X24, and the stop-rule count

**What it is.** Four steps.
- **The portfolio.** The table marks P25 "done (X23)" and fixes the totals. P27 gets an entry with its pages (`foundations/storage.md`, `classes/unions.md`, `foundations/language-support.md`), the rules it meets (STG-P2, STG-P3, STG-10, STG-28, UNI-13, UNI-14, UNI-6, ADD-10, LSUP-10, LSUP-11) and the rating ready. P28 gets an entry with ADD-9, MUL-8, MUL-10 and CONV-2, and a rating. P26's entry gives the fold and `for` counts at 126 programs.
- **The prompt.** Decision 0061 asks a reader report to list its host-only claims. Every Propose probe runs on the host `g++` and `clang++`, so the Propose prompt should ask for that list.
- **X24 on P26.** The design states what a left operand may be, whether a call of an operator function or through a pointer counts, and whether a fold is checked for each element or once. Section 4.4 counts at least `r5-hashmap-kept-pointer.cpp` and `r7-registry` as becoming invalid, with item 6's scanner if it exists. P26 meets OPR-7, OPR-P2 and PRIM-27.
- **The stop rule.** After X24, count X22 to X24's findings and the share that the text closed. X22 closed 7 of 9 in the text (X22-8 is `out of scope`, X22-6 a question), and X23 closed 7 of 7.

**Evidence.** Sections 1.2, 1.5 and 1.7. `comma.md` has one reading (X1), and X1 names no commit. Decision 0061 deferred X24 after decision 0059 did.

**Cost.** One short designer step for the portfolio, one line in the prompt, one reader session, one repair session.

**Risk.** Low for similarity. P26 changes existing behavior, so its sections 4.3 and 4.4 need the full count.

**What it unblocks.** The second reading of `comma.md`, and the stop rule.

### Item 2. The dependence rows that reach designs

**What it is.** A writer answers these rows with probes on the matrix, in this order.
- I6-8: make DEP-P6's CNS-13 row and CNS-13's class agree.
- I6-7: say in DEP-6 and DEP-12 whether DEP-30 applies when no specialization is made.
- I6-1 and I6-9: say which lookups an independent unqualified call, a qualified dependent call, and a call whose name finds a member use, and when. Give the DEP-P1 table the rows.
- I6-5: say when a member access through `this` is type-dependent.
- I6-10: write "a point of instantiation" in DEP-9 and link INST-31 and INST-32.
- G161: say on the loops page how a loop goes on after a jump into its body.

The designer then reads P23b's rating again with X22-8, Q-0411, Q-0426 and these answers.

**Evidence.** Sections 1.6 and 1.9. I6 scored 352 of 358, and its wrong assertions trace to I6-1 to I6-4. The caller of I6's component supplied facts the component could not find, which is the same gap a checked template has.

**Cost.** One writer session with probes. Each answer is a sentence or a table row.

**Risk.** Low. I6-5 and I6-6 have no test that decides them, so each answer needs a new test.

**What it unblocks.** P23b, the rows of X8 and X22 that rest on lookup at the definition, and the model of P22's labeled `continue`.

### Item 3. Refresh notes and a refresh check

Round 10 item 2, with phase 13's rows.
- X22: CNS-11's skip and Q-0426. The exemption of the current instantiation against the DEP-15 and DEP-P4 item. I6-8.
- X9: CPY-15's sentence on constraints, and whether a constructor template gives a relocation constructor.
- X23: NEW-22's sized aligned form, NEW-25's selected allocation function, and NEW-29's pointer to NEW-25.
- X14: REF-1's new bullet, REF-2's direct binding of a non-class prvalue.
- X7: DED-30's new sentence.
- `tools/propose_check.py` reads a note beside a report and counts the rows the note covers as "noted". It names the latest commit that changed each row. It uses the commit that added a report when the report names none, so X1 to X12 get a changed-row count. My `changed_since.py` does the second part in 20 lines.

### Item 4. The packet rows that reach proposals

Section 5 gives the rows. The writer step is to make the row 7 plan's count come from a tool (2797 `reject` tests, about 8391 compilations), so that the count stops going stale. A second request on the record can name row 4b's Q-0426 and row 9's Q-0412.

### Item 5. Propose X25 on P27

P27 adds a form and changes no existing behavior, so section 4.4 should find no program that becomes invalid. The design must say how an element's lifetime starts and ends, what `std::construct_at` and placement `new` do on one element, what a pointer and a length over the started elements give, and what constant evaluation allows. It meets UNI-14's constant-evaluation sentence and Q-0425. X25 runs after item 1 so that the portfolio entry exists.

### Item 6. A shared impact scanner and an order count

Round 10 item 3, with one addition. The interpreter explores unspecified orders and reports the other outcomes (P2-9). A run of the 126 programs that lists each program whose explored orders give more than one result is a measured lower bound for X1's section 4.3. A scanner under `tools/` takes the text queries of X21 and X22, the injected-header method of X23, a syntax query that the front end answers, and this order count. It honors `tidal-flags` and reports `tests/programs/` apart from the rule tests.

### Item 7. Pair exercise X14 with X23

Round 10 item 7, unchanged in scope. The pair asks what `new` does for a packed class whose members are under-aligned, and how a reference binds to such a member under the new REF-1 and REF-2.

### Item 8. The FR design document with a prototype hook

Decision 0061 deferred the design document and an amendment to decision 0060. Report 24 names failures that need the rewrite's model of the current instantiation. An amendment that lets the design document start before the completion test holds would give the prototype hook a place. The document names how a proposal prototype attaches to a page module behind a flag. A P25 prototype follows FR's stage for `new-delete`.

### Item 9. An index of what a test of validity can see

Round 10 item 9, unchanged. 49 of 559 `reject` rules have an `accept` or `run` test that uses `requires`.

## 3. Keep out of scope

- **Coroutines, modules, threads, the memory model and the general library.** No proposal needs them. P2-1 asks for guidance on programs with no library, which examples can give.
- **P23b's exercise in the next phase.** It needs a matching rule, Q-0411, Q-0412, Q-0426 and item 2.
- **P8b** until the maintainer decides rows 2 and 6.
- **Rules or `:since:` fields for later versions.** Row 7's probe makes an index only.
- **A source-reader designer** (Q-0116). Twenty-three exercises have not needed one.
- **A cost model with numbers** (Q-0183 option 2).
- **Full re-runs of old Propose reports.** Item 3's notes cover the changes that reach a design.
- **ILP32 runs as their own item.** The 76 tests with `-m32` flags did not change in number, and no proposal needs more.
- **A P25 prototype in today's front end.** It would be removed with the old code.

## 4. Process changes

1. **A repair names the done designs its text change reaches.** Rounds 8 to 10 process change 2, still not taken. Phase 13 changed 22 rows in 11 reports, and no commit names them. Item 3's tool change can print the list for the commit message.
2. **The portfolio states the present.** A Propose exercise that takes a proposal marks it done in the portfolio file in the same commit. P25 is "ready" in the file after X23.
3. **Keep the exercise records current.** Section 4's prose names the latest round, and section 8 lists the Propose rows in order (X21, X22, X23). Round 10 process change 4, not taken.
4. **The Propose prompt asks for host-only claims** (decision 0061, item 1).
5. **A `required` rule that states a rejection points to a `reject` rule.** NEW-29 now does. A check could list the 74 `required` rules that say "not valid" with no `reject` rule cited in the same paragraph.
6. **The stop rule stays.** It counts X22 to X24.
7. **Keep the designer's view in round 12** in this form. Rerun the counts of sections 1.1, 1.3, 1.5 and 1.9, and the diff of changed rows against each done design.

## 5. The decision packet from a designer's view

The packet in `documentation/decisions/pending.md` gives each row's questions and an upper bound on the tests whose expectation changes. The order below follows the number of done designs and proposals that each row reaches.

| Order | Row | What it settles | Tests whose expectation changes | Proposals and done designs it reaches |
|---|---|---|---|---|
| 1 | 4b, Q-0426 | whether a constraint check runs when a candidate that is not a template matches exactly (CNS-11, CPY-15) | 0 if the compilers; at most 4 if the review (4 tests name Q-0426) | X22 and P23b (the checked marker), X9 (the relocation constructor), and every design that adds a constraint |
| 2 | 9, Q-0412 | which effects a standard attribute may have | 0 under either option (no test names Q-0412) | P22, P23a (X22), P23b and P24 choose a marker by it. X23 cites it. Under the kept answer, X22's `[[checked]]` adds required rejections and must become a keyword |
| 3 | 4a, rule items DEP-15 with DEP-P4, and NEW-P5 | whether a static constant member equal to the parameter names the current instantiation; whether `new` adds an alignment argument on a second search | 0 if the compilers; the bound for the rule items is wide (16 test files name DEP-15 or DEP-P4) | X22's exemption of the current instantiation; P25's mirror case (NEW-30) |
| 4 | 4a, Q-0407 | ENUM-10's type for an enumerator from another enumeration | 0 (no test names it) | P24, X20 |
| 5 | 7 | Q-0007, Q-0087, Q-0116, Q-0181, Q-0188 | 5 tests name them; the probe changes no expectation | the version line of 17 Propose reports, and of P22, P24, P26 and P27 |
| 6 | 1 | 55 questions, among them Q-0425 (UNI-14), Q-0420 (DED-29) and Q-0421 (LSUP-11) | 0 (131 tests name them) | P27 through Q-0425 and LSUP-11; P23a and P23b through DED-29 |
| 7 | 9, Q-0183 and Q-0186 | a cost model; judgments from outside Tidal | 0 | section 4 and the outside-knowledge line of every Propose report |
| 8 | 2 and 6 | Q-0322; Q-0343 and Q-0041 | 0 for row 2; at most 5 for row 6 | P8b (relocation by bytes cites CPY-19's trivially copyable list) |
| 9 | 10 | 16 questions that Tidal answers as its own choice | 0 (30 tests name them) | Q-0020 (how tests record a compiler that lacks a feature), which X4 and X6 cite |

The request on the record asks for rows 1, 4a and 10. Rows 1 and 4a reach P27, X22 and P25 through Q-0425, the DEP-15 and DEP-P4 item, and NEW-P5. Row 10 reaches X4 and X6 through Q-0020. Rows 4b and 9 reach the most designs and are not in the request. I found no proposal that rows 3, 5 and 8 reach.
