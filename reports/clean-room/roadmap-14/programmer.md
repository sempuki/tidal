# Roadmap, round 14: what to do after phase 16, from a programmer's view

Date: 2026-11-11. Decision 0029. This report comes from a clean-room reader in the role of a C++ programmer who uses the specification to write and check programs.

**What the reader read.**
- AGENTS.md, CLAUDE.md, and the round 13 programmer report (`reports/clean-room/roadmap-13/programmer.md`) with its sketches and logs.
- The status page, the roadmap's phase 16 table, decisions 0060, 0063, and 0064, and `documentation/decisions/pending.md`.
- `git log b73a7d2d..HEAD` with every commit body (17 commits: the phase 15 corrections, decision 0064, the items T5, F6, RW, FE, RV9, R10, and E, three ledger commits, the close, and its correction).
- The X24 report, front-end reports `report-rw1.md` and `report-27.md`, the RV9 review and the read of its repair diff in `reports/review/`, `frontend/DESIGN-rewrite.md`, `frontend/TEMPLATE-MODEL.md`, `rewrite-marks.tsv`, `reports/clean-room/ledger.md`, and `rust-frontend/ledger.md`.
- The rules and tests that the findings name: LAM-1, LAM-5, LAM-18, CTOR-10, ICS-9, LIST-3, LIST-4, LIST-12, CPT-24, CPT-26, IF-13, IF-22, INST-5, CLS-2, CE-16, TPL-11, NAM-21, DEP-19, the Syntax segments of the lambdas and casts pages, and the tests `constant-parameter-class-template.cpp`, `literal-operator-template-class-parameter.cpp`, `member-call-before-definition.cpp`, and `typename-needed-block.cpp`.

The reader did not read the other round 14 reports, `frontend/` sources, `inputs/`, or `build/exercise-keys/`. Commit read: `f77bbe05`, the correction to the phase 16 close.

**What the reader did.** The reader built both front ends from `f77bbe05` into its scratchpad with cargo, with no build failure: `tidal-fe` from `frontend/` and `tidal-next` from `frontend/next/`. The reader reran the 94 round 13 files and wrote five new programs (1053 lines) with 14 earlier tries, 21 short programs, 8 planted bugs, 9 planted mistakes, 13 reductions, and 28 probes. Each file went through these checks:
- host GCC 16.2.1 and Clang 22.1.8 with `-std=c++23 -pedantic-errors -Wall -Wextra`,
- the same with `-std=c++23 -pedantic-errors` alone,
- the same with `-fsanitize=undefined,address`,
- the EDG 7.0 container (`cpfe --c++23 --strict --no_code_gen --instantiate=used`),
- `tidal-fe run`,
- `tidal-next parse`, the rewrite's parser, which prints a tree or the line where the grammar stops.

Everything is in `programmer-sketches/` beside this report:
- `round13/` holds the copies that were rerun, and `results-rerun.txt` is their log (`rerun13.sh`). `compare_rerun.py` compares the log with round 13's log (`compare-rerun.txt`).
- `new/` holds the new programs, their earlier tries, `one-line/`, `bugs/`, and `mistakes/`. `reductions/` and `probes/` hold the reductions and probes. `results-new.txt` is their log (`runnew.sh`).
- `runall.sh` runs one program on every tool. `reduce.py` is the line-based reducer of round 13, pointed at this round's build.
- `counts.py` gives the counts of section 1.1 from `git show` (`counts.txt`). `ledger_counts.py` counts both ledgers by state (`ledger-counts.txt`). `scan_tests.py` scans `tests/` for the forms of the reductions (`scan.txt`), and `scan_grammar.py` for the two grammar forms (`scan-grammar.txt`). `summarize.py` counts the front-end results of a log (`summary-new.txt`, `summary-rerun.txt`). `parse_stops.py` lists the parser's stops with their source lines (`parse-stops-new.txt`, `parse-stops-rerun.txt`). `parse_programs.py` runs `tidal-next parse` on the program tests and on the roadmap programs of rounds 8 to 14 (`parse-programs.txt`).

Statements that rest on general knowledge say so. Section 7 lists the claims that rest on the host compilers alone.

## 1. The state from a programmer's view

Front-end round 27 fixed all 10 faults of the round 13 reductions. Every round 13 file now gets the compilers' verdict from `tidal-fe`, except the planted pure virtual call, which still stops. Every program test runs to its result.

The new programs are again library code: an indexed d-ary heap with a stored comparator and a projection, compile-time string keys as constant template parameters, a non-owning callable reference, field-wise operations on plain aggregates found by braced initialization, and a table-driven state machine over type lists.
- Two first tries (`keys_first_try.cpp`, `fields_first_try.cpp`) build clean and exit 0 on GCC and Clang, and EDG accepts them. The front end stops on the first and gives a false reject on the second.
- After workarounds, all five run on the front end. `heap` needed no workaround, `keys` and `fsm` one each, and `fnref` and `fields` two each.
- Five intermediate tries build clean and exit 0 on GCC and Clang. The front end runs one of them and stops on four.

The 13 reductions show 9 faults:
- 5 in `tidal-fe`: 1 that gives a false reject in one form and a stop in another, 3 stops on valid programs, and 1 stop that also hides a required rejection. Three are new in kind. One falls under a cause that the marks file already names, and one has four unsupported tests.
- 4 in `tidal-next parse`: two grammar forms that no Syntax segment derives and no ledger row names, one oracle point with no row, and G173.

