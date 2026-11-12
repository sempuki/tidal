# Roadmap, round 15: what to do after phase 17, from a programmer's view

Date: 2026-11-11. Decision 0029. This report comes from a clean-room reader in the role of a C++ programmer who uses the specification to write and check programs.

**What the reader read.**
- AGENTS.md, CLAUDE.md, and the round 14 programmer report (`reports/clean-room/roadmap-14/programmer.md`) with its sketches and logs.
- The status page, the roadmap's phase 17 table, decisions 0060, 0064, and 0065, and `documentation/decisions/pending.md`.
- `git log fb0bcf87..HEAD` with every commit body (19 commits: the phase 16 corrections, decision 0065, the items GR (two parts), E, T6, F7, FE, RV10 (three parts and a settlement), and RW2, four ledger commits, the takeover commit, and the close).
- The X26 report, the front-end reports `report-rw2.md`, `report-28.md`, and `report-rw3.md`, the RV10 review and the read of its repair diff in `reports/review/`, `frontend/DESIGN-rewrite.md`, `frontend/TEMPLATE-MODEL.md`, `rewrite-marks.tsv`, `takeover.tsv`, `reports/clean-room/ledger.md`, and `rust-frontend/ledger.md`.
- The rules and tests that the findings name: TARG-10, TARG-18, PFX-10, PRIM-11, PRIM-12, DED-6, DED-17, DED-35, CNS-11, CNS-35, NAM-27, NAM-38, OVL-7, OVL-35, BIT-3, CTOR-9, REF-5, VC-9, STG-23, STG-29, LIST-4, the tests `explicit-argument.cpp`, `explicit-copy-init.cpp`, `recursive-satisfaction-operator.cpp`, `returned-reference-temporary-sanitizer.cpp`, `template-keyword-base-last-name.cpp`, and `placement-then-cast-read.cpp`.

The reader did not read the other round 15 reports, the sources under `frontend/src/` and `frontend/next/src/`, `inputs/`, or `build/exercise-keys/`. Commit read: `7a849708`, the close of phase 17.

**What the reader did.** The reader built both front ends from `7a849708` into its scratchpad with cargo, with no build failure: `tidal-fe` from `frontend/` and `tidal-next` from `frontend/next/`. The reader reran the 98 round 14 files and wrote five new programs (1208 lines) with 13 earlier tries, 20 short programs, 9 planted bugs, 9 planted mistakes, 20 reductions, and 27 probes. Each file went through these checks:
- host GCC 16.2.1 and Clang 22.1.8 with `-std=c++23 -pedantic-errors -Wall -Wextra`,
- the same with `-std=c++23 -pedantic-errors` alone,
- the same with `-fsanitize=undefined,address`,
- the EDG 7.0 container (`cpfe --c++23 --strict --no_code_gen --instantiate=used`),
- `tidal-fe run`,
- `tidal-next run`, the rewrite, which now analyzes and runs the step 3 constructs, and `tidal-next parse`.

Everything is in `programmer-sketches/` beside this report:
- `round14/` holds the copies that were rerun, and `results-rerun.txt` is their log (`rerun14.sh`). `compare_rerun.py` compares the log with round 14's log (`compare-rerun.txt`).
- `new/` holds the new programs, their earlier tries, `one-line/`, `bugs/`, and `mistakes/`. `reductions/` and `probes/` hold the reductions and probes. `results-new.txt` is their log (`runnew.sh`).
- `runall.sh` runs one program on every tool, and `quick.sh` runs the two host compilers and both front ends. `reduce.py` is the line-based reducer of round 14, pointed at this round's build, with a switch for `tidal-next`.
- `counts.py` gives the counts of section 1.1 from `git show` (`counts.txt`). `ledger_counts.py` counts both ledgers by state (`ledger-counts.txt`). `scan_tests.py` scans `tests/` for the forms of the reductions (`scan.txt`). `table.py` prints one line per file of a log. `summarize.py` sorts a log by the compilers' verdict and each front end's result (`summary-new.txt`, `summary-rerun.txt`). `run_programs.py` runs `tidal-next run` and `tidal-next parse` on the program tests and on the roadmap programs of rounds 8 to 15 (`run-programs.txt`). `conform-next.txt` holds the first table of `tidal-next conform tests -v`.

Statements that rest on general knowledge say so. Section 7 lists the claims that rest on the host compilers alone.

## 1. The state from a programmer's view

Front-end round 28 fixed every fault of the round 14 reductions that it did not mark. Every round 14 file now gets the compilers' verdict from `tidal-fe`, except the four files with a `static constexpr` member of a class template initialized by a call of its own member function, which round 28 marked for model section 3.

The new programs are again library code: strong types with skills as mixins, a type-erased value with a small buffer, flag sets over scoped enumerations, a set of intervals, and scope guards with exceptions.
- Two first tries (`intervals_first_try.cpp`, `guard_first_try.cpp`) build clean and exit 0 on GCC and Clang, and EDG accepts them. The old front end gives a false reject on the first and runs the second. A third, `anybox_first_try.cpp`, also builds clean and exits 0, and only the old front end finds its bug (section 1.6).
- Three of the five first clean tries get a false reject from the old front end: `strong_second_try.cpp`, `flags_third_try.cpp`, and `intervals_first_try.cpp`.
- After workarounds, all five run on the old front end. `anybox` and `guard` needed no workaround, `flags` one, `intervals` two, and `strong` three.

The 20 reductions show 16 faults:
- 7 in `tidal-fe`: 3 false rejects, 1 missed rejection, and 3 stops on valid programs. Five are new in kind. Two have a test that already fails for the same cause.
- 9 in `tidal-next run`: 6 kinds of rejection of valid programs where a construct of a later step is involved (7 reductions), 1 run stopped as undefined where the program is valid, 1 run that misses undefined behavior, and 1 parse stop at a class template's own name in its base clause.

The old front end runs every short program and catches planted errors well. It runs 20 of 20 short programs, stops 9 of 9 planted bugs with `undefined` and a rule, and rejects 9 of 9 planted mistakes. Its first error names the rule of the cause in 6 of the 9.

