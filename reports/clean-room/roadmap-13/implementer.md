# Roadmap, round 13: what to do after phase 15, from an implementer's view

Date: 2026-11-11. Decision 0029. Role: clean-room reader, acting as an implementer who builds a C++ front end and interpreter from Tidal alone and uses the specification as the design input. The evidence is the clean-room front end in `frontend/`, its reports, ledger, baseline, and marks file in `reports/clean-room/rust-frontend/`, the design of the rewrite (`frontend/DESIGN-rewrite.md`), and the phase 15 exercise and review records. Commit read: `a6a2b16c`.

Inputs:
- AGENTS.md, CLAUDE.md
- `reports/clean-room/roadmap-12/implementer.md`, the model for this report's form
- `documentation/`: status, roadmap (the phase 15 table), decisions 0060, 0062, and 0063, and `decisions/pending.md`
- `git log 51bd5060..HEAD` with every commit body (21 commits)
- `frontend/DESIGN-current.md`, `frontend/TEMPLATE-MODEL.md`, and `frontend/DESIGN-rewrite.md`
- `reports/clean-room/rust-frontend/`: report 26 in full, `rewrite-marks.tsv`, `ledger.md`, and `conformance.tsv` at `a91ccc77` (round 26), `b035efd4` (round 25), and `53577f27` (round 24)
- `reports/clean-room/X25/report.md` and `reports/clean-room/P4/report.md`
- `reports/clean-room/ledger.md` (the 16 open rows)
- the RV8 reviews and reads in `reports/review/` dated 2026-11-11 (the Result sections, the RV9-R points, and the headings)
- `questions/`: the status and Waits for lines of the questions that failing tests name
- `tests/`: the headers of the failing tests, and the whole text of four of them
- `tools/frontend_check.py`, and `frontend/src/` by line counts only

I did not read the other round 13 reports, `inputs/`, or `build/exercise-keys/`. I ran no compiler, so this report makes no host-only claim.

**Measurements.** I ran `tools/frontend_check.py` once under the run lock, then `tidal-fe conform tests -v` once, then `tidal-fe check` on each test whose base expectation is `reject`, each under the run lock, with the binary that the check built. The check's lines match the phase 15 close commit:

```
frontend: 6895/7015 pass, 6926/7015 pass their check, 0 worse, 0 better, 0 moved between passing statuses, 8 new (5 not passing)
frontend: 22/22 interpreter-only expectations met
frontend: completion (decision 0060): 80 tests fail their check, of which 32 need the template model (marks file) and 48 count toward the gate of decisions 0062 and 0063; 5/152 program tests do not run to their result; 9 tests out of scope
frontend: 2139/2766 passing rejects name a cited rule (1952) or a linked rule (187) in their first error; `tidal-fe conform` gives the table per page
```

The scripts and their output are in `reports/clean-room/roadmap-13/implementer-scratch/`. `results.tsv` holds the 454 rows of `tidal-fe conform -v` that are not a plain pass. `causes.py` reads them with the round 24, 25, and 26 baselines, the marks file, and the cause table of report 26 section 8. It sorts the 48 unmarked tests into cause groups, checks stability, lists the run stops, and lists the failing tests that cite a rule changed after RV8. The cause of each of the 45 tests of report 26 is report 26's. The three P4 tests take their cause from P4-1, by my judgment. The groups are my judgment. `questions.py` lists the open questions that the failing tests name. `parse_targets.py` and `first_errors.py` count the targets of the parse count that item 1 proposes.

## 1. Where the specification and the front end stand

| Measure | Round 12 (6947 tests, `bd87b27c`) | Now (7015 tests, `a6a2b16c`) |
|---|---|---|
| Full pass | 6804 (97.9%) | 6895 (98.3%) |
| Check pass | 6831 | 6926 |
| Completion line (decision 0060) | 107 tests, 3 of 138 program tests, 9 out of scope | 80 tests (32 marked, 48 toward the gate), 5 of 152 program tests, 9 out of scope |
| Marked for the template model | none (report 25 named 30 in a table) | 32 in `rewrite-marks.tsv` |
| `MISSED-REJECT` in scope | 45, and 6 that pass as `NO-DIAGNOSTIC-ALLOWED` | 38, and the same 6 |
| `unsupported` in scope | 50 | 30 |
| `FALSE-REJECT` in scope | 7 | 5 |
| `EDG-AGREES` (a failure) | 5 | 7 |
| `run-unsupported` | 27 | 31 |
| `RUN-WRONG-RESULT` and other wrong runs | 0 | 0 (27 runs pass with a partial exploration) |
| Passing rejects whose first error names a cited or linked rule | 2115 of 2743 (77%) | 2139 of 2766 (77%) |
| Rules | 1700 | 1700 (no new rule) |
| Questions open | 185 | 197 |
| Front-end findings (G) | G1 to G165, 2 open | G1 to G165, 0 open |
| Exercise ledger rows open | 26 | 16 (P3-9, P3-10, P4-1 to P4-5, P4-7, X25-1 to X25-7, X25-9) |
| Front-end source | 52045 lines, 28 files | 52487 lines, 28 files; round 26 changed 13 files |
| Files over 2000 lines | 7 | 7: `expr.rs` 8051, `decl2.rs` 7282, `interp/expr.rs` 4865, `templ.rs` 4360, `decl.rs` 3224, `expr2.rs` 2985, `parser.rs` 2421 |

