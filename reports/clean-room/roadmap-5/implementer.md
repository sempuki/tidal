# Roadmap, round 5: what to do after phase 7, from an implementer's view

Date: 2026-11-03. Decision 0029. Role: clean-room reader, acting as an implementer who builds a C++ front end and interpreter from Tidal alone. The clean-room front end in `frontend/` and its reports in `reports/clean-room/rust-frontend/` are the evidence.

Inputs:
- AGENTS.md, CLAUDE.md
- `documentation/`: charter, roadmap, status, style guide, specification format, exercises, decisions 0033, 0039, 0044, 0050, 0051, 0054, and `decisions/pending.md`
- `reports/clean-room/roadmap-4/implementer.md`, which is the model for this report's form
- `reports/clean-room/rust-frontend/` reports 15, 16, and 17, and `conformance.tsv`
- the phase 7 exercise reports: R-overloading, R-foundations, R-exceptions (the findings and the scores in the commit messages), X13 to X16 (headings only)
- `reports/review/`: the 2026-11-03 gap reviews of the exceptions chapter, names, the object model, and program structure (headings and results)
- `specification/` (the rules cited below), the headers of `tests/`, `questions/README.md`, Q-0376
- `git log --oneline 2798b37~1..HEAD` (22 commits) and the messages of the repair commits

**Measurements.** All counts are from commit `7d8c982`. I built the front end from that commit into my scratch directory (`cargo build --release --locked --target-dir`), so the repository's build directory was not touched. I ran `tidal-fe conform ../tests -v`. Its statuses match `conformance.tsv` on all 6337 tests, so the numbers below are the committed baseline. I also ran `tidal-fe run` on each of the 1564 `run`, `run-fail`, and interpreter-only tests to read the choice points and the explored outcomes, and ran `tidal-fe check` on each of the 2505 passing `reject` tests to read the rule that the first diagnostic names. Counts come from grep and short Python scripts, so they are approximate. Statements from general knowledge say so.

| Measure | Round 4 (6032 tests, `3d98c93`) | Now (6337 tests, `7d8c982`) |
|---|---|---|
| Check pass | 5708 (95%) | 6134 (97%): 5995 pass, 139 differ from EDG only |
| Full pass (check and run) | 5595 | 6068 |
| False rejects | 6 | 4 |
| Tests where the front end follows the EDG override against the base expectation, counted as pass | not measured | 99 (70 base `reject`, 29 base `accept` or `run`) |
| Missed rejects | 212 (187 cite a `reject` rule) | 121 (107 cite a `reject` rule, 7 only `recommended`, 6 only `no-diagnostic`, 1 only `unspecified`) |
| Unsupported (check) | 106 | 78 |
| Check, by expectation | accept 1926/2019, reject 2341/2564, run 1315/1323, run-fail 84/84, link-fail 39/39 | accept 2033/2107, reject 2505/2633, run 1454/1455, run-fail 95/95, link-fail 44/44 |
| Rejects that name one of the test's rules (harness count) | 1537 of 2283 (67%) | 1655 of 2505 (66%) |
| Run tests checked by the interpreter | 1411: 1286 pass, 0 wrong, 1 missed UB, 115 unsupported, 9 not run | 1564 (1455 `run`, 95 `run-fail`, 14 interpreter-only): 1491 pass, 1 wrong, 0 missed UB, 0 unexpected UB, 69 unsupported, 3 not run |
| Program tests (`tests/programs/`) | none | 67, all run to result 0 |
| Runs that pass a choice point | 45 | 56 (12 of them programs); 3 stop at the bound of 64 runs; 1 has another outcome that the harness ignores |
| Rules by kind | 1678 | 1691: 895 required, 557 reject, 57 undefined, 52 impl-defined, 44 recommended, 35 no-diagnostic, 31 unspecified, 5 conditionally-supported, 15 withdrawn |
| `undefined` rules with no run witness | 13 | 0 without a witness or a note; 3 with a note that no tool stops them (VOL-6, ADD-18, OBJ-12) |
| `impl-defined` rules with no matrix choice | 17 by grep | 0 by grep (SINIT-5 names the compilers in prose) |
| Questions | 352: 309 provisional, 1 open, 12 compiler defect, 30 settled | 376: 209 provisional, 0 open, 13 compiler defect, 154 settled |
| Front-end findings | G85 to G117 | G118 to G133, all acted on in `2eb7c8d` |

