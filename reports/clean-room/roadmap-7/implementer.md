# Roadmap, round 7: what to do after phase 9, from an implementer's view

Date: 2026-11-05. Decision 0029. Role: clean-room reader, acting as an implementer who builds a C++ front end and interpreter from Tidal alone. The evidence is the clean-room front end in `frontend/`, its reports in `reports/clean-room/rust-frontend/`, and the Implement exercises I1 and I2 in `reports/clean-room/I1/` and `reports/clean-room/I2/`. Commit read: `017b401`.

Inputs:
- AGENTS.md, CLAUDE.md
- `documentation/`: charter, roadmap, status, style guide, specification format, exercises, decisions 0033, 0039, 0044, 0050, 0051, 0054, 0055, 0056, and `decisions/pending.md`
- `reports/clean-room/roadmap-6/implementer.md`, the model for this report's form
- `reports/clean-room/rust-frontend/` reports 20 and 21, `conformance.tsv`, `examples.tsv`, and `frontend/README.md`
- `reports/clean-room/I1/` and `reports/clean-room/I2/` (reports, notes, score output)
- the phase 9 exercise records R9, PU1, PU2, and X21, read through their reports and the commit messages that used them
- `reports/review/`: the three 2026-11-04 gap reviews of the text phase 8 wrote (headings and Result sections)
- `specification/` (the rules cited below), the headers of `tests/`, and `questions/README.md`
- `git log 1b16115..HEAD` with commit bodies (17 commits; `e2f0de5` is round 6 itself)

**Measurements.** Every count comes from commit `017b401`, with a clean tree. I built the front end with `cargo build --release --locked --target-dir` into `reports/clean-room/roadmap-7/implementer-scratch/`, so the repository's build directory was not touched. I ran `tidal-fe conform tests -v` once. Its status counts match `conformance.tsv` exactly. I compared `conformance.tsv` at `1b16115` with the file at `017b401` to see which failures are old. Other counts come from grep and short Python scripts in the scratch directory, so they are approximate. Statements from general knowledge say so.

## 1. Where the specification and the front end stand

| Measure | Round 6 (6500 tests, `1b16115`) | Now (6619 tests, `017b401`) |
|---|---|---|
| Full pass | 6183 | 6362: 6089 `pass`, 254 `EDG-ONLY`, 10 `EDG-ALLOWED`, 9 `pass-partial` |
| Check pass | 6247 | 6414 |
| `EDG-AGREES` (a failure) | 21 | 2 |
| False rejects | 12 | 6 |
| Missed rejects | 135 | 118 |
| Unsupported (check) | 85 | 79 |
| `NO-BASE` (no base `tidal-expect` line) | not measured (3 tests passed on either answer) | 0 |
| Run tests | 1620: 1546 pass (6 partial), 7 wrong, 57 unsupported, 10 not run | 1660: 1603 pass (9 partial), 0 wrong, 0 missed or unexpected UB, 52 unsupported, 5 not run |
| Interpreter-only expectations | 19, all met | 19, all met |
| Passing rejects whose first error names a cited or linked rule | 1748 by the harness; 1753 of 2521 by my script (70%) | 1990 of 2593 (77%): 1813 cited, 177 linked, 602 another rule, 1 no rule |
| `reject` rules absent from the front end's source | 107 | 103 of 558 |
| Page examples | not run | 175 of 177 pass; 2 unsupported |
| Rules by kind | 1696 | 1697: 898 required, 558 reject, 57 undefined, 52 impl-defined, 43 recommended, 37 no-diagnostic, 32 unspecified, 5 conditionally-supported, 15 withdrawn |
| `unspecified` rules with a choice point or a fixed order | 9 of 31 | 9 of 32 by my count; LAM-28 became `unspecified` |
| Questions | 385: 221 provisional | 400: 172 provisional, 13 compiler defect, 215 settled |
| Front-end findings | G134 to G141 | G142 to G151 |

### What phase 9 took from round 6

