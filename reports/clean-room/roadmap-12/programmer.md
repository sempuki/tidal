# Roadmap, round 12: what to do after phase 14, from a programmer's view

Date: 2026-11-11. Decision 0029. This report comes from a clean-room reader in the role of a C++ programmer who uses the specification to write and check programs.

**What the reader read.**
- AGENTS.md, CLAUDE.md, and the round 11 programmer report (`reports/clean-room/roadmap-11/programmer.md`) with its sketches and logs.
- The status page, the roadmap's phase 14 table, decisions 0060, 0061, and 0062, and `documentation/decisions/pending.md`.
- `git log 45ee5e85..bd87b27c` with every commit body (14 commits: decision 0062, the items of phase 14, two ledger commits, and the close).
- The I7 report, the P3 report, front-end report 25, the two RV7 gap reviews (`reports/review/2026-11-11-gap-review-phase-14-*.md`), `frontend/DESIGN-current.md`, `frontend/TEMPLATE-MODEL.md`, `reports/clean-room/ledger.md`, `rust-frontend/ledger.md`, and `conformance.tsv`.
- The rules that the findings name: STG-P2, STG-P3, STG-23, STG-28, STG-29, ADD-14, TYN-P2, TYN-4, TYN-7, ACC-5, ACC-20, OPR-8, OPR-10, OPR-16, PRIM-2, DED-28, DED-35, ILC-3, INST-1, RET-2, TPL-7, TPL-11, CPT-34, UNI-9, NAM-2, LSUP-2, LSUP-12, DCMP-1, and the tests `allocated-storage-typed-step-after-byte-offset.cpp`, `eq-array-of-arrays-member-constant.cpp`, `strong-ordering-missing-member.cpp`, and `strong-ordering-minimal.cpp`.

The reader did not read the other round 12 reports, `frontend/` sources, `inputs/`, or `build/exercise-keys/`. Commit read: `bd87b27c`, the close of phase 14.

**What the reader did.** The reader built the front end from `bd87b27c` into its scratchpad with cargo, with no build failure. The reader reran the 98 round 11 files and wrote five new programs (986 lines) with 12 earlier tries, 24 short programs, 8 planted bugs, 9 planted mistakes, 9 reductions, and 12 probes. Each file went through these checks:
- host GCC 16.2.1 and Clang 22.1.8 with `-std=c++23 -pedantic-errors -Wall -Wextra`,
- the same with `-std=c++23 -pedantic-errors` alone,
- the same with `-fsanitize=undefined,address`,
- the EDG 7.0 container (`cpfe --c++23 --strict --no_code_gen --instantiate=used`),
- `tidal-fe run`.

Everything is in `programmer-sketches/` beside this report:
- `round11/` holds the copies that were rerun, and `results-rerun.txt` is their log (`rerun11.sh`). `compare_rerun.py` compares the log with round 11's logs.
- `new/` holds the new programs, their earlier tries, `one-line/`, `bugs/`, and `mistakes/`. `reductions/` and `probes/` hold the reductions and probes. `results-new.txt` is their log (`runnew.sh`).
- `runall.sh` runs one program on every tool. `reduce.py` is the line-based reducer that found the first cut of the ring's lookup fault. `gen.py` writes and runs one storage probe. `work/` holds the intermediate cuts.
- `counts.py` gives the counts of section 1.1 from `git show`. `ledger_counts.py` counts both ledgers by state. `scan_tests.py` scans `tests/` for the forms of the reductions. `summarize.py` counts the front-end results of a log.

Statements that rest on general knowledge say so. Section 7 lists the claims that rest on the host compilers alone.

## 1. The state from a programmer's view

Front-end round 25 fixed every fault in the round 11 reductions. All 11 round 11 reductions now run to result 0. Every round 11 last version runs. Five more earlier tries now run to 0, and two give the results that GCC and Clang give for my mistakes. One storage fault remains in the round 11 sketches, and it gives a new wrong verdict (section 1.3).

The new programs are again library code: a fixed-capacity ring buffer with union slots and a random-access iterator, an unsigned integer of any size that works in constant evaluation, parser combinators over generic lambdas, a string with a small buffer in a union, and matrices with expression templates and the subscript with two arguments.
- One first try (`matrix.cpp`) builds clean and exits 0 on GCC and Clang, and runs on the front end with no change. The other four fail to build. Each failure is my mistake.
- After workarounds, all five run on the front end. Rounds 10 and 11 also ran all five after workarounds.
- `matrix` and `sso` needed no front-end workaround. `bigint` and `parse` needed one each, and `ring` needed two. The workarounds cost `bigint` its literal suffix, and cost `ring` the braced returns of its iterators.
- Four intermediate tries build clean and exit 0 on GCC and Clang, and the front end stops or rejects each of them.

