# Roadmap, round 11: what to do after phase 13, from an implementer's view

Date: 2026-11-11. Decision 0029. Role: clean-room reader, acting as an implementer who builds a C++ front end and interpreter from Tidal alone. The evidence is the clean-room front end in `frontend/`, its reports and ledger in `reports/clean-room/rust-frontend/`, and the phase 13 exercise records. Commit read: `3bd790ae`.

Inputs:
- AGENTS.md, CLAUDE.md
- `reports/clean-room/roadmap-10/implementer.md`, the model for this report's form, and its scratch files
- `documentation/`: status, roadmap (the phase 13 table), decisions 0060 and 0061, and `decisions/pending.md` (the table, the request, the packet, and the row 4 items)
- `git log c67b349b..3bd790ae` with every commit body (17 commits)
- `reports/clean-room/rust-frontend/`: report 24 in full, `ledger.md`, `conformance.tsv` at `a15a877d` (round 22), `1017b19c` (round 23), and `3bd790ae` (round 24)
- `reports/clean-room/I6/report.md` and `reports/clean-room/P2/report.md`
- `reports/clean-room/ledger.md` (the 26 open rows and the RV6 rows)
- the two phase 13 gap reviews in `reports/review/` (the Result sections and the rule tables)
- `specification/`: the `productionlist` blocks of every page (by script) and the rule kinds of every page
- `questions/`: the status lines of Q-0424 to Q-0427
- `tests/`: the `tidal-rules` lines of the failing tests
- `frontend/src/` only by line counts and a grep for rule IDs

I did not read the other round 11 reports, `inputs/`, or `build/exercise-keys/`.

**Measurements.** I built the front end with `cargo build --release --locked --target-dir` into my session scratch directory and ran `tidal-fe conform tests -v` once under the run lock. I ran `tools/frontend_check.py` once under the run lock. Its lines match the phase 13 close commit:

```
frontend: 6721/6881 pass, 6745/6881 pass their check, 0 worse, 0 better, 0 moved between passing statuses, 3 new (0 not passing)
frontend: completion (decision 0060): 127 tests fail their check, 2/126 program tests do not run to their result, 9 tests out of scope
frontend: 2087/2709 passing rejects name a cited rule (1902) or a linked rule (185) in their first error
```

The failing rows, two counting scripts, and their output are in `reports/clean-room/roadmap-11/implementer-scratch/`. `counts.py` gives the counts by status, chapter, rule class, and message, and compares the failures with the round 22 and round 23 baselines. `causes.py` sorts the 127 tests of the completion line by cause. The front end prints nothing for a test that it accepts, so the cause of a missed reject is my judgment from the test's rules and report 24. Statements from general knowledge say so.

## 1. Where the specification and the front end stand

| Measure | Round 10 (6790 tests, `ebc109b9`) | Now (6881 tests, `3bd790ae`) |
|---|---|---|
| Full pass | 6586 (97.0%) | 6721 (97.7%) |
| Check pass | 6626 | 6745 |
| Completion line (decision 0060) | not printed; 164 by hand | 127 tests, 2 of 126 program tests, 9 out of scope |
| `MISSED-REJECT` | 100 | 69, and 6 that pass as `NO-DIAGNOSTIC-ALLOWED` |
| `unsupported` (check) | 58 | 57 (51 in scope) |
| `run-unsupported` | 40 | 24 |
| `FALSE-REJECT` | 4 | 8 (5 in scope) |
| `EDG-AGREES` (a failure) | 2 (CPY-27, CNS-32) | 2, the same |
| Run tests | 1756: 1711 pass, 0 wrong, 40 unsupported | 1796: 1761 pass (18 partial), 0 wrong, 24 unsupported, 11 not run because the check fails |
| Program tests under `tests/programs/` | 115, all run | 126; 124 run |
| Passing rejects whose first error names a cited or linked rule | 2030 of 2647 (77%) | 2087 of 2709 (77%) |
| `reject` rules absent from the front end's source | 101 of 559 | 95 of 559 |
| `unspecified` rules named in the front end's source | 14 of 32 | 14 of 32 |
| Rules with a run test and no passing run test | 11 | 10 |
| Rules | 1698 | 1698 |
| Questions open | 176 | 180 |
| Front-end findings (G) | G1 to G157, 5 open | G1 to G163, 2 open (G161, G162) |
| Exercise ledger rows open | 17 | 26 (I6-1 to I6-10, I6-13 to I6-20, P2-1 to P2-4, P2-6 to P2-9) |
| Front-end source | 50673 lines, 28 files | 51375 lines, 28 files; round 24 changed 11 files (+761, -59) |

