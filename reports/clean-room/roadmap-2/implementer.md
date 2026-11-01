# Roadmap, round 2: what to do after phase 4, from an implementer's view

Date: 2026-10-31. Decision 0029. Role: clean-room reader, acting as an implementer who builds a front end from Tidal.

Inputs:
- AGENTS.md, CLAUDE.md, `documentation/` (charter, roadmap, status, decisions 0040, 0041, 0047, 0048)
- `specification/`, `tests/`, `questions/`
- `frontend/` and its README
- `reports/clean-room/rust-frontend/report.md` and `report-2.md` to `report-7.md`. There is no `report-8.md` yet, because round 8 is in progress.
- `conformance.tsv`
- the round-1 report in `reports/clean-room/roadmap/implementer.md`

**Measurements.** I ran `tidal-fe conform ../tests -v` read-only and wrote the output to the scratchpad. The binary was built at 21:01 from the working tree, which holds uncommitted round-8 code (`frontend/src/templ.rs`) and uncommitted gap-review tests. The committed baseline `conformance.tsv` covers 4348 tests, with 3444 passing. Its 515 template tests in concepts, constraints, packs, class template argument deduction, dependent names, and instantiation do not appear in it yet. I counted with grep and short Python scripts, so the counts are approximate. Where I rely on general knowledge, the text says so.

| Measure | Value |
|---|---|
| Check, whole suite | 4913 tests: 3961 pass, 61 false rejects, 424 missed rejects, 385 unsupported |
| Check, outside `templates/` | 3424 of 3992 pass (86%) |
| Check, `templates/` | 537 of 921 pass (58%) |
| Run tests (`run`, `run-fail`) | 1047 tests: 829 pass, 2 wrong results, 199 unsupported, 17 not run because the check did not accept them |
| Rules by class | 825 required, 530 reject, 46 undefined, 43 recommended, 39 impl-defined, 34 no-diagnostic, 27 unspecified, 4 conditionally-supported |

## 1. Ranked items

| Rank | Item | Kind | Size | Risk |
|---|---|---|---|---|
| 1 | Templates in the front end and interpreter (finish round 8) | front end | large | medium |
| 2 | Run outcomes in the regression baseline | tooling | small | low |
| 3 | A rule that the grammar decides validity, and a parser that follows the grammar | specification + front end | small text, medium code | medium |
| 4 | A sequencing model the interpreter can run | specification | 3 to 5 rules | medium |
| 5 | A `run-fail` test for every `undefined` rule, with a decision on interpreter evidence | tests + decision | 24 to 40 tests | medium |
| 6 | Object representation and access through another type | specification (new Foundations page) | 8 to 12 rules | high |
| 7 | Interpreter coverage of constructs that already have pages | front end | medium | low |
| 8 | A Diagnose batch on the missed rejects of finished pages | exercise | medium | low |
| 9 | Scope: name the exceptions that the language throws; keep `typeid` out | scope change (narrow add) | small | low |
| 10 | Scope: keep lexical structure and the preprocessor beyond the slice out | scope decision (keep out) | none | low |

### 1. Templates in the front end and interpreter (finish round 8)

**What it is.** The front end checks and runs template code against the phase 4 pages. Round 8 is in progress (`templ.rs`, 3600 lines). Its report should sort each failure into a front-end gap or a text gap, as earlier rounds did.

**Evidence.**

| Subdirectory | Tests | Pass | False reject | Missed reject | Unsupported |
|---|---|---|---|---|---|
| class-template-deduction | 98 | 10 | 4 | 30 | 54 |
| concepts | 105 | 13 | 3 | 36 | 50 |
| constraints | 73 | 18 | 13 | 27 | 11 |
| packs | 54 | 14 | 11 | 19 | 10 |
| dependent-names | 95 | 64 | 4 | 16 | 9 |
| specialization | 81 | 62 | 4 | 10 | 3 |
| other five | 349 | 356 (includes EDG-only) | 9 | 12 | 23 |

