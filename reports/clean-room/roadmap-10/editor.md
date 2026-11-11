# Roadmap, round 10: what to do after phase 12, from an editor's view

Date: 2026-11-06. Decision 0029. This report is by a clean-room reader acting as an editor, who is responsible for the specification's consistency, organization, and records. The reader read AGENTS.md, CLAUDE.md, and its own round 9 report with the scripts beside it. In `documentation/` it read decisions 0059 and 0060, `decisions/pending.md` with its packet, the status page, the roadmap's phase 12 part, and sections 3, 4, and 8 of `exercises.md` where phase 12 changed them. It read `git log a15a877d..ebc109b9` with every commit body (20 commits) and the `--stat` of each. In `reports/clean-room/` it read `ledger.md`, the W20 and I5 reports, `rust-frontend/report-23.md`, and `rust-frontend/ledger.md`. In `reports/review/` it read the two phase 12 gap reviews (`2026-11-05-gap-review-phase-12-*`), and in `reports/similarity/` the summary line of the 16 phase 12 reports. It read the question files that the counts below name, and the parts of `specification/` and the test headers that its scripts read. It did not read the other round 10 reports, `inputs/`, `build/exercise-keys/`, or `frontend/`. It did not run `tools/run_tests.py` or `tools/frontend_check.py`, so the front-end counts come from the round 23 report, `conformance.tsv`, and the close commit. Read-only scripts made the counts from copies of the trees at a15a877d (round 9) and ebc109b9, made with `git archive`, with one method for both. Commit read: `ebc109b9`.

The page table uses the round 9 columns:

- **R/T**: live rules and the distinct tests they cite.
- **Q**: open questions whose `Affects:` line names a rule or principle on the page.
- **F**: lines of the `.md` files in `reports/clean-room/` that cite a rule or principle on the page, leaving out the roadmap reports.
- **X**: cited tests with a per-compiler override.
- **Ev**: lines of the Evidence segment against the page's lines.
- **R/P**: live rules per principle.
- **FE**: tests on the page that fail on the front end at round 23 (report 23, section 6).

## 1. The state from the editor's view

### Repo-wide counts

