# Roadmap, round 12: what to do after phase 14, from a language designer's view

Date: 2026-11-11. Decision 0029, with decisions 0037, 0039, 0060, 0061 and 0062. Role: clean-room reader, language-designer persona (`designer`). Inputs: AGENTS.md and CLAUDE.md; my round 11 report `reports/clean-room/roadmap-11/designer.md` (the model for this form) and its scratch scripts; `reports/clean-room/roadmap-7/portfolio.md` (P22 to P28, as item R8 refreshed it); `documentation/` (decisions 0060 to 0062, `decisions/pending.md` with its decision packet and its table of requests, the status page, the roadmap's phase 14 table, and sections 4, 8 and 9 of `exercises.md`); `git log 45ee5e85..HEAD` with every commit body; `frontend/DESIGN-current.md` and `frontend/TEMPLATE-MODEL.md`; the I7 and P3 reports; `reports/clean-room/rust-frontend/report-25.md` (sections 1, 7 and 8) and the front-end ledger rows G164 and G165; the two 2026-11-11 phase 14 gap reviews in `reports/review/` (result tables and the RV7-A and RV7-B points); `reports/clean-room/ledger.md`; the change tables of X5 to X9, X12 to X16, X18, X20 and X22 at the rows that phase 14 changed; targeted reads of ADD-14, CNS-13, CNS-P3, DED-20, DEP-2, DEP-5, DEP-9, DEP-40, DEP-P6, LOOP-9, PRIM-9, STG-P2, SW-1 and the table of storage forms in `foundations/storage.md` under `specification/`; the **Status** and **Waits for:** lines of Q-0007, Q-0196, Q-0300, Q-0411, Q-0412, Q-0424 to Q-0427, Q-0429, Q-0430, Q-0433 and Q-0434; `tests/` headers for the counts; and `tools/propose_check.py` plain, with `--corpus`, and with `--strict`. Counts come from read-only Python scripts in `reports/clean-room/roadmap-12/designer-scratch/scripts/`, run on this tree and on copies of `45ee5e85` (the round 11 roadmap commit, whose `specification/` and `tests/` equal those of `3bd790ae`, the tree round 11 read) and `6afa79f9` (the tree RV7 read) made with `git archive`. Their outputs are beside the scripts. I did not read the other round 12 reports, `inputs/`, `build/exercise-keys/` or the sources under `frontend/`. I did not run `tools/run_tests.py`, `tools/question_index.py`, the front end or a compiler. Statements from outside Tidal's files are marked "general knowledge". Commit read: `bd87b27c`.

## 1. The state from a designer's view

### 1.1 Counts at bd87b27c

| Measure | Round 11 (3bd790ae) | Now (bd87b27c) |
|---|---|---|
| Pages that hold rules | 85 | 85 |
| Rule IDs (with withdrawn) | 1698 | 1700 |
| `required` / `reject` / `undefined` | 898 / 559 / 57 | 900 / 559 / 57 |
| `impl-defined` / `recommended` / `no-diagnostic` | 52 / 43 / 37 | 52 / 43 / 37 |
| `unspecified` / `conditionally-supported` / `withdrawn` | 32 / 5 / 15 | 32 / 5 / 15 |
| Rules and principles whose text changed in the phase (my diff, `:tests:` left out) | 42 (34 rules, 8 principles) | 38 (27 rules, 11 principles), and DEP-40 and LOOP-9 added |
| Of these, changed after the gap review read the tree | 22 (after RV6) | 20 (13 rules, 7 principles) after RV7 (6afa79f9), the list of the close commit |
| Test files with a `tidal-expect` line | 6881 | 6947 |
| Of these, `reject` | 2797 | 2808 |
| Of these, `link-fail` | 46 | 46 |
| Test files with `-m32` in `tidal-flags` | 76 | 76 |
| Program corpus (`propose_check.py --corpus`) | 126 programs (89 Write, 37 sketches), 8 multi-unit, 149 files, 15442 lines | 138 programs (94 Write, 44 sketches), 8 multi-unit, 161 files, 17358 lines |
| Questions | 428 | 434 |
| Provisional / settled / compiler defects | 180 / 235 / 13 | 185 / 236 / 13 |
| Open questions in the packet's rows 1 to 9 | 113 of 180 (63%) | 114 of 185 (62%) |
| Propose reports | 23 | 23 |
| `propose_check.py`: conflicts | 31 (10 withdrawn, 4 class, 17 new-id) | 31 (10 withdrawn, 4 class, 17 new-id) |
| `propose_check.py`: rows changed since their report's commit | 148 | 155 |
| Change-table rows whose rule text changed in the phase (my diff) | 22, in 11 reports | 27, in 12 reports |
| `required` rules whose text contains "not valid" | 74 | 75 (LOOP-9 added) |
| `reject` rules cited by an `accept` or `run` test whose text contains `requires` | 49 of 559 | 49 of 559 |
| Front end, full pass / passes its check | 6721 / 6745 of 6881 | 6804 / 6831 of 6947 |
| Completion line of decision 0060 | 127 tests fail their check, 2 of 126 program tests do not run, 9 out of scope | 107 tests fail their check, 3 of 138 program tests do not run, 9 out of scope |
| Exercise ledger rows | 988; 920 `commit`, 26 `open`, 27 `text`, 12 `question`, 3 `out of scope` | 1044; 960 `commit`, 26 `open`, 33 `text`, 22 `question`, 3 `out of scope`, 0 `planned` |
| Front-end ledger rows `open` | G161, G162 | G164, G165 |

Phase 14 added two rule IDs (DEP-40, LOOP-9) and changed no class. The 7 new changed rows of `propose_check.py` are DED-20 in X13, DEP-2 in X16, INST-5 in X18, and CNS-13, CNS-P3, DEP-30 and INST-P1 in X22. X1 to X12 name no commit, so the tool counts no changed row for them. My diff finds 15 more rows there (section 1.3).

`propose_check.py --strict` exits 1 on the whole set. `tools/propose_check.py` has not changed since d9c18bcc, and no Propose report was added, so the per-report results of round 11 stand.

### 1.2 What phase 14 did with the round 11 designer items

Phase 14 took round 11 item 1 in part, item 2 for the text, and item 8 in part. Decision 0062 deferred X24, the refresh notes, the shared scanner, the pair exercise and the index of visibility by name. No Propose exercise has run since X23 in phase 11.

| Round 11 item | Phase 14 | State |
|---|---|---|
| 1. Portfolio, prompt, X24, stop-rule count | R8 (f4109f3a) | portfolio and prompt done. P25 is done (X23), P27 and P28 have entries, and every prompt asks for a "Host-only claims" heading. X24 deferred a third time (decisions 0059, 0061, 0062), so the stop-rule count still waits. P26's entry carries my round 11 count of 9 files with a fold over the comma. My own scan output lists 7 (section 1.7) |
| 2. The dependence rows that reach designs | F4 (1130ceb8), RV7 (b5d925e0) | text done. DEP-6 and DEP-12 now apply when no specialization is made (I6-7), DEP-P6 lists their class (I6-8), DEP-5 and the DEP-P1 table say which calls bind at the definition (I6-1), DEP-40 is new (I6-9), DEP-2 has the exception for a member access through `this` (I6-5, Q-0434), DEP-9 names a point of instantiation and INST-32 (I6-10), and LOOP-9 is new (G161). RV7 then set DEP-P6's CNS-13 row to "need not diagnose" (RV7-B-2) and limited DEP-40 to a namespace or an independent class qualifier (RV7-B-7). P23b's rating was not read again |
| 3. Refresh notes and a refresh check | none | deferred. `X22/X19-refresh.md` is still the one note |
| 4. The packet rows that reach proposals | none | the request on the record (2026-11-10) has no answer. Row 4 gained Q-0430 and the rule item ADD-14 with ADD-P6. Row 7's plan still says about 2640 tests and 7900 compilations. There are 2808 `reject` tests, so about 8424 compilations |
| 5. X25 on P27 | none | not taken. P27's entry exists |
| 6. A shared scanner and an order count | FE (b035efd4) for P2-9 | the scanner is deferred. `tidal-fe run` now says on its first line when every explored order gives one result, so the order count for X1 can be made. Nobody has made it |
| 7. Pair exercise X14 with X23 | none | deferred |
| 8. The FR design document with a prototype hook | RP (9359cc42) | `DESIGN-current.md` describes the current front end and `TEMPLATE-MODEL.md` the template model. Neither names a prototype hook. Decision 0062 set the gate at 50 tests, and the count is 107 |
| 9. An index of what a test of validity can see | none | deferred. 49 of 559 `reject` rules |

Round 11's process changes:
- Taken: process change 2 (R8 marked P25 done). Process change 4 (R8 put the host-only list in the shared rules of section 9). The I7 and P3 reports carry the heading.
- Not taken: process change 1. Phase 14 changed 27 change-table rows in 12 reports, and the one phase 14 commit body that names a done design is R8's line "P25 is done (X23)". Process change 3: section 4 of `exercises.md` still opens with the portfolio path `reports/clean-room/roadmap/designer.md`, its prose still ends at "Round 7 ... with X22 on P23", and section 8 still lists X20, X23, X22, X21 in that order. Process change 5 has no check. Process change 6 waits for X24.

### 1.3 Phase 14 text that reaches a done design

My diff of `45ee5e85` against `bd87b27c` finds 27 change-table rows in 12 reports whose rule or principle text changed. These reach a design:

| Report | Rows | Effect on the design |
|---|---|---|
| X22 (P23a) | CNS-13, CNS-P3, DEP-P6, DEP-30, INST-P1 | CNS-13's first bullet and CNS-P3's table now say that an independent template-id whose arguments fail the constraints, inside a template that is never instantiated, falls under DEP-30 and need not be diagnosed. DEP-P6's row says the same (RV7-B-2). X22's CNS-13 row says "independent template-ids are already checked at the definition", which no longer holds as a requirement. X22:new-3 requires a rejection for a dependent template-id in a checked template. A checked template then has a required check for dependent template-ids and an optional one for independent template-ids. INST-P1 now exempts only a call that is the whole operand of `decltype` (RV7-B-6), and X22's row ("the check makes no specialization") still fits |
| X8 (P16) | DEP-9; DEP-2 in prose | X8 section 1.7 says that `this` is type-dependent (DEP-2), so the choice of a member or the fallback for `this->f()` waits for the instantiation. DEP-2 now says that a member access through `this` is type-dependent only for a dependent member type, or when the name finds nothing and the class has a dependent base (Q-0434). With no dependent base, `this->f()` that finds no member is then not type-dependent, and X8's fallback is chosen at the definition. X8 states that choice for lookup and the opposite for resolution |
| X7 (P12) | DEP-2, DEP-4 | X7 amends DEP-2 (a `constexpr auto` parameter's name is type-dependent) and DEP-4 (a value-specialized parameter's name is value-dependent). DEP-4's last bullet now leaves out the operands of `sizeof`, `alignof` and `noexcept` (Q-0432). X7's amendment must say whether `sizeof` of a value-specialized parameter is value-dependent, which by the new DEP-4 depends on the parameter's type alone |
| X6 (P10) | DEP-2, DEP-4, DEP-30, DEP-P2, INST-P1, LSUP-1 | X6 adds items to DEP-2, DEP-4 and DEP-P2 for the loop variable of an expansion body. The new DEP-2 paragraph is an exception to the last item of the list, and X6's item must sit before or after the exception on purpose. DEP-30 now covers only a name that lookup finds. X6's DEP-30 amendment for an expansion body outside a template still fits. LSUP-1 now gives the order of the members (P2-6). X6's row says a braced list in an expansion statement needs no `std::initializer_list`, which still holds |
| X5 (P9) | DEP-2, DEP-9 | X5 amends DEP-2 for the name of an overload set. The amendment must rebase onto the new list. DEP-9's change does not reach X5's row |
| X15 (P21) | DEP-9 | DEP-9 now says "a point of instantiation" and that INST-32 applies when the points give different results. X15:new-2 makes lookup at the point of instantiation use the position where the call is written. The amendment must say whether a call with several points can still give different results, which INST-32 makes ill-formed with no diagnostic required |
| X14 (P20) | REF-2 | REF-2 now lists three direct cases (RV7-A-5). X14's REF-2 row binds a `const` reference to a copy of an under-aligned glvalue, which is none of the three. The row fits and should name the list |
| X9 (P8) | STG-28 | STG-28 now says that a `T*` at a multiple of `sizeof(T)` bytes into allocated storage points to that element (F4). X9's growable-array example relocates into elements that exist and are not built, and fits. G164 says STG-28 does not state which element a cast from a byte pointer reaches, and RV7-A-10 reads the step over a declared byte array as undefined (Q-0430, row 4a) |
| X13 (P6) | DED-20, DEP-30 | DED-20 now covers only a pack of the template that the call deduces (Q-0429). X13's row (a pack takes positional arguments only) fits |
| X12 (P19) | SW-1, DEP-30 | SW-1 now links LOOP-9. X12 amends SW-1 ("the end of the `switch`" gains "unless X12:new-4 applies"). Rebase note only |
| X16 (P4), X18 (P18) | DEP-2; DEP-30, INST-5 | unchanged in effect. INST-5 now says a friend body and a `constexpr` static member's initializer wait for a use. X18's row is about member declarations |

DEP-30 is in five change tables now (X6, X12, X13, X18, X22), and DEP-2 in four (X5, X6, X7, X16). These two rules and DEP-9 hold most of the rows that phase 14 changed.

### 1.4 The 20 rules and principles changed after RV7, ranked by design reach

| Rank | IDs | Done designs and proposals reached | Why a designer reads them |
|---|---|---|---|
| 1 | CNS-13, CNS-P3, DEP-P6, DEP-30 | X22; X6, X12, X13, X18 through DEP-30; P23b | The strength of the check before instantiation, which X22's marker raises for dependent template-ids only |
| 2 | STG-P2, UNI-14, UNI-P3 | P27; X14 names STG-P2 in prose | The base of X25. STG-P2 now states the typed step over bytes that RV7-A-10 reads against (Q-0430) |
| 3 | DEP-4, DEP-6, DEP-40, DEP-P1, DEP-P5 | X6, X7 through DEP-4; X8 and X15 name DEP-P1 or DEP-P5 in prose; P23b | Which calls bind at the definition, and which operands make a value dependent |
| 4 | INST-1, INST-2, INST-P1 | X6, X22; X7 names INST-1 | The `decltype` exemption |
| 5 | REF-2 | X14 | Rebase note |
| 6 | DED-20 | X13 | Rebase note |
| 7 | LOOP-9 | P22 | The model of a labeled `continue` |
| 8 | LSUP-6 | X23 names it in prose | `std::nothrow` beside X23's open placement case |
| 9 | DTOR-8 | none | |

The next gap review reads all 20. For a designer, the review should also read X22:new-3 beside the new DEP-P6 row. A source reader can say whether the reference treats the dependent and the independent cases with one strength.

### 1.5 Pages the Propose exercises have reached

All 85 pages are still reached by rule or principle IDs cited in X1 to X23. My scan finds the same two pages read by one exercise only, and phase 14 did not change either:

| Page | Exercise | Commits to the page since that report | Proposal that aims at it |
|---|---|---|---|
| `declarations/namespaces.md` | X15 | 7 | none open |
| `expressions/comma.md` | X1 | 5 | P26 |

`templates/class-template-deduction.md` had its first blind build in phase 14 (I7, 290 of 296). Two done designs cite it: X7 has rows on CTAD-5 and CTAD-22, and X20 cites the alias rows of CTAD-14 and gives an explicit alias no class template deduction.

### 1.6 What a designer still cannot state

**Whether a constraint check runs.** Unchanged. CNS-11 skips the constraints of template candidates when a candidate that is not a template matches every argument with no conversion, and CPY-15 relies on the skip. RV7-A-2 keeps the split, and Q-0426 waits in row 4b. Four tests name it. X9, X22 and P23b depend on the answer.

**Whether a typed step over bytes is defined.** New in phase 14. ADD-14 now says that `p + 1` for a `T*` that points to a byte holds the address `sizeof(T)` bytes on, and STG-29 and STG-P2 let placement `new` use that position. Clang 22 and GCC 16 give that answer, and EDG runs no program. Both RV7 reviews read the step as undefined (RV7-A-10, RV7-B-3), and Q-0430 sits in row 4a with two cases. G164 adds that STG-28 does not say which element a `T*` cast from a byte pointer at an offset into allocated storage reaches. The table of storage forms says that no form inside the object gives one array of `T` for pointer arithmetic, constant evaluation, and a `T` with no default constructor together. The byte-row forms of that table rest on the steps in dispute, and P27 gives a defined form in their place.

**Whether a checked template checks independent template-ids.** New in phase 14 (section 1.3). Under the kept answer of Q-0412, an attribute adds no required rejection, so X22's `[[checked]]` must become a keyword. Under either spelling, X22 must now say what the marker does to CNS-13's independent case.

**What a standard attribute may do.** Q-0412 still holds a provisional answer and waits on the maintainer (row 9). No test names it.

**Whether a later version has taken the space.** 17 Propose reports name Q-0007. Row 7's count is stale (section 1.2).

**How much code a change touches.** The three kept scanners use text and an injected header. The front end can now count the programs whose explored orders agree (P2-9), and no run has done so for X1. My text scans for P26 and P29 cannot tell a class operand from a pointer, so each needs a reading by hand (section 1.7).

**Which programs change meaning without a diagnostic.** 49 of 559 `reject` rules have an `accept` or `run` test that uses `requires`.

**How two changes compose.** No pair exercise has run. REF-2, STG-29 and STG-P2 changed again in phase 14, and X14 and X23 meet at ALN-4.

**What a change costs.** No change. Q-0183 keeps option 1.

These points of round 11 are now answered: which calls bind at the definition (DEP-5, DEP-40, DEP-9, the DEP-P1 table), whether a rejection before instantiation is required (DEP-6, DEP-12 and DEP-37 reject; DEP-30 and CNS-13's independent case need not diagnose), and how a loop goes on after a jump into its body (LOOP-9).

### 1.7 The portfolio

| # | Proposal | Round 11 | Now | What moved it | What still holds it back |
|---|---|---|---|---|---|
| P27 | An in-object array whose elements start their lives one at a time | ready, not in the file | ready | R8 added the entry. F4 and RV7 changed STG-28, STG-29, STG-P2, ADD-14, UNI-14 and UNI-P3 | Q-0430 (row 4a) and G164 bear on the workaround. Q-0425 (row 1) keeps the constant-evaluation answer |
| P26 | No overloaded comma, checked left operand | ready | ready | none | X24 has not run. By my scan of the 138 programs, none declares `operator,`, and 7 files hold a fold over the comma: `r10-list.cpp`, `r10-list-first-try.cpp`, `r10-tuple.cpp`, `r10-tuple-second-try.cpp`, `r7-registry/registry.h`, `r8-fmt.cpp` and `r9-vec.cpp`. The entry says 9, a figure from my round 11 prose that my own scan output does not give. 6 files fold calls, and `registry.h` folds an assignment. `r5-hashmap-kept-pointer.cpp` has a comma in a `for` increment with a left operand that is not a call, twice. So at least 2 of 138 programs become invalid. The 12 new programs (R11 and P3) add none |
| P22 | Labeled `break` and `continue` | ready | ready | F4: LOOP-9 states that a loop goes on as after a normal pass of the body (G161) | The marker. A version line (row 7) |
| P24 | Flag enumerations | ready | ready | | The marker under Q-0412 (row 9). ENUM-10 in row 4a (Q-0407, no test names it) |
| P23b | Expressions checked against requirements | partial | partial | F4 answered I6-1, I6-5, I6-9 and I6-10. `TEMPLATE-MODEL.md` section 1 describes a typed dependent tree with marks, the form in which a matching rule can be stated | No matching rule (X22-8, `out of scope`). Q-0411 (review, doubt). Q-0412. Q-0426. The CNS-13 strength of section 1.3 |
| P28 | Arithmetic that wraps on request | partial | partial | F4 added a wrapping hash in `unsigned` to the multiplicative page (P2-4) | The marker, and whether it applies to a type, an expression or an operator |
| P8b | Relocation by bytes | partial | partial | | Row 2 (Q-0322). Row 6 (Q-0343, CPY-19) |

Totals in the file: 4 ready (P22, P24, P26, P27), 2 partial (P23b, P28), 2 done from P22 on (P23a, P25), 0 blocked. P26 is still the one ready proposal that changes existing behavior.

P2-3 and P3-4 make a further entry. Two port suites wrote `==` on two `const char*` and expected the characters to be compared. PRIM-9 makes the result for two equal literals unspecified, and CMP-15 compares addresses. A designer would write P29, "a pointer comparison with a string literal operand is not valid", my own invention. Other languages compare strings by content or warn on such a comparison (general knowledge). P29 changes existing behavior. My scan finds 18 lines in 6 program files that compare with a string literal, and by my reading each compares a class object (a string or key class with its own `operator==`), so no program becomes invalid. The scan is text, so the count needs the syntax query of item 4. P29 meets PRIM-9, CMP-15, Q-0033 and the PRIM-9 example that F4 added for P2-3. It leaves open whether a literal on both sides counts, whether `nullptr` against a literal counts, and whether relational operators are included. Rating: ready, by my reading.

### 1.8 Questions and principles

Phase 14 opened 6 questions (Q-0429 to Q-0434) and settled 1 (Q-0422, on JMP-P3). Four open questions now wait for a principle: Q-0424 for a use of UNI-P3 (and a review), Q-0427 for a use of DEP-P5, Q-0432 for a use of DEP-P2, and Q-0429 for a new principle DED-P2. Q-0196 still waits for TPL-P3, a principle that does not exist. The share line is 62% for rows 1 to 9 and 70% with row 10.

Principles that a Propose finding met, now:

| Principle | Finding | State |
|---|---|---|
| ATTR-P1, ATTR-P2, ATTR-P4 | X14-7, X22-6 | Q-0412, row 9. Bears on P22, P23a, P23b and P24 |
| DEP-P6 | X22-4 | the CNS-13 row now says "need not diagnose; every baseline compiler rejects" (RV7-B-2). X22's own row for a dependent template-id says "rejected" |
| CNS-P3 | X22 (its row of uses) | the independent template-id row now adds "no diagnostic required if the template is never instantiated" |
| DEP-P4 | X22 (the exemption of the current instantiation) | row 4a rule item with DEP-15 |
| NEW-P5 | X23-1 | row 4a rule item |
| TR-P1 | X21-4 | unchanged |
| STG-P2 | W20-1, W20-6 (P27's input) | now states the typed step over bytes (Q-0430), which RV7-A-10 reads against |
| LOOP-P1 | P22's model | gained a diagram for a jump into the body (G161) |

### 1.9 The 28 open ledger rows, ranked by design reach

The exercise ledger has 26 `open` rows (I7-1 to I7-3, I7-5 to I7-15, P3-1 to P3-12), and the front-end ledger has 2 (G164, G165).

| Rank | Rows | What they reach | Who acts |
|---|---|---|---|
| 1 | G164 | STG-28 for a `T*` from a byte offset. P27's motivation, X9's STG-28 row, Q-0430 | writer |
| 2 | G165 | VIRT-13 when one overload is overridden and another is hidden. X18's VIRT-13 row (hiding without overriding stays a warning in a checked class) and X9's third warning | writer, with probes |
| 3 | I7-10, I7-5 | CTAD-P1 and OVL-27 for a guide with no template head; whether implicit constructors give guides (CTAD-5, CTAD-6). X7's rows on CTAD-22 and CTAD-5 | writer |
| 4 | P3-6 | INST-1 and INST-5 on when a member's return type is made. X18's INST-5 row. P3-7 and P3-8 have the same cause in the front end | writer, with probes |
| 5 | P3-4 | `==` on two `const char*` (PRIM-9). With P2-3, the motivation of P29 | writer |
| 6 | I7-7, I7-8 | CTAD-14 for an alias, and an alias of an alias. X20 relies on the alias rows | writer |
| 7 | I7-3, I7-9 | CTAD-16 with DER-8. No design reaches them | writer |
| 8 | I7-1, I7-2, I7-6, I7-11 to I7-14 | CTAD-10, CTAD-17, CTAD-18, CTAD-25 and CTAD-26 text. No design reaches them | writer |
| 9 | P3-1, P3-2, P3-3, P3-5, P3-11, P3-12, I7-15 | Examples, glossary entries and outside knowledge. P3-1 asks for a lazy generator, which coroutines would give and which is out of scope | writer |
| 10 | P3-7 to P3-10 | Front-end faults with tests | front end |

### 1.10 The distance to decision 0060's completion test

The completion line at the close of phase 14 reads "107 tests fail their check, 3/138 program tests do not run to their result, 9 tests out of scope". Front-end round 25 ended at 104, and P3 added 3 tests and 2 program tests that do not pass. By status, round 25's 104 are 45 missed rejects, 50 unsupported, 4 false rejects and 5 `EDG-AGREES`. By chapter, 60 are on template pages, 12 on class pages, 10 on expression pages, 8 on declaration pages, 8 on overloading pages, 4 on foundation pages, 1 on a statement page, and 1 is a program test. `templates/dependent-names` has the most (12). The three program tests are `r10-tuple-second-try`, `pipeline-first-try` and `events-first-try`.

The count went 164, 127, 104 over rounds 23 to 25, and no test moved from a passing status to a failing one in rounds 24 and 25. Report 25 marks 30 of the 104 as needing the template model, by section of `TEMPLATE-MODEL.md`: 10 for section 3 (a specialization as a substitution), 7 for section 5 (checks at the definition), 5 for section 2, 4 for section 4, and 4 for section 1 alone or with section 8. The other 74 fit the current design. If the next round took all 74 and the 3 P3 tests, about 30 would remain, inside the gate of 50.

For a designer, the rewrite's design document still matters more than the count. `DESIGN-current.md` shows that a page's rules sit in 6 files at the median, and that `expr.rs` names the rules of 65 pages. A prototype of a proposal in the current front end would touch most of those files. `TEMPLATE-MODEL.md` section 5 lists the checks at the definition as a table, and X22's checked marker would add one row to that table. Neither document says how a prototype attaches. `TEMPLATE-MODEL.md` section 8 still lists I6-4, I6-5, I6-6 and I6-9 as findings that can change the model, and F4 answered all four. Section 6 calls the cause of the `r10-list-first-try` stop open, and report 25 found it.

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | A short portfolio step (P26's fold count, P22's G161, P23b's rating, P29), then Propose X25 on P27 | 1.3, 1.6, 1.7 | 1 short designer step, 1 reader session, 1 repair session | low | the first design in storage and unions since X14; a defined form for the case that Q-0430 and G164 dispute |
| 2 | Propose X24 on P26, then the stop-rule count | 1.5, 1.7 | 1 reader session, 1 repair session | low for similarity, medium for the impact count | the second reading of `comma.md`; the stop rule's first full count |
| 3 | Refresh notes for X22, X8, X7, X6, X5, X15, X14 and X9, and a refresh check that names the commit that changed each row | 1.1: 155 rows; 1.3: 27 rows in 12 reports | 1 reader session, a small tool change | low | reuse of the done designs; item 7 |
| 4 | A shared impact scanner with a syntax query, and the order count for X1 from `tidal-fe run` | 1.2, 1.6, 1.7 | 1 front-end or writer session | low | section 4.4 of X24 and P29; section 4.3 of X1 with a measured count |
| 5 | The packet rows that reach proposals: row 4b's Q-0426, row 4a's Q-0430 with ADD-14 and ADD-P6, row 9's Q-0412, and row 7 with its count from a tool | section 5 | maintainer decisions; 1 writer step for the count | low | X22's marker, X9's relocation constructor, P27, P22 and P24 |
| 6 | The open rows with design reach: G164, G165, I7-10, I7-5, P3-6, then I7-7 and I7-8 | 1.9 | 1 writer session with probes | low | X25's base; the rows of X18, X7, X9 and X20 |
| 7 | Pair exercise X14 with X23 | 1.3, 1.4 | 1 reader session after item 3 | low | an interaction section that can be checked |
| 8 | A prototype hook in the rewrite's design document, and the rewrite documents brought to the present | 1.10 | a writer step for the hook; a line in each front-end round | medium | the first prototype of a proposal |
| 9 | An index of what a test of validity can see | 1.1: 49 of 559 | 2 writer sessions | medium | section 4.3 of every proposal |

### Item 1. The portfolio step, then X25 on P27

**What it is.** Two steps.
- **The portfolio.** P26's entry gives 7 files with a fold over the comma at 138 programs, and names them. P22's entry drops G161 and cites LOOP-9 as the model of a labeled `continue`. P23b's entry lists F4's answers and the blockers that remain (X22-8, Q-0411, Q-0412, Q-0426, the CNS-13 strength). P27's entry adds Q-0430, G164, ADD-14 and STG-29 to its cites. P29 gets an entry from section 1.7. The scan scripts go beside the file, so each count has its source.
- **X25 on P27.** The design states how an element's lifetime starts and ends, what `std::construct_at` and placement `new` do on one element, what a pointer and a length over the started elements give, and what constant evaluation allows. It meets STG-P2, STG-28, STG-29, ADD-10, ADD-14, UNI-13, UNI-14, UNI-P3, LSUP-10 and LSUP-11, and Q-0425 and Q-0430. P27 adds a form and changes no existing behavior, so section 4.4 should find no program that becomes invalid. Section 4.3 should count the programs that use a byte row per slot (`FixedVec` and its kin), since they are the users the design gives a defined form.

**Evidence.** Sections 1.3, 1.6 and 1.7. Phase 14 changed six rules and principles in P27's area (STG-28, STG-29, STG-P2, ADD-14, UNI-14, UNI-P3), and two reviews read the typed step over bytes against the text. No Propose has run for three phases.

**Cost.** One short designer step, one reader session, one repair session.

**Risk.** Low. G164 is open. If item 6 runs first, X25 reads the answer. If not, X25 records G164 as a finding.

**What it unblocks.** A ready design that changes no existing behavior, and an outside view on Q-0430 from a user of the storage.

### Item 2. X24 on P26, then the stop-rule count

Round 11 item 1, unchanged in scope. The design states what a left operand may be, whether a call of an operator function or through a pointer counts, and whether a fold is checked for each element or once. Section 4.4 counts at least `r5-hashmap-kept-pointer.cpp` and `r7-registry`, with item 4's scanner if it exists. P26 meets COMMA-P1, COMMA-5, OPR-7, OPR-P2 and PRIM-27. After X24, count X22 to X24's findings and the share that the text closed. X22 closed 7 of 9 (X22-8 is `out of scope`, X22-6 a question), and X23 closed 7 of 7.

If phase 15 has room for one Propose only, X25 goes first. P27's inputs changed in phase 14, and P26's did not, so P26 does not age while it waits.

### Item 3. Refresh notes and a refresh check

Round 10 item 2 and round 11 item 3, with phase 14's rows.
- X22: CNS-13 and DEP-P6 against X22:new-3 (section 1.3). CNS-11's skip and Q-0426. The DEP-15 and DEP-P4 item.
- X8: DEP-2's exception for a member access through `this`, and when the fallback for `this->f()` is chosen.
- X7: DEP-4's exception for `sizeof`, `alignof` and `noexcept` against a value-specialized parameter. DED-30 from round 11.
- X6 and X5: the place of their DEP-2 items beside the new exception.
- X15: DEP-9's several points and INST-32.
- X14: REF-2's list of three direct cases. REF-1 from round 11.
- X9: STG-28 and G164. CPY-15 and Q-0426 from round 11.
- `tools/propose_check.py` reads a note beside a report and counts the rows the note covers as "noted". The tool names the latest commit that changed each row. The tool uses the commit that added a report when the report names none, so X1 to X12 get a changed-row count. My `changed_since.py` does the second part in 23 lines.

### Item 4. A shared impact scanner and the order count

Round 11 item 6. The order count can run now. A run of `tidal-fe run` over the 138 programs that lists each program whose first line does not say that every explored order agrees is a measured lower bound for X1's section 4.3. A scanner under `tools/` takes the text queries of X21 and X22, the injected-header method of X23, and a syntax query that the front end answers. P26 needs "comma operator, left operand not a call and not of type `void`". P29 needs "equality operator, one operand a string literal, the other a pointer". Each query honors `tidal-flags` and reports `tests/programs/` apart from the rule tests.

### Item 5. The packet rows that reach proposals

Section 5 gives the rows. The writer step is to make the row 7 plan's count come from a tool (2808 `reject` tests, about 8424 compilations), so that the count stops going stale. The request on the record asks for rows 1, 4a and 10. Under "follow the compilers", that request settles Q-0425 (row 1) and Q-0430 (row 4a), the two questions of P27, with no test changed. A second request can name row 4b's Q-0426 and row 9's Q-0412.

### Item 6. The open rows with design reach

A writer answers, with probes on the matrix:
- G164: say in STG-28 which element a `T*` cast from a byte pointer at an element's offset reaches.
- G165: say whether VIRT-13's warning concerns each hidden base function or each derived function.
- I7-10 and I7-5: say whether a user guide with no template head is a function for OVL-27, and whether implicitly declared constructors give guides.
- P3-6: say in INST-5 when the parameter and return types of a member function definition are made.
- I7-7 and I7-8: a worked example of CTAD-14, and the rule for an alias of an alias.

The rest of the 28 rows follow in the order of section 1.9.

### Item 7. Pair exercise X14 with X23

Round 10 item 7, unchanged in scope. The pair asks what `new` does for a packed class whose members are under-aligned, and how a reference binds to such a member under REF-1 and the new REF-2. The pair runs after item 3, so that both designs carry their refresh notes.

### Item 8. A prototype hook, and the rewrite documents in the present

When the gate holds, the next phase decision adds the rewrite, and decision 0060 asks for its own design document. That document names how a proposal prototype attaches to a page module behind a flag. `TEMPLATE-MODEL.md` section 5 shows the form for templates: a prototype of X22 adds a row to the table of checks at the definition. Until then, `TEMPLATE-MODEL.md` sections 6 and 8 and `DESIGN-current.md` state the present. Each front-end round updates them or records that they still hold.

### Item 9. An index of what a test of validity can see

Round 10 item 9, unchanged. 49 of 559 `reject` rules have an `accept` or `run` test that uses `requires`.

## 3. Keep out of scope

- **Coroutines, modules, threads, the memory model and the general library.** No proposal needs them. P3-1 asks for a lazy generator, and an example of an iterator over a computed sequence answers the reader.
- **P23b's exercise in the next phase.** P23b needs a matching rule, Q-0411, Q-0412, Q-0426 and X22's CNS-13 strength.
- **P28's exercise** until its entry chooses where the marker attaches.
- **P8b** until the maintainer decides rows 2 and 6.
- **Rules or `:since:` fields for later versions.** Row 7's probe makes an index only.
- **A source-reader designer** (Q-0116). Twenty-three exercises have not needed one.
- **A cost model with numbers** (Q-0183 option 2).
- **Full re-runs of old Propose reports.** Item 3's notes cover the changes that reach a design.
- **ILP32 runs as their own item.** The 76 tests with `-m32` flags did not change in number, and no proposal needs more.
- **A prototype in today's front end.** A page's rules sit in 6 files at the median, and the code would be removed with the old front end.

## 4. Process changes

1. **A repair names the done designs its text change reaches.** Rounds 8 to 11 process change, still not taken. Phase 14 changed 27 rows in 12 reports, and no commit names them. Item 3's tool change can print the list for the commit message.
2. **A count in the portfolio comes with its script.** P26's entry carries a fold count that my round 11 prose gave and my round 11 scan output does not. The scan scripts and their output go beside the portfolio.
3. **Keep the exercise records current.** Section 4 of `exercises.md` names `reports/clean-room/roadmap-7/portfolio.md` as the portfolio, its prose names the latest round, and section 8 lists the Propose rows in order (X21, X22, X23). Rounds 10 and 11 process change, not taken.
4. **The rewrite documents state the present.** `TEMPLATE-MODEL.md` sections 6 and 8 name causes and findings that phase 14 closed. Each front-end round that changes the facts in them updates them.
5. **A phase with an exercise item runs a Propose until the stop rule has its count.** Phases 12 to 14 ran none.
6. **A `required` rule that states a rejection points to a `reject` rule.** 75 `required` rules say "not valid". LOOP-9 is the new one, and it points to JMP-6 in the same paragraph. A check could list the rules that cite no `reject` rule there.
7. **The stop rule stays.** The rule counts X22 to X24.
8. **Keep the designer's view in round 13** in this form. Rerun the counts of sections 1.1, 1.3, 1.5 and 1.9, and the diff of changed rows against each done design.

## 5. The decision packet from a designer's view

The packet in `documentation/decisions/pending.md` gives each row's questions and an upper bound on the tests whose expectation changes. The order below follows the number of done designs and proposals that each row reaches.

| Order | Row | What it settles | Tests whose expectation changes | Proposals and done designs it reaches |
|---|---|---|---|---|
| 1 | 4b, Q-0426 | whether a constraint check runs when a candidate that is not a template matches exactly (CNS-11, CPY-15) | 0 if the compilers; at most 4 if the review (4 tests name Q-0426) | X22 and P23b (the checked marker), X9 (the relocation constructor), and every design that adds a constraint |
| 2 | 4a, Q-0430, and the rule item ADD-14 with ADD-P6 | whether a typed step over bytes, and placement `new` at its position, is defined (ADD-14, STG-28, STG-29) | 0 if the compilers; the row's bound is 34 tests for its questions and 195 for its rule items | P27 (X25), X9's STG-28 row, and X14 through STG-29 and STG-P2 |
| 3 | 9, Q-0412 | which effects a standard attribute may have | 0 under either option (no test names Q-0412) | P22, P23a (X22), P23b and P24 choose a marker by Q-0412. X23 cites the question. Under the kept answer, X22's `[[checked]]` adds required rejections and must become a keyword |
| 4 | 4a, rule items DEP-15 with DEP-P4, and NEW-P5 | whether a static constant member equal to the parameter names the current instantiation; whether `new` adds an alignment argument on a second search | 0 if the compilers | X22's exemption of the current instantiation; P25's mirror case (NEW-30) |
| 5 | 4a, Q-0407 | ENUM-10's type for an enumerator from another enumeration | 0 (no test names it) | P24, X20 |
| 6 | 7 | Q-0007, Q-0087, Q-0116, Q-0181, Q-0188 | 5 tests name them; the probe changes no expectation | the version line of 17 Propose reports, and of P22, P24, P26, P27 and P29 |
| 7 | 1 | 55 questions, among them Q-0425 (UNI-14), Q-0420 (DED-29) and Q-0421 (LSUP-11) | 0 (131 tests name them) | P27 through Q-0425 and LSUP-11; P23a and P23b through DED-29 |
| 8 | 9, Q-0183 and Q-0186 | a cost model; judgments from outside Tidal | 0 | section 4 and the outside-knowledge line of every Propose report |
| 9 | 2 and 6 | Q-0322; Q-0343 and Q-0041 | 0 for row 2; at most 5 for row 6 | P8b (relocation by bytes cites CPY-19's trivially copyable list) |
| 10 | 10 | 17 questions that Tidal answers as its own choice | 0 (31 tests name them) | Q-0020 (how tests record a compiler that lacks a feature), which X4 and X6 cite |

The request on the record asks for rows 1, 4a and 10. Row 4a now holds Q-0430, so the request reaches P27 twice (Q-0425 in row 1, Q-0430 in row 4a), and X22 and P25 through the DEP-15 and DEP-P4 item and NEW-P5. Row 10 reaches X4 and X6 through Q-0020. Rows 4b and 9 reach the most designs and are not in the request. I found no proposal that rows 3, 5 and 8 reach.

## Host-only claims

None. I ran no compiler and no front end. Every behavior in this report comes from Tidal's files.