| Round-6 item | Phase 9 result |
|---|---|
| 1. Front-end round 20 | Done in rounds 20 and 21. Of the 24 phase 8 repair tests, 22 pass. `ellipsis-argument-conversions.cpp` fails at `va_list`, and `implicit-conversion-member-and-new-contexts.cpp` is unsupported. Wrong run results fell from 7 to 0. Every program test runs. |
| 2. G134, G135, and repair checks | Done (item FH, `2ebb5c3`). `tidal-allows-edg` exists, and a test with no base line is `NO-BASE`. Decision 0056 makes a repair commit name the next front-end round. |
| 3. Rule-ID scoring per page | Done (item FH). `tidal-fe conform` prints the table, with linked rules counted as decision 0052 does for run stops. TYP-7 now names its rule. |
| 4. Implement I2 | Done. 140 of 140 ordering checks, 22 unstated steps in `notes.md`, 17 findings, all acted on in `d9c18bc` and `593f9e5`. |
| 5. The library surface | Not taken. The seven C functions are still only in `frontend/README.md`. Two tests now fail at `va_list`. |
| 6. Labels for the `unspecified` rules | Not taken. |
| 7. Check-side round on missed rejects | Not taken as an item. Rounds 20 and 21 closed old failures where a repair or a review touched the same rule. |

### The old failures did not move

Phase 9 added 122 test files, and 119 of them are tests with a status row. 116 of the 119 pass on the front end. The three that fail are `across-units-class-member.cpp` (the interpreter panics), `ellipsis-nullptr-becomes-void-pointer.cpp` (`va_list`), and `nondeduced-only-parameter.cpp` (ORD-3).

The other 254 failures were already failures at `1b16115`. Rounds 20 and 21 fixed 63 old failures, mostly where RV2, I2, or a G finding changed the rule. The rest stayed as they were:

| Status at `1b16115` and now | Tests |
|---|---|
| `MISSED-REJECT` both times | 117 |
| `unsupported` both times | 79 |
| `run-unsupported` both times | 51 |
| `FALSE-REJECT` both times | 5 |
| `EDG-AGREES` both times | 2 (CPY-27, CNS-32) |

So every check-side unsupported test, and all but one missed reject, dates from round 6 or earlier.

### Failures by page

The failure rate per test directory (all statuses other than a full pass):

| Directory | Failing | Rate | Largest causes |
|---|---|---|---|
| templates/packs | 9 of 69 | 13.0% | PACK-8 (3 missed), the `same` idiom |
| classes/conversion-functions | 8 of 62 | 12.9% | 4 run stops at an overload set or a conversion function |
| overloading/overload-resolution | 19 of 168 | 11.3% | conversion to a function reference (6 unsupported), OVL-20 with no target (5 missed) |
| expressions/new-delete | 13 of 128 | 10.2% | `new` in a constant (4) and in a run (6) |
| overloading/conversion-sequences | 13 of 130 | 10.0% | ICS-45 under a concept (5), conversion functions in constants (3) |
| statements/range-for | 3 of 33 | 9.1% | |
| classes/access | 6 of 69 | 8.7% | ACC-3, ACC-15 after overload resolution |
| templates/dependent-names | 10 of 122 | 8.2% | DEP-37, run stops at a dependent name |
| templates/instantiation | 11 of 138 | 8.0% | 9 missed rejects, each a different INST rule |

By chapter, overloading has the highest rate: 32 of 298 tests (10.7%), 21 of them unsupported. Templates follow with 73 of 1180 (6.2%), 43 of them missed rejects.

The causes that the harness names for the 79 check-side unsupported tests:

| Construct | Tests |
|---|---|
| `static_assert` on a variable template with unknown arguments (the `same` idiom) | 22 |
| conversion to a function reference (constant initializer and `static_assert`) | 12 |
| conversion functions in a constant | 10 |
| a concept in `static_assert` | 8 |
| `new` in a constant | 6 |
| the `noexcept` operator | 4 |
| `sizeof` of a class with bit-fields that the layout code does not compute (BF-9) | 4 |

The 52 run-unsupported tests stop at `new` forms (8), constructor calls (6), jumps into a nested statement (3), variables of an enclosing function (3), and overload sets (3).

