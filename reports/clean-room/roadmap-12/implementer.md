# Roadmap, round 12: what to do after phase 14, from an implementer's view

Date: 2026-11-11. Decision 0029. Role: clean-room reader, acting as an implementer who builds a C++ front end and interpreter from Tidal alone and uses the specification as the design input. The evidence is the clean-room front end in `frontend/`, its reports and ledger in `reports/clean-room/rust-frontend/`, the rewrite preparation of item RP, and the phase 14 exercise records. Commit read: `bd87b27c`.

Inputs:
- AGENTS.md, CLAUDE.md
- `reports/clean-room/roadmap-11/implementer.md`, the model for this report's form, and its scratch files
- `documentation/`: status, roadmap (the phase 14 table), decisions 0060 and 0062, and `decisions/pending.md` (the table, the request, and the packet)
- `git log 45ee5e85..HEAD` with every commit body (14 commits)
- `frontend/DESIGN-current.md` and `frontend/TEMPLATE-MODEL.md`
- `reports/clean-room/rust-frontend/`: report 25 in full, `ledger.md`, and `conformance.tsv` at `1017b19c` (round 23), `53577f27` (round 24), and `b035efd4` (round 25)
- `reports/clean-room/I7/report.md` and `reports/clean-room/P3/report.md`
- `reports/clean-room/ledger.md` (the 26 open rows)
- the two phase 14 gap reviews in `reports/review/` (the Result sections and the headings)
- `questions/`: the status and Waits for lines of the questions that failing tests name
- `tests/`: the headers of the failing tests
- `tools/frontend_check.py`, and `frontend/src/` by line counts only

I did not read the other round 12 reports, `inputs/`, or `build/exercise-keys/`. I ran no compiler.

**Measurements.** I ran `tools/frontend_check.py` once under the run lock, and then `tidal-fe conform tests -v` once under the run lock with the binary that the check built. The check's lines match the phase 14 close commit:

```
frontend: 6804/6947 pass, 6831/6947 pass their check, 0 worse, 0 better, 0 moved between passing statuses, 9 new (6 not passing)
frontend: 21/21 interpreter-only expectations met
frontend: completion (decision 0060): 107 tests fail their check, 3/138 program tests do not run to their result, 9 tests out of scope
frontend: 2115/2743 passing rejects name a cited rule (1930) or a linked rule (185) in their first error
```

The rows that do not fully pass, two scripts, and their output are in `reports/clean-room/roadmap-12/implementer-scratch/`. `results.tsv` holds the 462 rows of `tidal-fe conform -v` that are not a plain pass. `causes.py` reads them with the round 23 and round 24 baselines. It takes the cause and the rewrite mark of each of the 104 tests from report 25 section 7, adds the three P3 tests by my judgment, sorts the 107 tests into cause groups, checks stability, and measures the gate. `questions.py` lists the open questions that the failing tests name. The cause groups are my judgment from report 25's causes.

## 1. Where the specification and the front end stand

| Measure | Round 11 (6881 tests, `3bd790ae`) | Now (6947 tests, `bd87b27c`) |
|---|---|---|
| Full pass | 6721 (97.7%) | 6804 (97.9%) |
| Check pass | 6745 | 6831 |
| Completion line (decision 0060) | 127 tests, 2 of 126 program tests, 9 out of scope | 107 tests, 3 of 138 program tests, 9 out of scope |
| `MISSED-REJECT` in scope | 69, and 6 that pass as `NO-DIAGNOSTIC-ALLOWED` | 45, and the same 6 |
| `unsupported` in scope | 51 | 50 |
| `FALSE-REJECT` in scope | 5 | 7 |
| `EDG-AGREES` (a failure) | 2 | 5 |
| `run-unsupported` | 24 | 27 |
| Run tests | 1796: 1761 pass, 0 wrong, 24 unsupported | 1834: 1793 pass (20 partial), 0 wrong, 27 unsupported, 14 not run because the check fails |
| Passing rejects whose first error names a cited or linked rule | 2087 of 2709 (77%) | 2115 of 2743 (77%) |
| Rules | 1698 | 1700 (DEP-40, LOOP-9) |
| Questions open | 180 | 185 |
| Front-end findings (G) | G1 to G163, 2 open | G1 to G165, 2 open (G164, G165) |
| Exercise ledger rows open | 26 | 26 (I7-1 to I7-3, I7-5 to I7-15, P3-1 to P3-12) |
| Front-end source | 51375 lines, 28 files | 52045 lines, 28 files; round 25 changed 17 files |
| Files over 2000 lines | 7 | 7: `expr.rs` 7928, `decl2.rs` 7198, `interp/expr.rs` 4860, `templ.rs` 4339, `decl.rs` 3178, `expr2.rs` 2894, `parser.rs` 2412 |

