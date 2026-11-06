# Roadmap, round 9: what to do after phase 11, from a programmer's view

Date: 2026-11-05. Decision 0029. This report comes from a clean-room reader in the role of a C++ programmer who uses the specification to write and check programs.

**What the reader read.**
- AGENTS.md, CLAUDE.md, and the round 8 programmer report (`reports/clean-room/roadmap-8/programmer.md`) with its sketches.
- Decision 0058, the status page, the roadmap's phase 10 and 11 tables, the headings and rows of `documentation/decisions/pending.md`, and sections 3, 8, and 9 of `documentation/exercises.md`.
- `git log 1e20bfa2..a15a877d` with every commit body (15 commits).
- The P1 and I4 reports with their programs and reductions, `reports/clean-room/ledger.md`, `rust-frontend/ledger.md`, the target table of front-end report 22, and `conformance.tsv`.
- The result sections and the ACC-10 and DED-29 sections of the two 2026-11-05 gap reviews of phase 10 text, and the summary lines of the eight phase 11 similarity reports.
- The phase 11 tests (30 files), PRIM-9, LSUP-10, INST-6, the language-support table, and the `construct_at` example on the unions page.

The reader did not read the other round 9 reports, the X23 report beyond its commit body, `frontend/` sources, `inputs/`, or `build/exercise-keys/`. Commit read: `a15a877d`.

**What the reader did.** The reader built the front end from `a15a877d` into the session scratchpad. `git diff 1e20bfa2 a15a877d -- frontend/` is empty, so this is the round 8 front end. The reader reran all 89 round 8 sketches, reductions, short programs, planted bugs, and planted mistakes. The reader wrote five new programs (650 lines), 28 new short programs, 8 new planted bugs, 8 new planted mistakes, and 17 reductions. Each went through these checks:
- host GCC 16.2.1 and Clang 22.1.8 with `-std=c++23 -pedantic-errors -Wall -Wextra`,
- the same with `-std=c++23 -pedantic-errors` alone,
- the same with `-fsanitize=undefined,address`,
- the EDG 7.0 container (`cpfe --c++23 --strict --no_code_gen --instantiate=used`),
- `tidal-fe run`.

The sketches, reductions, driver, and result logs are in `programmer-sketches/` beside this report: `round8/` holds the copies that were rerun, `new/` the new programs, `reductions/` the reductions, and `results-round8.txt` and `results-new.txt` the logs. All counts come from read-only `grep`, `git`, and Python on the trees at `1e20bfa2` and `a15a877d`. Statements that rest on general knowledge say so.

## 1. The state from a programmer's view

Phase 11 did round 8's text and test items, and it did not run a front-end round. Every round 8 result is the same as in round 8. The 11 phase 11 tests that the front end does not pass are the round 8 forms and the I4 and X23 cases.

The new programs are library code: a growable array, an optional in a union, a small string with a view, a type-erased callable, and a sorted map. They stop the front end more often than the round 8 programs did:
- None of the five runs on the first try. Round 8 ran four of six.
- After workarounds, one runs to a result (`vec.cpp`). The other four stop on a form that the reader could not write around.
- The 17 reductions and two planted mistakes show 14 front-end faults that no earlier report names.

The front end still runs nearly every short program of common forms. It runs 28 of 28 new short programs and catches 8 of 8 new planted bugs with a rule.

Two of the new faults give a wrong verdict on a correct program, which round 8 did not see:
- A pointer copied from one evaluation of a string literal compares unequal to itself in one explored order. PRIM-9 is about two evaluations; this is one.
- A loop over `[p, p + 5)` inside a string literal ends in `undefined [ADD-10]`. A view class with `begin` and `end` over a literal hits this in a range `for`.

`conformance.tsv` shows 0 wrong run results because no test has these forms.

### 1.1 Counts