The 9 reductions show faults that no earlier report names:
- 5 false rejects,
- 3 stops,
- 1 wrong verdict on a valid program, where the front end gives `undefined [STG-23]` for an object made in allocated storage.

The front end runs most short programs and catches planted errors well. It runs 23 of 24 short programs, stops 8 of 8 planted bugs with `undefined` and a rule, and rejects 9 of 9 planted mistakes. Its first error names the rule of the cause in all 9.

### 1.1 Counts

| Measure | Round 11 (`f940ea53`) | Now (`bd87b27c`) | Source |
|---|---|---|---|
| Rule headers | 1698 (1683 in force, 15 withdrawn) | 1700 (1685 in force, 15 withdrawn) | `counts.py` |
| Example blocks | 224: 69 unmarked, 28 `fragment`, 112 `reject`, 15 `diverges` | 230: 73 unmarked, 28 `fragment`, 113 `reject`, 16 `diverges` | `counts.py` |
| Tests | 6881 | 6947 | `counts.py`, files with `tidal-rules`, `.tu.cpp` left out |
| Program tests | 126 | 138 (7 `r11-`, 5 P3 added) | `counts.py` |
| Lines under `tests/programs/` | 15442 | 17358 | `counts.py` |
| Rules cited by a program test | 559 of 1683 | 568 of 1685 | `counts.py` |
| Tests that open `namespace std` | 269, 65 of them program tests | 278, 72 of them program tests | `counts.py` |
| Front end | 6721 of 6881 pass, 6745 their check | 6804 of 6947 pass, 6831 their check | close commits `3bd790ae`, `bd87b27c` |
| Completion line of decision 0060 | 127 tests fail their check, 2 of 126 program tests do not run | 107 tests fail their check, 3 of 138 program tests do not run, 9 out of scope | close commit `bd87b27c` |
| `conformance.tsv` | 6878 rows | 6938 rows: 6482 pass, 283 EDG-ONLY, 20 pass-partial, 10 EDG-ALLOWED, 6 NO-DIAGNOSTIC-ALLOWED, 45 MISSED-REJECT, 56 unsupported, 24 run-unsupported, 7 FALSE-REJECT, 5 EDG-AGREES | the file |
| Changes under `frontend/` | round 24 | round 25 and item RP: 20 files, 1583 lines added, 38 removed | `git diff --stat f940ea53 bd87b27c -- frontend/` |
| Port suites done | 2 of 8 (P1, P2) | 3 of 8 (P1, P2, P3) | `exercises.md` section 8 |
| Questions | 428: 180 open | 434: 185 open, 236 settled, 13 compiler defects | `status.md` |
| Front-end ledger | 163 rows, 2 open | 165 rows: 154 commit, 4 text, 4 out of scope, 1 question, 2 open (G164, G165) | `ledger_counts.py` |
| Exercise ledger | 988 rows, 26 open | 1044 rows: 960 commit, 33 text, 22 question, 3 out of scope, 26 open | `ledger_counts.py` |

### 1.2 Round 11 items that phase 14 took and left

| Round 11 item | Phase 14 | Result |
|---|---|---|
| 1. Tests for this round's reductions, probes, and clean first tries | item T3 | Done: the 11 reductions, the probes that stopped or rejected, the 5 last versions as `r11-` program tests, and the two clean first tries. The ADD-11 doubt has no test. |
| 2. Front-end round 25, with library-code targets first | item FE | Done for every round 11 target but one. `r10-list-first-try` runs. `r10-tuple-second-try` still stops, and the DEP-24 failures wait for the template model. No stop names line 0 in this round's files. |
| 3. Text for typed pointers over bytes, and G161 and G162 | item F4 | Done: ADD-14, STG-28, STG-29, and STG-P2 follow the compilers (Q-0430). The RV7 reviews read the step as undefined in the reference, so Q-0430 waits in `pending.md` row 4a. Report 25 adds G164. |
| 4. Answer the 26 open rows | item F4 | Done. P2-6 and I6-5, I6-6, and I6-15 went to questions. |
| 5. Interpreter output when every order agrees | item FE | Done (P2-9). |
| 6. First errors that name the rule of the cause | none | Deferred by decision 0062. Report 25 gives 2115 of 2743 passing rejects (77%). |
| 7. A gap review of the 22 rules changed after RV6, with a read of the RV6 repair diff | item RV7 | Done, with repairs and a recorded read of the RV6 repair diff. |
| 8. Port suite P3 on pipelines and events | item E | Done with a C# persona. Two of three first tries stopped on the front end. |
| 9. Guides | none | Deferred by decision 0062. |

Round 11 process change 1 is in place: T3 added this report's predecessor's reductions as tests. Decision 0062 names neither change 2 (a companion count of library programs) nor change 3 (a ledger row for a planted mistake whose first error names a general rule).