The front end runs every short program and catches planted errors well. It runs 21 of 21 short programs, stops 8 of 8 planted bugs with `undefined` and a rule, and rejects 9 of 9 planted mistakes. Its first error names the rule of the cause in 8 of the 9.

The rewrite's parser is further from programs than from tests. It gives a tree for 128 of 166 program tests (77%), where the parse line gives 6475 of 6913 tests (94%). It gives a tree for 17 of the 30 last versions of the roadmap programs of rounds 8 to 14.

### 1.1 Counts

| Measure | Round 13 (`a6a2b16c`) | Now (`f77bbe05`) | Source |
|---|---|---|---|
| Rule headers | 1700 (1685 in force, 15 withdrawn) | 1700 (1685 in force, 15 withdrawn) | `counts.py` |
| Example blocks | 238: 78 unmarked, 29 `fragment`, 114 `reject`, 17 `diverges` | 240: 79 unmarked, 30 `fragment`, 114 `reject`, 17 `diverges` | `counts.py` |
| Tests | 7015 | 7073 | `counts.py`, files with `tidal-rules`, `.tu.cpp` left out |
| Program tests | 152 | 166 (14 `r13-` added by T5) | `counts.py` |
| Lines under `tests/programs/` | 20156 | 23005 | `counts.py` |
| Rules cited by a program test | 577 of 1685 | 587 of 1685 | `counts.py` |
| Tests that open `namespace std` | 293, 82 of them program tests | 307, 96 of them program tests | `counts.py` |
| Front end | 6895 of 7015 pass, 6926 their check | 6967 of 7073 pass, 6990 their check | close commits `a6a2b16c`, `58060b8f` |
| Completion line of decision 0060 | 80 tests fail their check (32 marked, 48 toward the gate), 5 of 152 program tests do not run, 9 out of scope | 74 tests fail their check (36 marked, 38 toward the gate), 0 of 166 program tests do not run, 9 out of scope | close commits |
| `conformance.tsv` | 7007 rows | 7073 rows: 6616 pass, 305 EDG-ONLY, 30 pass-partial, 10 EDG-ALLOWED, 6 NO-DIAGNOSTIC-ALLOWED, 38 MISSED-REJECT, 32 unsupported, 23 run-unsupported, 7 FALSE-REJECT, 6 EDG-AGREES | the file |
| Passing rejects whose first error names a cited or linked rule | 2139 of 2766 (report 26) | 2154 of 2782, 77% (report 27) | front-end reports |
| Changes under `frontend/` | round 26, GC, and RD: 17 files, 1160 lines added, 77 removed | round 27 and RW: 230 files, 11001 lines added, 175 removed; `frontend/src/` 16 files, 492 added, 25 removed; `frontend/next/` 210 files, 10224 added | `git diff --stat a6a2b16c f77bbe05 -- frontend/` |
| Rewrite lines | none | `next: unit` 88, `next: lex` 7191/7191, `next: parse` 6475/6913, `next: owned` 0 | close commit `58060b8f` |
| Program tests that parse to a tree | none measured | 128 of 166 | `parse_programs.py` |
| Port suites done | 4 of 8 (P1 to P4) | 4 of 8 (P1 to P4) | `exercises.md` section 8 |
| Questions | 446: 197 open | 449: 200 open, 236 settled, 13 compiler defects | `status.md` |
| Front-end ledger | 165 rows, 0 open | 180 rows: 157 commit, 4 text, 4 out of scope, 1 question, 14 open (G166 to G179) | `ledger_counts.py` |
| Exercise ledger | 1097 rows, 16 open | 1129 rows: 1030 commit, 48 text, 33 question, 3 out of scope, 15 open | `ledger_counts.py` |

### 1.2 Round 13 items that phase 16 took and left

| Round 13 item | Phase 16 | Result |
|---|---|---|
| 1. Tests for the reductions, probes, and clean tries | item T5 | Done: 20 rule tests and 14 program tests. Report 27 passes every one. |
| 2. Front-end round 27 on the current design, library targets first | item FE | Done for every target. 0 of 166 program tests do not run, and the 23 run stops are older. |
| 3. The rewrite, steps 1 to 4, measured by programs as well as tests | item RW | Steps 0 to 2 are done. The parse line counts all tests together and has no line for program tests. Step 3 and its shadow line wait. |
| 4. Answer the 16 open rows | items F6 and FE | Done. |
| 5. A gap review of the 20 changed rules | item RV9 | Done, with a read of its repair diff (RV10-R-1 to RV10-R-8). |
| 6. Reject tests that pass on a grammar error | none | Deferred by decision 0064. `fold-atomic.cpp` now rejects with OVL-36, its own cause. |
| 7. Program first tries and their mistakes in the completion count | none | Deferred again. |
| 8. Port suite P5 on generic code and customization points | none | Deferred. |
| 9. Guides | none | Deferred. |

Round 13 process change 1 is in place: T5 added the reductions as tests in the phase that followed. Change 2 is not: the `next:` lines give no count for program tests. Decision 0064 deferred changes 3 and 4.

### 1.3 The round 13 sketches, rerun

