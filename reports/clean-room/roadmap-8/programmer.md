# Roadmap, round 8: what to do after phase 10, from a programmer's view

Date: 2026-11-05. Decision 0029. This report comes from a clean-room reader in the role of a C++ programmer who uses the specification to write and check programs.

**What the reader read.**
- AGENTS.md, CLAUDE.md, and the round 7 programmer report (`reports/clean-room/roadmap-7/programmer.md`) with its sketches.
- The charter, roadmap, status, style guide, and exercises catalog, decisions 0056 and 0057, and `documentation/decisions/pending.md`.
- Front-end report 22 with `conformance.tsv`, `examples.tsv`, and `ledger.md`, and the W18 and W19 reports.
- The headings and findings of the I3, R10, PU3, and X22 reports and of `roadmap-7/portfolio.md`, and the rules named below with their tests.
- `git log f0f53319..1e20bfa2` with commit bodies (21 commits; `f0f53319` is round 7 itself).
- The three 2026-11-05 gap reviews, through the commits that act on them.

The reader did not read the other round 8 reports. Commit read: `1e20bfa2`.

**What the reader did.** The reader built the front end from `1e20bfa2` into the session scratchpad and reran every round 7 sketch and reduction. The reader wrote six new programs (719 lines; one of them is two units and a header) and reduced each stop to a short program. Each program and reduction went through four checks:
- a build with host GCC 16.2.1 and Clang 22.1.8 (`-std=c++23 -pedantic-errors -Wall -Wextra`),
- a second build with `-fsanitize=undefined,address`,
- the EDG 7.0 container (`cpfe --c++23 --strict --no_code_gen --instantiate=used`),
- `tidal-fe run`.

The reader also ran 38 short programs of common forms, 8 programs with a common run-time bug, and 12 programs with a common compile-time mistake. The sketches, reductions, and result logs are in `sketches/` beside this report. All counts come from read-only `grep` and Python scripts on the tree at `1e20bfa2`. Statements that rest on general knowledge say so.

## 1. The state from a programmer's view

Phase 10 took round 7's items 1, 2, 3, 4, and 9. Every round 7 reduction gives the right answer in `tidal-fe run` except the declared-only placement form (section 1.3). The four round 7 programs give the right answer too. W18 and W19 wrote eight programs with the interpreter step, and front-end round 22 made all eight run.

The front end now runs most code that I write every day:
- It runs 37 of my 38 short programs of common forms. Round 7 ran 26 of 31.
- It runs four of my six new programs to the right result on the first try.
- It catches all 8 common run-time bugs that I planted and names a rule for each. The sanitizers catch 7 of them.

The front end now stops on forms that library-style code uses:
- a pack expansion inside a parenthesized initializer (`T x(args...)`, `: member(args...)`),
- a base pack written as a template-id (`leaf<I, T>{v}...`),
- braced initialization through a constructor template (`M m{2}`).

A forwarding constructor, an `emplace` function, and a tuple built by inheritance use these forms (general knowledge). No test under `tests/` uses them. A `grep` finds no test of the first two, and a rough `grep` finds none of the third. The PACK-P2 table lists call arguments and braced lists, and it has no row for the expression list of a parenthesized initializer.

The matrix splits in two new places that no test covers:
- A friend function defined in a nested class reads a private member of the enclosing class. EDG rejects this.
- A failing `consteval` converting constructor initializes a function parameter. EDG accepts this.

### 1.1 Counts