| Measure | Round 8 (`1e20bfa2`) | Now (`a15a877d`) | Source |
|---|---|---|---|
| Rule headers | 1697 (1682 in force, 15 withdrawn) | 1697 (1682 in force, 15 withdrawn) | `:::{rule}` headers and `:kind:` lines |
| Example blocks | 220: 65 unmarked, 28 `fragment`, 112 `reject`, 15 `diverges` | 220, the same split | `% tidal-example` markers |
| Tests | 6681 | 6711 | files with `tidal-rules`, `.tu.cpp` left out |
| Program tests | 97 (18 under `roadmap/`) | 103 (24 under `roadmap/`) | `tests/programs/` |
| Lines under `tests/programs/` | 11438 | 12187 | this report's script |
| Rules cited by a program test | 533 of 1682 | 540 of 1682 | `tidal-rules` in `tests/programs/` |
| Tests that open `namespace std` | 233, 43 of them program tests | 241, 48 of them program tests | `grep` |
| Front end, by `frontend_check.py` | 6459 of 6681 pass | 6478 of 6711 pass, 6520 their check; 11 of the 30 new tests do not pass | close commit `a15a877d` |
| `conformance.tsv` | 6681 rows | 6681 rows; the 30 new tests have none | the file |
| Changes under `frontend/` | (round 22) | none | `git diff` |
| Port suites done | 0 of 8 | 1 of 8 (P1) | `exercises.md` section 8 |
| Write suites in the phase | W18, W19 | none | `exercises.md` section 8 |
| Questions | 411: 171 open | 418: 174 open, 231 settled, 13 compiler defects | `status.md` |
| Front-end ledger, open rows | 4 (G4, G13, G25, G152) | 0 | `rust-frontend/ledger.md` |
| Exercise ledger | none | 867 rows: 762 commit, 68 open, 26 planned, 5 text, 3 question, 3 out of scope | `ledger.md` |
| Guides | 5 (last change 2026-10-31) | 5 (last change 2026-10-31) | `guides/` |

### 1.2 Round 8 items that phase 11 took and left

| Round 8 item | Phase 11 | Result |
|---|---|---|
| 1. Tests for the initializer forms with packs, and the two matrix splits | item P | Mostly done. Four pack tests, one braced test, a test for each EDG split (Q-0413, Q-0414), the G152 run test (item F), and six `r8-` program tests. Not done: the pack in a member initializer of a `constexpr` constructor or a constructor template (the TR-12 false reject), the braced form in a constant and as a base initializer, and a base pack `leaf<I, T>` with a type argument. |
| 2. Text: pack expansion in initializers, the nested friend, G152 | items P and F | Done: PACK-13 and the PACK-P2 row, ACC-10 with Q-0414, CEV-1 with Q-0413 (settled in RV4), DED-29 for G152, the LAM-18 note, and the placement form under ODR-8 and LSUP-7. Not done: the example of a forwarding constructor. The count of example blocks did not change. |
| 3. A front-end round on library-style initialization | item FE | Not done. FE moved to the next phase. |
| 4. Port suites | item E (P1) | One suite, Rust persona. |
| 5. Write suite W20 on library-shaped code | none | Deferred by decision 0058. |
| 6. Diagnose batches with the front end's message | none | Deferred by decision 0058. |
| 7. A count of grammar forms by test and front-end result | none | Not taken, and not in decision 0058's deferred list. |
| 8. A findings ledger, and records that drift | items L and R5 | Done, with a check. Section 1.8 lists two gaps. |
| 9. Interpreter output when every order agrees | none | Deferred by decision 0058. |
| 10. Guides | none | Not taken. |

Round 8 process changes 1 (a build with `-pedantic-errors` alone) and 2 (the persona's own forms) are in the prompt (`a2c246f8`). Change 3 (a ledger row for each finding) is a choice of decision 0058. Change 4 (the reader's sketches become program tests) was done for round 8 in item P. Change 5 (a front-end round lists grammar forms with no passing test) is not done.

### 1.3 The round 8 sketches, rerun

All 89 round 8 files give the same result on each compiler, on EDG, and on the front end as in round 8 (`results-round8.txt`). The new `-pedantic-errors` build differs from the `-Wall` build in one case: Clang rejects `mistakes/designator_out_of_order.cpp` under `-pedantic-errors` alone and only warns under `-Wall`, as round 8 said.