`compare_rerun.py` reads the 94 files of `results-rerun.txt`. 60 give the same front-end result as in round 13, and 34 changed. The compilers and EDG give the same result on every file as in round 13, apart from the exit codes of planted bugs.

Of the 34 changes:
- 11 are the round 13 reductions. The 9 valid ones run to 0, and the 2 reject reductions are rejected with CTOR-25, as GCC, Clang, and EDG reject them.
- 10 are probes. 6 now run to 0, and 4 are now rejected as the compilers reject them (INIT-7, LSUP-1 twice, CTOR-25).
- 9 are clean tries that now run to 0: `algo_third_try.cpp`, `bits_first_try.cpp`, `intrusive_second_try.cpp`, and `units_second_try.cpp` to `units_seventh_try.cpp`.
- 4 are mistakes that now give the compilers' first error: `algo_second_try.cpp` (DED-35), `intrusive_first_try.cpp` (LSUP-1), `units_first_try.cpp` (DCMP-11, as GCC), and `mistakes/no_default_constructor.cpp` (CTOR-25).

The one round 13 file without the compilers' verdict is `bugs/pure_virtual_from_constructor.cpp`. It still stops with "function 'run' has no definition in the program" (VIRT-12, among the 23 run stops).

`tidal-next parse` gives a tree for 74 of the 94 files and stops on 20:
- G168 (`using namespace literals;`) in the 7 versions of `units` and one probe,
- G166 (`NodeIter<int>{n}`) in 4 versions of `algo` and one probe,
- G167 (`struct Person : Hook<ByAge>, Hook<ByName>`) in 3 versions of `intrusive` and one probe,
- G169 in `one-line/using_enum.cpp`,
- a lambda attribute with no template head in `one-line/lambda_attribute.cpp`, which no row names (section 1.5),
- my mistake in `algo_first_try.cpp`, where every compiler stops too.

### 1.4 New programs

| Program | GCC, Clang (three flag sets) | EDG 7.0 | `tidal-fe run`, first clean try | `tidal-fe run`, last version | `tidal-next parse`, last version |
|---|---|---|---|---|---|
| `heap.cpp` (249 lines): `IndexedHeap<P, N, D, Cmp, Proj>` with the comparator and projection as `[[no_unique_address]]` members, decrease-key through a position table, a heap sort constrained by a concept and called with a captureless lambda, Dijkstra in constant evaluation | first and third fail (my mistakes); second and last exit 0, clean | accepts the clean versions | the second try runs to 0 | result 0, no workaround | tree |
| `keys.cpp` (181 lines): `FixedString<N>` as the type of a constant parameter made from a string literal, `Key<S>` with an FNV-1a hash, a literal operator template `_k`, a sorted `Table` with a deduction guide, `Record<Field<"x">, ...>` with fields looked up by key | first try exits 0, clean | accepts | unsupported: "call needing template deduction or unknown arguments" | result 0, after one workaround | stops on G166 (`keys::Key<"pear">{}`) |
| `fnref.cpp` (212 lines): `FunctionRef<R(A...)>` with an object pointer and a thunk from a function template specialization, an `invoke` for pointers to members through a generic lambda, a concept `invocable_r`, text callbacks | first to fourth fail (my mistakes); fifth, sixth, and last exit 0, clean | accepts the clean versions | fifth: unsupported, "call needing template deduction or unknown arguments"; sixth: unsupported, "converting constructor" | result 0, after two workarounds | tree |
| `fields.cpp` (220 lines): the field count of an aggregate by braced initialization with a type that converts to anything, fields reached by structured bindings, equality, a hash, and an encoder and decoder that round-trip | first try exits 0, clean | accepts | rejected, SB-2 at line 51. **False reject.** The second try gives "call needing template deduction or unknown arguments". | result 0, after two workarounds | stops on G173 (`template <class T> operator T() const;`) |
| `fsm.cpp` (191 lines): states and events as type lists, transitions found by overload resolution and tested by a concept, a table of next states made by pack expansion, `overloaded` lambdas with aggregate deduction, dispatch through an array of function pointers made from lambdas | first fails (my mistake); second and last exit 0, clean | accepts the clean versions | second: unsupported, "body of 'make_table' is not available yet" | result 0, after one workaround | stops on G167 (`struct MakeSeq : MakeSeq<N - 1, N - 1, I...>`) |

The workarounds:
- `keys`: `FixedString` is a plain class with a capacity of 15 characters, in place of a class template whose length the string literal gives. The literal operator template then works with no other change.
- `fnref`: `invoke`'s generic lambda captures the pointer to member by copy, in place of `[&]`. Then each closure passed to a `FunctionRef` parameter is converted to `FunctionRef` by name first.
- `fields`: the field counts of `Pixel` and `Sample`, whose last member is an aggregate, are explicit specializations of `fields::count`. Then the encoder and decoder capture the buffer through a pointer, in place of `[&]`.
- `fsm`: the table is a `static constexpr` member function, in place of a `static constexpr` data member initialized by a call of `make_table()`.

Each first try and each intermediate version is kept beside the last version (`*_first_try.cpp` to `*_sixth_try.cpp`).

### 1.5 Reductions