### What phase 14 took from round 11

| Round 11 item | Phase 14 result |
|---|---|
| 1. Front-end round 25 outside the template model | Done (`b035efd4`). The round passed 29 of round 24's 127 failures and 54 of 60 new tests. It fixed the TR-13 stop of `r10-list-first-try.cpp` (ICS-10 in the deduction of a converting constructor template) and gave P2-9's first line. |
| 2. Text for the I6 rows, G161, and G162 | Done in F4 (`1130ceb8`). I6-5, I6-6, I6-15, and P2-6 became Q-0434, Q-0432, Q-0433, and Q-0431. |
| 3. Rewrite preparation | Done in RP (`9359cc42`): `DESIGN-current.md` and `TEMPLATE-MODEL.md`. |
| 4. The grammar made usable by a parser | Done in RP: `translation_unit` is the start symbol, 17 productions carry a `tidal-grammar` mark, and `check_specification.py` fails on an unreached nonterminal. |
| 5. The gate for an early start | Partly done. Decision 0062 reads "few" as at most 50 and "stable" as no move from pass to fail. The `rewrite` ledger state and the third count were not adopted. Report 25 marks 30 tests in a table column, which no tool reads. |
| 6. Implement I7 | Done (`411ef68e`): 290 of 296 assertions, I7-1 to I7-17. Round 25 fixed both front-end faults (I7-16, I7-17). |
| 7. A review of the changed rules, DEP-24 first | Done in RV7 (`4dc57e81`, `b5d925e0`). Q-0427 (DEP-24) is supported with no doubt and waits for a use of DEP-P5. |

Decision 0062 also asks for a recorded read of each gap review's repair diff before the item ends. The RV7 reviews record a read of the RV6 diff. I found no file under `reports/review/` that records a read of the RV7 repair diff, `b5d925e0`. The roadmap marks RV7 done.

Of round 11's process changes, phase 14 took change 2 in part (report 25 section 7 names a cause for each test, in a Markdown table) and change 6. Changes 1 (the third count), 3 (a first-error target), and 5 (agreeing Implement programs become tests) are not in place. I7's 9 programs that GCC, Clang, and the front end agree on are not tests. I7 still took the conversion ranks and the rank of a braced list to an aggregate from outside knowledge (I7-15), so change 4 holds in part.

### The 107 tests of the completion line

| Measure | Round 22 | Round 23 | Round 24 | Round 25 | Now |
|---|---|---|---|---|---|
| In-scope tests that fail their check | 167 | 149 | 127 | 104 | 107 |
| Old failures passed in the round | | 18 | 32 | 29 | |
| New tests, and how many fail | | 105, 0 | 92, 10 | 60, 6 | 9, 3 |

98 of the 107 failed at round 24 with the same status, and 89 failed at round 23 too. The other 9 are tests added after round 24: F4's 6 and P3's 3. No test that passed at round 23 or 24 fails now. P3's 3 failing tests arrived after round 25, so report 25 does not name their cause.

By chapter and status:

| Chapter | Missed reject | Unsupported | False reject | EDG-AGREES | All |
|---|---|---|---|---|---|
| templates | 34 | 19 | 5 | 4 | 62 |
| classes | 4 | 7 | 0 | 1 | 12 |
| expressions | 2 | 8 | 0 | 0 | 10 |
| declarations | 1 | 6 | 1 | 0 | 8 |
| overloading | 1 | 7 | 0 | 0 | 8 |
| foundations | 2 | 2 | 0 | 0 | 4 |
| programs | 0 | 1 | 1 | 0 | 2 |
| statements | 1 | 0 | 0 | 0 | 1 |
| **total** | 45 | 50 | 7 | 5 | 107 |