The rewrite runs few programs and gives wrong verdicts on some. `tidal-next run` gives the expected result for 1 of 178 program tests. It rejects 21 valid program tests and stops one with `undefined`, and the other 155 stop as unsupported. On this round's files, it rejects 25 valid files.

### 1.1 Counts

| Measure | Round 14 (`f77bbe05`) | Now (`7a849708`) | Source |
|---|---|---|---|
| Rule headers | 1700 (1685 in force, 15 withdrawn) | 1700 (1685 in force, 15 withdrawn) | `counts.py` |
| Example blocks | 240: 79 unmarked, 30 `fragment`, 114 `reject`, 17 `diverges` | 240, the same split | `counts.py` |
| Tests | 7073 | 7148 | `counts.py`, files with `tidal-rules`, `.tu.cpp` left out |
| Program tests | 166 | 178 (12 `r14-` added by T6) | `counts.py` |
| Lines under `tests/programs/` | 23005 | 25553 | `counts.py` |
| Rules cited by a program test | 587 of 1685 | 589 of 1685 | `counts.py` |
| Tests that open `namespace std` | 307, 96 of them program tests | 307, 96 of them program tests | `counts.py` |
| Old front end | 6967 of 7073 pass, 6990 their check | 7049 of 7148 pass, 7069 their check | close commits `58060b8f`, `7a849708` |
| Completion line of decision 0060 | 74 tests fail their check (36 marked, 38 toward the gate), 0 of 166 program tests do not run | 70 tests fail their check (38 marked, 32 toward the gate), 1 of 178 program tests does not run | close commits |
| `conformance.tsv` | 7073 rows | 7131 rows (round 28): 6679 pass, 309 EDG-ONLY, 35 pass-partial, 35 MISSED-REJECT, 25 unsupported, 19 run-unsupported, 10 EDG-ALLOWED, 7 FALSE-REJECT, 6 NO-DIAGNOSTIC-ALLOWED, 6 EDG-AGREES. 17 RV10 tests have no row. | the file |
| Passing rejects whose first error names a cited or linked rule | 2154 of 2782, 77% | 2177 of 2805, 78% | report 27, `report-rw3.md` |
| Changes under `frontend/` | round 27 and RW: 230 files, 11001 lines added | GR, round 28, and RW2: 198 files, 13145 added, 384 removed; `frontend/src/` 11 files, 371 added; `frontend/next/` 183 files, 12543 added | `git diff --shortstat fb0bcf87 7a849708` |
| Rewrite lines | `next: unit` 88, `lex` 7191/7191, `parse` 6475/6913, `owned` 0 | `unit` 126, `lex` 7266/7266, `parse` 6828/6987, `shadow: step 3 plain` 1602/1897, `owned` 1327 | close commits |
| Program tests that parse to a tree | 128 of 166 | 176 of 178 | `run_programs.py` |
| Program tests that `tidal-next run` runs to their result | none | 1 of 178; 21 rejected, 1 `undefined`, 155 unsupported | `run_programs.py` |
| Program tests that the rewrite owns | none | 0 of 1327 owned tests | `takeover.tsv` |
| Port suites done | 4 of 8 (P1 to P4) | 4 of 8 (P1 to P4) | `exercises.md` section 8 |
| Questions | 449: 200 open | 452: 202 open, 237 settled, 13 compiler defects | `status.md` |
| Front-end ledger | 180 rows, 14 open | 183 rows: 172 commit, 4 text, 4 out of scope, 1 question, 2 open (G182, G183) | `ledger_counts.py` |
| Exercise ledger | 1129 rows, 15 open | 1164 rows: 1054 commit, 54 text, 40 question, 3 out of scope, 13 open | `ledger_counts.py` |

### 1.2 Round 14 items that phase 17 took and left

| Round 14 item | Phase 17 | Result |
|---|---|---|
| 1. Tests for the reductions, probes, and clean tries | item T6 | Done: 11 rule tests, 12 program tests, and G181's test. Round 28 passes all but the two that it marked. |
| 2. Syntax segments for the forms that programs use | item GR | Done: G166 to G179 and four more forms. 176 of 178 program tests parse to a tree, up from 128 of 166. |
| 3. Front-end round 28 on the current code, library targets first | item FE | Done for each target: the false reject, the required rejection (CE-16), both stops, and TPL-11. Run stops went from 25 at the start of the round to 19. |
| 4. The rewrite, step 3, with a line for program tests and the design kept to the marks | item RW2 | Step 3 in part. `DESIGN-rewrite.md` section 5 gives the 38 marks (17, 15, 6). The NAM-21 oracle point is fixed (G181). No `next:` line counts program tests. |
| 5. Answer the 15 open rows, X24-2 first | item F7 | Done. JMP-6 now accepts `continue` before a later declaration. |
| 6. A gap review of the 10 changed rules | item RV10 | RV10 read GR's and F7's text. The correction commit `5d1a26bf` says that the RV9 read had already covered the 10 rules. |
| 7. Reject tests whose first error is a grammar error | none | Deferred again by decision 0065. |
| 8. Program first tries in the completion count | none | Deferred again. |
| 9. Port suite P5 | none | Deferred. |

Round 14 process change 1 is in place: T6 added the reductions as tests. Change 3 holds for the marks of round 28, which `DESIGN-rewrite.md` and `TEMPLATE-MODEL.md` count. Changes 2 and 4 are not in place.

### 1.3 The round 14 sketches, rerun

`compare_rerun.py` reads the 98 files of `results-rerun.txt`. 77 give the same `tidal-fe` result as in round 14, and 21 changed. The compilers and EDG give the same verdict on every file as in round 14.

Each of the 21 changes now gives the compilers' verdict:
- 7 reductions and 8 probes of round 14 faults now run to 0: the reference captures in generic lambdas, the closure passed to a converting constructor template, the list with too many values beside a nested aggregate, and the class template as the type of a constant parameter.
- `static_constexpr_member_from_member_function_plain_class.cpp` is now rejected. The first error names CX-1, and its text names CE-16 and CLS-2.
- 5 clean tries now run to 0: `fields_first_try.cpp`, `fields_second_try.cpp`, `fnref_fifth_try.cpp`, `fnref_sixth_try.cpp`, and `keys_first_try.cpp`.