Twelve reductions build and exit 0 on GCC and Clang under the three flag sets, and EDG accepts them. One is a reject reduction: GCC, Clang, and EDG reject `static_constexpr_member_from_member_function_plain_class.cpp`. `scan_tests.py` scans `tests/` for the forms (`scan.txt`), and I read each hit:
- Four tests use a class template as the type of a constant parameter. All four are `unsupported` in `conformance.tsv` and count toward the gate.
- One test has a generic lambda with a reference capture (`r10-hash-map.cpp`). It captures no class object that the body calls or passes to a template.
- Two tests have a constructor template from a forwarding reference. Neither passes a closure to a parameter of the class.
- One test has a requires expression with a braced list of a type (`W7/date.cpp`). Its class has no aggregate member.
- No test initializes a `static constexpr` data member with a call of a member function of its own class. `member-call-before-definition.cpp` holds the call before the definition outside the class, and it passes.
- `scan_grammar.py` finds no test with a lambda attribute and no template head, and no test with a typename specifier as the type of a functional cast.

**Faults in `tidal-fe`.**

| Reduction | Lines | `tidal-fe` | Status | What the specification says |
|---|---|---|---|---|
| `generic_lambda_reference_capture_object_call.cpp`: `Dbl f; auto g = [&](auto x) { return f(x); };` | 7 | unsupported: "call needing template deduction or unknown arguments" | New. A copy capture of the object runs, and so do a reference capture of an `int` or a function pointer (probes). | LAM-5: `[&]` captures the object by reference. LAM-18: the capture is visible in the body, and the call has a class operand of known type. |
| `generic_lambda_reference_capture_member_pointer.cpp`: `[&](auto&& o) { return (o.*f)(); }` with `f` a pointer to member function | 7 | the same | Same cause. A copy capture runs (probe). | LAM-5, LAM-18 |
| `generic_lambda_reference_capture_template_argument.cpp`: `[&](auto v) { put(v, b); }` with `put` a template and `b` a class object | 8 | the same | Same cause. A plain function `put` runs, and so does `b.n += v` (probes). | LAM-5, LAM-18, and deduction from the argument `b` |
| `closure_argument_converting_constructor_template.cpp`: `int use(Ref)` with `template <class F> Ref(F)`, then `use([] {})` | 8 | unsupported: "converting constructor" | New. An argument of a class type runs, and so do `Ref r = f;` and `Ref r(f);` for a closure `f` (probes). | CTOR-10: the constructor template converts. ICS-9: one user-defined conversion initializes the parameter. |
| `requires_too_many_braced_initializers_nested_aggregate.cpp`: `struct S { short x; C c; };` with `C` an aggregate, then `!requires { T{1, 2, 3}; }` for `S` | 4 | unsupported: "check: static_assert: concept" | New. The same with two values, or with a flat `S`, runs. An array member gives the same stop (probes). | LIST-4 and LIST-12: the third value has no member, so the list is not valid. CPT-26: the requirement does not hold. |
| `if_constexpr_braced_initializers_nested_aggregate.cpp`: the same requirement as the condition of `if constexpr`, with a structured binding of three names in the branch it discards | 12 | rejected, SB-2. **False reject.** | Same cause. The branch that the compilers discard is checked. | IF-13: the discarded branch of a template is not instantiated. |
| `static_constexpr_member_from_member_function.cpp`: `static constexpr int value = make();` in a class template, with `make` a static member function defined in the class | 5 | unsupported: "body of 'make' is not available yet" | The cause of the marked tests `class-point-static-constexpr-made-at-use.cpp` and `class-static-constexpr-member-initializer-waits.cpp` (model section 3). No test holds this form. | INST-5: the initializer of a `static constexpr` member is not made with the class. At its first use the class is complete. |
| `static_constexpr_member_from_member_function_plain_class.cpp` (reject): the same in a class that is not a template | 5 | the same stop. GCC, Clang, and EDG reject it. | New. No test holds the form. | CLS-2 does not list a static data member's initializer, so the class is not complete there. CE-16 rejects the call before the definition. |
| `class_template_constant_parameter_from_string_literal.cpp`: `template <Str S> struct Key` with `Str` a class template, then `Key<"b">::first` | 8 | unsupported: "dependent name" | Known. `constant-parameter-class-template.cpp` and three other tests are unsupported for the same cause. A plain class with a fixed capacity runs (probe). | TPL-11, CTAD-1: the argument gives the class template's arguments. |

So the 9 `tidal-fe` reductions show 5 faults. Three are new in kind: the reference capture in a generic lambda, the closure passed to a converting constructor template, and the list with too many values beside a nested aggregate. The `static constexpr` member falls under a marked cause, and its form outside a template is a required rejection that the front end does not give. The class template as a constant parameter type has four tests.

**Faults in `tidal-next parse`.**

| Reduction | Lines | `tidal-next parse` | `tidal-fe` | What the specification says |
|---|---|---|---|---|
| `lambda_attribute_without_template_head.cpp`: `[] [[nodiscard]] (int x) { ... }` | 4 | stop: "expected a lambda specifier, found '['" | result 0 | The `lambda_expr` production takes `attribute_spec` only inside `lambda_template`. No ledger row names this form. |
| `typename_specifier_braced_cast.cpp`: `typename Box<T>::type{3} + typename Box<T>::type(4)` | 4 | stop: "expected a type name, found 'typename'" | result 0 | `cast_type` on the casts page has no typename specifier. DEP-19 requires `typename` in this place, so a program cannot follow both. G166 names only the template-id. |
| `qualified_specialization_member_type_from_namespace.cpp`: `template <> struct lib::count<Range> { static constexpr size value = 2; };` with `size` declared in `lib` | 5 | stop: "expected 'override' or 'final', found 'value'" | result 0 | NAM-21: the body of a class defined with a qualified name sees the members of the qualifier. The step 2 oracle does not. `programs/roadmap/r5-prelude.cpp` stops on this form. The same specialization inside `namespace lib` parses (probe). |
| `member_template_conversion_declared_only.cpp`: `struct any { template <class T> operator T() const; };` | 6 | stop: "expected 'const' or 'volatile', found ';'" | result 0 | G173. |