- 48 of the suite's 61 false rejects are in `templates/`.
- Ten constraints tests fail with "CLS-9 same member function declared static and non-static" (for example `unconstrained.cpp`, `subsumes-conjunction.cpp`). The front end does not yet treat two declarations with different constraints as different (CNS-23).
- 15 template tests stop at "SA-1 static assertion failed" with valid code. Examples are `packs/using-declaration-pack.cpp` and `ordering/constraints-partial.cpp`.
- The two largest unsupported causes in the suite are template causes: "variable template with unknown arguments" (64 tests, the `same` idiom) and "concept" (42).
- Open template questions run from Q-0195 to Q-0270. The uncommitted tests cite Q-0276 to Q-0299, and `check_specification.py` reports those questions as missing.

**What it unblocks.**
- An independent implementer's reading of phases 4's pages, in addition to the source reader's gap reviews.
- The `same` idiom across the suite.
- The interpreter on generic code (92 template run tests; 76 pass now).
- Items 7 and 8.

**Size.** Large. Concepts, satisfaction, and class template argument deduction are still mostly absent from the front end.

**Risk.** Medium. The false-reject clusters can hide text gaps, so the round-8 report needs a finding for each cluster whose cause is unclear. The working tree is mid-repair, so the round should rerun after the R and P repairs land.

### 2. Run outcomes in the regression baseline

**What it is.** `tools/frontend_check.py` compares only check lines. The harness prefixes run lines with `RUN`, and the tool skips them. The baseline should record run outcomes too, and the tool should fail when a run that passed stops passing.

**Evidence.**
- `frontend/README.md` says that `frontend_check.py` reads only the check lines.
- `expressions/multiplicative/floating-divide-by-zero-infinity.cpp` now gives `RUN-WRONG-RESULT` (result 4). MUL-11 changed from `undefined` to `impl-defined` with an IEEE table, and nothing flagged the effect on the interpreter.
- The second wrong result, `list-ctor-preferred-over-better-match.cpp` (ILC-3), has been known since round 7.
- Decision 0048 says that a defined result must match every compiler configuration and that a mismatch is a finding. Without a baseline, nobody sees the mismatch.

**What it unblocks.** Decision 0048's mismatch rule, and safe repairs of `undefined` and `unspecified` rules.

**Size.** Small. Add one column or one line kind to `conformance.tsv` and one comparison in the tool.

**Risk.** Low. Tests that pass a choice point can flip when the interpreter's fixed choice changes. The tool could treat `unspecified` results as passes when the status is in the allowed set.

### 3. A rule that the grammar decides validity, and a parser that follows the grammar

**What it is.** One rule on `foundations/translation.md` saying that a program the grammar does not derive is rejected. The disambiguation rules should be cited from that rule. Then the front end can report "no production matches" as a reject, and keep "unsupported" for constructs it does not model.

**Evidence.**
- No rule says that a program which fails the grammar is rejected. A search of `translation.md` for "grammar", "syntax", and "parse" finds nothing. TR-1 covers only breaches of `reject` rules.
- The grammar is complete under decision 0040. `check_specification.py` reports no nonterminal errors across 62 `productionlist` blocks.
- The front end stops with a parse message on 164 tests:

  | Group | Tests | Expect reject | Expect accept or run |
  |---|---|---|---|
  | foundations/keywords | 79 | 79 | 0 |
  | everything else | 85 | 53 | 32 |

- An example is `keyword-and.cpp` (`int and = 0;`, KW-1). The lexer makes `and` into `&&`, the parser fails, and the harness counts the test as unsupported.

**What it unblocks.**
- About 130 reject tests.
- A clear meaning for "unsupported".
- The 32 valid programs that do not parse become a list of front-end parser gaps against named productions.

**Size.** One or two rules. Medium front-end work, because the parser must be audited against the productions before parse failure can safely mean reject.

**Risk.** Medium. Where the grammar is more permissive or stricter than the compilers, the front end will produce false rejects. Each one is a grammar finding. The semantic `reject` rules remain the main source of diagnostics, so the new rule should state that a rule ID is optional for a grammar reject.

