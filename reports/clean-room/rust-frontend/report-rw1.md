# Front end, rewrite steps 0 to 2: the design brought to the marks file, the crate with its lexer, and the parser with the `Names` oracle

Date: 2026-11-11. Decisions 0029, 0035, 0060, 0062, 0063, 0064 (phase 16, item RW).
Role: clean-room reader in the front-end role. Inputs: AGENTS.md, CLAUDE.md, decisions 0035, 0060, 0062, 0063, and 0064, `documentation/exercises.md` section 9 ("Front end"), `frontend/README.md`, `frontend/DESIGN-current.md`, `frontend/DESIGN-rewrite.md`, `frontend/TEMPLATE-MODEL.md`, the marks file `reports/clean-room/rust-frontend/rewrite-marks.tsv`, report 26, the round 13 views `reports/clean-room/roadmap-13/implementer.md` (item 1 and its scratch scripts) and `designer.md` (item 1 and section 1.10), the front-end ledger, the review summaries of 2026-11-11 for the state of RV8, and the files under `specification/`, `tests/`, `questions/`, `frontend/`, and `tools/`. No compiler was run, and no crate was added. Commit read: `5f0ffdd9`; the counts are those of the tree rebased onto `1cc1696e`, after items T5 and F6.

The changed files are `frontend/README.md`, `frontend/DESIGN-current.md`, `frontend/DESIGN-rewrite.md`, `frontend/TEMPLATE-MODEL.md`, `frontend/src/main.rs`, `tools/frontend_check.py`, `reports/clean-room/rust-frontend/ledger.md`, and this report. The added files are `reports/clean-room/rust-frontend/takeover.tsv` and the crate `frontend/next/` (`Cargo.toml`, `Cargo.lock`, and 208 files under `src/`).

The reader returned this report as a message, and the integrator saved it here.

## 1. Conformance

### Method

`tools/frontend_check.py` and `tools/frontend_check.py --next` ran under the run lock on `1cc1696e` with this round's changes. No run used `--update`. The old front end stays the front end of record and judges every test (decision 0064), so no status and no baseline changed.

### Counts

Without `--next`, the tool prints these lines. The 54 new tests are those of items T5 and F6, and this item changes no status:

```
frontend: 6907/7061 pass, 6944/7061 pass their check, 0 worse, 0 better, 0 moved between passing statuses, 54 new (39 not passing)
frontend: 22/22 interpreter-only expectations met
frontend: completion (decision 0060): 108 tests fail their check, of which 32 need the template model (marks file) and 76 count toward the gate of decisions 0062 and 0063; 14/166 program tests do not run to their result; 9 tests out of scope
frontend: 2140/2768 passing rejects name a cited rule (1953) or a linked rule (187) in their first error; `tidal-fe conform` gives the table per page
```

The 39 `new test` lines come before them. With `--next`, the tool lists each parse disagreement (section 4) after those lines, and then prints:

```
next: unit: 88 tests pass
next: lex: 7179/7179 files give the old front end's tokens
next: parse: 6456/6894 tests parse as their outcome needs
next: parse: 159 of the 438 disagreements are reject tests that stop on the line of the old front end's first error
next: owned: 0 tests (takeover list)
```

| Measure | Target of implementer.md item 1 | Now |
|---|---|---|
| `next: lex` | all files (7179) | 7179/7179 |
| `next: parse` | all judged tests (6894), or a G row with a test and a cause for each disagreement | 6456/6894; section 4 gives a cause for each of the 438 |
| Nonterminals with one function in their page's `syntax.rs` part | 257 on 62 pages | 257 on 62 pages, by a unit test |
| `next: unit` | passes, with tests in each `syntax.rs` part and in `lex` | 88 pass |
| Files of `frontend/next/` over about 2000 lines | none | none; the longest is `expressions/primary/lexical.rs` at 856 |
| The old front end's line | 0 worse, no status changed | 0 worse, no status changed |
| The takeover list | exists, no row | exists, no row |