### Places where the text or the record still stops the implementer

1. **Five front-end findings have no answer.** Report 21 gave G146 to G151 on 2026-11-04. G146's case went into ORD-9 through R9D-1 in `32c89f7`. No commit, question, or page cites G147 (ORD-6 for members of two classes), G148 (COND-16 and direct binding to a class prvalue), G149 (INST-12 and a failure in a waiting requires clause), G150 (CMP-8 and a member of a base subobject), or G151 (BF-1 and ALN-8 with explicit conversion functions). `frontend/README.md` records the front end's own choice for G146, G147, and G149. A second implementer has to find these choices in that README. This is the same gap that G134 and G135 showed in round 6.

2. **The interpreter panics on one test.** `across-units-class-member.cpp` (SINIT-3, SINIT-2) ends with an index out of bounds in `src/interp/mod.rs`. The harness reports it as `run-unsupported` with "interpreter crashed". Report 21 says the run looks up a class of the second unit among the classes of the first.

3. **The library surface is still a front-end choice.** The seven C functions (`_Exit`, `exit`, `abort`, `malloc`, `free`, `puts`, `putchar`) are listed only in `frontend/README.md`. Two PFX-22 tests include `<cstdarg>`, and both are false rejects at `va_list`. The PFX-22 Evidence says that EDG has no `va_list`.

4. **Rule IDs on rejects.** The rate is 77% by the harness. The lowest pages by first error:

   | Page | Named (cited or linked) | What the front end names instead |
   |---|---|---|
   | foundations/values | 0 of 9 | other rules on all 9 |
   | expressions/decltype | 2 of 12 | |
   | foundations/conversions | 7 of 20 | INIT-7 for CONV-8 and CONV-9 |
   | expressions/member-pointers | 11 of 29 | CONV-17 for MPTR-11 |
   | foundations/language-support | 8 of 17 | |
   | templates/instantiation | 34 of 70 | SA-1 or NAM-16 where the test cites INST-1, INST-6, or INST-7 (about 30 tests) |
   | foundations/names | 47 of 94 | NAM-9 for NAM-14 and NAM-18 |

   The rules named most often in place of a cited rule are TR-12 (44), NAM-8 (37), INIT-7 (37), OVL-35 (25), NAM-16 (23), and SA-1 (18). Most pairs are a general rule and a specific rule that both reject the program. The instantiation tests are one pattern: the test cites the rule that makes the instantiation happen, and the front end names the error inside the instantiation.

5. **The `unspecified` rules have no labels.** Round 6 counted 22 `unspecified` rules with no choice point, no fixed order, and no label that says whether a test can observe them. Phase 9 added no label. LAM-28 became `unspecified` in RV2, and the interpreter's source does not name it, so 23 of the 32 rules are in that state by my count. SEQ-8 still explores two options per operator. Nine program runs are `pass-partial`.

6. **Two procedures have had a blind build.** I1 (class layout) and I2 (partial ordering) each found steps that Predict and many front-end rounds had passed: 13 findings in I1, 22 unstated steps and 17 findings in I2. The procedure pages that have not had one are overload ranking (ICS-P5, OVL-P3), deduction from a call (DED-P1), class template argument deduction, constraint normalization and subsumption, and the class-operand steps of the conditional operator (COND-16). The overloading pages hold 20 provisional questions between them, the most of any two pages.

## 2. Ranked items