### 1.3 The round 11 sketches, rerun

`compare_rerun.py` reads the 98 files of `results-rerun.txt`. 58 give the same front-end result as in round 11, and 40 changed. The compilers and EDG give the same result on every file as in round 11, apart from the exit codes of planted bugs.

Of the 40 changes:
- 10 change only the first line of the interpreter's output: "unspecified: result 0 with fixed choices" became "result: 0 in every explored order" (P2-9).
- 11 are the round 11 reductions. Each now runs to result 0, as GCC and Clang do. The typed step over a byte array now moves `sizeof(int)` bytes.
- 9 are probes. Eight now run as the compilers do. `typed_step_two_past_byte.cpp` now stops with ADD-14, which follows the text of F4.
- 8 are earlier tries that now match the compilers: `expected_first_try.cpp`, `signal_first_try.cpp`, `signal_second_try.cpp`, `views_third_try.cpp`, and `views_fourth_try.cpp` run to 0, `views_first_try.cpp` gives 4, `views_second_try.cpp` gives 1, and `one-line/template_lambda.cpp` runs.
- 2 are the arena's storage tries, which section 1.5 explains.

`arena_second_try.cpp` keeps its objects in a member byte array and reads them through a cast of the array's address. The front end now gives `undefined [STG-23]`, which names the rule of the cause (STG-29). In round 11 it gave STG-17. `arena_fourth_try.cpp` takes its storage from the aligned allocation function, holds it as `unsigned char*`, and casts to `T*`. The front end gives the same STG-23 there, where STG-28 makes the program valid. Section 1.5 reduces it.

`arena_third_try.cpp` still gives ADD-11 for my subtraction of a byte pointer into an object from the storage pointer, and the compilers run it. T3 wrote no test for that doubt.

### 1.4 New programs

| Program | GCC, Clang (three flag sets) | EDG 7.0 | `tidal-fe run`, first try | `tidal-fe run`, last version |
|---|---|---|---|---|
| `ring.cpp` (248 lines): `Ring<T, N>` with a union array of slots in the object, `emplace_back` through `std::construct_at`, `push_overwrite`, `pop_front`, a random-access iterator with `<=>` and a const conversion, copy and move, `==` over two capacities, a `lower_bound`, constant tests | first and second try fail (three mistakes of mine); third exits 0, clean; fourth fails (my mistake); fifth and last exit 0, clean | as GCC and Clang | rejected, NAM-8, on `r_->slots_` in the nested iterator. **False reject.** The third try, clean on GCC and Clang, gives the same. | result 0, after two workarounds |
| `bigint.cpp` (213 lines): limbs on the free store, `+`, `*`, `divmod`, `<=>` and `==` as hidden friends, decimal output, a literal operator template `_big`, factorials and Fibonacci numbers in `static_assert` | first fails (my mistake); second exits 0, clean; third fails (my mistake); last exits 0, clean | as GCC and Clang | rejected, PRIM-2, on a 30-digit literal with the suffix `_big`. **False reject.** The second try, clean on GCC and Clang, gives the same. | result 0, after one workaround |
| `parse.cpp` (166 lines): a concept `Parser`, `map`, `operator|`, `operator>>`, `many1` with a fold, `chain_left`, an arithmetic grammar with recursion through a function, ten `static_assert` checks | first and second fail (my mistakes); third and last exit 0, clean | as GCC and Clang | rejected, CPT-34, on my mistake. The third try, clean on GCC and Clang, gives TYN-7 on a lambda's braced return. **False reject.** | result 0, after one workaround |
| `sso.cpp` (203 lines): a small buffer and a heap buffer in an anonymous union, a `StrView` with constexpr `find`, a literal operator `_sv`, `<=>`, a `consteval` hash for a `switch` on command names | first fails (two mistakes of mine); second exits 0, clean | first rejects; second accepts | rejected, UNI-9, on my first mistake | result 0, with no front-end workaround |
| `matrix.cpp` (156 lines): `Matrix<T, R, C>` with a concept `Expr`, lazy `Sum`, `Scaled`, `Product`, and `Transposed` nodes, `m[i, j]`, a row view through an explicit object parameter, a defaulted `==`, matrix powers in `static_assert` | first try exits 0, clean | rejects: a constant evaluation of the defaulted `==` over `T a[2][3]` (Q-0403) | result 0 | the first try is the last version |

The workarounds:
- `ring`: no `friend class Ring;` in the nested iterator, a public constructor, and a `template <bool> friend class Iter;`. Then `iterator(this, 0)` with parentheses in place of `{this, 0}` in the four `begin` and `end` functions.
- `bigint`: a function `big("...")` in place of the literal `..._big`. A raw literal operator over `const char*` gives the same PRIM-2 reject (section 1.5).
- `parse`: `return Result<T>{...}` in place of `return {...}` in six lambdas.