### 4. A sequencing model the interpreter can run

**What it is.** Deepen `evaluation-order.md` and the operator pages. The rules should separate an operand's value from its side effect. They should say, for assignment, the compound assignments, and prefix and postfix increment, when the side effect happens relative to the values.

**Evidence.**
- SEQ-4 (`undefined`) has 4 tests. None is `run-fail`. One reject test depends on the SEQ-7 warning.
- The glossary defines *unsequenced* as possible overlap. No page uses the terms "value computation", "sequenced before", or "sequenced after" (0 hits).
- Report-7 section 2 says the interpreter does not detect SEQ-4. It evaluates operands left to right and records SEQ-8 or SEQ-1.
- From general knowledge, `i = i++ + 1` and `i = ++i + 1` differ in this respect. The current text cannot decide either.
- `unsequenced-constant-evaluation.cpp` expects accept, so compilers do not use constant evaluation to catch SEQ-4. Evidence must come from warnings and from reasoning.

**What it unblocks.**
- SEQ-4 detection in the interpreter.
- `run-fail` tests for SEQ-4.
- A precise SEQ-7 warning.
- The "try each allowed choice" part of decision 0048 for SEQ-1 and SEQ-8 (37 passing runs passed a choice point in round 7).

**Size.** 3 to 5 rules, plus a table row per operator on the operator pages.

**Risk.** Medium. The similarity risk is real, because the model is widely known in fixed wording. Compiler evidence is thin, since sanitizers do not report SEQ-4 (general knowledge). Decision 0044's method controls should apply.

### 5. A `run-fail` test for every `undefined` rule, with a decision on interpreter evidence

**What it is.** Write `run-fail` tests for the `undefined` rules that have none. A decision should say what an interpreter stop counts as evidence of.

**Evidence.**
- 22 of 46 `undefined` rules have a `run-fail` test.
- The other 24 have none: ADD-11, ADD-14, ADD-17, CAST-5, CTOR-12, CV-5, DTOR-7, INIT-1, INST-15, MPTR-6, MUL-12, MUL-14, NEW-3, OBJ-10, PFX-8, PRIM-8, REF-5, RET-6, SEQ-4, TR-8, TRY-11, UNI-6, VIRT-12, VOL-6.
- `status.md` says that neither sanitizer catches a second destruction (DTOR-7) or a write to a `const` object (CV-5).
- The interpreter already stops at several of these at run time (DTOR-7, INIT-1, NEW-3, PFX-8, UNI-6, VIRT-12, per report-7).

**What it unblocks.** A run-time witness for each `undefined` rule, the undefined-behavior index (item D), and Predict exercises on undefined behavior.

**Size.** 24 to 40 tests and one decision.

**Risk.** Medium. The evidence is circular, because the interpreter was written from the rules it would confirm. The decision should say that an interpreter stop shows only that the test reaches the named operation. The rule's own evidence still comes from compilers (constant evaluation, warnings, sanitizers, `-O2` differences).

### 6. Object representation and access through another type

**What it is.** A new Foundations page covering:
- the bytes of an object,
- which types may read another object's bytes,
- what a read through a `reinterpret_cast` result does,
- storage that an array of bytes provides for other objects,
- the full trivially-copyable list.

**Evidence.**
- CAST-6 says that `reinterpret_cast<T&>` "refers to the operand as type T". No rule says what reading through it does.
- CPY-19 says that a program may copy the bytes of a trivially copyable object (`trivially-copyable-bytes.cpp`). No page defines those bytes. CPY-19 and CLS-19 still end with *(class properties referenced)* (`copy-move.md:431`, `members.md:357`).
- `objects.md` has 11 rules, and none covers representation. A search for "object representation" and "aliasing" finds nothing.
- OBJ-10 (reuse of storage) has no `run-fail` test.
- 21 test files use `reinterpret_cast`. The interpreter stops at it in 12 run tests, plus one "reinterpreting pointer cast" and one "conversion from void*".
- `alignment.md` shows `alignas(16) unsigned char buffer[256]`, and the interpreter cannot place objects in such a buffer.