## 1. Where the specification still stops an implementer

Phase 7 took five of my eight round-4 items in full and one in part. It left the rule-ID scoring and the result sets.

| Round-4 item | Phase 7 result |
|---|---|
| 1. Matrix target profile | Done (item TP). CONV-7 and CONV-11 have matrix tables, and CLS-35 is the layout table. The front end does not use the CONV-11 table yet (2 run tests). |
| 2. Front-end round 15 | Done in rounds 15 to 17. Class layout, virtual bases, anonymous unions, aligned allocation, the tuple protocol, and generic lambdas now work. |
| 3. Write programs as run tests | Done (item WP). 67 programs, all run to result 0. |
| 4. Run witnesses for `undefined` rules | Done (item UW). 14 interpreter-only tests, and three Evidence notes. |
| 5. Predict batches for the ten pages | Done (item PR). 839 of 840 match. |
| 6. Rule-ID scoring per page, and a TR-12 overlap | Not taken. The harness still prints one total. |
| 7. Result sets for choice points | Not taken. Place 1 below shows a wrong outcome that this gap hides. |
| 8. Check-side round on missed rejects | In part. Round 17 closed 96 missed rejects. 121 remain. |

### Failures by page

The page is the page of the first rule that the test cites. "Run unsupported" counts `run` and `run-fail` tests whose check passed and whose run the interpreter cannot finish.

| Page | False reject | Missed reject | Unsupported | Run unsupported | Rules most cited |
|---|---|---|---|---|---|
| overloading/overload-resolution | 0 | 7 | 10 | 2 | OVL-20, OVL-29, OVL-9, OVL-36 |
| overloading/conversion-sequences | 0 | 2 | 11 | 0 | ICS-45, ICS-44, ICS-35, ICS-47 |
| expressions/new-delete | 0 | 2 | 5 | 5 | NEW-2, NEW-30, NEW-10, NEW-39 |
| templates/instantiation | 0 | 9 | 1 | 1 | INST-6, INST-12, INST-30 |
| expressions/conditional | 1 | 6 | 3 | 0 | COND-5, COND-3, COND-16, COND-8 |
| templates/packs | 0 | 6 | 4 | 0 | PACK-8, PACK-17, PACK-3 |
| exceptions/unwinding | 0 | 0 | 0 | 8 (and 1 wrong result) | UNW-7, UNW-3, UNW-2, UNW-1 |
| templates/class-template-deduction | 0 | 5 | 4 | 0 | CTAD-1, CTAD-2, CTAD-9 |
| templates/dependent-names | 0 | 5 | 2 | 2 | DEP-30, DEP-20, DEP-33, DEP-11 |
| classes/conversion-functions | 0 | 2 | 1 | 5 | CVF-1, CVF-2, CVF-9 |
| classes/access | 0 | 7 | 0 | 0 | ACC-15, ACC-3, ACC-19, ACC-18 |

No page has more than 19 failures. Round 4's largest page had 30. The overloading chapter now leads.

### Front-end gaps (the text is enough)

- **The tests from the G118 to G133 repairs.** `status.md` names 13 as the next front-end step. By the specification's answer there are 14 (place 1 below). By status:
  - 4 BF-9 layout tests are unsupported (wide bit-fields, a bit-field in a union). BF-9 now gives the rule (G125).
  - 3 false rejects: `tuple-index-size-type.cpp` (SB-5, Q-0376), `two-classes-promoted-types.cpp` (COND-11), `flow-off-end-constant-paths.cpp` (FN-7).
  - 4 missed rejects: two LSP-7 orders (G132), `class-ambiguous-conversion.cpp` (COND-16), `two-classes-several-conversions.cpp` (COND-11).
  - `class-converts-to-reference.cpp` is unsupported (the `same` idiom).
  - `return-local-destructor-throws-caught-inside.cpp` (UNW-1, G130) is the one wrong run result. The interpreter gives 10.
  - `qualified-definition-inline-member-inside-named.cpp` (NS-15, G133) is a run test that the check does not accept.
