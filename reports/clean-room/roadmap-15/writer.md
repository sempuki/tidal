# Roadmap, round 15: what to do after phase 17, from a clean-room writer's view

Date: 2026-11-11. Decision 0029, with decisions 0033, 0039, 0050, 0051, 0054, 0055, 0056, 0057, 0058, 0059, 0060, 0061, 0062, 0063, 0064, and 0065. A clean-room agent in the role of a clean-room writer, who drafts rules, Evidence, and tests from compiler behavior, prepared this report. It read AGENTS.md, CLAUDE.md, the round 14 writer report `reports/clean-room/roadmap-14/writer.md` with its probes and scripts, `documentation/status.md`, `documentation/roadmap.md` (the phase 17 table), decisions 0060 to 0065, `documentation/decisions/pending.md`, `git log fb0bcf87..7a849708` with every commit body, the phase 17 reports (`reports/clean-room/X26/report.md` with its probes, `rust-frontend/report-rw2.md`, `report-28.md`, and `report-rw3.md`), the RV10 review and the read of its repair diff in `reports/review/` dated 2026-11-11, the question files Q-0450 to Q-0452, `reports/clean-room/ledger.md`, `reports/clean-room/rust-frontend/ledger.md`, and the rules, tests, and Syntax segments that its items name. It did not read the other round 15 reports, `inputs/`, `build/exercise-keys/`, or the sources under `frontend/`. Counts come from read-only `git`, `grep`, and the scripts in `writer-scratch/` beside this report (`changed.py`, `counts.py`, `rule.py`, `waits.py`, and `doubt_only.py` copied from round 14, `make_grids.py` and `run_probes.py` rewritten), and from read-only runs of `tools/check_specification.py`, `tools/question_index.py --check`, and `tools/question_index.py --share`. Comparisons with the end of phase 16 read commit fb0bcf87 through `git show`. Seven probe grids ran with `tools/probe_grid.py`, and one run script built and ran programs, on the baseline compilers (`writer-probes/README.md`). Commit read: `7a849708`.

## 1. State from the writer's view

### 1.1 Counts

| Measure | End of phase 16 (fb0bcf87) | Now (7a849708) |
|---|---|---|
| Rule entries | 1700 (15 withdrawn), 1685 live | 1700 (15 withdrawn), 1685 live; no rule is new |
| Principles | 343 | 343 |
| Rules and principles whose text changed in the phase | 14 rules and 4 principles in phase 16 | 23 rules and 3 principles on 14 pages; no class change |
| Rules and principles changed after the phase's review read the tree | 9 rules and 1 principle (after RV9) | 9 rules on 8 pages (after RV10, e3cb5a3c), all in 88d14ea2 |
| Words on the 85 pages, without index and glossary | 283,533 | 286,554 |
| Evidence words on those pages | 76,329 | 78,303 |
| `cpp` example blocks | 240 | 240 |
| Tests (`*.cpp`, excluding `*.tu.cpp`) | 7073, of which 166 are program tests | 7148, of which 178 are program tests |
| Test files with a per-compiler override | 759 (937 override lines) | 772 (955 override lines) |
| Test files with an override for a newest family | 657 | 669: EDG alone 238, GCC alone 220, Clang alone 107, EDG and GCC 73, Clang and EDG 18, Clang and GCC 12, all three 1 |
| Tests with `tidal-error` patterns | 3201 | 3227 |
| Question files | 449, 200 open | 452, 202 open |
| `check_specification.py` | 1700 rules, 0 problems | 1700 rules, 0 problems |
| `question_index.py --check` | 449 questions, 0 problems | 452 questions, 0 problems |
| Front end | 6967 of 7073 pass, 6990 pass their check | 7049 of 7148 pass, 7069 pass their check |
| Completion line (decisions 0060, 0064) | 74 tests fail their check (36 marked, 38 counted), 0 of 166 program tests do not run | 70 tests fail their check (38 marked, 32 counted), 1 of 178 program tests does not run |
| The rewrite (`--next`) | unit 88, lex 7191/7191, parse 6475/6913, owned 0 | unit 126, lex 7266/7266, parse 6828/6987, step 3 plain 1602/1897, owned 1327 |
| Exercise ledger | 1129 rows: 1030 commit, 15 open, 48 text, 33 question, 3 out of scope | 1164 rows: 1054 commit, 13 open, 54 text, 40 question, 3 out of scope |
| Front-end ledger | 180 rows: 157 commit, 14 open, 4 text, 1 question, 4 out of scope | 183 rows: 172 commit, 2 open, 4 text, 1 question, 4 out of scope |
| Committed similarity reports | 252 | 266 |

