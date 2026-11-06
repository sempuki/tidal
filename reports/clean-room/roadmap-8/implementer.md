# Roadmap, round 8: what to do after phase 10, from an implementer's view

Date: 2026-11-05. Decision 0029. Role: clean-room reader, acting as an implementer who builds a C++ front end and interpreter from Tidal alone. The evidence is the clean-room front end in `frontend/`, its reports and ledger in `reports/clean-room/rust-frontend/`, and the phase 10 exercise records. Commit read: `1e20bfa2`.

Inputs:
- AGENTS.md, CLAUDE.md
- `documentation/`: charter, roadmap, status, style guide, specification format, exercises, decisions 0056 and 0057, and `decisions/pending.md`
- `reports/clean-room/roadmap-7/implementer.md`, the model for this report's form
- `reports/clean-room/rust-frontend/`: report 22, `ledger.md`, `conformance.tsv`, `examples.tsv`, and `frontend/README.md`; reports 2 and 3 for the open findings G4, G13, and G25
- the phase 10 exercise records R10, W18, W19, I3 (report and `notes.md`), PU3, and X22
- `reports/review/`: the three 2026-11-05 gap reviews of the text phase 9 wrote (headings and Result sections)
- `specification/` (the rules cited below), the headers of the failing tests, and `questions/` (status and `**Waits for:**` lines)
- `git log f0f5331..1e20bfa2` with commit bodies (20 commits after round 7)

**Measurements.** Every count comes from commit `1e20bfa2`, with a clean tree. I built the front end with `cargo build --release --locked --target-dir` into my scratch directory, so the repository's build directory was not touched. I ran `tidal-fe conform tests -v` once. Its status counts match `conformance.tsv` exactly. I compared `conformance.tsv` at `1e20bfa2` with the file at `f0f5331` (round 7, the same file as at `017b401`) and with the file at `1b16115` (round 6), to see which failures are old. Other counts come from grep and short Python scripts in the scratch directory, so they are approximate. Statements from general knowledge say so.

## 1. Where the specification and the front end stand

| Measure | Round 7 (6619 tests, `017b401`) | Now (6681 tests, `1e20bfa2`) |
|---|---|---|
| Full pass | 6362: 6089 `pass`, 254 `EDG-ONLY`, 10 `EDG-ALLOWED`, 9 `pass-partial` | 6459: 6174 `pass`, 264 `EDG-ONLY`, 10 `EDG-ALLOWED`, 11 `pass-partial` |
| Check pass | 6414 | 6499 |
| `EDG-AGREES` (a failure) | 2 | 2 (CPY-27, CNS-32) |
| False rejects | 6 | 4 |
| Missed rejects | 118 | 116 |
| Unsupported (check) | 79 | 60 |
| `NO-BASE` | 0 | 0 |
| Run tests | 1660: 1603 pass (9 partial), 0 wrong, 52 unsupported, 5 not run | 1696: 1651 pass (11 partial), 0 wrong, 0 missed or unexpected UB, 40 unsupported, 5 not run |
| Interpreter panics | 1 (`across-units-class-member.cpp`) | 0 |
| Interpreter-only expectations | 19, all met | 19, all met |
| Passing rejects whose first error names a cited or linked rule | 1990 of 2593 (77%) | 2004 of 2609 (77%): 1827 cited, 177 linked, 604 another rule, 1 no rule |
| `reject` rules absent from the front end's source | 103 of 558 | 103 of 558 |
| Page examples | 175 of 177 pass; 2 unsupported | 175 of 177 pass; 2 unsupported |
| Rules by kind | 1697 | 1697, the same split: 898 required, 558 reject, 57 undefined, 52 impl-defined, 43 recommended, 37 no-diagnostic, 32 unspecified, 5 conditionally-supported, 15 withdrawn |
| `unspecified` rules that the front end's source names | 9 of 32 with a choice point (round 7's count) | 13 of 32 by a grep for the ID; 19 are not named |
| Questions | 400: 172 provisional, 13 compiler defect, 215 settled | 411: 171 provisional, 13 compiler defect, 227 settled |
| Front-end findings | G142 to G151; 5 with no answer | G152; ledger rows for G1 to G152, 4 of them open |
| Front-end source (Rust lines, approximate) | 49087 | 50160 |