By the class of the cited rules, 54 tests cite a `reject` rule, 44 cite only `required` rules, 5 cite an `impl-defined` rule (4 of them BF-9), 4 cite a `no-diagnostic` rule, and 1 cites an `undefined` rule (CE-3).

### The causes

`causes.py` puts each test in the first group that fits. The first five groups are report 25's "rewrite" marks, by the section of `TEMPLATE-MODEL.md` that report 25 names.

| Cause | Tests | Rules |
|---|---|---|
| Rewrite, model section 2: the current instantiation | 5 | DEP-24 (4, three false rejects), DEP-11 |
| Rewrite, model section 1: dependence marks | 4 | DEP-15 with DEP-30 or DEP-1 (2), DEP-4 (2, EDG-AGREES) |
| Rewrite, model section 3: a specialization, its point, and its order | 10 | INST-33 with INST-5 or INST-30 or INST-32 (6), INST-12 (2), INST-15, TSP-31 |
| Rewrite, model section 4: the immediate context | 4 | DED-24 (2), DED-29 (2) |
| Rewrite, model section 5: checks at the definition | 7 | CNS-29 (2), DEP-37 (2), PACK-8 (2), IF-22 |
| INST-5: a member's parts made with the class (P3-7, P3-8) | 3 | INST-5 with INST-7 (2), and `programs/P3/pipeline-first-try.cpp` |
| Class template argument deduction | 10 | CTAD-1 (4, with TPL-11 in two), CTAD-2 (2), CTAD-12, CTAD-19, CTAD-29 with CTAD-11, and `r10-tuple-second-try.cpp` |
| Constant evaluation of a construct | 12 | NEW-2 (3), NEW-1, CVF-1 and DCL-5 (a class in an array bound), UNI-7 and LIST-4 (brace elision), PRIM-27, CE-3, LSUP-13, OVL-40 |
| Template checks outside the model (missed rejects) | 14 | TARG-24 (2), CNS-24 to CNS-27 with lambdas (2), TARG-15, TARG-7, CPT-37, CNS-13, TPL-31, TPL-7, INST-21, ORD-3, PACK-26, TSP-23 |
| Template constructs outside the model (unsupported) | 7 | TARG-14, CPT-1, CPT-20, CNS-30, DEP-20, PACK-3, PACK-5 |
| Missed rejects in other chapters | 10 | DCMP-11, DCMP-3, CTOR-12, CPY-15, CEV-1, UNEV-1, PFX-11, CE-3, NAM-8, OVL-20 |
| An explicit conversion function to a reference | 5 | ICS-45 |
| The `noexcept` operator before a specification is known | 4 | CLS-2, EXS-1, EXS-6 (2) |
| BF-9 layout of wide bit-fields | 4 | BF-9 (one on ILP32) |
| Unsupported constructs in other chapters | 5 | AUTO-7, COND-3, LAM-6, LAM-18, ICS-47 |
| EDG-AGREES outside the model | 2 | CPY-27, CNS-32 |
| False reject | 1 | SB-5 |
| **total** | 107 | |

Report 25 marks 30 tests as rewrite work: 17 missed rejects, 7 unsupported, 3 false rejects, and 3 EDG-AGREES. 24 of the 30 were present at round 24, and the 6 others are F4's tests of DEP-4 and INST-33. The model group was 16 at round 11 and is 30 now. Every test that F4 added and the front end fails is in it.

The three P3 tests are false rejects of a base class template that takes the derived class as its argument. The front end makes a member's return type and a constructor body while the derived class is incomplete. INST-5 does not make a constructor body with the class, and P3-6 asks the text to say the same for a member's return type. Model section 3 gives the order of a specialization's parts (INST-P2). The current front end already keeps the bodies of an instance's members for a use (`DESIGN-current.md`, "Lazy bodies of instances"). So I count the three outside the marks, and round 26 marks them if the current design cannot pass them.