### What phase 15 took from round 12

| Round 12 item | Phase 15 result |
|---|---|
| 1. Front-end round 26 on the unmarked tests and the run stops | Done (`a91ccc77`). The unmarked count went from 97 at the start of the round to 45. The round fixed the STG-23 wrong verdict, P3-7, and P3-8. P3-9 and P3-10 still stop in a run. |
| 2. An amendment to the gate | Partly done in GC (`17077248`). The marks file holds the marked tests with their model section, and the completion line counts them apart. The marks file took the place of a `rewrite` ledger state. No takeover list exists. |
| 3. Rewrite stage 1 | The design only, in RD (`14ebbde6`). Decision 0063 started no code before the gate held. |
| 4. Text for the I7 and P3 rows, G164, and G165 | Done in F5 (`232ac618`). I7-6, I7-13, I7-14, and P3-6 became Q-0438, Q-0439, Q-0437, and Q-0441. |
| 5. A gap review of the 20 changed rules, with the read of the RV7 repair diff | Done in RV8 (`6a73ea89`, `29e8ab25`, `9f3bceb4`, `d6fd9cea`, `76f9f670`, `bdc27d44`). The read of the RV8 repair diff withdrew RV8-R-1 with RV8-A-6, and RV8-R-4. |
| 6. Implement I8 on instantiation | Not taken. Decision 0063 defers it. |

