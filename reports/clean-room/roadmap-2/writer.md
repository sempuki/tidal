# Roadmap, round 2: what to do after phase 4, from a clean-room writer's view

Date: 2026-10-31. Decisions 0029, 0033, 0044, 0048. A clean-room agent working in the writer's role prepared this report. It read AGENTS.md, CLAUDE.md, `documentation/` (with decisions), `specification/`, `tests/`, `questions/`, `reports/clean-room/` (with the round-1 roadmaps), `frontend/` reports, and `tools/`. It also compiled four small probe programs on the host Clang 22.1.8 and GCC 16.2.1, using `-fsyntax-only` in the scratchpad. It did not read `inputs/`, `reports/review/`, or `reports/similarity/`. Similarity estimates therefore come from Tidal's own text and the round-1 flag patterns. The report wrote no repository files.

## Baseline numbers

Counts come from read-only `grep` and `awk` over the pages and tests.

| Measure | Templates chapter (11 pages) | All other pages (66) |
|---|---|---|
| Rules | 352 | 1189 |
| Test files | about 960 | 4096 cited |
| Tests per rule | 2.7 | 3.4 |
| Single-test rules | 116 (33%) | 330 (28%) |
| Tests with a per-compiler override | 122 (13%), 58 of them EDG | 264 (6%) |
| Questions cited | 83 (0.24 per rule) | about 176 (0.15 per rule) |
| Probe grid files | 17 across 11 pages | 23 on two pages (conversion sequences, overload resolution) |
| Front end (`conformance.tsv`) | 58 pass of 379 listed | 3386 pass of 3969 listed |

- `documentation/status.md` has no rows for the templates chapter. `tools/status_table.py` loops over six sections and leaves out `templates`. The Similarity column is the literal string "0 flagged" in the tool, so it does not count anything.
- The questions index has 270 entries: 253 provisional, 16 compiler defect, 1 settled. 74 index rows give "option 1" as the provisional answer, which a reader of the index cannot interpret.
- Templates cost more per rule than the earlier chapters. They raise about 1.6 times the questions per rule and twice the override rate. A new chapter of similar difficulty should be budgeted at about 15 rules per session, against 20 for the earlier chapters.
- The front-end round 7 gave 11 findings (G57 to G67) against existing pages. Spot checks show that MUL-11 and VIRT-10 have since been repaired.

## 1. Candidate items, ranked

| Rank | Item | Probes | Similarity risk | Questions | Sessions |
|---|---|---|---|---|---|
| 1 | T0. Close phase 4 | ~20 | low to medium | 2-4 | 1-1.5 |
| 2 | T1. Thicken the templates chapter | 150-200 | low | 3-6 | 2-3 |
| 3 | FE8. Findings from front-end round 8 | ~20 per round | low | 2-5 | 1-2 per round |
| 4 | U. Settle the arithmetic and comparison pages | 30-40 | low | settles ~6, adds 0-2 | 1.5-2 |
| 5 | K2. Language-support types, round 2, with `typeid` (T) | 30-50 | medium | 2-4 | 1-1.5 |
| 6 | V. Thicken thin pages | 50-60 | low; medium for evaluation order | 0-3 | 1.5-2 |
| 7 | X. Digest overrides | 20-30 | none | closes several | 1-2 |
| 8 | W15. Template exercises: a Write suite and Predict batches | 0 up front | low | 1-3 | 1-2 of repairs |
| 9 | S. Class property lists | 20-30 | high | 2-3 | 1 after a decision |
| 10 | Scope: coroutines (keep out) | 0 now; 100-150 if adopted | high | 6-10 if adopted | 0.25 now; 4-6 if adopted |
| 11 | Scope: extended floating-point types (keep out) | 0 now; ~30 if adopted | medium | 3-5 if adopted | 0.25 now; 1 if adopted |

Total for items 1 to 9, with the two scope decisions: about 11 to 16 sessions before review.

### 1. T0. Close phase 4

