# Roadmap, round 10: what to do after phase 12, from an implementer's view

Date: 2026-11-06. Decision 0029. Role: clean-room reader, acting as an implementer who builds a C++ front end and interpreter from Tidal alone. The evidence is the clean-room front end in `frontend/`, its reports and ledger in `reports/clean-room/rust-frontend/`, and the phase 12 exercise records. Commit read: `ebc109b9`.

Inputs:
- AGENTS.md, CLAUDE.md
- `reports/clean-room/roadmap-9/implementer.md`, the model for this report's form, and its scratch files
- `documentation/`: status, roadmap (the phase 12 table and the front-end section), exercises (the Implement prompt and the done table), decisions 0059 and 0060, and `decisions/pending.md` (the table, the settlements, and the start of the packet)
- `git log a15a877d..ebc109b9` with every commit body (20 commits)
- `reports/clean-room/rust-frontend/`: report 23 in full, `ledger.md`, `conformance.tsv` at `a15a877d` and at `ebc109b9`
- `frontend/README.md` (the sections on layout, the conditional operator, and templates), and `frontend/src/` only by line counts and a grep for rule IDs
- `reports/clean-room/I5/` (`report.md`, `notes.md`, the programs, the reductions) and `reports/clean-room/W20/` (`report.md`, the reductions)
- `reports/clean-room/ledger.md` (the 17 open rows)
- the two phase 12 gap reviews in `reports/review/` (the Result and rule tables)
- `specification/`: the Syntax segments of every page (by script), `templates/deduction.md` (DED-24), `declarations/declarators.md` (CV-6), `declarations/type-names.md`, `foundations/values.md` (the prvalue paragraph), `declarations/initialization/references.md` (REF-1), `foundations/translation.md` (the class table), `statements/blocks.md` (STMT-6), and the rule kinds of every page
- `tests/`: the headers of the 204 failing tests and of the REF-1, ACC-12, and CONV-17 tests

I did not read the other four round 10 reports, `inputs/`, or `build/exercise-keys/`.