### What phase 13 took from round 10

| Round 10 item | Phase 13 result |
|---|---|
| 1. Tests for every front-end target without one | Done in T2 (`afccbb13`). Round 24 passes every one. |
| 2. Front-end round 24, in five groups | Done (`53577f27`). Groups 1 to 4 were done. Group 5, the template missed rejects, was not targeted. The round passed 32 old failures and 82 of 92 new tests. |
| 3. The completion test made countable | Done in CT (`2e588e25`). `frontend_check.py` prints the line. G42 and G158 to G160 hold the 9 out-of-scope tests. 6 tests pass as `NO-DIAGNOSTIC-ALLOWED`. The two EXS-6 cycle tests stay `unsupported`, because the front end stops on them. |
| 4. Text for the 17 open rows and G153 to G157 | Done in F3 (`0ae4cdb5`). |
| 5. Implement I6 | Done (`e2592575`): 352 of 358 assertions, findings I6-1 to I6-20, two front-end faults with tests. Round 24 fixed both (I6-11, I6-12). |
| 6. Preparation for the rewrite that touches no code | Deferred by decision 0061. |
| 7. An amendment to decision 0060 | Deferred by decision 0061. |

Of round 10's process changes, phase 13 took change 1 (item R7 added a check that fails on a front-end row with no test, `45d5b9bd`), 2 and 3 (item CT), 4 (round 24 group 2 names the first unknown construct), 5 (report 24 section 9 gives the file sizes and the first-error rate), and 7. Change 6 is partly in place. I6's inputs named the linked rules, and I6 still took the overload ranking, the lookup scope, and the operator arities from outside knowledge (I6-17, I6-20).

### The 127 tests of the completion line

117 of the 127 failed at round 22 and at round 23 with the same status. The other 10 are tests that phase 13 added. No test that the front end passed at round 23 fails now. So the old set has only shrunk over two rounds, and each new failure is a new test.

| Measure | Round 22 | Round 23 | Round 24 |
|---|---|---|---|
| In-scope tests that fail their check | 167 | 149 | 127 |
| Old failures passed in the round | | 18 | 32 |
| New tests in the round, and how many fail | | 105, 0 | 92, 10 |
| `run-unsupported` | | 40 | 24 |

By status and chapter:

| Chapter | Missed reject | Unsupported | False reject | EDG-AGREES | All |
|---|---|---|---|---|---|
| templates | 33 | 20 | 3 | 1 | 57 |
| classes | 10 | 7 | 0 | 1 | 18 |
| expressions | 7 | 8 | 0 | 0 | 15 |
| declarations | 6 | 6 | 1 | 0 | 13 |
| overloading | 4 | 7 | 0 | 0 | 11 |
| foundations | 4 | 2 | 0 | 0 | 6 |
| statements | 3 | 0 | 0 | 0 | 3 |
| exceptions | 2 | 0 | 0 | 0 | 2 |
| programs | 0 | 1 | 1 | 0 | 2 |
| **total** | 69 | 51 | 5 | 2 | 127 |

By the class of the rules each test cites:

| Rule class of the test | Tests | What a front end must do |
|---|---|---|
| `reject`, alone or with others | 60 missed, 5 unsupported, 4 false rejects, 2 EDG-AGREES | a diagnostic, or the base answer |
| `required` only | 40 unsupported, 1 false reject | the right answer |
| `recommended`, with `-Werror` | 7 missed | a warning (VIRT-13, REF-15 twice, TRY-13, UNEV-4 twice, UN-19) |
| `impl-defined` with `required` | 4 unsupported | the layout (BF-9) |
| `no-diagnostic` with another class | 1 missed, 2 unsupported | DEP-30 with DEP-15; the two EXS-6 cycles stop |
| `undefined` | 1 missed | constant evaluation (CE-3) |

### The causes

`causes.py` puts each test in the first group that fits:

| Cause | Tests | Rules |
|---|---|---|
| The front end's model of a specialization and of the current instantiation | 16 | DEP-24 (4), DEP-15 with DEP-1 or DEP-30 (2), DEP-11, DED-24, DED-29 (2), CNS-29 (2), PACK-8 (2), IF-22, TSP-31 |
| A program test whose cause is open | 1 | `r10-list-first-try.cpp`: TR-13 nesting in `cond<0, const Node*, Node*>` |
| Class template argument deduction | 11 | CTAD-1 (3), CTAD-2 (2), CTAD-9, CTAD-12, CTAD-19, CTAD-29 with CTAD-11, TPL-11, and `r10-tuple-second-try.cpp` |
| Missed rejects in the templates chapter, outside the model | 20 | TARG-24 (2), DEP-37 (2), INST-12 (2), CNS-24 to CNS-27 (2), TARG-7, TARG-15, CPT-37, CNS-13, TPL-7, TPL-31, INST-15, INST-21, INST-33, ORD-3, PACK-26, TSP-23 |
| Missed rejects in other chapters | 26 | OVL-9 (2), DCMP-3, DCMP-11, CTOR-12, CTOR-21, CTOR-22, CVF-8, CVF-9, CPY-15, VIRT-5, ALN-2, CEV-1, DEFARG-4, UNW-11, UNEV-1, LOG-3, PFX-11, PRIM-26, CE-3, LSUP-1, NAM-8, ICS-10, OVL-20, AUTO-21, RFOR-7 |
| `recommended` warnings | 7 | VIRT-13, REF-15 (2), TRY-13, UNEV-4 (2), UN-19 |
| A concept in a `static_assert` | 8 | ICS-45 (5), CPT-16 (2), CPT-20 |
| BF-9 layout of wide bit-fields | 4 | BF-9 (one on ILP32) |
| The `noexcept` operator | 4 | CLS-2, EXS-1, EXS-6 (2) |
| A `new` expression in a constant | 4 | NEW-1, NEW-2 (3) |
| The width of the size type | 2 | DCL-19 on ILP32, STG-1 |
| Other unsupported constructs, one or two each | 21 | DCL-5 and CVF-1 (a class bound), LIST-4 and UNI-7 (brace elision in a constant), LAM-6 and LAM-18 (a lambda callee), AUTO-7, COND-3, PRIM-27, CE-3, LSUP-13, ICS-47, OVL-40, TARG-14, CPT-1, CNS-30, DED-24, DEP-20, INST-30, PACK-3, PACK-5 |
| Other false rejects | 1 | SB-5 (`tuple-member-get-type-parameter-uses-free.cpp`, the front end gives DED-35) |
| EDG-AGREES | 2 | CPY-27, CNS-32 |
| **total** | 127 | |

The model group grows. Report 23 section 3 named 9 template failures that come from the model, and 8 of them still fail (DEP-30 now passes as `NO-DIAGNOSTIC-ALLOWED`). Phase 13 added 7 more through T2 and RV6: the four DEP-24 tests of Q-0427, the two DEP-15 tests, and `substitution-member-wrong-kind.cpp`. Report 24 section 6 calls them rewrite work. Every other group shrank in round 24.

Five of the 8 concept tests cite ICS-45, and two model tests cite DEP-15 with DEP-P4. Both are rule items of `pending.md` row 4a. Q-0427 (DEP-24) waits for a review with a doubt. So 11 of the 127 tests rest on answers that can still change.

### Run stops