Phase 17 added 75 tests and program tests and no rule. It opened 3 questions (Q-0450 to Q-0452) and settled 1 (Q-0450).

The completion line counts 32 tests toward the gate. Round 28 left 26 of them. The other 6 are RV10 repair tests that the old front end does not pass: five missed rejects (`maybe-unused-on-using-directive.cpp`, `decltype-auto-destructor-call.cpp`, `decltype-auto-functional-cast.cpp`, `using-enum-dependent.cpp`, `two-template-heads-in-class.cpp`) and one false reject (`new-type-pointer-forms.cpp`). The program test that does not run is `r14-fsm-second-try.cpp`, which is marked (INST-5).

### 1.2 What phase 17 took from round 14

| Round 14 item | Phase 17 result |
|---|---|
| 1. F7 | Done in b511838d, with round 28 (c0461b86) for X24-4, RV10-R-1, and F7-1. 25 tests. X24-3 became Q-0451, and the X24-1 constant order became Q-0452. F7-2 is open. |
| 2. GR | Done in 1c223631 and 924fd6ce. The RV10 repairs (88d14ea2) added `decltype_expr` and `new_ptr`, and the parser follows them (5f100f69). G182 came from item GR and G183 from round 28. |
| 3. RW2 | Done in part (5f100f69). Analysis has its core, machine, and harness, 48 `check.rs` parts and 20 `run.rs` parts, the first takeover of 1327 tests, and the P22 prototype. Lambdas, structured bindings, `if consteval`, the `noexcept` operator, assumptions, `long double`, linking, and 122 missing reject checks remain. |
| 4. DQ | Not adopted. Decision 0065 deferred it, as decisions 0058, 0059, and 0061 to 0064 did. 44 open questions still wait only for a review (doubt). |
| 5. PQ2 and PU4 | Not adopted. Ten questions now wait for a principle and four for a use (section 1.5). |
| 6. RV10 | Done in 18efd5eb, 80a70210, 88d14ea2, and 7f61141b, with the recorded read of the repair diff. 19 points (RV10-1 to RV10-19) and 8 more (RV11-R-1 to R-8). Q-0450 settled. |
| 7. RC3 | Partly done. Item R11 made the parse count list every grammar stop (`tidal-next parse --all`, 5f100f69). The save step for reader reports is not done. The tool refused the writes of the X26, RW2 part 2, round 28, and RW2 step 3 reports, and the integrator saved each from the returned text. |
| Process 1, a parse count that shows every gap | Done (item R11). |
| Process 2, probe files with defines | Followed. X26 kept seven probe files, two of them with defines, and listed seven host-only claims. One grid and one run script of this round check all of them. |
| Process 3, the close names the rules changed after the read | Done. The close (7a849708) names the 9 rules and says that the read of 88d14ea2 covers them. |
| Process 4, a save step for reader reports | Not done. Four more reports came back as messages, 13 in four phases. |
| Process 5, a grammar finding names its rule table | Followed for G170 (both hidden ATTR-19 positions are in 1c223631). RV10 still found five forms that the reference derives and the new productions did not (RV10-1, RV10-3, RV10-5, RV10-8). |

### 1.3 The 13 open ledger rows, with the probes of this round

