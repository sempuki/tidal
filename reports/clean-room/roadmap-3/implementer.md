# Roadmap, round 3: what to do after phase 5, from an implementer's view

Date: 2026-11-01. Decision 0029. Role: clean-room reader, acting as an implementer who builds a C++ front end and interpreter from Tidal alone.

Inputs:
- AGENTS.md, CLAUDE.md
- `documentation/`: charter, roadmap, status, exercises, and decisions 0033, 0039, 0042, 0047, 0048, 0050
- `reports/clean-room/roadmap-2/implementer.md`, which is the model for this report's form
- `reports/clean-room/rust-frontend/report-9.md` and `conformance.tsv`
- the phase 5 exercise reports: W15, D-classes, R-templates (reports A to C), X5 to X8
- `specification/` (translation, evaluation-order, storage, types, keywords, exception-specifications, the glossary, and the cited rules), `questions/README.md`, Q-0312
- the commit messages of the phase 5 repair commits

**Measurements.** I ran the committed binary (`frontend/target/release/tidal-fe conform ../tests -v`) read-only, with its output in the scratchpad. Its statuses match `conformance.tsv` on all 5408 tests, so the numbers below are the committed baseline. I also ran `tidal-fe run` on each of the 1143 `run` tests to count choice points. Counts come from grep and short Python scripts over `specification/`, `tests/`, and the harness output, so they are approximate. Statements from general knowledge say so.

| Measure | Value |
|---|---|
| Check, whole suite | 5408 tests: 4675 pass (4570, plus 105 that differ from EDG only), 74 false rejects, 347 missed rejects, 312 unsupported |
| Check, `templates/` | 925 of 1104 (84%). Round 2 had 537 of 921 (58%). |
| Check, outside `templates/` | 3750 of 4304 (87%) |
| Check, by expectation | accept 1576/1802, reject 1874/2355, link-fail 36/36 |
| Rejects that name one of the test's rules | 1277 of 1874 (68%) |
| Run tests (`run`, `run-fail`) | 1212: 946 pass, 6 wrong results, 3 missed undefined behavior, 4 unexpected undefined behavior, 225 unsupported, 28 not run |
| Runs that pass a choice point | 37 `run` tests. The interpreter takes one fixed choice. |
| Rules by class | 1633: 875 required, 547 reject, 56 undefined, 43 impl-defined, 43 recommended, 35 no-diagnostic, 29 unspecified, 4 conditionally-supported, 1 withdrawn |
| Questions | 288 provisional, 20 compiler defect, 12 settled, 1 open |

## 1. Where the specification still stops an implementer

The text now stops the front end in few places. All findings of round 9 (G76 to G84) are repaired. Readers who used only the text did much better than the front end on the same pages:

| Reader | Pages | Score |
|---|---|---|
| Predict readers (R-templates) | the 11 template pages | 1089 of 1092 tests |
| Front end | the same pages | 925 of 1104 tests |
| Diagnose reader (D-classes) | virtual, handlers, unions, conversion functions | 143 of 143 rules named, 143 fixes compile |
| Front end | the same four pages | 79 missed rejects |

Most remaining failures are therefore front-end work. Predict answers are coarse (compiles, rejects, runs), and the readers also read the Evidence segments. So the comparison shows that the text decides the outcome. It does not show that the text gives an algorithm.

### Failures by page

The page is the page of the first rule that the test cites. Run columns count `run` and `run-fail` tests whose check passed.