### 1.6 Short programs, planted bugs, and planted mistakes

**Short programs.** The 21 short programs in `new/one-line/` build and run clean on GCC and Clang with every flag set. GCC warns once with `-Wall -Wextra` on `static_subscript.cpp`, that the variable `s` is set but not used, where the program calls its static `operator[]`. EDG accepts all 21. The front end runs all 21 (20 "result: 0" and 1 "result: 0 in every explored order"). The forms are:
- `if consteval`, `auto(x)` and `auto{x}`, a subscript operator with two arguments, a static `operator[]`, and a builder whose explicit object parameter returns the object by forwarding,
- `[[assume]]`, the `uz` suffix, a `consteval` function with a default argument, a `constexpr` virtual call, and `dynamic_cast` in constant evaluation,
- a change of the active union member in constant evaluation, aggregate initialization with parentheses, a lambda in a default template argument as a unique type, and deduction through an alias template,
- a defaulted `==` over an array member, a `u8` literal of type `char8_t`, `namespace lib::inline v2`, a `try` block in a `constexpr` function, a capture of a structured binding, `explicit(bool)`, and a generic lambda that calls an object captured by copy.

`tidal-next parse` gives a tree for all 21.

**Planted run-time bugs.** The front end stops all 8 bugs in `new/bugs/` with `undefined` and a rule:

| Rule the front end names | Bug | Sanitizers |
|---|---|---|
| OBJ-2 | a reference returned from a function to one of two temporary arguments | ASan; GCC also warns |
| OBJ-2, INIT-1 | a sum that starts from a local with no value | silent; Clang warns |
| ADD-10 | a pointer two past the end of an array | UBSan of Clang only |
| PFX-21, UN-7 | a list walk with no end test calls a member function through a null pointer | UBSan; the plain builds crash |
| CAST-5, CV-5 | a write to a string literal through `const_cast` | ASan; the plain builds crash |
| MUL-10 | the lowest `int` divided by -1 | UBSan; the plain builds trap |
| OBJ-2 | a read through a second pointer after `delete` | ASan |
| OBJ-2 | a callable reference to a pointer to member that ends with its full expression | ASan |

The read of a local with no value passes every sanitizer, and only the front end names it.

**Planted compile-time mistakes.** GCC, Clang, and EDG reject all 9 mistakes in `new/mistakes/`. The front end rejects all 9:
- Its first error names the rule of the cause in 8: CPY-21 (a copy of a move-only class), CX-1, ACC-2, VIRT-3, VIRT-8, CV-4, CEV-1, and RET-7.
- For `C::value_type x = 0;` in a template body, its first error is TR-12, and its second is DEP-19, the rule of the cause. `typename-needed-block.cpp` gives the same pair and passes.

The 9 planted mistakes of round 13 rerun with the same first errors, except `no_default_constructor.cpp`, which now names CTOR-25.

**My own mistakes from memory.** Four programs held seven:
- `heap_first_try.cpp` and `fnref_first_try.cpp`: a concept used in a compound requirement before its declaration. The front end names CPT-34, where Clang and EDG say that a concept name is expected.
- `heap_third_try.cpp`: an unconstrained `less::operator()` satisfies my ordering concept for every type. The front end names SA-1, as every compiler does.
- `fnref_second_try.cpp`: my `invoke` tests `(o.*f)` alone, which is not valid for a pointer to member function, and falls through to `*o`. The front end names UN-6 at the compilers' line.
- `fnref_third_try.cpp`: the thunk casts from `void*` in constant evaluation. The front end names CE-4, as every compiler rejects it.
- `fnref_fourth_try.cpp`: `static_cast<Word>(int)` is valid through aggregate initialization with parentheses, so my negative test failed. The front end names SA-1.
- `fsm_first_try.cpp`: a fold inside a lambda inside another fold, where the inner fold expands both packs. The front end stops first on the `static constexpr` member. With that worked around, it names PACK-6 at the compilers' line (probe).

One of my seven mistakes sat behind a front-end fault. Round 13 had three of five.

### 1.7 The phase 16 reports, from a programmer's view

**Front-end report 27.** The round did every library target that round 13 asked for, and program tests no longer stop. The library forms of this round are new: they come from callable wrappers, aggregate reflection, and tables in class templates. Two of the five faults sit in lambdas, which the round 13 programs used little.