Four files still stop as unsupported, all on the marked INST-5 form: `fsm_first_try.cpp` (also my PACK-6 mistake), `fsm_second_try.cpp`, and the round 14 reduction and probe of the form.

`tidal-next run` on the 98 files (`summary-rerun.txt`):
- On the 72 valid files, it runs 3 to 0, rejects 5, and stops as unsupported on 64. The 5 rejections are `fsm.cpp`, `fsm_second_try.cpp`, `keys.cpp`, and `keys_first_try.cpp` (NAM-16) and `multi_subscript.cpp` (COMMA-6).
- On the 8 planted bugs, it stops 4 with `undefined` and a rule. It runs `dangling_reference_return.cpp` to 0, where the sanitizers and the old front end stop it (section 1.5).
- On the 18 invalid files, it rejects 7 and stops as unsupported on 11.

`tidal-next parse` gives a tree for 95 of the 98 files. The 3 stops are my own mistakes, where every compiler stops too.

### 1.4 New programs

| Program | GCC, Clang (three flag sets) | EDG 7.0 | `tidal-fe run`, first clean try | `tidal-fe run`, last version | `tidal-next run`, last version |
|---|---|---|---|---|---|
| `strong.cpp` (234 lines): `Strong<T, Tag, Skills...>` with each skill a CRTP mixin of hidden friends, a binary skill that divides one strong type by another, an increment skill with an explicit object parameter, concepts that test which operations exist, and a hash table keyed by a strong id | first fails (my two mistakes); second to last exit 0, clean | rejects the first try; accepts the rest | second try: rejected, TARG-10. **False reject.** | result 0, after three workarounds | rejected, TR-12: the parser stops at the class template's own name in its base clause |
| `anybox.cpp` (230 lines): `Any` with a buffer of three pointers, a table of function pointers per type, the address of a variable template as the type's identity, values in place or on the heap, and counted lifetimes | both versions exit 0, clean | accepts | first try: `undefined`, STG-23 (my bug, section 1.6); second try: result 0 | result 0, no workaround | rejected, NAM-14 at `using ab::Any;` |
| `flags.cpp` (224 lines): `Flags<E>` over a scoped enumeration that opts in through a traits specialization, operator templates constrained by a concept, iteration over set bits, printing into a buffer, parsing `"read|write"`, and `using enum` in a switch | first and second fail (my mistakes); third and last exit 0, clean | rejects the first two; accepts the rest | third try: rejected, SA-1. **False reject.** | result 0, after one workaround | rejected, NAM-14 at `using fl::Flags;` |
| `intervals.cpp` (214 lines): `IntervalSet<T, N>` of sorted half-open intervals with merge on insert, split on erase, union, intersection, complement, a `lower_bound` with a projection, structured bindings in range `for`, and a meeting planner in constant evaluation | all versions exit 0, clean | accepts | first try: rejected, DED-35. **False reject.** The second try stops: "constructor call". | result 0, after two workarounds | unsupported: a template declaration |
| `guard.cpp` (306 lines): `ScopeExit`, `Rollback`, and `Resource` with conditional `noexcept`, a key-value store that undoes a batch when a change throws, a function try block on a constructor, a rethrow, and a handler that slices | first and last exit 0, clean (GCC warns on the name `log` in the first); second exits 23 (my mistake) | accepts | first try: result 0 | result 0, no workaround | unsupported: a template declaration |

The workarounds:
- `strong`: the binary skill is an alias template with one parameter, in place of a member template of a class template specialization. The skills are classes with a member template `apply<S>`, in place of a pack of template template parameters. The dependent base `K::template apply<S>` goes through an alias template.
- `flags`: the operators on the enumerations live in the global namespace, in place of the library namespace with using-declarations in the enumeration's namespace.
- `intervals`: `contains` calls `lower_bound` with `begin()` and `end()`, in place of the array member. A static member function template `of` takes the braced list of pairs, in place of a constructor template.

Each first try and each intermediate version is kept beside the last version (`*_first_try.cpp` to `*_fifth_try.cpp`). Four intermediate tries build clean and exit 0 on GCC and Clang: `strong_third_try.cpp`, `strong_fourth_try.cpp`, `strong_fifth_try.cpp`, and `intervals_second_try.cpp`. The old front end stops on all four.

### 1.5 Reductions

The 20 reductions build and exit 0 on GCC and Clang under the three flag sets, and EDG accepts them, with two exceptions. `recursive_constraint_hidden_friend_mixin.cpp` is a reject reduction: GCC and Clang reject it, and EDG accepts it. `next_dangling_reference_to_argument_temporary.cpp` has undefined behavior, and both sanitizer builds stop it. `scan_tests.py` scans `tests/` for the forms (`scan.txt`), and I read each hit:
- No test uses a member template of a class template specialization as a template template argument.
- The ten tests with a using-declaration of an operator function use it in a class, or for `!=` on a class. None has a scoped enumeration operand.
- Six tests have a template template parameter pack. `expansion-elements.cpp` (PACK-5) is unsupported and counts toward the gate. `member-template-pack-length.cpp` is marked.
- One test names a base with `::template`: `template-keyword-base-last-name.cpp` (DEP-25), which RV10 added and which stops in a run. It has no row in `conformance.tsv`.
- Eight program tests have a constructor template whose parameter is a reference to an array of deduced bound. Each passes a named array or a string literal. None passes a braced list.

**Faults in `tidal-fe`.**