The status page's run sentence gives 1652 passes and 4 tests not run. The harness's run table gives 1651 and 5. I did not find the one test that the two count differently.

### What phase 10 took from round 7

| Round-7 item | Phase 10 result |
|---|---|
| 1. Implement I3 (overload ranking) | Done (`4915c313`). 1513 of 1532 assertions, 22 unstated steps (N1 to N22), findings I3-1 to I3-18. The writer acted on I3-2, I3-3, I3-7, I3-8, I3-10, and I3-11 in `4a193ea4`. Round 22 closed I3-14 to I3-18. I3-1, I3-5, I3-12, and I3-13 have no action. |
| 2. Ledger and G147 to G151 | Done (`a9c480b7`). `ledger.md` has a row for each G finding, and `check_specification.py` fails on a finding with no row. G147, G149, G150, and G151 changed rules. G148 became Q-0401 and went to `pending.md` row 4. |
| 3. Front-end round 22 on the old backlog | Done (`eff1a372`). 36 old failures now pass, 0 tests got worse, the panic is gone, and report 22 has the before-and-after table by group. |
| 4. Implement I4 (COND-16) | Not taken. |
| 5. Rule boundaries on the lowest pages | Deferred by decision 0057. The rate stays at 77%. |
| 6. Library surface and `va_list` | Deferred by decision 0057. The two PFX-22 false rejects remain. |
| 7. Labels for the `unspecified` rules | Deferred by decision 0057. |

Phase 10 took five of round 7's process changes: the ledger, one Implement exercise, a backlog item, the run line on the status page, and the report form. The harness still reports an interpreter panic as `run-unsupported` with "interpreter crashed" (`src/conform.rs`). No test panics now.

### The old failures moved a little

Phase 10 added 62 tests. 61 of them pass on the front end. The one that fails is `expressions/postfix/default-template-argument-incomplete-class.cpp` (CLS-2, Q-0406), a missed reject.

The other 221 failures were already failures at round 7, with the same status:

| Status at `f0f5331` and now | Tests |
|---|---|
| `MISSED-REJECT` both times | 115 |
| `unsupported` both times | 60 |
| `run-unsupported` both times | 40 |
| `FALSE-REJECT` both times | 4 |
| `EDG-AGREES` both times | 2 |

Against round 6 (`1b16115`), 219 of the 222 failures had the same status there. Round 22 fixed 36 old failures. At that rate, the old backlog needs about six more rounds.

### Failures by page

The failure rate per test directory (all statuses other than a full pass):

| Directory | Failing | Rate | Round 7 | Largest causes |
|---|---|---|---|---|
| templates/packs | 8 of 69 | 11.6% | 13.0% | PACK-8 (3 missed), the `same` idiom |
| classes/conversion-functions | 7 of 63 | 11.1% | 12.9% | 4 run stops at an overload set or a conversion function |
| expressions/new-delete | 12 of 128 | 9.4% | 10.2% | `new` in a constant (5) and in a run (5) |
| overloading/conversion-sequences | 12 of 130 | 9.2% | 10.0% | ICS-45 inside a requires-expression (5), extended floating-point types (2) |
| classes/access | 6 of 71 | 8.5% | 8.7% | ACC-3, ACC-15, ACC-18 after overload resolution (all missed) |
| templates/dependent-names | 10 of 122 | 8.2% | 8.2% | DEP-37 (2), run stops at a dependent name |
| foundations/language-support | 4 of 49 | 8.2% | | |
| templates/instantiation | 11 of 140 | 7.9% | 8.0% | 9 missed rejects on 8 different INST rules |
| expressions/conditional | 5 of 68 | 7.4% | | COND-16, COND-3 |
| classes/comparisons | 6 of 82 | 7.3% | | DCMP-12 and DCMP-13 (3 missed) |

Overloading/overload-resolution fell from 19 of 168 to 6 of 169. By chapter, overloading still has the highest rate, 18 of 299 (6.0%, was 10.7%). Templates follow closely with 70 of 1192 (5.9%, was 6.2%). Templates hold 43 of the 116 missed rejects.

The causes that the harness names for the 60 check-side unsupported tests:

| Construct | Tests | Round 7 |
|---|---|---|
| `static_assert` on a variable template with unknown arguments (the `same` idiom) | 16 | 22 |
| a concept in `static_assert` (5 of them ICS-45 casts inside a requires-expression) | 8 | 8 |
| `new` in a constant | 6 | 6 |
| the `noexcept` operator naming a later member | 4 | 4 |
| `sizeof` of a class with bit-fields that the layout code does not compute (BF-9) | 4 | 4 |
| conversion functions in a constant | 4 | 10 |
| pointer arithmetic in a constant initializer | 4 | |
| conversion to a function reference | 0 | 12 |

The 40 run-unsupported tests stop at `new` forms (7), constructor calls (4), jumps into a nested statement (3), overload sets (3), variables that are declared in the unit but defined elsewhere or not at all (3), and variables of an enclosing function (3).

### Places where the text or the record still stops the implementer

1. **Findings with no answer, and findings outside the ledger.** The ledger covers G findings only, and four of its rows are open:
   - G152 (report 22) asks whether forming a candidate instantiates the body of a generic lambda's call operator with a deduced result type (DED-29, LAM-15, OVL-27). The front end holds such errors until a call selects the operator. It records that choice only in `frontend/README.md`.
   - G4 (literal types, CE-23), G13 (a page that had no TYP-2 then), and G25 (CTOR-14 needed overload ranking then) date from reports 2 and 3. G13 and G25 look stale, since the pages they name now exist.

   Exercise findings have no ledger at all:
   - R10-1 to R10-7 and R10-9 (objects page). Commit `7e4ddf7c` says the writer acts on them in RV3's repair pass. No later commit names R10, and `foundations/objects.md` did not change after that commit.
   - I3-1 and I3-5 wait for probes. The writer answered I3-12 by saying CONV-1 already gives the promotions, but the reader read CONV-1 as giving only `char32_t`. I3-13 has no test.
   - W18-14 and X22-6 are recorded as not acted on.

2. **G152 reaches two failing tests.** `substitution-lambda-body.cpp` and `substitution-lambda-default.cpp` (DED-29) put a lambda with a bad body in a function type or in a default template argument. The matrix rejects both, and the front end misses both. The W18 visitor program needs the opposite treatment for a generic lambda in an overload set. An implementer cannot tell from the text where the line falls.