The round 8 programs are now program tests. On the front end, `r8-fmt` gives result 0 and `r8-config`, `r8-owner`, and `r8-pipeline` give "unspecified: result 0". `r8-tasks` is still rejected with CTOR-15. `r8-logger` gives result 0 when both units are given.

### 1.4 New programs

| Program | GCC, Clang (three flag sets) | EDG 7.0 | `tidal-fe run`, first try | `tidal-fe run`, last version |
|---|---|---|---|---|
| `vec.cpp` (183 lines): a growable array in raw storage, `emplace_back` that forwards a pack into a placement `new`, growth that moves only when the move is `noexcept`, copy and swap, a variadic factory with a fold, `erase_if`, `Vec<Vec<int>>`, elements that convert from a literal and from another class | exit 0, clean | accepts | rejected (OVL-35): a `Token` binds to `const Name&` through `Name(Token)`. This is P1-2. | unspecified: result 0; the bound of 64 runs stopped the exploration |
| `optional.cpp` (129 lines): an `Optional<T>` in a union, an in-place constructor with a pack in a member initializer, `emplace`, a converting constructor from `Optional<U>`, a `nullopt_t` tag, `value_or` and `transform` with a deduced object parameter, the conditional operator with class operands, all in constant evaluation | first try fails on every compiler (section 1.6); last version exit 0, clean | first try rejects; last version accepts | rejected (TR-12) on the member initializer | rejected (TR-12). Without the member initializer: "unsupported: class prvalue" on a class conditional |
| `text.cpp` (103 lines): a string with inline storage, a view type, a converting constructor from a literal, a conversion function to the view, hidden-friend `+` and `==`, a user-defined literal, a fold over `+`, a class conditional, an insertion sort over an array | exit 0, clean | accepts | rejected (DED-35): `String words[] = {...}` deduced as `T (&)[N]` | unsupported: "class prvalue" in a `static_assert`; at run time, `undefined [ADD-10]` in some explored orders, which is wrong |
| `function.cpp` (119 lines): a type-erased `Function<R(Args...)>` as a partial specialization, a constrained constructor template, a virtual interface with `clone`, `bind_front` with an init-capture pack, an `invoke` for pointers to members, an event list | first try fails on every compiler (section 1.6); last version exit 0, clean | first try rejects; last version accepts | "unsupported: constructor with an initializer the front end does not model" | "unsupported: default constructor" on an array of `Function` |
| `map.cpp` (116 lines): a sorted flat map, `try_emplace` that forwards a pack into an `Entry` constructor, a lookup by string literal through hidden friends, `operator[]`, structured bindings in a range `for`, a constrained visitor | exit 0, clean | accepts | rejected (TR-12) on the member initializer | "unsupported: default constructor" on the array of `Entry` |

Each first try and each intermediate version is kept beside the last version (`*_first_try.cpp`, `*_second_try.cpp`, `*_third_try.cpp`, `optional_body_construct.cpp`).

### 1.5 Reductions

Each reduction builds and exits 0 on GCC and Clang under the three flag sets, and EDG accepts it. None has a test, by `grep` or by the front end's result on the tests that cite the rule.

