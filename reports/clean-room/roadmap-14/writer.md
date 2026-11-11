# Roadmap, round 14: what to do after phase 16, from a clean-room writer's view

Date: 2026-11-11. Decision 0029, with decisions 0033, 0039, 0050, 0051, 0054, 0055, 0056, 0057, 0058, 0059, 0060, 0061, 0062, 0063, and 0064. A clean-room agent in the role of a clean-room writer, who drafts rules, Evidence, and tests from compiler behavior, prepared this report. It read AGENTS.md, CLAUDE.md, the round 13 writer report `reports/clean-room/roadmap-13/writer.md` with its probes and scripts, `documentation/status.md`, `documentation/roadmap.md` (the phase 16 table), decisions 0060 to 0064, `documentation/decisions/pending.md`, `git log b73a7d2d..f77bbe05` with every commit body, the phase 16 reports (`reports/clean-room/X24/report.md` with its probes, `rust-frontend/report-rw1.md`, `rust-frontend/report-27.md`), the RV9 review and the read of its repair diff in `reports/review/` dated 2026-11-11, the question files Q-0447 to Q-0449, `reports/clean-room/ledger.md`, `reports/clean-room/rust-frontend/ledger.md`, and the rules, tests, and Syntax segments that its items name. It did not read the other round 14 reports, `inputs/`, `build/exercise-keys/`, or the sources under `frontend/`. Counts come from read-only `git`, `grep`, and the scripts in `writer-scratch/` beside this report (`changed.py`, `counts.py`, `rule.py`, `waits.py`, and `doubt_only.py` copied from round 13, `make_grids.py` and `run_probes.py` rewritten, and `grammar.py` new), and from read-only runs of `tools/check_specification.py`, `tools/question_index.py --check`, and `tools/question_index.py --share`. Comparisons with the end of phase 15 read commit a6a2b16c through `git show`. Four probe grids ran with `tools/probe_grid.py`, and one run script built and ran programs, on the baseline compilers (`writer-probes/README.md`). Commit read: `f77bbe05`.

## 1. State from the writer's view

### 1.1 Counts

| Measure | End of phase 15 (a6a2b16c) | Now (f77bbe05) |
|---|---|---|
| Rule entries | 1700 (15 withdrawn), 1685 live | 1700 (15 withdrawn), 1685 live; no rule is new |
| Principles | 343 | 343 |
| Rules and principles whose text changed in the phase | 28 rules and 8 principles in phase 15 | 14 rules and 4 principles on 10 pages; no class change |
| Rules and principles changed after the phase's review read the tree | 17 rules and 3 principles (after RV8) | 9 rules and 1 principle on 7 pages (after RV9, 1cc1696e), all in 8d33fcb1 |
| Words on the 85 pages, without index and glossary | 280,463 | 283,533 |
| Evidence words on those pages | 74,548 | 76,329 |
| `cpp` example blocks | 238 | 240 |
| Tests (`*.cpp`, excluding `*.tu.cpp`) | 7015, of which 152 are program tests | 7073, of which 166 are program tests |
| Test files with a per-compiler override | 743 (919 override lines) | 759 (937 override lines) |
| Test files with an override for a newest family | 643 | 657: EDG alone 236, GCC alone 214, Clang alone 107, EDG and GCC 71, Clang and EDG 18, Clang and GCC 10, all three 1 |
| Tests with `tidal-error` patterns | 3177 | 3201 |
| Question files | 446, 197 open | 449, 200 open |
| `check_specification.py` | 1700 rules, 0 problems | 1700 rules, 0 problems |
| `question_index.py --check` | 446 questions, 0 problems | 449 questions, 0 problems |
| Front end | 6895 of 7015 pass, 6926 pass their check | 6967 of 7073 pass, 6990 pass their check |
| Completion line (decisions 0060, 0064) | 80 tests fail their check (32 marked, 48 counted), 5 of 152 program tests do not run, 9 out of scope | 74 tests fail their check (36 marked, 38 counted), 0 of 166 program tests do not run, 9 out of scope |
| The rewrite (`--next`) | none | unit 88, lex 7191/7191, parse 6475/6913, owned 0 |
| Exercise ledger | 1097 rows: 1004 commit, 16 open, 44 text, 30 question, 3 out of scope | 1129 rows: 1030 commit, 15 open, 48 text, 33 question, 3 out of scope |
| Front-end ledger | 165 rows: 156 commit, 0 open, 4 text, 1 question, 4 out of scope | 180 rows: 157 commit, 14 open, 4 text, 1 question, 4 out of scope |
| Committed similarity reports | 241 | 252 |