19 of the 107 tests rest on an answer that can still change. 14 name an open question: Q-0427 (3, waits for a use of DEP-P5), Q-0432 (2, a use of DEP-P2), Q-0337 (2) and Q-0156 (1) on ICS-45, Q-0079 (2, the EXS-6 cycles), Q-0433, Q-0235 (INST-21, `pending.md` rows 3 and 5), Q-0339 (CPY-27), and Q-0406 (PFX-11, row 4). Five more cite a row 4a rule item: ICS-45 (2 more), DEP-15 with DEP-P4 (2), and TR-13 (1).

Of the 20 rules and principles that changed after RV7 read the tree, failing tests cite DEP-4 (2), DEP-30 (1), and CNS-13 (1). The INST-1, INST-2, and INST-P1 changes bear on the INST-5 group and on model section 3.

### Run stops

The 27 run stops pass their check, so the completion line leaves them out. They are an overload set (3: CVF-1, CVF-4, CVF-5), a class assignment (3: OVL-7 from P3-10, OVL-42, and `programs/P3/events-first-try.cpp`), a `new` expression (4: LAM-19, NEW-24 with DED-39, DED-39, DED-39 with ORD-9), a variable or function with no definition in the unit (3: VIRT-12, SINIT-1, ODR-15), a missing default argument (2: NEW-23, TSP-8), a destructor with a function try block (2: TRY-10), a dependent name (2: DEP-24 with DEP-25, DEP-20), an unknown callee (2: TARG-12, DEP-11), a range-based `for` over a base over the derived class (1: RFOR-2 from P3-9), and 5 single causes (DCMP-4, CVF-1, LNK-9, UN-7 with STG-15, INST-36). Four of them (DEP-24, DEP-20, DEP-11, INST-36) sit in the dependent-names and instantiation pages, where the model's limits are.

### Decision 0060 and the gate of decision 0062

The completion line counts 107 tests and 3 program tests: `r10-tuple-second-try.cpp` (class template argument deduction), `pipeline-first-try.cpp` (P3-7), and `events-first-try.cpp` (a run stop, P3-10).

The gate of decision 0062 has three parts.

- **Few.** The part does not hold. The count is 107, and the gate is 50. The old code has to pass 57 tests with no new failure.
- **Stable.** The part holds. No test moved from a passing status to a failing one in round 24, round 25, or since.
- **A named cause.** The part holds for the 104 tests of round 25. The three P3 tests have causes in P3's report and in this report.

The 30 marked tests are part of the 57. Report 25 says the current design cannot take them, and `TEMPLATE-MODEL.md` says what can. So the gate as written asks the old code to pass at least 57 of the 77 other tests, and every phase adds template tests that join the marked group. Rounds 23 to 25 passed 18, 32, and 29 old failures. At 30 a round, the gate as written holds after round 27 at the earliest, in phase 16, if no phase adds a failing test.

Decision 0060's own completion test needs all 107 tests and the 3 program tests. The 30 marked tests would need the template model built in the old code, which is the work the rewrite exists to do. Under decision 0060 as written, the rewrite starts only after the old code has done its hardest part.

With the 30 marked tests left out of the count, 77 tests remain, 27 over the gate. Round 26 at round 25's rate leaves about 48, plus the failing tests that phase 15 adds.

### Where the rewrite stands

- **The template model.** `TEMPLATE-MODEL.md` gives a typed dependent tree, the current instantiation as a class with known members, a specialization as a substitution, substitution failure apart from hard errors, and the checks at the definition. Its section 6 covers the 16 tests of round 11. Report 25 maps 14 more to its sections. The model assumes a definition is parsed once into a tree (section 1). The current front end has no syntax tree between parsing and analysis (`DESIGN-current.md`, "Parsing and semantic analysis"), so the model needs a parse that is separate from analysis.
- **The parser.** The grammar has a start symbol and marks. The RP commit records one gap: the `forward_class` shape allows attributes after the class key, and `elaborated_type` does not. No ledger row or later commit holds the gap.
- **The design document of the rewrite.** None yet. `DESIGN-current.md` describes the old code at round 24, before round 25 changed 17 files.
- **Unit tests per stage.** None exist.
- **The first-error rate.** It stays at 77% for a sixth phase. 628 passing rejects name another rule first.