- **What.** Make the repairs from the gap reviews of items R and P (status next step 1). Add `templates` to the section list in `tools/status_table.py`. Make the Similarity column read the latest report and stop printing a constant. Run the similarity check over the whole chapter.
- **Evidence.** The status table has 66 rows and none for templates. The tool's loop is `["foundations", "statements", "expressions", "declarations", "classes", "exceptions"]`. The working tree has 30 or more untracked tests under `tests/templates/class-template-deduction/`, so the R repairs are in progress.
- **Unblocks.** Honest counts for every later choice, including this report's per-page numbers.
- **Size.** 1 to 1.5 sessions.
- **Risk.** Low for the tool change. The concepts repairs carry medium similarity risk, because satisfaction, normal form, and subsumption use dense terms of art. Round 1 found that dense definitions are the units most likely to be flagged (MUL-8, OBJ-9, MPTR-7).

### 2. T1. Thicken the templates chapter

- **What.** Give every single-test template rule a second test, and add probe grids where the chapter has one or none.

  | Page | Rules | Tests cited | Single-test rules | Grid files |
  |---|---|---|---|---|
  | packs | 25 | 35 | 17 | 1 |
  | concepts | 35 | 68 | 17 | 0 |
  | deduction | 39 | 122 | 14 | 2 |
  | declarations | 47 | 110 | 13 | 0 |
  | dependent-names | 38 | 122 | 13 | 3 |
  | constraints | 32 | 91 | 9 | 2 |

- **Evidence.** 116 single-test rules. 2.7 tests per rule, against 3.4 elsewhere. Fold expressions appear in 3 pages and 14 tests. `explicit(bool)` has 3 tests and no page text found by `grep`.
- **Unblocks.** Front-end round 8 gets decisive tests. Predict batches on the chapter become possible (item 8).
- **Size.** 2 to 3 sessions. Most of the work is new tests. A few rules split where a second test shows that two conditions are fused.
- **Risk.** Low. Tests are written fresh and are outside the similarity corpus. The new grids follow decision 0044.

### 3. FE8. Findings from front-end round 8

- **What.** Answer the findings the front end raises while it adds templates (status next step 2).
- **Evidence.** In round 7, 141 template tests were missed rejects and 168 were unsupported. The interpreter ran 2 of 28 template run tests. The suite now has 92 template run tests. Round 7 gave 11 findings on pages that had been stable for some time. A first pass over 352 new rules will give more, an estimated 20 to 40.
- **Unblocks.** The interpreter for templates (decision 0048), and the conformance signal for the whole chapter.
- **Size.** 1 to 2 sessions per round, as findings arrive.
- **Risk.** Low. Findings come from Tidal text. Watch for findings marked "from outside knowledge" (G60, G61, G63 in round 7). These show where the front end filled a gap from memory. Each one needs a probe before the writer adopts it.

### 4. U. Settle the arithmetic and comparison pages

- **What.** Settle the provisional questions and the floating-point classes on the multiplicative, additive, and comparison pages.
- **Evidence.**

  | Page | Rules | Tests | Single-test rules | Overrides | Questions |
  |---|---|---|---|---|---|
  | multiplicative | 20 | 39 | 9 | 3 | Q-0001 to Q-0005 |
  | additive | 16 | 41 | 9 | 6 | Q-0001, Q-0005, Q-0031, Q-0049 |
  | comparison | 25 | 67 | 9 | 9 | Q-0029, Q-0030, Q-0050, Q-0051, Q-0103, Q-0104 |

  MUL-11 is now `impl-defined` for IEEE 754 types. MUL-12 (floating overflow) is still `undefined`, with one constant-evaluation test. That pair is the conflict that G58 reported.
- **Unblocks.** Write suite W1 (numbers and bits), Predict batches for the expressions pages, and Propose exercises that change arithmetic.
- **Size.** 1.5 to 2 sessions.
- **Risk.** Low. The probes are sanitizer, `-m32`, and constant-evaluation pairs, and the runner supports all three.