| Reduction | Lines | `tidal-fe` | What the specification says |
|---|---|---|---|
| `literal_pointer_copy_compare.cpp`: `const char* p = "world"; const char* e = p; return p == e ? 0 : 1;` | 7 | unspecified: result 0, other outcome result 1 (PRIM-9). **Wrong.** | PRIM-9 covers two literals, or two evaluations of one. Here one evaluation gives one pointer, and its copy is the same pointer. |
| `literal_pointer_range_loop.cpp`, `literal_view_range_for.cpp`: a loop, or a range `for` over a view class, from `p` to `p + 5` inside `"world"` | 9 to 14 | undefined [ADD-10]. **Wrong.** The `static_assert` in the second file passes. | The loop stops at `p + 5`, which points into the array. |
| `unknown_bound_converted_elements_deduce_bound.cpp`: `S a[] = {"x", "y", "z"};` passed to `T (&)[N]` | 6 | rejected, DED-35, a false reject | DCL-5 gives the bound from the list. P1-1 found the same array as "unsupported". |
| `capture_pack_by_copy_fold_generic.cpp`: `[xs...](auto r) { return (xs + ... + r); }` | 5 | rejected, PRIM-29, a false reject | PACK-17. The test `capture-pack.cpp` is `unsupported` in `conformance.tsv`. |
| `capture_pack_by_copy_fold.cpp`, `capture_pack_by_copy_generic.cpp`: the same capture with `(int r)`, and in a `bind_front` | 6 to 8 | unsupported | as above. The init-capture form `[... ys = xs]` runs. |
| `assign_through_converting_constructor.cpp`: `S s(1); s = 5;` | 4 | unsupported: "class assignment" | The implicit assignment takes a temporary that `S(int)` makes. |
| `assign_through_constructor_template.cpp`: a lambda assigned to a type-erased callable | 8 | unsupported: "class assignment" | as above, through a constructor template |
| `array_of_class_with_constructor_template.cpp`: `F h[2];` where `F` has `F() = default;` and `template <class G> F(G)` | 8 | unsupported: "default constructor", at line 0 | One object of `F` runs. |
| `direct_init_through_conversion_function.cpp`: `V(a)`, `static_cast<V>(a)`, and `V v(a)` with `S::operator V()` | 11 | unsupported: "class temporary", "cast to class", "constructor call" | `V v = a;` runs. |
| `cond_class_lvalue_vs_int.cpp`, `cond_class_lvalue_vs_string_literal.cpp`: `c ? b : 8` and `c ? s : "quiet"` with a converting constructor | 11 | unsupported: "class conditional" | COND-16 step 3. The test `nonclass-converts-to-class.cpp` checks the type and is `unsupported`. |
| `cond_class_lvalue_vs_convertible_class.cpp`: a class lvalue against an lvalue of another class that converts to it | 14 | unsupported: "class prvalue" | COND-16. The prvalue form (`cond_class_prvalue_vs_convertible_class.cpp`) runs. |
| `cond_class_lvalue_vs_prvalue_constant.cpp`: `b ? a : S{2}` with one class, in constant evaluation | 7 | unsupported: "class prvalue" | COND-3. The run form runs. |
| `lambda_trailing_decltype_auto.cpp`: `[](int x) -> decltype(auto)` and `[](auto x) mutable -> decltype(auto)` | 7 | unsupported: "object of unknown type", at line 0 | A generic lambda with `-> decltype(auto)` and no `mutable` runs (`W12/compose.cpp`). |

The P1 and I4 reductions give the same results as in their reports.

### 1.6 Short programs, planted bugs, and planted mistakes

**Short programs.** The 28 new short programs in `new/one-line/` build and run clean on GCC and Clang with every flag set, and EDG accepts all 28. The front end runs all 28. `recursive_lambda_deduced_this.cpp` gives "unspecified: result 0" because it reaches the bound of 64 runs. The forms are:
- a struct returned and unpacked by a structured binding, and a two-dimensional array in a range `for`,
- `using enum` in a `switch` on a scoped enumeration, and `[[maybe_unused]]` and `[[fallthrough]]`,
- a lookup table from an immediately invoked lambda, and a `consteval` call as a template argument,
- ref-qualified member functions, delegating constructors, and a noncopyable prvalue returned by value,
- CRTP, `+` written through `+=` with an `int` operand, and `explicit operator bool` in a condition,
- `[this]` and `[*this]`, a fold over the conditional operator, and `template <auto N>`,
- a catch by `const` reference to a base, and a constructor that throws after its members are built,
- an `inline static` counter in a class template, a function object passed by value to an algorithm,
- digit parsing, `alignas` and `alignof`, shifts on `unsigned`, and the bytes of an object through `unsigned char`,
- a conditional with two class lvalues bound to `auto&`, and a conditional on base pointers,
- overloads on `const int&` and `int&&` with a default argument, a `do`-`while` with an unsigned index, and a recursive lambda with a deduced object parameter.

**Planted run-time bugs.** The front end stops each of the 8 bugs in `new/bugs/` with `undefined` and a rule:

| Rule | Bug |
|---|---|
| ADD-9 | a sum that overflows `int` |
| SHF-7 | `1u << 32` |
| OBJ-2, INIT-1 | a read of a local that one path leaves with no value |
| NEW-5 | a second `delete` through a copied owner |
| OBJ-2 | a `const T&` from `max_of(W{1}, W{2})`, and a reference to a local returned |
| MUL-9 | a division by zero |
| CAST-5, CV-5 | a write to a string literal through `const_cast` |

The sanitizers stop 7 of them. No sanitizer reports the uninitialized read. Without sanitizers, the dangling `max_of` exits 0 on both compilers, and the returned local exits 0 on Clang.

**Planted compile-time mistakes.** GCC, Clang, and EDG reject all 8 mistakes in `new/mistakes/`. The front end rejects 6, stops on 1, and accepts 1:
- `member_of_function_type.cpp`: `Holder<int(int)>` with a data member `F f;`. The front end runs it to result 0. INST-6 rejects it, and its one test of the form uses a static member (`class-static-member-function-type.cpp`). This is a missed reject. A forwarding `Function` that takes a function by name makes the same mistake (`function_first_try.cpp`), and there the front end stops with "unsupported" and does not reject.
- `copy_in_range_for_move_only.cpp`: `for (auto m : a)` over an array of a move-only class. The front end stops with "unsupported: range-based for". The copy outside a loop is rejected with DEL-1.
- In 5 of the 6 rejects, the first error names the rule of the cause: AUTO-20, CTOR-9, FN-3, PACK-8, and NEW-37. NEW-37's message ("placement new expression outside std::construct_at") names the fix.
- `missing_template_keyword.cpp` (`u.get<0>()` in a template) gives TR-12 ("expected an expression, found ')'"). Clang names the `template` keyword. GCC gives a parse error, as the front end does.

The two other first tries had one mistake each from memory:
- `optional_first_try.cpp` used placement `new` in a `constexpr` function. LSUP-10 and the language-support table say that constant evaluation runs placement `new` only inside `std::construct_at`, and the unions page has an example. The front end stopped on the member initializer first and never reached NEW-37.
- `function_first_try.cpp` stored a function type as a member, as above. General knowledge: the library's `decay` does this step, and a program without headers has to write it.

### 1.7 The P1 and I4 reports, from a programmer's view

**P1.** P1-2 is the fault a programmer meets first. A `const R&` that binds to an object of another class through `R`'s converting constructor is rejected with OVL-35. P1 met it in `Result == Err{e}`, and `vec_first_try.cpp` met it in `name == Token{"bob"}`. Two independent programs hit it in one phase. P1-1 (an array of unknown bound with converted elements) also turns into a DED-35 false reject when the array is passed to `T (&)[N]` (section 1.5). Neither P1 finding has a test.

The text findings of P1 are still open. `index.md` still does not give the base flags `-std=c++23 -pedantic-errors` (P1-7). No example of a two-alternative class (P1-3), a link from `virtual.md` to the `Owner` example (P1-5), or a library-free requirement example (P1-8) was added. P1's three ports are not program tests. `exercises.md` section 3 makes Write programs into tests and says nothing about Port programs.

**I4.** I4-6 and I4-7 matter most to a programmer who checks programs. The front end runs a conditional that converts to an ambiguous base or to a private base, and both compilers reject it. A missed reject tells a programmer that a wrong program is right. I4-4 is a false reject (ASN-2 on an assignment to a conditional with an `int&` conversion). I4-1 to I4-3 changed the text, and their three tests do not pass on the front end. My reductions add four class conditional stops that I4 did not list (section 1.5). None of I4-4 to I4-7 has a test.

### 1.8 Ledger rows

**Planned rows whose item ended.** The close commit lists 21 such rows: I3-1, I3-5, I3-13, R10-4, R10-6, P1-3, P1-5, P1-7, P1-8, P1-9, I4-8 to I4-14, X23-3, X23-6, and X23-7. None has an answer. Each row still says "planned" and names an item of decision 0058, which has ended. Five of them (I3-1, I3-5, I3-13, R10-4, R10-6) name item RV4, and the RV4 repair commit does not mention them. Six more rows (P1-1, P1-2, I4-4 to I4-7) wait for front-end round 23, and none of them has a test that would show the fix.