### The I7 and P3 findings

| Rows | Who acts | Why the front end cares |
|---|---|---|
| I7-1 (CTAD-10, LIST-4), I7-2 (CTAD-25, CTAD-26), I7-3 (CTAD-16, DER-8) | text, with examples | The deduced type's initialization is a second overload resolution. The front end's `ctad.rs` needs the rule before round 26 changes it. |
| I7-6 (CTAD-18), I7-11 (CTAD-17, OVL-14), I7-13 (CTAD-2), I7-14 (CTAD-5) | text | The guide set and the sites. Two CTAD-2 tests are missed rejects, and I7-13 names two more sites. |
| I7-5, I7-7 to I7-10, I7-12, I7-15 | text or a link | Examples and definitions. No failing test depends on them. |
| P3-6 (INST-1, INST-5) | text | When a member's return type is made. The three P3 false rejects depend on it, and model section 3 follows it. |
| P3-7 to P3-10 | front end | Each has a test. P3-7 and P3-8 fail their check, and P3-9 and P3-10 stop in a run. |
| P3-1 to P3-5, P3-11, P3-12 | examples, glossary | No front-end work follows. |
| G164 (STG-28), G165 (VIRT-13) | text | The front end made a choice for each, and its test passes. |

## 2. Ranked items

| Rank | Item | Evidence | Size | Prerequisites |
|---|---|---|---|---|
| 1 | Front-end round 26 on the 77 unmarked tests and the run stops | 77 unmarked tests; 27 run stops; P3-7 to P3-10 | large | none |
| 2 | An amendment to the gate: a `rewrite` state, a third count, and a takeover list | gate 57 over; 30 marked tests the old code cannot pass; the model group grew from 16 to 30 | small | none |
| 3 | Rewrite stage 1: the design document, the lexer, and the parser from the Syntax segments, with unit tests | decision 0060's list; model section 1 needs a parse tree; the grammar has a start symbol | large | 2 |
| 4 | Text for the I7 and P3 rows, G164, and G165, P3-6 first | 14 open I7 rows, 12 open P3 rows, 2 open G rows | small each | none |
| 5 | A gap review of the 20 changed rules, with the read of the RV7 repair diff | DEP-4, DEP-30, CNS-13 have failing tests; no record of a read of `b5d925e0` | medium | 4 for its text |
| 6 | Implement I8 on instantiation: the points and the order of a specialization's parts | 12 failing tests cite INST rules; INST-1, INST-2, INST-P1 changed after RV7 | medium | none |

### 1. Front-end round 26 on the 77 unmarked tests and the run stops

**What it is.** A round in the form of report 25, with its section 7 table kept. The groups, in order:

1. **The program tests and the P3 faults.** INST-5 for a member's return type and a constructor body (P3-7, P3-8, three tests), the range-based `for` over a base over the derived class (P3-9), and a user `operator=` with a class parameter (P3-10, and `events-first-try.cpp`). If the current lazy bodies cannot pass the INST-5 tests, the round marks them for model section 3.
2. **Class template argument deduction (10).** `r10-tuple-second-try.cpp` and CTAD-29 with CTAD-11 first, then the CTAD-1 and CTAD-2 sites, CTAD-12, and the CTAD-19 tie. Item 4's text for I7-2 and I7-13 helps.
3. **Unsupported groups with several tests.** Constant evaluation of a construct (12), ICS-45 (5), the `noexcept` operator (4; the two EXS-6 cycles pass as `NO-DIAGNOSTIC-ALLOWED` once the front end accepts them), and BF-9 layout (4).
4. **Template checks outside the model (14) and template constructs outside the model (7).**
5. **Other chapters.** The 10 missed rejects, the 5 unsupported constructs, SB-5, CPY-27, and CNS-32.
6. **Run stops (24 after group 1).** The overload set, the class assignment (OVL-42), the `new` forms, the missing default argument, the destructor function try block, and the variables with no definition in the unit.
7. **Marks.** A test whose cause the round finds in the template model gets a `rewrite` row (item 2) that names the section of `TEMPLATE-MODEL.md`.

