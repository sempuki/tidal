# Roadmap, round 13: what to do after phase 15, from a programmer's view

Date: 2026-11-11. Decision 0029. This report comes from a clean-room reader in the role of a C++ programmer who uses the specification to write and check programs.

**What the reader read.**
- AGENTS.md, CLAUDE.md, and the round 12 programmer report (`reports/clean-room/roadmap-12/programmer.md`) with its sketches and logs.
- The status page, the roadmap's phase 15 table, decisions 0060, 0062, and 0063, and `documentation/decisions/pending.md`.
- `git log 51bd5060..HEAD` with every commit body (21 commits: decision 0063, the items of phase 15, three ledger commits, a runner fix, and the close).
- The X25 report, the P4 report, front-end report 26, the RV8 records of 2026-11-11 in `reports/review/` (the read of the RV7 repair diff, the two phase 15 gap reviews, and the read of the RV8 repair diff), `frontend/DESIGN-current.md`, `frontend/TEMPLATE-MODEL.md`, `frontend/DESIGN-rewrite.md`, `rewrite-marks.tsv`, `reports/clean-room/ledger.md`, `rust-frontend/ledger.md`, and `conformance.tsv`.
- The rules and tests that the findings name: CPY-1, CPY-4, CPY-5, OVL-9, OVL-24, OVL-27, CNS-3, CNS-7, CNS-24, DED-31, TSP-14, TARG-P4, INST-1, INST-2, ACC-5, ACC-20, NAM-5, DEL-1, INIT-9, CTOR-5, CTOR-25, VIRT-12, LSUP-1, and the tests `fold-atomic.cpp`, `eligible-copy.cpp`, `trivially-copyable-constraint-fails.cpp`, `tie-conversion-result-before-constraints.cpp`, `class-decltype-call-argument-instantiated.cpp`, `class-qualified-name.cpp`, `most-derived-must-init-virtual-base.cpp`, and `pure-call-in-ctor-sanitizer.cpp`.

The reader did not read the other round 13 reports, `frontend/` sources, `inputs/`, or `build/exercise-keys/`. Commit read: `a6a2b16c`, the close of phase 15.

**What the reader did.** The reader built the front end from `a6a2b16c` into its scratchpad with cargo, with no build failure. The reader reran the 79 round 12 files and wrote five new programs (994 lines) with 13 earlier tries, 22 short programs and one earlier try of a short program, 8 planted bugs, 9 planted mistakes, 11 reductions, and 25 probes. Each file went through these checks:
- host GCC 16.2.1 and Clang 22.1.8 with `-std=c++23 -pedantic-errors -Wall -Wextra`,
- the same with `-std=c++23 -pedantic-errors` alone,
- the same with `-fsanitize=undefined,address`,
- the EDG 7.0 container (`cpfe --c++23 --strict --no_code_gen --instantiate=used`),
- `tidal-fe run`.

Everything is in `programmer-sketches/` beside this report:
- `round12/` holds the copies that were rerun, and `results-rerun.txt` is their log (`rerun12.sh`). `compare_rerun.py` compares the log with round 12's log.
- `new/` holds the new programs, their earlier tries, `one-line/`, `bugs/`, and `mistakes/`. `reductions/` and `probes/` hold the reductions and probes. `results-new.txt` is their log (`runnew.sh`).
- `runall.sh` runs one program on every tool. `reduce.py` is the line-based reducer of round 12, pointed at this round's build. `work/` holds one intermediate cut.
- `counts.py` gives the counts of section 1.1 from `git show`. `ledger_counts.py` counts both ledgers by state. `scan_tests.py` scans `tests/` for the forms of the reductions (`scan.txt`). `grammar_first_errors.py` lists the reject tests whose first front-end error is TR-12 (`grammar-first-errors.txt`). `summarize.py` counts the front-end results of a log (`summary-new.txt`, `summary-rerun.txt`).

Statements that rest on general knowledge say so. Section 7 lists the claims that rest on the host compilers alone.

## 1. The state from a programmer's view

Front-end round 26 fixed 7 of the 9 round 12 reductions and the round 12 wrong verdict. The two braced-list reductions still stop in the run. Four round 12 clean intermediate tries now run. Three of the four mistakes that round 12 found hidden behind a false reject now get the compilers' first error. The fourth now gives a wrong verdict of its own (section 1.3).

The new programs are again library code: a quantity type with a dimension as a constant template parameter of class type, a fixed-size bit set with a proxy reference, an intrusive list with one hook base per list, a slot map with generational keys over a union array, and algorithms as function objects with a customization point found by argument-dependent lookup.
- Two first tries (`bits_first_try.cpp`, `slotmap.cpp`) build clean and exit 0 on GCC and Clang. The slot map runs on the front end with no change. The other three first tries fail to build, and each failure is my mistake.
- After workarounds, all five run on the front end. Rounds 10 to 12 also ran all five after workarounds.
- `slotmap` needed no workaround. `bits`, `intrusive`, and `algo` needed one each, and `units` needed five.
- Eight intermediate tries build clean and exit 0 on GCC and Clang, and the front end stops or rejects each of them. Six are tries of `units`.

The 11 reductions show 10 faults. Nine are new in kind, and one is a new form of the P3-10 stop. By reduction:
- 1 wrong verdict on a valid program: a partial specialization whose argument is a value of class type matches every value, so the program returns 2 where the compilers return 0,
- 4 false rejects,
- 4 stops,
- 2 missed rejects.

The front end runs every short program and catches planted errors well. It runs 22 of 22 short programs, stops 7 of 8 planted bugs with `undefined` and a rule, and rejects 8 of 9 planted mistakes. Its first error names the rule of the cause in 5 of the 9.

