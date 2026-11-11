# Roadmap

This page orders the work ahead. Five clean-room roadmaps are the
page's evidence, each from one view: a programmer, an implementer, an
editor, a clean-room writer, and a language designer
(`reports/clean-room/roadmap/`). Decisions 0033 and 0039 adopt the
roadmap.
Item letters refer to the writer's table, which gives evidence, probe counts, similarity risk, and size for each item.

The order follows dependencies that Tidal's own pages show and the
findings of the clean-room exercises. The order does not follow any other
specification's organization.

## Where the views agree

- Fix what exists before adding to it: contradictions between rules,
  rule classes that do not say what to diagnose, stale *(referenced)*
  markers, and the grammar.
- A page of fundamental types and target values is cheap and needed
  everywhere.
- Name lookup comes before overload resolution, and overload resolution
  has the highest value of the new topics.
- A templates core is needed for generic code and for the type-checking
  idiom 184 tests use.
- The language relies on a few library types (`std::initializer_list`,
  the comparison categories, the tuple protocol, the size types). Tidal
  states only what the language requires of them.
- Coroutines, modules, threads and the memory model, the general
  library, and the preprocessor beyond `#include` stay out of scope.

## Phase 1: repair what exists

| Item | Work | Evidence |
|---|---|---|
| A | Fix the contradictions: TR-3 against FN-6, CX-1 and default initialization, ENUM-10 inside the enumerator list, RET-1 and `void` | editor 1, front end F16 to F18, F23 |
| C | Decide and check that a reject test cites a `reject` rule; split prohibitions out of `required` rules, page by page | editor 2, front end F15 (420 tests, 268 rules) |
| B | Replace about 40 stale *(referenced)* markers with rule links | editor 4, implementer 3 |
| D | Constant evaluation: every subobject of a constant initialized, "literal type" and "constant initializer" defined, a generated index of `undefined` rules | editor 3, W0 F2, front end F14, F19 |
| Process | check_specification checks (class of cited rules, nonterminals, stale markers); status columns for single-test rules, overrides, and findings per page; a "compiler defect" question status; Predict outcomes for undefined behavior and compiler-dependent results; front-end conformance as a regression signal | editor 3, writer 3 |

## Phase 2: foundations that later pages need

| Item | Work | Evidence |
|---|---|---|
| E | A page of fundamental types, ranks, and per-target values (LP64 and `-m32`), written as tables; the EDG 32-bit matrix entry | all four views |
| F | A grammar notation decision, then one consolidated grammar with every nonterminal defined | front end F1 to F8, writer 4 |
| Y | Cross-page inventories on the Foundations pages: rule IDs for the usual arithmetic conversions and integer promotion, the contexts of implicit conversion, and the full expression | designer topic 1, D4, D17, D31 |
| S1 | Definitions of trivial special member functions, which JMP-6, SW-5, and UNI-5 use | designer topic 7 |
| G1 | A narrow lexical slice, written as tables: keywords, reserved names, and the literal suffix grammar | designer topic 5, D2, D3, D20 |
| K | Language-support types, as the shapes the language requires, with a header policy for clean-room exercises | programmer 3, implementer 6 |
| Guides | Task-level guides: a class that owns a resource, error handling, integer arithmetic without undefined behavior, enumerations in switches, reading the specification | programmer section 2, W0 |

## Phase 3: lookup and overload resolution

The phase 3 items move topics the charter lists as referenced into
scope. Decision
0033 adopts the move provisionally (Q-0113); the charter changes when
each item starts.

| Item | Work | Prerequisites |
|---|---|---|
| H | Name lookup completed: qualified lookup, lookup in bases, using-directives, argument-dependent lookup, the injected class name | inputs fetched |
| I, J | Use of an entity and the one-definition rule, with the minimal `#include` slice | an optimization-level variant in the runner |
| L | Implicit conversion sequences | a probe grid tool, a procedure-presentation decision, a source reader's behavior inventory |
| M | Overload resolution: candidate sets per context, viability, choosing the best, ambiguity | L, H |

## Phase 4: templates