- **CONV-11.** The rule now has a table, and Clang and GCC give infinity on both targets. The interpreter still stops `floating-narrowing-out-of-range-run.cpp` and `floating-narrowing-infinity-run.cpp` with "floating conversion out of range (CONV-11, implementation-defined)". Item TP closed the text side. The front end has not caught up.
- **Run-time constructs.** 65 run tests are unsupported. The largest groups:

  | Construct | Run tests |
  |---|---|
  | a C library function that the test declares (`_Exit`, `malloc`, `free`) | 7 |
  | `new` forms: a parenthesized type, a placement argument through a converting constructor, placement deallocation templates, nothrow bound checks | 8 |
  | constructor calls the interpreter does not run | 6 |
  | block-scope `static` used before its declaration ran | 4 |
  | jumps into a nested statement (IF-12, SW-1) | 3 |
  | variables of an enclosing function in a local class or lambda | 3 |
  | the address of an explicit object member function | 3 |
  | a destructor function try block the front end does not parse | 2 |

- **Check-side unsupported constructs.** 78 tests. The `same` idiom with arguments the front end cannot compute is 23 (28 in round 4). Then concepts in `static_assert` (8), conversion to a function reference (6), conversion functions in constants (6), the `noexcept` operator (4), and `new` in a constant (4).
- **Missed rejects.** 121, spread thinly. Report-17 names the analyses the front end lacks: access checks after overload resolution (ACC-3, ACC-15, ACC-18, ACC-19), defaulted comparisons (DCMP-11 to DCMP-13), consteval escalation (CEV-1, CEV-2), five warnings, overload sets with no target (OVL-20, 5 tests), and template checks. The phase 7 Predict batches matched 839 of 840 on the overloading pages and storage, so the text decides most of these outcomes.
- **`undefined` rules the interpreter does not stop.** The interpreter names 45 of the 57 `undefined` rules in some run. Eight more are covered through a general rule under decision 0052 (ADD-14 through ADD-10, MUL-14 through SEQ-4, RET-6 and REF-5 through OBJ-2, OBJ-10 through STG-22, PRIM-8 through CV-5, CAST-14 through DER-17 and ENUM-6, TR-8 through ADD-9). Four are not detected at all:
  - VOL-6 and OBJ-12: the Evidence says no tool stops them, and that includes the interpreter. An interpreter can see both, because it knows the kind of each glvalue and when a declaration is passed.
  - ADD-18: the interpreter cannot hold an array large enough.
  - LAM-19: `capture-outlived-sanitizer.cpp` stops earlier at an unsupported `new` expression.

### Places where the text still stops the implementer