### 1.1 Counts

| Measure | Round 12 (`bd87b27c`) | Now (`a6a2b16c`) | Source |
|---|---|---|---|
| Rule headers | 1700 (1685 in force, 15 withdrawn) | 1700 (1685 in force, 15 withdrawn) | `counts.py` |
| Example blocks | 230: 73 unmarked, 28 `fragment`, 113 `reject`, 16 `diverges` | 238: 78 unmarked, 29 `fragment`, 114 `reject`, 17 `diverges` | `counts.py` |
| Tests | 6947 | 7015 | `counts.py`, files with `tidal-rules`, `.tu.cpp` left out |
| Program tests | 138 | 152 (9 `r12-`, 5 P4 added) | `counts.py` |
| Lines under `tests/programs/` | 17358 | 20156 | `counts.py` |
| Rules cited by a program test | 568 of 1685 | 577 of 1685 | `counts.py` |
| Tests that open `namespace std` | 278, 72 of them program tests | 293, 82 of them program tests | `counts.py` |
| Front end | 6804 of 6947 pass, 6831 their check | 6895 of 7015 pass, 6926 their check (6892 of 7007 and 6921 at round 26) | close commits `bd87b27c`, `a6a2b16c` |
| Completion line of decision 0060 | 107 tests fail their check, 3 of 138 program tests do not run, 9 out of scope | 80 tests fail their check (32 marked, 48 toward the gate), 5 of 152 program tests do not run, 9 out of scope | close commits |
| `conformance.tsv` | 6938 rows | 7007 rows: 6559 pass, 290 EDG-ONLY, 27 pass-partial, 10 EDG-ALLOWED, 6 NO-DIAGNOSTIC-ALLOWED, 37 MISSED-REJECT, 36 unsupported, 29 run-unsupported, 7 FALSE-REJECT, 6 EDG-AGREES | the file |
| Changes under `frontend/` | round 25 and item RP: 20 files, 1583 lines added, 38 removed | round 26, GC, and RD: 17 files, 1160 lines added, 77 removed | `git diff --stat bd87b27c a6a2b16c -- frontend/` |
| Port suites done | 3 of 8 (P1 to P3) | 4 of 8 (P1 to P4) | `exercises.md` section 8 |
| Questions | 434: 185 open | 446: 197 open, 236 settled, 13 compiler defects | `status.md` |
| Front-end ledger | 165 rows, 2 open | 165 rows: 156 commit, 4 text, 4 out of scope, 1 question, 0 open | `ledger_counts.py` |
| Exercise ledger | 1044 rows, 26 open | 1097 rows: 1004 commit, 44 text, 30 question, 3 out of scope, 16 open | `ledger_counts.py` |

### 1.2 Round 12 items that phase 15 took and left

| Round 12 item | Phase 15 | Result |
|---|---|---|
| 1. Tests for this round's reductions, probes, and clean tries | item T4 | Done: the 9 reductions, the 2 probes that stopped, the 5 last versions as `r12-` program tests, and the 4 clean intermediate tries. `r12-matrix` carries the EDG expectation of Q-0403. |
| 2. Front-end round 26, with library-code targets first | item FE | Done for the wrong verdict, the friend fault, the lambda's braced return, the literal operators, PRIM-2, and TPL-7. The braced list beside a constructor template now passes its check and still stops in the run. P3-7 and P3-8 are done, and P3-9 and P3-10 stay open as run stops. `r10-tuple-second-try` still fails. |
| 3. Text for literal operators and STG-28, and G164 and G165 | item F5 | Done. RV8-A-1 reads STG-28's step to element `k` against the reference, and `pending.md` row 4 holds it with Q-0430. |
| 4. Answer the 26 open rows | item F5 | Done. I7-6, I7-13, I7-14, and P3-6 went to questions. |
| 5. Program first tries and their mistakes in the completion count | none | Deferred again by decision 0063. |
| 6. A gap review of the 20 changed rules | item RV8 | Done, with reads of the RV7 and RV8 repair diffs. The second read withdrew two points of the first (RV9-R-1, RV9-R-2). |
| 7. The rewrite gate, read with library programs | items GC and RD | Done. The marks file counts the template model apart, and round 26 ended inside the gate. |
| 8. Port suite P4 on iterators and owned resources | item E | Done with a Swift persona. Two of three first tries stopped (P4-1, P4-2). |
| 9. Guides | none | Deferred. |

Round 12 process changes 1 and 2 are in place: T4 added the reductions as tests and the clean intermediate tries as program tests. Decision 0063 deferred change 3, the count of library programs beside the completion line.

### 1.3 The round 12 sketches, rerun

`compare_rerun.py` reads the 79 files of `results-rerun.txt`. 62 give the same front-end result as in round 12, and 17 changed. The compilers and EDG give the same result on every file as in round 12, apart from the exit codes of planted bugs.

Of the 17 changes:
- 7 are round 12 reductions. Each now runs to result 0, as GCC and Clang do.
- 2 are probes that now run: the specialization with an array member as a constant parameter with explicit arguments, and placement through a byte pointer of a class with only a user destructor.
- 4 are clean intermediate tries that now run to 0: `ring_third_try.cpp`, `ring_fifth_try.cpp`, `bigint_second_try.cpp`, and `parse_third_try.cpp`.
- 4 are tries with my mistakes, which section 1.6 explains.