## 2. Step 0: the documents

- `DESIGN-rewrite.md` section 5 and its build table follow the marks file: 32 tests, 4, 6, 11, 4, and 7 by model section. Step 5 takes the 17 tests of sections 1, 2, and 5, and the marked count goes from 32 to 15. Step 6 takes the 11 of section 3 (to 4), and step 7 the 4 of section 4 (to 0). Section 4's row "6. The 32 tests" follows.
- The build order carries the Propose hook (designer.md item 1): the `--proposal` flag and an empty registry at step 1, the nonterminal point at step 2 with P22's syntax part as its unit test, the node-check point in section 7 before step 3, the `Ext` node point at step 3, rule overrides at step 4, and definition checks at step 5. Section 7 lists the fifth point (node checks, owned by `sem`) and states the impact diff's base.
- Section 10 drops the `forward_class` point (RM12-2, `d6fd9cea`) and the P3 point (round 26 passed the three tests). It also drops the point that the tool has no `--next`. It adds the points that step 2 found: the member-template question after `.` or `->`, the limits of the step 2 oracle, the parse targets of reject tests (section 4 of this report), and the run override that X25 and X1 would need.
- Section 2 adds two questions to the oracle: a member template after `.` or `->`, and the current instantiation before `::`. It also says how the parser reports declarations and scopes to the oracle.
- Section 3 gives three facts about the syntax parts. `expressions::primary` keeps its lexical productions in `lexical.rs`. A `shape` production's function is a recognizer over a tree. Four pages gain the syntax stage that the page table had left out: `foundations/constant-evaluation`, `classes/copy-move`, `templates/instantiation`, and `templates/class-template-deduction`.
- `TEMPLATE-MODEL.md` section 8 replaces the P3-6 row with Q-0441 and adds Q-0433. It replaces the RV8 row with RV9, which covers FN-12, INST-1's new clause, and the hunks of `bdc27d44` (decision 0064's item RV9 reviews them).
- `DESIGN-current.md` describes the code after round 26 (`a91ccc77`) and the `lex` subcommand: 52510 lines in 28 files. Three tables were counted again at `5f0ffdd9`: the file table, the pages each large file names, and the map by page. The method is the one the document states: the rule IDs of each page in each file, comments and diagnostics together. Seven files are still over 2000 lines.
- `frontend/README.md` gains a section on the rewrite.

## 3. Step 1: the crate, the lexer, and the driver

`frontend/next/` is a crate of its own (`tidal-next`) with its own `Cargo.lock` and `target/`, so the build of `frontend/` does not change. Its modules follow `DESIGN-rewrite.md` section 1:

| Module | Holds | From the old front end |
|---|---|---|
| `source` | `FileId`, `Span` (file, line, column), `SourceMap` with the include stack and the line in the unit's main file | the line of `lexer.rs` |
| `diag` | `Diag` with a span, `Diags` with no muted state, the printed form of the old front end | `diag.rs` |
| `lex` | the token stream, the preprocessing slice, the dump | `lexer.rs`, moved |
| `foundations::odr::syntax` | the 12 `preprocessing` productions, which `lex` calls for each directive line | the directive code of `lexer.rs` |
| `foundations::keywords` | KW-1's keywords, KW-2's spellings, KW-3's words | `alt_token` |
| `expressions::primary::lexical` | the lexical productions of the primary page, and the literal checks (PRIM-2, PRIM-5, PRIM-6, PRIM-16 to PRIM-19, PRIM-23, PRIM-26, OPR-8) | the literal code of `lexer.rs` |
| `proposals` | the registry of prototypes, empty | none |
| `main.rs` | `check`, `run`, `conform`, `dump`, `lex`, `parse`, and `--proposal` | `main.rs` |

The lexer moved with small changes. Each token now has a span. A token that `#include` brings in keeps its own file and line, and the dump and the diagnostics print the line of the `#include` in the main file, as the old front end did. The old `lexer.rs` steps past the end of the text when a quoted literal is not closed at the end of a file (section 5). The new copy stops at the end.