**`report-rw1.md` and the parse line.** The parse line counts tests of all kinds together. `parse_programs.py` gives a tree for 128 of 166 program tests. The 38 stops fall on these causes (`parse-programs.txt`):
- G167, a base written as a template-id: 14 tests, such as `P3/events.cpp`, `W9/mixin.cpp`, and the index sequences of `r8-tasks.cpp` and `r10-tuple.cpp`,
- G166, a template-id before a braced list: 9 tests,
- G168, `using namespace` with an unqualified name: 8 tests,
- G171, a template-id after an operator function name or a destructor name: 3 tests,
- G169, `using enum`: 2 tests,
- G174, a structured binding in an init statement: 1 test,
- the NAM-21 oracle point: 1 test (`r5-prelude.cpp`).

The 13 roadmap programs that stop show the same causes. A base class template and a braced temporary of a class template are among the first lines of almost every library program I write.

**`frontend/DESIGN-rewrite.md`.** Section 5 still lists 32 marked tests, and its build table gives 11 tests at step 6 and 4 at step 7. The marks file and `TEMPLATE-MODEL.md` section 6 have 36: 4, 6, 13, 6, and 7 by model section. So step 5 takes 17, step 6 takes 13, and step 7 takes 6. Round 13 found the same lag after round 26 added two marks.

**X24.** Labeled `break` and `continue` would remove the flag that my Dijkstra loop and round 12's `matrix` use (general knowledge). X24-2 matters more to a programmer today. Read literally, JMP-6 rejects `if (odd) continue; int y = i;` in a loop body, a form that I write in most loops and that both host compilers accept.

**The RV9 records.** RV10-R-2 says that FN-12's outermost enclosing class does not fit a nested class defined outside its class. RV10-R-5 says that CPT-28 reaches a variable template and an `if constexpr` condition, where the reference is silent. My `heap` and `fields` programs test such requirements in `if constexpr` and in concepts. Their parameter types are valid, so no verdict here rests on the point.

### 1.8 Ledger rows

The status page names 15 open exercise rows (X24-1 to X24-8, RV10-R-1 to RV10-R-3, and RV10-R-5 to RV10-R-8) and 14 open front-end rows (G166 to G179). `ledger_counts.py` gives the same rows. No row is `planned`.

## 2. Ranked work items

| # | Item | Evidence | Size | Inputs |
|---|---|---|---|---|
| 1 | Tests for this round's reductions, probes, and clean tries | 9 `tidal-fe` reductions, 4 `tidal-next` reductions, 9 probes that the front end fails, 2 clean first tries, 5 clean intermediate tries | small | `reductions/`, `probes/`, `new/` |
| 2 | Syntax segments that derive the forms that programs use | G166 to G179; 2 forms with no row; 38 of 166 program tests and 13 of 30 roadmap programs stop | medium | `report-rw1.md` section 10, `parse-programs.txt` |
| 3 | Front-end round 28 on the current code, with library-code targets first | 1 false reject, 3 stops on valid programs, 1 required rejection not given; 4 unsupported tests of TPL-11; the 38 tests toward the gate | large | item 1 |
| 4 | The rewrite, step 3, with a line for program tests and the design kept to the 36 marks | `DESIGN-rewrite.md` section 5 lists 32; no `next:` line for programs; the NAM-21 oracle point | large, over several phases | decision 0060, `DESIGN-rewrite.md`, the marks file |
| 5 | Answer the 15 open exercise rows, X24-2 first | X24-1 to X24-8, RV10-R-1 to RV10-R-3, RV10-R-5 to RV10-R-8 | medium | the X24 report, the RV9 read |
| 6 | A gap review of the 10 rules that changed after RV9 read the tree, with a read of its repair diff | close commit `58060b8f`: UNI-14, FN-12, CE-22, OBJ-1, OBJ-2, STG-6, CPT-28, INST-P4, INST-1, INST-30 | one review session, then repairs | none |
| 7 | Reject tests whose first error is a grammar error | my missing `typename` and `typename-needed-block.cpp` give TR-12 before DEP-19 | small | round 13 `grammar_first_errors.py` |
| 8 | Program first tries and their mistakes in the completion count | 6 of 7 clean tries meet a front-end fault; 1 of 7 mistakes hidden | small | `results-new.txt` |
| 9 | Port suite P5 on generic code and callable wrappers | 4 of this round's 5 `tidal-fe` faults come from generic lambdas, conversions, and templates in library code | one session | items 1 and 3 |

### Item 1. Tests for this round's reductions, probes, and clean tries

**What.** A clean-room writer adds a test for each of these, probes it on the matrix, and writes the expectation that the compilers and the text give:
- run tests that expect result 0: a generic lambda that calls a class object captured by reference, applies a pointer to member captured by reference, and passes a class object captured by reference to a function template (LAM-5, LAM-18); a closure passed to a parameter whose class has a converting constructor template (CTOR-10, ICS-9); a requires expression with too many values for an aggregate whose last member is an aggregate or an array, alone and as an `if constexpr` condition beside a discarded structured binding (LIST-4, LIST-12, CPT-26, IF-13); a `static constexpr` member of a class template initialized by a call of a member function of the class (INST-5),
- a reject test: the same `static constexpr` member in a class that is not a template (CLS-2, CE-16),
- an accept test that a class template as a constant parameter type takes a string literal (TPL-11, CTAD-1), beside the four unsupported tests,
- parse forms with no test: a lambda attribute with no template head, a typename specifier as the type of a functional cast with braces and with parentheses (DEP-19), and a member type of a namespace used in a specialization defined with a qualified name (NAM-21),
- program tests: `keys_first_try.cpp` and `fields_first_try.cpp` as clean first tries, the clean intermediate tries `heap_second_try.cpp`, `fnref_fifth_try.cpp`, `fnref_sixth_try.cpp`, `fields_second_try.cpp`, and `fsm_second_try.cpp`, and the last versions of all five as `r14-` program tests.