The 30 marked tests wait for the rewrite.

**Evidence.** Section 1. Rounds 23 to 25 passed 18, 32, and 29 old failures, every remaining test has a cause, and report 25 says the current design can take the 74 unmarked tests of its table.

**Size.** Large. Groups 2 and 3 are medium. The others are small to medium.

**Prerequisites.** None. Item 4's tests join when they land.

**Risk.** Low. A group that the front end cannot close from the text becomes a G row with a test.

### 2. An amendment to the gate: a `rewrite` state, a third count, and a takeover list

**What it is.** The phase 15 decision amends decision 0062's reading of decision 0060.

- The front-end ledger gains a state `rewrite`. A `rewrite` row names its tests and the section of `TEMPLATE-MODEL.md` that passes each. Report 25's 30 marks become such rows.
- `tools/frontend_check.py` prints the tests that `rewrite` rows name as a third count on the completion line, apart from the in-scope count, as it does for `out of scope` rows.
- "Few" counts the in-scope tests that no `rewrite` row names. The limit stays at 50. "Stable" and "a named cause" stay as they are.
- Rewrite stage 1 (item 3) starts before the gate holds, because it takes over no test and changes no result. A stage that takes over tests waits for the gate.
- The takeover list. A file beside `conformance.tsv` names the tests that the new front end owns. `frontend_check.py` runs both front ends, takes the new one's status for an owned test, and fails on a test that either front end makes worse. The completion line counts owned tests by the new front end's status. Decision 0060's rule stays: the old code goes when the new code owns every test that the old code passed.

**Evidence.** Section 1. The marked tests are 28% of the count. The old code cannot pass them without first building the model that the rewrite brings. Each phase's template tests add to them. A count that includes them keeps the gate closed until the old code has done the rewrite's work.

**Size.** Small. `frontend_check.py` reads one more state and one list.

**Prerequisites.** None.

**Alternative.** Keep the gate as written and run rounds 26 and 27 on the old code. The gate then holds in phase 16 at the earliest. I rank the amendment first, because the old code's work on the 30 marked tests would be thrown away, and stage 1 gains a phase.

### 3. Rewrite stage 1: the design document, the lexer, and the parser from the Syntax segments, with unit tests

**What it is.** A clean-room writer starts the new front end in a crate of its own beside the old one, under decisions 0029, 0035, and 0060.

- **The design document** of the new code: the stages (lexing, parsing to a syntax tree, analysis to typed trees with the dependence marks of model section 1, constant evaluation, interpretation), the data each passes, a module for each page of the specification, and a target for the first-error rate.
- **The lexer and the preprocessing slice**, from the lexical rules and the ODR page's grammar.
- **The parser** from the Syntax segments, with `translation_unit` as its start symbol. The marked shape productions become checks on the tree. The `forward_class` gap of the RP commit gets a row and an answer.
- **Unit tests** for each stage, and a parse check over `tests/`: each test whose outcome is not a parse error parses to a tree, and each test that cites TR-12 alone fails to parse. `frontend_check.py` prints the count and takes over no test.

The first semantic stage (declarations, types, names, classes, overloading, and the template model) waits for the gate of item 2. It takes over the 30 marked tests first, then the templates pages, one page at a time:

| Model section | Tests the rewrite takes over |
|---|---|
| 1, dependence marks | `other-specialization-constant-argument-member-missing.cpp`, `other-specialization-constant-argument-no-typename.cpp`, `sizeof-value-dependent-operand-call-bound-early.cpp`, `sizeof-value-dependent-operand-no-typename.cpp` |
| 2, the current instantiation | `current-instantiation-enclosing-base.cpp`, `member-template-current-instantiation-pointer.cpp`, `member-template-current-instantiation-qualifier-dependent-base.cpp`, `member-template-non-dependent-base.cpp`, `member-template-through-current-instantiation-object.cpp` |
| 3, a specialization | `class-point-not-later-member-type.cpp`, `class-point-not-later-type.cpp`, `class-point-not-later.cpp`, `class-point-static-const-member.cpp`, `class-point-static-constexpr-made-at-use.cpp`, `class-static-constexpr-member-initializer-waits.cpp`, `local-class-default-argument.cpp`, `local-class-exception-specification.cpp`, `recursion-endless-function.cpp`, `partial-no-primary-variable.cpp` |
| 4, the immediate context | `substitution-explicit-specifier.cpp`, `substitution-lambda-body.cpp`, `substitution-lambda-default.cpp`, `substitution-member-wrong-kind.cpp` |
| 5, checks at the definition | `constexpr-template-nondependent-call-checked.cpp`, `out-of-class-other-clause.cpp`, `out-of-class-without-clause.cpp`, `binary-operator-pack-never-fits.cpp`, `operator-pack-never-fits.cpp`, `fold-in-outer-pattern.cpp`, `size-operator-in-outer-pattern.cpp` |

The full paths are in `implementer-scratch/causes-output.txt`. The three P3 INST-5 tests join section 3 if round 26 marks them.

**Evidence.** Section 1. Decision 0060 lists a design document, modules by page, a parser from the Syntax segments, and unit tests. RP gave the template model, the grammar's start symbol, and a description of the old code. Model section 1 needs a definition parsed once into a tree, and the old code parses and analyzes in one pass with no tree between. The parser is the stage that every later stage needs, and it changes no test result.

**Size.** Large. The design document and the lexer are medium.

**Prerequisites.** Item 2.

**Risk.** Medium. Two code bases run for several phases. The takeover list and the 0 worse check keep them from disagreeing without a record.

### 4. Text for the I7 and P3 rows, G164, and G165, P3-6 first

**What it is.** The clean-room writer acts on each row, in this order:

1. **P3-6.** INST-5 says when the parameter and return types of a member definition are made. The three P3 tests depend on it, and model section 3 follows it.
2. **I7-2 and I7-3.** CTAD-25 says that the deduced type's constructors are chosen again, and CTAD-16 notes that the inherited copy candidate can give a type whose initialization fails (DER-8). The note of `inherited-copy-candidate-user-guide.cpp` gets I7-3's correction.
3. **I7-1.** CTAD-10 says that a member whose type is not dependent follows LIST-4.
4. **I7-6, I7-11, I7-13, I7-14.** A definition of an initializer-list guide, the copy-initialization limit on guides, the two sites of CTAD-2, and when a conditional `explicit` is evaluated. Each gets a probe on the matrix and a test where the compilers agree.
5. **G164 and G165.** STG-28 names the element at a byte offset into allocated storage. VIRT-13 says whether the warning concerns each hidden base function.
6. **I7-5, I7-7 to I7-10, I7-12, I7-15.** Examples, definitions, and links.
7. **P3-1 to P3-5, P3-11, P3-12.** The programmer view ranks them. They need no front-end work.

**Evidence.** Section 1, "The I7 and P3 findings".

**Size.** Small each. I7-6 and I7-11 may need a question.

**Prerequisites.** None.

### 5. A gap review of the 20 changed rules, with the read of the RV7 repair diff

**What it is.** A source-reader gap review of the 20 rules and principles that changed after RV7 read the tree, and of item 4's text. It records a read of `b5d925e0`, hunk by hunk, as decision 0062 asks for each repair diff. It reads DEP-4, DEP-30, CNS-13, and INST-1, INST-2, and INST-P1 first.

**Evidence.** Section 1. Four failing tests cite DEP-4, DEP-30, or CNS-13, and model sections 1, 3, and 5 follow those rules. No file records a read of the RV7 repair diff.

**Size.** Medium.

**Prerequisites.** Item 4 for its text.

### 6. Implement I8 on instantiation