Each first try and each intermediate version is kept beside the last version (`*_first_try.cpp` to `*_fifth_try.cpp`).

### 1.5 Reductions

Each reduction builds and exits 0 on GCC and Clang under the three flag sets, and EDG accepts it. `scan_tests.py` scans `tests/` for the forms of seven of them. It finds no test with a literal operator template, a raw literal operator with a long literal, a lambda that returns a braced list into a specialization, or a braced initialization beside a constructor template that fails. Its other hits are other forms, which I read: `r10-list.cpp` declares `friend class List;` in its iterator but names `List` nowhere after it, and the two storage hits use `long` and a class with no constructor.

| Reduction | Lines | `tidal-fe` | Status | What the specification says |
|---|---|---|---|---|
| `friend_enclosing_class_in_nested_class_of_template.cpp`: `friend class Ring;` in a struct nested in `Ring<T>`, then `const Ring* r` and `r->v[1]` | 17 | rejected, NAM-8: no member named `v`. **False reject.** | New. Without the friend declaration it runs (probe). | TYN-4: a class key before a name refers to the class that lookup finds. ACC-5: the friend is that class. |
| `friend_enclosing_class_in_nested_class.cpp`: the same with no template | 16 | rejected, TYN-7: member access into an incomplete class. **False reject.** | New | as above |
| `braced_init_beside_failed_constructor_template.cpp`: `S(int)` beside `template <class T> S(T*)`, then `S s{5}` | 9 | unsupported: "constructor call" | New. `S s(5)` runs, and a template with the wrong number of parameters does not stop it (probes). | DED-35: the template adds no candidate. ILC-3 then picks `S(int)`. |
| `braced_init_beside_constructor_template_no_conversion.cpp`: `template <bool C = true> S(double)` beside `S(const int*)`, then `S s{&a}` | 9 | unsupported: "constructor call" | New. The ring's form: an iterator's const conversion beside a braced `{this, 0}`. | DED-28: no conversion for a parameter with no template parameter, so the template adds no candidate. |
| `lambda_return_specialization_braced_return.cpp`: `[](char c) -> Result<char> { return {true, c}; }` with `Result<char>` made nowhere before | 9 | rejected, TYN-7: object of incomplete class type. **False reject.** | New. Naming `Result<char>` in the return, making it before, or a function in place of the lambda runs (probes). | INST-1: the definition makes the return type. RET-2: the list initializes it. |
| `placement_class_through_byte_pointer_into_allocated_storage.cpp`: storage from `::operator new` held as `unsigned char*`, a `T*` to its start, `::new (d) T(2)`, then `d->id`, where `T` has a constructor | 17 | undefined [STG-23]. **Wrong.** | New. The cause of `arena_fourth_try.cpp`'s stop. An aggregate `T`, or a `T*` cast from the `void*` result, runs. A `T` with only a user destructor also stops (probes). | STG-28: a `T*` made from the result plus 0 bytes points to element 0, also when `T` is not an implicit-lifetime type. |
| `raw_literal_operator_long_literal.cpp`: `operator""_n(const char*)` and a 30-digit `..._n` | 5 | rejected, PRIM-2. **False reject.** | New | OPR-8: the raw form takes the digits as a string. Section 6 asks whether PRIM-2 applies to a literal with a suffix of the program. |
| `literal_operator_template.cpp`: `template <char... Cs> int operator""_n()` and `12_n` | 5 | unsupported: "literal operator template" | New | OPR-8 and OPR-10 do not list this form (section 6). GCC, Clang, and EDG accept it. |
| `class_template_specialization_with_array_member_as_constant_parameter_type.cpp`: `template <Pt<1> P>` where `Pt<N>` has one member `int x[N]` | 7 | rejected, TPL-7: not a structural type. **False reject.** | New. A plain class with an array member runs (probe). Found from the short program below. | TPL-7 and TPL-11: a class whose members are public and structural is structural. |

By status: 9 are new. The two friend reductions have one cause, and the two braced reductions have one cause. So the 9 reductions show 7 faults.

**Another fault, without a reduction of its own.** `one-line/class_type_template_parameter.cpp` declares `template <Name N>` with a class template name, and the front end stops with "cast of unknown operand". Report 25 lists this case as `constant-parameter-class-template.cpp`, so it is known.

**An EDG split that a test holds.** `matrix.cpp` and the probe `defaulted_equality_two_dimensional_array_constant.cpp` evaluate a defaulted `==` over an array of arrays in a constant expression. GCC, Clang, and the front end accept, and EDG reports an access one past an array. `eq-array-of-arrays-member-constant.cpp` and Q-0403 hold the case, with GCC and EDG as the families that differ. So `matrix.cpp` needs an EDG expectation as a program test.