Each row that depends on compiler behavior has a probe in `writer-probes/`. Every host-only claim of the X26 report holds on Clang 18 and 22 and GCC 14 and 16, and EDG accepts every X26 probe. The grids found four splits that no report claims. GCC alone rejects a write through the chain to a union member of class type whose default constructor is deleted (RV11-R-5). Clang alone accepts `{1}` as the argument for a `double` parameter (RV11-R-7). EDG alone accepts `typename T::template Box(3)` (RV11-R-2). EDG rejects `deprecated` and `nodiscard` before a using-directive, as it rejects `maybe_unused` (RV11-R-3). Every compiler also accepts `typename T::Box(3)`, which the read of the RV10 repair diff reads as not valid (RV11-R-2).

| Row | What the probe shows | What the text needs | Question |
|---|---|---|---|
| X26-1 | `x26-compile`: in constant evaluation, every compiler ends the temporary of a comma's left operand after the right operand's temporary is made, also with a cast to `void`. A class prvalue as an expression statement ends before the next statement. | UNEV-P2's first bullet says that a class prvalue becomes a temporary that ends with its full expression (VC-6). UNEV-2 says "which is destroyed when its full expression ends". One constant test of the cast form, no override. | none |
| X26-2 | `x26-compile`: every compiler ends a class prvalue of a `for` iteration expression before the condition runs again, and keeps the temporary of a comma in it to the end of the expression. `x26-warnings`: every Clang and GCC warns for each form in the iteration expression exactly as for an expression statement. | The list of discarded-value expressions in the Syntax segment of `decltype.md` gains the iteration expression of a `for`. LOOP-3 links the term and cites UNEV-2. One constant test, and one warning test with `-Wunused-value -Werror` for a name as the iteration expression. | none |
| X26-3 | `x26-warnings`: 30 forms in three contexts. The contexts never differ. Every Clang and GCC warns for the 12 forms with no effect that X26 lists, and none warns for a volatile name, a class construction, `new int`, a user-defined literal, an increment, an assignment, a call, a cast to `void`, or a conditional of calls. Clang alone warns for an uncalled lambda and for `(f(), a)`. | UNEV-3 becomes the one rule for the warning. It says that an evaluation has an effect when it calls a function (a constructor included), writes an object, reads a volatile object, or allocates, and gives the forms with none. COMMA-5 and STMT-4 link to UNEV-3. The Evidence gives the table of `x26-warnings`. Two warning tests (a literal and a member access as a left operand), and one accept test of the forms that no compiler warns about. | none |
| X26-4 | None needed. | A glossary entry "discarded-value expression" that lists the four places (with X26-2's iteration expression) and links UNEV-2. COMMA-1 links the term. | none |
| X26-5 | `x26-compile` and `run-probes`: every compiler calls an overloaded comma in a right fold and in a left fold over class elements, in constant evaluation and at run time (`o1.cpp`). | PRIM-28 cites OPR-6 for an operand of class type. An Example adds a fold over a class with an overloaded comma. One constant test with both folds. | none |
| X26-6 | `x26-library`: with their own library headers, Clang and GCC find no member or non-member `operator,` for any type of the language-support table, with itself or with `int`. The EDG container has no library headers. | None. The row becomes `text`, with a record that names the grid. P26 stays a Propose exercise. | none |
| RV11-R-1 | `rv11-rows`: every compiler accepts a friend declaration with two template heads that names a member function template, or a nested class template, that the specializations of a class template declare. Every compiler rejects the form when the first head's parameter is not in the qualifier, and two heads before a member declaration. | TPL-4's sentence names a member declaration. A second sentence says that a friend declaration in a class can take two template heads when the first belongs to the class template that the qualifier names with its parameters. Two tests: the friend forms (accept) and `A<int>` (reject). | none |
| RV11-R-2 | `rv11-rows`: Clang 18 alone rejects a functional cast to an alias template, with parentheses and with braces. Every compiler accepts `typename T::Box(3)` after substitution, and every compiler except EDG rejects `typename T::template Box(3)`. | The casts paragraph names an alias template whose arguments are deduced (CTAD-14). One accept test with a Clang 18 override (Q-0020, Q-0240). For the dependent qualifier, the paragraph keeps the matrix's answer and says that such a name with no argument list deduces after substitution. A pending.md row 4 rule item records the read. The EDG form joins Q-0226's table. | new row 4 item; Q-0226 (row) |
| RV11-R-3 | `rv11-rows`: every compiler accepts `[[]]` before a using-directive, and rejects any attribute before a using-declaration. Clang and GCC accept `maybe_unused`, `deprecated`, and `nodiscard` before a using-directive, and EDG rejects all three. | ATTR-19's table gains a row "Before a using-directive", which applies to the directive. ATTR-2's item names `deprecated` and `nodiscard` beside `maybe_unused`. Two reject tests with Clang and GCC overrides (Q-0021), whose table gains the two rows. | Q-0021 (rows) |
| RV11-R-5 | `rv11-rows`: GCC alone rejects a write through the chain to a union member of class type whose default constructor is deleted. Clang and EDG accept it. Every compiler accepts the form with a defaulted constructor. | Only one newest family follows the read, so the text keeps the matrix's answer (pending.md row 4). A UNI-13 Note records the deleted constructor. One accept test with a GCC override that names a new question. The second part of the row is F7-2. | new: a deleted trivial default constructor on the chain (GCC alone rejects) |
| RV11-R-7 | `rv11-rows`: every compiler accepts `{3}` and `{}` for an `int` parameter and `{nullptr}` for a pointer parameter, and rejects `{3.5}` and `{3, 4}` for `int` and `{300}` for `char`. Clang alone accepts `{1}` for a `double` parameter, and every compiler rejects `X<3.5>` for `int`. | TARG-16 says that a braced list for a parameter of scalar type holds at most one value, which converts as TARG-13 says and must not narrow, and that empty braces give zero. The row 4 item "TARG-16" names the scalar case. Two tests: the accepted forms, and `{1}` for `double` (reject, Clang override). | new: a braced integer argument for a floating-point parameter (Clang alone accepts) |
| RV11-R-8 | `rv11-r8`: every compiler rejects `decltype(auto)::v` in an expression and `decltype(auto)::t` in `sizeof`. GCC alone names the placeholder. Clang and EDG give a parse error, and in a declaration they read `::t` as a qualified declarator. | One reject test of the expression form for AUTO-11, with a `tidal-error` pattern for GCC alone. The test joins the tests where a family has no pattern. | none |
| F7-2 | `rv11-rows`: Clang alone rejects a write to an ended `int` member of a union's living struct member, and the read of the struct's other member after it. GCC and EDG accept. | One reject test, which OBJ-2's plain reading gives, with GCC and EDG overrides that name Q-0447, as the array element test does. | Q-0447 (dated note) |

Eleven rows need text, X26-6 needs only a record, and X26-4 needs only the glossary. Ten rows need tests, about 18 in all. Two rows need a new question (RV11-R-5 and RV11-R-7), and one needs a row 4 rule item (RV11-R-2).

### 1.4 The two open front-end rows

| Row | What the probe shows | What the text needs | Rules to cite |
|---|---|---|---|
| G182 | `g182-union`: every compiler accepts a union partial specialization, also with `final`, an explicit specialization of a union template, a union defined with a namespace or a class qualifier, a member union template defined outside its class, and an explicit specialization of a member union. Every compiler rejects a mismatched class key in either direction. | `union_def` takes `[nested_name] (class_name \| template_id) ["final"]`, as `class_head` does. The parser function follows (decision 0065, grammar text before parser code). Two accept tests (the qualified definition and the member union template defined outside). `r10-variant.cpp` and the two TSP-32 tests then parse to their first error. | UNI-1, TSP-32, TPL-P1 |
| G183 | `g183-list4`: every compiler lets a value whose class converts to the member's aggregate class initialize the member whole. That holds also when the class also converts to `int`, through a conversion function template, and for an object of a derived class. Three such values for a struct of two members are too many. | LIST-4's last sentence says that a value initializes the member whole when it converts to the member's type, such as an object of that type or of a class derived from it, or an object whose class has a conversion function to it. Braces are then not elided, even when the value also converts to the type of the member's first element. Two tests: a constant check of the members, and the reject of three values. | LIST-4, LIST-12, CVF-1, DED-31 |

Both rows are text work that every compiler agrees on. G183 changes no front-end status, because round 28 already counts the value as initializing the member whole. G182 raises the parse count by one program test.

### 1.5 The questions phase 17 opened

| Question | **Waits for:** | What a writer can do now |
|---|---|---|
| Q-0450 (DTY-3, a braced list after a `decltype` base name) | settled (80a70210) | Nothing. |
| Q-0451 (JMP-14, a `goto` to a missing label in a template) | a review (doubt); a principle (JMP-P1) | Add a JMP-P1 sentence saying that the labels of a function belong to its body as written, so a discarded statement keeps its labels and jumps. That sentence gives the answer for the discarded form, the one that RV11-R-6 leaves open. |
| Q-0452 (OBJ-3, the end of a statement's own variables in constant evaluation) | newest families that agree | Nothing. GCC 16 and EDG 7.0 differ from the run order. |

Ten open questions now wait for a principle that a writer can add: Q-0196 (TPL-P3), Q-0429 (DED-P2), Q-0436 and Q-0440 (PRIM-P1), Q-0437 (CTAD-P1), Q-0443 (CTAD-P3), Q-0444 (CTOR-P1), Q-0446 (OPR-P3), Q-0449 (CE-P3), and Q-0451 (JMP-P1). Four wait for a use of a principle: Q-0424 (UNI-P3, with a doubt), Q-0427 (DEP-P5), Q-0432 (DEP-P2), and Q-0435 (OPR-P3). Rounds 13 and 14 gave the sentences for the first nine (`roadmap-13/writer.md` section 1.4, `roadmap-14/writer.md` section 1.5), and none is written yet.

44 open questions wait only for a review (doubt), 15 for a review (silent), 6 for a review (against), and 1 for a review (part) (`writer-scratch/doubt_only.py`). By item of the **Waits for:** line, 60 name a review (doubt) and 61 name newest families that agree (`waits.py`). Decisions 0058, 0059, and 0061 to 0065 each deferred a narrow second review of the doubt-held questions. Under decision 0054, a review that supports one of the 44 with no doubt settles it.

### 1.6 The pending.md row 4 items that phase 17 added

| Item | Families | What it waits for |
|---|---|---|
| TARG-16 (a braced list as a template argument) | Every family accepts; the review gives a template argument no braced form | Row 4a, a rule item. RV11-R-7 widens it to a scalar parameter. |
| JMP-14, Q-0451 | Added in 18efd5eb and removed in 88d14ea2, when Q-0451's answer changed | Nothing. |

Q-0077, Q-0123, Q-0226, and the item "PFX-P3, CLS-2, Q-0406" add the RV10 review to their review column. Row 4a holds 20 questions and 19 rule items. Row 4b holds 24 questions. The request to the maintainer of 2026-11-10 has no answer. `question_index.py --share` reports "Rows 1 to 9 hold 123 of the 202 open questions (61%). With row 10, the rows hold 138 (68%)."

### 1.7 The 9 rules that changed after RV10 read the tree

The close lists ATTR-2, AUTO-11, ENUM-20, CE-3, OBJ-2, JMP-7, JMP-14, TPL-4, and DEP-30. `changed.py` from e3cb5a3c to 7a849708 gives the same 9 rules, and from 88d14ea2 to 7a849708 gives none. The read of the RV10 repair diff (7f61141b) read every hunk of 88d14ea2. So each of the 9 has a recorded read of its present text, and the read's points are the open RV11-R rows.

| Rule | The read's verdict | Open point | This round's probe |
|---|---|---|---|
| ATTR-2 | agrees | RV11-R-3 (ATTR-19's table) | EDG alone rejects three attributes before a using-directive |
| AUTO-11 | agrees | RV11-R-8 (no test before `::`) | every compiler rejects; GCC alone names the placeholder |
| ENUM-20 | agrees | none | none needed |
| CE-3 | agrees | none (Q-0449 keeps its doubt) | none needed |
| OBJ-2 | agrees with a doubt | RV11-R-5, F7-2 | GCC alone rejects the deleted constructor form; Clang alone rejects the scalar inside a living member |
| JMP-7, DEP-30 | agree | none | none needed |
| JMP-14 | agrees for the unused template | RV11-R-6 (Q-0451) | none possible; the reference leaves the form open |
| TPL-4 | adds a condition | RV11-R-1 | every compiler accepts the friend with two heads |

None of the 9 needs a second reading before phase 18 changes it. F8 changes TPL-4, ATTR-2, ATTR-19, OBJ-2 (a Note), TARG-16, LOOP-3, UNEV-P2, UNEV-2, UNEV-3, COMMA-5, STMT-4, PRIM-28, and LIST-4. The next review reads that text.

### 1.8 Distance to decision 0060's completion test

The completion line reads "70 tests fail their check, of which 38 need the template model (marks file) and 32 count toward the gate of decisions 0062 and 0063; 1/178 program tests do not run to their result; 9 tests out of scope". Decision 0064 made the gate's count a measure. Report 28 names a cause for each of its 26, and section 1.1 names the other 6. 19 tests stop in a run after they pass their check.

The rewrite owns 1327 tests. In the plain scope of step 3 it passes the check of 1602 of 1897 tests, where the old front end passes 1892. Report rw3 names the causes of the 292 that differ: 122 missing reject checks, 67 lambdas, 13 structured bindings, 12 `if consteval`, 13 `noexcept` forms, 12 EDG-AGREES, 9 braced lists bound to an array reference, 6 assumptions, 5 `long double`, 32 other constructs, and 1 false reject (the named escape, G158 to G160). None is a finding against the text. Of the 159 parse disagreements, 142 are reject tests that stop on the line of the old front end's first error.

Writer work touches the measures in three ways:

- F8 adds about 18 tests. Most are in step 3 pages (expressions, statements, declarations), so they count in the plain scope of the shadow line. The RV11-R-1, RV11-R-2, and G183 tests are in later steps.
- GR2 raises the parse count by `r10-variant.cpp` and lets the two TSP-32 tests reach their error.
- The 6 RV10 tests that the old front end fails are front-end work. Five are missed rejects in step 3 pages, and the rewrite already passes three of them (report rw3 section 1).

## 2. Proposed items, ranked

| Rank | Item | Evidence | Cost (writer sessions) | Similarity risk | Unblocks |
|---|---|---|---|---|---|
| 1 | F8. Answer the 13 open rows with the probes of this round | section 1.3; six grids and one run script | 1.0 | low | FE29 targets; text that the next review reads |
| 2 | GR2. G182 in `union_def` and G183 in LIST-4, then the parser function | section 1.4; `g182-union`, `g183-list4` | 0.25 | low | the rewrite's parse count; an empty front-end ledger |
| 3 | RW3. The rest of build step 3, a second takeover, and front-end round 29 | decisions 0060, 0064, 0065; section 1.8 | 0.25 for ledger rows and answers | low | the completion test |
| 4 | DQ. A narrow second review of the 44 doubt-held questions | section 1.5; seven deferrals | 0.75 of records and repairs | medium | settlement of up to 44 questions with no maintainer row |
| 5 | PQ2 and PU4. Principles for the 10 questions that wait for one, and a use batch for the 4 that wait for a use | section 1.5 | 0.5 | medium | Q-0427, Q-0432, and Q-0435 in phase 18, and up to 10 more in phase 19 |
| 6 | RV11. Gap review of F8's, GR2's, and PQ2's text, with the read of its repair diff | section 1.7 | 0.5 of repairs | medium | a reviewed tree for the rule IDs of step 4 |
| 7 | RC4. Records: a save step for reader reports, a directory for X24's prototype tests, and a question in the same commit as each new split | sections 1.2 and 1.3 | 0.2 | low | fewer reports carried by messages; a measured P22 prototype |

Items 1, 2, and 4 to 7 cost about 3.2 writer sessions. With RW3, about 3.45.

### Item 1. F8. Answer the open rows with the probes of this round

- **What.** Section 1.3, row by row. In order:
  - Discarded values (X26-1 to X26-4): UNEV-P2 and UNEV-2 on the end of the temporary, the iteration expression in the list of discarded-value expressions with LOOP-3's link, UNEV-3 as the one warning rule with COMMA-5 and STMT-4 linked to it, and the glossary entry. About 6 tests.
  - Folds (X26-5): PRIM-28's citation of OPR-6 and one Example. One test.
  - X26-6 becomes `text`.
  - The read of the RV10 repair diff (RV11-R-1 to R-3, R-5, R-7, R-8): TPL-4's friend sentence, the casts paragraph and its row 4 item, ATTR-19's directive row and ATTR-2's two attributes, a UNI-13 Note, TARG-16's scalar sentence, and the AUTO-11 test. About 10 tests and two new questions.
  - F7-2: one test under Q-0447.
- **Needs first.** Nothing. The probes decide every row that depends on behavior.
- **Evidence.** Section 1.3 and `writer-probes/README.md`.
- **Cost.** 1.0 session.
- **Risk.** Low. Each test records the families that differ as overrides. UNEV-3's sentence on what counts as an effect is a definition, where a writer can recall the reference's phrasing, so the similarity run of F8 reads `decltype.md` first.
- **Unblocks.** FE29's targets and the plain-scope tests of the rewrite. X26's open points close, so a later Propose on P26 starts from settled text on discarded values.

### Item 2. GR2. The two front-end rows

- **What.** `union_def` takes the qualified name and the template-id that `class_head` takes (G182). LIST-4's last sentence names a value that converts to the member's type (G183). Each change gets its tests (section 1.4). A front-end step adds the parser function for the changed `union_def`.
- **Needs first.** Nothing.
- **Evidence.** Section 1.4, report-rw2 section 6 (RW2-1), and report-28 section 10.
- **Cost.** 0.25 session.
- **Risk.** Low. Every compiler agrees on each form.
- **Unblocks.** An empty front-end ledger, and the parse of `r10-variant.cpp` and the two TSP-32 tests.

### Item 3. RW3. The rest of step 3, a second takeover, and round 29

- **What.** The rewrite finishes build step 3 (lambdas, structured bindings, `if consteval`, the `noexcept` operator, assumptions, `long double`, linking, and the 122 missing reject checks), then takes over the tests that pass on both front ends. The old front end takes the 6 RV10 tests and F8's tests in a round 29. The writer answers the findings that each step gives.
- **Needs first.** F8 for the round's tests. GR2 for `union_def`.
- **Cost.** 0.25 writer session for the ledger rows and answers.
- **Risk.** Low for the writer. Each round keeps `frontend_check.py` at 0 worse, and a listed test that fails is a worse status.
- **Unblocks.** The completion test, and step 4 (classes).

### Item 4. DQ. A narrow second review of the doubt-held questions

- **What.** A source reader reads the 44 questions whose **Waits for:** line names only a review (doubt), against the current text and tests, and says for each whether the doubt stands. The list comes from `writer-scratch/doubt_only.py`. The writer records each reading, settles each question with no doubt left under decision 0054, and repairs the text where the reader reads against.
- **Needs first.** Nothing. F8 touches Q-0447 and Q-0021, which are not on the list.
- **Evidence.** Section 1.5. Phases 15 to 17 settled one question, and 123 of the 202 open questions wait on the maintainer's rows.
- **Cost.** One source-reader session in two or three parts. 0.75 writer session.
- **Risk.** Medium. Many of the questions had their last reading in early November, and their rules changed since.
- **Unblocks.** The only large drop in the open count that does not wait on the maintainer.

### Item 5. PQ2 and PU4. Principles, then their use

- **What.** PQ2 adds one sentence to each of PRIM-P1, CTAD-P1, CTAD-P3, CTOR-P1, OPR-P3, DED-P2, TPL-P3, CE-P3, and JMP-P1. Rounds 13 and 14 give the first eight, and section 1.5 gives JMP-P1's. PU4 is a principle-use batch on Q-0424, Q-0427, Q-0432, and Q-0435.
- **Needs first.** Nothing for PU4. The OPR-P3 sentence comes after PU4, so that the batch uses the OPR-P3 that Q-0435 names.
- **Cost.** 0.5 session.
- **Risk.** Medium. A principle states a model that can echo the reference's organization. Each sentence cites the rule that already gives the case.
- **Unblocks.** Up to three settlements in phase 18 and ten in phase 19 (decision 0055).

### Item 6. RV11. Gap review of the changed text

- **What.** A source-reader review of F8's text, GR2's production and LIST-4 sentence, and PQ2's sentences, then the repairs and a recorded read of their diff. The prompt names the two new questions of F8 and the row 4 item of RV11-R-2.
- **Needs first.** F8, GR2, and PQ2.
- **Cost.** 0.5 writer session of repairs.
- **Risk.** Medium. UNEV-3's definition of an effect and TPL-4's friend sentence sit near the reference's own wording.
- **Unblocks.** A tree whose rule text a reviewer has read, for the rule IDs that the analysis of steps 3 and 4 cites.

### Item 7. RC4. Records

- **What.** Three records. The reader prompts name a path in the scratchpad for the report, and the integrator copies the file, so that a refused write does not route the report through a message. X24's 17 new and 2 changed prototype tests get their directory, so that the P22 prototype's impact diff is measured on them (report rw3 section 4). A split that a grid finds gets its question in the same commit as the text that rests on it, as decision 0064 asks of a line left open.
- **Needs first.** Nothing.
- **Cost.** 0.2 session.
- **Risk.** Low.

### What the rules need before the next steps

- **Before FE29.** F8's tests. None of the open rows waits only for the front end.
- **Before the rest of step 3.** GR2's `union_def`, and X26-2's iteration expression, so that the machine's discarded values have a rule to cite in each context.
- **Before step 4.** RV11-R-1's TPL-4 sentence, G183's LIST-4 sentence, and the casts paragraph of RV11-R-2, since classes and templates read them.
- **Before a Propose on P24 or P30.** Nothing new. P30 reads JMP-14 and FN-7, which F7 wrote and RV10 read.
- **Propose and Diagnose.** Unchanged. F8 changes text by sentences, rows, and examples. GR2 changes one production.

## 3. Out of scope

1. **No new topic.** Phase 17 added no rule and no page.
2. **Coroutines, modules, threads and the memory model, extended floating-point types, the preprocessor beyond `#include "file"`, and the general library stay out**, as the charter and decision 0051 say. X26-6's grid reads only the types of the language-support table.
3. **The maintainer's rows.** 123 of 202 open questions wait on rows 1 to 9 (61%), 138 with row 10 (68%). The request of 2026-11-10 has no answer.
4. **P26 itself.** X26 is a Propose exercise. F8 answers its findings on the current text and adopts none of its new rules. `operator,` stays overloadable, and COMMA-5 stays.
5. **The rewrite's code.** The writer's items add no test that needs the template model.
6. **Lexical structure.** GR2 writes one production that tests use. The roadmap's deferral of lexical grammar beyond G1 stands.
7. **MSVC and later versions** stay deferred (row 7).

## 4. Process changes

Ranked by writer time saved.

1. **A save step for reader reports** (item RC4). Thirteen reports in four phases came back as messages, four of them in phase 17.
2. **A reader keeps its probes as files with defines.** X26's seven probe files went into one grid and one run script with no rewriting, and every claim was checked in one run. The practice of rounds 13 and 14 held.
3. **A grid row whose first error is a parse error gets a second form.** RV11-R-8's declaration form gave Clang and EDG a different parse, and only a second grid showed that every compiler rejects the placeholder. A writer who adds a row for a reject test checks that each family's first error names the rule, or writes the form where the parse cannot change.
4. **A new split gets its question in the same item.** The RV10 repairs found splits in four rows and filed them under existing questions. F8 has two splits that no question holds. Filing each with the text keeps the ledger free of rows left open for a question.
5. **The close names the rules changed after the read of the repair diff.** The phase 17 close did, and `changed.py` from the read's commit confirms it in one line. The practice of round 14 held.

General knowledge used in this report: none. Every count comes from Tidal's files, its commit messages, scripts run on commits fb0bcf87, e3cb5a3c, 88d14ea2, and 7a849708, and the writer probes in `writer-probes/`.