**Measurements.** I built the front end with `cargo build --release --locked --target-dir` into my session scratch directory, so `frontend/target` was not touched. I did not run `tools/frontend_check.py` or `tools/run_tests.py`. I ran `tidal-fe conform tests -v --out` once. Its counts match the phase 12 close commit: 6586 of 6790 tests pass, 6626 pass their check. I compared the result with `conformance.tsv` at `a15a877d` (the round 22 baseline). I also ran `tidal-fe run` on the three I5 reductions, the 13 I5 programs, and the four W20 reductions. The failure list and the counting script (round 9's, unchanged) are in `reports/clean-room/roadmap-10/implementer-scratch/`. Counts that come from grep are approximate. Statements from general knowledge say so.

## 1. Where the specification and the front end stand

| Measure | Round 9 (6711 tests, `a15a877d`) | Now (6790 tests, `ebc109b9`) |
|---|---|---|
| Full pass | 6478 (96.5%) | 6586 (97.0%) |
| Check pass | 6520 | 6626 |
| Failing tests | 233 | 204 |
| `MISSED-REJECT` | 118 | 100 |
| `unsupported` (check) | 62 | 58 |
| `run-unsupported` | 42 | 40 |
| `FALSE-REJECT` | 9 | 4 |
| `EDG-AGREES` (a failure) | 2 (CPY-27, CNS-32) | 2, the same |
| Run tests | 1715: 1662 pass (11 partial), 42 unsupported, 11 not run | 1756: 1711 pass (15 partial), 0 wrong, 40 unsupported, 5 not run because the check fails |
| Program tests under `tests/programs/` | 103; `r8-tasks.cpp` a false reject | 115, all run (the second condition of decision 0060 holds) |
| Passing rejects whose first error names a cited or linked rule | 2005 of 2612 (77%) | 2030 of 2647 (77%) |
| `reject` rules absent from the front end's source | 103 of 558 | 101 of 559 |
| `unspecified` rules named in the front end's source | 13 of 32 | 14 of 32 |
| Rules with a run test and no passing run test | 14 | 11 |
| Rules | 1697 | 1698 (TR-17 added) |
| Questions open | 174 | 176 |
| Front-end findings (G) | G1 to G152, 0 open | G1 to G157, 5 open (G153 to G157) |
| Exercise ledger rows open | 68 | 17 (W20-1 to W20-6, I5-1 to I5-11) |
| Front-end source | 50160 lines of Rust, 28 files | 50673 lines, 28 files; round 23 changed 13 files (+863, -350) |

### What phase 12 took from round 9

| Round 9 item | Phase 12 result |
|---|---|
| 1. Front-end round 23 | Done (`1017b19c`). The two wrong verdicts went, all 105 tests added since round 22 pass, and 18 old failures passed. |
| 2. Tests for the six reductions | Done in item T (`87209705`), with the round 9 programmer reductions and the r9 and P1 program tests. |
| 3. Answers for I4-8 to I4-14 and the ended `planned` rows | Done in F2 (`03e578c4`, `3b83788d`, `0f3f522f`). No row is `planned` now. |
| 4. Implement I5 | Done (`2bb7d26c`): 416 of 427 assertions, findings I5-1 to I5-11. |
| 5. Take or drop the three items deferred three times | Dropped by decision 0059. The first-error rate stays at 77% for a fourth phase. |
| 6. The rules the interpreter cannot run | X23-6 answered in STG-8 (`0f3f522f`). ADD-18 still has no run test: its two tests are ILP32 constant tests, both unsupported. 11 rules have no passing run test. |
| 7. A front-end prototype of P23 | Deferred again by decision 0059. |

Of round 9's process changes, phase 12 took changes 1 (re-plan ended rows, with a check in `13c03237`), 2 (a front-end round in each phase that changes rules), 3 (a test for each front-end finding), 5, 6 (`status_table.py` writes the front-end sentence), and 7. Change 3 became decision 0059's choice "A front-end finding gets a test in the commit that records it". Item E did not apply it. I5-9, I5-10, W20-2, and W20-3 each have a reduction that stops the front end, and no test under `tests/` holds any of them (grep for the reductions' forms and for the finding IDs).

### The 204 failures

Every one of the 204 failed at round 22 with the same status. No test that the front end passed at round 22 fails now, and no test added in phase 12 fails. So the failing set has only shrunk over two rounds. The front end failed 222 at round 22, and round 23 passed 18 of them.

By status and chapter:

| Chapter | Missed reject | Unsupported | Run unsupported | False reject | EDG-AGREES | All |
|---|---|---|---|---|---|---|
| templates | 35 | 17 | 8 | 0 | 1 | 61 |
| declarations | 20 | 7 | 3 | 1 | 0 | 31 |
| classes | 16 | 7 | 11 | 0 | 1 | 35 |
| expressions | 11 | 12 | 6 | 3 | 0 | 32 |
| foundations | 7 | 4 | 3 | 0 | 0 | 14 |
| overloading | 6 | 11 | 1 | 0 | 0 | 18 |
| statements | 3 | 0 | 5 | 0 | 0 | 8 |
| exceptions | 2 | 0 | 3 | 0 | 0 | 5 |
| **total** | 100 | 58 | 40 | 4 | 2 | 204 |

By the class of the rules each test cites:

| Rule class of the test | Tests | What a front end must do |
|---|---|---|
| `reject`, alone or with `required` or `impl-defined` | 86 missed, 7 unsupported, 3 run | a diagnostic |
| `required` only | 43 unsupported, 34 run, 4 false rejects | the right answer or the right run |
| `recommended`, with `-Werror` | 7 missed | a warning (VIRT-13, REF-15 twice, TRY-13, UNEV-4 twice, UN-19) |
| `no-diagnostic` only | 6 missed, 2 unsupported | nothing: TR-P1 says an implementation need not notice |
| `undefined` or `impl-defined`, with no `reject` | 10 | the run's stop, or the layout |
| `EDG-AGREES` (CPY-27, CNS-32) | 2 | the base answer |

The 8 `no-diagnostic` tests are `inline-variable-undefined-used.cpp` (INL-6), `alignment-enumeration-class.cpp` (LSUP-5), `base-pack-expanded-twice.cpp` (DEP-33), `incomplete-then-complete.cpp` (DEP-31), `independent-error-unused-call.cpp` (DEP-30), `partial-after-use.cpp` (TSP-17), and `operand-cycle.cpp` and `operand-cycle-static.cpp` (exception specifications). The harness counts an accept as a missed reject. By TR-P1, an implementation that accepts these programs follows Tidal.

### The causes that the front end gives

The check-side `unsupported` messages (58):

| Message | Tests | Note |
|---|---|---|
| `static_assert`: variable template with unknown arguments | 14 | A symptom. The root causes differ: `auto x = {1, 2}` (AUTO-7), the `f64` suffix, class template argument deduction at six sites, a pack in a list. |
| `static_assert`: concept | 8 | 5 are ICS-45 explicit reference conversions |
| `static_assert`: `new` expression | 5 | |
| `sizeof` of a class with a layout the front end does not compute | 4 | BF-9 wide bit-fields, one on ILP32 |
| `noexcept` operator | 4 | 2 are `no-diagnostic` cycles |
| constant initializer: pointer arithmetic | 3 | 2 are ILP32 tests of ADD-18 |
| constant initializer: conversion function | 2 | both extended floating-point |
| array bound: non-integer constant | 2 | a class with a conversion function as a bound |
| others, one or two each | 16 | among them a named escape, `#include <...>`, a dependent name, a call needing deduction |

The run stops (40): `new` forms (7), constructor calls (4), a variable of an enclosing function (3), an overload set (3), a jump into a nested statement (3), a variable or function with no definition in the unit (4), a missing default argument (2), a destructor with a function try block (2), a dependent name (2), a call of an unknown callee (2), and 8 single causes.

The 11 rules with no passing run test: DEP-20, DEP-25, IF-12 (2 tests), JMP-16, LAM-19, ORD-9, OVL-42, PFX-22 (2 false rejects), SB-12, TARG-12, TSP-8.

### The missed rejects by page

| Chapter | Pages and rules |
|---|---|
| templates (35) | instantiation 5 (INST-12 twice, INST-15, INST-21, INST-33); dependent-names 5 (DEP-30, DEP-31, DEP-33, DEP-37 twice); constraints 5 (CNS-13, CNS-24, CNS-26, CNS-29 twice); class-template-deduction 4 (CTAD-2 twice, CTAD-12, CTAD-19); arguments 4 (TARG-24 twice, TARG-7, TARG-15); packs 3; specialization 3; deduction 2 (DED-29, the lambda tests); declarations 2; concepts 1; ordering 1 |
| declarations (20) | default arguments 5 (DEFARG-13 four times, DEFARG-4); initialization 6 (LIST-13 twice, LIST-16 twice, REF-15 twice); constexpr 3 (CEV-1, CEV-2 twice); linkage specifications 2 (LSP-7); auto, alignment, declarators (ILP32), inline 1 each |
| classes (16) | comparisons 5 (DCMP-12 and DCMP-13 three times, DCMP-11, DCMP-3); access 3 (ACC-15 twice, ACC-18); constructors 3; conversion functions 2; virtual 2; copy-move 1 (CPY-15) |
| expressions (11) | member pointers 4 (MPTR-11 three times, MPTR-10); decltype 3; logical, postfix, primary, unary 1 each |
| others (18) | names 3 (NAM-42 twice, NAM-27); overload resolution 4 (OVL-9 twice, OVL-20 twice); conversion sequences 2; range-for 2; language support 2; storage, constant evaluation, `if`, handlers, unwinding 1 each |

Report 23 section 3 gives the cause of 9 template failures, and each one comes from the way the front end makes a specialization. The front end parses a template's tokens once as a pattern and again with the arguments bound (`frontend/README.md`, section "Templates"). The DED-29 lambda tests (2) need the lambda parsed again during substitution. CNS-29 (2) needs clause tokens that the check never recorded. PACK-8 in an outer pattern (2) needs a check that tracks which pack a fold consumes. DEP-30 and IF-22 (2) need a call with only non-dependent arguments resolved inside the pattern. TSP-31 (1) needs the variable partial specialization path. One of the 9 (DEP-30) cites only a `no-diagnostic` rule. The `no-diagnostic` missed rejects hold two more DEP tests of the same kind (DEP-31, DEP-33).

### Decision 0060: how far the front end is from the completion test

The second condition holds. All 115 program tests run to their result.

The first condition counts 204 tests, because no front-end ledger row marks a test `out of scope`. Report 23 section 6 names six tests that could take the state: the two `<cstdarg>` tests, `char-named-escape.cpp`, and the three tests that need an ILP32 bound or a size above the size type. I read the list differently:

| Group | Tests | Proposal |
|---|---|---|
| `<cstdarg>` and `va_list` (PFX-22) | 2 | out of scope: the general library and standard headers (charter) |
| named escapes `\N{...}` (PRIM-5) | 2: `char-named-escape.cpp` (false reject) and `char-named-escape-unknown.cpp` (unsupported) | out of scope: a Unicode name table is data outside the specification |
| `#include <...>` (`include-missing-file-angle.cpp`) | 1 | out of scope: the front end reads only `#include "file"` by design |
| extended floating-point types (ICS-35 through PRIM-22) | 4: `conversion-choice-equal-rank-extended-target.cpp`, `conversion-choice-equal-rank-source.cpp`, `floating-before-rvalue-reference.cpp`, `floating-equal-rank.cpp` | out of scope: G42 already marks the topic `out of scope` |
| `no-diagnostic` only | 8 | not a failure: accepting follows TR-P1. The harness should judge them as it judges `tidal-allows-edg` |
| ILP32 (DCL-19 4 GiB array, two ADD-18 tests, one BF-9 layout) | 4 | in scope: the front end takes `--m32`, the matrix runs EDG ILP32, and each test needs a feature that LP64 code also needs (the size type's width, pointer arithmetic in a constant, wide bit-field layout) |

With these, 9 tests leave by `out of scope` and 8 leave by their class. 187 tests remain. Of those, 7 are `recommended` warnings.

The front-end ledger cannot hold a test today. Its rows are findings (G1 to G157), its `out of scope` state names a topic (G42), and `tools/frontend_check.py` does not compute the completion test. So the 9 tests need either one G row per group with the test paths in the Record, or a column for tests, and the harness has to read it.

At round 22's rate (36 old failures) or round 23's (18 old failures beside 105 new tests), 187 tests need between 5 and 10 rounds. Decision 0060 lets a roadmap round propose the rewrite earlier "if the remaining failures are few and stable over two rounds". The set is stable over two rounds. It is not few.

### What the rewrite needs first

Decision 0060 lists a design document, modules by chapter and page, a parser from the Syntax segments, unit tests per stage, and short files. The tree gives these facts on each:

- **Design document.** `frontend/README.md` (445 lines) describes use, layout, and some procedures (the conditional operator, templates, bytes). It does not name the data that passes between stages, and decision 0060's context says so too. A design document for the current code can be written now without changing any code. The rewrite's own design document has to choose the template model first. The current model (tokens parsed again with arguments bound) causes the 9 template failures above. Whether the rewrite keeps that model or builds a typed dependent form decides most of `templ.rs` (4226 lines), `cons.rs`, and `ctad.rs`.
- **The parser from the Syntax segments.** The specification has 255 nonterminals in `productionlist` blocks on 66 pages (expressions 87, declarations 58, classes 44, templates 27, statements 21, foundations 13, exceptions 5). `check_specification.py` checks that each nonterminal has one definition and that each used nonterminal is defined. Two gaps stop a generated or derived parser:
  - No production for a translation unit. `declaration` is used in namespace bodies, linkage specifications, and explicit specializations, and nothing derives a whole file.
  - 17 nonterminals are used nowhere: `anonymous_union`, `conversion_decl`, `ctor_def`, `dtor_decl`, `dtor_call`, `inheriting_decl`, `copy_ctor`, `move_ctor`, `copy_assign`, `move_assign`, `defaulted_comparison`, `auto_var_decl`, `typedef_decl`, `forward_class`, `include_directive`, `guarded_header`, and `pragma_once`. Most describe the shape of a declaration that a general production already derives (`copy_ctor` names a constructor whose parameter is `cv class_name&`). A parser has to know which productions are grammar and which are shapes.
  - The grammar has no disambiguation of its own. STMT-6 and TR-12 give the declaration-or-expression rule in prose, and a parser also needs to know whether a name is a type or a template. A hand-written parser that cites a production in each function fits these facts better than a generated one (general knowledge on C++ parsers).
- **Modules by page.** The front end names rules from 84 of the 85 pages. The median page has rules named in 6 files. `expr.rs` names rules from 65 pages, `decl2.rs` from 52, `interp/expr.rs` from 48, and `parser.rs` from 46. Seven files exceed decision 0060's limit of about 2000 lines: `expr.rs` 7570, `decl2.rs` 7089, `interp/expr.rs` 4746, `templ.rs` 4226, `decl.rs` 3135, `expr2.rs` 2643, `parser.rs` 2389.
- **Unit tests per stage.** None exist. The conformance harness is the only test.
- **The first-error rate.** 617 passing rejects name a rule that the test neither cites nor links, and the rate has stayed at 77% for four phases. Modules by page with rule IDs in comments give the rewrite a place to fix this, and the rewrite needs a target for it.

### The I5 findings on deduction

The reader scored 416 of 427. The eleven open rows sort into three groups by who acts:

| Row | Who acts | What the tree already holds |
|---|---|---|
| I5-10, front end binds a reference to a private or ambiguous base | front end, after a test | Reductions r2 and r3 and programs p02 and p08 still return 1 on the front end. The front end rejects the same conversion through a pointer (`classes/access/private-base-conversion.cpp`, `overloading/conversion-sequences/chosen-private-base.cpp`). No REF-1 test binds a reference to a private or ambiguous base. Round 23 fixed the same gap inside the conditional operator only (I4-6, I4-7, G153). |
| I5-9, `T::type` with an incomplete `T` stops as unsupported | text first (I5-5), then front end | Reduction r1 and program p03 still stop: "parameter of unknown type". |
| I5-4, DED-15 does not point to OVL-37 | text | Same mechanism as I5-10. A test with two equal bases and a fallback is the missing test. |
| I5-5, `T::type` with an incomplete `T` | text | DED-24's list and INST-1's list. |
| I5-3, `const` on a function type | text | CV-6 and `declarations/type-names.md` say a cv-qualifier on a function type through an alias has no effect. Neither names a template parameter, and DED-7 and DED-14 do not link them. The text needs one link and the words "or a template parameter". |
| I5-6, cv of a non-class prvalue | text | `foundations/values.md` already says that a prvalue of a type that is not a class type has no cv-qualifiers. DED-8 does not link it. The reader did not read the page. |
| I5-1, I5-2 | text | DED-30 to DED-29's address paragraph; the wording of DED-29's first list. Both are one sentence. |
| I5-7, I5-8, I5-11 | text | Examples for DED-20 and DED-P4; DED-24 adds arrays of functions and of `void`; DED-11 names the size type as `decltype(sizeof 0)`. |

Five of I5's eleven wrong assertions come from its interface (the empty explicit list, a specialization named without a call) and six from the text or from pages outside the reader's list (close commit, correcting `2bb7d26c`). Two of the six come from the ICS and partial-ordering pages that the reader did not read (I5-11). I3 and I4 also took conversion ranks from outside knowledge.

### W20 and the front end

The four W20 programs run on the front end, and all four are program tests. The reductions:

| Reduction | Front end now | Row |
|---|---|---|
| `rows-not-contiguous.cpp` | `undefined [PFX-4]` | W20-1: the front end is right; the text should say so beside `FixedVec` |
| `union-array-member-subscript.cpp` | "unsupported: pointer arithmetic" | W20-2: the text has no rule for the elements of an array member of a union, so the front end cannot be fixed first |
| `self-by-value-template-constraint.cpp` | rejected `[CNS-35]` | W20-3: GCC and Clang accept. A false reject of a valid program, and no test holds it |
| `tag-address-compare-gcc-ubsan.cpp` | result 0 | W20-4: no front-end work |

## 2. Ranked items

| Rank | Item | Evidence | Size | Prerequisites |
|---|---|---|---|---|
| 1 | Tests for every front-end target without one | I5-9, I5-10 (r1, r2, r3, p02, p08), W20-3, G154 (copy-initialization through a private copy constructor) | small | none |
| 2 | Front-end round 24, in five groups | 187 tests in scope after item 3; the item 1 tests; G153 to G157 | large | 1 for group 1; 3 for the count; 4 for group 5 |
| 3 | The completion test made countable | 9 `out of scope` tests in 4 groups; 8 `no-diagnostic` tests; no ledger form for a test; `frontend_check.py` does not count | small | none |
| 4 | Text for the 17 open rows and G153 to G157 | section 1 tables; I5-3 and I5-6 are links to rules that exist | small each | none |
| 5 | Implement I6: dependent names and the checks before instantiation | dependent-names has the most failures of any template page (10); 9 failures trace to the template model; the rewrite has to choose that model | medium | none |
| 6 | Preparation for the rewrite that touches no code | design document of the current stages; the grammar gaps (no start symbol, 17 shape productions); a module map by page | small to medium | none |
| 7 | An amendment to decision 0060 | 187 tests at 18 to 36 per round; 9 failures that the current template model causes | small | 3, 5 |

### 1. Tests for every front-end target without one

**What it is.** The clean-room writer turns each reduction that stops the front end into a test, as item T did in phase 12:
- I5-10: two reject tests from r2 and r3 (REF-1 with ACC-12, REF-1 with CONV-17), and one from p02 with two equal `Base<int>` subobjects and a fallback (DED-15, OVL-37).
- I5-9: a run test from r1 (DED-24) that expects the fallback.
- W20-3: an accept test from the reduction (CPY-15, CNS-11, CNS-35). The front end and both compilers disagree, so the writer probes first and records a question if the text does not decide it.
- G154: copy-initialization, a by-value argument, and a return that select a private copy or move constructor (ACC-3).

**Evidence.** Section 1. Decision 0059's choice says a front-end finding gets a test in the commit that records it, and item E recorded four findings without one. Programs p02 and p08 return 1 on the front end, and GCC and Clang reject both. These are wrong verdicts on invalid programs, and no baseline row can record their fix.

**Size.** Small. Each reduction has under ten lines.

**Prerequisites.** None. W20-3 may need a question.

### 2. Front-end round 24, in five groups

**What it is.** A round in the form of report 23, with section 6 kept. The groups, in order:

1. **Wrong verdicts and the item 1 tests.** I5-10 first: the reference binding checks the base as the pointer conversion does. One fix serves REF-1, DED-15 with OVL-37, and the I4-6 and I4-7 forms outside the conditional operator. Then I5-9 once I5-5 has text, W20-3 once item 1 decides it, and the false reject `tuple-member-get-type-parameter-uses-free.cpp` (SB-5, the front end gives DED-35).
2. **Root causes in the unsupported messages.** The 14 "variable template with unknown arguments" tests report a symptom. The round makes the message name the first construct that gave an unknown type. Then it closes the causes with more than one test: `new` in a constant (5) and in a run (7, among them DED-39 twice and LAM-19), constructor calls in a run (4), BF-9 layout (4, one ILP32), pointer arithmetic in a constant (3, among them ADD-18), the `noexcept` operator on a later member (2 in scope), a class bound with a conversion function (2), and the CTAD sites behind the symptom.
3. **The run stops that free whole rules.** Jumps into a nested statement (IF-12 twice, SW-1), variables of an enclosing function (JMP-16, SW-5, UNI-7), overload sets in a run (CVF-1, CVF-4, CVF-5), missing default arguments (TSP-8, NEW-23), dependent names in a run (DEP-20, DEP-25), unknown callees (TARG-12, DEP-11), structured bindings (SB-12), class assignment (OVL-42). These close 8 of the 11 rules with no passing run test. Group 2's `new` forms close LAM-19 and ORD-9, and PFX-22 leaves by item 3.
4. **Missed rejects outside templates (65).** The groups with several tests first: DCMP-11 to DCMP-13 (5), DEFARG-13 (4), LIST-13 and LIST-16 (4), MPTR-11 (3), CEV-1 and CEV-2 (3), OVL-9 and OVL-20 (4), ACC-15 and ACC-18 (3), NAM-42 (2), LSP-7 (2), REF-15 (2 warnings). Then the 7 `recommended` warnings and the single rules.
5. **Template missed rejects and unsupported tests (61).** After item 5 reports. CTAD-2 (2), TARG-24 (2), CNS-24 to CNS-27 (2), INST-12 (2), and the single rules first, since they do not depend on the template model. The 9 tests whose cause is the template model go last, or to the rewrite under item 7.

**Evidence.** Section 1. Round 23 passed 18 old failures and every new test. All 204 failures date from round 22 or earlier.

**Size.** Large. Groups 1 and 3 are small to medium. Groups 2 and 4 are medium. Group 5 is medium to large.

**Prerequisites.** Item 1 for group 1. Item 3 so that section 6 of the report counts the same set as the decision. Item 5 before group 5 changes the template model.

**Risk.** Low. A group that the front end cannot close from the text becomes a G finding with a ledger row and a test.

### 3. The completion test made countable

**What it is.**
- The front-end ledger gains a way to mark a test `out of scope`: one G row per group with the test paths in the Record, or a Tests column. The four groups are `<cstdarg>` (2), named escapes (2), `#include <...>` (1), and extended floating-point types under G42 (4).
- The harness judges a test whose cited rules are all `no-diagnostic` as passing when the front end accepts, as `tidal-allows-edg` already lets a second answer pass. These are 8 tests.
- `tools/frontend_check.py` prints the completion count: the tests that fail their check, the tests that an `out of scope` row holds, and the program tests that do not run.

**Evidence.** Section 1. Report 23 section 6 gives 6 candidates and a different set from mine. Without one tool, each round and each roadmap will count a different distance.

**Size.** Small.

**Prerequisites.** None. The `no-diagnostic` judgment changes the harness's rules, so the phase decision should adopt it as a choice.

### 4. Text for the 17 open rows and G153 to G157

**What it is.** The clean-room writer acts on each row, in this order:
1. **I5-4 with G153.** One sentence in DED-15 that the base's access and ambiguity are checked after the choice (OVL-37). For G153, COND-16 says whether steps 1 and 2 ask for the base relation without access, and REF-1 then agrees. The front end already takes that reading in the conditional operator and needs the same answer for item 2's group 1.
2. **I5-5.** DED-24's list gains a qualified name in an incomplete class. INST-1's list gains a qualified name that names a member.
3. **W20-3.** CPY-15 or CNS-11 says when the by-value specialization leaves the candidates, relative to the constraint check.
4. **I5-3 and I5-6.** Links that point to rules that exist: CV-6 gains "or a template parameter", DED-7 and DED-14 link it, and DED-8 links the prvalue sentence of `foundations/values.md`.
5. **I5-1, I5-2, I5-7, I5-8, I5-11.** One sentence or one example each.
6. **G155.** NEW-25 or a new reject rule covers a deleted or inaccessible allocation function. The front end now reports DEL-1 or ACC-3 first.
7. **G156.** LSUP-11 states every element type, or a question holds the rest. The front end rejects every element type now.
8. **G157.** INST-11's Evidence names the uses that make a virtual function on the matrix compilers.
9. **W20-2.** A rule for the elements of an array member of a union, or a question. The front end cannot run the reduction until then.
10. **W20-1, W20-5, W20-6, W20-4.** Storage-page notes, a CAST-1 sentence, the storage-form table, and a CMP-15 Evidence note. The front end is already right on W20-1.

**Evidence.** Section 1.

**Size.** Small each. W20-2 may need a probe and a question.

**Prerequisites.** None.

### 5. Implement I6: dependent names and the checks before instantiation

**What it is.**
- A fresh clean-room reader builds a component from `templates/dependent-names.md` (39 rules, 6 principles), with INST-1 to INST-13 and the DED-29 paragraph on instantiation, filtered of Evidence segments and `:tests:` lines as in I4 and I5.
- The component takes a template definition in a small tree form and returns, for each name and expression, whether it is type-dependent or value-dependent, where its name binds (at the definition, or at the instantiation with argument-dependent lookup), whether `typename` or `template` is needed, and which checks run before any instantiation. For a `no-diagnostic` rule it says that a check may run.
- The scoring phase uses the 125 tests in `tests/templates/dependent-names/` and the page's probe grids.
- The prompt names I5's notes items 4 and 23 (which uses instantiate a class, and when an error in an instantiated class body is a hard error) and asks the reader to log them if the text does not decide them.

**Evidence.**
- The dependent-names page has 10 failures, the most of any template page, and DEP-20 and DEP-25 have no passing run test.
- 9 template failures trace to the front end's model of a specialization (section 1). The rewrite must choose a template model before it writes `templ.rs` again, and a blind reading of the dependence rules tests whether the text gives that model.
- No blind build has read the dependent-names page. I1 to I5 each found more than a dozen unstated steps, and I5 found two front-end faults through programs that its passing assertions did not show.

**Size.** Medium. The type model of I5 covers part of the input.

**Prerequisites.** None.

**Alternatives.** Class template argument deduction (CTAD-P1, 29 rules, 118 tests, 8 front-end failures) is a smaller, bounded algorithm, and I5's deduction would serve as its callback. Constraint normalization and subsumption (CNS-P1, 12 failures on two pages, Q-0411) is the round 9 alternative. I rank dependent names first because the rewrite depends on it.

### 6. Preparation for the rewrite that touches no code

**What it is.**
- A design document of the current front end in `frontend/`: the stages, the data each passes (tokens, entities and scopes, the template record of tokens, the interpreter's values and places), and where each chapter's rules sit. A clean-room writer can write it from `frontend/` now. The rewrite's design document then starts from a description of what it replaces.
- The grammar made usable by a parser: a production for a translation unit, and a mark on each of the 17 shape productions (or a move of each into prose). `check_specification.py` can then fail on a nonterminal that no start symbol reaches and that has no mark.
- A module map: for each page, the files that name its rules now, and the module the rewrite gives it.

**Evidence.** Section 1, "What the rewrite needs first".

**Size.** Small for the grammar, medium for the design document.

**Prerequisites.** None. These items change no front-end code and keep 0 worse by construction.

### 7. An amendment to decision 0060

**What it is.** The phase 13 decision states what "few" means, for example at most 50 in-scope failures, and lets the round that reaches it name the tests the rewrite takes over. Each such test needs a reason, such as the template model.

**Evidence.** 187 in-scope failures at 18 to 36 old failures per round give 5 to 10 rounds before the rewrite can start. Some of the last failures cost more in the old template model than in a new one (section 1). Decision 0060 already allows an earlier proposal for few, stable failures, and the set is stable now.

**Size.** Small.

**Prerequisites.** Item 3 for the count, and item 5 for the template model.

### Dependency sketch

```
1 tests ──────────────> 2 FE group 1 (wrong verdicts)
4 text (I5-4, I5-5, W20-3) ──> 2 FE group 1
3 countable test ─────> 2 FE report section 6, 7 amendment
5 Implement I6 ───────> 2 FE group 5, 6 design document, 7 amendment
6 preparation (independent of the old code)
```

Items 1, 3, 4, and 6 are small and can start with item 2. Item 5 reads pages that item 2's first four groups do not change.

## 3. Scope

| Topic | Charter's reason to keep out | Implementer's view | Proposal |
|---|---|---|---|
| `<cstdarg>` and `va_list` | the general library, and no standard headers | 2 PFX-22 false rejects since round 6; PFX-22 has no passing run test | Out of scope, with a front-end ledger row |
| Named escapes `\N{...}` | lexical structure is in scope; the name table is data | 2 tests, a false reject and an unsupported | Out of scope for the front end, with a row; keep the tests |
| `#include <...>` | real headers come with the library | 1 test, unsupported by design | Out of scope, with a row |
| Extended floating-point types | *(referenced)* only; G42 | 4 tests stop on `f64` | Out of scope under G42 |
| `no-diagnostic` rules | the class needs no diagnostic | 8 tests count as failures | A pass when the front end accepts |
| ILP32 | in the matrix (EDG ILP32) | 4 tests need the size type's width or a layout | Keep in scope |
| Coroutines, modules, threads, the memory model | outside the language goal | No test or front-end path needs them | Keep out |
| Later language versions (`pending.md` row 7) | the charter fixes C++23 | The front end's work would not change | Keep out |
| MSVC | not in the matrix | A second ABI would double the layout tables | Keep out |
| Propose designs (P23, P25) | not part of the rules | Deferred four times | Keep out until the rewrite; a prototype in the new code costs less |

## 4. Process changes

1. **Apply decision 0059's test choice to exercises.** An Implement or Write commit that records a front-end finding with a reduction adds the test in the same commit, or lists the finding as waiting for item T. Item E left I5-9, I5-10, W20-2, and W20-3 without tests.
2. **One tool counts the completion test.** `tools/frontend_check.py` prints the tests that fail their check, the tests that an `out of scope` row holds, and the program tests that do not run. Report section 6 and the phase decision copy that line.
3. **The front-end ledger can hold tests.** A row in state `out of scope` names its tests, and `check_specification.py` fails on a path that does not exist.
4. **Unsupported messages name the first unknown.** A message that reports a symptom ("variable template with unknown arguments") hides its group. The round counts the root causes.
5. **Each round report gives the rewrite's measures.** Beside section 6: files over 2000 lines, pages per file, and the first-error rate. The rewrite needs a baseline for each.
6. **Implement prompts list the pages a component needs.** I5's reader missed the ICS page and partial ordering, and two wrong assertions came from them (I5-11). The prompt should name the pages whose results the component takes as callbacks and give their rules, or give the callbacks' answers.
7. **Keep the front-end report form.** Round 24 keeps report 23's tables by target and by group, and its section 6 on the completion test, and adds a row for each item 1 test.