| Measure | Round 9 (a15a877d) | Now (ebc109b9) |
|---|---|---|
| Live rules | 1682, plus 15 withdrawn | 1683, plus 15 withdrawn. TR-17 is new (`reject`). No rule changed class |
| Principles | 343 | 343 |
| Rule and principle text changed in the phase | 17 rules and 9 principles | 32 rules and 4 principles, plus the new TR-17, leaving out `:tests:`, `:rules:`, and `:exceptions:` lines. Foundations hold 10 of the 36, expressions 9, declarations 5, overloading 5 |
| Test files | 6711. 6608 cited by a rule, 103 program tests | 6790. 6675 cited by a rule, 115 program tests. 79 new, none removed, no expectation of an old test changed |
| Rules that rest on one test | 0 | 0 |
| Rules that rest only on tests with an override | 28, as warnings | The check now accepts a recorded reason for each (item QR), and warns for none |
| `check_specification.py` | 1697 rules, 0 problems, 28 warnings | 1698 rules, 0 problems, 0 warnings |
| Tests with an override | 688 | 699 |
| Overrides by label | EDG 285, GCC 258, Clang 132, Clang 18 76, GCC 14 57, other 80 | EDG 291, GCC 267, Clang 132, Clang 18 76, GCC 14 57, other 80 |
| Tests with an override for exactly one newest family | 510 (Clang 107, GCC 195, EDG 208) | 517 (Clang 107, GCC 200, EDG 210) |
| Tests with an override for all three newest families | 1 (`replace-base-old-pointer-constant.cpp`, Q-0307) | 1, the same test |
| Tests with `tidal-error` patterns (this method) | 3066. The status page gave 3070 | 3090. The status page gives 3094 |
| Questions | 418 | 422 |
| Open questions (all provisional) | 174 | 176 |
| Settled, compiler defect | 231, 13 | 233, 13 |
| Share that `pending.md` holds (the tool's method) | Rows 1 to 9: 111 of 174 (64%). With row 10: 124 (71%) | Rows 1 to 9: 111 of 176 (63%). With row 10: 124 (70%) |
| `pending.md` rows decided by the maintainer | none of 10 | none of 10 |
| Front end (close commit) | 6478 of 6711 pass, 6520 pass their check | 6586 of 6790 pass, 6626 pass their check. `conformance.tsv` (round 23) gives 6582 of 6786 and 6622 |
| Front end on page examples | 175 of 177 | 177 of 179 |
| Evidence lines, whole specification | 7805 of 43780 (17.8%) | 7874 of 44046 (17.9%) |
| `tests/RESULTS.md` | 6711 rows | 6790 rows |
| `question_index.py --check` | 418 questions, 0 problems, 5 warnings | 422 questions, 0 problems, 0 warnings |
| Divergences tags that disagree with the question file | 0 | 0 |
| Exercise ledger | 867 rows: 762 commit, 68 open, 26 planned, 5 text, 3 question, 3 out of scope | 925 rows: 878 commit, 17 open, 0 planned, 17 text, 10 question, 3 out of scope |
| Front-end ledger | 152 rows, 0 open | 157 rows, 5 open (G153 to G157) |

The pages grew by 266 lines, and Evidence grew by 69 of them. Phase 12 changed twice as many rules as phase 11. Most of the change came from F2, RD, and RV5's repairs.

### The read-only checks

- `tools/check_specification.py`: "1698 rules, 0 problems", exit 0, no warnings.
- `tools/question_index.py --check`: "422 questions, 0 problems", exit 0, no warnings. QR removed the five family warnings of round 9.
- `tools/question_index.py --share`: "Rows 1 to 9 hold 111 of the 176 open questions (63%). With row 10, the rows hold 124 (70%)." The line matches the packet and the close commit.
- `tools/checks.py --no-frontend`, with no reference corpus: similarity did not run, the other two lines are as above, exit 0. The script has no read-only mode beyond leaving out the front end and the corpus.
- A replay of `check_specification.py` and `question_index.py --check` on a `git archive` copy of each of the 20 phase 12 commits gives 0 problems from `question_index.py` at every commit. `check_specification.py` gives one problem at every commit, and the same problem on the copy of ebc109b9: the status page's similarity column is stale. `status_table.py` orders similarity reports by the time git first recorded them, and a copy has no git history. The live tree at ebc109b9 gives 0 problems. The replay therefore finds no commit whose checks failed, apart from that one check that a copy cannot run.
- The 16 phase 12 similarity reports (`2026-11-05-phase-12-t`, `-f2-cond`, `-f2-ics`, `-f2-x23-p1`, `-rd`, `-qr`, `-r6`, `-l2`, `-pk3`, `-rv5`, `-rv5r`, `-rv5-ledger`, and `2026-11-06-front-end-23`, `-phase-12-w20`, `-phase-12-i5`, `-phase-12-close`) each report "0 flagged".

### What phase 12 did to the questions

Each count comes from each file's status and `**Waits for:**` line at a15a877d and at ebc109b9.

- 4 new questions. Q-0419 (ICS-16, GCC calls the list calls ambiguous) waits for a review (doubt). Q-0420 (DED-29, the address case) waits for newest families that agree. Q-0421 (LSUP-11, assignment before construction) waits for newest families and a review (doubt). Q-0422 (JMP-10, a structured binding in a return) waits for a first review.
- 2 settled, both by item QR under decision 0054 (Q-0323, Q-0383). RV5 settled none. No question reopened.
- Open questions went from 174 to 176.
- 156 of the 172 questions that were open at both ends did not change at all in phase 12, `**Waits for:**` line included.

The `**Waits for:**` items, at the start and the end of the phase (a question can hold more than one item):

| Item | Start | End |
|---|---|---|
| the maintainer (`pending.md` rows 2 to 9) | 62 | 63 |
| newest families that agree (row 1's group) | 56 | 54 |
| a review (doubt) | 48 | 50 |
| a review (silent) | 15 | 15 |
| a review (against) | 9 | 9 |
| a review (part) | 1 | 1 |
| a first review | 2 | 1 (Q-0422) |
| a principle, a rule, child questions | 1 each | 1 each |

These match the close commit, which gives every line but the maintainer's. When each question goes in the first group that fits (maintainer, then row 1's group, then first review, against, silent, doubt, part), the 176 split this way:

| What holds the question | Round 9 | Now |
|---|---|---|
| A row of `pending.md` (rows 2 to 9) | 62 | 63 |
| Two newest families differ (row 1's group) | 49 | 48 |
| A review that reads with doubt | 38 | 41 |
| A review that finds the reference silent (packet row 10) | 13 | 13 |
| A review that reads against the answer, outside row 4 | 7 | 7, the same seven (Q-0063, Q-0083, Q-0273, Q-0328, Q-0344, Q-0357, Q-0364) |
| A first review | 2 | 1 |
| A principle (Q-0196), child questions (Q-0294), part of a review | 1 each | 1 each |

The doubt group grew by Q-0274 (QR dropped its family item), Q-0417, and Q-0419. 63 of the 75 review-held questions did not change in phase 12. Of the 41 held by doubt alone, 20 were last read on 2026-11-03, 3 on 2026-11-04, and 18 on 2026-11-05.

### Text phase 12 wrote that no review has read

RV5's two reviews read the 40 rules and principles that changed after RV4 read the tree, with the text of F2, RD, and QR. The reviews name every one of the 40. They gave 33 points (RV5-A-1 to RV5-A-18, RV5-B-1 to RV5-B-15). The repair commit 3bed9cb5 then changed 20 rules and principles, and no later commit changed rule text. The close commit lists the same 20. Each row gives the words that 3bed9cb5 added and removed, and the other phase 12 commits that changed the same block:

| Block | Page | Repair | Other changes |
|---|---|---|---|
| ALN-4 | declarations/alignment | +19 -5 | phase 11 (ee6085aa), 0f3f522f, 6c6f69f8 |
| REF-1 | declarations/initialization/references | +57 -3 | 3b83788d. RV5 read it against. G153 and I5-10 name it after the repair |
| RET-6 | declarations/functions/return-deduction | +62 -24 | 6c6f69f8. Q-0422 |
| JMP-10 | statements/jumps | +59 -2 | 6c6f69f8. Q-0422 |
| ICS-11 | overloading/conversion-sequences | +44 -28 | 3b83788d. RV5 read it against |
| TR-P1 | foundations/translation | +46 -10 | 6c6f69f8 |
| CONV-1 | foundations/conversions | +36 -2 | 03e578c4 |
| STG-P3 | foundations/storage | +29 -8 | 0f3f522f |
| UNW-3 | exceptions/unwinding | +10 | 0f3f522f |
| SINIT-6 | declarations/initialization/static | +13 -2 | 3b83788d |
| AUTO-P3 | declarations/auto | +13 -9 | 6c6f69f8 |
| STG-5 | foundations/storage | +39 | phase 11 (ee6085aa) |
| CLS-26 | classes/members | +35 | phase 11 (7cf4574b) |
| NAM-21 | foundations/names | +13 -1 | phase 11 (7cf4574b). Q-0415 |
| ORD-13 | templates/ordering | +5 -5 | phase 11 (7cf4574b) |
| SW-P1 | statements/switch | +27 -4 | none |
| LAM-5, LAM-6, LAM-7 | expressions/lambdas | +28, +26 -3, +1 -2 | none |
| BF-6 | classes/unions | +17 -1 | none |

Q-0422 rests on this text and has no review. Five points kept the compilers' answer with an Evidence note, and their ledger rows say `commit`: COND-16 and COND-3 (B-2, B-3), NEW-22 (B-5), ICS-17 (B-7), and TR-17 (A-14). Two more (NEW-P5, B-4; ICS-45, B-9) joined the row 4 table of `pending.md` as items with no question.

### What phase 12 took from round 9

| Round 9 item | Outcome |
|---|---|
| 1. Review the text phase 11 wrote after RV4 (RV5), then the repairs | Done. Two reviews read 40 rules and principles, gave a first reading of Q-0415 and Q-0417 to Q-0421, and read the RV4 doubts on LAM-6, LAM-29, and SW-P1 again. No question settled. The repairs changed 20 rules and principles, which no review has read |
| 2. Answer the 20 planned rows of ended items | Done. F2 and RD answered all 20, each with a commit, and Q-0419 and Q-0420 hold the two splits. No row was deferred again |
| 3. Put the packet to the maintainer, with the tests each option changes | Done in part. The packet gives 0 for each option whose answers stay and "at most N" for the others, and says the bound counts tests that name the questions. The share line comes from `question_index.py --share`. The roadmap marks "the request to the maintainer" done, and no file or commit records a request or an answer. No row was decided. The seven "against" questions outside row 4 stayed outside |
| 4. Read the open ledger rows, and tighten the ledger check | Done. L2 read the 68 rows (60 to commit, 6 to text, 2 to question), filled the 7 commit rows, and added the commit and planned checks. RV4's doubts got rows RV4-1 to RV4-7 |
| 5. Repair the records that drift (R6) | Done. The status page's front-end sentence names round 23 and `conformance.tsv`. Q-0299's Opinions are in date order. The stacked "Kept open" lines are gone (no open question has three). QR answered the five family warnings |
| 6. A narrow second review of the doubt-held questions | Not done. Decision 0059 defers it again. 41 questions are held by doubt alone, against 38 |
| Process: a planned row names a live item | Adopted, and a check enforces it |
| Process: a row's state comes from a reading | Done once, by L2 |
| Process: review findings get IDs and ledger rows | Adopted. 40 rows (RV4-1 to RV4-7, 33 RV5 rows) |
| Process: the share comes from a tool | Adopted |
| Process: the packet gives each option's effect, or says what it counts | Adopted, as an upper bound |
| Process: a commit follows a passing chain | Adopted. Every phase 12 Checks paragraph reports passing checks |
| Process: the review of the previous phase's text reads the last repairs | Not adopted. RV5 ran in the middle of phase 12, as RV4 did in phase 11, and 20 blocks changed after it |

### Decision 0059's standing choices in phase 12

**A `planned` row names a live item.** Held. The 26 planned rows of phase 11 moved to F2, T, and FE in c9aa15bd. RV4-1 to RV4-3 were planned for RV5 and answered in f7641eb3. P1-1, P1-2, and I4-4 to I4-7 named FE for one commit after FE ended (1017b19c), and 5f572a3a moved them. The close has no planned row. W20 and I5 rows went in as `open` "for the next phase's decision", which the choice allows. The new check reads only Records of the form "decision NNNN, item X".

**A front-end finding gets a test in the commit that records it.** Held for T and FE. Not held for item E:

- I5-9 (the front end stops on `T::type` with an incomplete `T`) and I5-10 (the front end binds a reference to a private or ambiguous base) are front-end faults. Each has a reduction in `reports/clean-room/I5/reductions/`, and the report asks for each as a test. 2bb7d26c adds no test.
- W20-3 (the front end rejects with CNS-35 a constraint that GCC and Clang accept) has a reduction in `reports/clean-room/W20/reductions/`. W20-2's subscript of an array member of a union stops as unsupported. 1056cca2 adds only the four program tests, and the front end runs those.
- G154 and G157 ask for tests. 1017b19c adds none. G153, G155, and G156 cite tests that exist.

**A front-end round runs in each phase that changes rules.** Held. Round 23 ran after the last rule change of the phase, and all 105 tests added since round 22 pass, the 13 RV5 repair tests among them.

**Review points get IDs and ledger rows.** Held. Every RV5 point has a row: 24 name 3bed9cb5, 5 name a question (Q-0416 to Q-0419, Q-0421), and 4 are `text`. The choice says that a point the repair leaves gets a `planned` row. No point got one. The five points that keep the compilers' answer against the review's reading (B-2, B-3, B-5, B-7, A-14) are `commit` rows that name an Evidence note, and B-4 and B-9 are `commit` rows that name a row 4 item with no question. The ledger check reads only `reports/clean-room/`, so a review point with no row would not fail it. The decision and the ledger header write the ID form as `RV5-A3`, and the reviews used `RV5-A-3`.

**A commit follows a passing chain.** Held, by every Checks paragraph and by the replay above. 3bed9cb5 added 13 tests and ran the chain without the front end, and says so. Four commits keep their similarity output under `build/`, which git ignores, so no tracked file records the run: f4caf402, c9aa15bd, e7bcbb2a, and 5f572a3a.

**The share comes from a tool.** Held for the line. The comparison is off. The close commit says "At the start of the phase, PK3's first count gave 63% and 70% for 175 open questions, so the share held." PK3 (df4b5539) ran after F2, RD, and QR had changed the open count. The phase started with 174 open questions, and the tool's method gives 111 (64%) and 124 (71%) there. The counts held and the share fell one point by rounding. Phase 11's closing line ("The rows 1 to 9 hold 111 of the open questions") had no total, so no tool line from the previous close existed to copy.

### Phase 12 commit messages against their diffs

The reader compared each of the 20 commit bodies with its `--stat`, the ledger at each commit, and the rule blocks at each commit. These agree with the diffs:

- The test counts: 87209705 adds 33 test files (25 tests, 8 program tests), 03e578c4 12, 3b83788d 9, 0f3f522f 1, 6c6f69f8 5 tests in 6 files with one test changed, da51ae9e 2, 3bed9cb5 13, 1056cca2 4. The total is 79, as the close says.
- The rule changes that each message lists, against the blocks that each commit changed: 03e578c4 (6), 3b83788d (7), 0f3f522f (7), 6c6f69f8 (8, with TR-17 new), da51ae9e (LSUP-11), 3bed9cb5 (20). The close's list of 20 is the 3bed9cb5 set.
- The ledger moves: 3b83788d 7, 0f3f522f 5, 6c6f69f8 8, 13c03237 7 new rows (4 commit, 3 planned), f7641eb3 33 new rows and 3 moves, 5f572a3a 6, 1056cca2 7 new rows, 2bb7d26c 11 new rows. f7641eb3's split of the RV5 rows (24, 5, 4) is right.
- The open-question counts in each message, the four new questions, the two settlements, and the close's `**Waits for:**` changes.
- 1017b19c's counts against `conformance.tsv` (6582 full passes: 6284 pass, 274 EDG-ONLY, 10 EDG-ALLOWED, 14 pass-partial).

Three later commits record corrections, and each is right:

- df4b5539 corrects f10c661a: 60 rows moved to commit and 6 to text, against the 59 and 7 that f10c661a gives.
- f7641eb3 corrects 3bed9cb5: 13 tests, against "Tests (14)".
- ebc109b9 corrects 2bb7d26c: of I5's 11 wrong assertions, 6 come from the component's interface (the two empty-list tests and four specialization tests). The I5 report's list supports the correction.

These statements misstate something or leave something out, and no commit corrects them:

- ebc109b9's share comparison (above).
- ebc109b9 says "No ledger row is planned. 17 rows are open", and the status page's next step names "the 17 open ledger rows of W20 and I5". The front-end ledger has 5 more open rows, G153 to G157, from 1017b19c.
- 3bed9cb5 says that NEW-P5 and ICS-45 each "join pending.md row 4". Each joined the row 4 table with no question, so the packet's row 4 counts and the share do not include them. Row 4's count rose from 36 to 37 by Q-0418 alone.

### Records that drift

These records disagree with the tree at ebc109b9, or with each other:

- **The close's share comparison** names a mid-phase count as the start of the phase.
- **The open front-end rows** (G153 to G157) are missing from the close and the status page's next step.
- **Row 4 items with no question.** 14 items of the row 4 table name no question: THR-3, TRY-2, UNW-3, NAM-40 with NAM-41, STG-9, LSUP-11, TR-P6 with TR-13 to TR-15, UNW-1, OBJ-11 (the older case), ENUM-10 with ENUM-P1, NS-12, NEW-P5, ICS-45, and CMP-8. The packet counts only questions, so the share and the "tests whose expectation changes" column leave all 14 out.
- **The review-ID form.** Decision 0059 and the ledger header give `RV5-A3`. The reviews and the ledger use `RV5-A-3`.
- **Decision 0059's planned-row choice for review points** has no case for a point that the repair keeps against the review. Five such points are `commit` rows.
- **Decision 0060's first condition** says "every test passes its check". `frontend_check.py` counts a `run-` status as a passed check, so 164 tests fail by its count. Report 23, section 6 counts 204, with the 40 `run-unsupported` tests. The condition also excepts tests "that the front-end ledger marks out of scope". The front-end ledger has a row per finding and no row per test, and report 23 names six tests that could take such a mark.
- **The front-end faults of item E** (I5-9, I5-10, W20-3) have reductions under `reports/clean-room/` and no test, so the harness cannot see them.
- **The request to the maintainer** for rows 1, 4a, and 10 is marked done in the roadmap, and no file records it.
- **Similarity output under `build/`** for four commits.

### The ten weakest pages

| # | Page | R/T | Q | F | X | Ev | R/P | FE | Other evidence |
|---|---|---|---|---|---|---|---|---|---|
| 1 | overloading/conversion-sequences | 47 / 144 | 14 | 247 | 31 | 196 / 1072 | 9.4 | 12 | Q-0419 is new. No question settled in phases 9 to 12. 7 are in row 1's group, 5 held by doubt. ICS-11 changed twice in phase 12, and RV5 read it against before the repair. ICS-45 has a row 4 item with no question. The most front-end failures of any page |
| 2 | foundations/storage | 29 / 98 | 11 | 292 | 6 | 163 / 1036 | 3.6 | 2 | Q-0323 settled. STG-5 and STG-P3 changed after RV5, and STG-8 in F2. W20-1 and W20-6 are open on STG-P2 and the storage forms. Q-0307 is still the one test against all three families |
| 3 | overloading/overload-resolution | 44 / 196 | 10 | 374 | 30 | 205 / 1054 | 11.0 | 6 | The same 10 questions. The highest R/P for a seventh round. OVL-11 changed in F2 |
| 4 | foundations/constant-evaluation | 27 / 159 | 8 | 443 | 27 | 194 / 835 | 5.4 | 2 | Q-0421 is new. 6 of 8 are in row 1's group. The highest F |
| 5 | classes/members | 34 / 174 | 8 | 188 | 16 | 151 / 863 | 8.5 | 1 | The same 8. CLS-26 changed in phases 11 and 12, and after RV5. 3 wait for a review that finds the reference silent |
| 6 | expressions/new-delete | 42 / 151 | 7 | 251 | 25 | 229 / 1002 | 6.0 | 10 | NEW-22 and NEW-27 changed in F2. NEW-P5 has a row 4 item with no question. G155: no reject rule names a deleted or inaccessible allocation function |
| 7 | templates/class-template-deduction | 29 / 119 | 7 | 84 | 22 | 209 / 894 | 9.7 | 8 | No change in phases 11 and 12. 6 of 7 wait on the maintainer (rows 3 and 4). Report 23 did not target its failures |
| 8 | templates/deduction | 39 / 143 | 3 | 304 | 13 | 214 / 1027 | 7.8 | 5 | New on the list. F rose from 207, from I5. DED-29 changed in RD and holds Q-0420 and RV5-B-10. 9 of the 11 I5 rows name DED rules (I5-1 to I5-8, I5-11) |
| 9 | templates/ordering | 17 / 64 | 6 | 186 | 19 | 200 / 822 | 4.2 | 1 | ORD-13 changed in phases 11 and 12, and after RV5 |
| 10 | expressions/conditional | 16 / 93 | 3 | 127 | 8 | 75 / 435 | 5.3 | 1 | COND-3, COND-8, COND-11, and COND-16 changed in F2, with 21 new tests. RV5 read COND-16 against (Q-0418, now row 4) and kept the compilers' answer on B-2 and B-3. Front-end failures fell from 8 to 1. G153 is open on COND-16 and REF-1 |

Foundations/language-support left the list (Q 5, F 181). Its LSUP-11 holds Q-0421 and G156. Templates/deduction joined it.

## 2. Ranked work items

| # | Item | Needs first | Evidence | Size |
|---|---|---|---|---|
| 1 | **Answer the 17 open rows and the 5 open front-end rows** (F3), with the front-end faults as tests first | the phase decision names the item in each row | W20-1 to W20-6, I5-1 to I5-11, G153 to G157. I5-9, I5-10, and W20-3 have reductions and no test. DED-29 holds I5-1, I5-2, Q-0420, and RV5-B-10 | one writer session with probes |
| 2 | **Review the text phase 12 wrote after RV5, and the text of item 1** (RV6), then the repairs and a read of the repair diff | item 1 | 20 rules and principles changed after RV5. ALN-4 changed four times in two phases. REF-1 and ICS-11 read against before their repair. Q-0422 has no review | one source-reader session in two parts, one writer session, one short source-reader pass |
| 3 | **Front-end round 24, with the distance to decision 0060 from a tool** | item 1's tests | 164 tests fail their check by `frontend_check.py`, 204 by report 23. Six tests could be out of scope, and no file can mark a test so | one front-end session, plus a tool change |
| 4 | **Put the packet to the maintainer, on the record, with every row 4 item counted** | nothing | No row decided in three phases. 14 row 4 items have no question. The seven "against" questions outside row 4 are unchanged since round 8 | under one session, plus the maintainer's answers |
| 5 | **Repair the records that drift** (R7), and extend the ledger check | nothing | Section 1, "Records that drift" | under one session |
| 6 | **A narrow second review of the doubt-held questions** | item 2, so the review reads current text | 41 questions held by doubt alone, 20 last read on 2026-11-03. Decisions 0058 and 0059 deferred it | one source-reader session in two or three parts, one writer session |

### 2.1 Answer the 17 open rows and the 5 open front-end rows

The order follows the choice that a front-end finding gets a test, then the rules that phase 12 already changed, then the rest. Each answer sets the row to `commit`, `text`, `question`, or `out of scope`.

| Rank | Rows | Work | Size |
|---|---|---|---|
| 1 | I5-9, I5-10, W20-3 | A test from each reduction (`I5/reductions/r1` to `r3`, `W20/reductions/self-by-value-template-constraint.cpp`), run on the matrix. W20-3 also asks when CPY-15's by-value specialization leaves the candidates against CNS-11's check, and needs a probe. A question if the front end and the compilers stay apart | small |
| 2 | I5-1, I5-2 | DED-30 names DED-29's paragraph on an address; DED-29's first list says "a body that the function type needs". DED-29 changed in RD and RV5 left a doubt on it (RV5-B-10), so these go before RV6 reads it | small |
| 3 | I5-4, I5-10, G153 | One statement across DED-15, REF-1, and COND-16 on a base that is private or ambiguous: the base relation decides the step, and ACC-12, CONV-17, and OVL-37 apply after the choice. REF-1 changed in RV5's repairs, and RV5 read it against before them. A reject test with two equal bases and a fallback | medium, with a probe |
| 4 | I5-3, I5-6 | Two missing statements: cv-qualifiers on a function type in deduction (DED-7 or DED-14), and the cv-qualifiers of a non-class prvalue in DED-8. The I5 programs p01 and p04 are the probes | small |
| 5 | I5-5, with I5-9 | DED-24 and INST-1 on `T::type` with an incomplete `T`. The I5 program p03 is the probe | small |
| 6 | W20-2 | A rule, or a question, for the elements of an array member of a union (UNI-13, UNI-14). Needs a probe on the newest families | medium |
| 7 | W20-1, W20-5, W20-6 | A sentence beside the `FixedVec` example (STG-P2), a sentence in CAST-1 that links STG-16, and one table of the storage forms on the storage page. These fall on the second weakest page | small |
| 8 | G155, G156, G157 | A reject rule for a deleted or inaccessible allocation function (NEW-25 or a new rule); LSUP-11's element forms beside Q-0421; the INST-11 Evidence on the uses that make the matrix compilers instantiate a virtual function | small to medium |
| 9 | W20-4, G154 | A CMP-15 Evidence note and a constant test under the sanitizer flags; the ACC-3 tests that G154 asks for | small |
| 10 | I5-7, I5-8, I5-11 | Examples for DED-20 and the DED-P4 table; DED-24's list gains arrays of functions and of `void`; DED-11 names the size type as the tests do | small |

### 2.2 Review the text phase 12 wrote after RV5

RV4 and RV5 each ran in the middle of their phase, and each left 15 to 20 blocks for the next phase. RV6 runs after item 1, so that item 1's text and the 20 blocks above meet one review. The review has three parts:

- Part A reads the 20 blocks of 3bed9cb5, ALN-4 and REF-1 first.
- Part B reads the text item 1 writes, and gives a first reading of Q-0422 and of each question item 1 opens.
- Part C reads the five points that the RV5 repair kept against the review with an Evidence note (B-2, B-3, B-5, B-7, A-14), and states for each whether it reads against or with a doubt. A point that reads against gets a question.

After the repairs, a short source-reader pass reads only the repair diff. The close commit then lists the blocks that changed after that pass, which should be few.

### 2.3 Front-end round 24, with the distance to decision 0060 from a tool

Item 3 is the front-end view's item, and the editor ranks only its records:

- `frontend_check.py` prints one line for decision 0060: the tests that fail their check, the `run-unsupported` tests, and the program tests that do not run. The decision text then says which count the first condition uses.
- A file says which tests are out of scope for the front end, with a reason, and the check leaves them out. Report 23 names six candidates (two `<cstdarg>` tests, `char-named-escape.cpp`, and three tests that need an ILP32 bound or a size above the size type).
- Round 23 cut the tests that fail their check from 182 (of 6681) to 164 (of 6786). Decision 0060 allows a roadmap to propose the rewrite early when the failures are few and stable over two rounds. Round 24 gives the second measurement.

### 2.4 Put the packet to the maintainer, on the record, with every row 4 item counted

- The integrator records the request in `pending.md` with its date and the rows it names, and records the answer, or the lack of one, in the next close commit.
- Each of the 14 row 4 items with no question gets a question, or the packet counts the items beside the questions. The tool already reads the row 4 table.
- The seven "against" questions outside row 4 join row 4a or 4b by the family table, or the packet says why each stays out. Round 9 asked for this, and phase 12 did not do it.
- The close commit copies `question_index.py --share` and compares it with the line of ebc109b9 (63% and 70%).

### 2.5 Repair the records that drift, and extend the ledger check

- The close commit and the status page's next step name the open rows of both ledgers.
- Decision 0059's ID form and the ledger header say `RV5-A-3`, the form in use.
- `check_specification.py` also reads point IDs in `reports/review/` and fails on one with no ledger row.
- `check_specification.py` fails on an exercise ledger row in state `open` or `planned` whose finding the report calls a front-end fault and whose Record names no test. The choice of decision 0059 then has a check.
- The phase 12 similarity outputs under `build/` stay as they are, and later commits write theirs under `reports/similarity/`.

### 2.6 A narrow second review of the doubt-held questions

Rounds 8 and 9 proposed this item, and decisions 0058 and 0059 deferred it. The count grew from 38 to 41, and 20 of the 41 were last read in the 2026-11-03 cluster reviews. The prompt reads one question per section, lists its tests, and asks for a verdict on a fixed scale (supports; supports, with the missing step named; reads against; silent). A named step becomes a repair, and a "silent" verdict moves the question to row 10. Running it after RV6 keeps it on current text.

## 3. What to keep out of scope

| Topic | Proposal | Reason |
|---|---|---|
| New pages | None | Every topic in scope has a page. Phases 10 to 12 added no page |
| The front-end rewrite (FR) | Wait for decision 0060's test, or for two stable rounds | 164 tests fail their check after one measured round |
| Adopting P25 (X23) | Leave to a later decision | Adopting it adds text for RV6 to read |
| Propose X24 on P26, Diagnose batches, Port suite P2 | Defer | Items 1 to 4 reach more open rows and questions per session. Decision 0057 asks each phase to run an exercise that writes programs, and the editor has no view on its topic |
| Splitting the overloading principles | Keep off the list, as decision 0059 left it | R/P is unchanged (11.0, 9.4). The open rows on those pages are missing steps |
| Later language versions | Keep out until row 7 is decided | The plan is costed in `pending.md` |
| Extended floating-point types, coroutines, modules, threads and the memory model, the library, the preprocessor beyond `#include "file"`, MSVC | Keep out, as the charter and decisions 0051 to 0059 say | 33 *(referenced)* markers name them |

## 4. Process changes

- **The review reads after the last item that writes rule text.** RV4 and RV5 ran in the middle of their phases, and 15 and then 20 blocks changed after each. The phase decision orders the review after the answer items, and a short pass reads the review's own repair diff.
- **A point kept against the review gets a question.** A repair that keeps the compilers' answer with an Evidence note leaves the reading open, and a question records it. A row 4 item gets a question ID, so the packet counts it.
- **An exercise's front-end fault becomes a test in the exercise's commit.** Decision 0059's choice held for T and FE and not for E. A check on the ledger makes it hold.
- **The close names the open rows of both ledgers.**
- **The share comparison uses the previous close's tool line.** From phase 13 on, both ends are tool lines.
- **The request to the maintainer is a dated record.** A request that only the roadmap marks done leaves no trace of what was asked.
- **Decision 0060's distance comes from a tool, with one meaning of "passes its check".**
- **Similarity output goes under `reports/similarity/`**, so every commit's claim has a tracked record.

## 5. Files

The reader's scripts are in `reports/clean-room/roadmap-10/editor-scratch/`: the round 9 scripts with the trees changed (`count.py`, `diff.py`, `when.py`, `q.py`, `share.py`, `pages.py`, `pagewaits.py`, `smalldiff.py`, `divtags.py`), `ledger_moves.py` (the ledger states and moves between two commits), `replay.sh` (the two read-only checks at each phase 12 commit, on `git archive` copies), and `rv5_repairs.py` (the word changes of 3bed9cb5). They ran on copies made with `git archive` in `/tmp/claude-1000/-home-ryanm-Code-tidal/939f1ac9-57f9-42e8-8e86-91384ffa67eb/scratchpad/r10e/`: the tree at a15a877d (`t9/`), at ebc109b9 (`t10/`), and the specification before and after 3bed9cb5 (`pre/`, `post/`). The reader also ran `tools/check_specification.py`, `tools/question_index.py --check` and `--share`, and `tools/checks.py --no-frontend` with no reference corpus, which change no file.