The two reductions that do not change are `braced_init_beside_failed_constructor_template.cpp` and `braced_init_beside_constructor_template_no_conversion.cpp`. Each still stops with "unsupported: constructor call". Their T4 tests pass their check and stop in the run (report 26, section 9). `ring_third_try.cpp` returns a braced list of two elements to a class of the same kind and now runs. The two reductions initialize a variable from a list of one element.

The short program `class_type_template_parameter.cpp` still stops with "cast of unknown operand". Report 26 lists its test (`constant-parameter-class-template.cpp`) among the 45.

### 1.4 New programs

| Program | GCC, Clang (three flag sets) | EDG 7.0 | `tidal-fe run`, first try | `tidal-fe run`, last version |
|---|---|---|---|---|
| `units.cpp` (192 lines): `Quantity<Dim D, T>` with `Dim` a class type, `*` and `/` that add and subtract dimensions, literals `_m`, `_km`, `_s`, `_kg`, a sum constrained to one dimension, a trait by partial specialization, unit text made in constant evaluation, `Vec2` with a deduction guide | first fails (my mistake); second to seventh exit 0, clean; last exits 0, clean | rejects the first try as GCC and Clang do, and every later version because a defaulted `<=>` asks for `std::weak_ordering` (Q-0386) | rejected, TR-12, on the fold in a requires clause | result 0, after five workarounds |
| `bits.cpp` (209 lines): `Bits<N>` over 64-bit words, a proxy `reference` with `operator=`, `flip`, and a hidden-friend `swap`, shifts across words, `&`, `\|`, `^`, `~`, a constructor from `"1011"`, a range over the set bits with its own end type, a sieve of primes | first try exits 0, clean | accepts | unsupported: "class assignment" in a `static_assert` | result 0, after one workaround |
| `intrusive.cpp` (175 lines): `Hook<Tag>` as a base once per list, `IList<T, Tag>` with a sentinel, `insert_sorted`, `erase`, `splice`, hooks that unlink when their object ends, copies that are not linked, constant tests | first fails (my mistake); second and last exit 0, clean | as GCC and Clang | unsupported: "object of unknown type". The second try, clean on GCC and Clang, gives the same. | result 0, after one workaround |
| `slotmap.cpp` (170 lines): `SlotMap<T, N>` with values packed in a union array, keys with a generation, erase by moving the last value into the hole, a free list, a value type that counts its live objects, constant tests | first try exits 0, clean | accepts | result 0 | the first try is the last version |
| `algo.cpp` (248 lines): `lib::swap` as a customization point, `lib::invoke` with pointers to members as projections, concepts for iterators, `sort`, `lower_bound`, `reverse`, `rotate`, `unique` over a linked range, `merge`, a card type with its own `swap` that counts calls | first and second fail (my mistakes); third and last exit 0, clean | as GCC and Clang | rejected, CPT-34, on my mistake. The third try, clean on GCC and Clang, gives SA-1. **False reject.** | result 0, after one workaround |

The workarounds:
- `units`: a variable template `all_same_dim<Q, Qs...>` in place of the fold `(SameDim<Q, Qs> && ...)` in the requires clause. Then `is_speed` reads `Q::dim` in place of a partial specialization over `Quantity<length - duration, T>`. Then a constrained member `number()` in place of the constrained conversion function, which also cost the conversion `static_cast<long long>(q)`. Then an alias `Quotient` in place of `decltype(10_m / 2_s)::dim`. Then a `constexpr Force` variable in place of `decltype(2_kg * (3_m / (1_s * 1_s)))` as a template argument.
- `bits`: `b[5] = static_cast<bool>(b[2])` in place of `b[5] = b[2]`.
- `intrusive`: a declaration of `IList` before `Hook`, whose friend declaration names it.
- `algo`: no deleted `swap` template in `lib::detail`. The concept that looks for a `swap` by argument-dependent lookup works without it in this program, because `lib::swap` is declared after the concept (general knowledge).

Each first try and each intermediate version is kept beside the last version (`*_first_try.cpp` to `*_seventh_try.cpp`).

### 1.5 Reductions

Nine reductions build and exit 0 on GCC and Clang under the three flag sets, and EDG accepts them. Two are reject reductions, which GCC, Clang, and EDG reject and the front end accepts. `scan_tests.py` scans `tests/` for the forms (`scan.txt`), and I read each hit:
- `fold-atomic.cpp` has a fold in a requires clause. It is a reject test, and the front end passes it with TR-12 as its first error, where the test cites CNS-3 and CNS-24.
- `eligible-copy.cpp`, `trivially-copyable-constraint-fails.cpp`, and `r10-list.cpp` constrain a copy constructor beside an unconstrained one. No test constrains the only copy constructor.
- `tie-conversion-result-before-constraints.cpp` constrains two conversion functions whose constraints hold. No test has a conversion function whose constraint fails.
- Four tests declare a conversion function template, and none uses a binary operator on the class.
- Four tests use `decltype(v)::` with a variable `v`, and a fifth hit is a comment in `W19/typelist.cpp`. `class-decltype-call-argument-instantiated.cpp` and `class-qualified-name.cpp` are reject tests with plain functions.
- `W5/deleted.cpp` deletes a template beside a plain function in one scope, with no argument-dependent lookup.
- No test has a friend class template that names a template declared later, or a partial specialization with a braced argument of class type.