| Rank | Item | Evidence | Size | Prerequisites |
|---|---|---|---|---|
| 1 | Implement I3: ranking of conversion sequences and the best viable function, blind | overloading fails 32 of 298 tests (10.7%), the highest chapter rate; 20 provisional questions on its two pages; ICS-P5 is already a step diagram; I2 found 22 unstated steps | medium | none |
| 2 | A ledger of front-end findings, and answers for G147 to G151 | five findings with no commit or question; three front-end choices live only in the README | small | none |
| 3 | Front-end round 22: the old check-side and run-side backlog | 117 of 118 missed rejects and all 79 unsupported tests date from round 6 or earlier; one interpreter panic | large | 1 for the overloading groups, 2 |
| 4 | Implement I4: the conditional operator with class operands (COND-16) | G148, PU1-7, PU2-3, and the RV2 review all name COND-16; Q-0378 stays open; 5 of 67 conditional tests fail | small | none |
| 5 | Rule boundaries on the lowest pages, with links for the general rules | 77% rate; values 0 of 9, decltype 2 of 12, conversions 7 of 20; about 30 instantiation tests name the inner error | small | none |
| 6 | The library surface and the variadic facility | 7 C functions in a README; 2 false rejects at `va_list`; round 6 item 5 not taken | small | none |
| 7 | Labels for the 32 `unspecified` rules, then allowed results | 23 rules unlabelled; SEQ-8 has two options; 9 partial runs; round 6 item 6 not taken | small (labels), medium (results) | none |

### 1. Implement I3: ranking of conversion sequences and the best viable function, blind

**What it is.**
- A fresh clean-room reader builds one component under the Implement prompt in `exercises.md` section 9. The blind inputs are the conversion-sequences page and the overload-resolution page, with Evidence segments and `:tests:` lines removed, and the glossary. The reader does not read `tests/` or `frontend/`.
- The component takes two candidate functions, each with its parameter types, and a list of argument types and value categories. It forms each implicit conversion sequence by ICS-P1 to ICS-P4, compares the sequences per argument by ICS-P5, and applies OVL-P3 across the arguments and through the tie-breakers. It returns the better function, or neither.
- The scoring phase turns the ICS and OVL tests and the 23 grid files under `tests/overloading/*/grids/` into calls of the component, as I2 did for the ordering grids.
- Partial ordering enters through OVL-P3 as one tie-breaker. The reader can treat ORD as a black box that the call supplies, so I3 does not repeat I2.

**Evidence.**
- Overloading has the highest failure rate of any chapter on the front end (section 1). 21 of its 32 failures are unsupported, and most of those stop at a conversion function or a conversion to a function reference.
- `conversion-sequences.md` holds 12 provisional questions and `overload-resolution.md` holds 8. Several of them are about the order of the steps (Q-0133 for step 1 against step 2, Q-0152 for 4d against 4f, Q-0148 for the kind before the template tie-breaker).
- ICS-P5 already presents the comparison as a numbered diagram, as decision 0044 asks. It also states which steps never decide the same case. A blind build tests whether those statements hold for a reader who has no tests.
- I2 found that the ordering page left out how values combine across pairs (I2-1) and how packs pair (I2-2). Predict batches R-conversion-sequences (129 of 129) and R-overload-resolution (171 of 171) check one outcome per program and would not find such a gap.

**Size.** Medium. The component needs a type model with references, cv-qualifiers, pointers, arrays, classes with bases and conversion functions, and list initialization. I2's type model covered about half of this.

**Prerequisites.** None. I3 should run before front-end round 22 works on the overloading groups, so the blind reader's findings come first.

**Risk.** Medium. General knowledge: overload ranking is a part of C++ that many programmers know by rule of thumb, so a reader may fill gaps from memory. The prompt should ask the reader to log each step the text did not give before reading any test, as I1 and I2 did.

### 2. A ledger of front-end findings, and answers for G147 to G151

**What it is.**
- A table of every G finding with its state: acted on in a commit, recorded as a question, or open. The table can live in `reports/clean-room/rust-frontend/` or in `questions/`.
- `tools/question_index.py` or `tools/check_specification.py` warns when a G finding in a front-end report has no ledger row.
- The clean-room writer answers G147 to G151. Each answer is a rule change, a question with a provisional answer, or a note that the text already decides the case. Each answer that changes a rule adds the test that the finding suggests.
- The front-end choices in `frontend/README.md` that answer a gap in the text (G146, G147, G149) get a ledger row each, so a second implementer finds them without reading the first implementer's README.

**Evidence.** Section 1, place 1. Round 6 asked for a ledger (process change 3), and G134 and G135 waited a phase. G147 to G151 now wait in the same way.