### 5. K2. Language-support types, round 2, with `typeid` (T)

- **What.** Extend `foundations/language-support.md` (decision 0041) to the remaining types and declarations that core rules name: `std::type_info` for `typeid`, the exception types that the language itself throws (failed allocation, failed `dynamic_cast` of a reference, `typeid` of a null pointer, a bad array length), `std::align_val_t`, `std::nothrow_t`, and `std::destroying_delete_t`.
- **Evidence.** Five *(library referenced)* markers remain (glossary, `virtual.md:320`, `new-delete.md:58`, `copy-move.md:473`, `language-support.md:15`). `grep` finds 0 tests for `align_val_t`, `nothrow_t`, `bad_alloc`, and `type_info`, and 2 tests for `typeid`. Host probes gave these results:
  - A user-declared `std::type_info` makes `typeid(b) == typeid(B)` compile on Clang 22 and GCC 16.
  - Aligned `new` works with only a user-declared `std::align_val_t`.
  - `new (std::nothrow) int` fails on both compilers until the program declares the nothrow `operator new`.

  So the shapes method works. The page must separate types the implementation provides from declarations the program supplies.
- **Unblocks.** T leaves the Deferred list. VIRT-10 and NEW-5 lose their markers. W8 (ownership) and W10 (errors) can catch the language's own exceptions.
- **Size.** 1 to 1.5 sessions, plus a header-policy amendment. A run test that catches an exception the runtime throws needs the real header, because the runtime's type cannot be redeclared. Such tests run on the library compilers only, and EDG checks them.
- **Risk.** Medium. A list of standard type names is a fixed-order list, the pattern that flagged PRIM-1 and CLS-19. A table keyed by Tidal rule, in the existing page's form, avoids that pattern.

### 6. V. Thicken thin pages

- **What.** Add tests and split fused rules on the pages with the fewest tests per rule or the most findings per rule.
- **Evidence.**

  | Page | Rules | Tests | Single-test rules | Report mentions |
  |---|---|---|---|---|
  | logical | 14 | 21 | 8 | 11 |
  | static initialization | 6 | 12 | 2 | 16 |
  | comma | 8 | 17 | 1 | 7 |
  | blocks | 8 | 15 | 3 | 3 |
  | evaluation order | 9 | 25 | 2 | 68 |
  | lambdas | 28 | 60 | 15 | 27 |
  | deleted and defaulted | 5 | 17 | 2 | 8 |

  Evaluation order has the highest ratio of report mentions to rules in the specification, about 7.6. Lambdas is added to the round-1 list because 15 of its 28 rules have one test.
- **Unblocks.** W2 and W14, and follow-up to Propose exercise X1 (left-to-right evaluation).
- **Size.** 1.5 to 2 sessions.
- **Risk.** Low, except on evaluation order. "Sequenced before" and its relatives are terms of art. A rewrite there should define Tidal terms and keep one condition per rule.

### 7. X. Digest overrides

- **What.** For each per-compiler override, confirm that it names a question or an implementation choice. Group the overrides by cause. Close compiler-defect questions where every current version now agrees.
- **Evidence.** Alignment has 13 overrides on 31 tests, with 5 questions. Attributes has 13 on 72. Exception specifications has 6 on 40. Lane X should take in conversion sequences (22), overload resolution (25), references (14), and templates/constraints (21, of which 16 are EDG). Q-0270 is typical: every current compiler agrees and Clang 18 differs.
- **Unblocks.** Fewer `depends` answers in Predict batches. A cleaner baseline if MSVC is ever added.
- **Size.** 1 to 2 sessions.
- **Risk.** None for similarity. The work touches questions and test notes only.

### 8. W15. Template exercises