Phase 16 added 58 tests and program tests and no rule. It opened 3 questions (Q-0447 to Q-0449) and settled none.

### 1.2 What phase 16 took from round 13

| Round 13 item | Phase 16 result |
|---|---|
| 1. F6 | Done in b3f17197 and 1cc1696e. 12 new tests. P4-3 to P4-5, X25-1 to X25-4, X25-6, and F6-1 are `commit`. P4-7, X25-7, and F6-2 closed in front-end round 27 (aa8c8e60). X25-5 and X25-7 became Q-0448 and Q-0447. RV9-R-4 stays with Q-0445, and X25-9 is `text`. |
| 2. DQ | Not adopted. Decision 0064 deferred it, as decisions 0058, 0059, and 0061 to 0063 did. 44 open questions now wait only for a review (doubt). |
| 3. RW | Done. Steps 0 to 2 in 8f487785, and front-end round 27 in aa8c8e60. The rewrite parses 6475 of 6913 tests as their outcome needs and found G166 to G179. |
| 4. PQ2 and PU4 | Not adopted. Nine questions wait for a principle and four for a use (section 1.5). |
| 5. RV9 | Done in b310bf11, 8d33fcb1, and b24e224f, with the recorded read of the repair diff. 14 points (RV9-1 to RV9-14) and 8 more (RV10-R-1 to R-8). No question settled. |
| 6. RC2 | Partly done. Decision 0064 made three standing choices: a review summary lands with its question records (b310bf11 and b24e224f do), every commit message gives both checks (each phase 16 message does), and a "left open" line becomes a ledger row in the same commit. 8f487785 named the old lexer's panic with no row, and 66fc1016 added G180 after it. The save step for reader reports is not done. The tool refused the writes of the X24, RW, and round 27 reports, and the integrator saved each from the returned text. |
| Process 1, probe files with defines | Followed. X24 kept nine probe files with defines and listed eleven host-only claims. One grid and one run script of this round check all of them. |
| Process 2, the read of a repair diff before row 4 items | Not followed, with no cost. RV9 part 1 added the item "PRIM-15, Q-0063" and part 2 the item "LSUP-10, UNI-14" before the read. The read agrees with both records, with one small gap (RV10-R-6). |
| Process 3, a check before each commit that adds a review | Done (decision 0064). |
| Process 4, a front-end fault with no test gets a test and a row | Done late for G180 (66fc1016). |
| Process 5, a save step for reader reports | Not done. Three more reports came back as messages. |

### 1.3 The 15 open ledger rows, with the probes of this round

Each row that depends on compiler behavior has a probe in `writer-probes/`. Every host-only claim of the X24 report holds on the matrix, and EDG agrees with Clang wherever it takes part. The grids found two splits that no report claims. GCC alone accepts a `goto` to a missing label in a member function of a class template and in a generic lambda, as it does in X24's two template forms. Clang alone rejects an assignment to an ended `int` element of a union's array member, as it does for a struct's element (Q-0447).

