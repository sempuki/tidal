# Roadmap, round 7: what to do after phase 9, from a programmer's view

Date: 2026-11-05. Decision 0029. This report comes from a clean-room reader in the role of a C++ programmer who uses the specification to write and check programs. The reader read AGENTS.md, CLAUDE.md, the round 6 programmer report (`reports/clean-room/roadmap-6/programmer.md`) and its sketches, the charter, roadmap, status, style guide, specification format, exercises catalog, decisions 0033, 0039, 0050, 0051, 0054, 0055, and 0056, and `documentation/decisions/pending.md`. The reader also read the headings and results of the I2, X21, R9, PU1, and PU2 reports, front-end reports 20 and 21 with `conformance.tsv` and `examples.tsv`, the headings of the three 2026-11-04 gap reviews of the text phase 8 wrote, `questions/README.md`, the rules named below with their tests, and `git log 1b16115..HEAD` with commit bodies (17 commits; `e2f0de5` is round 6 itself). The reader did not read the other round 7 reports. Commit read: `017b401`.

The reader built the front end from `017b401` into the session scratchpad. The reader reran the round 6 reductions and whole sketches, wrote four new programs (476 lines, one of them in two units), and reduced each stop to a short program. Each program and reduction was built with host GCC 16.2.1 and Clang 22.1.8 (`-std=c++23 -pedantic-errors -Wall -Wextra`, then `-fsanitize=undefined,address`), checked on the EDG 7.0 container (`cpfe --c++23 --strict`), and run with `tidal-fe run`. The reader also ran 32 one-line programs of common forms through the front end. The sketches and reductions are in `reports/clean-room/roadmap-7/programmer-sketches/`. All counts come from read-only `grep` and Python scripts on the tree at `017b401`. Statements that rest on general knowledge say so.

## 1. The state from a programmer's view

Phase 9 took every item that the round 6 programmer view ranked first to third, and item 7. The round 6 reductions and the correct whole sketches of rounds 4 to 6 are tests. Front-end rounds 20 and 21 made every program test run, removed every wrong run result, and made the front end agree with every page example marker it checks. The Write and Port prompt now has an interpreter step.

Phase 9 ran no Write and no Port session. Its exercise by use was I2, an Implement exercise. The program corpus grew from 71 to 85 only through the roadmap sketches.

My four new programs use forms that a programmer of C++23 writes every day (general knowledge): a member function with a deduced explicit object parameter, a function object with a call operator template, a generic lambda with a template head or an `auto...` pack, `auto(x)`, and a function pointer passed between units. The front end stops on each of these forms. No test under `tests/` uses three of them, and the tests of the other two fail on the front end. The rule text covers most of them. Three rules are thin for a programmer: CLS-13 for the deduced object parameter, LAM-15 for the lambda template head, and LSUP-2 for the point where the ordering class must be declared.

### 1.1 Counts