| Measure | Round 7 (`017b401`) | Now (`1e20bfa2`) | Source |
|---|---|---|---|
| Rule headers | 1697 (1682 in force, 15 withdrawn) | 1697 (1682 in force, 15 withdrawn) | `:::{rule}` headers and `:kind:` lines |
| Content pages | 85 | 85 | pages with a rule |
| Tests | 6619 | 6681 | files with `tidal-rules` under `tests/`, `.tu.cpp` units left out |
| Rules with one test, by test headers | 1 (CPY-28) | 0 | this report's script |
| Program tests | 85 (14 under `tests/programs/roadmap/`) | 97 (18 under `roadmap/`, 4 under `W18/`, 4 under `W19/`) | `tests/programs/` |
| Lines under `tests/programs/` | 10361 | 11438 | this report's script |
| Rules cited by a program test | 511 of 1682 | 533 of 1682 | `tidal-rules` in `tests/programs/` |
| Pages reached by a program test | 84 of 85 | 84 of 85 (`keywords.md` is the one left) | the pages of those rules |
| Program tests that run to a result in `tidal-fe` | 85 of 85 (9 `pass-partial`) | 97 of 97 (11 `pass-partial`, 6 `EDG-ONLY`) | `conformance.tsv` |
| Run tests on the interpreter | 1660: 1604 expected (9 partial), 0 wrong, 52 unsupported, 4 not run | 1696: 1652 expected (11 partial), 0 wrong, 40 unsupported, 4 not run | `status.md` |
| Write or Port sessions in the phase | 0 | 2 (W18, W19) | `exercises.md` section 8 |
| Exercises by use in the phase | I2 | R10, PU3, I3, W18, W19, X22 | `exercises.md` section 8 |
| Port suites | 0 of 8 | 0 of 8 | `exercises.md` section 3 |
| Pages with a Diagnose batch | 4 | 4 | `exercises.md` section 8 |
| Pages whose only Predict batch showed test names | 1 | 0 | `exercises.md` section 7 |
| Guides | 5 (last change 2026-10-31) | 5 (last change 2026-10-31) | `guides/` |
| Questions | 400: 172 open, 215 settled, 13 compiler defects | 411: 171 open, 227 settled, 13 compiler defects | `status.md` |
| *(referenced)* markers | 33 | 33 | `status.md` |
| Front end, full pass | 6362 of 6619 (6414 pass their check) | 6459 of 6681 (6499 pass their check) | `conformance.tsv` |
| Front end, run-unsupported | 52 | 40 | `conformance.tsv` |
| Front end, false rejects and wrong run results | 6 and 0 | 4 and 0 | `conformance.tsv` |
| Front end, missed rejects and unsupported checks | 118 and 79 | 116 and 60 | `conformance.tsv` |
| Passing rejects whose first error names a cited or linked rule | 1990 of 2593 | 2004 of 2609 | reports 21 and 22 |
| Example blocks | 219: 65 unmarked, 27 `fragment`, 112 `reject`, 15 `diverges` | 220: 65 unmarked, 28 `fragment`, 112 `reject`, 15 `diverges` | `% tidal-example` markers |
| Example blocks where `tidal-fe check` disagrees with the marker | 0; 2 unsupported | 0; 2 unsupported | `examples.tsv` |
| Tests that open `namespace std` | 222, 34 of them program tests | 233, 43 of them program tests | `grep` |
| Front-end findings with an open ledger row | no ledger | 4 (G4, G13, G25, G152) | `ledger.md` |

### 1.2 Round 7 items that phase 10 took and left

| Round 7 item | Phase 10 | Result |
|---|---|---|
| 1. Tests for the C++23 call forms | item T | Done. 18 tests, and the four programs as `r7-` program tests. |
| 2. A front-end round on the C++23 call forms | item FE (round 22) | Done. Every reduction runs. The declared-only placement form still stops at UNI-6, and it has no test (report 22, section 3). |
| 3. Text: CLS-13, LAM-15, LSUP-2 | item T | Done. CLS-13 has the deduced form and a table of `Self`. LAM-15 covers a template head and `auto...`. For return type `auto`, LSUP-2 says "before the defaulted declaration". Q-0402 holds the split. The LAM-18 note on Clang's unused-capture warning is not done. |
| 4. A Write wave on modern idioms | item E (W18, W19) | Done. 8 programs, 21 findings, 9 front-end faults in W18 and 3 in W19. |
| 5. Port suites | not taken | Decision 0057 defers Port again. |
| 6. A count of grammar forms by test and front-end result | not taken | Deferred. |
| 7. Diagnose batches with the front end's message | not taken | Deferred. |
| 8. Guides | not taken | Deferred. |
| 9. Records that drift | item R4 | Done. The Write rows take their interpreter results from `conformance.tsv`. R10 ran on the objects page. CPY-28 has a second test. |