**Rows that name programs.** Front-end report 22 marks W18-1, W18-3 to W18-8, W18-10, W18-11, W19-3, and W19-4 as done. The ledger gives all 11 the state `open` with "no later commit or question names the finding". The script that set the rows read commit messages and not the front-end reports. The other 57 open rows are findings of W1, W3, W4, W8, W13, W14, W18, W19, RPC, X5, and X6.

## 2. Ranked work items

| # | Item | Evidence | Size | Prerequisites |
|---|---|---|---|---|
| 1 | Front-end round 23, with library-code targets | 2 wrong results, 8 false rejects, 3 missed rejects, and 10 kinds of stop in this report, P1, I4, and round 8; 0 of 5 new programs run on the first try | large | item 2 for the targets without tests |
| 2 | Tests for every front-end target that has none | 17 reductions here, P1-1, P1-2, I4-4 to I4-7, and 3 round 8 forms have no test; the TR-12 form stops 2 of 5 programs | small | none |
| 3 | Answer the 21 planned rows whose item ended, starting with P1's | close commit `a15a877d`; P1-7, P1-3, P1-5, P1-8, and the PACK-13 example are each a few lines | small to medium | none |
| 4 | Repair the ledger, and check planned rows against open phases | 11 rows `open` that report 22 calls done; 21 `planned` rows name an ended item | small | none |
| 5 | Write suite W20 on library-shaped code, then Port P2 | every new stop is in a container, an optional, a callable, or a string; W18, W19, and P1 did not write these | one session per suite | items 1 and 2 |
| 6 | Interpreter output when every order agrees | `vec.cpp` and a short program stop at 64 runs; 3 of the 6 `r8-` programs print "unspecified: result 0" with no other outcome | small | none |
| 7 | Diagnose batches with the front end's message | missing `template` and missing `typename` give TR-12 first; a failed constraint gives DED-35 | one session per 4 pages | none |
| 8 | A count of grammar forms by test and front-end result | each new fault is again a form that no test uses | small script | none |
| 9 | Guides on owners, containers, and type erasure | deferred since phase 5; the five new programs are what such guides would show | one decision, then one session per guide | maintainer |

### Item 1. Front-end round 23, with library-code targets

**What.** Decision 0058 gives FE the P tests, G152, the template missed rejects, and the backlog groups. Round 23 should take the targets in this order:

1. The two wrong results. A pointer derived from one evaluation of a string literal keeps its identity, so a copy compares equal and a loop to `p + n` ends. These rest on PRIM-9 and ADD-10.
2. The false rejects:
   - PACK-8 on `T x(a...)`, TR-12 on `: m(a...)` in a `constexpr` constructor or constructor template, and CTOR-15 on `leaf<I>{v}...` (round 8),
   - OVL-35 on a `const R&` bound through `R`'s converting constructor (P1-2),
   - DED-35 on an array of unknown bound with converted elements (new), with P1-1's "unsupported",
   - PRIM-29 on a fold over a pack captured by copy (new),
   - ASN-2 (I4-4) and COND-5 (I4-1, I4-2).
3. The missed rejects: a conditional to an ambiguous or a private base (I4-6, I4-7), and a data member of function type in an instantiated class (INST-6, new).
4. The stops that library code meets:
   - a pack in a member or base initializer,
   - braced initialization through a constructor template,
   - an array of a class with a constructor template,
   - assignment through a converting constructor or a constructor template,
   - direct-initialization through a conversion function,
   - the class conditional forms of section 1.5 and I4-3 and I4-5,
   - a lambda with `-> decltype(auto)`,
   - a pack captured by copy,
   - a range `for` by value over move-only elements, which should reject with DEL-1.
5. The X23 aligned forms, the 4 remaining false rejects, the 40 run-unsupported tests, and the 116 missed rejects, grouped as report 22 groups them.

A stop at "line 0" should give the line of the declaration (P1-1, the array of `F`, the `decltype(auto)` lambda).