| Measure | Round 6 (`1b16115`) | Now (`017b401`) | Source |
|---|---|---|---|
| Rule headers | 1696 (1681 in force, 15 withdrawn) | 1697 (1682 in force, 15 withdrawn) | `:::{rule}` headers and `:kind:` lines |
| Content pages | 85 | 85 | pages with a rule |
| Tests | 6500 | 6619 | files with `tidal-rules` under `tests/`, `.tu.cpp` units left out |
| Rules with one test, by test headers | 2 (CPY-28, CPT-23) | 1 (CPY-28) | this report's script |
| Program tests | 71 | 85 (14 under `tests/programs/roadmap/`) | `tests/programs/` |
| Lines under `tests/programs/` | not measured | 10361 | this report's script; `propose_check.py --corpus` gives the same |
| Rules cited by a program test | 498 of 1681 | 511 of 1682 | `tidal-rules` in `tests/programs/` |
| Pages reached by a program test | 84 of 85 | 84 of 85 (`keywords.md` is the one left) | the pages of those rules |
| Program tests that run to a result in `tidal-fe` | 69 of 71 | 85 of 85 (9 of them `pass-partial`) | `conformance.tsv` |
| Write or Port sessions in the phase | 2 (W16, W17) | 0 | `exercises.md` section 8 |
| Exercises by use in the phase | W16, W17, I1 | I2 | `exercises.md` section 8 |
| Port suites | 0 of 8 | 0 of 8 | `exercises.md` section 3 |
| Pages with a Diagnose batch | 4 | 4 | `exercises.md` section 8 |
| Pages whose only Predict batch showed test names | 12 | 1 (`foundations/objects`) | `exercises.md` section 7 |
| Guides | 5 (last change 2026-10-31) | 5 (last change 2026-10-31) | `guides/` |
| Questions | 385: 221 open, 151 settled, 13 compiler defects | 400: 172 open, 215 settled, 13 compiler defects | `questions/README.md`, `status.md` |
| *(referenced)* markers | 33 | 33 | `status.md` |
| Front end, full pass | 6183 of 6500 | 6362 of 6619 (6414 pass their check) | `conformance.tsv` |
| Front end, run-unsupported | 57 | 52 | `conformance.tsv` |
| Front end, false rejects and wrong run results | 12 and 7 | 6 and 0 | `conformance.tsv`, report 20 |
| Front end, missed rejects and unsupported checks | not counted | 118 and 79 | `conformance.tsv` |
| Passing rejects whose first error names a cited or linked rule | not measured | 1990 of 2593 | report 21 |
| Example blocks | 197: 54 unmarked, 27 `fragment`, 109 `reject`, 7 `diverges` | 219: 65 unmarked, 27 `fragment`, 112 `reject`, 15 `diverges` | `% tidal-example` markers |
| Example blocks where `tidal-fe check` disagrees with the marker | 6 of 109 `reject` blocks accepted, 1 unsupported | 0 disagree; 2 unsupported (`templates/instantiation.md` line 718, `templates/packs.md` line 502) | `examples.tsv` |
| Tests that open `namespace std` | 212, 28 of them program tests | 222, 34 of them program tests | `grep` |

The largest reasons for a run-unsupported test are a `new` expression (8), a constructor call (6), a variable of an enclosing function in a local class or lambda (3), an overload set (3), a jump into a nested statement (3), and an lvalue of a union discriminant (3). The largest reasons for an unsupported check are a variable template with unknown arguments in a `static_assert` (21), a concept in a `static_assert` (8), a conversion to a function reference in a constant initializer (6), and a `new` expression in a `static_assert` (5). I grouped these by running `tidal-fe` on each such test.

### 1.2 Round 6 items that phase 9 took and left

| Round 6 item | Phase 9 | Result |
|---|---|---|
| 1. Tests from the reductions and the earlier sketches | item RS | Done. The 8 reductions are tests, and 14 whole sketches are program tests under `tests/programs/roadmap/`. |
| 2. A front-end round on the program faults | item FE (rounds 20 and 21) | Done. VIRT-12 on a delete through a middle class runs. All 85 program tests run. Wrong run results go from 7 to 0. |
| 3. The interpreter in the Write and Port prompts, and in the example check | item FH and item R3 | Done. No Write or Port session ran after the change, so no Write reader has used the step yet. I2 ran the front end on its 46 programs. |
| 4. Port suites | not taken | Decision 0056 defers Port again. |
| 5. Run tests where two pages meet | not taken | Deferred. |
| 6. Diagnose batches | not taken | Deferred. |
| 7. LSUP-2 tests for the three-class demand | items RS and RV2 | Done. RV2 changed LSUP-2 so that a use needs only the class it yields (Q-0386). EDG still checks all three classes for a constant built-in `<=>` and a defaulted `<=>`. |
| 8. Guides | not taken | Deferred. |
| 9. Records that drift | item R3 | Done, with a check for `:tests:` entries and a generated count for section 7. Two new drifts are in section 1.6. |

Round 6 process changes 1 to 5 are in decision 0056, the tools, or the prompts. Change 6 holds for the W16 and W17 rows, which now carry a stale interpreter result (section 1.6).

### 1.3 The earlier sketches, rerun

Every round 6 reduction now gives the right answer in `tidal-fe run`. The run reductions give result 0. `member_template_signature_invalid.cpp` is rejected with MPTR-8. The two LSUP-2 reductions give result 0, which LSUP-2 now allows. Round 6 `inventory_first_try.cpp` is rejected with MPTR-8, as GCC, Clang, and EDG reject it. Round 6 `scheduler.cpp` and `text.cpp` give "unspecified: result 0", with the note that the exploration stopped at its bound of 64 runs.

### 1.4 New programs