| Page | False reject | Missed reject | Unsupported | Run unsupported | Run wrong or wrong stop | Rules most cited |
|---|---|---|---|---|---|---|
| foundations/keywords | 0 | 0 | 79 | 0 | 0 | KW-1 (79) |
| expressions/new-delete | 1 | 3 | 11 | 38 | 0 | NEW-21, NEW-23, NEW-2, NEW-28, NEW-24 |
| classes/virtual | 0 | 28 | 5 | 2 | 2 | VIRT-15, VIRT-3, VIRT-12, VIRT-2 |
| classes/unions | 0 | 17 | 15 | 4 | 0 | UNI-6, UNI-14, UNI-8, UNI-7 |
| foundations/overload-resolution | 3 | 10 | 21 | 2 | 0 | OVL-20, OVL-29, OVL-27, OVL-17 |
| expressions/member-pointers | 0 | 12 | 3 | 19 | 0 | MPTR-10, MPTR-3, MPTR-6, MPTR-4 |
| templates/dependent-names | 4 | 18 | 12 | 0 | 0 | DEP-24, DEP-15, DEP-19, DEP-30 |
| templates/packs | 15 | 11 | 8 | 0 | 0 | PACK-8, PACK-16, PACK-14, PACK-13 |
| foundations/storage | 0 | 2 | 7 | 21 | 0 | STG-5, STG-9, STG-13, STG-16, STG-14 |
| classes/conversion-functions | 0 | 15 | 4 | 10 | 0 | CVF-7, CVF-1, CVF-11, CVF-2 |
| classes/operators | 0 | 9 | 5 | 15 | 0 | OPR-8, OPR-10, OPR-3 |
| exceptions/handlers | 0 | 19 | 2 | 5 | 0 | TRY-5 (11), TRY-7, TRY-2 |
| templates/class-template-deduction | 5 | 8 | 13 | 0 | 0 | CTAD-1, CTAD-9, CTAD-27, CTAD-26 |
| declarations/structured-bindings | 0 | 7 | 11 | 7 | 0 | SB-5, SB-13, SB-15 |
| classes/derived | 0 | 13 | 2 | 8 | 0 | DER-20, DER-8, DER-7 |
| classes/comparisons | 3 | 15 | 2 | 1 | 0 | DCMP-9, DCMP-13, DCMP-12, DCMP-11 |
| expressions/lambdas | 0 | 13 | 4 | 3 | 0 | LAM-17, LAM-26, LAM-15 |
| templates/specialization | 9 | 5 | 1 | 1 | 0 | TSP-14 (5), TSP-22, TSP-21 |
| declarations/exception-specifications | 1 | 3 | 11 | 0 | 0 | EXS-4 (9) |
| foundations/language-support | 0 | 2 | 3 | 6 | 1 | LSUP-10, LSUP-8, LSUP-5 |

### Front-end gaps (the text is enough)

- **Missed rejects.** 306 of the 347 cite a `reject` rule. 31 cite only `recommended` rules (warnings under `--werror`), and 5 cite only `no-diagnostic` rules (for example DEP-30). The D-classes result shows that the text decides the classes misses.
- **False rejects.** 47 of the 74 are in `templates/`. Three clusters stand out:
  - packs: 15 tests, mostly `static_assert` failures on valid code (PACK-13, PACK-16, PACK-21)
  - placeholder patterns in partial specializations: 5 tests (TSP-14, DED-12)
  - rules added after round 9 that the front end does not follow yet: PFX-20 (2 tests, where the front end still reports PFX-14) and ICS-47 (1 test)
- **Check-side unsupported constructs:**

  | Construct | Tests |
  |---|---|
  | the `same` idiom with arguments the front end cannot compute | 34 |
  | the `noexcept` operator (EXS-4) | 22 |
  | designated initializers in constant evaluation | 18 |
  | generic lambdas | report-9, section 4 |

- **Interpreter stops:**

  | Construct | Run tests |
  |---|---|
  | `new` with a class-specific or replaced allocation function (NEW-21 to NEW-24, NEW-32); 30 of these are in new-delete | 41 |
  | `reinterpret_cast` to another object type (STG-12 to STG-14, STG-23, CLS-19, STG-16) | 29 |
  | pointers to members | about 23 |
  | inherited constructors, anonymous unions, and the `initializer_list` backing array | 11 |
  | structured bindings with `get` | 8 |
  | user-defined literals | 8 |
  | virtual bases | 7 |