3. **Reviewers dispute COND-16 on two points.** The 2026-11-05 expressions review reads COND-16 against Tidal twice. The first point is the binding to a class prvalue (Q-0401, now `pending.md` row 4, with GCC on the review's side). The second is the base class row, where the review lets a converting constructor of the derived class form the conversion. The page records the second as a split. Q-0378 stays open. The front end fails 5 of 68 conditional tests.

4. **The library surface is still a front-end choice.** The seven C functions are listed only in `frontend/README.md`. The two PFX-22 tests that include `<cstdarg>` have been false rejects at `va_list` since round 6.

5. **Rule IDs on rejects did not move.** The rate is 77%, as in round 7. The lowest pages by first error:

   | Page | Named (cited or linked) | Round 7 |
   |---|---|---|
   | foundations/values | 0 of 9 | 0 of 9 |
   | expressions/decltype | 2 of 12 | 2 of 12 |
   | foundations/conversions | 7 of 20 | 7 of 20 |
   | expressions/member-pointers | 11 of 29 | 11 of 29 |
   | foundations/names | 47 of 95 | 47 of 94 |
   | templates/instantiation | 36 of 71 | 34 of 70 |

   The rules named most often in place of a cited rule are TR-12 (45), NAM-8 (37), INIT-7 (37), NAM-16 (23), OVL-35 (22), and NAM-9 (19). In templates/instantiation, 35 passing rejects name another rule. Most of them cite INST-1, INST-6, or INST-7, and the front end names SA-1 or NAM-16 for the error inside the instantiation.

6. **The `unspecified` rules have no labels.** 19 of the 32 do not appear by ID in the front end's source. SEQ-8 still explores two options per operator. 11 program runs are `pass-partial`.

7. **Template procedures have had no blind build.** I1 (layout), I2 (partial ordering), and I3 (overload ranking) each found more than a dozen unstated steps. I3 found that ICS-P1 named too little of the argument (I3-3, a wrong statement). No blind build has read deduction from a call (DED-P1) with the substitution boundary (DED-24, DED-29) and the instantiation triggers (INST-6, INST-7). No blind build has read constraint normalization either (CNS-P1, CPT-19). Q-0411 records that EDG treats the folds of CPT-19 as one atom. Templates hold 43 of the 116 missed rejects.

## 2. Ranked items

| Rank | Item | Evidence | Size | Prerequisites |
|---|---|---|---|---|
| 1 | One ledger for every finding, and answers for G152, G4, G13, G25, and the R10 findings | 8 R10 findings with no action after a commit said RV3 would act; 4 open G rows; I3 and W18 leftovers in commit bodies only | small | none |
| 2 | Implement I4: the conditional operator with class operands (COND-16), blind | two review readings against COND-16; Q-0401 in row 4; Q-0378 open; 5 of 68 tests fail; round 7 item 4 not taken | small | none |
| 3 | Implement I5: deduction from a call and the substitution boundary, blind | G152; 2 DED-29 missed rejects; 9 instantiation missed rejects; templates hold 43 missed rejects | medium | 1 for G152 |
| 4 | Front-end round 23: the template check side, then the rest of the backlog | 221 of 222 failures predate phase 10; round 22 fixed 36 | large | 1; 3 for the template groups |
| 5 | Rule boundaries on the lowest pages, with links for the general rules | 77% for two phases; 35 instantiation rejects name another rule | small | none |
| 6 | The library surface and the variadic facility | 7 C functions in a README; 2 false rejects since round 6; deferred twice | small | none |
| 7 | Labels for the 32 `unspecified` rules, then allowed results | 19 rules not named in the front end; 11 partial runs; deferred twice | small (labels), medium (results) | none |
| 8 | A front-end prototype of P23 (Propose X22) | X22 section 1.6 says how an implementation can do it; decision 0057 deferred "a front-end prototype of a done proposal" | medium | 4 for the constraint groups |

### 1. One ledger for every finding, and answers for G152, G4, G13, G25, and the R10 findings

**What it is.**
- `ledger.md`, or a file beside it, takes a row for each finding ID that an exercise report names: R, W, I, PU, X, and D findings as well as G findings. Each row has the states that the G rows use (commit, question, text, out of scope, open).
- `check_specification.py` fails when a report names a finding of any kind that has no row.
- The clean-room writer answers G152 with a rule change or a question. The answer says when overload resolution needs the deduced result type of a candidate. It also says whether a candidate that a call does not select has its body instantiated. The answer adds the test that report 22 suggests.
- The writer sets G13 and G25 to `text` or `out of scope` if they are stale, and answers G4 against CE-23.
- The writer acts on R10-1 to R10-7 and R10-9, or records why not.

**Evidence.** Section 1, place 1. Round 7's ledger item closed the gap for G findings. The same gap now affects the exercise findings. The R10 batch shows it most clearly: its commit names the pass that would act on it, and that pass did not.

**Size.** Small. The R10 findings ask for notes, sentences, and cross references.

**Prerequisites.** None.

### 2. Implement I4: the conditional operator with class operands (COND-16), blind

**What it is.** This is the item round 7 ranked fourth. A fresh clean-room reader builds a component from `expressions/conditional.md`, with Evidence and `:tests:` lines filtered out. The reader also reads the parts of the conversion-sequences and reference pages that COND-16 cites. The component takes the types and value categories of the second and third operands, with the classes' bases, constructors, and conversion functions. It returns the type and category of the result, or a rejection. The scoring phase uses the 68 conditional tests.

**Evidence.**
- Section 1, place 3. The phase 9 review of the text found two readings against COND-16. One became Q-0401 in `pending.md` row 4. The page records the other as a split.
- G148 began as a front-end finding on the same step, and its answer went to the maintainer.
- I3's component already has the conversion and binding steps that COND-16 uses. I4 can reuse that type model (`reports/clean-room/I3/ranking.py`), or a fresh reader can write its own.

**Size.** Small. The page has 16 rules.

**Prerequisites.** None.

**Risk.** Low. General knowledge: the class-operand cases of the conditional operator are rarely taught, so a reader is less likely to fill gaps from memory than with overload ranking.

### 3. Implement I5: deduction from a call and the substitution boundary, blind

**What it is.**
- A fresh clean-room reader builds a component from the deduction page (DED-P1 and its rules), DED-24 and DED-29, and the instantiation rules that DED-29 relies on (INST-6, INST-7). Evidence and `:tests:` lines are filtered out.
- The component takes a function template's parameter types and a call's argument types and categories. It returns one of four results: the deduced arguments, a deduction failure, a substitution failure, or a hard error. It also names the step that decided.
- The scoring phase uses the deduction tests and the grids under `tests/templates/deduction/grids/`.
- Constraints and partial ordering stay callbacks, as in I3.

**Evidence.**
- G152 and the two DED-29 missed rejects (section 1, place 2) sit on the line between a substitution failure and a hard error. X22-2 changed DED-24 in phase 10.
- Templates hold 43 of the 116 missed rejects. templates/instantiation has 9 of them, on 8 different rules.
- I1, I2, and I3 each found more than a dozen steps that Predict and many front-end rounds had passed. No blind build has read a template page other than ordering.

**Size.** Medium. The type model needs templates, packs, and dependent types. I2's type model covered part of this.

**Prerequisites.** Item 1 should answer G152 first. Otherwise, the I5 prompt should list G152 as an open question that the reader must log.

**Alternative.** Constraint normalization and subsumption (CNS-P1, CNS-4, CNS-22, CPT-19). Q-0411 records a split on CPT-19 atoms, and X22 found two gaps in the constraint text (X22-1, X22-3). I rank deduction higher because it reaches more failing tests.

### 4. Front-end round 23: the template check side, then the rest of the backlog

**What it is.** Close the failures that round 22 did not reach, grouped by analysis:
- template missed rejects: PACK-8 (3), DEP-37 (2), CTAD-2 (2), CNS-29 (2), TARG-24 (2), DED-29 (2, after item 1), the 9 instantiation tests, and the CLS-2 default template argument test (Q-0406),
- the `same` idiom with variable templates (16 unsupported),
- concepts in `static_assert` (8), with the ICS-45 casts inside a requires-expression,
- the `noexcept` operator naming a later member (4), which needs a parse at the end of the class,
- `new` in a constant (6) and in a run (7), and constructor calls in a run (4),
- the BF-9 layout of classes with bit-fields (4),
- the older missed-reject groups: ACC-3, ACC-15, and ACC-18 (6), DEFARG-13 (4), LIST-13 and LIST-16 (4), MPTR-11 (3), and DCMP-12 and DCMP-13 (3),
- jumps into a nested statement (3) and variables of an enclosing function (3) in the interpreter.

**Evidence.** Section 1. 221 of the 222 failures predate phase 10, and 219 predate phase 9. Round 22 had many targets and still fixed 36 old failures.

**Size.** Large.

**Prerequisites.** Item 1 for G152. Item 3 for the deduction and instantiation groups.

**Risk.** Low. Each group that the front end cannot close from the text becomes a finding with a ledger row.

### 5. Rule boundaries on the lowest pages, with links for the general rules

**What it is.** Round 7 item 5, unchanged.
- For each page under 60% in section 1, place 5, the writer reads the pairs that `tidal-fe conform -v` prints. The writer marks each pair as one of three cases: a front-end shortcut, a test that should also cite the named rule, or a rule that should link the general rule.
- For the instantiation pattern, INST-1, INST-6, and INST-7 link the rules whose errors an instantiation reports, or the tests cite the inner rule as well.

**Evidence.** Section 1, place 5. The rate and the lowest pages did not change in phase 10.

**Size.** Small.

**Prerequisites.** None.

**Risk.** Low. A front end can raise the rate by naming rules loosely. The rate should prompt review, and it should not be a target.

### 6. The library surface and the variadic facility

**What it is.** Round 7 item 6, unchanged.
- The language-support page, or a test-convention section of `specification-format.md`, lists the C functions that tests may declare. Each gets one sentence on the effect a test relies on. The list holds no rules.
- For `va_list`, there are two options. The two PFX-22 tests can observe the conversions without `<cstdarg>`. Or decision 0041 can list the variadic-argument facilities as a language-support shape.

**Evidence.** Section 1, place 4. Rounds 6 and 7 ranked the item, and two phases deferred it.

**Size.** Small.

**Prerequisites.** None.

### 7. Labels for the 32 `unspecified` rules, then allowed results

**What it is.** Round 7 item 7, unchanged.
- Each rule without a choice point gets one label: the interpreter explores it, a matrix table fixes it, or no test can observe it.
- SEQ-8 explores more than two options where an operator has more than two side effects.
- Where a rule gives a finite set of results, the test lists the set, and the harness compares the explored set with it.

**Evidence.** Section 1, place 6. Rounds 6 and 7 ranked the item, and two phases deferred it.

**Size.** Small for the labels, medium for the result lists.

**Prerequisites.** None.

### 8. A front-end prototype of P23 (Propose X22)

**What it is.** A clean-room reader adds the `checked` marker and the check of X22 section 1.4 to a branch of the front end. The reader runs X22's probes and the constraint tests through the branch. The prototype reports each step the design needed that the X22 report did not give.

**Evidence.**
- X22 section 1.6 describes an implementation.
- The front end already keeps constraints as normal forms, with atoms identified by their place in the source (`frontend/README.md`).
- Decision 0057 lists the prototype among the deferred work, and no Propose design has been built.

**Size.** Medium.

**Prerequisites.** Item 4 for the constraint groups, so the prototype starts from a front end that passes the constraint tests.

**Risk.** Medium. The prototype must stay out of the conformance baseline, because P23 is not part of Tidal's rules.

### Dependency sketch

```
1 ledger, G152, R10 ──> 3 Implement I5 (deduction) ──> 4 front-end round 23 (template groups)
1 ───────────────────────────────────────────────────> 4 (DED-29 tests)
2 Implement I4 (COND-16; independent, small; can reuse I3's model)
5 rule boundaries (independent, small)
6 library surface (independent, small) ──> 4 (the two PFX-22 tests)
7 unspecified labels (independent; result lists after the labels)
4 (constraint groups) ──> 8 P23 prototype
```

Items 1, 2, 5, and 6, and the labels of item 7, are small and can start at once. Item 3 should report before item 4 works on the template groups.

## 3. Scope

| Topic | Charter's reason to keep out | Implementer's view | Proposal |
|---|---|---|---|
| C library functions in tests | the general library | The interpreter knows seven by a front-end choice. A second implementer has to guess. | Keep the library out. State the seven as a test convention (item 6). |
| `<cstdarg>` and `va_list` | the general library, and no standard headers | Two PFX-22 tests need them, and EDG lacks them. | Keep out. Rewrite the tests, or list the shape under decision 0041. |
| Extended floating-point types | *(referenced)* only | I3-9 and G42 could not score or run them. 2 conversion-sequences tests stop on them, and so do some of the 16 `same` idiom tests. | Keep out. The front end may leave those tests unsupported. |
| Named escapes `\N{...}` | lexical structure beyond G1 | One test needs a Unicode name table, which is data. | Keep the test. The front end may leave it unsupported. |
| Preprocessor beyond `#include "file"` | real headers come with the library | Multi-unit tests and test headers work. `include-missing-file-angle.cpp` is unsupported by design. | Keep out. |
| Coroutines, modules, threads, the memory model | outside the language goal | No test or front-end path needs them. | Keep out. |
| Later language versions (`pending.md` row 7) | the charter fixes C++23 | The front end's work would not change. | Keep out of the front end's work. |
| MSVC | not in the matrix | A second ABI would double the CLS-35 and BF-9 tables. | Keep out. |
| `typeid` | needs a library shape | One test uses it. | Keep out. |
| Propose designs (P23) | not part of the rules | A prototype tests whether a design can be built. | Allow a prototype on a branch, outside the baseline (item 8). |

## 4. Process changes

1. **Give every finding a ledger row.** The G ledger worked: G147 to G151 all have answers. R10's findings show the same gap for exercise findings. A commit said a later pass would act on them, and no later commit names them.
2. **Keep one Implement exercise each phase.** I3 found a wrong principle sentence (I3-3) and closed five front-end stops. COND-16 (small) and deduction (medium) are the next candidates. They read different pages, so they can run in the same phase.
3. **Start each front-end round from the ledger's open rows.** G152 is open now, and two failing tests depend on its answer.
4. **Take or drop items deferred twice.** Two rounds ranked the library surface and the `unspecified` labels, and two phases deferred them. A phase decision should adopt each item or drop it with a reason.
5. **Give the run counts one definition.** The status page and the harness's run table differ by one test. One script should produce both.
6. **Keep the front-end report form.** Report 22 gave counts on the old and the full test set, and a table of old failure groups before and after. It also listed the tests that arrived during the round. Round 23 should keep that form.