| Reduction | Lines | `tidal-fe` | Status | What the specification says |
|---|---|---|---|---|
| `partial_specialization_class_type_constant_argument.cpp`: `template <D V, class T = int> struct Q`, a partial specialization `is_one<Q<D{1}, T>>`, then `is_one<Q<D{2}>>::value` | 8 | result 2. **Wrong.** GCC and Clang give 0. | New. An `int` argument and an explicit specialization with `D{1}` work (probes). | TSP-14: the partial specialization matches only when deduction gives the argument list. TARG-P4: two class values are the same argument when each member is the same. |
| `fold_in_requires_clause.cpp`: `template <class... T> requires (Small<T> && ...)` | 5 | rejected, TR-12: "expected an expression, found '...'". **False reject.** | New. A fold of plain expressions in a requires clause gives the same. A fold of concept-ids in a return statement runs (probes). | CNS-3: a fold in a constraint is one atomic constraint. `fold-atomic.cpp` passes on this fault. |
| `conversion_function_unsatisfied_constraint.cpp`: `explicit operator int() const requires B`, then `!HasNumber<Q<false>>` | 8 | rejected, SA-1. **False reject.** | New. `int x = q` for `Q<false>` runs where every compiler rejects, and two constrained conversion functions stop with "conversion function" (probes). A constrained member function is checked (probe, CNS-12). | OVL-24 and CNS-7: a candidate whose constraints fail is not viable. |
| `adl_function_beside_deleted_template.cpp`: `n::f(C&)` found by argument-dependent lookup beside a deleted `template <class T> int f(T&)` at namespace scope | 3 | rejected, DEL-1. **False reject.** | New. The same with a template that is not deleted runs and calls `n::f`. A plain function beside a deleted template in one scope runs (probes). | NAM-5: the function found by argument-dependent lookup joins the set. OVL-27: a function that is not a specialization wins. DEL-1 applies only to the function chosen. |
| `decltype_qualifier_specialization_from_template_return.cpp`: `template <int A, int B> Q<A - B> div(Q<A>, Q<B>)`, then `decltype(div(Q<1>(), Q<2>()))::dim` | 4 | rejected, NAM-9: no member named `dim`. **False reject.** | New. An alias of the same `decltype`, or a template whose return type is `Q<A>`, runs (probes). | INST-1: a qualified name that names a member instantiates the class. INST-2 keeps the call from instantiating it, and the qualifier then needs it. |
| `decltype_nested_operator_template_calls.cpp`: `decltype(Q<1>() * (Q<2>() * Q<3>()))` as a template argument, with `operator*` a template that returns `Q<A + B>` | 6 | unsupported: "overloaded operator (in a variable template argument)" | New. One cause with the reduction above. The same expression in an evaluated context runs (probe). | INST-2: a call nested as an argument of another call instantiates the class. |
| `binary_operator_beside_conversion_function_template.cpp`: `template <class U> explicit operator U()` in `Q`, and `Q{10} / Q{2}` with a user `operator/` | 8 | unsupported: "member of unknown" | New. A constrained conversion template gives the same (probe). | OVL-9 and DED-31: the conversion template gives no built-in candidate here, and the user operator is chosen. |
| `friend_class_template_declared_later_in_class_template.cpp`: `template <class> friend class IList;` in a class template, with `IList` declared after it | 10 | unsupported: "object of unknown type" | New. A declaration of `IList` before the friend, or a class that is not a template, runs (probes). | ACC-20: a friend declaration with a template head names a class template, and every specialization is a friend. |
| `assignment_beside_converting_assignment.cpp`: `R& operator=(int)` and `R& operator=(const R&)` beside `operator int()`, then `x = y` | 12 | unsupported: "class assignment" | New form of the P3-10 stop. Without the assignment from `int`, the program runs (probe). | OVL-7 and OVL-25: the copy assignment takes `y` by an exact match and wins. |
| `copy_constructor_unsatisfied_constraint.cpp` (reject): `It(const It<false>&) requires C` in `It<C>`, then a copy of `It<false>` | 10 | result 1. **Missed reject.** The front end calls the constructor whose constraint fails. | New. The cause of `ring_first_try.cpp`'s new verdict. A converting constructor with a failed constraint is checked (probe). | CPY-1 and CPY-4: the constructor is a user-declared copy constructor, so the class has no implicit one. OVL-24: it is not viable, so the copy has no constructor. |
| `member_without_default_constructor_not_initialized.cpp` (reject): `Server() {}` with a member `Port` that has no default constructor | 3 | result 0. **Missed reject.** With an object of `Server`, the run stops with "default constructor" (probe). | New. `most-derived-must-init-virtual-base.cpp` holds the base form, and it passes. | INIT-9: the member is default-initialized. CTOR-25: no constructor accepts no arguments. |

By status: 11 are new. The two `decltype` reductions have one cause. The conversion-function and copy-constructor reductions both skip a trailing requires clause, and I count them apart because a constrained converting constructor and a constrained member function are checked. So the 11 reductions show 10 faults. Nine are new in kind, and the assignment stop is a new form of P3-10.

**A fault with no reduction of its own.** A range-based `for` over a braced list with no `std::initializer_list` declared stops with "range-based for", where every compiler rejects it (LSUP-1). `initializer-list-missing.cpp` holds the `auto` form, which the front end rejects. The probe `range_for_braced_list_without_initializer_list.cpp` holds the loop form. My first try of `intrusive` met it.

**An EDG split that a test holds.** EDG rejects every version of `units.cpp` after the first, because its defaulted `<=>` looks for `std::weak_ordering`. Q-0386 and the P4 money programs hold the case, so `units.cpp` needs the same EDG expectation as a program test.

### 1.6 Short programs, planted bugs, and planted mistakes