`tidal-fe lex FILE...` is new code in the old `main.rs`. It prints the same form as `tidal-next lex`: one line per token (kind, text, line, literal) and one line per lexer diagnostic. It changes no test result (section 1). The tool compares the two dumps for the 7061 test files and the 118 extra translation units, and they are equal for each.

The driver keeps the subcommands and exit statuses of `tidal-fe`. Analysis comes at step 3, so for now:
- `check` gives `reject` with a TR-12 error when the parser stops, and `unsupported` when the unit parses.
- `run` gives `unsupported`.
- `conform` refuses, because its harness moves with the machine.
- `--proposal ID` takes a prototype's alternatives from the registry, and refuses an ID that the registry does not hold.

## 4. Step 2: the parser and the `Names` oracle

### What is in place

The syntax part of every page with a Syntax segment is written: all 62 pages and 257 nonterminals, one function each, named after its production. The unit test `syntax::tests::each_nonterminal_has_one_function_in_its_page` reads the Syntax segments from `specification/`. It fails when a nonterminal has no function in its page's module, has two, or has a function in another page. It also fails when a function whose doc comment opens with a production is named after another one.

| Chapter | Pages | Nonterminals |
|---|---|---|
| foundations | 2: `constant-evaluation`, `odr` | 13 |
| expressions | 17: the 16 operator pages and `primary` (with `lexical.rs`) | 87 |
| statements | 7: `index` and 6 pages | 21 |
| declarations | 17: `index` and 16 pages | 60 |
| classes | 10 | 44 |
| templates | 7 | 27 |
| exceptions | 2 | 5 |

- **`syntax`.** It holds the cursor, the generic syn node, the `Names` trait, the stub oracle, the extension point, the deferred bodies, and the TR-12 stop. The cursor takes a `>` from `>>`, `>=`, or `>>=` at the end of a template argument list. A syn node has a production, a span, children, and the proposal of an `Ext` node. The stop is at the furthest token that the grammar did not derive. A level of the expression chain makes a node only when its operator is present.
- **Deferred bodies and scopes.** A member function body is kept as a token range until the outermost class closes. It is then parsed with the class complete (NAM-6). The parser tells the oracle about each declaration as soon as its declarator or head is read. It opens and closes the scopes of namespaces, classes, blocks, template heads, and prototypes.
- **`sem::names`.** This is the declare pass of step 2. It records names and their kinds, the bases of each class, using-directives, and inline and unnamed namespaces, and it answers the oracle's questions. A template-id with dependent arguments is read as the current instantiation in two cases: the template's definition is open, or the arguments are template parameters alone (a member defined outside its class).
- **Shapes.** The 15 `shape` productions, and the 10 productions that only shapes use, are recognizers over trees. Each has a unit test (`copy_ctor`, `ctor_def`, `dtor_call`, `typedef_decl`, `forward_class`, `specialization_class`, `anonymous_union`, `defaulted_comparison`, and the rest).
- **The nonterminal point.** A unit test registers P22's syntax part (`break` or `continue` with a label) at `jump_stmt`. Under that alternative the form gives an `Ext` node, and without it the form is a TR-12 stop. The real P22 prototype waits for item E's design.

The parser parses the 7179 files in about one second.

### The parse count's disagreements

438 tests disagree with the targets of implementer.md item 1. The "need a tree" column counts tests whose outcome is accept, run, run-fail, or link-fail. Each disagreement has one of these causes:

| Cause | Tests that need a tree | Reject tests |
|---|---|---|
| G166: `cast_type` has no template-id (`Box<int>{1}`) | 89 | 22 |
| G167: a base or a member initializer written as a template-id or with `decltype` | 48 | 27 |
| G168: `using namespace lib;` and `namespace n = m;` need a qualified name | 36 | 23 |
| G169: no production for `using enum` | 4 | 3 |
| G170: attributes after the decl-specifiers, before a parameter, or before a base | 5 | 2 |
| G171: a template-id after `operator()` or a literal operator's name, and a destructor name with a template-id or `decltype` | 10 | 1 |
| G172: no opaque enumeration declaration in a class | 4 | 0 |
| G173: a member template declared with no decl-specifiers | 2 | 4 |
| G174: no structured binding in an init-statement | 2 | 1 |
| G175: no `throw()` | 3 | 1 |
| G176: a `static_assert` message of two string literals | 1 | 0 |
| G177: no pointer to member in `conversion_type` | 2 | 1 |
| G178: no braced list as a template argument | 2 | 0 |
| G179: `template` and a name with no argument list | 3 | 2 |
| `<cstdarg>`, out of scope (G158) | 2 | 0 |
| The step 2 oracle: an alias template of the current instantiation (`current-instantiation-alias.cpp`), and the marked test `sizeof-value-dependent-operand-no-typename.cpp` (model section 1, step 5) | 2 | 0 |
| A form the grammar does not derive, where the old front end's first error names a more specific rule (KW-1, `register`, DEL-3, DTY-5, CNS-29, and others) | 0 | 126 |
| The old front end's first error names TR-12, and the grammar derives the form: `= delete("...")` as an initializer (DEL-3), `decltype` in a declarator's qualifier (DTY-5), `T::template plain<int>` (DEP-26), a fold in a requires-clause (CNS-24), `object->get<int>()` (DEP-24, the oracle point of section 5), and an attribute in an explicit instantiation's class head (TYN-10) | 0 | 8 |
| **total** | 215 | 223 |

The G causes are forms that the Syntax segments do not derive. An accept or run test uses each of them, or a reject test uses one before its error. The old front end parses each of these forms from general knowledge, so it gives no TR-12. The new parser follows the grammar and stops. Each G row is a finding against the text (section 10). Once the text derives a form, the parse count rises with no parser change beyond a function for the new production.

The reject rows are not findings:
- Where the grammar does not derive the form, a stop is right, and TR-12's text allows a diagnostic that names TR-12 alone.
- Where the grammar derives the form, a rule other than TR-12 rejects it, and the text has that rule.

The parse targets read both cases from the old front end's first error, so both count as disagreements. The tool's second parse line counts the reject tests whose stop is on the line of the old first error: 159 of the 438. A target that accepted such a stop would make them agree.

## 5. Faults with no test

- **The old lexer at the end of a file.** In `lexer.rs`, `lex_quoted` steps one place past the end of the text when a quoted literal is not closed at the end of a file, and the slice of the suffix panics. No test ends that way. The new copy stops at the end. The old code gets no change for this in phase 16, so the fault stays there.
- **The member-template question.** DEP-24 decides by the object's type whether `<` after a member name after `.` or `->` opens a template argument list. The parser has no types, so the step 2 oracle answers "a member template of this name is declared in some class". That is why `templates/dependent-names/template-keyword-needed-member.cpp` gives a tree. `DESIGN-rewrite.md` section 10 records the point. Analysis can answer it once the object's type is known.

## 6. Ledger rows of `reports/clean-room/ledger.md` that this round answers

None. Item RW answers no exercise ledger row.

## 7. Tests added during the round

None. The unit tests of `frontend/next/` are not conformance tests.

## 8. Decision 0060: the completion test

The completion line is the old front end's, and this item changes it in no way:
- 108 tests fail their check. 32 of them need the template model, and 76 count toward the gate's count, which decision 0064 makes a measure. The 28 more than at `5f0ffdd9` are new tests of T5 and F6.
- 14 of 166 program tests do not run to their result.
- 9 tests are out of scope.

The new front end owns no test. The marked tests stay with the old front end until step 5.

## 9. Run stops

This item changes no run. 34 tests pass their check and stop in a run (31 at `5f0ffdd9`), and 3 new tests give `RUN-WRONG-RESULT`.

## 10. Findings

The parse count finds 14 forms that tests use and that the Syntax segments do not derive. Each has a row in the front-end ledger in state `open`.