**What it unblocks.**
- `reinterpret_cast` and placement `new` in the interpreter.
- UNI-6 type punning.
- The deferred items S beyond S1 and T on the class-property lists.
- A byte-level object model decision for the interpreter, which currently holds abstract values.

**Size.** 8 to 12 rules, about the size of `objects.md`.

**Risk.** High.
- The similarity risk is high, because this area has well-known fixed wording.
- Compilers seldom diagnose these errors. The behavior shows mostly under optimization, so the `-O2` report compilers (decision 0043) are the main evidence.
- The interpreter would need a second memory model.
- The item should wait until items 4 and 5 show how interpreter evidence is handled.

### 7. Interpreter coverage of constructs that already have pages

**What it is.** Front-end work only. These constructs are specified, and the interpreter still stops at them.

**Evidence** (interpreter stops, by construct):

| Construct | Tests | Page |
|---|---|---|
| `new` with placement arguments or class allocation functions | 29 | new-delete.md |
| overloaded operators, including defaulted `==` and `<=>` | 19 + 3 + 2 | operators.md, comparisons.md |
| pointer to member (constant, function, access, cast) | 13 + 5 + 2 + 1 | member-pointers.md |
| class layout the front end does not model (anonymous unions, inherited constructors, `initializer_list` backing array) | 11 | unions.md, derived.md, initializer-lists.md |
| structured binding or init capture | 8 | structured-bindings.md |
| user-defined literal | 7 + 1 | operators.md |
| virtual base class | 7 | derived.md |

On the check side, 18 tests stop at the `noexcept` operator and 13 of the 40 tests in `exception-specifications` are unsupported, mostly on EXS-4.

**What it unblocks.** Most of the 199 unsupported run tests, and evidence for items 4 to 6.

**Size.** Medium.

**Risk.** Low. Any rule gap found during the work becomes a G finding.

### 8. A Diagnose batch on the missed rejects of finished pages

**What it is.** Clean-room Diagnose exercises (`documentation/exercises.md`) on the pages with the most missed rejects outside templates. The batch separates front-end gaps from text that does not say what to reject.

**Evidence.**
- There are 274 missed rejects outside `templates/`, thinly spread:
  - by directory: classes/virtual 28, exceptions/handlers 18, classes/unions 16, classes/conversion-functions 15, classes/comparisons 15, expressions/lambdas 13, expressions/member-pointers 13, classes/derived 13
  - by rule: TRY-5 (9), VIRT-3 (7), VIRT-15 (7), NEW-25 (6), LSP-7 (6), CVF-7 (6)
- Round 1 suggested this batch for attributes and virtual. Attributes is now 70 of 72, so that page's misses were front-end gaps. Virtual still has 28.

**What it unblocks.** The C item (reject rules say what to diagnose) on the classes and exceptions chapters, and a cleaner signal for item 1.

**Size.** Medium: 3 or 4 pages, about 100 tests.

**Risk.** Low.

### 9. Scope change: name the exceptions that the language throws; keep `typeid` out

**What it is.** Add to `language-support.md` (decision 0041) the facts the core language needs about the exceptions it throws:
- a failed `dynamic_cast` of a reference,
- a failed global allocation function,
- an array `new` with a bad length.

The rule would say only that each throws an object of a library class that a `catch (...)` handler or a handler for its public base catches. `typeid` and `std::type_info` stay out.

**Evidence.**
- Two of the five *(library referenced)* markers are these throws (`virtual.md:320`, `new-delete.md:58`).
- Round-7 finding G62 asked for the reference case. The test `dynamic-cast-reference-fails-throws.cpp` can only use `catch (...)`, because no test can name the type.
- `typeid` is a keyword (`keywords.md:40`) with no page. One test uses it, and TARG-15's Evidence says that test needs `<typeinfo>`. Deferred item T covers `typeid`.