- **Wrong results and missed stops:**
  - Three missed stops: a double destruction (DTOR-7), delete through a base with no virtual destructor (VIRT-12), and a write to a `const` object (CV-5).
  - Six wrong results: ILC-3, SINIT-1, SINIT-7, CMP-26, MUL-11 with MUL-20, and LSUP-8. The text of each rule states the result.

### Places where the text still stops the implementer

1. **No rule says that a program the grammar does not derive is rejected.**
   - `translation.md` has no rule on syntax. TR-P1 lists the classes of rules, and none of them covers a failed parse.
   - Q-0312's provisional answer says "A program that no production accepts is not valid". The answer states the rule, but no rule in the specification does.
   - The front end stops at a parse error on 127 tests. 106 of them expect reject (79 are KW-1), and 21 expect accept or run.
   - The harness counts each of the 127 as unsupported, because a parse error names no rule.
   - Round 2 ranked this item third, and phase 5 did not take it up.

2. **Overlapping `undefined` rules have no convention.** Decision 0042 covers only `reject` rules. Four new sanitizer tests stop in the interpreter at a general rule, while the test cites a specific one:

   | Test | Rule the test cites | Rule the interpreter names |
   |---|---|---|
   | `returned-reference-temporary-sanitizer.cpp` | REF-5 | OBJ-2 |
   | `decltype-auto-paren-local-sanitizer.cpp` | RET-6 | OBJ-2 |
   | `string-modify-sanitizer.cpp` | PRIM-8 | CAST-5 |
   | `delete-array-through-base-sanitizer.cpp` | VIRT-12 | NEW-5 |

   Each stop is correct. No rule says which rule a stop should name, and no general rule links to its specific cases.

3. **Some `undefined` rules have no test that the interpreter runs.**
   - These 15 `undefined` rules have no `run-fail` test: ADD-11, ADD-14, ADD-17, ADD-18, CTOR-12, INIT-1, INST-15, MUL-12, MUL-14, NEW-3, OBJ-10, SEQ-4, STG-23, TRY-11, VOL-6.
   - The repair of G78 moved seven run tests of STG-19, STG-20, STG-23, and VOL-6 to `accept`, because no sanitizer stops them. The interpreter does stop STG-19 and STG-20, but an `accept` test never runs. Its stop is now unchecked.
   - Decision 0048 wants the harness to expect a stop that names the rule. The test format has no expectation for "compilers build this and the run is undefined".

4. **Class layout beyond member padding.**
   - TYP-12 gives padding between members only. BF-9 is `impl-defined` with a partial table. The text has no per-target table for these:
     - where a base subobject goes,
     - the size of an empty base (STG-3 states only the facts),
     - the hidden pointer of a polymorphic class,
     - virtual bases.
   - Five tests stop on `sizeof` for this reason: `sizeof-at-least-base.cpp` (DER-2), `bitfield-layout-lp64.cpp` and `bitfield-zero-width-splits.cpp` (BF-9), and `empty-base-takes-no-bytes.cpp` and `empty-subobjects-same-type-distinct.cpp` (STG-3).

5. **The class-property lists are incomplete.**
   - CPY-19 (trivially copyable) and CLS-19 (standard-layout) still end with *(class properties referenced)*.
   - The interpreter needs both lists for byte copies (STG-12), first-member pointers (CLS-19, STG-16), and implicit-lifetime types (STG-7).
   - Items S and T wait for a decision on how to observe a property that only the library names (decision 0050).

6. **The launder operation.**
   - STG-P4 and STG-19 send a program to the launder operation when old names do not reach a new object. The operation is *(library referenced)*.
   - W15-2 reports that a program cannot declare `std::launder` itself, because the declared name fails to link.
   - So no test can exercise the one way out of STG-19, and the interpreter has nothing to model.

7. **Choice points.**
   - Decision 0048 allows the interpreter to try each allowed choice. It takes one fixed choice.
   - 37 run tests pass a choice point. The rules named most often are SEQ-8 (17 times) and SEQ-1 (16 times), then PRIM-9, CMP-8, and CMP-20 (twice each).
   - The interpreter does not detect SEQ-4. The text is now enough for that check. SEQ-3, SEQ-8, the glossary entry *unsequenced*, and the assignment rules together say which accesses can overlap.