| Row | What the probe shows | What the text needs | Question |
|---|---|---|---|
| X24-1 | `run-probes`: on the six compilers that build, a `goto`, a `return`, and a throw out of nested loops each end the inner body's objects, then each inner statement's condition and initializer variables and the range temporary, statement by statement from the inside out. A `goto` out of an `if` or a `switch` with an init-statement, and out of a `while` with a condition variable, ends the body's objects first. A `break` from a `for` gives the same order. | OBJ-3 adds that the variables of a statement's own scope (an init-statement, a condition, the hidden variables of a range-based `for`) end when control leaves the statement by any exit, after the objects of its body. LOOP-3 says "also after any jump, `return`, or exception that leaves the statement". Two run tests, no override. | none |
| X24-2 | `x24-compile`: every compiler accepts a `continue`, a `break`, or a `return` before a later initialized declaration, also of a class with a constructor and a destructor in constant evaluation. Every compiler rejects a `goto` to the end of the body past such a declaration. | JMP-6 names the transfers it covers: a `goto`, and a transfer to a `case` or `default` label (SW-5). `break`, `continue`, and `return` only leave scopes. The JMP-P2 row "Forward past an initialization" follows. One accept test from `k1.cpp`. | none |
| X24-3 | `x24-compile`: GCC 14 and 16 accept, and Clang and EDG reject, a `goto` to a missing label in four places: a function template never instantiated, a member function of a class template never instantiated, a generic lambda never called, and a discarded branch of a template. Every compiler rejects a label declared twice and a `continue` outside a loop in a template never instantiated, and a `break` outside a loop in a discarded branch of an instantiated template. | JMP-14 says that its conditions are checked at the definition, also in a template that no specialization uses and in a discarded statement (IF-22). DEP-30's list does not change, and its Evidence cites JMP-14. Four reject tests with a GCC override. | New: a `goto` to a missing label in a templated context (GCC alone accepts) |
| X24-4 | `x24-warnings` under `-Wreturn-type -Werror`: every Clang and GCC warns when a `goto` leaves a `while (true)` or a `for (;;)` and when a `break` leaves the loop itself. None warns when a `break` leaves only an inner `switch`, `for`, or `while`, or when a `goto` stays in the body. EDG skips the grid. | FN-7's last item reads "if no `break` of that loop and no `goto` to a label outside it leaves it". A `break` inside a nested loop or `switch` ends only that statement. The Evidence names GCC's need for code generation. Two tests with the flags of `flow-off-end-goto-loop.cpp`. | none |
| X24-5 | None needed. | A glossary entry "label" for the three kinds, with links to `blocks.md`, `jumps.md`, and `switch.md`. ATTR-P1's place table reads "an identifier label". | none |
| X24-6 | `x24-compile`: every compiler rejects a `break` and a `continue` in a member function of a local class inside a loop, and a `goto` from such a member function to a label of the enclosing function. | JMP-7 and JMP-14 name the member functions of a local class beside the lambda body, as SW-3 does. Two reject tests. | none |
| X24-7 | `x24-compile` and `run-probes`: every compiler accepts one label name in a lambda body and in its enclosing function, and in a member function of a local class and its enclosing function. `l1.cpp` exits 0 on the six compilers that build. | One line in the JMP Examples and two tests of JMP-14. | none |
| X24-8 | None possible. | None. No rule rests on the outside knowledge. The row becomes `text`. | none |
| RV10-R-1 | `rv10-points`: every compiler accepts `Holder<Coin>` in the operand of a `noexcept` specifier inside `Coin`, also when `Holder` needs a member declared after the operand. EDG alone rejects the same use in a default member initializer. Every compiler rejects a default template argument of a member template that reads a member of `Holder<Coin>`. | INST-30's list gains the operand of a `noexcept` specifier. Its Evidence names the default template argument (CLS-2, Q-0406) and keeps the default member initializer Note. One test. | none; Q-0406 holds the default template argument |
| RV10-R-2 | `rv10-points`: every compiler accepts a nested class defined after its enclosing class with a qualified name whose member and friend functions return `Holder<Inner>` or `Inner`, and rejects a return type completed only after that definition. | FN-12 reads "where the outermost class definition that contains the function definition ends". Two tests. | none |
| RV10-R-3 | `rv10-points`: every compiler accepts, in constant evaluation, an assignment through the name of a union's `int` member after a pseudo-destructor call ended it, and a read after it. Clang alone rejects the same for an element of a union's array member and for a member of a struct member of a union. | OBJ-2's item names the exception: an assignment through the name of a scalar union member makes the member active (UNI-5) and is valid. One accept test, no override. One reject test of the array element form with GCC and EDG overrides that name Q-0447, whose table gains the two union rows. | Q-0447 (dated note) |
| RV10-R-5 | `rv10-points`: every compiler gives `false` for a requires expression whose parameter type is not valid, in a concept, a variable template, an `if constexpr` condition, and a requires clause. | A CPT-28 Evidence note says that the variable template and `if constexpr` forms rest on CPT-28 alone and that every compiler agrees. The row becomes `text`. | none |
| RV10-R-6 | None needed. | The LSUP-10 Note and the row 4 item "LSUP-10, UNI-14" say "an object whose lifetime began in the same evaluation". | none |
| RV10-R-7 | None possible. No program observes when an array's lifetime begins. | OBJ-1's array row names an element type whose default constructor is not trivial. | none |
| RV10-R-8 | None needed. | The OPR Note names `1e5000_ld`, the suffix that `literal-float-form-out-of-range.cpp` declares. | none |

Thirteen rows need text. X24-8 needs no change, and RV10-R-5 needs only a note. Nine rows need tests, about 22 in all. One row needs a new question (X24-3).

Two points outside the 15 rows also need tests:

- **The Q-0449 forms that no test holds (RV10-R-4).** `rv10-points`: Clang and GCC reject, and EDG accepts, the copy of a `constexpr` union whose active member is not `mutable`, the copy of a `constexpr` struct with a `mutable` member, and a read of the `mutable` member of a `constexpr` union. No rule of the constant-evaluation page says that constant evaluation cannot read a `mutable` member of an object that it did not create. CE-3 or CE-22 gains that sentence, with two reject tests that carry an EDG override. The first form follows CE-22 and Q-0449's option 1, so its test expects acceptance with Clang and GCC overrides that name Q-0449.
- **`[[maybe_unused]]` after the type specifiers (side finding of `grammar-forms`).** GCC accepts `int [[maybe_unused]] c;`, where the attribute applies to the type `int` (ATTR-19). Clang and EDG reject it. One reject test of ATTR-2 with a GCC override that names Q-0021.

### 1.4 The 14 open front-end rows, and how the Syntax segments should derive each form

`grammar-forms` compiles each form that rows G166 to G179 name on the baseline compilers. Every form that the rows name is valid on every compiler, except where a question already holds the split: `throw()` (EDG rejects, Q-0077), the name after `template` with no argument list (Clang rejects, Q-0226), `~decltype(t)` (GCC rejects, Q-0089), and an attribute before a base (GCC 14 rejects, Q-0177). So each row is text work. The parser of the rewrite then needs one function for each new production (`syntax::tests::each_nonterminal_has_one_function_in_its_page`) and nothing more.

| Row | Page of the Syntax segment | How the segment should derive the form | Rules to cite |
|---|---|---|---|
| G166 | `expressions/casts.md` | `cast_type` takes `[nested_name] (identifier \| template_id)` in place of `[nested_name] identifier`. A sentence after the CAST rules says that lookup decides whether `name(args)` or `name<args>(args)` is a cast or a call, from the kind of entity the name denotes. | CAST-18, CAST-10 |
| G167 | `classes/derived.md`, `classes/constructors.md` | A new nonterminal on the derived-classes page, such as `base_type: [nested_name] (class_name \| template_id) \| decltype_spec`. `base_spec` ends in `base_type`, and `mem_init_id` reads `identifier \| base_type`. `base_clause` and `ctor_init` already carry `["..."]`, which covers the pack of bases. | DER-1, CTOR-15, PACK-14, DTY-3 |
| G168 | `declarations/namespaces.md` | `using_directive` and `namespace_alias` take `[nested_name] identifier` in place of `qualified_name`. | NS-8, NS-14, NS-17 |
| G169 | `declarations/enumerations.md` | A new production `using_enum_decl: "using" "enum" [nested_name] identifier ";"`, as an alternative of `block_decl` and of `member_decl`. Namespace scope reaches it through `declaration`. | ENUM-9, ENUM-20 |
| G170 | `declarations/attributes.md`, `declarations/index.md`, `default-arguments.md`, `classes/derived.md` | `decl_specifiers` ends in `{attribute_spec}`, `parameter` starts with `{attribute_spec}`, and `base_spec` starts with `{attribute_spec}`. The ATTR-19 table has two more positions that no production derives: after `namespace` (`namespace_def`) and after the type or the bound in a `new` expression (`new_type`, `new_bounds`). The tests that use them stop first at a G170 position, so the parse count does not show them. | ATTR-19, ATTR-2 |
| G171 | `templates/arguments.md`, `expressions/primary.md`, `classes/constructors.md` | `template_id` gains `(operator_function_id \| literal_operator_id) "<" [template_args] ">"`. `unqualified_id`'s destructor alternative reads `"~" (class_name \| template_id \| decltype_spec)`, and the `dtor_call` shape follows. | DTOR-6, DTOR-8, DTY-3, OPR-1, OPR-8, Q-0089 |
| G172 | `classes/members.md` | `member_decl` gains `enum_opaque`. | ENUM-7 |
| G173 | `classes/members.md`, `templates/declarations.md` | `member_decl` replaces `template_decl` with `template_head member_decl`, so a member template's declaration need not have decl-specifiers. A TPL sentence says which member declarations can follow a template head. | TPL-24, CVF-1 |
| G174 | `statements/if.md` | `init_stmt` gains `sb_decl`. | IF-P2, SB-7, SB-15 |
| G175 | `declarations/exception-specifications.md`, `declarations/declarators.md` | A production on the exception-specifications page, such as `exception_spec: noexcept_specifier \| "throw" "(" ")"`, takes the place of `[noexcept_specifier]` in `function_suffix`. | EXS-5, Q-0077 |
| G176 | `declarations/static-assert.md` | `static_assert_decl` takes `string_literal {string_literal}`, as `literal` does. | SA-4 |
| G177 | `classes/conversion-functions.md` | `conversion_type: type_specifiers {ptr_part}`, which reuses `ptr_part` and so takes `nested_name "*"` and cv-qualifiers after `*`. | CVF-1 |
| G178 | `templates/arguments.md` | `template_arg` gains `braced_init`, for a constant parameter of class type. | TARG-16, TARG-19 |
| G179 | `templates/dependent-names.md`, `templates/declarations.md` | `typename_specifier` gains `"typename" nested_name "template" identifier`, and the default of `template_template_param` reads `[nested_name] ["template"] identifier`. DEP-26 limits where the name alone is valid. | DEP-25, DEP-26, CTAD-1, Q-0226, Q-0243 |