| Item | Work | Prerequisites |
|---|---|---|
| N1 | Done: template declarations, parameters, template-ids, explicit and partial specialization, variable and alias templates (`specification/templates/`) | F notation |
| O | Done: deduction, function templates, partial ordering (`specification/templates/deduction.md`, `ordering.md`) | M, N1 |
| N2 | Done: dependent names, two-phase checking, instantiation (`specification/templates/dependent-names.md`, `instantiation.md`) | runner outcome sets |
| P | Done: concepts and constraints (`specification/templates/concepts.md`, `constraints.md`) | M, N1, N2 |
| R | Done: packs, forwarding references, class template argument deduction (`specification/templates/packs.md`, `class-template-deduction.md`) | O |

## Front end

| Item | Work | Prerequisites |
|---|---|---|
| FI | The interpreter of decision 0048: run tests on the abstract machine, stop at undefined behavior with the rule, and serve constant evaluation as a client | front-end round 6 |
| FR | Rewrite the front end as software that a person can read and extend: a design document, modules by chapter and page, a parser from the Syntax segments, unit tests per stage, landed in stages with 0 worse (decision 0060) | the completion test of decision 0060: every test passes its check, except ledger rows marked `out of scope`, and every program test runs |

## Phase 5: consolidate, fill the object model, exercise

Decision 0050 (provisional) adopts this phase from the second round of
clean-room roadmaps (`reports/clean-room/roadmap-2/`). All items are done.

| Item | Work | Prerequisites |
|---|---|---|
| T0 | Close phase 4: gap-review repairs of R and P, front-end round 8 | phase 4 |
| C1 | Consolidate question clusters; merge duplicates; close moot questions | none |
| C2 | Thicken the templates chapter: second tests, every template test cited | none |
| F1 | Objects in raw storage (new Foundations page) | an inventory |
| F2 | Language-support types, round 2: placement and aligned allocation, the exceptions the language throws | an inventory |
| F3 | Templates across translation units on `odr.md` | none |
| L | Lanes U, V, and X | none |
| E | Write suites for generic code, Predict batches for templates, a Diagnose batch, Propose round 2 | C2 |
| FE | Templates in the interpreter, run outcomes in the baseline, `run-fail` tests for `undefined` rules | T0 |

## Phase 6: settle, cover by use, and check the object model

Decision 0051 (provisional) adopts this phase from the third round of
clean-room roadmaps (`reports/clean-room/roadmap-3/`).

| Item | Work | Prerequisites |
|---|---|---|
| R0 | Done: records from tools: status counts, question numbering and index, similarity column, Predict format and scoring | none |
| Q | Done: question clusters 1 to 6, each with a gap review whose reading its questions record and weigh | R0 (index) |
| G | Done: grammar-validity rule (TR-12); `undefined` overlap convention and an interpreter-only expectation | none |
| W | Write suites W1 to W14 done. The programs did not become run tests; phase 7 item WP takes that part | none |
| P | Done: Predict batches for expressions, statements, declarations, classes | R0 (format) |
| D | Done: designer repair batch, then Propose X9 to X12 and their repairs | none |
| C | Done: Overloading chapter, hotspot pages, consistency pass, second tests, character values, implementation limits | Q for the overload pages |
| O | Done: class properties and per-target class layout (CPY-19, CPY-28, CLS-19, CLS-29 to CLS-34, UNI-15, UNI-16), with grids under `tests/classes/copy-move/grids/` and `tests/classes/members/grids/`, its gap review, and the repairs | an inventory |
| FE | Done: front-end rounds 10 to 14: byte memory, parse failures as rejects, interpreter coverage, choice exploration | G |

## Phase 7: close the record, test by whole programs, and review every page

Decision 0054 (provisional) adopts this phase from the fourth round of
clean-room roadmaps (`reports/clean-room/roadmap-4/`). All items are
done. Open questions went from 310 to 209.

| Item | Work | Prerequisites |
|---|---|---|
| R1 | Done: close the phase 6 record: full matrix results, questions in similarity runs, roadmap and exercise rows, cluster 2 readings, Q-0312 | none |
| WP | Done: the 67 Write programs are run tests under `tests/programs/` | none |
| SR | Done: a second settling rule, applied to reviewed questions, and one decision packet for the maintainer | R1 |
| TP | Done: a matrix choice for every `impl-defined` rule, and the matrix layout as a table (CLS-35) | none |
| UW | Done: a run witness for each `undefined` rule, or an Evidence note that no tool stops it (ADD-18, OBJ-12, VOL-6) | none |
| GR | Done: gap reviews of the eleven pages that never had one, then of open questions with no review | SR |
| PR | Done: Predict batches for the pages with no batch, first overloading and storage | none |
| FE | Done: front-end rounds 15 to 17: class layout, allocation by name, virtual bases, false rejects, Write programs | TP, WP |
| X | Done: a refresh check for Propose reports, then Propose X13 to X16 | none |