**What it is.** A fresh clean-room reader builds a component from `templates/instantiation.md` (INST-P1, INST-P2, and the INST rules), filtered of Evidence segments and `:tests:` lines as in I4 to I7. The component takes a unit's uses of a class template and its members in order, and returns which parts of each specialization are made, at which point, and what lookup at each point sees. The prompt gives the callbacks' answers for lookup and deduction (round 11 process change 4).

**Evidence.** 12 failing tests cite INST rules: 9 of the 10 marked tests of model section 3, the 2 P3 fault tests, and an INST-21 test. `pipeline-first-try.cpp` fails on the same cause as the P3 tests. INST-1, INST-2, and INST-P1 changed after RV7. P3 found the INST-5 gap through a program, and I5 to I7 each found front-end faults through programs. Model section 3 is the rewrite's largest group, and a blind build checks it before code.

**Size.** Medium.

**Prerequisites.** None. Item 4's P3-6 text helps.

**Alternative.** Constraint normalization and subsumption (CNS-P1). Ten failing tests sit on the constraints and concepts pages, two of them marked. I rank instantiation first because its group is larger and the rewrite meets it first.

### Dependency sketch

```
1 round 26 ───────────────────────────────> gate after round 26 (the count)
2 amendment ──> 3 rewrite stage 1 (parser) ──> semantic stage, after the gate,
                                               takes over the 30 marked tests
4 text (P3-6) ──> 1 round 26 (INST-5 group)
4 text ──> 5 review
6 Implement I8 ──> semantic stage (model section 3)
```

Items 1, 2, 4, and 6 can start together. Item 3 starts after item 2.

## 3. Scope

| Topic | Charter's reason to keep out | Implementer's view | Proposal |
|---|---|---|---|
| `<cstdarg>`, named escapes, `#include <...>`, extended floating-point types | the library, a name table, real headers, *(referenced)* | G42 and G158 to G160 hold 9 tests; the completion line leaves them out | Keep the rows |
| ILP32 | in the matrix (EDG ILP32) | One BF-9 test remains; DCL-19 on ILP32 passes since round 25 | Keep in scope |
| `no-diagnostic` rules | the class needs no diagnostic | 6 tests pass as `NO-DIAGNOSTIC-ALLOWED`; the EXS-6 cycles pass once the front end accepts them | Keep the status |
| Coroutines and lazy generators (P3-1) | outside the language goal | A program with a computed sequence writes its own iterator; the front end runs it | Keep out; an example only |
| Threads, the memory model, modules | outside the language goal | No test or front-end path needs them | Keep out |
| Later language versions (`pending.md` row 7) | the charter fixes C++23 | The front end's work would not change | Keep out |
| MSVC | not in the matrix | A second ABI would double the layout tables | Keep out |
| Propose designs | not part of the rules | The rewrite's design document is the place where they would cost least | Keep out until the semantic stage |

## 4. Process changes

1. **A `rewrite` state and a third count** (item 2). A cause in a report table helps no tool. A ledger row does.
2. **Causes in the baseline.** `conformance.tsv` can carry the cause group and the rewrite section of each failing test in its third and fourth columns. `frontend_check.py` ignores later columns, and `--update` writes only two, so `--update` should keep them. Each round then updates the causes in place of a new table.
3. **A gap review item ends with the read of its repair diff on file.** The RV7 item closed with no record of a read of `b5d925e0`. `check_specification.py` can fail when a gap review's repair commit, named in the ledger, has no review file that names it.
4. **A fault that an exercise finds in the template area names the model section.** P3-7 and P3-8 have tests, and no record says whether the model or the current design takes them.
5. **The rewrite sets the first-error target.** The rate has stayed at 77% for six phases. The design document of item 3 sets the target for the new code, and the old code stops reporting the rate per page when the new code owns the page.
6. **Implement programs that agree become program tests.** I7's 9 programs that GCC, Clang, and the front end agree on are not tests. Round 11 asked for this change, and it is not in place.
7. **Keep the front-end report form.** Round 26 keeps report 25's tables, its completion section, and its measures for the rewrite, and adds the parse check of item 3 when it exists.