**The case for adding:**
- It closes two markers.
- It lets the interpreter model the throws with a defined type.
- It matches LSUP-P1: the language builds these objects itself.

**The case against:**
- The model in LSUP-P1 depends on a program that declares the type itself. From general knowledge, the runtime library defines these classes. A program's own `namespace std` declaration of one would not reliably be the class the runtime throws. Tests could then catch only by `catch (...)`, and the rule would add little that tests can check.
- `typeid` needs `std::type_info`, which a program cannot declare in a usable shape. It also needs real headers, which needs the preprocessor and part of the library.

**Recommendation.** Add the narrow rule that each failure throws, with `catch (...)` tests. Name the classes as *(library referenced)*. Keep `typeid` deferred.

**Size.** 1 or 2 rules and about 4 tests.

**Risk.** Low.

### 10. Scope decision: keep lexical structure and the preprocessor beyond the current slice out

**What it is.** Confirm the charter's exclusion of lexical structure and of the preprocessor beyond `#include "file"`, guards, and the conditional-group slice on the ODR page. Do not move them in.

**Evidence.**
- `lexer.rs` (1037 lines) was written from general knowledge, as round 1 reported. The front end's lexer and preprocessing slice pass every test that depends on them, except two named-escape tests that need a Unicode name table.
- All 18 `#include` lines in tests name test headers. No test includes a standard header.
- Roadmap item G beyond G1 is deferred for similarity risk.

**The case for adding:**
- Every program depends on tokens.
- The complete grammar of item 3 uses terminals that no page defines.
- Alternative tokens decide the KW-1 tests.

**The case against:**
- High similarity risk.
- The compilers agree, so the tests can show little.
- A full preprocessor brings function-like macros and real headers, and real headers bring the library, which the charter excludes.

**Recommendation.** Keep both out. If item 3 shows that a terminal is unclear, add a row to the G1 token tables and leave the rest out.

**Size.** None.

**Risk.** Low. The risk grows if Tidal later wants tests that include standard headers. A future decision would need to handle that.

## 2. Dependency sketch

```
2 Run baseline ─┬─> 5 Undefined coverage ─> 6 Object representation
                └─> 4 Sequencing model ──┘
1 Templates in front end ─> 8 Diagnose batch
3 Grammar reject rule ─> 10 Keep lexical out (revisit only on item 3 findings)
7 Interpreter coverage ─> 5, 6
9 Language-thrown exceptions (independent)
```

Items 2 and 3 are small and can run beside item 1. Items 4 and 5 need item 2, so that run results are guarded. Item 6 comes last among the text items, because it needs the evidence decision of item 5.

## 3. In-scope text still thin for an implementer

- VIRT-10, NEW (failed allocation): the thrown type is unnamed (item 9).
- CAST-6: no rule for a read through the result (item 6).
- SEQ-4: no rule separates value from side effect (item 4).
- TYP-12 gives padding between members only. The front end cannot compute `sizeof` for a class with a base, a virtual function, or bit-fields. The affected tests are `sizeof-at-least-base.cpp` (DER-2), `bitfield-layout-lp64.cpp` and `bitfield-zero-width-splits.cpp` (BF-9), and `abstract-sizeof.cpp` (VIRT-8). Per-target layout tables (decision 0036) would cover these four tests. I do not rank the item because only four tests need it.
- Decision 0048 allows the interpreter to try every allowed choice and report the set. The interpreter takes one fixed choice. Item 4 makes this practical for SEQ-1 and SEQ-8.

## 4. Out of scope, unchanged

Coroutines, modules, threads and the memory model, the general library, extended floating-point types, and `typeid` stay out. Item 9 asks for a narrow add. Item 10 asks to keep lexical structure and the full preprocessor out.

The full harness output is in `/tmp/claude-1000/-home-ryanm-Code-tidal/f142cdb8-5f85-4266-b9df-c7416ea6d3a0/scratchpad/conform.txt`.