| Reduction | Lines | `tidal-fe` | Status | What the specification says |
|---|---|---|---|---|
| `adl_operator_scoped_enum_own_namespace.cpp`: `app::P::r \| app::P::w` outside `app`, with `operator\|` for the scoped enumeration `P` declared in `app` | 6 | rejected, BIT-3: "scoped enumeration operand". **False reject.** | New. The same expression inside `app` runs, and so does an operator on a class operand found the same way (probes). A named function found by argument-dependent lookup runs too. | NAM-27: the enumeration adds `app` as an associated namespace. NAM-38, OVL-7: an operator expression finds non-member candidates by argument-dependent lookup. |
| `adl_operator_through_using_declaration_requires.cpp`: `requires(E a) { a \| a; }` for the same kind of operand, with the operator brought into the enumeration's namespace by a using-declaration | 10 | rejected, SA-1 | Same cause. The concept is false, so a library that tests for operators in a concept goes wrong without a word. | NAM-27, OVL-7, CPT-26 |
| `const_member_function_array_member_deduction.cpp`: `first(items, items + 4)` with `template <class It> It first(It, It)`, in a `const` member function, with `items` an array member | 7 | rejected, DED-35. **False reject.** | New. `items + 0`, a non-const member function, and a `const` local array run (probes). | PRIM-11, PRIM-12: in a `const` member function, `items` is `const int[4]`. DED-6: the array becomes `const int*`, so both arguments deduce the same type. |
| `const_member_function_array_member_decay.cpp`: the same array member passed to one parameter of type `It` | 7 | rejected, PFX-16: "no implicit conversion from const int* to int*" | Same cause. The decay drops `const`. | PRIM-12, DED-6 |
| `member_template_of_specialization_as_template_argument.cpp`: `Use<Wrap<char>::skill>` with `skill` a member class template | 3 | rejected, TARG-10. **False reject.** | New. A member template of a class that is not a template runs, and so does `Wrap<char>::skill<int>` (probes). | TARG-10 applies where a type or a value is needed. A template template argument is neither, and TARG-18 checks it. |
| `recursive_constraint_hidden_friend_mixin.cpp` (reject): a CRTP base whose hidden friend templates `operator*` take a constraint `requires(const S& s, N n) { s.get() * n; }`, then `2 * M(30) * 1` | 15 | result 0. GCC and Clang reject it; EDG accepts it. | New. `recursive-satisfaction-operator.cpp` holds a concept form and passes. | CNS-35: the check of the constraint for `N = M` needs the same check again through the reversed operator. |
| `constructor_template_array_reference_braced.cpp`: `template <size_t M> S(const P (&init)[M])`, then `S({{1, 2}, {3, 4}})` | 8 | unsupported: "class temporary" | New. A function template with the same parameter, a named array, and a fixed bound run (probes). | DED-17: each element of the list is an argument for `P`, and `M` takes the number of elements. |
| `template_template_pack_base_expansion.cpp`: `template <template <class> class... K> struct L : K<int>... {};` | 3 | unsupported: "class layout the front end does not model" | Known. `expansion-elements.cpp` (PACK-5) is unsupported for the same cause. The same form through a partial specialization runs, and so does a pack of types (probes). | PACK-5 |
| `template_template_pack_as_arguments.cpp`: the same pack passed on as `L<K...>` | 5 | unsupported: "dependent name" | Same cause. | PACK-5 |
| `dependent_template_keyword_base.cpp`: `template <class K> struct D : K::template apply<int> {};` | 3 | unsupported: "class layout the front end does not model" | Known. `template-keyword-base-last-name.cpp` stops in a run. The same base through an alias template runs, and so does `typename K::template apply<int>` as a member type (probes). | DEP-25 |

So the 10 `tidal-fe` reductions show 7 faults. Five are new in kind: argument-dependent lookup for an operator on a scoped enumeration, the array member in a `const` member function, the member template of a specialization as a template template argument, the recursive constraint, and the constructor template from a braced list. The template template pack and the `::template` base each have a failing test.

**Faults in `tidal-next run`.** Each reduction runs to 0 on the old front end, except the dangling reference, which it stops with OBJ-2.

| Reduction | Lines | `tidal-next run` | Status | What the specification says |
|---|---|---|---|---|
| `next_alias_of_class_qualifier_false_reject.cpp`: `using B = S; return B::n - 3;` | 3 | rejected, NAM-16: "'B' does not name a namespace, class, or enumeration" | New. `d::S::n` with no alias gives unsupported (probe). 9 program tests stop on this form, among them `r13-units.cpp`, `r14-fsm.cpp`, and `r14-keys.cpp`. | NAM-16 rejects a name that names no class. `B` names a class, which step 3 does not model. Decision 0060 keeps the `unsupported` result for such constructs. |
| `next_using_declaration_class_false_reject.cpp`: `using a::T;` with `T` a class | 3 | rejected, NAM-14: "'T' is not a member of the named scope" | New. A function template named the same way gives the same rejection (probe). `anybox` and `flags` stop on this form. | NAM-14 and decision 0060, as above |
| `next_multi_subscript_class_false_reject.cpp`: `g[1, 2]` with `G::operator[](int, int)` | 2 | rejected, COMMA-6 | New. 4 program tests stop on this form, among them `r7-grid.cpp` and `r12-matrix.cpp`. | COMMA-6 applies to a built-in subscript. A class operand calls the operator function. |
| `next_enum_operator_function_false_reject.cpp`: `operator&` for a scoped enumeration, then `Flag::a & Flag::b` | 3 | rejected, BIT-3 | New. `r8-config.cpp` stops on this form. | OVL-7: the operator function is a candidate. Step 3 does not model operator functions, so the result is unsupported. |
| `next_forward_class_declaration_false_reject.cpp`: `struct Later;` | 2 | rejected, TR-12: "the declaration declares nothing" | New. `P3/machine.cpp` and `W9/visitor.cpp` stop on this form. | The grammar derives the declaration, and it declares a class. |
| `next_out_of_class_const_member_false_reject.cpp`: `int S::get() const { ... }` | 3 | rejected, DCL-21: "a function that is not a member has a cv-qualifier" | New. 3 program tests stop on this form or the next. | DCL-21 applies to a function that is not a member. `S::get` is a member. |
| `next_member_function_pointer_type_false_reject.cpp`: `using Getter = int (S::*)() const;` | 3 | the same | Same cause. | DCL-21 allows the cv-qualifier in the type of a pointer to member function. |
| `next_byte_copy_false_undefined.cpp`: copy a `float` into an `unsigned` byte by byte through `unsigned char*` | 8 | `undefined`, PFX-4: "the subscript 1 is outside the array of 1 elements" | Known in kind: `DESIGN-rewrite.md` section 5 says that step 3 lacks the byte view. `r4-bits.cpp` gives the same stop. | The byte view is a step that the machine does not model, so the result is unsupported. A stop with `undefined` is a wrong verdict. |
| `next_dangling_reference_to_argument_temporary.cpp`: `const int& pick(const int& a) { return a; }`, then a read through `pick(2)` after the full expression | 5 | result 0 | Known: `returned-reference-temporary-sanitizer.cpp` (REF-5, OBJ-2) gives result 0 on `tidal-next`, and the takeover list leaves it out. A reference to a local and a pointer to a local of a returned frame stop (probes). | VC-9 and REF-5: the temporary ends with the full expression. OBJ-2: the read is undefined. |
| `next_class_template_own_name_in_base.cpp`: `template <class T> struct Box : Mixin<Box<T> > {};` | 3 | rejected, TR-12: "expected '{', found '>'" | Known as RW2-2 in `report-rw2.md`. No ledger row and no point in `DESIGN-rewrite.md` section 10 names it. `strong.cpp` and its tries stop on it. | The grammar derives the form, and the class template's name is declared at its head. |