## Phase 8: honest counts, principles that answer, and exercises by use

Decision 0055 (provisional) adopts this phase from the fifth round of
clean-room roadmaps (`reports/clean-room/roadmap-5/`). All items are
done. Open questions went from 209 to 221: item PQ reopened 8 settled
questions, the phase opened 9 (Q-0377 to Q-0385), and 5 settled.

| Item | Work | Prerequisites |
|---|---|---|
| R2 | Done: close the phase 7 record: counts in `pending.md` from a tool, review readings, `propose_check.py`, stale catalog and roadmap text, size estimate, Q-0322, single-test rules, the later-version plan | none |
| HN | Done: the harness judges by the base expectation and every explored outcome | none |
| RT | Done: tests from the programmer's reductions, and a tool that compiles the page examples | none |
| RV | Done: A gap review of the text phase 7 wrote, then repairs | R2 |
| PQ | Done: one test for "a principle gives the answer", then principles for the unanswered questions | R2 |
| TE | Done: thin evidence, check warnings for it, and a class for link-time diagnostics | R2 |
| FE | Done: front-end rounds 18 (the harness) and 19 | HN, RT |
| E | Done: Write wave 2, an Implement exercise on class layout, Predict on repaired pages, Propose X17 to X20 | R2, PQ for Predict |
| V | Done: the Evidence format, and Evidence that does not repeat the questions | RV |
| L | Done: test directories that match the pages | before the closing matrix run |

## Phase 9: use the widened principles, review phase 8, and catch the front end up

Decision 0056 (provisional) adopts this phase from the sixth round of
clean-room roadmaps (`reports/clean-room/roadmap-6/`). All items are
done. Open questions went from 221 to 172: the phase opened 15
(Q-0386 to Q-0400) and settled 64, of which 63 settled through the
review RV2 and the principle-use batches PU1 and PU2.

| Item | Work | Prerequisites |
|---|---|---|
| R3 | Done: close the phase 8 record, with a definition of a use of a principle | none |
| RS | Done: tests from the round 6 reductions and sketches, and LSUP-2 tests | none |
| FH | Done: answers for G134 and G135, the front end on the page examples, per-page rule-ID scoring, the interpreter in the exercise prompts | none |
| RV2 | Done: a gap review of the text phase 8 wrote, then repairs | R3 |
| PU | Done: a principle-use batch for the questions that wait on a widened principle, then settling | R3 |
| FE | Done: front-end rounds 20 and 21 | RS, FH |
| V3 | Done: the Evidence format on the other 68 pages | RV2, PU |
| E | Done: Implement I2 on partial ordering, Predict on 11 of the 12 pages with only named batches (the objects page waits for a tool fix that came later), Propose X21 on P15 | FH (interpreter step) |

## Phase 10: review phase 9, hold questions by a field, and test the C++23 call forms

Decision 0057 (provisional) adopts this phase from the seventh round of
clean-room roadmaps (`reports/clean-room/roadmap-7/`). All items are
done. Open questions went from 172 to 171: the phase opened 11
(Q-0401 to Q-0411) and settled 12, 10 through the review RV3 and 2
through the principle-use batch PU3.

| Item | Work | Prerequisites |
|---|---|---|
| R4 | Done: close the phase 9 record, with a `**Waits for:**` field on each open question and generated Split tests tables | none |
| G | Done: a ledger of front-end findings, and answers for G147 to G151 | none |
| T | Done: tests and text for the C++23 call forms (CLS-13, LAM-15, LSUP-2), and the round 7 programs as program tests | none |
| RV3 | Done: a gap review of the text phase 9 wrote and of the held questions, then repairs | G, T |
| PU3 | Done: a principle-use batch on the questions whose field names a use, then settling | R4, RV3 |
| PK | Done: a decision packet for `pending.md`, and the review-held questions sorted by kind | R4 |
| E | Done: Implement I3 on overload ranking; Write W18 and W19; a refreshed Propose portfolio, a refresh note for X19, and Propose X22 | none (W18 and W19 after T) |
| FE | Done: front-end round 22 | T, G, and the I3 and Write reports |

## Phase 11: answer every finding, review phase 10, and reach library-style code