**Short programs.** The 22 short programs in `new/one-line/` build and run clean on GCC and Clang with every flag set. Clang warns twice with `-Wall -Wextra` on `aggregate_with_base.cpp`, for braces left out around the base. EDG accepts all 22. The front end runs all 22 (`summary-new.txt`: 17 "result: 0" and 5 "result: 0 in every explored order"). The forms are:
- a lambda with a `static` call operator, designated initializers, `using enum` in a `switch`, a lambda with a template head and a requires clause, and `new` and `delete` in constant evaluation,
- `constinit`, an aggregate with a base, `==` with an `int` on the left by rewriting, inherited constructors, and a recursive lambda through `this auto`,
- a partial specialization of a variable template, an abbreviated function template with a pack, a `switch` with an init statement, a move out of an rvalue reference parameter named in a return, and an `auto` constant parameter,
- an attribute on a lambda, a label at the end of a block, an alias declaration as the init statement of a range `for`, a pointer to a base member applied to a derived object, `decltype(auto)` on a parenthesized member, a delegating constructor, and a conditional `noexcept`.

No short program met a front-end fault this round. The faults of section 1.5 all came from the library programs.

**Planted run-time bugs.** The front end stops 7 of the 8 bugs in `new/bugs/` with `undefined` and a rule:

| Rule the front end names | Bug | Sanitizers |
|---|---|---|
| PFX-4 | a loop reads one element past the end of an array | ASan |
| ADD-9 | a sum of two `int` values overflows | UBSan |
| SHF-7 | `1u << 32` | UBSan |
| NEW-5 | an object deleted twice through two pointers | ASan; the plain builds abort |
| UNI-6 | a union written through `float` and read through `int` | silent |
| DER-17 | `static_cast` to `Square&` of a `Circle` | UBSan (GCC only) |
| MUL-9 | an average of an empty range divides by zero | UBSan; the plain builds trap |
| none: "unsupported: function 'run' has no definition" | a base constructor reaches a pure virtual function through a helper | the run-time library stops every build |

The inactive union read passes every sanitizer, and only the front end names it. The pure virtual call is VIRT-12, and `pure-call-in-ctor-sanitizer.cpp` holds it as one of the 29 run stops.

**Planted compile-time mistakes.** GCC, Clang, and EDG reject all 9 mistakes in `new/mistakes/`. The front end rejects 8:
- Its first error names the rule of the cause in 5: LIST-2, OVL-36, CTOR-9, TYN-7, and CLS-25.
- It names a general rule in 3: TR-12 for a missing `template` before a member template of a dependent object (DEP-24), OVL-35 for a temporary bound to a non-`const` lvalue reference (a REF rule), and DED-35 for an argument that fails a concept (OVL-24 and CNS-7).
- It does not reject `Server() {}` with a member that has no default constructor. The run stops with "default constructor" (section 1.5).

The 9 planted mistakes of round 12 rerun with the same first errors.

**My own mistakes from memory.** Four programs and one short program held five:
- `units_first_try.cpp`: my `std::strong_ordering` had no `==` with an `int`, so the defaulted `<=>` was deleted. The front end stopped first on the fold fault. With the fold worked around, it rejects the use of the deleted `<=>` with DCMP-11, as GCC does (probe).
- `intrusive_first_try.cpp`: a range `for` over `{&a, &b, &c}` with no `std::initializer_list`. The front end stopped first on the friend fault. With the friend worked around, it stops on the loop (section 1.5).
- `algo_first_try.cpp`: `{ i == i } -> bool`, a type after `->` in a compound requirement, as in round 12. The front end named CPT-34, the rule of the cause.
- `algo_second_try.cpp`: `invoke(cmp, a, b)` with three arguments, where my `invoke` takes two. The front end stopped first on the deleted-template fault. Without the deleted template, it names DED-35 at the compilers' line (probe).
- `one-line/implicit_move_rvalue_param_first_try.cpp`: a `static_assert` with the wrong sign. The front end named SA-1, as every compiler does.

Three of my five mistakes sat behind a front-end fault. Round 12 had four of ten.

**The round 12 mistakes that were hidden.** Three now give the compilers' first error: `bigint_first_try.cpp` (OVL-35 on the missing division), `parse_second_try.cpp` (DED-35 where the concept fails), and `ring_second_try.cpp` (SA-1). `ring_first_try.cpp` now gives SA-1 at line 195, where every compiler rejects the copy at line 168 or 182. Its reduction is the missed reject of the constrained copy constructor.

None of this round's findings rests on the 20 rules and principles that changed after the RV8 reviews read the tree, except the two `decltype` reductions. Those rest on INST-1 and INST-2, which changed in the RV8 repairs.

### 1.7 The phase 15 reports, from a programmer's view

**Front-end report 26.** The round did what round 12 asked for library code first: the wrong verdict, the friend fault, the lambda's braced return, the literal suffix on a long literal, and the literal operator template. My workarounds of round 12 are no longer needed, except the braced list beside a constructor template in a plain variable. Report 26 section 5 names a fault with no test: a literal operator template called by name with explicit arguments. Section 8 gives a cause for each of the 45 tests that count toward the gate. 23 of them are unsupported constructs, among them CTAD sites, a fold in constant evaluation, and an expansion of a template template pack.

**`frontend/DESIGN-rewrite.md`.** The design takes over the marked tests by model section, and the build reaches the program tests only in step 8. Its section 5 still lists 30 marked tests and plans step 5 as "16 tests" and step 6 as "10 tests". The marks file now has 32 rows: 4 for section 1, 6 for section 2, 11 for section 3, 4 for section 4, and 7 for section 5. So step 5 takes 17 and step 6 takes 11. From my side, none of this round's 10 faults is in the 32, and only the `decltype` fault sits near a model section (section 3, a specialization made at its point). The others are lookup, overload resolution, constraint checks, and the interpreter's assignment, which steps 3 and 4 of the build write again.