| Program | GCC, Clang, sanitizers | EDG 7.0 | `tidal-fe run` |
|---|---|---|---|
| `grid_first_try.cpp` (99 lines): a grid class template with a two-argument `operator[]` through a deduced explicit object parameter, a static call operator template, `if consteval`, `auto(x)`, the `uz` suffix, and a constrained member | both exit 5 | accepts | rejected (DED-35), a false reject |
| `grid.cpp`: the same, with my `if consteval` mistake fixed | exit 0, clean | accepts | rejected (DED-35), a false reject |
| `grid_two_overloads.cpp`: the same with two plain `operator[]` overloads | exit 0, clean | accepts | unsupported: "call operator" |
| `jobs_first_try.cpp` (145 lines): a result type with a value or an error in an anonymous union, a move-only buffer, an undo guard, a function-try-block on a constructor, a rethrow, and conditional `noexcept` | both reject | rejects | rejected (SA-1), which is correct |
| `jobs.cpp` with the placement form only declared | GCC fails to link; Clang exits 0 | accepts | undefined (UNI-6), the wrong rule |
| `jobs.cpp`: the same with the placement form defined | exit 0, clean | accepts | result 0 |
| `registry/main_first_try.cpp` with `commands.cpp` (139 lines in three files): a registry that units fill at static initialization through a function-local static, designated initializers, a lambda converted to a function pointer, `using enum` in a `switch`, structured bindings in a range-based `for`, `extern template`, and a defaulted `<=>` | Clang rejects; GCC exits 0 | rejects | not run |
| `registry/main.cpp` with `commands.cpp` | exit 0, clean | rejects (Q-0386) | unsupported: "call through a pointer to a function of another translation unit" |
| `registry` merged into one unit | not built | not run | result 0 |
| `lambdas.cpp` (93 lines): lambdas that capture `this` and `*this`, a lambda with a template head, a recursive lambda with an explicit object parameter, an immediately invoked lambda, a `mutable` counter, a class template with a variadic call operator template and a deduction guide, and attributes | exit 0; Clang warns once (LAM-18 note) | accepts | unsupported: "object of unknown type" |

A copy of each finished program with one wrong expected value exits nonzero on GCC.

My four first tries had four mistakes from memory, and the text has the rule for each:

- `grid_first_try.cpp` ran `check()` at run time and expected the constant-evaluation branch of `if consteval`. The IF-P4 table gives the branch for each mode.
- `jobs_first_try.cpp` asserted `noexcept(Result<int>(Result<int>(1)))`. The inner temporary is made by a constructor that is not `noexcept`. UN-18 lists the constructor of a temporary.
- `jobs.cpp` first declared the placement form of `operator new` and did not define it. LSUP-7 says that a program defines the form. GCC fails to link. The front end reports UNI-6 for a read of an inactive union member, which leads away from LSUP-7 and ODR-8.
- `registry/main_first_try.cpp` declared `std::strong_ordering` after the class whose `<=>` is defaulted, and before the first use. LSUP-2 says "must first declare". Clang and EDG reject at the class. GCC and the front end accept. Section 1.5 has the reduction.

The compilers named the cause of three of the four. The front end named the right rule in one (SA-1). In `grid_first_try.cpp` its false DED-35 reject hid my mistake.

### 1.5 Reductions

Each run reduction builds and exits 0 on GCC and Clang with both sanitizers, and EDG accepts it, except where the table says otherwise.