The 24 run stops pass their check, so the completion line leaves them out. They are a `new` expression (4: LAM-19, NEW-24, DED-39 twice with ORD-9), an overload set (3: CVF-1, CVF-4, CVF-5), a missing default argument (2: NEW-23, TSP-8), a dependent name (2: DEP-20, DEP-24 with DEP-25), an unknown callee (2: TARG-12, DEP-11), a destructor with a function try block (2: TRY-10), a variable with no definition in the unit (2: SINIT-1, ODR-15), and 7 single causes (DCMP-4, CVF-1, VIRT-12, LNK-9, UN-7, OVL-42, INST-36).

The 10 rules with no passing run test are CTAD-11, CTAD-29, DEP-20, DEP-25, LAM-19, ORD-9, OVL-42, PFX-22 (out of scope, G158), TARG-12, and TSP-8. Round 24 closed IF-12, JMP-16, and SB-12. CTAD-11 and CTAD-29 are new, through the T2 test of a user guide beside a constrained constructor template.

### Decision 0060: how far the front end is from the completion test

The completion line counts 127 tests and 2 program tests. The second condition fails on two program tests of phase 13: `r10-list-first-try.cpp` (TR-13 nesting, no reduction yet) and `r10-tuple-second-try.cpp` (class template argument deduction).

Decision 0060 lets a roadmap propose the rewrite early "if the remaining failures are few and stable over two rounds". Rounds 23 and 24 are the two measurements.

- **Stable.** The condition holds. 117 tests have kept the same status since round 22, and no test moved from pass to fail in two rounds.
- **Few.** The condition does not hold for the whole set. Round 24 passed 32 old failures, and round 23 passed 18. At those rates the 99 tests outside the model and the deduction group need 3 to 6 rounds, and the model group grows by the tests each phase writes on templates.

The two conditions hold for the 17 tests of the model group and the open program test. Report 24 does not try them, and each costs more in the current template model than in a new one. So this report proposes the rewrite early in two steps (items 3 and 5). Phase 14 prepares it with work that touches no code and runs round 25 on the tests outside the model. The decision after round 25 starts the rewrite if round 25 leaves at most 50 in-scope tests and names the cause of each.

### What the rewrite needs first

Decision 0060 lists a design document, modules by chapter and page, a parser from the Syntax segments, unit tests per stage, and short files. The tree now gives these facts:

- **The template model.** The rewrite has to choose it before it writes `templ.rs` again. The current front end parses a template's tokens once as a pattern and again with the arguments bound (`frontend/README.md`, section "Templates"). In a pattern, the type of an object of the current instantiation is unknown (report 24 section 6), so the front end cannot tell where DEP-24 finds a member template. I6 gives a starting point. Its component answers dependence (DEP-1 to DEP-4), the current instantiation (DEP-15), binding (DEP-5 to DEP-18), the keywords (DEP-16 to DEP-27), and the checks before instantiation (DEP-28 to DEP-33) with 352 of 358 assertions right. Its report also names three facts that the caller had to supply (whether some argument list makes a construct valid, what a qualified name finds for given arguments, and the members of the independent bases). A typed dependent form has to compute those three facts. I6-4 adds that the first phase only chooses a reading and the second reports the error.
- **Design document.** `frontend/README.md` (458 lines) still describes use, layout, and some procedures. No document names the data that passes between the stages.
- **The parser from the Syntax segments.** The two gaps of round 10 remain. The grammar has 255 nonterminals on 62 pages and no production for a translation unit. 17 nonterminals are used by no other production: `anonymous_union`, `auto_var_decl`, `conversion_decl`, `copy_assign`, `copy_ctor`, `ctor_def`, `defaulted_comparison`, `dtor_call`, `dtor_decl`, `forward_class`, `guarded_header`, `include_directive`, `inheriting_decl`, `move_assign`, `move_ctor`, `pragma_once`, and `typedef_decl`. Most describe the shape of a declaration that a general production derives.
- **Modules by page and short files.** Seven files exceed decision 0060's limit of about 2000 lines: `expr.rs` 7726, `decl2.rs` 7190, `interp/expr.rs` 4815, `templ.rs` 4310, `decl.rs` 3154, `expr2.rs` 2829, `parser.rs` 2396. Round 24 added 702 lines net, and each of the seven grew.
- **Unit tests per stage.** None exist.
- **The first-error rate.** It stays at 77% for a fifth phase. 621 passing rejects name another rule first. No round has targeted it, and the rewrite needs a target for it.