**P4.** P4-1 and P4-2 stop the two P4 first tries, which are two of the five program tests that do not run. Both are library forms. P4-3 and P4-4 ask for examples that my `algo` program needed too: an adaptor over any range, and the result type of a member call with no library. My concept for `input_iterator` used a requires-expression parameter, which P4-4 calls the cleaner form. P4-5 asks for the move constructor of the `Maybe` example. My slot map moved its values with a user move constructor, and the example would have saved me a read of CPY-5.

**X25.** My slot map keeps an array of unions, one value per slot, which is the form that works today. X25 designs the other form, an array member of one union whose elements start one at a time. X25-4 and X25-5 are open doubts on the run-time reading of that form, and my program does not depend on them.

**The RV8 records.** The read of the RV8 repair diff withdrew two points of the read of the RV7 diff (RV9-R-1, RV9-R-2), and the records repaired four places that held a disagreement the reference does not have. For a programmer, the readings that stay against the compilers are STG-28's element `k`, the integer step of Q-0442, and the CTAD rows. My programs avoid byte arithmetic, so none of these changes a verdict on them.

### 1.8 Ledger rows

The status page names 16 open exercise rows (P3-9, P3-10, P4-1 to P4-5, P4-7, X25-1 to X25-7, and X25-9). `ledger_counts.py` gives the same 16 and no open front-end row. No row is `planned`. P3-9, P3-10, P4-1, and P4-2 name their tests and wait for a front-end round. This round's assignment reduction is a second form of P3-10.

## 2. Ranked work items

| # | Item | Evidence | Size | Inputs |
|---|---|---|---|---|
| 1 | Tests for this round's reductions, probes, and clean tries | 11 reductions, 10 probes that the front end fails, 2 clean first tries, 8 clean intermediate tries | small | `reductions/`, `probes/`, `new/` |
| 2 | Front-end round 27 on the current design, with library-code targets first | 1 wrong verdict, 4 false rejects, 4 stops, 2 missed rejects; P3-9, P3-10, P4-1, P4-2; report 26 section 5 | large | item 1 |
| 3 | The rewrite, steps 1 to 4, measured by programs as well as tests | the gate holds; `DESIGN-rewrite.md` sections 5 and 9; 5 of 152 program tests do not run | large, over several phases | decision 0060, `DESIGN-rewrite.md`, the marks file |
| 4 | Answer the 16 open rows, starting with the rows a programmer meets | P4-3, P4-4, P4-5, P4-7, then the X25 rows | medium | the P4 and X25 reports |
| 5 | A gap review of the 20 rules and principles changed after RV8 read the tree, with a read of its repair diff | close commit `a6a2b16c`; INST-1 and INST-2 under this round's `decltype` fault | one review session, then repairs | none |
| 6 | Reject tests that pass on a grammar error | `fold-atomic.cpp` passes on TR-12; 129 of 2823 single-file reject tests get TR-12 first and cite no TR rule | small | `grammar-first-errors.txt` |
| 7 | Program first tries and their mistakes in the completion count | 8 clean intermediate tries stop; 3 of 5 mistakes hidden | small | `results-new.txt` |
| 8 | Port suite P5 on generic code and customization points | 8 of this round's 10 faults come from templates, constraints, and lookup in generic code | one session | item 2 |
| 9 | Guides on constrained special members, customization points, and class-type constant parameters | this round's `units` and `algo` | one decision, then one session per guide | maintainer |

### Item 1. Tests for this round's reductions, probes, and clean tries

**What.** A clean-room writer adds a test for each of these, probes it on the matrix, and writes the expectation that the compilers and the text give:
- run tests that expect result 0: the partial specialization with a class-type argument (TSP-14, TARG-P4); a fold in a requires clause, of concept-ids and of plain expressions (CNS-3); a conversion function whose constraint fails, read through a concept (OVL-24, CNS-7); argument-dependent lookup beside a deleted template (NAM-5, OVL-27, DEL-1); `decltype` of a template call as a qualifier and as a nested argument (INST-1, INST-2); a binary operator on a class with a conversion function template (OVL-9, DED-31); a friend class template declared later (ACC-20); an assignment beside an assignment from a converted type (OVL-7, OVL-25),
- reject tests: the constrained copy constructor (CPY-1, CPY-4, OVL-24); the member with no default constructor (INIT-9, CTOR-25); an implicit conversion through a conversion function whose constraint fails (OVL-24); a range `for` over a braced list with no `std::initializer_list` (LSUP-1),
- an accept test beside `fold-atomic.cpp` that runs a fold in a requires clause, so the fold no longer passes only as a reject,
- a test of report 26 section 5, a literal operator template called by name with explicit arguments (OPR-8),
- program tests: `bits_first_try.cpp` and `slotmap.cpp` as clean first tries, and the clean intermediate tries `units_second_try.cpp` to `units_seventh_try.cpp`, `intrusive_second_try.cpp`, and `algo_third_try.cpp`. The last versions of all five go in as `r13-` program tests, `units` with an EDG expectation for Q-0386.

**Why.** Sections 1.4 and 1.5. The front-end harness sees only tests. Decision 0061 makes a clean first try a program test, and T4 did the same for clean intermediate tries.

**Size.** Small. Each reduction has at most 12 lines.

**Inputs.** `programmer-sketches/reductions/`, `probes/`, `new/`.

### Item 2. Front-end round 27 on the current design, with library-code targets first