Of round 12's process changes, phase 15 took change 1 (the marks file and the third count), change 3 (R9's check for a recorded read of each repair diff), and change 7 (report 26 keeps report 25's form). Change 4 holds in part: report 26 marked two tests by model section, and `TEMPLATE-MODEL.md` section 6 says why the three P3 tests needed no mark. Change 2 (causes in the baseline) is not in place, and the 48 unmarked causes live in a report table and in the P4 report. Change 5 (a first-error target) is an open point of `DESIGN-rewrite.md` section 10. Change 6 had no Implement exercise to act on.

### The 80 tests of the completion line

| Measure | Round 23 | Round 24 | Round 25 | Round 26 | Now |
|---|---|---|---|---|---|
| In-scope tests that fail their check | 149 | 127 | 104 | 77 | 80 |
| Of these, marked for the template model | | 16 (by report) | 30 (by report) | 32 | 32 |
| Of these, counted toward the gate | | | 74 | 45 | 48 |
| Old failures passed in the round | 18 | 32 | 29 | 30 (the check's "better" count) | |
| New tests, and how many fail their check | 105, 0 | 92, 10 | 60, 6 | 69, 3 | 8, 3 |

44 of the 48 unmarked tests failed at round 24 and at round 25 with the same status. The other 4 arrived after round 25: P4's three tests and RV8's `literal-operator-template-class-parameter.cpp`. No test that passed its check at round 24, 25, or 26 fails now.

By chapter and status:

| Chapter | Missed reject | Unsupported | False reject | EDG-AGREES | All | Unmarked |
|---|---|---|---|---|---|---|
| templates | 27 | 19 | 3 | 5 | 54 | 24 |
| expressions | 2 | 4 | 0 | 0 | 6 | 6 |
| classes | 3 | 1 | 0 | 1 | 5 | 4 |
| declarations | 2 | 1 | 2 | 0 | 5 | 5 |
| foundations | 2 | 2 | 0 | 0 | 4 | 4 |
| overloading | 1 | 2 | 0 | 0 | 3 | 3 |
| programs | 0 | 1 | 0 | 1 | 2 | 2 |
| statements | 1 | 0 | 0 | 0 | 1 | 0 |
| **total** | 38 | 30 | 5 | 7 | 80 | 48 |

### The causes

The 32 marked tests, by the section of `TEMPLATE-MODEL.md` that the marks file names:

| Model section | Tests | Status now | Rules |
|---|---|---|---|
| 1, dependence marks | 4 | 1 missed reject, 1 unsupported, 2 EDG-AGREES | DEP-15 with DEP-30 or DEP-1 (2), DEP-4 (2) |
| 2, the current instantiation | 6 | 3 false rejects, 2 missed rejects, 1 unsupported | DEP-24 (4), DEP-11, CPY-15 |
| 3, a specialization | 11 | 6 missed rejects, 3 unsupported, 2 EDG-AGREES | INST-33 with INST-30, INST-5, or INST-32 (6), INST-5 (2), INST-12 (2), INST-15, TSP-31 |
| 4, the immediate context | 4 | 2 missed rejects, 2 unsupported | DED-24 (2), DED-29 (2) |
| 5, checks at the definition | 7 | 7 missed rejects | CNS-29 (2), DEP-37 (2), PACK-8 (2), IF-22 |

The 48 unmarked tests. `causes.py` puts each test in the first group that fits.

| Cause | Tests | Rules |
|---|---|---|
| P4-1: the types of a member function defined in its class are checked while the class is incomplete | 3 | FN-12 (a false reject and a missed reject), and `programs/P4/money-first-try.cpp` (EDG-AGREES) |
| Class template argument deduction | 5 | CTAD-1 sites (2), CTAD-19 with CTAD-23, CTAD-29 with CTAD-11, and `r10-tuple-second-try.cpp` |
| A constant parameter whose type is a class template name | 3 | TPL-11 with CTAD-1 (2), and OPR-8 with OPR-10 (RV9-R-5) |
| Template checks outside the marks (missed rejects) | 10 | TARG-24, TARG-15, TARG-7, CPT-37, CNS-13, TPL-31, TPL-7, ORD-3, PACK-26, TSP-23 |
| Template constructs outside the marks (unsupported) | 7 | TARG-14, CPT-1, CPT-20, CNS-30, DEP-20, PACK-3, PACK-5 |
| Constant evaluation of a construct (unsupported) | 4 | PRIM-27, CE-3, LSUP-13, OVL-40 |
| Missed rejects in other chapters | 8 | DCMP-11, CTOR-12, CEV-1, UNEV-1, PFX-11, CE-3, NAM-8, OVL-20 |
| Unsupported constructs in other chapters | 5 | AUTO-7, COND-3, LAM-6, LAM-18, ICS-47 |
| EDG-AGREES | 2 | CPY-27, CNS-32 |
| False reject | 1 | SB-5 |
| **total** | 48 | |

Report 26 says the current design can take each of its 45 tests. I read three of them as closer to the model than to the old code:

- `templates/arguments/argument-makes-parameter-invalid.cpp` (TARG-24). Report 26 says the front end keeps `typename T::size_type` as an opaque dependent type, so its substitution with `int` cannot tell a failure from an unknown result. Model section 4 decides that case with the kind table of `Member(T, name, reading)` (DEP-19, DEP-22, DEP-26).
- `templates/packs/member-template-pack-length.cpp` (PACK-26). The member template's declaration is made with the outer pack expanded, which DED-20 describes and model section 3's substitution does.
- `templates/dependent-names/type-only-positions-more.cpp` (DEP-20), and its run stop `type-only-positions.cpp`. The rewrite's `Names` oracle answers in the type-only contexts of DEP-20 (`DESIGN-rewrite.md` section 2).

Nine of the 80 tests rest on an open question. Marked: Q-0427 (3, waits for a use of DEP-P5), Q-0432 (2, a use of DEP-P2), Q-0433 (1, a review), and Q-0441 (1, a review and newest families that agree). Unmarked: Q-0339 (CPY-27) and Q-0406 (PFX-11, `pending.md` row 4). Q-0231, which three marked tests of section 3 cite, is settled.

Of the 20 rules and principles that changed after RV8 read the tree, failing tests cite INST-5 (4 marked tests of section 3), FN-12 and INST-1 (the two P4-1 tests), and OPR-8 and OPR-10 (two unmarked tests).

### Run stops

31 tests pass their check and stop in a run, 27 at round 25 and 29 at round 26. The two new ones are P4's `initializer-list-parameter.cpp` (P4-2) and `programs/P4/sequences-first-try.cpp`. Three of the 31 are program tests: `events-first-try.cpp` (P3-10, a class assignment), `pipeline-first-try.cpp` (P3-9, a range `for`), and `sequences-first-try.cpp` (P4-2). With `money-first-try.cpp` and `r10-tuple-second-try.cpp`, which fail their check, they are the 5 program tests of the completion line. The others are report 25's groups: an overload set (CVF-1, CVF-4, CVF-5), a class assignment (OVL-7, OVL-42), the `new` and placement forms (NEW-23, NEW-24, DED-39), a variable or function with no definition in the unit (VIRT-12, SINIT-1, ODR-15, LNK-9), a destructor function try block (TRY-10), the braced list beside a constructor template (ILC-3, two T4 tests), and single causes (DCMP-4, UN-7, TARG-12, DEP-11, DEP-24, DEP-20, INST-36, TSP-8).

### Decision 0060 and the gate of decisions 0062 and 0063

The gate held at the end of round 26, and the phase 15 close records it:

- **Few.** 45 unmarked tests, at most 50. Now 48 with P4's tests.
- **Stable.** No test moved from a passing status to a failing one in rounds 25 and 26.
- **A named cause.** Report 26 section 8 names a cause for each of the 45, and the marks file names a model section for each of the 32.

So decision 0062 has the phase 16 decision add the rewrite as an item and name the tests it takes over, and decision 0063's ban on rewrite code ends.

The margin is 2. Phase 15's test items added 51 tests before round 26, and 30 of them did not pass. Phase 16's test items will put the unmarked count over 50 before the next round. The gate is the start condition of the rewrite, and it has held. The phase 16 decision should say that the count stays a measure and stops nothing once the rewrite has started.

Decision 0060's own completion test needs all 80 tests and the 5 program tests. The old code is removed only when the new code passes every test that the old code passed.

### Where the rewrite stands

- **The design.** `DESIGN-rewrite.md` gives the crate (`frontend/next/`, `tidal-next`), the stages, a module for each of the 85 rule pages, the place of each model section, a build of 10 steps (0 to 9), unit tests, and the prototype hook for Propose. It has no code, as decision 0063 asked.
- **Stale counts in the design.** Section 5 names the 30 tests of report 25. The marks file has 32 since round 26: `classes/copy-move/class-template-ctor-by-value.cpp` (section 2) and `templates/instantiation/static-inline-member-incomplete-argument.cpp` (section 3) are missing from it. The build table's counts follow the 30. Step 5 takes 17 tests (4, 6, and 7), and the marked count goes from 32 to 15. Step 6 takes 11, to 4. Step 7 takes 4, to 0. Section 4's row "6. The 30 tests" has the same count.
- **Stale open points.** Section 10 says the `forward_class` gap has no ledger row. RM12-2 holds it, and `d6fd9cea` answered it with TYN-10 tests that pass. Section 10 also keeps the three P3 tests as a possible mark, and round 26 passed them on the old code.
- **The template model.** `TEMPLATE-MODEL.md` section 6 follows the marks file (32). Section 8 still lists P3-6 (now Q-0441) and an RV8 row that names RV8 as the review to come. After RV8, INST-1, INST-2, INST-5, DEP-P1, and DEP-40 changed, and model sections 1 and 3 cite them.
- **The old code's description.** `DESIGN-current.md` says it describes the code after round 24 (`53577f27`) at 51375 lines. Rounds 25 and 26 changed `src/` since, and the source is 52487 lines.
- **The tool.** `frontend_check.py` has no `--next`, no takeover list, and no parse count (`DESIGN-rewrite.md` section 10).
- **The grammar.** The Syntax segments hold 256 nonterminals on 62 pages (`parse_targets.py`). Step 2 writes one function for each.
- **The first-error rate.** 77% for a seventh phase. 627 passing rejects name another rule first.

### The P4, X25, and RV8 findings

| Rows | Who acts | Why the front end cares |
|---|---|---|
| P4-1 (FN-12, CLS-2, INST-1) | front end | The text now says the types of a member defined in its class must be complete at the end of the class. The old front end checks them at the declaration, and it fails two tests and `money-first-try.cpp`. The deferred parts of a class body (NAM-6) are the place for the check. |
| P4-2 (OPR-6, LSUP-1) | front end | An operator function with an `std::initializer_list<E>` parameter stops the run. One test and `sequences-first-try.cpp`. |
| P3-9 (RFOR-2), P3-10 (OVL-7) | front end | Run stops with tests, and two program tests. |
| P4-3, P4-4, P4-5, P4-7 | text, examples | P4-7 asks OPR-8 to say which value the numeric form receives. The machine gives the right value today. |
| X25-1 (LSUP-10, UNI-14), X25-4 (UNI-14, UNI-6), X25-5 (CE-22), X25-7 (CE-21) | text | Each is a constant-evaluation or run rule that the machine follows. The two rules of X25-1 disagree on `slot-array-constant.cpp`, which the front end passes by UNI-14. |
| X25-2, X25-3, X25-6, X25-9 | text, glossary | No front-end work follows. |
| Report 26 section 5 | a test, then the front end | A literal operator template called by its name with explicit arguments gives a wrong value in constant evaluation. No test covers it. |
| RV9-R-4 (TYN-10), RV9-R-5 (OPR-8) | text | Small doubts. RV9-R-5 is the form of the unmarked test `literal-operator-template-class-parameter.cpp`. |

## 2. Ranked items

| Rank | Item | Evidence | Size | Prerequisites |
|---|---|---|---|---|
| 1 | The rewrite, steps 1 and 2: the crate, the lexer, and the parser with the `Names` oracle, and the tool for two front ends | the gate held at round 26; decision 0062 adds the rewrite now; 32 marked tests the old code cannot pass; no syntax tree exists | large | none |
| 2 | Front-end round 27 on the old code: the program tests, the exercise faults, phase 16's tests, and the groups in code the rewrite moves | 5 program tests; P3-9, P3-10, P4-1, P4-2 open; the old code is the only front end that judges a test in phase 16 | medium | the phase's test items for their tests |
| 3 | Text for the P4 and X25 rows | 6 P4 rows and 8 X25 rows open; four X25 rows are rules the machine follows | small each | none |
| 4 | Gap review RV9 of the 20 rules changed after RV8, with the read of its repair diff | 4 marked tests cite INST-5; model sections 1 and 3 cite INST-1, INST-2, DEP-P1, DEP-40 | medium | 3 for its text |
| 5 | Implement I8 on instantiation | model section 3 is the largest marked group (11); Q-0433 and Q-0441 are open; deferred twice | medium | none |

### 1. The rewrite, steps 1 and 2: the crate, the lexer, and the parser with the `Names` oracle, and the tool for two front ends

**What it is.** A clean-room writer in the implementer role starts the new front end from `DESIGN-rewrite.md`, under decisions 0029, 0035, and 0060.

**Scope in phase 16.** The item does steps 1 and 2 of `DESIGN-rewrite.md` section 5, and brings step 0's document up to date.

| Step | In phase 16 | Work |
|---|---|---|
| 0 | yes, as an edit | Section 5 and its build table follow the marks file (32 tests; 17, 11, and 4 by step). Section 10 drops the `forward_class` point and the P3 point. |
| 1 | yes | The crate `frontend/next/` with its own `Cargo.toml`, `Cargo.lock`, and `target/`, so the old crate's build does not change. `source`, `diag`, `lex`, and the driver with the subcommands of `tidal-fe`. `lexer.rs` (1098 lines), `diag.rs` (59), `main.rs` (234), and `conform.rs` (730) move with spans added. |
| 2 | yes | `syntax`, and the `syntax.rs` part of each page module with Syntax segments: 256 nonterminals on 62 pages, one function each, named after its production. The oracle answers over a declare pass that records names and their kinds only. A stub oracle serves the unit tests. |
| 3 | only after 1 and 2 end | The core parts that move with small changes (`target` from `layout.rs`, `machine` from `interp/mod.rs` and `interp/f80.rs`). No page check. |
| 4 to 9 | later phases | |

**The tests it takes over.** In phase 16 the new front end owns no test. Steps 1 and 2 give no outcome, so the old front end keeps every status. The phase 16 decision names the 32 tests of `rewrite-marks.tsv` as the tests the rewrite takes over first, by model section and step:

| Step | Model section | Tests (under `tests/`) |
|---|---|---|
| 5 | 1 | `templates/dependent-names/`: `other-specialization-constant-argument-member-missing`, `other-specialization-constant-argument-no-typename`, `sizeof-value-dependent-operand-call-bound-early`, `sizeof-value-dependent-operand-no-typename` |
| 5 | 2 | `templates/dependent-names/`: `current-instantiation-enclosing-base`, `member-template-current-instantiation-pointer`, `member-template-current-instantiation-qualifier-dependent-base`, `member-template-non-dependent-base`, `member-template-through-current-instantiation-object`; `classes/copy-move/class-template-ctor-by-value` |
| 5 | 5 | `statements/if/constexpr-template-nondependent-call-checked`; `templates/constraints/`: `out-of-class-other-clause`, `out-of-class-without-clause`; `templates/dependent-names/`: `binary-operator-pack-never-fits`, `operator-pack-never-fits`; `templates/packs/`: `fold-in-outer-pattern`, `size-operator-in-outer-pattern` |
| 6 | 3 | `templates/instantiation/`: `class-point-not-later-member-type`, `class-point-not-later-type`, `class-point-not-later`, `class-point-static-const-member`, `class-point-static-constexpr-made-at-use`, `class-static-constexpr-member-initializer-waits`, `static-inline-member-incomplete-argument`, `local-class-default-argument`, `local-class-exception-specification`, `recursion-endless-function`; `templates/specialization/partial-no-primary-variable` |
| 7 | 4 | `templates/deduction/`: `substitution-explicit-specifier`, `substitution-lambda-body`, `substitution-lambda-default`, `substitution-member-wrong-kind` |

Each name ends in `.cpp`. The decision also names these as candidates. Round 27 (item 2) marks each one whose cause it finds in the model, or the rewrite takes it in step 8 with its page:

- `templates/arguments/argument-makes-parameter-invalid.cpp` (TARG-24), for section 4,
- `templates/packs/member-template-pack-length.cpp` (PACK-26), for section 3,
- `templates/dependent-names/type-only-positions-more.cpp` and the run stop `type-only-positions.cpp` (DEP-20), for the oracle's type-only contexts.

The 32 tests are 340 lines in all, and each is short. Step 3 can order its work by the constructs those tests use, so that step 5's first takeover does not wait for every page outside templates. The chapter-by-chapter shadow line stays the measure of steps 3 and 4.

**What `tools/frontend_check.py` needs.** Each part lands with the step that first needs it, and none changes a line that the tool prints now.

1. **`--next` (step 1).** The tool builds `frontend/next/` and runs `tidal-next conform tests -v` beside the old binary. It prints the `next:` lines of `DESIGN-rewrite.md` section 5. With no `--next`, the tool runs as now.
2. **The lex count (step 1).** Both binaries gain a `lex FILE` subcommand that prints one token a line (kind, text, line). `tidal-fe lex` is new code in the old front end that changes no test result. The tool compares the two dumps for each of the 7015 test files and 118 extra units, and prints `next: lex: T/7133`.
3. **The parse count (step 2).** `tidal-next` gains a parse status for each test: a tree, or a stop with TR-12. The targets come from the test headers and from the old front end's first error (`parse_targets.py`, `first_errors.py`):

   | Tests | Target | Count |
   |---|---|---|
   | `accept`, `run`, `run-fail`, and `link-fail` tests | a tree | 4191 |
   | `reject` tests whose first error on the old front end names a rule other than TR-12 | a tree | 2529 |
   | `reject` tests whose first error on the old front end names TR-12 (6 of them cite TR-12 alone) | a stop in the parser | 135 |
   | `reject` tests with no error that names a rule on the old front end | not judged | 160 |

   The line reads `next: parse: N/6855 tests parse as their outcome needs`, and the tool lists each disagreement. A disagreement is a fault of one front end, or a finding with a G row and a test.
4. **The takeover list (step 5, read from step 1).** A file `reports/clean-room/rust-frontend/takeover.tsv` beside `conformance.tsv`. A row is a test path, the step, and the commit that took it over. The tool reports a row whose test does not exist, a test listed twice, and a row for a test that the old front end passes and the new one fails. For a listed test, the tool takes the new front end's status, and `conformance.tsv` records it. A marked test that the new front end passes then reaches the tool's existing message, "passes its check; remove its row", and leaves the marks file.
5. **A shadow baseline (step 3).** `--update --next` also writes `conformance-next.tsv`, one status per test from the new front end. A move in it is printed. A move to a failing status fails the check only for a listed test, so "0 worse" holds for both front ends on the tests each owns.
6. **The completion line.** Once the list has a row, the line adds "O owned by the new front end" after the marked count and keeps its earlier numbers in order.
7. **Unit tests and the grammar check (step 1).** The tool runs `cargo test` in `frontend/next/` and prints `next: unit: K tests pass`. A failing unit test fails the check. A unit test of `syntax` reads the Syntax segments and fails when a nonterminal has no function or a function names no nonterminal (`DESIGN-rewrite.md` section 10).

**How the old front end is kept.**

- `tidal-fe` stays the front end of record. Its statuses fill `conformance.tsv` for every test that the takeover list does not name, and its rounds keep "0 worse".
- The old code takes phase 16's new tests and the exercise faults (item 2), because it is the only front end that judges a test in phase 16 (decision 0035).
- No round fixes a marked test in the old code. A fix to a listed test goes in the new code only (`DESIGN-rewrite.md` section 10).
- A moved file has two copies until step 9. A round that changes the old copy of a moved file (in phase 16, `lexer.rs`, `diag.rs`, `main.rs`, or `conform.rs`) makes the same change in the new crate and lists it in its report. The lex count shows a change to the lexer that reaches one copy only.
- The old code gets no refactoring. `DESIGN-current.md` stays its description, with its header and counts brought to round 26.
- The old code is removed only at step 9, when the new code owns every test that the old code passed (decision 0060).

**The measure that ends the item.**

- `next: lex: 7133/7133`.
- `next: parse: 6855/6855`, or each disagreement has a G row with a test and a cause in the round report.
- Each of the 256 nonterminals has one function in the `syntax.rs` part of its page module, by the unit test of part 7.
- `next: unit` passes, with unit tests in each `syntax.rs` and in `lex`.
- No file of `frontend/next/` passes about 2000 lines.
- The old front end's line stays at "0 worse" on every commit of the item, and the item changes no status.
- The takeover list exists and has no row.

**Evidence.** Section 1. The gate held at round 26, and decision 0062 has this decision add the rewrite. The old code cannot pass the 32 marked tests without the model, and it has no syntax tree for model section 1 to rest on (`DESIGN-current.md`, "Parsing and semantic analysis"). Every later step needs the lexer and the parser, and neither changes a test result. The design names the shadow run, the takeover list, and the parse count, and the tool has none of them yet.

**Size.** Large. Step 1 is medium, and most of its code moves. Step 2 is large: 256 functions and an oracle.

**Prerequisites.** None. The step 0 edit comes first and is small.

**Risk.** Medium. The oracle's three questions may not cover every ambiguity of the grammar (`DESIGN-rewrite.md` section 10). The parse count finds such points across 6855 tests before analysis is built on them.

**Alternative.** Steps 1 to 3 in phase 16, with the moved core of step 3 in full. I rank the narrower scope first, because step 2 is new code with no counterpart in the old front end, and a parse count at 6855 is a clear end.

### 2. Front-end round 27 on the old code

**What it is.** A round in the form of report 26, with its section 8 table kept and the completion line read with the marks file. The groups, in order:

1. **The 5 program tests** (decision 0060's second part). `events-first-try.cpp` (P3-10, a user `operator=` with a class parameter), `pipeline-first-try.cpp` (P3-9, a range `for` whose `begin` and `end` come from a base), `sequences-first-try.cpp` (P4-2), `money-first-try.cpp` (P4-1), and `r10-tuple-second-try.cpp` (CTAD-29 beside a constrained constructor template).
2. **P4-1 (FN-12).** The parameter and return types of a member function defined in its class are made complete at the end of the class, with the deferred parts of NAM-6. One false reject and one missed reject.
3. **P4-2, P3-9, and P3-10.** The three run stops with tests.
4. **Report 26 section 5.** A test of a literal operator template called by its name with explicit arguments comes first, from the phase's test item, and then the fix.
5. **Phase 16's new tests**, as they land.
6. **The groups in code that the rewrite moves.** The machine and constant evaluation move into the new crate with small changes (`DESIGN-rewrite.md` section 8), so a fix there carries over. Constant evaluation of a construct (4: PRIM-27, CE-3, LSUP-13, OVL-40), the CE-3 and CTOR-12 missed rejects, and the run stops of report 25's groups.
7. **Marks.** The round reads the three candidates of item 1 and marks each whose cause is in the model. A test that phase 16 adds and that fails on the model gets a mark when the round meets it.

The template groups outside the marks (17), class template argument deduction (4 after group 1), and the TPL-11 group (3) sit in `templ.rs`, `cons.rs`, and `ctad.rs`, which the rewrite writes again (`DESIGN-rewrite.md` section 8). The round takes them only where a fix is small. The other unmarked tests (DCMP-11, CEV-1, UNEV-1, PFX-11, NAM-8, OVL-20, AUTO-7, COND-3, LAM-6, LAM-18, ICS-47, CPY-27, CNS-32, SB-5) are the round's last group.

**Evidence.** Section 1. Rounds 23 to 26 passed 18, 32, 29, and 30 old failures. Four exercise rows wait for the front end. The new front end judges no test in phase 16, so the old code is the test of phase 16's text.

**Size.** Medium. Groups 1 to 4 are small to medium.

**Prerequisites.** The phase's test items for their tests. None for groups 1 to 3.

**Risk.** Low. A fix to `lexer.rs`, `diag.rs`, `main.rs`, or `conform.rs` after item 1 copies them is made twice (item 1).

### 3. Text for the P4 and X25 rows

**What it is.** The clean-room writer acts on each open row with text, in this order:

1. **X25-1 (LSUP-10, UNI-14).** LSUP-10 names a third place where `std::construct_at` can create an object in constant evaluation, or UNI-14 says why it accepts the construction. `slot-array-constant.cpp` and the `FixedStack` example depend on the answer. The machine follows UNI-14 today, and the test passes.
2. **X25-5 (CE-22) and X25-4 (UNI-14, UNI-6).** Whether a union's trivial copy reads the parts of its active member, and which reading of a construction inside an inactive member Tidal takes for a run. Each gets a probe on the matrix and a test where the compilers agree.
3. **P4-7 (OPR-8).** The value that the numeric literal form receives.
4. **X25-7 (CE-21) and X25-2 (OBJ-1).** An element ended by an explicit destructor call, and a row for an array's own lifetime.
5. **P4-5, P4-3, P4-4, X25-3, X25-6.** Examples, the glossary entry "lifetime", and a link from STG-P2 to the storage-form table.
6. **X25-9.** No change. The row becomes `text`.

P4-1's text is in place. Its front-end part is item 2.

**Evidence.** Section 1, "The P4, X25, and RV8 findings". X25's host-only claims need probes on the matrix before X25-4, X25-5, and X25-7 get tests.

**Size.** Small each. X25-1 and X25-4 may need a question.

**Prerequisites.** None.

### 4. Gap review RV9 of the 20 rules changed after RV8, with the read of its repair diff

**What it is.** A source-reader gap review of the 20 rules and principles that the phase 15 close lists, the text outside rule blocks on 13 pages, and item 3's text. It reads INST-5, INST-1, INST-2, DEP-P1, and DEP-40 first, then FN-12. The item ends with the recorded read of its repair diff (decision 0063).

**Evidence.** Section 1. Four marked tests of model section 3 cite INST-5, and model sections 1 and 3 cite INST-1, INST-2, DEP-P1, and DEP-40. Step 6 writes section 3 from these rules. Item 2's fix of P4-1 follows FN-12's new sentence, which no review has read.

**Size.** Medium.

**Prerequisites.** Item 3 for its text.

### 5. Implement I8 on instantiation

**What it is.** A fresh clean-room reader builds a component from `templates/instantiation.md` (INST-P1, INST-P2, and the INST rules), filtered of Evidence segments and `:tests:` lines as in I4 to I7. The component takes a unit's uses of a class template and its members in order, and returns which parts of each specialization are made, at which point, and what lookup at each point sees. The prompt gives the callbacks' answers for lookup and deduction.

**Evidence.** Model section 3 holds 11 of the 32 marked tests, the largest group. Four of them cite INST-5, which changed in F5 and in RV8's repairs. Q-0433 and Q-0441 are open on the same rules. Decisions 0062 and 0063 deferred I8. Step 6 of the rewrite writes section 3, and a blind build checks the text before that code.

**Size.** Medium.

**Prerequisites.** None. Item 4's reading of INST-5 helps.

**Alternative.** Constraint normalization and subsumption (CNS-P1). Six unmarked tests and two marked tests sit on the constraints and concepts pages. I rank instantiation first because the rewrite meets section 3 first.

### Dependency sketch

```
1 rewrite step 0 edit ──> step 1 (crate, lex, --next, unit) ──> step 2 (parser, oracle, parse count)
                                                               └──> step 3 in phase 17; steps 5 to 7 take the 32
2 round 27 (old code) ──> marks for the candidates ──> item 1's takeover table
3 text ──> 4 RV9 ──> step 6 (model section 3)
5 Implement I8 ──> step 6 (model section 3)
```

Items 1, 2, 3, and 5 can start together. Item 4 waits for item 3's text.

## 3. Scope

| Topic | Charter's reason to keep out | Implementer's view | Proposal |
|---|---|---|---|
| `<cstdarg>`, named escapes, `#include <...>`, extended floating-point types | the library, a name table, real headers, *(referenced)* | G42 and G158 to G160 hold 9 tests; the completion line leaves them out. The new crate keeps the same rows. | Keep the rows |
| ILP32 | in the matrix (EDG ILP32) | BF-9 on ILP32 passes since round 26; `target` keeps both tables | Keep in scope |
| `no-diagnostic` rules | the class needs no diagnostic | 6 tests pass as `NO-DIAGNOSTIC-ALLOWED` | Keep the status |
| Coroutines and lazy generators | outside the language goal | P4's sequences ran with RFOR-P1's class form, and the front end ran the final program | Keep out |
| Threads, the memory model, modules | outside the language goal | No test or front-end path needs them | Keep out |
| Later language versions (`pending.md` row 7) | the charter fixes C++23 | X25-9 names them as outside knowledge; the rewrite's work would not change | Keep out |
| MSVC | not in the matrix | A second ABI would double `target`'s tables | Keep out |
| Prototypes of Propose designs | not part of the rules | `DESIGN-rewrite.md` section 7 gives the hook; P22 and X25's P27 are candidates once steps 3 and 4 exist | Keep out until step 4 |

## 4. Process changes

1. **The rewrite's design reads the marks file.** `DESIGN-rewrite.md` section 5 copied the 30 tests of report 25, and one round later the marks file has 32. Section 5 can name the marks file and give only the counts by section, and `check_specification.py` can fail when those counts differ from the file's.
2. **The gate is a record once the rewrite has started.** The phase 16 decision says that the completion line keeps its counts as a measure and that a count over 50 stops nothing.
3. **A failing new test names its front end.** A test item's commit says, for each test that the old front end fails, whether it is marked, a candidate for the takeover list, or work for the old code.
4. **Two copies of a moved file change together.** A round that changes the old copy of a file that the new crate copied lists the change and makes it in both, until step 9.
5. **The three front-end documents keep their counts.** `DESIGN-current.md` (round 24, 51375 lines), `TEMPLATE-MODEL.md` section 8 (P3-6 and RV8 rows), and `DESIGN-rewrite.md` sections 4, 5, and 10 hold stale counts or closed points. Each front-end round brings them up to date, and the round report says so.
6. **The rewrite sets the first-error target.** The rate has stayed at 77% for seven phases. The new code's target for owned tests is at least the old rate on the same page.
7. **Keep the front-end report form.** Round 27 keeps report 26's tables and adds the `next:` lines of `DESIGN-rewrite.md` section 9 once item 1 prints them.