| Finding | Pages | Kind | What happened | Suggestion |
|---|---|---|---|---|
| G166 | `expressions/casts` | grammar gap | `cast_type` is a name, `decltype`, `auto`, or a keyword. A template-id followed by a braced list, such as `Box<int>{1}` or `std::initializer_list<M>{M{}}`, is not derived. A template-id followed by parentheses is derived only as a call. 111 tests | Let `cast_type` take `[nested_name] template_id`, and say which rules then read `Box<int>(1)` as a cast. |
| G167 | `classes/derived`, `classes/constructors` | grammar gap | `base_spec` and `mem_init_id` end in `class_name`, an identifier. These are not derived: a base `B<int>`, a pack of bases `Part<K>...`, a base `decltype(b)`, and a member initializer `B<T>(x)`. 75 tests | Let both end in a class name or a template-id, and let `base_spec` take a `decltype_spec`. |
| G168 | `declarations/namespaces` | grammar gap | `using_directive` and `namespace_alias` take a `qualified_name`, and a `qualified_name` needs a `nested_name`. So `using namespace lib;` is not derived. 59 tests | Let both take `[nested_name] identifier`. |
| G169 | `declarations/enumerations` | grammar gap | No production derives `using enum Color;`. 7 tests | Add a production to `block_decl` and `member_decl`. |
| G170 | `declarations/attributes` | grammar gap | These attribute positions are not derived: after the decl-specifiers (`int [[a]] c;`), before a parameter, and before a base specifier. 7 tests | Add `{attribute_spec}` at those places, or say which places Tidal leaves out. |
| G171 | `templates/arguments`, `classes/constructors` | grammar gap | `template_name` is an identifier, so `x.operator()<int>()`, `&S::operator()<long>`, and `operator""_count<'4', '0'>()` are not derived. `unqualified_id`'s `"~" class_name` does not derive `p->~Box<T>()` or `t.~decltype(t)()`. 11 tests | Let a template-id name an operator function, and let a destructor name take a template-id or a `decltype_spec`. |
| G172 | `classes/members` | grammar gap | `member_decl` has no `enum_opaque`, so `enum class Kind : short;` and `enum Kind : int;` in a class are not derived. 4 tests | Add `enum_opaque` to `member_decl`. |
| G173 | `templates/declarations`, `classes/members` | grammar gap | A member template's `declaration` derives a declaration with no decl-specifiers only as a definition. So `template <class T> operator long();` in a class is not derived. 2 tests | Let a member template's declaration be a `member_decl`. |
| G174 | `statements/if` | grammar gap | `init_stmt` has no `sb_decl`, so `if (auto [a, b] = s; b > 2)` is not derived. 2 tests | Add `sb_decl` to `init_stmt`. |
| G175 | `declarations/exception-specifications` | grammar gap | No production derives `throw()`, which EXS-5 and Q-0077 accept. 3 tests | Add `"throw" "(" ")"` to `function_suffix`, or a mark that names it. |
| G176 | `declarations/static-assert` | grammar gap | `static_assert_decl` takes one `string_literal`, so a message of two adjacent literals is not derived. 1 test | Take `string_literal {string_literal}`, as `literal` does. |
| G177 | `classes/conversion-functions` | grammar gap | `conversion_type` takes only `*`, `&`, and `&&` after its specifiers, so `operator int Base::*()` is not derived. 2 tests | Add `nested_name "*"`, as `ptr_part` has. |
| G178 | `templates/arguments` | grammar gap | `template_arg` has no `braced_init`, so `At<{3, 4}>` is not derived. 2 tests | Add `braced_init` for a constant parameter of class type. |
| G179 | `templates/dependent-names`, `templates/declarations` | grammar gap | `typename_specifier` and the default of `template_template_param` take a name after `template` only as a template-id. DEP-26 allows the name alone for a deduced class template or a template template argument (Q-0226). 3 tests | Let both take `["template"] identifier` where DEP-26 allows it. |