## 2. Ranked items

| Rank | Item | Kind | Size | Risk |
|---|---|---|---|---|
| 1 | Byte-level memory in the interpreter | front end | large | medium |
| 2 | A grammar-validity rule, and parse errors as rejects | specification + front end | 1 or 2 rules; medium code | medium |
| 3 | Undefined behavior: overlap convention and interpreter-checked tests | decision + test format | small | medium |
| 4 | Interpreter coverage of specified constructs, and the wrong results | front end | medium to large | low |
| 5 | Front-end round 10 on the check side: template false rejects, classes and exceptions missed rejects | front end | large | low |
| 6 | Predict triage of the remaining front-end failures | exercise | medium | low |
| 7 | Choice exploration and SEQ-4 detection in the interpreter | front end | medium | medium |
| 8 | Class-property lists and per-target class layout | specification | 6 to 10 rules and tables | medium |
| 9 | The launder operation as a language-support name | scope (narrow add) | 1 or 2 rules, 3 to 5 tests | low |

### 1. Byte-level memory in the interpreter

**What it is.** The interpreter holds abstract values. It needs a second view of each object as `sizeof` bytes, so that it can do the following:
- read and write through `unsigned char`, `char`, and `std::byte` (STG-13),
- copy bytes (STG-12, CPY-19),
- convert pointers with `reinterpret_cast` (CAST-6, STG-16, CLS-19),
- stop with STG-23 on an access through a type that STG-14 does not list.

**Evidence.**
- 29 run tests stop at `reinterpret_cast`, and 2 stop at "bytes of storage that holds an object of another type".
- The storage page has 21 run tests that the interpreter does not run.
- STG-23 is not detected (report-9, section 3), and UNI-6 is detected only in part.
- The text is in place:
  - STG-11 to STG-14 and STG-23 define the bytes and the allowed access types.
  - A storage rule fixes the byte order (little-endian) and two's complement.
  - MUL-20 gives the IEEE formats.

**What it unblocks.**
- The STG, CLS-19, and STG-16 run tests.
- A run-time witness for STG-23 and UNI-6.
- Item 9, if it is adopted.
- The first real check of the storage page, which phase 5 wrote and which no implementation has run.

**Size.** Large. Report-9 lists the object representation as the main construct still missing.

**Risk.** Medium. The two views of an object must stay in step. Any byte the text does not fix, such as padding values or the bytes of a pointer, will surface as a finding. No specification text is written, so the item carries no similarity risk.

### 2. A grammar-validity rule, and parse errors as rejects

**What it is.**
- One `reject` rule on `translation.md`: a token sequence that the grammar does not derive is not a valid program. A diagnostic for such a program need not name a rule.
- KW-1 and the disambiguation rules cite the new rule.
- The front end then reports a parse failure as a reject, and keeps "unsupported" for constructs it knows it does not model.

**Evidence.**
- 127 parse stops: 106 expect reject (79 KW-1), and 21 expect accept or run.
- Q-0312's provisional answer already states the principle.
- The grammar is complete under decision 0040.

**What it unblocks.**
- About 100 reject tests.
- A clean meaning for "unsupported".
- A list of 21 valid programs that the front end's parser does not derive, each a parser gap against a named production.
- An answer for Q-0312.

**Size.** One or two rules. Medium front-end work, because the parser must follow the productions closely enough for a failure to mean "not valid".

**Risk.** Medium. Where the grammar differs from the compilers, the front end will give false rejects. Each one is a grammar finding. The similarity risk is low, because the statement is short and generic.

### 3. Undefined behavior: overlap convention and interpreter-checked tests

**What it is.** Two small changes.

- **Overlapping rules.** Extend decision 0042 to `undefined` rules.
  - A test cites the most specific rule.
  - A general rule (OBJ-2, CAST-5, NEW-5, UN-7) names its specific cases (REF-5, RET-6, PRIM-8, VIRT-12, PFX-21).
  - A stop that names either rule matches.