| Reduction | Lines | `tidal-fe` | What the specification says | Tests that use the form |
|---|---|---|---|---|
| `xobj_template_ref.cpp`, `xobj_template_fwd_value.cpp`, `xobj_auto_ref.cpp`, `xobj_named_call.cpp`: a member function whose explicit object parameter is `Self&`, `Self&&`, or `auto&` | 10 to 13 | rejected, DED-35 | CLS-13: the object of the call initializes the parameter. DED-8 gives the deduction for a forwarding reference. `xobj_plain.cpp` with `this G&` runs. | none for a member function; two lambda tests (LAM-16) |
| `xobj_subscript_two_args.cpp`, `xobj_subscript_one_arg.cpp`: the same form on `operator[]` | 12 | rejected, DED-35 or OVL-35 | CLS-13, CLS-15, OPR-3. `subscript_two_args_plain.cpp` runs. | none |
| `call_operator_template_run.cpp`, `call_operator_template_constant.cpp`, `call_operator_template_class_param.cpp`: a class with a member `operator()` template, called as `Id{}(3)` | 5 to 12 | unsupported: "call operator" | OPR-3 and the deduction rules. A function object of this shape is an ordinary comparator or visitor (general knowledge). | none in a user class |
| `static_call_operator_template_run.cpp`, `static_call_operator_template_constant.cpp`: a static `operator()` template | 5 to 7 | unsupported: "call operator" | CLS-15. `static_call_operator_constant.cpp`, the form without a template, runs. | none |
| `lambda_template_parameter_list.cpp`: `[]<class T>(T x) { return x + x; }` | 4 | unsupported: "object of unknown type" | The lambdas page grammar has `lambda_template`. LAM-15 covers only `auto` parameters. | 2 tests under `templates/concepts/`, both unsupported |
| `lambda_auto_pack.cpp`: `[](auto... a) { return (a + ...); }` | 1 | unsupported: "object of unknown type" | LAM-15, PACK-1. `lambda_auto_parameter.cpp` with one `auto` runs. | 1 test, unsupported |
| `lambda_explicit_object_recursion_constant.cpp`: a recursive lambda with `this auto self` in a constant evaluation | 8 | unsupported: "call needing template deduction" | LAM-16 | 2 tests, both failing |
| `auto_cast_int.cpp`, `auto_cast_class.cpp`, `auto_cast_class_in_template.cpp`: `auto(x)` and `auto{x}` | 7 to 11 | unsupported: "object of unknown type" | CAST-13 | 2 tests, both failing |
| `static_call_operator_address.cpp`, `template_function_address_explicit.cpp`: `&Sum::add<long>`, the address of a static member function template with explicit arguments | 8 | rejected, TR-12, a false reject | OVL-18 and the address rules. `&add<long>` and `&N::add<long>` run. | none |
| `static_member_template_address_target.cpp`, `static_member_template_no_ampersand.cpp`: the same member with a target type, and without `&` | 5 | unsupported: "address of static member function", "object of unknown type" | CLS-17 for the explicit object form; the address rules | none |
| `fnptr_other_unit/main.cpp` with `other.cpp`: a call through a function pointer that another unit returned | 3 and 3 | unsupported | ODR-8 and the program's link. GCC and Clang run it. | none |
| `zero_to_nullptr_t_operator.cpp`: an operator whose second parameter is `decltype(nullptr)`, used as `o < 0` | 8 | unsupported: "overloaded operator" | The null pointer conversion of a literal `0`. `zero_to_nullptr_t_call.cpp`, the same conversion in a named call, runs. The language-support page's ordering class uses `int` parameters, which run (`defaulted_spaceship_less_run_int.cpp`). | none |
| `placement_new_declared_only.cpp`: a placement `new` through a declared, undefined `operator new(std::size_t, void*)` | 10 | undefined, UNI-6 | LSUP-7 and ODR-8. GCC fails to link, and Clang runs it. Elsewhere the interpreter reports a missing definition as "has no definition in the program". | none |
| `ordering_declared_after_class.cpp`: a defaulted `<=>` declared before `std::strong_ordering`, used after it | 17 | result 0 | LSUP-2 says "must first declare" and does not say first before what. Clang and EDG reject at the defaulted declaration, and GCC accepts. | none |

Of the 32 one-line programs of common forms that I tried, 31 are valid, and the front end runs 26 of them: abbreviated function templates, `constexpr` virtual calls, folds, designated initializers with bindings, `using enum`, class template argument deduction, range-based `for` with an initializer, `new` and `delete`, pointers to members, `consteval`, a converting constructor template, `uz`, `if consteval`, a two-argument `operator[]`, a static `operator()`, a static lambda, `[[assume]]`, and an explicit object parameter of a named type. The 5 that stop are in the table above. The front end rejects the invalid one with INC-3, as the compilers do. So the front end handles C++20 and the C++23 forms without templates well. It stops when a C++23 form meets template deduction.

### 1.6 Records that drift