- **What.** Add a Write suite for generic code (for example, a fixed-capacity vector, a type-erased callback, a constrained algorithm, a tuple of packs) and Predict batches for deduction, ordering, and constraints. The writer's cost is the repairs that follow.
- **Evidence.** No suite from W0 to W14 lists a template page. The only Predict batches done are the seven Foundations batches, scored 231/231. A perfect score suggests those batches were easy, so a harder chapter is the better test of the method.
- **Unblocks.** The first reader evidence on the chapter that costs the most per rule.
- **Size.** 1 to 2 writer sessions, after T1.
- **Risk.** Low. A reader must write its own containers, because tests use no library headers. That is a fair test of the specification.

### 9. S. Class property lists

- **What.** Complete the standard-layout and trivially-copyable conditions (CLS-19, and the list at `copy-move.md:431`).
- **Evidence.** Two *(class properties referenced)* markers remain. `specification-format.md` forbids compiler built-ins in tests, so the direct trait channel is closed. The core-language channels are the union common-initial-sequence read (standard-layout), constant evaluation of a copy, and the first-member address. These channels are indirect, and each one observes only part of a property.
- **Unblocks.** CLS-19 loses its partial-list marker. Some later work on `offsetof` and `bit_cast`.
- **Size.** 1 session, after an observation-channel decision.
- **Risk.** High. CLS-19 was flagged once already, and condition lists are the riskiest form. Write each condition as its own rule, with an indirect test per condition.

### 10. Scope: coroutines

- **What.** Decide whether coroutines enter scope.
- **Case for.**
  - One marker (`operators.md:85`) points at them.
  - A host probe shows that a user-declared `std::coroutine_traits` and `std::coroutine_handle` let a `co_return` coroutine compile on Clang 22 and GCC 16 with no header. So the shapes method of decision 0041 reaches the check side.
  - The topic is closed. It adds no dependency on threads.
- **Case against.**
  - From general knowledge, the real `coroutine_handle` resumes through compiler built-ins. A run test therefore needs the `<coroutine>` header. That breaks the no-header rule for every run test, and EDG can only check such tests.
  - The coroutine transformation is a fixed-order procedure, like L and M. It would need all five method controls, a source reader inventory included.
  - Frame allocation, elision, and destruction order are full of `unspecified` choices. Expect 6 to 10 questions.
  - The interpreter would need a suspension model.
  - Both round-1 reports that ranked topics (the programmer's and the implementer's) put coroutines out of scope.
- **Recommendation.** Keep coroutines out. Record the decision and revisit after K2 and the header policy. Estimated cost if adopted: 4 to 6 sessions and 100 to 150 probes.

### 11. Scope: extended floating-point types

- **What.** Decide whether the C++23 extended floating-point types enter scope.
- **Case for.** Three markers point at them (`conversions.md:108`, `conversion-sequences.md:645` and `:745`). The topic is small: a rank rule, a conversion table, and literal suffixes. It would cost about 1 session.
- **Case against.** A host probe shows that GCC 16 accepts the `f16`, `bf16`, `f32`, `f64`, and `f128` suffixes. Clang 22 rejects all five and rejects the conversion and overload probes too. Every rule would be `conditionally-supported` with evidence from one family. The type names come from a library header. Both round-1 reports that mention the topic leave it out.
- **Recommendation.** Keep the types out. The markers already state the boundary. Revisit when a second baseline family accepts the suffixes.

## 2. The lanes and the Deferred list

| Entry | Round-1 status | Assessment now |
|---|---|---|
| U | parallel lane | rank 4. The questions are ripe. G57 and G58 show that the interpreter now catches these classes. |
| V | parallel lane | rank 6, with lambdas and deleted functions added |
| X | parallel lane | rank 7, extended to the L, M, and templates overrides |
| G beyond G1 (lexical structure) | deferred | keep deferred. The keywords page (4 rules, 96 tests) covers what programs need. Literal grammar and character sets carry the highest similarity risk for the least value. |
| S beyond S1 | deferred | rank 9, after an observation-channel decision |
| T (`typeid`) | deferred | fold into K2. The host probe shows that the shape works without a header. |
| MSVC | deferred | keep deferred until lane X is done |
| C++26 features | Q-0007: C++23 | keep out. Newer compilers accept C++26 features, and Q-0007 already says how tests record that. |