### 1.6 Short programs, planted bugs, and planted mistakes

**Short programs.** The 24 short programs in `new/one-line/` build and run clean on GCC and Clang with every flag set. GCC warns once, on a variable set but not used in `static_subscript_operator.cpp`. EDG accepts 23. It rejects `defaulted_spaceship.cpp` because the program declares no `std::weak_ordering`, which LSUP-2 and Q-0386 do not ask for. The front end runs 23 (`summarize.py`: 20 "result: 0" and 3 "result: 0 in every explored order"). The forms are:
- structured bindings through a tuple protocol of my own, `if consteval`, a `static operator[]`, `auto(x)`, `[[assume]]`, and the `uz` suffix,
- a virtual call in constant evaluation, aggregate initialization with parentheses, a lambda in `decltype` as a unique type, and default member initializers on bit-fields,
- a defaulted `<=>` with my own `std::strong_ordering`, a range `for` with an init statement, a pack in an init-capture, a constrained `auto` parameter, and `explicit(bool)`,
- a class type as a constant template parameter, a builder through `this auto&&`, bit operators on a scoped enumeration, a virtual base in a diamond, and a function try block on a constructor,
- a rethrow caught by a base reference, calls through `.*` and `->*`, `operator[]` with two arguments, and a `consteval` constructor.

The front end stops on `class_type_template_parameter.cpp` (section 1.5). Its probe with `Name<6>` in place of `Name` found the TPL-7 false reject. None of this round's findings rests on the 20 rules and principles that changed after RV7 read the tree.

**Planted run-time bugs.** The front end stops all 8 bugs in `new/bugs/` with `undefined` and a rule:

| Rule the front end names | Bug | Sanitizers |
|---|---|---|
| UN-7 | `pop` on an empty list reads through a null head | UBSan |
| NEW-5 | an array from `new[]` freed with `delete` | ASan; Clang also warns |
| OBJ-2 | a pointer to an element kept while the buffer grows | ASan |
| MUL-10 | the smallest `int` divided by -1 | UBSan; the plain builds trap |
| OBJ-2, INIT-1 | a local read before any value is stored | silent; Clang warns |
| ENUM-6 | 9 converted to an enumeration whose values are 0 to 2 | UBSan (Clang only) |
| CAST-5, CV-5 | a `const` global written through `const_cast` | a crash with no report |
| FN-3 | control flows off the end of a function that returns `int` | UBSan |

The uninitialized read passes every sanitizer, and only the front end names the write to a `const` object.

**Planted compile-time mistakes.** GCC, Clang, EDG, and the front end reject all 9 mistakes in `new/mistakes/`. The front end's first error names the rule of the cause in all 9: DEL-1, ACC-2, VIRT-3, VIRT-8, CX-1 (with CE-2), CV-4, RET-7, JMP-6, and SB-2. The round 11 mistakes rerun with the same first errors: 5 of 9 name the cause, and TR-12, DED-35, and OVL-35 still stand for DEP-19, DEP-24, a CNS rule, and REF-1.

**My own mistakes from memory.** Four programs held ten:
- `ring_first_try.cpp`: a constrained `Iter(const Iter<false>&) requires Const`, which is the copy constructor of `Iter<false>` and removes the implicit one. The front end stopped first on the friend fault.
- `ring_second_try.cpp`: a wrong expected index and a missing `operator->`. The front end stopped first on the friend fault.
- `ring_fourth_try.cpp`: without the friend declaration, `Iter<true>` lost access to `Iter<false>`'s members. The front end named ACC-2, the rule of the cause (ACC-5 for what the friend gave).
- `bigint_first_try.cpp`: a division that the class does not define. The front end stopped first on PRIM-2.
- `bigint_third_try.cpp`: a variable `big` that hides the function `big` (NAM-2). The front end named OVL-35.
- `parse_first_try.cpp`: a type after `->` in a compound requirement. The front end named CPT-34, the rule of the cause.
- `parse_second_try.cpp`: `{ p(s).ok } -> Boolean` checks `bool&&`, so the concept failed. The front end stopped first on the lambda fault.
- `sso_first_try.cpp`: a struct with no name inside an anonymous union, and placement `new` with no declared placement form. The front end named UNI-9 for the first. GCC accepts that form (Q-0424).

Four of my mistakes were hidden behind a front-end false reject. A reader who trusts the front end's first error fixes the false reject first and meets the mistake later.

### 1.7 The phase 14 reports, from a programmer's view