1. **A pass can hide a wrong outcome.** This is the most serious gap I found. The EDG override and the explored outcomes both hide failures.

   *The EDG override.*
   - `conform.rs` takes `tidal-expect-edg` as the expectation when a test has one, and the base expectation only as a fallback. A front end that agrees with EDG gets `pass`. A front end that agrees with Clang, GCC, and the specification gets `EDG-ONLY`, which also counts as a pass.
   - 244 tests have an EDG override that differs from the base on reject against accept. The front end follows the base on 139 and follows EDG on 99. The harness counts all 238 as passes.
   - Of the 99, 70 accept a program whose base expectation is `reject` (for example `main-four-params.cpp`, `using-reduces-access.cpp`, and the four `template-keyword-*` tests), and 29 reject a program whose base expectation is `accept` or `run` (for example `goto-past-nontrivial-destructor.cpp`). 18 of the 99 cite an `impl-defined`, `unspecified`, `no-diagnostic`, or `conditionally-supported` rule, where EDG's answer can be allowed. The other 81 are missed or false rejects against the specification that no count shows.
   - `qualified-definition-inline-member-inside-named.cpp` (NS-15, G133, Q-0375) is one of them. The front end rejects it as EDG does, so `conformance.tsv` says `pass`, and the run table says "the check did not accept the program". `status.md` counts 13 open tests from the repairs. By the specification's answer there are 14.
   - Against the base expectation alone, the check passes 6035 of 6337 tests, and not 6134.

   *The explored outcomes.*
   - `classes/operators/subscript-multi-order.cpp` tests OPR-7, which says that in `a[x, y]` the arguments are evaluated from left to right. The interpreter treats the arguments of a multi-argument `operator[]` as SEQ-1 arguments in an unspecified order. Its first run gives result 0. Its second run gives result 1. The harness reads only the first run, so the test passes.
   - `tidal-fe run` prints "other outcomes: result 1". Neither `conform` nor `frontend_check.py` looks at that line.
   - The fault is in the front end, and the text is clear. The harness has no way to notice.
   - SEQ-1 still tries two argument orders. `parameter-destruction-order-three.cpp` has three arguments and six orders, and the interpreter explores two.
   - Three runs stop at the bound of 64 (`W0/calc.cpp`, `W3/nested.cpp`, `W4/grid.cpp`). Their result sets are incomplete, and the harness does not say so.
   - Of the 31 `unspecified` rules, the interpreter has a choice point for 6 (SEQ-1, SEQ-8, CMP-8, CMP-20, MPTR-7, PRIM-9) and a fixed order with a message for SINIT-3. CLS-34 and CLS-35 are fixed by the matrix table. The other 22 have no choice point. Some of them have only one result that a test can observe. The list of allowed results that round 4 asked for (item 1, last point) was not made, so I cannot tell which.

2. **Rule IDs on rejects.**
   - The harness counts 1655 of 2505 passing rejects (66%) that name one of the test's rules. My count by the first diagnostic is lower: 1516 name a rule of the test, 673 name another rule, and 315 name no rule.
   - 89 of the 92 keyword tests cite KW-1, and the front end names TR-12. KW-1 links TR-12. Decision 0052 lets a run stop match a general `undefined` rule, and the harness has no such match for a rejection. Round 4 asked for one.
   - By the first diagnostic, the lowest pages are types (0 of 20, and all 20 name no rule), the one-definition rule (3 of 18), names (21 of 94), conversions (5 of 20), volatile (5 of 18), member pointers (9 of 28), dependent names (18 of 47), and instantiation (26 of 67).
   - 125 of the 557 `reject` rules do not appear anywhere in the front end's source. 176 are never the first rule of a diagnostic on a passing test. A low rate on a page points at a front-end shortcut or at a rule boundary the text leaves unclear (decision 0038).

3. **The front end learns from the tests as well as the text.** The front end reads `tests/`, which decision 0035 allows. Some choices come from the tests and not from the rules.
   - G119 says that the front end chose when the set of implicit objects is fixed "from the tests, not from the rule text".
   - G123 to G127 found five gaps in CLS-35 and BF-9, which item TP had just written. Each gap was found because a test asserted a size that the table did not derive.
   - The front end has had 17 rounds. Each round fixed the front end against the tests and filed findings against the text. No reader has built a component from the rules alone and then run the tests as a blind check. Predict batches do this for single outcomes. Nothing does it for a procedure such as class layout or conversion ranking.

4. **The run-time library surface is not stated.** Ten tests declare C library functions (`_Exit` in 7, `malloc` and `free` in 3) to observe a destructor or an allocation. The specification does not say what these functions do, because the charter keeps the library out. An interpreter must guess, or leave the tests unsupported (7 run-unsupported now). Decision 0041 lists the language-support names that the rules use. These three functions are in no list.

5. **Test directories do not match the pages.** The overloading chapter is `specification/overloading/`. Its tests are under `tests/foundations/overload-resolution/` and `tests/foundations/conversion-sequences/`. The harness prints its tables by test directory, so the overloading failures appear under foundations.

## 2. Ranked items