- `exercises.md` section 8, rows W16 and W17, say that the interpreter stops `evaluator.cpp` and `units/main.cpp`. `conformance.tsv` gives `pass-partial` and `pass` for them since round 20.
- `exercises.md` section 6, "Order", still gives the order of decision 0055 item E and names no phase 9 exercise.
- The V3 commit records that the "Kept open" line of Q-0290 names GCC 16 and EDG 7.0, while its test shows only Clang rejecting. The record waits for a later pass.
- `foundations/objects` is the one page whose only Predict batch showed test names. The tool fix that it waited for is in `32c89f7`.

## 2. Ranked work items

| # | Item | Evidence | Size | Prerequisites |
|---|---|---|---|---|
| 1 | Tests for the C++23 call forms that no test uses | 8 faults on forms that no test uses, 4 on forms whose only tests fail; each reduction builds clean | small | none |
| 2 | A front-end round on the C++23 call forms and the cross-unit call | 2 false rejects (DED-35 on every deduced object parameter, TR-12), 9 stops, and 1 wrong rule, in 476 lines | medium | item 1 |
| 3 | Text: CLS-13, LAM-15, and LSUP-2 for a programmer | CLS-13 shows only `this T self`; LAM-15 covers only `auto`; LSUP-2 does not say before what, and the matrix splits | small | none |
| 4 | A Write wave on modern idioms, with the interpreter step | 0 Write or Port sessions in phase 9; 1055 of 1055 R9 predictions right, while 4 programs found 12 front-end faults | one session per suite | item 3 for the prompts' pages |
| 5 | Port suites P1 to P8 | 0 of 8 since phase 5; deferred by decisions 0054, 0055, and 0056 | one session per suite | item 4's prompt |
| 6 | A count of grammar forms by test and by front-end result | every new fault is a grammar form that no test or only failing tests use | small script, then a front-end option | none |
| 7 | Diagnose batches with the front end's message | 4 pages; 603 of 2593 passing rejects name another rule or none first; the front end named the right rule for 1 of my 4 mistakes | one session per 4 pages | none |
| 8 | Guides on callables and on raw storage | deferred since phase 5; the callables forms of section 1.5 are what a guide would show | one decision, then one session per guide | maintainer |
| 9 | Records that drift, and the last named Predict page | section 1.6 | tiny | none |

### Item 1. Tests for the C++23 call forms that no test uses

**What.** A clean-room writer adds tests from the reductions of section 1.5, each probed on the matrix:

- a member function with a deduced explicit object parameter (`this Self&`, `this Self&&`, `this auto&`), called on a non-`const` lvalue, a `const` lvalue, and an rvalue, with a `static_assert` on the deduced type, under CLS-13 and DED-8, as a run test and in constant evaluation,
- the same form on a two-argument `operator[]`, under CLS-15 and OPR-3,
- a class with a member `operator()` template and one with a static `operator()` template, under OPR-3 and CLS-15, as run tests,
- `&S::f<long>` for a static member function template, with and without a target type, under OVL-18,
- a call through a function pointer that another unit returns, as a two-unit run test,
- a defaulted `<=>` declared before the ordering class is declared and used after it, under LSUP-2, with overrides and a new question (item 3),
- the integrator copies `grid.cpp`, `jobs.cpp`, `lambdas.cpp`, and `registry/` into `tests/programs/roadmap/` as `r7-` program tests. EDG needs an override on `registry` for Q-0386.

**Why.** Section 1.5. The deduced explicit object parameter on a member function is the main use of the C++23 feature (general knowledge), and no test of 6619 has it. No test has a user class with a call operator template. Decision 0055 asks the commit that acts on a roadmap round to add each correct sketch and reduction.

**Size.** Small. Each reduction has at most 17 lines.

**Prerequisites.** None. The tests give item 2 its targets.

### Item 2. A front-end round on the C++23 call forms

**What.** Front-end round 22 takes these targets, in this order:

1. the false DED-35 reject for a deduced explicit object parameter of a member function, and the TR-12 false reject for `&S::f<long>`, since a false reject on valid code stops a programmer at once,
2. the wrong UNI-6 verdict for a declared placement form with no definition, which should name the missing definition as the interpreter does elsewhere,
3. the call operator templates: a member `operator()` template, a static one, a lambda with a template head, and a lambda with an `auto...` pack,
4. `auto(x)` and `auto{x}` (CAST-13), and the recursive explicit object lambda in a constant evaluation (LAM-16),
5. the call through a function pointer from another unit, and the null pointer conversion of `0` in an operator call,
6. the 6 remaining false rejects, then the run-unsupported reasons of section 1.1, largest first.