**Front-end report 25.** The round did what round 11 asked for library code: every reduction runs, `r10-list-first-try` runs, and the interpreter's first line says when every order agrees. The completion line is 104 at the round and 107 at the close, and the gate of decision 0062 asks for at most 50. The table of section 7 gives a cause for each test, and 30 need the template model. From my side, the 30 rewrite tests hold DEP-24 through an object of the current instantiation, which round 10 met in a variant. None of this round's 7 faults is in the 30, and none needs the template model as `TEMPLATE-MODEL.md` describes it. The current design can take them.

**P3.** Its three faults on a base over the derived class (P3-7 to P3-9) and the `operator=` stop (P3-10) are library forms too. A range adaptor or an iterator facade uses that base (general knowledge). P3-1 asks for an iterator example over a computed range. My `ring` iterator is one such input. P3-6 asks when a member's return type is made. My lambda reduction is a neighbor: a return type made too late, where P3-7 makes it too early.

**I7.** The rows I meet as a programmer are I7-11 (copy initialization through a guide) and I7-17, which round 25 fixed. My programs used no class template argument deduction this round, so I7 gives me no new input.

**The RV7 reviews.** RV7-A-10 and RV7-B-3 read the typed step over bytes as undefined in the reference. F4's text follows the compilers, and Q-0430 now waits in `pending.md` row 4a. For a programmer, the compilers' answer keeps the arena on a member byte array valid, and the review's answer makes it undefined. My new storage reduction does not depend on that answer: it steps no pointer, and STG-28 decides it.

**`DESIGN-current.md` and `TEMPLATE-MODEL.md`.** These give a reader of report 25 a map from a rule ID to the files that implement it. The front end's line count grew from 50160 lines (decision 0060) to 52045 (report 25).

### 1.8 Ledger rows

The status page names 26 open exercise rows (I7-1 to I7-3, I7-5 to I7-15, and P3-1 to P3-12) and the open front-end rows G164 and G165. `ledger_counts.py` gives the same 26 and 2. No row is `planned`. P3-7 to P3-10 name their tests. G164 is the text side of this round's storage reduction: STG-28 does not say which element a `T*` from a byte pointer points to, and the front end reads a byte pointer to the start as a byte.

## 2. Ranked work items

| # | Item | Evidence | Size | Inputs |
|---|---|---|---|---|
| 1 | Tests for this round's reductions, probes, and clean tries | 9 reductions, 2 probes that stop, `matrix.cpp` and four clean intermediate tries | small | `reductions/`, `probes/`, `new/` |
| 2 | Front-end round 26, with library-code targets first | 1 wrong verdict, 5 false rejects, 2 kinds of stop; P3-7 to P3-10; `r10-tuple-second-try` | large | item 1; item 3 for the literal operator template |
| 3 | Text for literal operators and for STG-28, and G164 and G165 | OPR-8, OPR-10, PRIM-2; G164, G165 | small, with probes | `reductions/`, report 25 section 8 |
| 4 | Answer the 26 open rows, starting with the rows a programmer meets | P3-1, P3-2, P3-5, P3-6, P3-11, I7-11, then the rest | medium | the P3 and I7 reports |
| 5 | Program first tries and their mistakes in the completion count | 4 clean intermediate tries stop; 4 of my mistakes hidden | small | `results-new.txt` |
| 6 | A gap review of the 20 rules and principles changed after RV7 read the tree | close commit `bd87b27c` | one review session, then repairs | none |
| 7 | The rewrite gate, read with library programs | 104 at round 25 against a gate of 50; 0 of this round's faults in the 30 rewrite tests | one decision | report 25 section 7 |
| 8 | Port suite P4 on iterators and owned resources | 5 of this round's 7 faults come from iterators, literals, and storage | one session | item 2 |
| 9 | Guides on storage, iterators, and literal operators | the arena's storage tries, P3-1, P3-2 | one decision, then one session per guide | maintainer |

### Item 1. Tests for this round's reductions, probes, and clean tries

**What.** A clean-room writer adds a test for each of these, probes it on the matrix, and writes the expectation that the compilers and the text give:
- run tests that expect result 0: the friend declaration of the enclosing class in a nested class, with and without a template (TYN-4, ACC-5); list-initialization beside a constructor template that adds no candidate, by deduction and by conversion (DED-35, DED-28, ILC-3); a lambda's braced return into a specialization made nowhere before (INST-1, RET-2); placement `new` of a class with a constructor through a byte pointer to the start of allocated storage, and the probe with only a user destructor (STG-28); a raw literal operator with a long literal (OPR-8); a specialization with an array member as the type of a constant parameter (TPL-7, TPL-11),
- the literal operator template after item 3 states the form,
- program tests: `matrix.cpp` as a clean first try, with an EDG expectation for Q-0403, and `ring_third_try.cpp`, `ring_fifth_try.cpp`, `bigint_second_try.cpp`, and `parse_third_try.cpp`, which build clean on GCC and Clang. The last versions of all five go in as `r12-` program tests.