Decision 0058 (provisional) adopts this phase from the eighth round of
clean-room roadmaps (`reports/clean-room/roadmap-8/`). Every item is
done except FE, which moved to the next phase. Open questions went from
171 to 174: the phase opened 7 (Q-0412 to Q-0418) and settled 4
through the review RV4 (Q-0263, Q-0386, Q-0395, Q-0413), two of them
by its principle uses.

| Item | Work | Prerequisites |
|---|---|---|
| F | Done: answer the phase 10 findings that no commit acted on, and make TR-P1 and TR-4 agree | none |
| L | Done: a ledger row for every finding, with a check | none |
| R5 | Done: repair and generate the records that drift; a script for the Checks paragraph | none |
| P | Done: tests and text for library-style initialization, the two EDG splits, and the round 8 programs | none |
| RV4 | Done: a gap review of the text phase 10, F, and P wrote, then repairs | F, P |
| PK2 | Done: a decision packet by option, with row 4 split | R5 |
| E | Done: Implement I4 on COND-16; Port suite P1; Propose X23 on P25 | none (P1 after R5's prompt changes) |
| FE | Moved to the next phase: front-end round 23 | P, F (G152) |

## Phase 12: run the front end again, answer the ended rows, and review phase 11

Decision 0059 (provisional) adopts this phase from the ninth round of
clean-room roadmaps (`reports/clean-room/roadmap-9/`). Every item is
done. Open questions went from 174 to 176: the phase opened 4 (Q-0419
to Q-0422) and settled 2 through item QR (Q-0323, Q-0383).

| Item | Work | Prerequisites |
|---|---|---|
| T | Done: tests for every front-end target that has none, and the round 9 and P1 programs as program tests | none |
| FE | Done: front-end round 23: the two wrong verdicts, the phase 11 and T tests, the conditional operator, the template missed rejects, then the backlog groups | T; F2's COND rows before the conditional operator |
| F2 | Done: answer the 20 `planned` rows of ended items, COND rows first | none |
| RD | Done: the RV4 points that a probe decides, and a class for the program's duty in ALN-4 and TR-4 | none |
| QR | Done: question records, the family check, and a reason for each rule that rests only on overridden tests | none |
| L2 | Done: read the `open` ledger rows, fill the `commit` rows with no commit, and check `planned` rows against their items | none |
| R6 | Done: repair the records that drift; a Propose prompt; Port programs as program tests | none (Q-0415's line after RV5) |
| PK3 | Done: the packet with each option's changed tests and the share line, then the request to the maintainer | none |
| RV5 | Done: a gap review of the text phase 11 wrote after RV4 and the text F2 and RD write, then repairs | F2, RD |
| E | Done: write suite W20 on library-shaped code; Implement I5 on deduction from a call | W20 after FE; I5 after RD |

## Phase 13: test every front-end finding, answer the open rows, and count the distance to the rewrite

Decision 0061 (provisional) adopts this phase from the tenth round of
clean-room roadmaps (`reports/clean-room/roadmap-10/`). Every item is
done. Open questions went from 176 to 180: the phase opened 6 (Q-0423
to Q-0428) and settled 2 (Q-0419 through item QR2, Q-0428 through item
RV6).

| Item | Work | Prerequisites |
|---|---|---|
| T2 | Done: tests for every front-end finding that has none, and the round 10 programs and clean first tries as program tests | none (F3's DEP-24 and UNI-9 answers for those two tests) |
| F3 | Done: answer the 17 `open` rows and the text parts of G153 to G157, base relation and storage rows first; then the text points of the round 10 reductions | none |
| CT | Done: the completion count: an `out of scope` file, the `no-diagnostic` pass, one line from `frontend_check.py` | none |
| FE | Done: front-end round 24: wrong verdicts and T2 tests, root causes, run stops, missed rejects outside templates, then template missed rejects | T2; CT for the count; F3's G153 sentence before the base-relation fix |
| RV6 | Done: a gap review of the 20 rules and principles changed after RV5, F3's text, and Q-0422, then repairs and a read of the repair diff | F3 |
| QR2 | Done: question records: Q-0419 split, Q-0421's note, Q-0422's Context | none |
| PK4 | Done: the packet counts row 4 rule items; the request to the maintainer goes on the record | none |
| R7 | Done: repair the records that drift and extend the ledger check; the Propose prompt and P26's entry | none |
| E | Done: port suite P2 on generic collections; Implement I6 on dependent names | P2 after FE; I6 none |

## Phase 14: answer the open rows, review phase 13, and set the gate for the rewrite

Decision 0062 (provisional) adopts this phase from the eleventh round of
clean-room roadmaps (`reports/clean-room/roadmap-11/`). Every item is
done. Open questions went from 180 to 185: the phase opened 6 (Q-0429
to Q-0434) and settled 1 (Q-0422, through item RV7). Front-end round 25
ended with 104 tests that fail their check, so the gate of decision 0062
does not hold.

| Item | Work | Prerequisites |
|---|---|---|
| T3 | Done: tests for the round 11 reductions and probes, and the round 11 programs and clean first tries as program tests | none |
| F4 | Done: answer the 24 `open` rows and G161 and G162, INST-33, DEP-4, and INST-5 first; text for a typed pointer over bytes | none |
| FE | Done: front-end round 25: the T3 and F4 tests, the TR-13 stop, missed rejects outside templates, and a cause for each failure that needs the rewrite | T3; F4 for its tests |
| RV7 | Done, except the recorded read of the repair diff b5d925e0, which the phase 15 decision takes: a gap review of the 22 rules and principles changed after RV6, the rest of 9c009448, F4's text, and a read of the RV6 repair diff; then repairs and a read of their diff | F4 |
| RP | Done: rewrite preparation with no code change: a design document, a grammar start symbol, and the template model | none |
| R8 | Done: records: front-end ledger hashes and a check; host-only claims in the prompts; the Propose portfolio refreshed | none |
| E | Done: port suite P3 on pipelines and events; Implement I7 on class template argument deduction | P3 after FE; I7 none |

## Phase 15: answer the open rows, read the RV7 repairs, and count the rewrite's share of the gate

Decision 0063 (provisional) adopts this phase from the twelfth round of
clean-room roadmaps (`reports/clean-room/roadmap-12/`).

| Item | Work | Prerequisites |
|---|---|---|
| T4 | Tests for the round 12 reductions and probes, and the round 12 programs and clean tries as program tests | none |
| F5 | Answer the 26 `open` rows and G164 and G165, with a question for each new split; literal operators and STG-28 | none |
| FE | Front-end round 26: the T4 and F5 tests, the STG-23 wrong verdict, and the failures the current design can take | T4; GC for the count |
| RV8 | A recorded read of the RV7 repair diff; a gap review of the 20 changed rules, the five blocks repaired twice, and F5's text; repairs and a read of their diff | F5 for the review; none for the read of b5d925e0 |
| GC | A marks file for the tests that need the template model, the completion line with the marked tests apart, and TEMPLATE-MODEL.md up to date | none |
| RD | The design document of the rewrite's code, and no code | GC |
| R9 | Records: checks for repair-diff reads, settlement commits, and the request sentence; ledger rows for three findings; the portfolio step | none |
| E | Port suite P4 on iterators and owned resources; Propose X25 on P27 | P4 after FE; X25 after R9's portfolio step |

## Parallel lanes

Phase 5 ran three lanes beside its items, and all three are done: U
(multiplicative, additive, and comparison), V (thin pages), and X (the
overrides on alignment, attributes, and exception specifications). The
exercise lanes and the change notation for Propose reports are done too
(`documentation/exercises.md`, sections 4 and 8). From phase 6 on, work
that runs in parallel is an item in the phase's table, and the
Prerequisites column says which items can run together.

## Deferred

- G beyond G1, lexical structure: high similarity risk for little
  value; an alternative-token and digraph note goes on the operator
  pages.
- `typeid`: the type it gives and the exception that a failed `typeid`
  throws. Both come from the library, and they wait for a decision on
  how to observe library-dependent properties. Phase 6 item O wrote
  the trivially-copyable and standard-layout lists (S and T) without
  them.
- MSVC: the existing overrides come first.

## Method controls for phases 3 and 4

Writing an algorithm from compiler outcomes risks filling gaps from
memory, and memory follows the reference's organization. For items L,
M, O, and N2:

- a source reader gives an inventory of behaviors before drafting (no
  wording, no order, no examples), checked for similarity;
- a probe grid covers every pair of categories;
- each tie-breaker rule has a test that it alone decides;
- rules follow Tidal's principles, and Evidence names each rule's source;
- a gap review follows the draft.