## 3. Method controls: cost per item

These are the controls from the roadmap page and decision 0044.

| Control | Writer cost per use | Who else pays |
|---|---|---|
| Source reader inventory before drafting | ~0.25 session: similarity check, then one grid row per inventory item | one source-reader session |
| Probe grid | 0.1-0.25 session per grid file, plus matrix time | none |
| Decisive test per step or tie-breaker | one test per rule, often already there | none |
| Evidence names each rule's source | about 5% more text per page | none |
| Gap review after drafting | 0.5-1 session of repairs (N2, R, and P each needed a repair pass) | one source-reader session |

| Item | Inventory | Grids | Decisive tests | Gap review | Added sessions |
|---|---|---|---|---|---|
| T0 | no | no | yes | already done | 0 |
| T1 | no | yes, 6 to 10 grids | yes | no | 0.5-1 |
| FE8 | no | as needed | no | no | 0 |
| U | no | 1 to 2 grids | no | no | 0.25 |
| K2 | no | no | no | yes, light | 0.5 |
| V | no | evaluation order only | no | yes, evaluation order only | 0.5 |
| X | no | no | no | no | 0 |
| S | no | yes | yes | yes | 0.75 |
| Coroutines, if adopted | yes | yes | yes | yes | 1.5-2 |
| Extended floating point, if adopted | no | yes | no | no | 0.25 |

The full set of controls is only needed for procedures, which in round 2 means coroutines alone. That is one more reason to keep coroutines out for now.

## 4. Recommended order

1. T0, with FE8 running as findings arrive.
2. T1, then W15 (the reader's turn), then the repairs it causes.
3. U and K2. K2 needs the header-policy amendment first.
4. V and X, as parallel lanes.
5. The observation-channel decision, then S.
6. Record the scope decisions on coroutines and extended floating-point types now, as one decision. Neither needs any writing.

## 5. Tooling and process, ranked

1. `tools/status_table.py` includes `templates` and reads the Similarity column from the latest report.
2. The list-order check from round 1 (tooling item 5) still does not exist. K2 and S need it.
3. The questions index writes out each "option 1" answer (74 rows).
4. A header-policy amendment for run tests that catch exceptions the runtime throws.
5. The front end refreshes `conformance.tsv` over all template tests. It lists 379 of about 960.
6. The status page shows template counts. Its Next steps, size estimate, and counts still describe the phase-3 state.

## 6. Risks

1. **Filling template gaps from memory.** Front-end findings marked "from outside knowledge" show where readers have done this. A writer can do the same during T1. Each new test should come from a probe, not from recall.
2. **Fixed-order lists in K2 and S.** These are the forms most likely to be flagged. Use one condition per rule and tables keyed by Tidal rule.
3. **Header creep.** K2 needs real headers for a few run tests. The amendment should name the allowed headers and the reason for each one, so the exception stays small.
4. **Question load.** Templates raise 0.24 questions per rule. Lane X and U should retire questions faster than new work adds them.

General knowledge used in this report: the real `coroutine_handle` resumes through compiler built-ins, and the runtime's exception types cannot be redeclared by a program. Everything else comes from Tidal's files or from the host probes, whose inputs are in /tmp/claude-1000/-home-ryanm-Code-tidal/f142cdb8-5f85-4266-b9df-c7416ea6d3a0/scratchpad/co.cpp, /tmp/claude-1000/-home-ryanm-Code-tidal/f142cdb8-5f85-4266-b9df-c7416ea6d3a0/scratchpad/t.cpp, and /tmp/claude-1000/-home-ryanm-Code-tidal/f142cdb8-5f85-4266-b9df-c7416ea6d3a0/scratchpad/n.cpp.