**What.** Round 27 works on the old front end beside the rewrite and takes the targets in this order:
1. The wrong verdict. A partial specialization matches a class-type argument only when each member is the same (TSP-14, TARG-P4).
2. The missed rejects. A trailing requires clause of a copy constructor takes part in CPY-4 and OVL-24, and a member with no default constructor that the list does not name is an error (INIT-9).
3. The false rejects: a fold in a requires clause, a conversion function whose constraint fails, argument-dependent lookup beside a deleted template, and `decltype(call)::member`.
4. The stops: the nested `decltype`, the binary operator beside a conversion function template, the friend class template declared later, the assignment beside a converting assignment with P3-10, and the braced list beside a constructor template in a plain variable.
5. P3-9, P4-1, P4-2, report 26 section 5, `r10-tuple-second-try`, and the 48 tests that count toward the gate.

**Why.** Sections 1.4 and 1.5. The rewrite reaches program tests only at step 8 of `DESIGN-rewrite.md`, so until then the old front end is the one that checks library code. These targets are what a library programmer writes first: a trait over a quantity type, a constrained sum, a conversion that only some specializations have, a customization point, and a friend container declared after its node. Five workarounds cost `units` its trait, its conversion, and two type checks.

**Size.** Large.

**Inputs.** Item 1's tests.

### Item 3. The rewrite, steps 1 to 4, measured by programs as well as tests

**What.** The phase decision adds the rewrite (FR) as decision 0062 directs, names the 32 marked tests as the tests it takes over, and starts from `DESIGN-rewrite.md`. For a programmer, the measure matters as much as the order. These programs and tests should measure it:
- **The marks file.** The takeover list starts with the 32 marked tests. `DESIGN-rewrite.md` section 5 needs the two rows that round 26 added, so step 5 takes 17 tests and step 6 takes 11.
- **A parse count over programs.** Step 2's `next: parse` line counts the 152 program tests apart, and also the roadmap sketches of rounds 8 to 13 and the port programs as a corpus outside `tests/`. A program is long, mixes chapters, and meets the parser's oracle at every template-id. This round's TR-12 false reject on a fold in a requires clause, and the 129 rejects that pass on TR-12, belong to this step.
- **A program line in the shadow run.** From step 3, `next: shadow` prints the program tests apart: how many run to their result on `tidal-next`, and how many of the old front end's runs it matches. `DESIGN-rewrite.md` gives the program tests to the new front end only in step 8. The shadow line shows the distance earlier.
- **The library faults of rounds 12 and 13.** Each reduction is a test after item 1, so the shadow line counts them by page. The rewrite passes a reduction when the page module of its rule passes it, and a round report names the reductions that the new code passes first.
- **The first-error rate.** The new code keeps or raises the old rate of 77% of passing rejects whose first error names a cited or linked rule (`DESIGN-rewrite.md` section 10), and it reports the rate for program tests with planted mistakes apart.

**Why.** The gate holds (report 26 section 8). Decision 0062 then adds the rewrite. `DESIGN-rewrite.md` measures each step by tests of one chapter or one model section, and a program uses many at once. My five programs met 10 faults on eight pages, and the 7015 tests held none of them.

**Size.** Large, over several phases. Steps 1 and 2 fit in phase 16 with the tool lines they need.

**Inputs.** Decision 0060, `DESIGN-rewrite.md`, the marks file, item 1.

### Item 4. Answer the 16 open rows, starting with the rows a programmer meets

**What.** For a programmer, these come first:
- P4-3: an example of an adaptor over any range. `algo.cpp`'s `unique` over a linked range and round 11's `views` are inputs.
- P4-4: an example that names a member call's result type with no library.
- P4-5: the move constructor of the `Maybe` example, and CPY-5 beside CPY-8.
- P4-7: the value that the numeric literal form receives (OPR-8).

Then X25-1 to X25-7 and X25-9. P3-9, P3-10, P4-1, and P4-2 go with item 2.

**Why.** Section 1.7. P4 and this round met the same library forms from two personas.

**Size.** Medium.

**Inputs.** The P4 and X25 reports and their programs.

### Item 5. A gap review of the 20 rules and principles changed after RV8 read the tree, with a read of its repair diff

**What.** A source-reader review of DTOR-8, OPR-P3, OPR-8, OPR-16, OPR-10, UNI-P3, FN-12, REF-2, TYN-10, TYN-6, ADD-11, ADD-14, STG-16, STG-28, CTAD-18, DEP-P1, DEP-40, INST-1, INST-2, and INST-5, a recorded read of its repair diff, then repairs. The doubts of section 6 join it.

**Why.** The close commit names these rules for the next review. INST-1 and INST-2 decide this round's `decltype` reductions, and FN-12 decides P4-1. A programmer ranks the review below items 1 to 4, because the other findings of this report rest on rules that did not change.

**Size.** One review session, then repairs.

**Inputs.** None.

### Item 6. Reject tests that pass on a grammar error

**What.** A writer reads the reject tests whose first front-end error is TR-12 while the test cites no TR rule (`grammar-first-errors.txt`), and marks the ones whose construct is valid C++ apart from the rule the test checks. `fold-atomic.cpp` is one such test. Its fold is valid, and the test checks that two constraints do not subsume each other. A companion accept test then keeps the fault visible (item 1).

**Why.** Section 1.5. The harness counts a reject that the front end gives for another reason as a pass. Of the 129 hits, 84 stop at "expected a declarator", which is often the right reading of a test about names. I read only the 15 hits that stop at an expression or a declaration, and one of them is a valid construct.

**Size.** Small.

**Inputs.** `grammar_first_errors.py`.

### Item 7. Program first tries and their mistakes in the completion count