**Why.** Sections 1.4 and 1.5. No test holds any of the three new forms, and no test holds the two grammar forms. The front-end harness sees only tests.

**Size.** Small. Each reduction has at most 12 lines.

**Inputs.** `programmer-sketches/reductions/`, `probes/`, `new/`.

### Item 2. Syntax segments that derive the forms that programs use

**What.** A clean-room writer answers G166 to G179 in the text, each with the production that `report-rw1.md` section 10 suggests, and two more forms:
- `lambda_expr` takes `{attribute_spec}` after the captures when the lambda has no template head,
- `cast_type` takes a typename specifier, so that the form DEP-19 asks for has a derivation.

G167 and G166 come first, then G168, G171, G169, and G174, in the order of the program tests they stop.

**Why.** Section 1.7. The Syntax segments are the text a programmer reads to learn what to write. A base `B<int>`, a temporary `Box<int>{1}`, and `using namespace lib;` are in nearly every program, and today no production derives them. The parse line rises with each answer and needs no parser change beyond a function for the new production (`report-rw1.md` section 4).

**Size.** Medium. Fourteen rows and two new forms, each a production and a test.

**Inputs.** `report-rw1.md`, the front-end ledger, `parse-programs.txt`.

### Item 3. Front-end round 28 on the current code, with library-code targets first

**What.** Round 28 works on the old front end and takes the targets in this order:
1. The false reject. A requirement with too many values for an aggregate with a nested aggregate is false (LIST-12, CPT-26), so the branch of `if constexpr` that it discards is not checked (IF-13).
2. The required rejection. The call of a member function in the initializer of a `static constexpr` member of a class that is not a template is an error (CLS-2, CE-16).
3. The stops: the reference capture in a generic lambda, and the closure passed to a converting constructor template.
4. The class template as a constant parameter type (TPL-11), with its four tests, and the literal operator template over such a class.
5. The 38 tests that count toward the gate, and the 23 run stops, among them the pure virtual call of round 13.

The template form of the `static constexpr` member stays with the rewrite, because its cause is marked (decision 0064).

**Why.** Sections 1.4 and 1.5. The rewrite reaches program tests at step 8, so until then the old front end checks library code. The targets are what a library programmer writes first: a callable wrapper, a reflection helper over aggregates, a table in a class template, and string keys.

**Size.** Large.

**Inputs.** Item 1's tests.

### Item 4. The rewrite, step 3, with a line for program tests and the design kept to the 36 marks

**What.** The phase decision adds build step 3 of `DESIGN-rewrite.md`. For a programmer, three changes go with it:
- **The design kept to the marks file.** Section 5 and the build table list the 36 marked tests: step 5 takes 17, step 6 takes 13, and step 7 takes 6.
- **A line for program tests.** The parse line and the shadow line of step 3 give the program tests apart, as `parse_programs.py` does: 128 of 166 today. The roadmap and port programs outside `tests/` give a second count.
- **The NAM-21 oracle point.** The step 2 oracle looks up names in the namespace of a class defined with a qualified name. `DESIGN-rewrite.md` section 10 lists the point beside the member-template question.

**Why.** Section 1.7. The parse line gives 94% for tests and 77% for program tests. A program uses many pages at once, and the per-test count hides that distance. Round 13's process change 2 asked for this line, and the close does not give it.

**Size.** Large, over several phases.

**Inputs.** Decision 0060, `DESIGN-rewrite.md`, the marks file, item 2.

### Item 5. Answer the 15 open exercise rows, X24-2 first

**What.** For a programmer, these come first:
- X24-2: JMP-6 names the jumps it covers, so that `continue` before a later declaration in a loop body is valid, with the X24 probe as a test.
- X24-4 and X24-6: FN-7's loop condition and the lambda and local-class scope of JMP-7 and JMP-14.
- RV10-R-2 and RV10-R-5: FN-12 for a nested class defined outside its class, and CPT-28's reach.

Then the other X24 rows and the RV10-R rows.

**Why.** Section 1.7. X24-2 reads as a rejection of a form in most loops that compilers accept.

**Size.** Medium.

**Inputs.** The X24 report and the RV9 read.

### Item 6. A gap review of the 10 rules that changed after RV9 read the tree, with a read of its repair diff

**What.** A source-reader review of UNI-14, FN-12, CE-22, OBJ-1, OBJ-2, STG-6, CPT-28, INST-P4, INST-1, and INST-30, a recorded read of its repair diff, then repairs. The doubts of section 6 join it.

**Why.** The close commit names these rules for the next review. A programmer ranks the review below items 1 to 5, because this round's findings rest on rules that did not change, except INST-5 beside INST-1.

**Size.** One review session, then repairs.

**Inputs.** None.

### Item 7. Reject tests whose first error is a grammar error

**What.** A writer reads the reject tests whose first front-end error is TR-12 while the test cites another rule, and marks those whose second error names the cited rule. The front end then reports the cited rule first for those forms.

