# Front end, rewrite: build step 3 in part, the first takeover, and the P22 prototype

Date: 2026-11-11. Decision 0065 (phase 17, item RW2, with the parts of item R11 that touch the same files).
Role: clean-room reader in the front-end role. Inputs: AGENTS.md, CLAUDE.md, decisions 0035, 0060, 0064, and 0065, `documentation/exercises.md` section 9, `frontend/README.md`, `frontend/DESIGN-current.md`, `frontend/DESIGN-rewrite.md`, `frontend/TEMPLATE-MODEL.md`, `reports/clean-room/rust-frontend/report-rw1.md` and `report-rw2.md`, `reports/clean-room/roadmap-14/implementer.md` (item 2) and `designer.md` (items 1 and 2, section 1.10), `reports/clean-room/X24/report.md`, the rules of `specification/`, the tests under `tests/`, `tools/frontend_check.py`, the crate `frontend/next/`, and the old front end in `frontend/src/` (its harness, its types, and the rule IDs its checks name). No compiler was run, and no crate was added. Commit read: `cb40b18b`; the diff was then applied onto `73ed0807`, and the counts below are of that tree.

The coordinator reported that front-end round 28 (`c0461b86`) changed `frontend/src/interp/expr.rs` and `frontend/src/interp/mod.rs`. This item copied no file of `frontend/src/interp/`: the machine of step 3 is written new (section 2), so no change of `c0461b86` was taken. `DESIGN-rewrite.md` section 5 gives 38 marked tests (17, 15, and 6) and names the two new marks of model section 3, as the marks file at `73ed0807` holds.

Between `cb40b18b` and `73ed0807`, item F7 (`b511838d`), front-end round 28 (`c0461b86`), and the RV10 repairs (`88d14ea2`) landed. The RV10 repairs changed the Syntax segments, and this item follows them in the parser: the new nonterminals `decltype_expr` (`expressions::decltype`) and `new_ptr` (`expressions::new_delete`); `decltype_spec` as `decltype_expr` or `decltype(auto)`; `decltype_expr` after `~` in `unqualified_id`, before `::` in `nested_name`, and in `dtor_call`; the arm `nested_name "template" template_id` and `decltype_expr` in `base_type` and `cast_type`; leading attributes on `using_directive` and `sb_decl`; and a template-id in `using_enum_decl`. The nonterminal test counts 262 nonterminals on 62 pages, and two unit tests cover the new nonterminals.

The reader returned this report as a message, and the integrator saved it here.

## 1. Conformance

### Method

`tools/frontend_check.py --next` ran under the run lock on the tree at `73ed0807` with this item's diff, and exited 0. No run used `--update`. The old front end's lines are round 28's: the item changed no file that the old front end or its part of the tool reads.

```
frontend: 7049/7148 pass, 7069/7148 pass their check, 0 worse, 0 better, 0 moved between passing statuses, 17 new (7 not passing)
frontend: 22/22 interpreter-only expectations met
frontend: completion (decision 0060): 70 tests fail their check, of which 38 need the template model (marks file) and 32 count toward the gate of decisions 0062 and 0063; 1/178 program tests do not run to their result; 9 tests out of scope
frontend: 2177/2805 passing rejects name a cited rule (1988) or a linked rule (189) in their first error; `tidal-fe conform` gives the table per page
```

The `next:` lines:

```
next: unit: 126 tests pass
next: lex: 7266/7266 files give the old front end's tokens
next: parse: 6828/6987 tests parse as their outcome needs
next: parse: 142 of the 159 disagreements are reject tests that stop on the line of the old front end's first error
next: shadow: step 3 plain: 1602/1897 pass their check, 1600 of the old front end's 1892
next: shadow: step 3: 1795/3832 pass their check, 1792 of the old front end's 3816; 3 pass on the new front end only
next: shadow: all: 1915/7148 pass their check; 4954 unsupported
next: owned: 1327 tests (takeover list)
```

The tool also lists the 292 step 3 plain tests whose check the old front end passes and the new one does not. The tests that pass on the new front end only are three new tests of RV10 (`declarations/attributes/maybe-unused-on-using-directive.cpp`, `declarations/auto/decltype-auto-destructor-call.cpp`, `declarations/auto/decltype-auto-functional-cast.cpp`). Each is a reject test whose first error on the new front end names neither a cited nor a linked rule, so none joins the takeover list.

### Counts