**What.** A phase close also gives how many clean tries of the roadmap and port programs run on the front end. In this round, 1 of 2 clean first tries and 8 of 8 clean intermediate tries meet a front-end fault, and 3 of my 5 mistakes sat behind a fault.

**Why.** Section 1.6. Decision 0060's line counts tests only. Five library programs in one session found 10 faults that the 7015 tests do not hold. Round 12 found 7 and round 11 found 11 the same way. This is round 12 process change 3, which decision 0063 deferred.

**Size.** Small.

**Inputs.** `results-new.txt`, `summarize.py`.

### Item 8. Port suite P5 on generic code and customization points

**What.** A port suite with a persona from a language with generic interfaces and type-level values, such as Haskell or Rust. Ports: a unit-checked quantity, a generic algorithm that a user type customizes, and a container that grants access to its node type.

**Why.** Eight of this round's 10 faults come from those forms: constraints, class-type constant arguments, `decltype` over generic calls, conversion templates, lookup of a customization, and friend templates.

**Size.** One session.

**Inputs.** Item 2, so that P5 measures the repaired front end.

### Item 9. Guides on constrained special members, customization points, and class-type constant parameters

**What.** A maintainer decision on publishing guides, then how-to guides on a copy constructor that only some specializations have, a customization point with a fallback, and a quantity type with a class-type dimension.

**Why.** My `ring` mistake of round 12 and this round's `algo` show that these forms take more than one try even on GCC and Clang. The rules sit on the copy-move, constraints, names, and arguments pages.

**Size.** One decision, then one session per guide.

**Inputs.** The maintainer.

## 3. Out of scope

| Topic | Proposal | For | Against |
|---|---|---|---|
| The general library, including `construct_at`, `strong_ordering`, `initializer_list`, and `size_t` | Keep out. Programs declare their own, as the five new programs do. | Each program needs them (general knowledge). Two of my five mistakes came from library types that the program declares itself. | The charter leaves out the library. LSUP-1, LSUP-2, LSUP-10, and LSUP-12 give the shapes. |
| Rewrite steps 5 to 9 in phase 16 | Keep for later phases. | The 32 marked tests wait for them. | Steps 1 to 4 come first in `DESIGN-rewrite.md`, and item 2 keeps the old front end useful for programs until step 8. |
| A freeze of the old front end while the rewrite runs | Do not freeze it before step 8. | `DESIGN-rewrite.md` section 10 says a fix to an owned test in the old code changes no result. | No program test is owned before step 8, and this round's 10 faults sit in tests the old front end keeps. |
| `typeid` | Keep deferred. | | Each test needs the library's `type_info` (Q-0322). |
| Threads, the memory model, coroutines, modules, extended floating-point types | Keep out. | P4-3's lazy stages would use a generator (general knowledge). | as the charter says |
| Language versions after C++23 | Keep out until `pending.md` row 7 is decided. | | Strict mode rejects them. |

## 4. Process changes

1. **A roadmap round adds its own reductions as tests in the phase that follows it**, as T2 to T4 did. Item 1 keeps that practice.
2. **The rewrite reports programs from its first steps.** The `next:` lines of `DESIGN-rewrite.md` gain a parse count and a shadow count for program tests, so a phase close shows how far the new code is from the programs that the old code runs (item 3).
3. **The phase close gives a count of library programs beside the completion line** (item 7). Decisions 0062 and 0063 did not take this change from rounds 11 and 12.
4. **A reject test whose first error is a grammar error gets a read** when the test cites no grammar rule (item 6).

## 5. Suggested order

1. Item 1's tests come first. They are the first commit of items 2 and 3.
2. Item 2 and item 3 run side by side. Item 2 takes the wrong verdict and the missed rejects first. Item 3 takes steps 1 and 2 with the tool lines that count programs.
3. Item 6 runs beside item 3's step 2, because the parse count meets the same tests.
4. Item 4 runs beside items 2 and 3. Item 5 runs after item 1, because item 1 adds tests under INST-1 and INST-2.
5. Item 7 runs at the phase close. Item 8 follows item 2. Item 9 waits on the maintainer.

## 6. Doubts for the clean-room writers

- Whether a copy constructor with a trailing requires clause that fails still removes the implicit copy constructor (CPY-1, CPY-4). Every compiler says yes. CPY-1 says only that a copy constructor is not a template, and a member of a class template is not one.
- Whether INST-2's exception for the whole operand of `decltype` lets a qualifier after the `decltype` instantiate the class. INST-1 lists a qualified name among the uses that instantiate, and every compiler accepts `decltype(div(...))::dim`.
- Whether a conversion function template with a failing or no constraint can add a built-in candidate for a binary operator (OVL-9, DED-31). My reading is no for an `explicit` one, because OVL-9 needs an implicit conversion except for `!`, `&&`, and `||`.
- Whether the front end's message for a range `for` over a braced list should name LSUP-1, as it does for `auto x = {1, 2}`. The compilers reject both forms.

## 7. Host-only claims

Decision 0061 asks a reader report to list these, so that a writer probes them on the matrix first:
- Every reduction and probe result for GCC and Clang comes from host GCC 16.2.1 and Clang 22.1.8. EDG 7.0 ran in its container. GCC 14, GCC 16.2.0, and Clang 18 did not run.
- The 9 valid reductions and the clean tries of section 1.4 build and exit 0 on the host GCC and Clang, and the 2 reject reductions fail to build on both.
- The sanitizer results of section 1.6 come from the host compilers. UBSan of the host Clang does not report the bad downcast, and no host sanitizer reports the inactive union read.
- Clang's two warnings on `aggregate_with_base.cpp` come from the host Clang with `-Wall -Wextra`.