**Size.** Small.

**Prerequisites.** None.

### 3. Front-end round 22: the old check-side and run-side backlog

**What it is.** Close the failures that phase 9 did not touch, grouped by analysis:
- the `same` idiom with variable templates (22 unsupported),
- conversion to a function reference and conversion functions in constants (22 unsupported, mostly overloading),
- concepts in `static_assert` (8) and the `noexcept` operator (4),
- `new` in a constant (6) and in a run (8), and constructor calls in a run (6),
- the BF-9 layout of classes with bit-fields (4),
- missed rejects by group: OVL-20 with no target (5), access after overload resolution (ACC-3, ACC-15, ACC-18, 6), DEFARG-13 (4), LIST-13 and LIST-16 (4), PACK-8 (3), MPTR-11 (3), DCMP-12 and DCMP-13 (3), and the 9 instantiation rules,
- the interpreter panic on `across-units-class-member.cpp`,
- the two `EDG-AGREES` tests (CPY-27, CNS-32), the four older false rejects, and `nondeduced-only-parameter.cpp` (ORD-3).

**Evidence.** Section 1. 117 of the 118 missed rejects and all 79 unsupported tests had the same status at `1b16115`. Round 6 ranked this item seventh, and phase 9 did not take it.

**Size.** Large, for a modest gain per test.

**Prerequisites.** Item 1 for the overloading groups. Item 2, so that each group the front end cannot close from the text becomes a ledger row.

**Risk.** Low. Each group that the front end cannot close from the text alone is a finding, and worth having.

### 4. Implement I4: the conditional operator with class operands (COND-16)

**What it is.** A second, smaller blind build. The component takes the types and value categories of the second and third operands, with the classes' bases and conversion functions, and returns the type and category of the result or a rejection. The blind inputs are `expressions/conditional.md` and the parts of the conversion-sequences and reference-initialization pages that COND-16 cites, without Evidence or `:tests:` lines. The scoring phase uses the 67 conditional tests.

**Evidence.**
- COND-16 has three ordered steps and a table, so it is a procedure in the sense of decision 0044.
- The 2026-11-04 gap review found a condition in the COND-16 table that the reference lacks, and RV2 changed the rule.
- PU1-7 and PU2-3 each marked a COND-P2 answer doubtful. Q-0378 stays open for that reason.
- G148 asks whether a reference binds directly to a class prvalue in step 2, and the text can be read both ways.
- The front end fails 5 of the 67 conditional tests: 3 missed rejects and 2 unsupported.

**Size.** Small. The page has 16 rules, and the component needs only part of I3's type model.

**Prerequisites.** None. It can share I3's type model if it runs after I3.

### 5. Rule boundaries on the lowest pages, with links for the general rules

**What it is.**
- For each page under 60% in section 1, place 4, the writer reads the list that `tidal-fe conform -v` prints and marks each pair as a front-end shortcut, a test that should also cite the named rule (decision 0042), or a rule text that should link the general rule.
- For the instantiation pattern, INST-1, INST-6, and INST-7 link the rules whose errors an instantiation reports, or the tests cite the inner rule as well. The harness then counts the pair as linked.
- The general rules named most often (TR-12, INIT-7, OVL-35, NAM-16, SA-1) get the same review.

**Evidence.** Section 1, place 4. Item FH built the table. No phase item reads it yet.

**Size.** Small.

**Prerequisites.** None.

**Risk.** Low. A front end can raise the rate by naming rules loosely, so the rate should prompt review and should not be a target.

### 6. The library surface and the variadic facility

**What it is.**
- The language-support page, or a test-convention section of `specification-format.md`, lists the C functions that tests may declare, with one sentence each on the effect a test relies on. The list holds no rules.
- For `va_list`, one of two changes. The two PFX-22 tests observe the conversions without `<cstdarg>`, or decision 0041 lists the variadic-argument facilities as a language-support shape.

**Evidence.** Section 1, place 3. Round 6 ranked this item fifth, and phase 9 did not take it. A second PFX-22 test with `<cstdarg>` arrived in RV2.