Round 7 process changes 1, 2, and 4 are in decision 0057 or the tools. Change 3 (a front-end round reports grammar forms with no passing test) is not in the tools. Change 5 (the Write prompt asks for the persona's own forms) is not in the prompt.

### 1.3 The earlier sketches, rerun

Every round 7 sketch and reduction gives the right answer in `tidal-fe run`, except one:

- The 33 other single-file sketches and reductions give result 0. They include `grid.cpp`, `grid_two_overloads.cpp`, and `jobs.cpp`.
- `registry/` with `commands.cpp`, and `fnptr_other_unit/`, give result 0.
- `grid_first_try.cpp` gives result 5, as GCC and Clang do. `jobs_first_try.cpp` is rejected with SA-1.
- `ordering_declared_after_class.cpp` and `registry/main_first_try.cpp` are rejected with LSUP-2, as Clang and EDG reject them.
- `lambdas.cpp` and `lambda_explicit_object_recursion_constant.cpp` give "unspecified: result 0", with the note that the exploration stopped at its bound of 64 runs.
- `placement_new_declared_only.cpp` still gives "undefined [UNI-6]". GCC fails to link the program, and Clang runs it.

### 1.4 New programs

| Program | GCC, Clang, sanitizers | EDG 7.0 | `tidal-fe run` |
|---|---|---|---|
| `pipeline_first_try.cpp` (134 lines): a fixed vector filled from a braced list, filter and transform views with a sentinel type, adaptors joined by `\|`, concepts with requires-expressions, constrained `auto` parameters, CTAD | GCC rejects `__is_integral`; Clang exits 0 | rejects (line 19) | rejected (TR-12) |
| `pipeline.cpp` (137 lines): the same with my own `is_same_v` | exit 0, clean | rejects: a friend defined in a nested class reads a private member of the enclosing class | unspecified: result 0 (4 orders explored, all 0) |
| `fmt.cpp` (102 lines): a format string checked by a `consteval` converting constructor against the argument pack, `type_identity_t` to block deduction, `if constexpr` on own traits, a fold over `,`, a user-defined literal | exit 0, clean; GCC warns once (a lambda that is unused when the pack is empty) | accepts | result 0 |
| `fmt_wrong_count.cpp`: the same with one argument too few | both reject at the call | accepts | rejected (CEV-1, CE-2), which is correct |
| `owner.cpp` (140 lines): a move-only `Box<T>` with a constrained converting move from `Box<U>`, `explicit operator bool`, `[[nodiscard]]` `make_box` with forwarding, `override`, `final`, a covariant `clone`, `dynamic_cast`, and a vector in raw storage that moves on growth only when the move is `noexcept` | exit 0, clean | accepts | unspecified: result 0 (4 orders, all 0) |
| `config.cpp` (136 lines): an `enum class` bit mask with `constexpr` operators, `using enum` in a `switch`, a `Maybe<T>` with a deduced object parameter on `value()`, designated initializers, defaulted `==` and `<=>`, the tuple protocol through a member `get`, `constinit`, `if` with an initializer, a nested namespace definition | exit 0, clean | rejects (Q-0386: no `weak_ordering`) | unspecified: result 0 (16 orders, all 0) |
| `tasks.cpp` (145 lines): a tuple built by inheriting `leaf<I, T>...` with a deduction guide, an index sequence and `apply`, a lambda with an init-capture pack, a `Result<T>` with `and_then` and `transform` through a deduced object parameter, an exception hierarchy with a rethrow | exit 0, clean | accepts | rejected (CTOR-15), a false reject |
| `tasks_type_pack_bases.cpp`: the same with the leaves as a type parameter pack and a braced base initializer | exit 0, clean | accepts | unsupported: "constructor with an initializer the front end does not model" |
| `logger/main.cpp` with `sink.cpp` and `logger.h` (59 lines): a ring buffer class template in a header, an inline function with a static local, an inline variable, and a function defined in the other unit | exit 0, clean | accepts both units | result 0 |

My six programs had one mistake from memory. `pipeline_first_try.cpp` used the compiler builtins `__is_same` and `__is_integral` for traits. Clang accepts both, and GCC knows only the first. EDG and the front end reject both. KW-4 reserves names with two underscores and says nothing about builtins that an implementation provides. GCC named the cause. The front end's TR-12 message ("expected '(' or '{' after a type name") did not.

`pipeline.cpp` meets a matrix split that I did not expect. ACC-5 gives a friend the access of a member. ACC-10 gives a nested class access to the enclosing class. Read together, the two rules let a friend of the nested class read the enclosing class's private member. GCC, Clang, and the front end accept the program. EDG rejects it.

### 1.5 Reductions

Each run reduction builds and exits 0 on GCC and Clang with both sanitizers, and EDG accepts it, except where the table says otherwise.

| Reduction | Lines | `tidal-fe` | What the specification says | Tests that use the form |
|---|---|---|---|---|
| `paren-pack/variable_paren_pack_int.cpp`, `variable_paren_pack.cpp`, `variable_paren_pack_nontemplate_class.cpp`: `int v(a...);` and `T x(a...);` in a function template | 2 to 3 | rejected, PACK-8, a false reject | PACK-P2 lists call arguments and braced lists. No row names a parenthesized initializer. `new T(a...)`, `T(a...)`, and `T x = T(a...)` run (`paren-pack/`). | none, by `grep` |
| `member_init_pack_argument.cpp`, `base_template_id_mem_init_pack_argument.cpp`, `mem_init_pack_argument_partial_spec.cpp`: `: p(a...)` and `: base<T...>(v...)` | 9 to 14 | unsupported: "parse: expected ')' but found '...'" | PACK-P2 has a row for `D() : B(0)...`, which expands a whole member initializer. It has no row for an expansion inside one. The braced form `: p{a...}` runs. | none, by `grep` |
| `mem_init_pack_argument_constexpr.cpp`, `one-line/forward_ctor_parens.cpp`: the same in a `constexpr` constructor or a constructor template | 4 to 13 | rejected, TR-12, a false reject | as above | none |
| `base_pack_mem_init.cpp`: `struct all : leaf<I>... { all(int v) : leaf<I>{v + I}... {} };` | 11 | rejected, CTOR-15 ("'leaf' is initialized two times"), a false reject | PACK-14. `base_pack_mem_init_type.cpp` (a type parameter pack) and `base_type_pack_mem_init_one.cpp` (one element) run. | 2 PACK-14 tests, both with a type parameter pack |
| `braced_init_constructor_template.cpp`, `braced_init_constructor_template_constant.cpp`, `braced_base_init_constructor_template.cpp`: `M y{2};` where `M` has a constructor template, at run time, in a constant, and as a base initializer `M{v}` | 5 to 8 | unsupported: "constructor call" or "constructor with an initializer the front end does not model" | The list initialization rules choose the constructor as for parentheses. `M y(2)` runs. | none found by a rough `grep` |
| `nested_friend_reads_enclosing.cpp`: a friend defined in a nested class reads a private member of the enclosing class | 13 | result 0 | ACC-5 and ACC-10 together make it valid. EDG rejects; GCC and Clang accept. | none |
| `consteval_converting_constructor_not_constant.cpp`, `consteval_template_converting_constructor_not_constant.cpp`: a `consteval` converting constructor that fails, called to initialize a function parameter | 10 to 14 | rejected, CEV-1 and CE-2, which is correct | CEV-1 counts an implicit conversion through a `consteval` constructor. GCC and Clang reject. EDG accepts. EDG rejects the same failure when it initializes a variable (`consteval-converting-constructor-throws.cpp`). | 2 tests, both of the variable form |
| `overload_set_generic_body_invalid.cpp`: an overload set of a plain lambda and a generic lambda whose body is not valid for the plain lambda's argument type (G152) | 12 | unspecified: result 0 (2 orders, both 0) | DED-29 and LAM-15 do not say whether forming the candidate instantiates the body (report 22, G152). GCC, Clang, and EDG all accept the program, and GCC and Clang run it. | `closure-as-base-overload-set.cpp`, `W18/visitor.cpp` |
| `mistakes/designator_out_of_order.cpp`: `P p{.y = 1, .x = 2};` | 2 | rejected, LIST-14, which is correct | LIST-14. GCC and EDG reject. Clang rejects under `-pedantic-errors` alone, and only warns when `-Wall` is also given, in either order. | the LIST-14 order tests, built without `-Wall` |

### 1.6 Short programs, planted bugs, and planted mistakes

**Short programs.** All 38 short programs of common forms in `one-line/` build and run clean on GCC and Clang. EDG accepts 37 and rejects one for Q-0386. The front end runs 37. The forms are:
- structured bindings on an aggregate, on an array, and in a range-based `for`,
- init-captures with a move and with a pack,
- `if constexpr` with a requires-expression, and `decltype(auto)`,
- conditional `noexcept` and `explicit(bool)`,
- an immediately invoked lambda that calls a `consteval` function,
- an `std::initializer_list` constructor,
- `throw` in a conditional, and `using enum` at namespace scope,
- `<=>` on `double`,
- aggregate CTAD, parenthesized aggregate initialization, and nested designators,
- `char8_t`, folds, a requires clause on a member function, and a template template parameter,
- a lambda in an unevaluated operand, and nested requirements,
- `new T{a...}`, and a pointer to member in a template,
- a `constexpr` virtual destructor,
- an overload set built from lambdas with CTAD.

The one stop is `forward_ctor_parens.cpp`, the forwarding constructor of section 1.5.

**Planted run-time bugs.** The front end stops each of the 8 bugs in `bugs/` with `undefined` and a rule:

| Rule | Bugs |
|---|---|
| OBJ-2 | a reference from a deduced-`this` accessor on a temporary; a reference into storage that a growth freed; a lambda that captured a dead local by reference; a view of a temporary's buffer |
| PFX-4 | an index past the end |
| UN-4 | negating the lowest `long` |
| VIRT-12 | a delete through a base without a virtual destructor |
| UNI-6 | a read of an inactive union member |

The sanitizers stop 7 of them. No sanitizer reports the inactive union member.

**Planted compile-time mistakes.** The front end rejects all 12 mistakes in `mistakes/`. GCC rejects all 12. Clang rejects 11 under the Write prompt's flags; the twelfth is the designator of section 1.5. The front end's first error names the rule of the cause in 10. The two others:

- A missing `typename` before `T::type x` gives TR-12 ("expected ';' but found 'x'") first and DEP-19 second. Both compilers name `typename` in their first error.
- A call that fails a constrained `auto` parameter gives DED-35 ("deduction fails for each template"). The message does not name the constraint.

### 1.7 Records that drift

- `status.md`, section "Known limits of the tooling", says that tests added after front-end round 21 have no row in `conformance.tsv` and that round 22 takes them. Round 22 is done, and its `--update` wrote the rows.
- R10's findings R10-1 to R10-9 have no record of action. Commit `7e4ddf7c` says the writer acts on them in RV3's repair pass. No later commit, page, or question names them, and the objects page did not change in phase 10. The front-end ledger covers G findings only.
- Commit `7b9cd715` left two round 7 doubts for RV3: the declared-only placement form and the LAM-18 note. No RV3 commit names them.

## 2. Ranked work items

| # | Item | Evidence | Size | Prerequisites |
|---|---|---|---|---|
| 1 | Tests for the initializer forms with packs, and the two new matrix splits | 4 front-end faults on forms that no test uses; 2 EDG splits with no test; each reduction builds clean | small | none |
| 2 | Text: pack expansion in initializers, a friend of a nested class, G152 | PACK-P2 has no row for a parenthesized initializer; ACC-5 with ACC-10 decides a split that no rule names; the matrix agrees on G152 | small | none |
| 3 | A front-end round on library-style initialization | 3 false rejects (PACK-8, TR-12, CTOR-15) and 2 stops in 719 lines; 1 of 38 short programs stops | medium | item 1 |
| 4 | Port suites, starting with two | 0 of 8 since phase 5; deferred by decisions 0054 to 0057; the front end now runs 37 of 38 common forms | one session per suite | the Write prompt changes of section 4 |
| 5 | A Write suite on library-shaped code | every new stop is in a forwarding constructor, an emplace, or a tuple; W18 and W19 did not reach these forms | one session per suite | item 2 for the pages |
| 6 | Diagnose batches with the front end's message | 2 of 12 planted mistakes name another rule first; 605 of 2609 passing rejects do the same (report 22) | one session per 4 pages | none |
| 7 | A count of grammar forms by test and front-end result | each new front-end fault is a form no test uses, as in round 7 | small script | none |
| 8 | A findings ledger for every exercise, and the records that drift | section 1.7 | small | none |
| 9 | Interpreter output that says when every order agrees | 5 of 6 new programs print "unspecified: result 0" when every explored order gives 0; 11 program tests stop at the bound of 64 runs | small | none |
| 10 | Guides on callables, owners, and raw storage | deferred since phase 5; `owner.cpp`, `fmt.cpp`, and `tasks.cpp` are what such guides would show | one decision, then one session per guide | maintainer |

### Item 1. Tests for the initializer forms with packs, and the two new matrix splits

**What.** A clean-room writer adds tests from the reductions of section 1.5, each probed on the matrix:

- a pack expanded in a parenthesized initializer, under PACK-13 and PACK-14, as run tests and in a `constexpr` constructor. The initializer is one of these:
  - a variable's initializer (`T x(a...)`),
  - a member initializer (`: m(a...)`),
  - a base initializer (`: B<T...>(v...)`),
- a base pack written as a template-id with a constant pack (`leaf<I, T>{v}...`) in a member initializer, under PACK-14,
- braced initialization of a class through a constructor template, at run time, in a constant, and as a base initializer, under the list initialization rules,
- a friend defined in a nested class that reads a private member of the enclosing class, under ACC-5 and ACC-10, with an EDG override and a new question,
- a failing `consteval` converting constructor that initializes a function parameter, under CEV-1, with an EDG override and a new question,
- the G152 reduction, as a run test under OVL-27 and LAM-15.

The integrator copies `pipeline.cpp`, `fmt.cpp`, `owner.cpp`, `config.cpp`, `tasks.cpp`, and `logger/` into `tests/programs/roadmap/` as `r8-` program tests. `pipeline.cpp` needs an EDG override for the nested friend, and `config.cpp` needs one for Q-0386.

**Why.** Section 1.5. A forwarding constructor is the common way to write `emplace` or a wrapper (general knowledge). No test of 6681 has a pack inside a parenthesized initializer. Decision 0055 asks the commit that acts on a roadmap round to add each correct sketch and reduction.

**Size.** Small. Each reduction has at most 14 lines.

**Prerequisites.** None. The tests give item 3 its targets.

### Item 2. Text: pack expansion in initializers, a friend of a nested class, G152

**What.**

- PACK-P2's table gains rows for the expression list of four initializers: a parenthesized initializer (`T x(p...)`), a member initializer (`: m(p...)`), a `new` initializer, and a functional cast. PACK-13 names them with the call arguments. An example shows a forwarding constructor.
- ACC-5 or ACC-10 says what a friend declared in a nested class may access in the enclosing class. A question records the EDG split and the provisional answer.
- CEV-1's Divergences part names the EDG split on a parameter, once the test exists.
- DED-29 or LAM-15 answers G152 with the matrix result. GCC, Clang, and EDG accept an overload set where a generic lambda's body is not valid for an argument that a plain lambda wins.
- The two round 7 leftovers: the LAM-18 note on Clang's unused-capture warning, and the declared-only placement form (LSUP-7 or ODR-8).

**Why.** Section 1.5. A programmer who reads PACK-P2 to check `: value(args...)` finds call arguments and braced lists, and has to guess that a parenthesized initializer counts. The nested friend has the shape of an iterator's `==` against a sentinel (general knowledge). The text decides the case only by joining two rules.

**Size.** Small: one table, two sentences, two questions.

**Prerequisites.** None.

### Item 3. A front-end round on library-style initialization

**What.** Front-end round 23 takes these targets, in this order:

1. the false rejects: PACK-8 on `T x(a...)`, TR-12 on `: m(a...)` in a `constexpr` constructor or a constructor template, and CTOR-15 on `leaf<I, T>{v}...`,
2. the parse stop on `: m(a...)` outside those cases,
3. braced initialization through a constructor template,
4. the declared-only placement form, which should name the missing definition and not UNI-6,
5. the 4 remaining false rejects, the 40 run-unsupported tests, and the 116 missed rejects, grouped as report 22 groups them.

**Why.** Sections 1.4 to 1.6. The front end runs nearly every C++20 and C++23 form I tried. Each stop that remains is a form of library-shaped code, and three of the stops reject valid programs.

**Size.** Medium. The parse of an expansion inside an initializer touches every initializer form.

**Prerequisites.** Item 1, so that `tools/frontend_check.py` shows each fix.

### Item 4. Port suites, starting with two

**What.** Port suites P1 and P2 of `exercises.md` section 3, with the interpreter step and with the prompt changes of section 4. A Rust persona and a Java or C# persona come first. Their idioms map to the forms of `owner.cpp`, `tasks.cpp`, and `config.cpp` (general knowledge): a trait object or interface, a generic container, a result type, and a closure with generic parameters.

**Why.** Port has waited since phase 5, and decisions 0054 to 0057 each deferred it. Round 7 held it back because the front end stopped on the C++23 call forms. The front end now runs 37 of my 38 short programs and four of my six programs. Write suites W18 and W19 met the call forms. A Port reader starts from a program in another language and meets the forms that a translation produces, and no Write suite has tried those forms.

**Size.** One session per suite.

**Prerequisites.** The prompt changes of section 4.

### Item 5. A Write suite on library-shaped code

**What.** Write suite W20 (persona C# or Java) with four tasks:
- a `Vec<T>` with `emplace_back` that forwards its arguments,
- an owner with a converting move,
- a tuple with `apply`,
- a format function checked at compile time.

**Why.** All my stops are in this kind of code. W18 and W19 wrote callables and compile-time code, and neither wrote a forwarding constructor. The tasks also reach the nested-friend split and the `consteval` parameter split.

**Size.** One session.

**Prerequisites.** Item 2, so that PACK-P2 covers the forms the tasks need.

### Item 6. Diagnose batches with the front end's message

**What.** Diagnose batches for templates, dependent names, and constraints. Each batch shows the compiler message and the `tidal-fe check` message, and scores whether the message leads the reader to the rule.

**Why.** Section 1.6 shows two weak messages. A missing `typename` gives TR-12 first, and a failed constraint gives DED-35 with no word on the constraint. 605 of 2609 passing rejects name another rule or none first (report 22). Diagnose batches have covered 4 pages since phase 5.

**Size.** One session per 4 pages.

**Prerequisites.** None.

### Item 7. A count of grammar forms by test and front-end result

**What.** Round 7 item 6, unchanged. A script lists each production of the Syntax blocks, the tests that match it, and their front-end status. A later front-end option prints the productions that a check derives.

**Why.** Each new fault is a form that no test uses: a pack inside a parenthesized initializer, a template-id base pack, and a braced list through a constructor template. A count of productions finds such a gap before a program does.

**Size.** A small script.

**Prerequisites.** None.

### Item 8. A findings ledger for every exercise, and the records that drift

**What.** Each finding of an exercise report (R, PU, I, W, X, D) gets a row, in `ledger.md` or in a ledger beside it. The row says one of four things: a commit acted on the finding, a question holds it, the text decides it, or no action was taken, with the reason. `check_specification.py` fails when a report names a finding with no row. The `status.md` paragraph of section 1.7 is fixed, and `status_table.py` writes it.

**Why.** Section 1.7. R10-1 to R10-9 have no record. The W18 and I3 repair commits give each finding a fate, and no commit does so for R10. G findings already have this check.

**Size.** Small.

**Prerequisites.** None.

### Item 9. Interpreter output that says when every order agrees

**What.** When the exploration ends and every outcome is the same, `tidal-fe run` prints the result first and the choices after, such as "result: 0 (every order of 4 gives 0)". When the bound stops the exploration, the line says so first. The front end raises the bound or prunes orders that touch no shared object, so that fewer program tests are `pass-partial`.

**Why.** Five of my six programs print "unspecified: result 0". A call's arguments can be evaluated in either order, and no order changes the result. A programmer reads "unspecified" as a warning about the program. 11 of 97 program tests stop at the bound of 64 runs.

**Size.** Small for the message. The bound belongs to a later round.

**Prerequisites.** None.

### Item 10. Guides on callables, owners, and raw storage

**What.** A maintainer decision on publishing guides, then how-to guides for an owner type with a converting move, a vector in raw storage, and a format function checked at compile time.

**Why.** Rounds 3 to 7 asked for guides. `owner.cpp`, `fmt.cpp`, and `tasks.cpp` are what such guides would show.

**Size.** One decision, then one session per guide.

**Prerequisites.** The maintainer.

## 3. Out of scope

| Topic | Proposal | For | Against |
|---|---|---|---|
| The general library, including `std::forward`, `std::move`, `std::exchange`, type traits, and ranges | Keep out. Programs define their own, as `owner.cpp` and `tasks.cpp` do. | Every program of mine needs them (general knowledge). | The charter leaves out the library. Each took a few lines to write. |
| Compiler builtins such as `__is_same` | Keep out. A note under KW-4 could say that they are extensions. | Programmers who know the library's internals reach for them (general knowledge). | They differ by compiler, as section 1.4 shows. |
| `typeid` | Keep deferred. | | Each test needs the library's `type_info` (Q-0322). |
| The preprocessor beyond `#include "file"` and the conditional groups | Keep out. | `#pragma once` and `assert` are common (general knowledge). | `logger.h` works with an include guard. |
| Threads, the memory model, coroutines, modules, extended floating-point types | Keep out. | | as the charter says |
| Language versions after C++23 | Keep out until `pending.md` row 7 is decided. | | Strict mode rejects them, and the matrix follows C++23. |

## 4. Process changes

1. **The Write and Port prompt builds once with `-pedantic-errors` alone.** Under `-Wall`, Clang only warns on out-of-order designators, which LIST-14 rejects. The matrix builds without `-Wall`, so a reader and the matrix can disagree on the same program.
2. **The Write and Port prompt asks for the persona's own forms** (round 7 change 5), such as a generic closure, a method on `Self`, a forwarding constructor, or a result type.
3. **Each exercise finding has a ledger row** (item 8), as each front-end finding has one under decision 0057.
4. **A roadmap reader's whole sketches become program tests in the next phase,** as item T did for round 7.
5. **A front-end round reports grammar forms with no passing test** (item 7), beside its status counts.

## 5. Suggested order

1. Items 1, 2, and 8 come first. Each is small, and item 3 needs item 1's tests. Item 7's script runs beside them and can add targets to item 3.
2. Item 3 runs next.
3. Item 4 runs after the prompt changes and beside item 3. Its reductions feed the round after item 3.
4. Item 5 follows item 2.
5. Items 6 and 9 run beside the others at any point. Item 10 waits on the maintainer.

## 6. Doubts for the clean-room writers

- Whether PACK-13's "arguments of a call" is meant to cover the expression list of a parenthesized initializer, a member initializer, and a `new` initializer. Every compiler accepts all of them. The front end accepts only the `new` and functional cast forms.
- Whether a friend function defined in a nested class has the enclosing class's private access through the nested class (ACC-5 with ACC-10). GCC and Clang say yes, and EDG says no.
- Whether EDG's acceptance of a failing `consteval` converting constructor that initializes a parameter is a defect or a reading of when the immediate invocation happens. EDG rejects the same failure for a variable.
- Whether the matrix settles G152. All three compilers accept my reduction, in which the generic lambda's body is not valid for the argument that the plain lambda wins.
- Whether Tidal should record that a reject depends on Clang's warning flags, as LIST-14 does under `-Wall`.
- Whether a declared, undefined placement form belongs under ODR-8 or LSUP-7, and what the interpreter should print (round 7, still open).
- Whether "unspecified: result 0" should stay the first word when every explored order gives the same result (round 7, still open).