**Why.** Sections 1.4 and 1.5. The front-end harness sees only tests. Decision 0061 makes a clean first try a program test, and this round's clean intermediate tries hit the same faults.

**Size.** Small. Each reduction has at most 17 lines.

**Inputs.** `programmer-sketches/reductions/`, `probes/`, `new/`.

### Item 2. Front-end round 26, with library-code targets first

**What.** Round 26 takes the targets in this order:
1. The wrong verdict. A `T*` cast from a byte pointer to the start of allocated storage points to element 0 of the array of `T` (STG-28), whether or not `T` is an implicit-lifetime type. `arena_fourth_try.cpp`, the natural form of a vector in an arena, then runs.
2. The two stops. List-initialization skips a constructor template that adds no candidate. A literal operator template runs, after item 3.
3. The false rejects: `friend class Outer;` in a nested class, a lambda's braced return into a new specialization, PRIM-2 on a literal with a suffix of the program, and TPL-7 on a specialization with an array member.
4. P3-7 to P3-10, the base over the derived class and the `operator=` stop.
5. `r10-tuple-second-try` and the 74 tests that report 25 marks as possible on the current design.

**Why.** Sections 1.4 and 1.5. The first three targets are what a library programmer writes first: an iterator that returns `{this, 0}`, a lambda that returns `{true, v}`, a friend declaration of the container, and a vector over raw storage. My workarounds cost `ring` its braced returns and `bigint` its literal suffix.

**Size.** Large.

**Inputs.** Item 1's tests. Item 3 for the literal operator template.

### Item 3. Text for literal operators and for STG-28, and G164 and G165

**What.**
- OPR-8 and OPR-10: say whether a literal operator template with a `char` pack is a valid form, and which literals call it. Every compiler accepts it.
- PRIM-2: say whether the rule applies to a literal whose suffix names a literal operator of the program. OPR-8 gives the raw form the digits, so no type needs to hold them.
- STG-28 and G164: say that a `T*` cast from a pointer to the byte at element `k`'s offset in allocated storage points to element `k`. Say that the cast can pass through `unsigned char*` and that `T` can have a constructor. STG-28 already says that the elements exist for any `T`.
- G165: say whether VIRT-13 warns for each hidden base overload.

**Why.** Section 1.5. A literal suffix for a big number or a unit is a common library form (general knowledge), and the text has no answer for its template form. G164 and G165 are the open front-end rows.

**Size.** Small. Each needs a probe on the matrix.

**Inputs.** `reductions/literal_operator_template.cpp`, `raw_literal_operator_long_literal.cpp`, `placement_class_through_byte_pointer_into_allocated_storage.cpp`, its probes, and report 25 section 8.

### Item 4. Answer the 26 open rows, starting with the rows a programmer meets

**What.** For a programmer, these come first:
- P3-1: an iterator example over a computed range with a separate end type. This round's `ring` iterator and round 11's `views` are inputs.
- P3-2: the delegate form of LAM-P4. Round 11's `signal` is an input.
- P3-5 and P3-11: a stage that holds a temporary by reference, and copy-initialization through a converting constructor.
- P3-6: when a member definition's return type is made (INST-1, INST-5).
- I7-11: copy initialization through a guide.

Then P3-3, P3-4, P3-12, and the other I7 rows. P3-7 to P3-10 go with item 2.

**Why.** Section 1.7. P3 and this round met the same library forms from two personas.

**Size.** Medium.

**Inputs.** The P3 and I7 reports and their programs.

### Item 5. Program first tries and their mistakes in the completion count

**What.** A phase close also gives how many clean tries of the roadmap and port programs run on the front end. In this round, 1 of 5 first tries and 4 of 4 clean intermediate tries meet a front-end fault, and 4 of my 10 mistakes sat behind a false reject.

**Why.** Section 1.6. Decision 0060's line counts tests only. In this round, five library programs in one session found 7 faults that the 6947 tests do not hold. Round 11 found 11 the same way. This is round 11 process change 2, which decision 0062 did not take.

**Size.** Small.

**Inputs.** `results-new.txt`, `summarize.py`.

### Item 6. A gap review of the 20 rules and principles changed after RV7 read the tree

**What.** A source-reader review of DTOR-8, UNI-P3, UNI-14, REF-2, LSUP-6, STG-P2, LOOP-9, CNS-P3, CNS-13, DED-20, DEP-P1, DEP-P5, DEP-P6, DEP-4, DEP-6, DEP-40, DEP-30, INST-P1, INST-1, and INST-2, a recorded read of the RV7 repair diff, then repairs. Item 3's text joins the review.

**Why.** The close commit names these rules for the next review. A programmer ranks the review below items 1 to 5, because no finding of this report rests on these rules.

**Size.** One review session, then repairs.

**Inputs.** None.

### Item 7. The rewrite gate, read with library programs