**Why.** Section 1.6. My missing `typename` gets TR-12 first and DEP-19 second, and `typename-needed-block.cpp` passes on the same pair. A programmer reads the first error. Decisions 0063 and 0064 deferred the item.

**Size.** Small.

**Inputs.** Round 13's `grammar_first_errors.py`.

### Item 8. Program first tries and their mistakes in the completion count

**What.** A phase close also gives how many clean tries of the roadmap and port programs run on the front end. In this round, 6 of 7 clean tries meet a front-end fault, and 1 of 7 mistakes sat behind a fault.

**Why.** Section 1.4. Every program test runs, and the five new programs still met five faults in one session. The completion line counts tests only.

**Size.** Small.

**Inputs.** `results-new.txt`, `summarize.py`.

### Item 9. Port suite P5 on generic code and callable wrappers

**What.** A port suite with a persona from a language with closures and generic interfaces, such as Rust or Swift. Ports: a callback API over closures, a reflection helper over plain records, and a table-driven state machine.

**Why.** Four of this round's five `tidal-fe` faults come from those forms: generic lambdas, conversions to a callable wrapper, aggregate initialization in a requirement, and string constant parameters.

**Size.** One session.

**Inputs.** Items 1 and 3, so that P5 measures the repaired front end.

## 3. Out of scope

| Topic | Proposal | For | Against |
|---|---|---|---|
| The general library, including `function_ref`, `index_sequence`, and `initializer_list` | Keep out. Programs declare their own, as the five new programs do. | Each program rebuilt a part of the library (general knowledge). | The charter leaves out the library. |
| Constant evaluation of a cast from `void*` | Keep to C++23. | A non-owning callable reference cannot run in constant evaluation without it (general knowledge). | Strict mode follows C++23, and CE-4 rejects it, as every compiler does. |
| Rewrite steps 4 to 9 in phase 17 | Keep for later phases. | The 36 marked tests wait for them. | Steps 2 and 3 come first, and item 3 keeps the old front end useful for programs until step 8. |
| Guides on callable wrappers, aggregate reflection, and constant parameters of class type | Keep deferred until the maintainer decides on guides. | This round's `fnref` and `fields` took several tries even on GCC and Clang. | No decision on guides exists. |
| `typeid` | Keep deferred. | | Each test needs the library's `type_info` (Q-0322). |
| Threads, the memory model, coroutines, modules, extended floating-point types | Keep out. | | as the charter says |
| Language versions after C++23 | Keep out until `pending.md` row 7 is decided. | | Strict mode rejects them. |

## 4. Process changes

1. **A roadmap round adds its own reductions as tests in the phase that follows it**, as T2 to T5 did. Item 1 keeps that practice.
2. **The rewrite's `next:` lines give program tests apart** (item 4). Round 13 asked for this, and the parse line still counts all tests together.
3. **A design document follows the marks file in the same commit** that adds a mark. Rounds 26 and 27 each added marks, and `DESIGN-rewrite.md` lagged both times.
4. **The phase close gives a count of clean library tries beside the completion line** (item 8). Decisions 0062 to 0064 did not take this change.

## 5. Suggested order

1. Item 1's tests come first. They are the first commit of items 2, 3, and 4.
2. Item 2 runs beside item 1, because each answer to a G row needs a test and a production.
3. Item 3 and item 4 run side by side. Item 3 takes the false reject and the required rejection first. Item 4 takes the design's counts and the program line before step 3's code.
4. Item 5 runs beside items 3 and 4. Item 6 runs after item 1.
5. Item 7 runs beside item 2, because both read the grammar's first errors. Item 8 runs at the phase close. Item 9 follows item 3.

## 6. Doubts for the clean-room writers

- Whether the text says that a call of a member function defined in its class, in the initializer of a `static constexpr` member of a class that is not a template, is a call before the definition (CE-16). CLS-2 does not list that initializer among the places where the class is complete, and every compiler rejects the program. No test holds it.
- Whether the lambdas page means to allow an attribute after the captures of a lambda with no template head. Every compiler accepts `[] [[nodiscard]] (int x)`, and no Syntax production derives it.
- Whether `cast_type` should take a typename specifier. DEP-19 requires `typename` before a dependent qualified type in an expression, and no production derives `typename T::type{3}`.
- Whether NAM-21 also covers an explicit specialization defined with a qualified name, such as `template <> struct lib::count<Range>`. Every compiler finds `lib::size` in its body.

## 7. Host-only claims

Decision 0061 asks a reader report to list these, so that a writer probes them on the matrix first:
- Every reduction and probe result for GCC and Clang comes from host GCC 16.2.1 and Clang 22.1.8. EDG 7.0 ran in its container. GCC 14, GCC 16.2.0, and Clang 18 did not run.
- The 12 valid reductions and the clean tries of section 1.4 build and exit 0 on the host GCC and Clang, and the reject reduction fails to build on both.
- The sanitizer results of section 1.6 come from the host compilers. Only the host Clang's UBSan reports the pointer two past the end, and no host sanitizer reports the read of a local with no value.
- GCC's warning on `static_subscript.cpp` and on `dangling_reference_return.cpp`, and Clang's warning on `uninitialized_read.cpp`, come from the host compilers with `-Wall -Wextra`.
- The X24-2 form (`continue` before a later declaration) rests on the X24 report's host probes. This round did not probe it again.