### The I6 findings

I6 is the first blind build of the dependent-names page. Its 18 open rows sort by what the front end needs:

| Rows | Who acts | Why the front end cares |
|---|---|---|
| I6-5 (DEP-2), I6-6 (DEP-4) | text, with a test | Type and value dependence decide where a name binds. I6-5 asks for a test with `f(this->n)` and a later `f(int)`. No test decides either reading. |
| I6-1 (DEP-5, DEP-P1), I6-9 (DEP-8, DEP-9, DEP-14), I6-10 (DEP-9, INST-31) | text | The lookup of an independent call, a qualified dependent call, and a member call with a dependent argument. The front end already follows the compilers on the programs. A template model needs the rule before code. |
| I6-4 (DEP-P5), I6-7 (DEP-6, DEP-12, DEP-30), I6-8 (CNS-13, DEP-P6) | text | Which phase reports an error, and whether a check before instantiation is `reject` or `no-diagnostic`. The completion line counts a `no-diagnostic` test as a pass when the front end accepts, so these classes change the count. |
| I6-13 (INST-1, INST-2), I6-14 (DED-29, INST-6) | text, with tests | I6 programs p13 and p14 agree on GCC, Clang, and the front end, and neither is a test. I6-14 is I5 notes item 23 and needs one example. |
| I6-15 (INST-5, INST-33), I6-16 (INST-P2) | text | When a `static constexpr` initializer and a friend body are made. `class-point-not-later-type.cpp` (INST-33) is a missed reject now. |
| I6-3, I6-17, I6-18, I6-19 | text | An example or a link each. I6-17 (operator arities for DEP-37) bears on the two DEP-37 missed rejects. |
| I6-2, I6-20 | none, or a prompt change | I6-2 is a fault in the component. I6-20 lists outside knowledge, which process change 4 below answers. |

### P2, G161, G162, and the rules changed after RV6

P2's three programs ran to result 0 on the front end on the first try. P2-5 is a probe on which the front end stops on OBJ-2, as the text says. P2-9 is the front end's own output: the first word is "unspecified" when every explored order gives one result. Rounds 7 to 10 raised the same point, and decision 0061 deferred it. P2-1 to P2-4 and P2-6 to P2-8 ask for examples and links, and no front-end work follows from them.

G161 (the loops page and IF-12, SW-1, JMP-6) and G162 (LSUP-6 and `std::nothrow`) are text rows. The front end already made a choice for each, and each has two tests that pass. If the text chooses differently, the tests change and the front end follows.

Of the 22 rules and principles that changed after RV6 read the tree, three are cited by tests that fail on the front end now. CPY-15 has one missed reject (`class-template-ctor-by-value.cpp`). DED-24 has two unsupported tests. DEP-24 has four failing tests and one run stop. The next review's reading of DEP-24 and DED-24 decides part of the template model's target.

## 2. Ranked items

