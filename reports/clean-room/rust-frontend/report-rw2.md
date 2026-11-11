# Front end, rewrite: the parser functions for the grammar gaps G166 to G179 and the four more forms

Date: 2026-11-11. Decision 0065 (phase 17, item GR, part 2).
Role: clean-room reader in the front-end role. Inputs: AGENTS.md, CLAUDE.md, decision 0065, `frontend/DESIGN-rewrite.md`, the crate `frontend/next/`, `reports/clean-room/rust-frontend/report-rw1.md`, the G rows of `reports/clean-room/rust-frontend/ledger.md`, the Syntax segments under `specification/` as commit `1c223631` (item GR, part 1) left them, and the tests under `tests/`. No compiler was run, and no crate was added. Commit read: `1c223631`.

The changed files are 40 files under `frontend/next/src/` (section 2) and this report. No file outside `frontend/next/` changed.

The reader returned this report as a message, and the integrator saved it here.

## 1. Conformance

### Method

`tools/frontend_check.py --next` ran under the run lock on `1c223631`, once before the change and once after it. No run used `--update`. The old front end stays the front end of record, and its four lines are the same in both runs:

```
frontend: 6974/7082 pass, 6997/7082 pass their check, 0 worse, 0 better, 0 moved between passing statuses, 9 new (2 not passing)
frontend: 22/22 interpreter-only expectations met
frontend: completion (decision 0060): 76 tests fail their check, of which 36 need the template model (marks file) and 40 count toward the gate of decisions 0062 and 0063; 0/166 program tests do not run to their result; 9 tests out of scope
frontend: 2154/2782 passing rejects name a cited rule (1966) or a linked rule (188) in their first error; `tidal-fe conform` gives the table per page
```

### Counts