**Why.** Sections 1.4 and 1.5. A programmer can trust the front end's "undefined" only when it is never wrong on a correct program. The literal faults break that for any view type over a literal, which library code writes often (general knowledge). The false rejects and stops come in forms that every container, optional, or callable uses: forwarding in a member initializer, a `const&` parameter that takes a converted argument, an array of elements with a constructor template, and assignment from a convertible value.

**Size.** Large. The interpreter's model of literal objects, the parse of an expansion inside an initializer, and the conversion paths for class sources each touch many forms.

**Prerequisites.** Item 2, so that `tools/frontend_check.py` shows each fix.

### Item 2. Tests for every front-end target that has none

**What.** A clean-room writer adds a test for each reduction of section 1.5, for P1-1, P1-2, and I4-4 to I4-7, and for the three round 8 forms that item P left (section 1.2). Each test is probed on the matrix. The two literal tests are run tests under PRIM-9 and ADD-10 that expect result 0. The function-type member is a reject test under INST-6. The integrator copies the last versions of the five new programs into `tests/programs/roadmap/` as `r9-` program tests, and P1's three ports into `tests/programs/P1/`.

**Why.** The front end passes every test and fails these forms. P1 and I4 recorded front-end faults with reductions, and no commit turned them into tests, so round 23 has no check for six of its planned targets. Decision 0055 asks the commit that acts on a roadmap round to add each correct sketch and reduction.

**Size.** Small. Each reduction has at most 14 lines.

**Prerequisites.** None. The tests are the first commit of item 1.

### Item 3. Answer the 21 planned rows whose item ended, starting with P1's

**What.** For the programmer, these come first:
- `index.md` names the base flags `-std=c++23 -pedantic-errors` beside "strict mode" (P1-7).
- An example of a class with two alternatives: a tag, an anonymous union, a constructor for each, and a hand-written `==`, citing DCMP-2 (P1-3).
- A link from `virtual.md` to the `Owner` example (P1-5).
- A library-free requirement that a comparison gives a truth value (P1-8).
- The example of a forwarding constructor that round 8 item 2 asked for under PACK-13.

Then I4-8 to I4-14, X23-3, X23-6, X23-7, I3-1, I3-5, I3-13, R10-4, R10-6, and P1-9, each with the probe that its row names.

**Why.** Section 1.8. The P1 items are what a programmer who ports or writes library code looks for, and P1 found each one missing. The count of example blocks did not change in phase 11.

**Size.** Small for the P1 items. Medium for the rest, which need probes.

**Prerequisites.** None.

### Item 4. Repair the ledger, and check planned rows against open phases

**What.** The 11 rows of section 1.8 move to `commit` with the round 22 commit `eff1a372`. Each of the 21 planned rows names an item of the new phase or a new state. `check_specification.py` fails on a `planned` row whose item belongs to a phase whose decision says it is done. The script that sets rows also reads the target tables of front-end reports.

**Why.** Section 1.8. The ledger exists so that no finding is lost. A row that says `open` for a fixed fault, or `planned` for an item that has ended, gives a wrong answer to that check.

**Size.** Small.

**Prerequisites.** None.

### Item 5. Write suite W20 on library-shaped code, then Port P2