**Why.** Sections 1.4 and 1.5. All 85 program tests run, so the corpus no longer finds new faults by itself. My four programs found 12 faults, and each one is a C++23 form, a join with templates, or a join between units. The 7 tests that already use four of these forms all fail on the front end.

**Size.** Medium. Deduction of the object parameter sits in overload resolution, which the interpreter shares with every call.

**Prerequisites.** Item 1, so that `tools/frontend_check.py` shows each fix.

### Item 3. Text for a programmer: CLS-13, LAM-15, and LSUP-2

**What.**

- CLS-13 gains an example with `template <class Self> auto&& get(this Self&& self)`, a sentence that the object of the call is the argument that DED-8 deduces from, and a table of the deduced `Self` for a non-`const` lvalue, a `const` lvalue, and an rvalue. CLS-15 and OPR-3 link the example for `operator[]`.
- LAM-15, or a new rule beside it, says what a lambda template head gives: the call operator is a template with those parameters, and a call deduces them as for a function template. The rule cites the grammar's `lambda_template` and the two concepts tests.
- LSUP-2 says where "first" is: before the defaulted `<=>` is declared, or before its first use. Clang and EDG reject the declaration, and GCC accepts. A new question records the split and a provisional answer.
- The LAM-18 note on Clang's unused-capture warning adds the case of `lambdas.cpp`: a copy capture of a `const int` with a constant initializer that the body only reads. ODR-3 says such a read is not a use.

**Why.** Section 1.5. A programmer who reads CLS-13 sees `this T self` and cannot find the deduced form, which is the form that replaces a `const` and non-`const` pair of overloads (general knowledge). The lambdas page has a grammar line for the template head and no rule. LSUP-2's "first" admits both readings, and the matrix follows each.

**Size.** Small: three examples, one sentence, one question.

**Prerequisites.** None.

### Item 4. A Write wave on modern idioms

**What.** Write suites W18 and W19, run with the interpreter step of the current prompt, on tasks that use C++20 and C++23 forms with templates:

- W18, callables (persona Rust or Swift): a container with one accessor through a deduced object parameter, a set of comparators as function objects with call operator templates, a visitor built from lambdas with template heads, and a callback table that units fill,
- W19, compile-time and generic code (persona Haskell): a fixed-size matrix with `operator[]` of two arguments, a type list with a lambda template head, a `consteval` table builder, and an `if consteval` helper with a run-time path.

**Why.** Phase 9 ran no Write or Port session, so the prompt's interpreter step has not met a Write reader. The R9 batches scored 1055 of 1055, and PU1 and PU2 scored 64 of 66 and 42 of 44, so prediction from the text works. Writing programs found 12 front-end faults and 3 thin rules in 476 lines. Decision 0055 asks each phase for an exercise by use, and I2 met that in phase 9 with one Implement exercise.

**Size.** One session per suite.

**Prerequisites.** Item 3 for the pages that the tasks need.

### Item 5. Port suites P1 to P8

**What.** The eight Port suites of `exercises.md` section 3, with the interpreter step.

**Why.** Port has waited since phase 5. Decisions 0054, 0055, and 0056 each deferred it. The idioms of other languages map to the forms of section 1.5: a Rust trait object or a Swift protocol method with `Self` maps to a deduced object parameter, and a closure with generic parameters maps to a lambda template head (general knowledge).

**Size.** One session per suite.

**Prerequisites.** Item 4's prompt, so that a Port reader also runs the interpreter.

### Item 6. A count of grammar forms by test and by front-end result

**What.** A script lists each production of the Syntax blocks of `specification/`, the tests whose text matches a pattern for that production, and the front-end status of those tests. A later front-end option prints the productions that a check derives, so that the list needs no patterns.

**Why.** Each fault of section 1.5 is a grammar form that no test uses, or that only failing tests use: `lambda_template` has 2 tests, both unsupported, and the deduced object parameter of a member has none. Round 6 asked for a count of rule pairs. A count of productions is smaller and finds this kind of gap before a program does.

**Size.** A small script. The front-end option is part of a later round.

**Prerequisites.** None.

### Item 7. Diagnose batches with the front end's message

**What.** Diagnose batches for templates, classes, and overloading. Each batch shows the compiler message and the `tidal-fe check` message, and scores whether the message leads the reader to the rule.