| Measure | At `cb40b18b` | After the item |
|---|---|---|
| `next: unit` | 107 | 126 |
| Lines of `frontend/next/src/` | 10604 in 208 files | about 22240 in 302 files; the longest file is `expressions/primary/lexical.rs` (857) |
| Step 3 plain tests whose check passes (shadow) | none (no analysis) | 1602 of 1897; the old front end passes 1892 |
| Of these, full passes (the run passes too) | none | about 1515 |
| Step 3 tests whose check passes | none | 1795 of 3832; the old front end passes 3816 |
| Tests owned by the new front end | 0 | 1327 |

## 2. What changed

### Item R11, records

- **`DESIGN-rewrite.md` section 7 follows X24.** The worked example uses X24's rule IDs and change table: X24:new-1 and X24:new-2 are the checks, the same-function condition is X24's JMP-14 amendment, and X24:new-3 is the run-time rule with no code. "Run: none" is stated as a condition on X24-1, which item F7 answers. A design's own rule IDs come with the `Ext` node point at step 3, and rule overrides at step 4 change only existing rules. The kinds of the own IDs come from the change table: `p22.rs` lists them, and a unit test reads X24's table and checks each kind. The section names the crate's two points (`sem::check::NodeCheck`, `sem::check::Hooks::ext_stmt`) and gives the impact diff with its base. It says that `reports/clean-room/X24/tests/` does not exist yet.
- **Generic node or typed nodes.** Section 2 has a new part, "What analysis reads": analysis reads the parser's generic `syntax::Node`, each page's `check.rs` reads the nodes of its own productions by production name, and the typed tree of `sem` is the only typed tree. The data sketch changed to match. In step 3, analysis runs after the parse of the whole unit with the step 2 oracle (section 10).
- **The takeover list fails the check.** With `--next`, a row whose test does not exist, a row of the wrong form, a test listed twice, and an owned test whose new status is not a passing status each fail the check (exit 1). A test of `read_takeover` on a list of three bad rows gave the three problems.
- **The parse count lists every grammar gap.** `tidal-next parse --all` adds a recovery mode to the parser (`Parser::item`): a statement, a member, or a declaration that fails is recorded and skipped, and the parse goes on. The first stop stays the normal parse's stop. The tool prints each other stop after the first ("also stops at LINE: why").
- Without `--next` the tool prints what it printed before.

### Item RW2, build step 3

| Part | State | Where |
|---|---|---|
| a. The tree that analysis reads | settled (generic node) | `DESIGN-rewrite.md` section 2 |
| b. The node-check point | in place; no design uses it yet | `sem::check::NodeCheck`, run by `expressions::check::expr` |
| c. `sem` | the type table without `DepType`, the typed tree without marks, entities, scopes, `Unit`, the `Check` context | `sem/types.rs`, `sem/tree.rs`, `sem/mod.rs`, `sem/check.rs` |
| d. `target`, `machine`, `conform` | `target` from TYP-1 and TYP-2; a new machine for the step 3 constructs that names no rule and calls the `run.rs` parts; the harness with the old statuses and output | `target.rs`, `machine/`, `conform/mod.rs` |
| e. Constant evaluation | on the machine with its limits (CE-2 to CE-30) | `foundations/constant_evaluation/check.rs`, `run.rs` |
| f. The pages' `check.rs` and `run.rs` parts | 48 `check.rs` parts on pages, 3 chapter dispatchers, and 20 `run.rs` parts on the foundations, expressions, statements, and declarations pages | `frontend/next/src/` |
| g. The overloading pages for non-class operands | standard sequences, reference bindings, ranks and tie-breaks, the best viable function | `overloading/conversion_sequences/check.rs`, `overloading/overload_resolution/check.rs` |
| h. The `Ext` node point | in place, with P22 as its user | `sem::check::Hooks::ext_stmt`, called by `statements::check` |

`tidal-next check`, `run`, and `conform` now give analysis, a run of a program of one unit, and the harness. The module `target` is the file `src/target.rs`, because `frontend/.gitignore` ignores every directory named `target/`.

### What step 3 does not do yet

Of the 292 step 3 plain tests whose check the old front end passes and the new one does not, the causes are:

| Cause | Tests |
|---|---|
| A lambda expression (its closure is a class) | 67 |
| A missing check: a reject test that the new front end accepts | 122 |
| A structured binding declaration | 13 |
| `if consteval` | 12 |
| `EDG-AGREES` (a missing check where EDG agrees) | 12 |
| A braced list bound to a reference to an array | 9 |
| A `noexcept` specifier with an operand, and the `noexcept` operator | 13 |
| An assumption `[[assume(e)]]` (its tokens are no expression tree) | 6 |
| `long double` arithmetic at the machine's double precision | 5 |
| Other unsupported constructs | 32 |
| A false reject: `expressions/primary/char-named-escape.cpp` (the named escape needs a Unicode name table, G158 to G160) | 1 |

Of the 1602 tests whose check passes, about 85 do not run to their result: 45 programs of several translation units (linking has not moved), 22 other run stops (`reinterpret_cast` addresses, calls of functions with no definition, `long double`), 8 missed stops at undefined behavior (the byte view, SEQ-4 at run time, enumeration ranges), 7 unexpected stops, and 3 wrong results.

## 3. Takeover

`reports/clean-room/rust-frontend/takeover.tsv` holds 1327 rows, each with build step 3 and the commit cell `RW2`, which the integrator can replace with the item's commit. A row is a test that:

- cites only rules of step 3 pages and uses no keyword of a later step (the step 3 plain scope, 1890 tests at `cb40b18b`),
- passes on both front ends with a passing status,
- for a reject test, has a first error on the new front end that names a cited rule or a rule that a cited rule links to. At `cb40b18b`, 185 passing reject tests whose first error names another rule stayed with the old front end.

The rows were chosen at `cb40b18b`. At `73ed0807` no owned test fails on the new front end, and none of the 42 test files that changed between the two commits is an owned test, so no row was removed. The three tests of RV10 that pass on the new front end only (section 1) do not join, by the third condition.

The largest groups are `declarations/initialization`, `expressions/primary`, `foundations/keywords`, `foundations/names`, `foundations/constant-evaluation`, `foundations/types`, `declarations/declarators`, `declarations/functions`, and `foundations/translation`, with 47 other page directories after them. The list is the file itself.

None of the 10 candidates of implementer.md item 2 (tests the old front end fails) passes on the new front end: 7 use a class, a template, a pack, or a lambda, and the CE-3 and UNEV-1 tests need constructs of later steps.

## 4. The P22 prototype

`frontend/next/src/proposals/p22.rs` and its registry line in `proposals/mod.rs`. It adds the alternatives `"break" identifier ";"` and `"continue" identifier ";"` at `jump_stmt`, and checks the node at the `Ext` node point: the label must mark an enclosing loop or `switch` in the jump's own function (X24:new-1), and the label of a `continue` must mark a loop (X24:new-2). The check gives an ordinary `Break` or `Continue`, and X24:new-3 runs on the unchanged machine.

The impact diff: `tidal-next conform tests -v` with and without `--proposal P22`. The base (tests that pass without the flag) is 1819 of 7148 tests and 1 of the 178 program tests. Exactly two tests change, `statements/jumps/break-with-label.cpp` and `continue-with-label.cpp`, from `pass` to `MISSED-REJECT`, which is the change of X24's table (reject to accept). Run with a `main` added, their `f()` gives 5 and 6, as X24 section 2 says. No program changes in the base, so X24's "no program changes" holds as a lower bound.

The unit tests of `p22.rs` hold X24's examples: the forms are a TR-12 stop without the flag, `find` of section 1.1 is a constant, a labeled `continue` goes on with the outer loop (status 6), the chain of section 1.3 is accepted, and a `continue` to a `switch`, a label on a sibling loop, a label on a block, and an unknown label give X24:new-2 or X24:new-1. The 17 new tests and 2 changed ones of X24 section 2 have no directory beside the report yet.

## 5. Faults with no test

None found.

## 6. Findings

No finding against the text. Each false reject that the shadow run showed came from the new front end's own code, and the text gave the answer (SA-3 on narrowing in a static assertion, NAM-23 on the region of a using-directive, NAM-13 on the order of a qualified lookup, ENUM-13 on the type of an enumerator inside its list, LSP-7 and LSP-8 on C names, CMP-26 on the end of a row, JMP-10 on a returned parameter).

`DESIGN-rewrite.md` section 10 gains two open points: analysis runs after the parse in step 3, and 64 rule IDs in the `check.rs` and `run.rs` parts belong to another page by the prefixes of section 3, mostly the rule of a context that a caller passes, such as INIT-7 in a return statement. Section 9 asks a round to report that count.

## 7. Ledger rows

None. No row from G190 is needed.

## 8. Tests added during the round

None. The unit tests of `frontend/next/` are not conformance tests.