- **A run expectation for the interpreter.** Add a test expectation for undefined runs that no sanitizer stops on both compilers. The matrix compilers build the program and do not judge the run. The interpreter must stop and name one of the test's rules. A per-implementation line in the style of `tidal-expect-edg` would do.

**Evidence.**
- Four RUN-UNEXPECTED-UB results, each the general rule for the test's specific rule (section 1, place 2).
- 15 `undefined` rules have no `run-fail` test.
- Seven tests moved from `run` to `accept` under G78. Two of them (STG-19, STG-20) are stops that the interpreter already makes and that nothing checks now.

**What it unblocks.**
- A run-time witness for each `undefined` rule.
- The four unexpected stops become passes.
- Decision 0048's harness rule applies to every `undefined` rule.

**Size.** One decision, a change to `specification-format.md` and the runner, and about 20 test lines.

**Risk.** Medium. The evidence is circular, because the interpreter comes from the rules that it would confirm. The decision should say that an interpreter stop shows only that the program reaches the named operation. The rule's class still rests on compiler evidence (decision 0047). No specification text changes, so the similarity risk is low.

### 4. Interpreter coverage of specified constructs, and the wrong results

**What it is.** Front-end work on constructs whose pages are finished:
- `new` with class-specific and replaced allocation functions, and deallocation after a throw (NEW-21 to NEW-24, NEW-32),
- pointers to members,
- overloaded `operator[]`, `operator->`, and conversion to `bool`,
- user-defined literals,
- virtual bases,
- structured bindings with `get` (SB-5, SB-17),
- inherited constructors, anonymous unions, and the `initializer_list` backing array,
- the three missed stops (DTOR-7, VIRT-12, CV-5),
- the six wrong results (ILC-3, SINIT-1, SINIT-7, CMP-26, MUL-11, LSUP-8).

**Evidence.** 225 run tests are unsupported. 41 stop at `new`, about 23 at pointers to members, 11 at the class model, 8 at structured bindings, 8 at user-defined literals, and 7 at virtual bases. The wrong result for ILC-3 has been open since round 7.

**What it unblocks.** About 120 run tests. Run-time witnesses for NEW-32, LAM-19, and PFX-19.

**Size.** Medium to large.

**Risk.** Low. A rule gap found during the work becomes a G finding. The item writes no text.

### 5. Front-end round 10 on the check side

**What it is.** Close the check gaps on finished pages:
- the 47 template false rejects (packs, TSP-14 placeholder patterns, CTAD-1 sites),
- the 147 missed rejects in `classes/` and `exceptions/` (TRY-5, VIRT-3, VIRT-15, CVF-7, DCMP-9 to DCMP-13),
- the `noexcept` operator (EXS-4),
- generic lambdas,
- designated initializers in constant evaluation,
- the rules added after round 9 (PFX-20, PFX-21, ICS-47, CNS-39, CMP-27, FN-15, NAM-37, NAM-38, DEP-39, TPL-48, TPL-49, SB-17, CE-30, INST-36, UNI-14).

**Evidence.**
- 74 false rejects, 306 missed rejects that cite a `reject` rule, and 312 unsupported tests.
- The D-classes and R-templates scores show that the text decides these outcomes.

**What it unblocks.**
- A front end that follows the current text, which item 6 needs as a clean baseline.
- The `same` idiom in 34 tests.
- The 31 warning tests, if the front end adds the `recommended` warnings.

**Size.** Large.

**Risk.** Low. The round should end with a report in the usual form, so that any text gap it finds is recorded.

### 6. Predict triage of the remaining front-end failures

**What it is.**
- A Predict batch built from the tests that the front end fails outside the pages that phase 5 already covered (templates, the four D-classes pages, Foundations). That leaves about 500 tests.
- Each test that the reader predicts correctly from the text is a front-end gap. Each wrong or "depends" answer is a candidate text gap.
- The batches also fill the gap in the catalog, because no Predict batch exists yet for expressions, declarations, classes, statements, or exceptions.