**Why.** Diagnose has 4 pages since phase 5. 603 of the 2593 passing rejects in the front end name another rule or no rule first (report 21). Of my four mistakes, the compilers named the cause of three. The front end named the right rule for one, gave UNI-6 for a missing placement form, and hid one mistake behind a false DED-35.

**Size.** One session per 4 pages.

**Prerequisites.** None.

### Item 8. Guides on callables and on raw storage

**What.** A maintainer decision on publishing guides, then two how-to guides: callables (function objects, lambdas, deduced object parameters, function pointers between units) and a container in raw storage (the placement forms of LSUP-7, the kept pointer of the storage page, and a result type in a union).

**Why.** Rounds 3 to 6 asked for guides. My `jobs.cpp` and `grid.cpp` are what the two guides would show, and two of my four mistakes (LSUP-7, UN-18) are the steps a guide would name.

**Size.** One decision, then one session per guide.

**Prerequisites.** The maintainer.

### Item 9. Records that drift, and the last named Predict page

**What.** Fix the records of section 1.6. Have `tools/status_table.py` write the interpreter result of each Write row from `conformance.tsv`. Rewrite section 6 of `exercises.md` for the phase that follows. Run a Predict batch without test names on `foundations/objects`.

**Size.** Tiny. **Prerequisites.** None.

## 3. Out of scope

| Topic | Proposal | For | Against |
|---|---|---|---|
| The general library, including `std::move`, `std::forward`, and `<cstdarg>` | Keep out. Programs define their own `move` as `jobs.cpp` does. | Programs use them (general knowledge). | The charter leaves out the library. Two front-end tests stop on `<cstdarg>` only. |
| `typeid` | Keep deferred. | A hierarchy program would use it (general knowledge). | Each test needs the library's `type_info` (Q-0322). |
| The launder operation | Keep out until `pending.md` row 2 is decided. | A raw-storage container written the common way uses it (general knowledge). | The storage page's kept-pointer form needs no library. |
| The preprocessor beyond `#include "file"` and the conditional groups | Keep out. | `assert` is common (general knowledge). | High similarity risk. No program here needed it. |
| Threads, the memory model, coroutines, modules, extended floating-point types | Keep out. | | as the charter says |
| Language versions after C++23 | Keep out until `pending.md` row 7 is decided. | Programmers try newer forms (general knowledge). | Strict mode rejects them, and the matrix follows C++23. |

## 4. Process changes

1. **Each phase runs a Write or Port session.** An Implement exercise reads one procedure. A Write session meets the joins between pages and the newest forms, and phase 9 had none.
2. **A roadmap reader's whole sketches become program tests in the next phase,** as RS did for rounds 4 to 6. Round 7's four programs are ready.
3. **A front-end round reports grammar forms with no passing test** (item 6), beside its status counts.
4. **The exercise status rows take the interpreter result from `conformance.tsv`,** so that a front-end round updates them.
5. **A Write or Port prompt asks for the forms of the persona's own language,** such as a generic closure or a method on `Self`, so that the programs reach the newest forms of C++.

## 5. Suggested order

Items 1, 3, and 9 come first, because each is small and item 2 needs item 1's tests. Item 6's script runs beside them and can add targets to item 2. Item 2 then runs. Item 4 runs after item 3 and beside item 2, and its reductions feed the round after item 2. Item 5 follows item 4 with the same prompt. Item 7 runs beside the others. Item 8 waits on the maintainer.

## 6. Doubts for the clean-room writers

- Whether the false DED-35 reject comes from deduction that leaves out the object argument. A plain `this G&` parameter runs, and every templated form fails, also in a named call.
- Whether "first" in LSUP-2 means before the defaulted declaration or before the use. Clang and EDG reject at the declaration, GCC accepts, and the rule text admits both.
- Whether a declared placement form with no definition belongs under ODR-8 (no diagnostic) or under LSUP-7 as a link failure, and what a programmer should see from the interpreter.
- Whether the nine `pass-partial` program tests and the "unspecified: result 0" lines with a stopped exploration should tell a programmer more than "result 0". Round 6 `scheduler.cpp` reaches the bound of 64 runs.
- Whether a copy capture of a constant variable that the body only reads should stay explicit in Tidal's own examples, since Clang warns about it under `-Wall` and GCC does not.