By report-rw1's table, the G rows account for 211 of the 438 parse disagreements in tests that need a tree, and 87 in reject tests. With the productions in place, `next: parse` can rise toward 6686 of 6913. Some tests hold a second gap behind the first, as the two ATTR-19 positions show, so the rise can be smaller until those are written too.

The grammar text carries a similarity risk that the rules do not. A production can echo the reference's grammar in names and shape. Each new production keeps Tidal's nonterminal names and reuses the productions the segments already have (`ptr_part`, `braced_init`, `template_id`).

### 1.5 The questions phase 16 opened

| Question | **Waits for:** | What a writer can do now |
|---|---|---|
| Q-0447 (CE-21, OBJ-2, an ended part in a constant) | a review (doubt); newest families that agree | Add the two union rows of RV10-R-3 to the table and OBJ-2's exception for a union member named directly. The families still split in the table's fifth row. |
| Q-0448 (CE-22, a part with no value after a union copy) | a review (doubt) | Nothing. EDG alone differs. |
| Q-0449 (CE-22, a `mutable` active member) | a review (doubt); newest families that agree; a principle (CE-P3) | Add the three forms of RV10-R-4 as tests. Add a CE-P3 sentence: a copy cannot read a `mutable` part of an object that the evaluation did not create. |

Nine open questions now wait for a principle that a writer can add: Q-0196 (TPL-P3), Q-0429 (DED-P2), Q-0436 and Q-0440 (PRIM-P1), Q-0437 (CTAD-P1), Q-0443 (CTAD-P3), Q-0444 (CTOR-P1), Q-0446 (OPR-P3), and Q-0449 (CE-P3). Four wait for a use of a principle: Q-0424 (UNI-P3, with a doubt), Q-0427 (DEP-P5), Q-0432 (DEP-P2), and Q-0435 (OPR-P3). Round 13 gave the sentences for the first seven (`roadmap-13/writer.md`, section 1.4), and none is written yet.

44 open questions wait only for a review (doubt), 15 for a review (silent), 6 for a review (against), and 1 for a review (part) (`writer-scratch/doubt_only.py`, `waits.py`). Decisions 0058, 0059, and 0061 to 0064 each deferred a narrow second review of the doubt-held questions. Under decision 0054, a review that supports one of the 44 with no doubt settles it.

### 1.6 The pending.md row 4 items that phase 16 added

| Item | Families | What it waits for |
|---|---|---|
| PRIM-15, Q-0063 (a floating literal out of range, also through the `long double` form) | Clang 22 and GCC 16 accept with a warning and keep the matrix; EDG follows the review | Row 4b. Q-0063 joined it. |
| LSUP-10, UNI-14 (`std::construct_at` of a union member that is not active, in constant evaluation) | Every family keeps the matrix | Row 4a, a rule item. RV10-R-6 corrects its wording. |

Row 4a holds 20 questions and 18 rule items. Row 4b holds 24 questions. Q-0447 and Q-0449 joined the packet's row 1. The request to the maintainer of 2026-11-10 has no answer. `question_index.py --share` reports "Rows 1 to 9 hold 122 of the 200 open questions (61%). With row 10, the rows hold 137 (68%)."

### 1.7 The 10 rules that changed after RV9 read the tree