**Evidence.**
- The R-templates and D-classes scores show the method works and is cheap.
- `documentation/exercises.md` lists all other Predict batches as planned.
- In round 9, every finding came from the front-end author's own judgment of which failures were text gaps.

**What it unblocks.** A ranked list of text gaps outside templates, with no one's judgment between the failure and the finding. A cleaner signal for items 4 and 5.

**Size.** Medium: five or six batches, built by `tools/exercises.py`.

**Risk.** Low. Readers read the Evidence segments, so a correct prediction can come from a compiler table and not from a rule. The batch README should ask the reader to cite the rule that decides, and the scoring should count a match only when the cited rule is one of the test's rules.

### 7. Choice exploration and SEQ-4 detection in the interpreter

**What it is.**
- At each choice point under SEQ-1, SEQ-8, PRIM-9, CMP-8, or CMP-20, the interpreter runs each allowed choice up to a bound and reports the set of results.
- Within one full expression, the interpreter records the reads and writes of operands that SEQ-8 leaves unsequenced. It stops with SEQ-4 on a conflict.

**Evidence.**
- 37 run tests pass a choice point with one fixed choice.
- SEQ-4 has no `run-fail` test and no sanitizer support.
- Decision 0048 promises the set, and decision 0047 makes the set the test of an `unspecified` rule.

**What it unblocks.**
- A check that each `unspecified` rule's set contains every matrix result.
- `run-fail` tests for SEQ-4 under item 3.
- A precise SEQ-7 warning.

**Size.** Medium.

**Risk.** Medium. The number of paths grows quickly, so the run needs a bound, and a run that hits the bound should say so. No specification text changes.

### 8. Class-property lists and per-target class layout

**What it is.**
- Complete CPY-19 (trivially copyable) and CLS-19 (standard-layout), and decide items S and T.
- Add per-target tables (decision 0036) for the placement of base subobjects, empty bases, the hidden pointer of a polymorphic class, virtual bases, and bit-field allocation (BF-9). The rule class is `impl-defined`.

**Evidence.**
- Two *(class properties referenced)* markers remain.
- Five tests stop on `sizeof` of such classes.
- Item 1 needs the property lists for STG-12, STG-16, and CLS-19.
- On observation, each matrix compiler has builtin queries for these properties (general knowledge). A source reader or a clean-room writer can probe them for Evidence while the tests keep observing behavior only. The decision 0050 question on how to observe a property can be answered this way.

**What it unblocks.**
- Byte copies and layout in the interpreter.
- The five `sizeof` tests.
- A defined meaning for "trivially copyable" in STG-12 and for "standard-layout" in STG-16.
- Removal of the last two class-property markers.

**Size.** 6 to 10 rules, two tables, and about 15 tests.

**Risk.** Medium.
- The property lists have well-known fixed wording, so the similarity risk is medium to high. The method controls should apply: a source-reader inventory first, then probe grids.
- Layout facts come from a platform convention outside ISO (general knowledge), so the tables carry less risk.

### 9. The launder operation as a language-support name

**What it is.** Extend decision 0041 with the launder operation. A rule would say what the language requires of it: the result reaches the new object of STG-19. Tests could use it under one of two options:
- **(a)** allow `#include <new>` as the only standard header, for this name and the placement forms, or
- **(b)** keep the marker and document why.

**Evidence.** Four *(library referenced)* markers on `storage.md` name the operation. W15-2 shows that a generic container needs it, or must keep the pointer that `new` returns. A declared `std::launder` fails to link.

**What it unblocks.** A tested path for STG-19, a modelled operation in the interpreter, and removal of four markers.

**Size.** One or two rules, three to five tests, and a header policy line.

**Risk.** Low for the text. Option (a) is a scope change, which section 3 weighs.

### Dependency sketch

```
3 UB convention ──> 7 choices and SEQ-4 ──┐
1 byte memory ──> 8 property lists ──> 9 launder
2 grammar rule ──> 5 front-end round 10 ──> 6 Predict triage
4 interpreter coverage (independent; feeds 1 and 7)
```