| Rank | Item | Evidence | Size | Prerequisites |
|---|---|---|---|---|
| 1 | Front-end round 25 on the tests outside the template model | 99 in-scope tests outside the model and deduction groups; 24 run stops; P2-9 | large | none |
| 2 | Text for the I6 rows, G161, and G162, with their tests | 18 open I6 rows, 2 open G rows; tests that I6-5, I6-13, and I6-14 ask for | small each | none |
| 3 | Rewrite preparation: the template model and the design document | 16 tests in the model group (8 of report 23's 9, and 7 from phase 13); I6's component and its three supplied facts | medium | 2 for I6-4, I6-5, I6-9 |
| 4 | The grammar made usable by a parser | no start symbol; 17 shape productions | small | none |
| 5 | An amendment to decision 0060, and the gate for an early start | stable over two rounds; 127 in scope; the model group grows | small | 1 for the count |
| 6 | Implement I7: class template argument deduction | 11 failures, 2 rules with no passing run test, no blind build | medium | none |
| 7 | A review of the 22 changed rules with the I6 text, DEP-24 first | DEP-24, DED-24, CPY-15 have failing tests; Q-0427 waits for a review | medium | 2 |

### 1. Front-end round 25 on the tests outside the template model

**What it is.** A round in the form of report 24, with the completion section kept. The groups, in order:

1. **The program tests.** A reduction of `r10-list-first-try.cpp` (TR-13 nesting through a constrained converting constructor). If the reduction shows a model cause, the round says so and the test joins the model group.
2. **Unsupported groups with several tests.** A concept in a `static_assert` (8: ICS-45 five times, CPT-16 twice, CPT-20), BF-9 layout (4, one on ILP32), the `noexcept` operator on a later member (4, CLS-2, EXS-1, and the two EXS-6 cycles, which pass as `NO-DIAGNOSTIC-ALLOWED` once the front end accepts them), a `new` expression in a constant (4: NEW-1, NEW-2), a class as an array bound (2: DCL-5, CVF-1), and brace elision in a constant (2: LIST-4, UNI-7).
3. **Missed rejects outside the templates chapter (26).** OVL-9 (2) first, then the single rules. The DCMP, CTOR, and CVF pages each hold two or three.
4. **Template missed rejects outside the model (20).** TARG-24 (2), DEP-37 (2, after I6-17's link), INST-12 (2), CNS-24 to CNS-27 (2), then the single rules.
5. **Run stops (24).** The overload set in a run (CVF-1, CVF-4, CVF-5), the missing default argument (NEW-23, TSP-8), the destructor function try block (TRY-10, two tests), the `new` forms (LAM-19, DED-39, ORD-9, NEW-24), the class assignment (OVL-42), and the unknown callee (TARG-12). These free 5 of the 9 in-scope rules with no passing run test (LAM-19, ORD-9, OVL-42, TARG-12, TSP-8).
6. **The 7 `recommended` warnings, the size type width (2), the SB-5 false reject, and the single unsupported constructs.**
7. **P2-9.** The interpreter's first line says "result 0 in every explored order" when every order agrees, and `programs/P2/ledger.cpp` shows the form.

Class template argument deduction (11) waits for item 6. The model group (16) waits for item 3.

**Evidence.** Section 1. Round 24 passed 32 old failures, every failure of the round predates it or is a new test, and report 24 groups 1 to 4 left these tests untried.

**Size.** Large. Groups 2 and 5 are medium. The others are small to medium.

**Prerequisites.** None. Item 2's tests join when they land.

**Risk.** Low. A group the front end cannot close from the text becomes a G row with a test.

### 2. Text for the I6 rows, G161, and G162, with their tests

**What it is.** The clean-room writer acts on each row, in this order:
1. **I6-5 and I6-6.** DEP-2 says when a member access through `this` or an object of the current instantiation is type-dependent. DEP-4 says whether `sizeof`, `alignof`, and `noexcept` of a value-dependent operand are value-dependent. Each gets a test that decides it, probed on the matrix.
2. **I6-1, I6-9, I6-10.** DEP-5 and the DEP-P1 table give the independent call's argument-dependent lookup. DEP-9 gives the qualified dependent call and the call whose name finds a member. DEP-9 points to INST-31 for the point of instantiation.
3. **I6-4, I6-7, I6-8.** DEP-P5 says the first phase chooses a reading and the second reports. DEP-6 and DEP-12 say whether DEP-30 applies. CNS-13 and the DEP-P6 row agree on one class.
4. **I6-13 and I6-14.** INST-1 and INST-2 name the call by value and the call in `decltype`. A DED-29 example shows the hard error that stands against a fallback. I6 programs p13 and p14 become tests.
5. **I6-15, I6-16.** INST-5 names INST-33's exception, and the INST-P2 table gains a row for a friend body.
6. **I6-3, I6-17, I6-18, I6-19.** A DEP-P5 row, a link from DEP-37 to the operator parameter counts, two comments on the keywords example, and the return type in DEP-20.
7. **G161.** The loops page says how a loop goes on after a jump into its body, and what JMP-6 makes of a skipped init statement.
8. **G162.** LSUP-6 says the library defines `std::nothrow`.
9. **P2-6 and P2-8.** LSUP-1 on the constructor's access and the spelling of the count type; a link to CE-3 from the array bound rule. The other P2 rows are examples, which the programmer view ranks.

**Evidence.** Section 1, "The I6 findings".

**Size.** Small each. I6-5, I6-6, and I6-9 may need a probe and a question.

**Prerequisites.** None.

### 3. Rewrite preparation: the template model and the design document

**What it is.** Two documents in `frontend/`, written by a clean-room writer from Tidal's files and the current front end, with no code change.
- **The template model.** It says how the rewrite represents a template definition: what a dependent type and a dependent expression hold, how the current instantiation is known inside a pattern, and how the second phase checks a name against the arguments. It answers the three facts that I6's caller supplied (validity under some arguments, what a qualified name finds, the members of the independent bases). It lists the 16 model tests and says how the new form passes each. I6's `depnames.py` is an input as a reading of the rules.
- **The design document of the current stages.** The stages, the data each passes (tokens, entities and scopes, the template record of tokens, the interpreter's values and places), and where each chapter's rules sit, with a module map by page for the rewrite.

**Evidence.** Section 1, "What the rewrite needs first". 8 of the 9 model failures of report 23 remain, phase 13 added 7, and report 24 names these tests as rewrite work. Decision 0061 deferred this preparation once.

**Size.** Medium.

**Prerequisites.** Item 2 for I6-4, I6-5, and I6-9, which the model has to follow. Item 7's reading of DEP-24 helps and is not needed to start.

### 4. The grammar made usable by a parser

**What it is.** A production for a translation unit. A mark on each of the 17 shape productions, or a move of each into prose. `check_specification.py` then fails on a nonterminal that the start symbol does not reach and that has no mark.

**Evidence.** Section 1. Decision 0060 asks for a parser from the Syntax segments, and the gaps of round 10 remain.

**Size.** Small.

**Prerequisites.** None. The change touches no rule.

### 5. An amendment to decision 0060, and the gate for an early start

**What it is.** The phase 14 decision states three things.
- "Few" means at most 50 in-scope tests on the completion line.
- A front-end round can name a test whose cause is the current template model. A front-end ledger row in state `rewrite` names the tests and the cause, and the completion line prints them as a third count.
- The phase decision after round 25 adds the rewrite as an item if round 25 leaves at most 50 in-scope tests, the set is stable against round 24, and each test has a cause. The rewrite then takes the `rewrite` rows as its first targets.

**Evidence.** Section 1. The set is stable over rounds 23 and 24. It is not few at 127. Round 25 at round 24's rate leaves about 95, and the model group grows each phase. A count that the old code cannot lower in a fair time keeps the rewrite waiting.

**Size.** Small. `frontend_check.py` reads one more state.

**Prerequisites.** Item 1 for the count.

**Alternative.** Start the rewrite in phase 14 with the lexer and the parser, beside the old code under 0 worse. I rank the gate higher, because the template model (item 3) and the grammar (item 4) are the rewrite's first inputs and neither exists.

### 6. Implement I7: class template argument deduction

**What it is.** A fresh clean-room reader builds a component from `templates/class-template-deduction.md` (CTAD-P1 and its rules), filtered of Evidence segments and `:tests:` lines as in I4 to I6. The component takes a class template, its constructors, its guides, and an initializer, and returns the deduced arguments or the failure. I5's deduction serves as its callback. The prompt gives the callbacks' answers for overload resolution and ranking (process change 4). The scoring phase uses the 118 tests of the page.

**Evidence.** Class template argument deduction holds 11 of the 127 tests, among them `r10-tuple-second-try.cpp`, and CTAD-11 and CTAD-29 have no passing run test. No blind build has read the page. I1 to I6 each found more than a dozen unstated steps, and I5 and I6 each found front-end faults through programs.

**Size.** Medium.

**Prerequisites.** None.

**Alternative.** Constraint normalization and subsumption (CNS-P1, 12 failures on two pages in round 10, now 7 on the constraints page and 5 on the concepts page). I rank deduction first because a program test depends on it.

### 7. A review of the 22 changed rules with the I6 text, DEP-24 first

**What it is.** The next source-reader gap review covers the 22 rules and principles that changed after RV6, and item 2's text. It reads DEP-24 and DED-24 first and gives Q-0427 its review.

**Evidence.** Section 1. Four DEP-24 tests, two DED-24 tests, and one CPY-15 test fail on the front end, and the template model of item 3 has to pass them.

**Size.** Medium.

**Prerequisites.** Item 2.

### Dependency sketch

```
2 text (I6-4, I6-5, I6-9) ──> 3 template model ──> 5 gate after round 25
1 round 25 ───────────────────────────────────────> 5 gate (the count)
4 grammar (independent)
6 Implement I7 ──> round 26 (deduction group)
2 text ──> 7 review (DEP-24, DED-24) ──> 3 template model (revision)
```

Items 1, 2, 4, and 6 can start together. Item 3 starts after item 2's first three steps.

## 3. Scope

| Topic | Charter's reason to keep out | Implementer's view | Proposal |
|---|---|---|---|
| `<cstdarg>`, named escapes, `#include <...>`, extended floating-point types | the library, a name table, real headers, *(referenced)* | G42 and G158 to G160 hold 9 tests; the completion line leaves them out | Keep the rows |
| ILP32 | in the matrix (EDG ILP32) | 2 tests remain (DCL-19 4 GiB array, BF-9 layout); both ADD-18 ILP32 tests pass now | Keep in scope |
| `no-diagnostic` rules | the class needs no diagnostic | 6 tests pass as `NO-DIAGNOSTIC-ALLOWED`; the two EXS-6 cycles pass once the front end accepts them | Keep the status |
| A program with no library (P2-1) | the library is out of scope | The front end runs P2's hand-written containers on the first try; examples cost nothing in the front end | Examples only |
| Coroutines, modules, threads, the memory model | outside the language goal | No test or front-end path needs them | Keep out |
| Later language versions (`pending.md` row 7) | the charter fixes C++23 | The front end's work would not change | Keep out |
| MSVC | not in the matrix | A second ABI would double the layout tables | Keep out |
| Propose designs (P23, P25) | not part of the rules | Deferred again by decision 0061 | Keep out until the rewrite |

## 4. Process changes

1. **The completion line gives a third count.** Tests that a `rewrite` row names leave the in-scope count, as `out of scope` rows do, and the line prints them apart (item 5).
2. **Each round keeps a cause file.** The front end prints nothing for a test it accepts, so each roadmap sorts the missed rejects by judgment. A round report should write a file of the remaining tests with a cause group each, as `causes.py` does here, and the next round updates it.
3. **The first-error rate becomes a target of the rewrite.** The rate has stayed at 77% for five phases with no round that targets it. The design document of item 3 sets the rewrite's target, and the old code stops reporting the rate per page.
4. **Implement prompts give the callbacks' answers.** I6 took the overload ranking, the lookup scope, and the operator arities from outside knowledge (I6-17, I6-20) although its prompt named the linked rules. The prompt should give the answers that the component takes as callbacks, or the pages with their rules.
5. **An Implement program that agrees on every tool becomes a test when no test holds its case.** I6's p13 and p14 decide INST-1 cases that no test covers, and they are not tests.
6. **Keep the front-end report form.** Round 25 keeps report 24's tables by group, its completion section, and its measures for the rewrite, and adds the cause file.