So the 10 `tidal-next` reductions show 9 faults. Six are kinds of rejection of valid programs where a construct of a later step is involved. With the parse stops, they reach 21 of the 178 program tests: the 9 qualifiers, 4 subscripts, 3 cv-qualified members, 2 forward declarations, 1 operator, and 2 parse stops (the CRTP point of P3's first try and G182 in `r10-variant.cpp`). One more program test, `r4-bits.cpp`, stops with `undefined` on the byte view. `tidal-next conform` agrees in kind: over the 178 program tests it gives 6 passes, 12 FALSE-REJECT, 10 EDG-AGREES, and 150 unsupported (`conform-next.txt`).

### 1.6 Short programs, planted bugs, and planted mistakes

**Short programs.** The 20 short programs in `new/one-line/` build and run clean on GCC and Clang with every flag set, with no warning. EDG accepts all 20. The old front end runs all 20 (17 "result: 0" and 3 "result: 0 in every explored order"). The forms are:
- a `static` call operator on a lambda, a `static constexpr` local in a `constexpr` function, a label at the end of a block, a recursive lambda with `this auto`, and an explicit object parameter by value,
- the implicit move of an rvalue reference parameter in a return, `using enum` in a class, nested designated initializers, range `for` with an init statement, and `[[likely]]` and `[[unlikely]]`,
- `constinit`, a template lambda, a constrained abbreviated function template, aggregate deduction with no guide, and an init capture pack,
- `T::type` as a return type with no `typename`, a floating constant template parameter, a structured binding in an `if` init statement, a lambda in `decltype`, and default member initializers for bit-fields.

`tidal-next run` runs 6 of the 20 to 0 and stops as unsupported on 14 (classes, templates, lambdas). `tidal-next parse` gives a tree for all 20.

**Planted run-time bugs.** The old front end stops all 9 bugs in `new/bugs/` with `undefined` and a rule:

| Rule the old front end names | Bug | Sanitizers | `tidal-next run` |
|---|---|---|---|
| SHF-7 | a shift by the width of the type | UBSan; the plain builds give a wrong result | SHF-7 |
| PFX-4 | a loop that reads one element past the end | ASan; the plain builds give a wrong result | PFX-4 |
| NEW-5 | a second `delete` of one object | ASan; the plain builds abort | unsupported |
| FN-3 | a function that flows off its end, and its result is used | UBSan; the plain builds trap; both compilers warn | FN-3 |
| UNI-6 | a read of a union member that is not active | none | unsupported |
| OBJ-2 | a pointer to a local that a returned object keeps | ASan; GCC's plain build gives a wrong result | unsupported |
| VIRT-12 | `delete` through a base with no virtual destructor | ASan; the plain builds abort | unsupported |
| SEQ-4 | two unsequenced increments of one variable | none; both compilers warn | result 0 |
| ENUM-6 | a cast of 9 to an enumeration whose values end at 3 | UBSan of Clang only; Clang warns | result 0 |

The union read passes every sanitizer, and only the old front end names it. `tidal-next` misses two bugs in the step 3 scope. `report-rw3.md` lists both kinds among its missed stops.

**Planted compile-time mistakes.** GCC, Clang, and EDG reject all 9 mistakes in `new/mistakes/`. The old front end rejects all 9:
- Its first error names the rule of the cause in 6: OVL-36, DEL-1, JMP-6, LIST-2, CLS-25, and DED-35 for the unsatisfied constraint, which CNS-11 makes a deduction failure.
- For an `explicit` constructor in the copy-initialization of a parameter, its first error is OVL-35. CTOR-9 is the specific rule. `explicit-argument.cpp` cites CTOR-9, holds the same form, and passes with OVL-35 first.
- For a temporary passed to a non-const lvalue reference parameter, its first error is OVL-35. `tidal-next` names REF-9.
- For `t.get<0>()` in a template with no `template`, its first error is TR-12, and its second is DEP-24, the rule of the cause.

`tidal-next run` rejects 5 of the 9 with the same rule as the old front end, or REF-9, and stops as unsupported on 4 (classes and templates).

**My own mistakes from memory.** Five files held seven:
- `strong_first_try.cpp`: a compound requirement with `same_as`, which no header declares here. The old front end names CPT-34 at the compilers' line.
- `strong_first_try.cpp`: an `operator*` constraint that needs its own result through the reversed operator (CNS-35). GCC and Clang reject. The old front end stops first on its TARG-10 fault, and the reduction shows that it accepts the form, as EDG does.
- `flags_first_try.cpp`: operators reached through `using namespace fl;` in the enumeration's namespace. Argument-dependent lookup does not follow a using-directive. The old front end names SA-1 at the compilers' line.
- `flags_second_try.cpp`: a range `for` over a braced list, which needs `std::initializer_list`. The old front end names LSUP-1 for it, as its seventh error, after its own SA-1 fault.
- `anybox_first_try.cpp`: after placement `new` into the buffer, a cast of the buffer's address reads the object. GCC, Clang, EDG, and every sanitizer pass the program. The old front end stops it with STG-23. STG-29 says that the cast reaches only a byte of the buffer. The program has no launder operation, so the second try keeps the pointer that placement `new` returns.
- `guard_second_try.cpp`: a wrong expected log. Every tool exits 23.
- `one-line/implicit_move_rvalue_reference_return_first_try.cpp`: `pass(5)` returns a reference to a temporary that ends with the full expression. Both sanitizer builds and the old front end stop it (OBJ-2). `tidal-next` runs it to 0.

Two of my seven mistakes sat behind a fault of the old front end. One of them it does not catch at all. Round 14 had one of seven.

### 1.7 The phase 17 reports, from a programmer's view

**Front-end report 28.** The round took every library target of round 14 and left no program test that stops in a run. This round's faults sit in name lookup and deduction for operator-heavy library code: flag operators, member arrays in `const` member functions, and skills built from member templates. A programmer meets them in the first try of ordinary code.

**`report-rw3.md` and the rewrite's lines.** The shadow line counts step 3 plain tests, which use no class, no template, and no lambda. Library programs use all three on their first lines. So the shadow line and the takeover list say nothing about programs, and no program test is owned. `tidal-next run` on programs shows two problems that the shadow line cannot see:
- It rejects 21 valid program tests and 25 valid files of this round where a construct of a later step is involved. Decision 0060 says that the rewrite keeps the `unsupported` result for constructs that it does not implement. `tidal-next conform` gives 113 FALSE-REJECT over all tests, and `report-rw3.md` counts 1 false reject in the step 3 plain scope.
- It misses undefined behavior that the old front end catches in the step 3 scope: a reference to an argument's temporary, unsequenced increments, and an enumeration out of range. The takeover list leaves out the tests of these forms, so no status is wrong. A programmer who runs `tidal-next` on a program gets result 0.

**`DESIGN-rewrite.md`.** Section 5 and the build table give the 38 marks of round 28 (17, 15, 6), as the marks file does. Section 10 lists the oracle points of the step 2 parser. The class template's own name in its first base clause (RW2-2) is not among them, and no ledger row names it, although `report-rw2.md` says that the point goes to section 10.

**X26.** P26 would reject a comma whose left operand is a name or a literal. By my reading, the comma operators in my five programs have an assignment or a call on the left, so none changes. X26-1 matters to scope guards. UNEV-P2 and UNEV-2 say that a discarded class temporary is destroyed at once, and COMMA-1 keeps it to the end of the full expression. A guard written as a discarded temporary depends on the answer.

**The RV10 records.** RV11-R-7 says that the braced arm of `template_arg` also derives a braced list for a scalar parameter, which TARG-16 does not name. RV11-R-2 says that the casts paragraph on a class template with no argument list leaves out alias templates. My programs use neither form, so no verdict here rests on them.

### 1.8 Ledger rows

The status page names 13 open exercise rows (X26-1 to X26-6, RV11-R-1 to RV11-R-3, RV11-R-5, RV11-R-7, RV11-R-8, and F7-2) and 2 open front-end rows (G182 and G183). `ledger_counts.py` gives the same rows. No row is `planned`. G183 (LIST-4 with a conversion function) decides the field count of `r14-fields.cpp`, and G182 decides whether `r10-variant.cpp` parses.

## 2. Ranked work items

| # | Item | Evidence | Size | Inputs |
|---|---|---|---|---|
| 1 | Tests for this round's reductions, probes, and clean tries | 10 `tidal-fe` reductions, 10 `tidal-next` reductions, 3 clean first tries and 4 clean intermediate tries that meet a fault | small | `reductions/`, `probes/`, `new/` |
| 2 | The rewrite reports `unsupported`, never a rejection or `undefined`, where a later step's construct is involved; with a line for program tests | 21 program tests rejected, 1 stopped as `undefined`, 1 of 178 run; 113 FALSE-REJECT over all tests; 8 reductions | medium | decision 0060, `run-programs.txt`, `conform-next.txt` |
| 3 | Front-end round 29 on the old code, with library-code targets first | 3 false rejects, 1 missed rejection, 3 stops; the 7 RV10 tests that fail; 17 tests with no row | large | item 1 |
| 4 | The rest of build step 3, with the missed undefined behavior first, then build step 4 | 122 missing reject checks; 3 missed stops in step 3; 58 program tests stop first on a class definition and 66 on a template | large, over several phases | `report-rw3.md` section 2, `DESIGN-rewrite.md` |
| 5 | Answer the 15 open rows, G183 and X26-1 first | G182, G183, X26-1 to X26-6, RV11-R-1 to RV11-R-3, RV11-R-5, RV11-R-7, RV11-R-8, F7-2 | medium | the X26 report, the RV10 read, report 28 |
| 6 | First errors that name the cause | 3 of 9 planted mistakes; `explicit-argument.cpp` passes with OVL-35 first | small | `results-new.txt` |
| 7 | Program first tries and their mistakes in the completion count | 3 of 5 first clean tries meet a fault; 2 of 7 mistakes hidden | small | `summarize.py` |
| 8 | A gap review of the phase 18 text, with a read of its repair diff | the text that items 5 and 6 write | one review session, then repairs | none |
| 9 | Port suite P5 on generic code and operator-heavy libraries | 5 of this round's 7 `tidal-fe` faults come from templates, operators, and lookup in library code | one session | items 1 and 3 |

### Item 1. Tests for this round's reductions, probes, and clean tries

**What.** A clean-room writer adds a test for each of these, probes it on the matrix, and writes the expectation that the compilers and the text give:
- run tests that expect result 0: an operator on a scoped enumeration found by argument-dependent lookup outside its namespace, directly and through a using-declaration, also in a requires expression (NAM-27, NAM-38, OVL-7, CPT-26); an array member passed to a function template in a `const` member function, to two parameters and to one (PRIM-12, DED-6); a member template of a class template specialization as a template template argument (TARG-10, TARG-18); a constructor template whose parameter is a reference to an array of deduced bound, called with a braced list of braced pairs (DED-17),
- a reject test: the recursive constraint through hidden friend templates of a CRTP base (CNS-35), with EDG's acceptance as an override and a question if the matrix splits,
- run tests for the rewrite's verdicts, each of which runs on the old front end: a class qualifier through an alias, a using-declaration of a class, a subscript with two arguments on a class, an operator function on a scoped enumeration, a forward class declaration, a `const` member function defined outside its class, and the type of a pointer to a `const` member function,
- a run test: the byte copy of a `float` into an `unsigned`, so that the byte view has a plain test,
- a parse test: a class template whose base clause names the class template with arguments at its first declaration,
- program tests: `intervals_first_try.cpp`, `guard_first_try.cpp`, `strong_second_try.cpp`, and `flags_third_try.cpp` as clean first tries, the clean intermediate tries `strong_third_try.cpp` to `strong_fifth_try.cpp` and `intervals_second_try.cpp`, and the five last versions as `r15-` program tests. `anybox_first_try.cpp` is a run-fail test of STG-29 and STG-23 that only the interpreter can check.

**Why.** Sections 1.4 and 1.5. No test holds five of the seven `tidal-fe` forms or the seven rejection forms of `tidal-next`. The front-end harness sees only tests.

**Size.** Small. Each reduction has at most 15 lines.

**Inputs.** `programmer-sketches/reductions/`, `probes/`, `new/`.

### Item 2. The rewrite reports `unsupported` where a later step's construct is involved

**What.** The rewrite's checks of step 3 run on programs that hold classes, templates, and operator functions. Where a check depends on such a construct, the result is `unsupported`. Three changes go with it:
- **The rejections.** NAM-14, NAM-16, COMMA-6, BIT-3, DCL-21, and the TR-12 of "declares nothing" give `unsupported` when the name or the operand involves a declaration that step 3 does not model. A run that met an `unsupported` construct before an error gives `unsupported`.
- **The byte view.** A pointer from `reinterpret_cast` to `unsigned char*` stops as unsupported, until the machine has the byte view.
- **A line for program tests.** `frontend_check.py --next` gives `tidal-next run` on the program tests apart, by kind: result, unsupported, rejected, undefined. Today the line would read 1 result, 155 unsupported, 21 rejected, 1 undefined. The same line counts FALSE-REJECT over all tests, 113 today.

`DESIGN-rewrite.md` section 10 gains RW2-2 as an oracle point, with a ledger row.

**Why.** Sections 1.5 and 1.7. Decision 0060 keeps the `unsupported` result for constructs that the rewrite does not implement. A programmer reads a rejection as a statement about the program. Today the rewrite tells a programmer that `r14-keys.cpp` names no class and that `r4-bits.cpp` has undefined behavior. The program line is round 13's and round 14's process change 2, and the rewrite now runs programs.

**Size.** Medium. The eight reductions have two to eight lines each.

**Inputs.** Decision 0060, `run-programs.txt`, `conform-next.txt`, item 1's tests.

### Item 3. Front-end round 29 on the old code, with library-code targets first

**What.** Round 29 works on the old front end and takes the targets in this order:
1. The false rejects: argument-dependent lookup for an operator on a scoped enumeration (NAM-27, OVL-7), the array member in a `const` member function (PRIM-12, DED-6), and the member template of a specialization as a template template argument (TARG-10).
2. The missed rejection: the recursive constraint (CNS-35).
3. The stops: the constructor template from a braced list (DED-17), the template template pack (PACK-5, with `expansion-elements.cpp`), and the base named with `::template` (DEP-25, with `template-keyword-base-last-name.cpp`).
4. The 7 RV10 tests that the old front end does not pass, and rows for the 17 tests that `conformance.tsv` lacks.
5. The 32 tests toward the gate and the 19 run stops.

The INST-5 form stays with the rewrite, because its cause is marked (decision 0064).

**Why.** Sections 1.4 and 1.5. The rewrite reaches program tests at step 8, so until then the old front end checks library code. The first three targets are what a programmer writes for flag enumerations, small containers, and policy classes.

**Size.** Large.

**Inputs.** Item 1's tests.

### Item 4. The rest of build step 3, with the missed undefined behavior first, then build step 4

**What.** The phase decision finishes build step 3 and starts step 4 of `DESIGN-rewrite.md`. For a programmer, the order inside step 3 is:
1. The missed stops in the step 3 scope: a reference to an argument's temporary (REF-5, VC-9, OBJ-2), unsequenced writes (SEQ-4), and an enumeration value out of range (ENUM-6). Each has a test that the old front end passes.
2. The 122 missing reject checks of the plain scope.
3. Lambdas and structured bindings, which `report-rw3.md` counts first among the unsupported constructs (67 and 13 tests).
4. Linking of several units, which 45 tests need.

Then step 4 takes the classes, overloading, and exceptions pages.

**Why.** Section 1.1. `tidal-next run` stops first on a class definition in 58 program tests and on a template declaration in 66 (`run-programs.txt`). Library code needs step 4 before step 5. The missed stops come first, because a program that the rewrite runs to 0 must not hide undefined behavior that the old front end catches.

**Size.** Large, over several phases.

**Inputs.** `report-rw3.md` section 2, `DESIGN-rewrite.md`, item 2.

### Item 5. Answer the 15 open rows, G183 and X26-1 first

**What.** For a programmer, these come first:
- G183: LIST-4 says whether a value of another class that converts to a member's class initializes that member whole. `r14-fields.cpp` counts fields this way.
- X26-1: UNEV-2 and COMMA-1 agree on when a discarded class temporary ends. Scope guards depend on it.
- G182: `union_def` takes a template-id, so that `r10-variant.cpp` parses.
- RV11-R-7 and RV11-R-2: TARG-16 for a braced list as the argument of a scalar parameter, and the alias template in the casts paragraph.

Then the other X26 rows, the RV11-R rows, and F7-2.

**Why.** Sections 1.7 and 1.8. G183 and G182 decide program tests, and X26-1 decides a common library pattern.

**Size.** Medium.

**Inputs.** The X26 report, the RV10 read, report 28, `report-rw2.md`.

### Item 6. First errors that name the cause

**What.** A writer reads the passing reject tests whose first error on the old front end is a general rule (TR-12, OVL-35) while the test cites a specific one, and the front end reports the specific rule first for those forms. The forms of this round: CTOR-9 for an `explicit` constructor in the copy-initialization of a parameter (`explicit-argument.cpp`), REF-9 for a temporary passed to a non-const lvalue reference, and DEP-24 for a missing `template`.

**Why.** Section 1.6. A programmer reads the first error. OVL-35 says that no function is viable and does not say why. Decisions 0063 to 0065 deferred the item.

**Size.** Small.

**Inputs.** `results-new.txt`, round 13's `grammar_first_errors.py`.

### Item 7. Program first tries and their mistakes in the completion count

**What.** A phase close also gives how many clean tries of the roadmap and port programs run on the old front end and on the rewrite. In this round, 3 of 5 first clean tries meet a fault of the old front end, and 2 of 7 mistakes sat behind a fault.

**Why.** Section 1.4. Every program test runs on the old front end, and the five new programs still met seven faults in one session. The completion line counts tests only.

**Size.** Small.

**Inputs.** `results-new.txt`, `summarize.py`.

### Item 8. A gap review of the phase 18 text, with a read of its repair diff

**What.** A source-reader review of the text that items 5 and 6 write, a recorded read of its repair diff, then repairs. The doubts of section 6 join it.

**Why.** The close of phase 17 says that the RV10 read covers the rules that changed after the review read the tree, so no older text waits. A programmer ranks the review after items 1 to 7, because this round's findings rest on rules that did not change.

**Size.** One review session, then repairs.

**Inputs.** None.

### Item 9. Port suite P5 on generic code and operator-heavy libraries

**What.** A port suite with a persona from a language with traits or type classes and operator overloading, such as Rust or Haskell. Ports: a flag set over an enumeration, a numeric newtype with derived operations, and a type-erased container.

**Why.** Five of this round's seven `tidal-fe` faults come from those forms: operators found by argument-dependent lookup, skills built from member templates and template template parameters, and deduction in containers.

**Size.** One session.

**Inputs.** Items 1 and 3, so that P5 measures the repaired front end.

## 3. Out of scope

| Topic | Proposal | For | Against |
|---|---|---|---|
| The general library, including `launder`, `initializer_list`, the comparison categories, and `same_as` | Keep out. Programs declare their own, as the five new programs do. | My first tries reached for `same_as`, a braced range, and a cast after placement `new` (general knowledge). | The charter leaves out the library. STG-29 already tells a program without a header to keep the pointer that placement `new` returns. |
| `operator<=>` in programs with no header | Keep out. | `strong.cpp` writes four comparison operators by hand (general knowledge). | Each use needs the library's comparison category types. |
| Rewrite steps 5 to 9 in phase 18 | Keep for later phases. | The 38 marked tests wait for them. | Steps 3 and 4 come first, and item 3 keeps the old front end useful for programs until step 8. |
| Guides on mixins, type erasure, and flag enumerations | Keep deferred until the maintainer decides on guides. | Two of my seven mistakes were lookup rules that a guide would state. | No decision on guides exists. |
| `typeid` | Keep deferred. | `anybox.cpp` uses the address of a variable template as a type's identity. | Each test needs the library's `type_info` (Q-0322). |
| Threads, the memory model, coroutines, modules, extended floating-point types | Keep out. | | as the charter says |
| Language versions after C++23 | Keep out until `pending.md` row 7 is decided. | | Strict mode rejects them. |

## 4. Process changes

1. **A roadmap round adds its own reductions as tests in the phase that follows it**, as T2 to T6 did. Item 1 keeps that practice.
2. **The rewrite's `next:` lines give program tests apart** (item 2). Rounds 13 and 14 asked for this. The rewrite now runs programs, so the line can count results, not only parses.
3. **The shadow line counts false rejects over all tests**, beside the step 3 plain count. A wrong verdict outside the plain scope is a fault of the rewrite, even where it owns no test.
4. **A finding that a front-end report sends to a design document gets its record in the same commit**, with a ledger row. RW2-2 has neither.
5. **The phase close gives a count of clean library tries beside the completion line** (item 7). Decisions 0062 to 0065 did not take this change.

## 5. Suggested order

1. Item 1's tests come first. They are the first commit of items 2, 3, and 4.
2. Item 2 runs before any other rewrite work, because each later step adds constructs that today's checks reject.
3. Item 3 and item 4 run side by side. Item 3 takes the false rejects first. Item 4 takes the missed stops first.
4. Item 5 runs beside items 3 and 4. Item 6 runs beside item 3, because both change the old front end's diagnostics.
5. Item 7 runs at the phase close. Item 8 follows items 5 and 6. Item 9 follows item 3.

## 6. Doubts for the clean-room writers

- Whether TARG-10 should name a template template argument among the places outside its rule. Its first sentence covers only places that need a type or a value, and the old front end applies it to `Use<Wrap<char>::skill>`.
- Whether CNS-35 reaches a requires expression in the constraint of a hidden friend template of a CRTP base, where the reversed operator needs the same check for `N = M`. GCC and Clang reject the program, EDG accepts it, and the test of CNS-35 holds only a concept form.
- Whether the rewrite's text on unsupported results should say what a run reports when an error depends on an entity that the front end does not model. Decision 0060 says that the rewrite keeps the `unsupported` result, and `DESIGN-rewrite.md` does not say which result wins when both appear.
- Whether LIST-4 counts a value of another class with a conversion function as a value that initializes the member whole (G183). The answer decides the field count of `r14-fields.cpp`.

## 7. Host-only claims

Decision 0061 asks a reader report to list these, so that a writer probes them on the matrix first:
- Every reduction and probe result for GCC and Clang comes from host GCC 16.2.1 and Clang 22.1.8. EDG 7.0 ran in its container. GCC 14, GCC 16.2.0, and Clang 18 did not run.
- The valid reductions and the clean tries of section 1.4 build and exit 0 on the host GCC and Clang.
- GCC and Clang reject `recursive_constraint_hidden_friend_mixin.cpp` with a message that the satisfaction depends on itself, and EDG accepts it.
- The sanitizer results of section 1.6 come from the host compilers. Only the host Clang's UBSan reports the enumeration value out of range. No host sanitizer reports the union read or the unsequenced increments, and both host compilers warn on the increments and on the function that flows off its end.
- With `-fsanitize=undefined`, the host GCC rejects `static_assert(&tag<int> != &tag<long>)` for a variable template `tag`, as not a constant expression. The host Clang accepts it, and so does the host GCC without the sanitizer (`probes/variable_template_address_compare_constant.cpp`). `anybox.cpp` fails to build in the GCC sanitizer run for this reason.
- GCC's warnings on `guard.cpp` (the name `log` in the first try, and a polymorphic type caught by value in the last version) come from the host compiler with `-Wall -Wextra`.