| Rank | Item | Evidence | Cost | Risk | What it unblocks |
|---|---|---|---|---|---|
| 1 | A harness that judges by the specification's answer: the base expectation first, and the whole result set | 99 tests pass by following EDG against the base (81 cite only rules where EDG's answer is not allowed); 1 hidden wrong outcome (`subscript-multi-order.cpp`); SEQ-1 two orders; 3 runs at the bound | small (the statuses), medium (permutations and the lists) | low | counts that mean what they say; decisions 0047 and 0048 as promised |
| 2 | Front-end round 18 | 14 repair tests; 81 tests that agree with EDG against the rule; CONV-11 table unused; 4 false rejects; 65 run-unsupported; VOL-6 and OBJ-12 | medium | low | the phase 7 repairs checked by the front end; the last `undefined` rules at run time |
| 3 | Rule-ID scoring per page, and a reject overlap | 66% rate; 315 rejects name no rule; 89 KW-1 tests; types 0 of 20 | small | low | a list of rule boundaries to check |
| 4 | An Implement exercise: one component from the rules alone | G119; G123 to G127 on a table written in the same phase; 17 rounds of test-driven repair | medium | medium | a measure of whether the text, without the tests, gives a procedure |
| 5 | The run-time library surface | 10 tests declare `_Exit`, `malloc`, or `free`; 7 run-unsupported | small | low | those tests in the interpreter; a stated boundary for the library |
| 6 | Check-side round on the missed rejects | 121 missed rejects; 78 unsupported | large | low | a front end that rejects what the text rejects |

### 1. A harness that judges by the specification's answer

**What it is.**
- `conform` judges each test by its base expectation. A result that matches only the EDG override becomes a status of its own, for example `EDG-AGREES`, which counts as a failure except where the test's rule is `impl-defined`, `unspecified`, `no-diagnostic`, or `conditionally-supported`. `EDG-ONLY` keeps its meaning, which is that the front end follows the specification where EDG differs.
- The 99 tests go into the front-end round (item 2) as 70 missed rejects and 29 false rejects, less the ones whose rules allow both answers.
- `tidal-fe conform` reports a test whose explored runs give more than one outcome. For a rule whose order is fixed (OPR-7), any second outcome is a failure. For a test of an `unspecified` rule, the set is compared with the test's expectation. A new status, for example `RUN-OTHER-OUTCOME`, makes the case visible in `conformance.tsv`.
- `conform` reports a run that the bound stopped, so a partial set is never read as a full one.
- SEQ-1 explores every order of the arguments up to the bound, and not only the two directions.
- A list of the allowed results for each `unspecified` rule where the text gives a finite set. The 22 rules with no choice point each get one of three labels: the interpreter explores it, the matrix table fixes it, or no test can observe it.

**Evidence.**
- Section 1, place 1. `conform.rs` lines 112 to 121 put the EDG override ahead of the base expectation, and lines 205 to 222 map a match with EDG to `Pass`. 99 tests pass this way, among them the NS-15 repair test.
- `subscript-multi-order.cpp` passes with "other outcomes: result 1". OPR-7 fixes the order, so this is a front-end fault that no tool reports.
- 56 runs pass a choice point. 55 agree on every explored run, and 3 stop at the bound.
- `parameter-destruction-order-three.cpp` has six orders, and two run.
- Round 4 asked for this (item 7). Decisions 0047 and 0048 make the set of results the test of an `unspecified` rule.

**What it unblocks.** A pass means that the front end gives the specification's answer, and that every explored outcome is allowed. The pass counts in `status.md` and `exercises.md` then measure the front end against Tidal and not against EDG. Item 2's changes to argument order and `operator[]` can then be checked.

**Cost.** Small for the status and the bound message. Medium for permutations and the list of allowed results.

**Risk.** Low. The pass count drops by up to 99 when the change lands, which is a correction and should be recorded as one. Paths grow fast with permutations, so the bound stays, and the output says when it stopped a run. General knowledge: GCC and Clang evaluate function arguments in different orders on x86-64, so a test with two allowed outcomes is common in practice.

### 2. Front-end round 18