**Size.** Small.

**Prerequisites.** None. Decision 0041 should say whether a test convention widens the scope.

### 7. Labels for the 32 `unspecified` rules, then allowed results

**What it is.**
- Each of the 23 rules without a choice point gets one label: the interpreter explores it, a matrix table fixes it, or no test can observe it. The 23 are the 22 of round 6 and LAM-28.
- SEQ-8 explores more than two options where an operator has more than two side effects.
- Where a rule gives a finite set of results, the test lists that set, and the harness compares the explored set with it.

**Evidence.** Section 1, place 5. Round 6 ranked this item sixth, and phase 9 did not take it.

**Size.** Small for the labels, medium for the result lists.

**Prerequisites.** None.

### Dependency sketch

```
1 Implement I3 (overload ranking) ──> 3 front-end round 22 (overloading groups)
2 ledger, G147 to G151 ─────────────> 3
4 Implement I4 (COND-16; after 1 if it shares the type model)
5 rule boundaries (independent, small)
6 library surface (independent, small) ──> 3 (the two PFX-22 tests)
7 unspecified labels (independent; result lists after the labels)
```

Items 2, 5, 6, and the labels of item 7 are small and can start at once. Item 1 should run before item 3 touches overloading.

## 3. Scope

| Topic | Charter's reason to keep out | Implementer's view | Proposal |
|---|---|---|---|
| C library functions in tests | the general library | The interpreter knows seven by a front-end choice. A second implementer has to guess. | Keep the library out. State the seven as a test convention (item 6). |
| `<cstdarg>` and `va_list` | the general library, and no standard headers | Two PFX-22 tests need them, and EDG lacks them. | Keep out. Rewrite the tests, or list the shape under decision 0041. |
| Named escapes `\N{...}` | lexical structure beyond G1 | One test needs a Unicode name table, which is data. | Keep the test. The front end may leave it unsupported. |
| Lexical structure beyond G1 | similarity risk | No phase 9 finding needed a new lexical rule. | Keep out. |
| Preprocessor beyond `#include "file"` | real headers come with the library | Multi-unit tests and test headers work. `include-missing-file-angle.cpp` is unsupported by design. | Keep out. |
| Coroutines, modules, threads, the memory model | outside the language goal | No test or front-end path needs them. X21 stayed within one thread. | Keep out. |
| Later language versions (`pending.md` row 7) | the charter fixes C++23 | The front end's work would not change. | Keep out of the front end's work. |
| MSVC | not in the matrix | A second ABI would double the CLS-35 and BF-9 tables. | Keep out. |
| `typeid` | needs a library shape | One test uses it. | Keep out. |

## 4. Process changes

1. **Keep a ledger of front-end findings** (item 2). Round 6 asked for one. Five findings from report 21 now wait with no record outside the report.
2. **Run one Implement exercise each phase, picked by the front end's failure rate.** I1 and I2 each found steps that many test-driven rounds had passed. Overloading has the highest chapter failure rate, so I3 comes next. The conditional operator is the smaller second candidate.
3. **Give the old backlog an item of its own.** Phase 9's front-end rounds followed the repairs, and they closed new tests well: 116 of 119 phase 9 tests pass. The 254 older failures moved only where a repair touched the same rule. A phase item for the backlog, grouped as in item 3, keeps it from waiting another phase.
4. **Report the run side in `status.md`.** Rounds 4, 5, and 6 asked for this. The status page gives 6362 full passes and 6414 check passes, and no run counts. Add one line: 1603 of 1660 runs pass (9 partial), 0 give a wrong result, 52 are unsupported, and 5 are not run because the check failed.
5. **Treat an interpreter panic as a failure of its own.** The harness counts the panic on `across-units-class-member.cpp` as run-unsupported. A separate status, or a line in the `frontend_check.py` summary, would show it.
6. **Keep the front-end report form.** Rounds 20 and 21 gave before-and-after counts on one copy of the tests and listed the tests that arrived during the round. Round 22 should keep the form and add a table of the old failures by group, with their status before and after.