The close lists UNI-14, FN-12, CE-22, OBJ-1, OBJ-2, STG-6, CPT-28, INST-P4, INST-1, and INST-30. `changed.py` from 1cc1696e to f77bbe05 gives the same 9 rules and 1 principle. Every change is in 8d33fcb1, and `changed.py` from 8d33fcb1 to f77bbe05 gives none. The read of the RV9 repair diff (b24e224f) read every hunk of 8d33fcb1. So each of the 10 has a recorded read of its present text, and the read's points are the open RV10 rows.

| Rule | The read's verdict | Open point | This round's probe |
|---|---|---|---|
| UNI-14 | agrees | RV10-R-6 (the Note's wording) | none needed |
| STG-6 | agrees | none | none needed |
| FN-12 | agrees with a doubt | RV10-R-2 | every compiler checks a nested class defined outside its enclosing class at the end of its own definition |
| CE-22 | agrees with a doubt | RV10-R-4 (Q-0449) | the three untested forms split Clang and GCC against EDG |
| OBJ-1 | agrees with a doubt | RV10-R-7 | none possible |
| OBJ-2 | adds a condition | RV10-R-3 | a union member named directly takes the assignment on every compiler |
| CPT-28 | agrees for a concept | RV10-R-5 | every compiler gives `false` in four places |
| INST-30, INST-1, INST-P4 | agree with a doubt | RV10-R-1 | every compiler puts the `noexcept` operand's point after the class |

None of the 10 needs a second reading before phase 17 changes it. F7 changes FN-12, INST-30, OBJ-1, OBJ-2, and CE-22 again, with OBJ-3, LOOP-3, JMP-6, JMP-7, JMP-14, FN-7, and CE-3. The next review reads that text.

### 1.8 Distance to decision 0060's completion test

The completion line reads "74 tests fail their check, of which 36 need the template model (marks file) and 38 count toward the gate of decisions 0062 and 0063; 0/166 program tests do not run to their result; 9 tests out of scope". Decision 0064 made the gate's count a measure. Report 27 names a cause for each of the 38: 19 missed rejects, 16 unsupported, 1 false reject, and 2 EDG-AGREES. Every program test runs, and 23 tests stop in a run after they pass their check.

The rewrite owns no test. Its next step is build step 3 (analysis). It takes the 36 marked tests at steps 5 to 7: model sections 1, 2, and 5 at step 5, the 13 tests of section 3 at step 6, and the 6 of section 4 at step 7. The rewrite parses 6475 of 6913 tests as their outcome needs, and the G rows hold most of the rest (section 1.4).

Writer work touches the measures in two ways:

- F7 adds about 22 tests to the old front end's suite. The X24-3 rejects in templates, the X24-1 run order, and the Q-0449 forms read cases that front-end round 27 did not test, so some can fail.
- GR changes no test and no old front-end status. It raises only the rewrite's parse count.

## 2. Proposed items, ranked

| Rank | Item | Evidence | Cost (writer sessions) | Similarity risk | Unblocks |
|---|---|---|---|---|---|
| 1 | F7. Answer the 15 open rows, the Q-0449 forms, and the `maybe_unused` side finding | section 1.3; four grids and one run script | 1.25 | low | FE28 targets; the P22 prototype's base; text that the next review reads |
| 2 | GR. Write G166 to G179 and the two hidden ATTR-19 positions into the Syntax segments | section 1.4; `grammar-forms` | 0.75 | medium | the rewrite's parse count; step 3 on more trees |
| 3 | RW2. Rewrite step 3, with front-end round 28 on the current code | decisions 0060 and 0064; section 1.8 | 0.25 for ledger rows and grammar answers | low | the completion test |
| 4 | DQ. A narrow second review of the 44 doubt-held questions | section 1.5; six deferrals | 0.75 of records and repairs | medium | settlement of up to 44 questions with no maintainer row |
| 5 | PQ2 and PU4. Principles for the 9 questions that wait for one, and a use batch for the 4 that wait for a use | section 1.5 | 0.5 | medium | Q-0427, Q-0432, and Q-0435 in phase 17, and 9 more in phase 18 |
| 6 | RV10. Gap review of F7's and GR's text, with the read of its repair diff | section 1.7 | 0.5 of repairs | medium | a reviewed tree for the rule IDs that the rewrite's analysis cites |
| 7 | RC3. Records: a save step for reader reports, and a parse count that lists every gap in a test | sections 1.2 and 1.4 | 0.2 | low | fewer messages that carry reports; no hidden grammar gaps |

Items 1, 2, and 4 to 7 cost about 3.95 writer sessions. With RW2, about 4.2.

### Item 1. F7. Answer the open rows with the probes of this round

- **What.** Section 1.3, row by row. In order:
  - Jumps and scopes (X24-1, X24-2, X24-6, X24-7): the OBJ-3 sentence on a statement's own scope, LOOP-3's "any exit", JMP-6's list of transfers, JMP-7 and JMP-14 on local classes, and a JMP Example line. About 8 tests from `d1.cpp`, `d2.cpp`, `k1.cpp`, `l1.cpp`, `l2.cpp`, and the local class rows.
  - Jumps in templates (X24-3): JMP-14's check at the definition, four reject tests with a GCC override, and a new question.
  - Warnings (X24-4): FN-7's last item and two tests with `-Wreturn-type -Werror`.
  - Labels (X24-5): the glossary entry "label" and ATTR-P1's wording.
  - The read of the RV9 repair diff (RV10-R-1 to R-3, R-5 to R-8): INST-30's `noexcept` operand, FN-12's outermost definition, OBJ-2's exception for a union member with Q-0447's two rows, OBJ-1's default constructor, the CPT-28 note, and the LSUP-10 and OPR Notes. About 7 tests.
  - Q-0449's forms: a CE-3 or CE-22 sentence on a `mutable` member of an object that the evaluation did not create, and three tests.
  - The ATTR-2 side finding: one test under Q-0021.
  - X24-8 and RV10-R-5 become `text`.
- **Needs first.** Nothing. The probes decide every row that depends on behavior.
- **Evidence.** Section 1.3 and `writer-probes/README.md`.
- **Cost.** 1.25 sessions.
- **Risk.** Low. Each test records the families that differ as overrides. The OBJ-3 and JMP-6 sentences are about scopes and jumps, where a writer can recall the reference's phrasing, so the similarity run of F7 reads those pages first.
- **Unblocks.** FE28's targets. The X24 hook of `DESIGN-rewrite.md` section 7 assumes that the machine ends a statement's scope variables when a `goto` leaves (X24 report, section 3). OBJ-3 then states that order, and the P22 prototype has a rule to test against.

### Item 2. GR. The grammar gaps

- **What.** Section 1.4, row by row: about 20 changed alternatives and 3 new nonterminals (`base_type`, `using_enum_decl`, `exception_spec`) on 14 pages, with the two ATTR-19 positions of G170. Each page's Syntax segment changes, and each new production gets a sentence in a rule that already covers its form. The writer checks the ATTR-19 table against the grammar row by row, since the parse count hides a second gap in one test.
- **Needs first.** Nothing. `grammar-forms` gives the class of each form.
- **Evidence.** Section 1.4, report-rw1 section 10, and the front-end ledger rows.
- **Cost.** 0.75 session.
- **Risk.** Medium for similarity, as section 1.4 says. Low for behavior, because every form is in passing tests now.
- **Unblocks.** `next: parse` toward 6686 of 6913, and trees for 211 more tests at build step 3.

### Item 3. RW2. Rewrite step 3, and front-end round 28

- **What.** Status step 2 continues the rewrite from build step 3. The old front end stays the front end of record and takes F7's tests in a round 28. The writer answers the grammar findings that step 3 gives, as for G166 to G179.
- **Needs first.** F7 for the round's tests. GR for the trees.
- **Cost.** 0.25 writer session for the ledger rows and grammar answers.
- **Risk.** Low for the writer. Each round keeps `frontend_check.py` at 0 worse.
- **Unblocks.** The completion test.

### Item 4. DQ. A narrow second review of the doubt-held questions

- **What.** A source reader reads the 44 questions whose **Waits for:** line names only a review (doubt), against the current text and tests, and says for each whether the doubt stands. The list comes from `writer-scratch/doubt_only.py`. The writer records each reading, settles each question with no doubt left under decision 0054, and repairs the text where the reader reads against.
- **Needs first.** Nothing. F7 touches Q-0447 and Q-0449, which are not on the list.
- **Evidence.** Section 1.5. Phases 15 and 16 settled no question, and 122 of the 200 open questions wait on the maintainer's rows.
- **Cost.** One source-reader session in two or three parts. 0.75 writer session.
- **Risk.** Medium. Many of the questions had their last reading in early November, and their rules changed since.
- **Unblocks.** The only large drop in the open count that does not wait on the maintainer.

### Item 5. PQ2 and PU4. Principles, then their use

- **What.** PQ2 adds one sentence to each of PRIM-P1, CTAD-P1, CTAD-P3, CTOR-P1, OPR-P3, DED-P2, TPL-P3, and CE-P3, as round 13 section 1.4 and section 1.5 here give. PU4 is a principle-use batch on Q-0424, Q-0427, Q-0432, and Q-0435.
- **Needs first.** Nothing for PU4. The OPR-P3 sentence comes after PU4, so that the batch uses the OPR-P3 that Q-0435 names.
- **Cost.** 0.5 session.
- **Risk.** Medium. A principle states a model that can echo the reference's organization. Each sentence cites the rule that already gives the case.
- **Unblocks.** Up to three settlements in phase 17 and nine in phase 18 (decision 0055).

### Item 6. RV10. Gap review of the changed text

- **What.** A source-reader review of F7's text, GR's productions, and PQ2's sentences, then the repairs and a recorded read of their diff. The prompt names the points left as `text` that F7 touches (RV10-R-5, RV9-3, RV9-5, RV9-13).
- **Needs first.** F7, GR, and PQ2.
- **Cost.** 0.5 writer session of repairs.
- **Risk.** Medium. The grammar review is new in kind for the reviews, and the union and `mutable` sentences sit near Q-0115, Q-0425, and Q-0448.
- **Unblocks.** A tree whose rule text a reviewer has read, for the rule IDs that the analysis of build step 3 cites.

### Item 7. RC3. Records

- **What.** Two records. The reader prompts name a path in the scratchpad for the report, and the integrator copies the file, so that a refused write does not route the report through a message. `frontend_check.py --next` lists, for each test that stops, every form after the first stop that the grammar also does not derive, or the writer's check of a rule's table against the grammar becomes a step of GR.
- **Needs first.** Nothing.
- **Cost.** 0.2 session.
- **Risk.** Low.

### What the rules need before the next steps

- **Before FE28.** F7's tests. None of the open rows waits only for the front end.
- **Before the rewrite's step 3.** GR, so that analysis starts from trees for the forms that tests use.
- **Before the P22 prototype.** The OBJ-3 sentence of X24-1 and the JMP-14 sentence of X24-3, so that the prototype's run step and its definition checks have rules to cite. JMP-7 and JMP-14 on local classes (X24-6) give the same-function condition.
- **Before the next Propose.** The glossary entry "label" (X24-5).
- **Propose and Diagnose.** Unchanged. F7 changes text by sentences, rows, and examples. GR changes productions.

## 3. Out of scope

1. **No new topic.** Phase 16 added no rule and no page.
2. **Coroutines, modules, threads and the memory model, extended floating-point types, the preprocessor beyond `#include "file"`, and the general library stay out**, as the charter and decision 0051 say.
3. **The maintainer's rows.** 122 of 200 open questions wait on rows 1 to 9 (61%), 137 with row 10 (68%). The request of 2026-11-10 has no answer.
4. **P22 itself.** X24 is a Propose exercise. F7 answers its findings on the current text and adopts none of its new rules. JMP-17 stays.
5. **The rewrite's code.** The writer's items add no test that needs the template model.
6. **Lexical structure.** GR writes only the productions that tests use and the segments do not derive. The roadmap's deferral of lexical grammar beyond G1 stands.
7. **MSVC and later versions** stay deferred (row 7). X24-8 names later work, and no rule rests on it.

## 4. Process changes

Ranked by writer time saved.

1. **A parse count that shows every gap in a test.** Two ATTR-19 positions hide behind G170 in the same tests, and the parse count names only the first stop. A list of every gap, or a table check in GR, finds them in one pass (item RC3).
2. **A reader keeps its probes as files with defines.** X24's nine probe files went into one grid and one run script with no rewriting, and every claim was checked in one run. The practice of round 13 held.
3. **The close names the rules that changed after the read of the repair diff.** The phase 16 close lists 10 rules that changed after RV9 read the tree. All 10 changed in 8d33fcb1, and the read b24e224f read that commit, so none waits for a reading. A line from `changed.py` run from the read's commit gives that fact in the close.
4. **A save step for reader reports** (item RC3). Nine reports in three phases came back as messages.
5. **A grammar finding names its rule table.** G170's suggestion named three positions where ATTR-19's table holds five that the grammar does not derive. A finding that cites the table row by row is complete at once.

General knowledge used in this report: none. Every count comes from Tidal's files, its commit messages, scripts run on commits a6a2b16c, 1cc1696e, 8d33fcb1, and f77bbe05, and the writer probes in `writer-probes/`.