**What it is.**
- The 14 tests from the G118 to G133 repairs (section 1).
- The 81 tests that pass only by agreeing with EDG on a rule that does not allow EDG's answer (item 1).
- CONV-11 from its table: infinity on both targets.
- The four false rejects, with `char-named-escape.cpp` left unsupported (section 3).
- The multi-argument `operator[]` order under OPR-7, which item 1 exposes.
- Run constructs: the `new` forms, constructor calls, block-scope `static` before its declaration ran, jumps into nested statements, a destructor function try block, and the address of an explicit object member function.
- VOL-6 and OBJ-12 in the interpreter. If the interpreter stops them, each gains an interpreter-only test, and the Evidence notes change from "no tool" to "only the interpreter".

**Evidence.** Section 1. `status.md` names 13 of the repair tests as the next front-end step. The interpreter already tracks the kind of each glvalue and the lifetime of each object, so VOL-6 and OBJ-12 need no new model.

**What it unblocks.** Every repair of phase 7 checked by an implementation, and pass counts that hold once item 1 lands. All but ADD-18 of the `undefined` rules stopped at run time by some tool.

**Cost.** Medium.

**Risk.** Low. The round ends with G findings in the usual form.

### 3. Rule-ID scoring per page, and a reject overlap

**What it is.**
- `tidal-fe conform` prints the named-rule rate per page and lists the passing rejects that name no rule.
- Decision 0052's matching extends to `reject` rules that link a general rule, so a diagnostic that names TR-12 matches a test of KW-1. The other choice is for the keyword tests to cite TR-12 as well, under decision 0042.
- A list of `reject` rules that no passing diagnostic names first (176 now). Each one is either a front-end shortcut or a rule that another rule always reaches first.

**Evidence.** Section 1, place 2. Round 4 asked for this (item 6 and process change 3). The harness still prints one total.

**What it unblocks.** A short list of pages where a correct rejection comes with the wrong rule. Types, the one-definition rule, and names come first. Those are candidate rule-boundary findings (decision 0038).

**Cost.** Small.

**Risk.** Low. A front end can raise the rate by naming rules loosely. The rate is a signal for review, and should not become a target.

### 4. An Implement exercise: one component from the rules alone

**What it is.**
- A new exercise kind, or a variant of Write. A fresh clean-room reader builds one self-contained component from the rules, with the `:tests:` lines removed as in Predict, and without reading `frontend/` or `tests/`.
- The first component is the class layout: CLS-31, CLS-33 to CLS-35, BF-9, and ALN. The reader writes a function from a class description to size, alignment, and offsets. A script then runs it on every `sizeof`, `alignof`, and `offsetof`-style assertion in the layout tests and grids.
- A second candidate is the ranking of implicit conversion sequences on the conversion-sequences page.
- Each test that the component gets wrong is a finding, and the report says which rule the reader followed.

**Evidence.**
- Section 1, place 3. G119 says that a front-end choice came from the tests. G123 to G127 found five gaps in a table that the same phase wrote, and each gap surfaced only because a test asserted a value.
- Predict batches check single outcomes and reached 839 of 840. A procedure with many steps, such as layout, needs a reader to apply every step, and Predict does not ask for that.
- The layout is deterministic and needs no parser, so the component is small.

**What it unblocks.** A measure of how far the text alone carries an implementer. A procedure page that passes this check can be trusted by a second implementation, which is the charter's purpose for the specification.

**Cost.** Medium: one reader, one component, a small scoring script.

**Risk.** Medium. The reader may bring layout knowledge from outside Tidal. General knowledge: the matrix targets follow a published platform ABI, and a reader who knows it could fill gaps from memory. The prompt should ask the reader to mark each step that the text did not give, as Propose reports mark outside knowledge.

### 5. The run-time library surface

**What it is.** One of two changes, chosen by the writer:
- The language-support page lists the C library functions that tests may declare (`_Exit`, `malloc`, `free`), with one sentence each on the effect a test relies on. This list is a test convention and holds no rules.
- Or the tests observe the same effects without them, for example by a destructor that sets a global and a `main` that returns it.

**Evidence.** Section 1, place 4. Ten tests declare these functions, and seven of them are run-unsupported.

**What it unblocks.** Seven unwinding and allocation tests in the interpreter. A stated boundary between the language-support names and the library, which an implementer otherwise has to guess.