Before the change (part 1's text, the parser of report-rw1):

```
next: unit: 87 tests pass, 1 fail
next: lex: 7200/7200 files give the old front end's tokens
next: parse: 6475/6922 tests parse as their outcome needs
next: parse: 159 of the 447 disagreements are reject tests that stop on the line of the old front end's first error
next: owned: 0 tests (takeover list)
```

The failing unit test was `each_nonterminal_has_one_function_in_its_page`. Part 1 added three nonterminals, so the Syntax segments held 260 and the crate had functions for 257.

After the change:

```
next: unit: 107 tests pass
next: lex: 7200/7200 files give the old front end's tokens
next: parse: 6764/6922 tests parse as their outcome needs
next: parse: 142 of the 158 disagreements are reject tests that stop on the line of the old front end's first error
next: owned: 0 tests (takeover list)
```

| Measure | Before | After |
|---|---|---|
| `next: unit` | 87 pass, 1 fail | 107 pass |
| Nonterminals with one function in their page's `syntax.rs` part | 257 of 260 | 260 of 260 on 62 pages, by the unit test |
| `next: parse` | 6475/6922 | 6764/6922 (289 more) |
| Disagreements | 447: 225 that need a tree, 222 reject tests | 158: 6 that need a tree, 152 reject tests |
| The old front end's lines | | the same, 0 worse, no status changed |

## 2. What changed

Each new or changed production of part 1 has its function in its page's `syntax.rs` part, named after the production, with the production in its doc comment.

| Production | Page module | Change |
|---|---|---|
| `cast_type` | `expressions::casts` | takes `[nested_name] template_id` and `typename_specifier`. A template-id is a cast type when the oracle finds a class template or an alias template. Under a dependent qualifier, only in a position of DEP-20. Otherwise `functional_cast` fails, and the tokens are a call, as the casts page says. |
| `base_spec` | `classes::derived` | starts with `{attribute_spec}` and ends in `base_type` |
| `base_type` (new) | `classes::derived` | `[nested_name] (class_name \| template_id) \| decltype_spec`. A name followed by `<` here is a template-id without a question to the oracle, because a base names a class. |
| `mem_init_id` | `classes::constructors` | `identifier \| base_type`. A qualifier, a `<` after the name, or `decltype` selects `base_type`. |
| `dtor_call` (shape) | `classes::constructors` | recognizes `~` followed by a `class_name`, a `template_id`, or a `decltype_spec` in the callee's `unqualified_id` |
| `using_directive`, `namespace_alias` | `declarations::namespaces` | take `[nested_name] identifier` |
| `namespace_def` | `declarations::namespaces` | `["inline"] "namespace" {attribute_spec} [identifier]` or `"namespace" ns_path`. The first form keeps its one name in an `ns_path` node, so the declare pass reads both forms alike. |
| `using_enum_decl` (new) | `declarations::enumerations` | `"using" "enum" [nested_name] identifier ";"`. `block_decl` and `member_decl` call it. |
| `decl_specifiers` | `declarations` (index) | ends in `{attribute_spec}` |
| `block_decl` | `declarations` (index) | takes `using_enum_decl` |
| `parameter` | `declarations::functions::default_arguments` | starts with `{attribute_spec}` |
| `exception_spec` (new) | `declarations::exception_specifications` | `noexcept_specifier \| "throw" "(" ")"` |
| `function_suffix` | `declarations::declarators` | takes `exception_spec`. A `throw` with anything inside its parentheses is no `exception_spec`, so EXS-7's form stops there. |
| `static_assert_decl` | `declarations::static_assert` | takes adjacent string literals after the comma |
| `new_type`, `new_bounds` | `expressions::new_delete` | take attributes after the type and after each bound |
| `lambda_expr`, `lambda_template` | `expressions::lambdas` | the attributes move from `lambda_template` to `lambda_expr`, after the optional template head |
| `member_decl` | `classes::members` | takes `using_enum_decl`, `enum_opaque`, and `template_head member_decl`. The head's parameters are in a template scope for the member after it, which can have no decl-specifiers. |
| `init_stmt` | `statements::if_stmt` | takes `sb_decl`. `if`, `switch`, `for`, and the range `for` share the function. |
| `conversion_type` | `classes::conversion_functions` | `type_specifiers {ptr_part}` |
| `template_id` | `templates::arguments` | also takes an `operator_function_id` or a `literal_operator_id` before the argument list |
| `template_arg` | `templates::arguments` | takes `braced_init` |
| `unqualified_id` | `expressions::primary` | `"~"` takes a `class_name`, a `template_id`, or a `decltype_spec`. An operator function's or a literal operator's name followed by a whole argument list is a template-id. |
| `typename_specifier` | `templates::dependent_names` | takes `["template"] identifier` |
| `template_template_param` | `templates::declarations` | the default takes `[nested_name] ["template"] identifier` |

These changes outside the page functions follow from the new trees:

- `declarator_id` (`declarations::declarators`) reads an operator function's template-id through its `id_expression` alternative, for a declarator such as `S::operator()<int>`.
- `Parser::defer_body` (`syntax`) skips a member initializer whose base is named by `decltype`, with parentheses or braces after it. A `{` after the `)` of `decltype(...)` no longer begins the body. Q-0450 holds the braced form.
- The declare pass (`sem::names`) reads a base from `base_type`. A template-id base with arguments that name a template parameter, and a `decltype` base, are dependent for the pass, as a base under a dependent qualifier is. It reads the namespace of a directive or an alias from the node's own `nested_name` and identifier.
- The declare pass gives the scope of a specialization defined with a qualified name, such as `template <> struct std::tuple_size<Range>`, the qualifier's scope as its parent (NAM-21). The body then finds `size_t` in `std`. Two run tests needed this: `foundations/language-support/tuple-protocol-page-example.cpp` and `programs/roadmap/r5-prelude.cpp`.
- `syntax::tests` counts 260 nonterminals on 62 pages.

### Unit tests

19 tests are new, one or more in the `tests.rs` of each page that changed. They cover each new form:

- a class template's template-id and a typename specifier as a cast type, and a function template's template-id as no cast type,
- bases written as a template-id, a pack, `decltype`, or with an attribute,
- member initializers by template-id and by `decltype`,
- the destructor call shape with a template-id or `decltype`,
- the namespace forms with no qualifier and with an attribute,
- `using enum` in a namespace, a block, and a class, with an opaque enumeration in a class,
- `throw()`, and the rejection of `throw(int)`,
- a message of two literals,
- a conversion to a pointer to member,
- a braced template argument,
- the operator and literal operator template-ids,
- member templates with no decl-specifiers,
- a structured binding in the init-statement of `if` and `for`,
- `template` before a name with no list,
- the lambda attribute positions,
- the attributes in `new`,
- an attribute after the decl-specifiers, and an attribute before a parameter.

## 3. The G rows

Every test that disagreed before the change through one of these forms now parses as its outcome needs. No remaining disagreement (section 4) comes from one of these forms. Each row can move to `commit` with this item's commit.

| Row | Form | State |
|---|---|---|
| G166 | a template-id as a cast type (`Box<int>{1}`) | commit: every test of the form parses |
| G167 | a base or a member initializer by template-id or `decltype` | commit: every test of the form parses. `expressions/decltype/member-initializer-base-braces.cpp` (Q-0450) parses too. |
| G168 | `using namespace lib;` and `namespace n = m;` | commit: every test of the form parses |
| G169 | `using enum` | commit: every test of the form parses, `using-enum-qualified.cpp` included |
| G170 | attributes after the decl-specifiers, before a parameter, before a base | commit: every test of the form parses, both attribute-position tests included |
| G171 | an operator's template-id, and a destructor name by template-id or `decltype` | commit: every test of the form parses |
| G172 | an opaque enumeration declaration in a class | commit: every test of the form parses |
| G173 | a member template with no decl-specifiers | commit: both tests parse |
| G174 | a structured binding in an init-statement | commit: every test parses, the three `init-structured-binding` tests of `if`, `for`, and `switch` included |
| G175 | `throw()` | commit: the three tests parse |
| G176 | a `static_assert` message of two literals | commit: the test parses |
| G177 | a pointer to member in `conversion_type` | commit: both tests parse |
| G178 | a braced list as a template argument | commit: both tests parse |
| G179 | `template` and a name with no argument list | commit: the three tests parse |

The four more forms of part 1 parse too:

- the attribute after `namespace`, and after the type and the bound in `new` (`attribute-positions.cpp`, `attribute-positions-base-and-new-bound.cpp`),
- the lambda attribute with no template head (`expressions/lambdas/attribute-without-template-head.cpp`),
- `typename T::type{...}` as a cast (`templates/dependent-names/typename-functional-cast.cpp`).

## 4. The remaining disagreements

158 tests disagree. Each has one of these causes.

| Cause | Tests that need a tree | Reject tests |
|---|---|---|
| `<cstdarg>`, out of scope (G158): `expressions/postfix/ellipsis-argument-conversions.cpp`, `ellipsis-nullptr-becomes-void-pointer.cpp` | 2 | 0 |
| The step 2 oracle: an alias template of the current instantiation (`current-instantiation-alias.cpp`), and the marked test `sizeof-value-dependent-operand-no-typename.cpp`, as in report-rw1 | 2 | 0 |
| The step 2 oracle: a class template's own name in its base clause. In `template <class T> struct ArraySource : Enumerable<ArraySource<T>>`, the declare pass learns `ArraySource` only after the class head, so the inner `ArraySource<T>` is no template-id (`programs/P3/pipeline-first-try.cpp`) | 1 | 0 |
| A grammar gap (finding RW2-1, section 6): `union_def` takes a `class_name` and no template-id, so a union's partial specialization is not derived (`programs/roadmap/r10-variant.cpp`) | 1 | 0 |
| A form the grammar does not derive, where the old front end's first error names a more specific rule | 0 | 145 |
| The old front end's first error names TR-12, and the grammar derives the form: DEL-3 (2 tests), DTY-5 (2), TYN-10 (1), DEP-24 (the oracle point of report-rw1, 1), DEP-26 (1) | 0 | 7 |
| **total** | 6 | 152 |

Of the 145 reject tests in the fifth row, 142 stop on the line of the old front end's first error. The other 3 stop one line from it. Two FN-9 tests stop at the `}` after a definition in a block. `template-keyword-in-declarator-class.cpp` stops at `template` in a class head, one line before the old error.

Two reject tests now disagree that agreed before: `declarations/namespaces/nested-first-inline.cpp` and `nested-first-inline-both.cpp`. Part 1's `namespace_def` takes `inline` only before a name of one identifier, so `inline namespace a::b {}` is not derived, and the parser stops at `::`. The stop is on the line of the old front end's first error, which names NS-11. The fifth row counts both.

The reject rows are not findings, for the reasons report-rw1 section 4 gives.

## 5. Faults with no test

None found.

## 6. Findings

| Finding | Pages | Kind | What happened | Suggestion |
|---|---|---|---|---|
| RW2-1 | `classes/unions` | grammar gap | `union_def` takes `[class_name ["final"]]`, so `template <class T, class... Ts> union Storage<T, Ts...> { ... };` is not derived. `class_head` takes `[nested_name] (class_name \| template_id)`. The run test `programs/roadmap/r10-variant.cpp` uses the form. The reject tests `templates/specialization/explicit-union-key.cpp` and `partial-union-key.cpp` (TSP-32) use it before their error. | Let `union_def` take `[nested_name] (class_name \| template_id) ["final"]`, as `class_head` does. A new front-end ledger row, numbered by the integrator. |
| RW2-2 | `foundations/names` | oracle point | The step 2 declare pass declares a class after its whole head. A class template's own name with an argument list in its base clause is then no template-id. | Analysis at step 3 answers it. The text gives no point of declaration for a class name, so the point goes to `DESIGN-rewrite.md` section 10 with the other oracle points. |
| RW2-3 | `declarations/namespaces` | editorial | NS-11 rejects `inline` before a nested namespace definition, and `namespace_def` no longer derives that form. The rule and the grammar both reject it. | None needed. The parse target reads the case from the old front end's first error. |

## 7. Ledger rows

G166 to G179 can move from `open` to `commit` with this item's commit (section 3). This report does not edit the ledgers.

## 8. Tests added during the round

None. The unit tests of `frontend/next/` are not conformance tests.