Items 2 and 3 are small and can start at once. Item 6 needs item 5 to finish first, so that its failure list matches the current text. Item 8's text should come after item 1 shows which bytes the interpreter needs.

## 3. Scope changes

| Topic | Charter's reason to keep out | Implementer's view | Proposal |
|---|---|---|---|
| Lexical structure beyond G1 | high similarity risk, little test value | The grammar-validity rule (item 2) needs defined terminals. The front end's lexer passes every lexical test except two named-escape tests that need a Unicode name table. | Keep out. Add rows to the G1 tables only when item 2 finds an unclear terminal. |
| Preprocessor beyond `#include "file"` | real headers bring the library | The front end handles `#include "file"`, guards, and `#pragma once`. 62 tests include test headers, and none includes a standard header. | Keep out. Option (a) of item 9 would allow one header, `<new>`, whose names Tidal already defines as shapes. The front end would treat that header as built in and would not parse it. |
| General library | a large surface, and outside the charter's language goal | The launder operation is the one library name whose absence blocks a specified rule. `construct_at` already works as a program-declared shape (LSUP-10). | Keep out, with the narrow add of item 9. Weigh it against the charter: one header opens a door that later requests will push on. The decision should list the allowed names and close the list. |
| `typeid` and `std::type_info` | a program cannot declare a usable shape | No change since round 2. One test uses `typeid`. | Keep out. |
| Coroutines | large, and needs library traits | No test or front-end path depends on coroutines. One marker remains. | Keep out. |
| Extended floating-point types | little value, platform-specific | Four markers remain. No test needs the types. | Keep out. |
| Memory model and threads | out of the language goal | `thread_local` and the once-per-thread rule are in scope without a memory model. The interpreter is single-threaded. | Keep out. |
| Class-property lists (items S and T) | deferred until properties can be observed | Item 1 needs them. Builtin queries give Evidence without tests that use them. | Bring in as item 8, with method controls. |

## 4. Process changes

1. **Score the rule ID in the front-end harness.**
   - 1277 of 1874 correct rejects name one of the test's rules. The other 597 pass with a wrong rule or none.
   - Report the named-rule rate per page beside the pass rate, as the Diagnose exercises do. A low rate on a page points at `reject` rules whose boundaries the text leaves unclear (decisions 0038, 0042).
2. **Report one front-end number.**
   - `status.md` gives 4439 of 5408, which counts only `pass` and leaves out the 105 EDG-only passes and the run statuses.
   - `exercises.md` gives 4439 and 4675 together.
   - Use two columns: check pass (4675) and run pass (946 of 1212).
3. **Fix a stale limit in `status.md`.** "The front end has no preprocessor, so it rejects the valid tests that use `#include`" is no longer true. The front end reads `#include "file"` with guards (front-end README, INC rules).
4. **Show the front-end effect of each text repair.**
   - The repair commits after round 9 added about 15 rules. Tests that cite those rules now fail in the front end (PFX-20, ICS-47), and nothing flags them, because `frontend_check.py` reports only tests that stop passing.
   - A repair commit could list the tests of each new or changed rule whose front-end status is not `pass`. Or `status_table.py` could add a front-end column per page.
5. **Decide what an interpreter stop is evidence of** before item 3 adds the new expectation. The decision should say that a stop shows only that the test reaches the named operation, and that a rule's class rests on compiler evidence (decision 0047).
6. **Make Predict scoring cite the deciding rule.** R-templates readers sometimes answered from Evidence tables of compiler results. For item 6, a correct outcome with a wrong or missing rule should count as a finding, so that the triage separates text that decides from Evidence that reports.
7. **Answer Q-0312 with item 2.** Its provisional answer states a rule that the specification does not have. Record such cases, where a question's answer stands in for a missing rule, as findings and not only as answers.
8. **Keep the front-end report form.** Each round should end with findings (G85 onward) and a table of `undefined` rules with their witnesses, as report-9 section 3 does. The next roadmap round can then measure items 1, 3, and 7 directly.