**Cost.** Small.

**Risk.** Low. The first option widens the library surface by three names, and decision 0041 should say whether a test convention counts as scope.

### 6. Check-side round on the missed rejects

**What it is.** Close the 121 missed rejects and the 78 check-side unsupported tests, grouped by analysis: access after overload resolution (7), defaulted comparisons, consteval escalation, overload sets with no target (OVL-20, 5), the `same` idiom (23), conversion to a function reference (6), and the template checks.

**Evidence.** The failures by page table. No page has more than 9 missed rejects.

**What it unblocks.** A front end that follows the text on every page.

**Cost.** Large, for a modest gain per test.

**Risk.** Low. Each group that the front end cannot close from the text becomes a G finding.

### Dependency sketch

```
1 harness by the specification ──> 2 front-end round 18 ──> 6 check-side round
3 rule-ID scoring (independent, small)
4 Implement exercise (independent; best before more layout repairs)
5 library surface ──> 2 (seven run tests)
```

Items 1, 3, and 5 are small and can start at once. Item 4 needs a prompt in `documentation/exercises.md` and can run beside item 2.

## 3. Scope

| Topic | Charter's reason to keep out | Implementer's view | Proposal |
|---|---|---|---|
| The launder operation and `<new>` | the general library | No Write program and no test needs it. STG-19 keeps one way out that no test exercises. | Keep out, as decision 0051 says. |
| C library functions in tests (`_Exit`, `malloc`, `free`) | the general library | Ten tests declare them, and an interpreter must guess their effect. | Keep the library out. State the three as a test convention, or remove them from tests (item 5). |
| Named escapes `\N{...}` | lexical structure beyond G1 | One test needs a Unicode name table, which is data and holds no rules. | Keep the test. The front end may leave it unsupported. |
| Lexical structure beyond G1 | similarity risk | No phase 7 finding needed a new lexical rule. | Keep out. |
| Preprocessor beyond `#include "file"` | the library comes with real headers | The front end handles the test headers and the multi-unit tests. | Keep out. |
| Coroutines, modules, threads, the memory model | outside the language goal | No test or front-end path needs them. | Keep out. |
| Run tests with `-m32` | the host compilers have no 32-bit C library | The interpreter runs no `-m32` test either. The `-m32` choices are checked in constant evaluation. | Keep as is. |
| `typeid` | needs a library shape | One test uses it. | Keep out. |
| Later language versions (Q-0007, pending item 7) | the charter fixes C++23 | An index of later changes would not change what the front end must do. | Keep out of the front end's work. |
| MSVC | not in the matrix | A second target ABI would double the CLS-35 and BF-9 tables. | Keep out. |

## 4. Process changes

1. **Fail on a hidden outcome.** `frontend_check.py` ranks statuses, so a change from one failing status to another passes, a pass that agrees only with EDG passes, and a pass with a second outcome passes too. With item 1's statuses, the check fails on each.
2. **Report the run side in `status.md`.** Round 4 asked for this. `status.md` gives 6068 full passes and 6134 check passes, and no run counts. Add one line: 1491 of 1564 runs pass, 69 are unsupported, 1 gives a wrong result. Add the count of tests that pass only by agreeing with EDG until item 1 lands.
3. **Report the named-rule rate per page** (item 3). Round 3 and round 4 both asked for this.
4. **Name the front-end status of each test a repair adds.** The commit `2eb7c8d` added 41 tests. 13 of them fail in the harness, and 14 fail by the specification's answer. `status.md` lists them as a next step, which works. The repair commit could say the same per test, so a later reader sees which tests wait on the front end and which on the text.
5. **Align test directories with the pages.** Move `tests/foundations/overload-resolution/` and `tests/foundations/conversion-sequences/` under `tests/overloading/`, or have the harness group by the page of the first rule. Then the per-directory tables match the chapters.
6. **Keep the front-end report form.** Each round ends with G findings and the conformance tables. Two tables would help item 1 and the `undefined` work: the choice-point tests with their explored outcomes, and the `undefined` rules with the rule the interpreter names for each.