**What.** The phase decision says whether round 26 aims at the gate of decision 0062 on the current design, or whether the template model starts for the 30 tests that need it. From a programmer's view, round 26 on the current design comes first, because none of this round's faults needs the model.

**Why.** Report 25 section 7: 104 tests, of which 30 need the model and 74 do not. The gate asks for at most 50.

**Size.** One decision.

**Inputs.** Report 25, `TEMPLATE-MODEL.md`.

### Item 8. Port suite P4 on iterators and owned resources

**What.** A port suite with a persona from a language with iterators and ownership in its library, such as Rust or Python. Ports: an iterator over a computed sequence, a buffer that owns its elements, and a number type with a literal suffix.

**Why.** Five of this round's 7 faults come from those forms, and P3 met the iterator gap from another side (P3-1).

**Size.** One session.

**Inputs.** Item 2, so that P4 measures the new front end.

### Item 9. Guides on storage, iterators, and literal operators

**What.** A maintainer decision on publishing guides, then how-to guides on a vector in raw storage, an iterator with a const conversion, and a literal suffix.

**Why.** The arena needed three storage changes over two rounds. The rules sit on the storage, additive, and new-delete pages, and RV7 reads part of them against the compilers.

**Size.** One decision, then one session per guide.

**Inputs.** The maintainer.

## 3. Out of scope

| Topic | Proposal | For | Against |
|---|---|---|---|
| The general library, including `construct_at`, `strong_ordering`, `tuple_size`, and `size_t` | Keep out. Programs declare their own, as the five new programs do. | Each program needs them (general knowledge). | The charter leaves out the library. LSUP-1, LSUP-2, LSUP-10, and LSUP-12 give the shapes. |
| The launder operation | Keep referenced. | An arena on a member byte array needs it, or the compilers' answer of Q-0430. | Storage from the allocation function avoids it. |
| The front-end rewrite (FR) before the gate of decision 0062 | Keep waiting for the gate. | 30 failing tests need a new model. | 74 tests and this round's 7 faults need no new model. |
| `typeid` | Keep deferred. | | Each test needs the library's `type_info` (Q-0322). |
| Threads, the memory model, coroutines, modules, extended floating-point types | Keep out. | P3-1 asks for a lazy generator. | as the charter says |
| Language versions after C++23 | Keep out until `pending.md` row 7 is decided. | | Strict mode rejects them. |

## 4. Process changes

1. **A roadmap round adds its own reductions as tests in the phase that follows it**, as T2 and T3 did. Item 1 keeps that practice.
2. **A clean intermediate try is a program test**, as a clean first try is under decision 0061. In this round, four clean intermediate tries met a front-end fault, and only one first try built clean.
3. **The phase close gives a count of library programs beside the completion line** (item 5). Decision 0062 did not take round 11's change 2.

## 5. Suggested order

1. Item 3 comes first, because it is small and item 1's literal operator test needs its answer.
2. Item 1's tests are the first commit of item 2. Round 26 then takes the wrong verdict and the two stops first.
3. Item 4 runs beside item 2.
4. Item 7 is part of the phase decision. Item 8 follows item 2.
5. Items 5 and 6 run beside the others at any point. Item 9 waits on the maintainer.

## 6. Doubts for the clean-room writers

- Whether a literal operator template with a `char` pack is valid. OPR-10 lists the valid parameter lists, and an empty list with a template head is not among them. Every compiler accepts the form.
- Whether PRIM-2 applies to a literal whose suffix names a literal operator of the program. The front end says yes, and the compilers say no.
- Whether STG-28's "made from the result" covers a `T*` made through `unsigned char*`, for a `T` with a constructor. My reading is yes, and the front end says the pointer points to a byte.
- Whether the arena on a member byte array is valid. F4 follows the compilers, and the RV7 reviews read the reference as undefined (Q-0430, `pending.md` row 4a).
- Whether an `unsigned char*` that points into an object can be subtracted from the pointer to the storage that hosts the object (ADD-11). This round 11 doubt has no test and no text.

## 7. Host-only claims

Decision 0061 asks a reader report to list these, so that a writer probes them on the matrix first:
- Every reduction and probe result for GCC and Clang comes from host GCC 16.2.1 and Clang 22.1.8. EDG 7.0 ran in its container. GCC 14, GCC 16.2.0, and Clang 18 did not run.
- The 9 reductions and the clean tries of section 1.4 build and exit 0 on the host GCC and Clang.
- The sanitizer results of section 1.6 come from the host compilers. UBSan of the host GCC does not report the enumeration bug, and no host sanitizer reports the uninitialized read.
- The host GCC accepts the struct with no name inside an anonymous union of `sso_first_try.cpp` under `-pedantic-errors`, as Q-0424 records for GCC 16.