**What.** W20 (C# or Java persona), with four tasks: a `Vec<T>` with `emplace_back`, an `Optional<T>` that keeps its value in a union, a type-erased callable, and a sorted map with `try_emplace`. Port P2 follows with a Java or C# persona, whose generic collections and delegates map to the same forms (general knowledge).

**Why.** Every new stop in section 1.4 is in this kind of code. Round 8 asked for W20, and decision 0058 deferred it. W18, W19, and P1 wrote callables, compile-time code, and ports of short Rust programs, and none of them wrote a forwarding constructor or an array of type-erased objects.

**Size.** One session per suite.

**Prerequisites.** Items 1 and 2, so that the suite measures the fixed front end.

### Item 6. Interpreter output when every order agrees

**What.** Round 8 item 9, unchanged. When every explored order gives one result, `tidal-fe run` prints the result first, such as "result: 0 (every order of 4 gives 0)". When the bound stops the exploration, the line says so first.

**Why.** `vec.cpp` prints "unspecified: result 0" and stops at the bound of 64 runs. So does the recursive lambda short program. The round 8 programs `r8-config`, `r8-owner`, and `r8-pipeline` print "unspecified: result 0" with no other outcome. A programmer reads "unspecified" as a fault in the program.

**Size.** Small for the message.

**Prerequisites.** None.

### Item 7. Diagnose batches with the front end's message

**What.** Round 8 item 6, unchanged: batches for templates, dependent names, and constraints, each scoring whether the `tidal-fe check` message leads to the rule.

**Why.** Section 1.6. A missing `template` and a missing `typename` both give a TR-12 parse message first. Clang names the keyword in each case.

**Size.** One session per 4 pages.

**Prerequisites.** None.

### Item 8. A count of grammar forms by test and front-end result

**What.** Round 8 item 7, unchanged: a script lists each production of the Syntax blocks with the tests that match it and their front-end status.

**Why.** Each of the 14 new faults is a form that no test uses: a capture `[xs...]` in a generic lambda, `-> decltype(auto)` on a plain lambda, an array of a class with a constructor template, and the rest of section 1.5.

**Size.** A small script.

**Prerequisites.** None.

### Item 9. Guides on owners, containers, and type erasure

**What.** A maintainer decision on publishing guides, then how-to guides for a growable array in raw storage, an optional in a union with `std::construct_at`, and a type-erased callable.

**Why.** Rounds 3 to 8 asked for guides. Two of my mistakes from memory (placement `new` in constant evaluation, a function type as a member) are what such guides would prevent.

**Size.** One decision, then one session per guide.

**Prerequisites.** The maintainer.

## 3. Out of scope

| Topic | Proposal | For | Against |
|---|---|---|---|
| The general library, including `forward`, `move`, `decay`, `function`, `optional`, and `string_view` | Keep out. Programs define their own, as the five new programs do. | Each program needs them (general knowledge). | The charter leaves out the library. Each took a few lines. |
| `std::construct_at` beyond LSUP-10's shape | Keep as is. | | LSUP-10 and the unions example gave what `optional.cpp` needed. |
| A sanitizer for uninitialized reads | Keep out of the matrix. | The front end catches the read, and ASan and UBSan do not. | General knowledge: that sanitizer needs a build of every library with it. |
| `typeid` | Keep deferred. | | Each test needs the library's `type_info` (Q-0322). |
| Threads, the memory model, coroutines, modules, extended floating-point types | Keep out. | | as the charter says |
| Language versions after C++23 | Keep out until `pending.md` row 7 is decided. | | Strict mode rejects them. |

## 4. Process changes

1. **A front-end finding of any exercise gets a test in the commit that records it.** P1 and I4 recorded six front-end faults with reductions and no test, so round 23 cannot show their fixes.
2. **The ledger check fails on a `planned` row whose item has ended,** and the ledger script reads the target tables of front-end reports (section 1.8).
3. **Port programs become program tests,** as Write programs do. `exercises.md` section 3 says so only for Write suites.
4. **A planted mistake that the front end runs becomes a reject test,** as the data member of function type should.
5. **A front-end round lists grammar forms with no passing test** (round 8 change 5, item 8).

## 5. Suggested order

1. Item 1 comes first in the phase, as decision 0058 says. Item 2's tests are its first commit, so that each target has a check. Its first target after that is the two wrong results.
2. Items 3 and 4 run beside item 1. Each is small, and neither touches `frontend/`.
3. Item 5 follows item 1.
4. Items 6, 7, and 8 run beside the others at any point. Item 9 waits on the maintainer.

## 6. Doubts for the clean-room writers

- Whether PRIM-9 should say that a pointer from one evaluation of a literal, and every copy of it, refers to one object. The rule speaks of two literals and two evaluations. The front end applies it to one.
- Whether INST-6's list names a non-static data member of function type. Its test of the form uses a static member.
- Whether a range `for` by value over move-only elements belongs in a RFOR rule or only under DEL-1. Every compiler rejects it.
- Whether the specification should say where a program without headers gets the step that turns a function type into a pointer for storage (`function_first_try.cpp`). The step rests on general knowledge of the library.
- Whether "unspecified: result 0" should stay the first word when every explored order gives the same result (rounds 7 and 8, still open).
