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
clean-room roadmaps (`reports/clean-room/roadmap-4/`).

| Item | Work | Prerequisites |
|---|---|---|
| R1 | Close the phase 6 record: full matrix results, questions in similarity runs, roadmap and exercise rows, cluster 2 readings, Q-0312 | none |
| WP | Done: the 67 Write programs are run tests under `tests/programs/` | none |
| SR | A second settling rule, applied to reviewed questions, and one decision packet for the maintainer | R1 |
| TP | Done: a matrix choice for every `impl-defined` rule, and the matrix layout as a table (CLS-35) | none |
| UW | A run witness for each `undefined` rule | none |
| GR | Gap reviews of the eleven pages that never had one, then of open questions with no review | SR |
| PR | Predict batches for the pages with no batch, first overloading and storage | none |
| FE | Front-end rounds from 15: class layout, allocation by name, virtual bases, false rejects, Write programs | TP, WP |
| X | A refresh check for Propose reports, then Propose X13 to X16 | none |

## Parallel lanes

These lanes run beside the phases when a phase waits for review.

- U: deepen multiplicative, additive, and comparison, and settle their
  questions.
- V: thicken thin pages (logical, static initialization, comma, blocks,
  evaluation order).
- X: digest the overrides on alignment, attributes, and exception
  specifications.
- Clean-room exercises (`documentation/exercises.md`): W8 and W1 next; Predict
  batches for the expressions pages once the format can catch errors;
  Diagnose batches after item C; Propose exercises X1 to X4 after item
  Y.
- A change notation for Propose reports: a table of rule ID, kind of
  change, old class, new class, and the tests affected.

## Deferred

- G beyond G1, lexical structure: high similarity risk for little
  value; an alternative-token and digraph note goes on the operator
  pages.
- S beyond S1, and T, the full trivially-copyable and standard-layout
  lists and `typeid`: they wait for a decision on how to observe
  library-dependent properties.
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
