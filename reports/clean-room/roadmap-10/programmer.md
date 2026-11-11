# Roadmap, round 10: what to do after phase 12, from a programmer's view

Date: 2026-11-10. Decision 0029. This report comes from a clean-room reader in the role of a C++ programmer who uses the specification to write and check programs.

**What the reader read.**
- AGENTS.md, CLAUDE.md, and the round 9 programmer report (`reports/clean-room/roadmap-9/programmer.md`) with its sketches and logs.
- Decisions 0059 and 0060, the status page, the roadmap's phase 12 table, the rows and the decision packet of `documentation/decisions/pending.md`, and the suite rows of `documentation/exercises.md` section 8.
- `git log a15a877d..ebc109b9` with every commit body (20 commits).
- The W20 report with its four reductions, the I5 report with its three reductions, `reports/clean-room/ledger.md`, front-end report 23, `rust-frontend/ledger.md`, and `conformance.tsv`.
- The rules that the findings name: PACK-13, PACK-14, CTOR-8, CTOR-15, DEP-2, DEP-15, DEP-24, DEP-39, TSP-14 to TSP-16, STG-9, UNI-6, UNI-9, UNI-14, NEW-5, VIRT-12, CLS-9, DTOR-6, ACC-15, and the rule list of the class template argument deduction page.
- The tests `template-keyword-forms.cpp`, `anonymous-union-nested-type.cpp`, `decltype/destructor-call.cpp`, `chained-fold-not-chain.cpp`, `pure-call-in-ctor-sanitizer.cpp`, `delete-array-through-base-sanitizer.cpp`, the pack initializer tests under `templates/packs/`, and Q-0089.

The reader did not read the other round 10 reports or their scratch directories, `frontend/` sources, `inputs/`, or `build/exercise-keys/`. Commit read: `ebc109b9`.

**What the reader did.** An earlier session in this role built the front end of round 23 and did the program work. `git diff 1017b19c ebc109b9 -- frontend/` is empty, so that build is the front end at `ebc109b9`. That session reran the 165 round 8 and round 9 files, and wrote five new programs (855 lines) with 14 earlier tries, 28 short programs, 8 planted bugs, 9 planted mistakes, and 13 reductions. This session read those logs, built the front end again from `ebc109b9` into its scratchpad with cargo, got the same result on each of the 13 reductions, and ran 10 probes. Each file went through these checks:
- host GCC 16.2.1 and Clang 22.1.8 with `-std=c++23 -pedantic-errors -Wall -Wextra`,
- the same with `-std=c++23 -pedantic-errors` alone,
- the same with `-fsanitize=undefined,address`,
- the EDG 7.0 container (`cpfe --c++23 --strict --no_code_gen --instantiate=used`),
- `tidal-fe run`.

Everything is in `programmer-sketches/` beside this report:
- `round8/` and `round9/` hold the copies that were rerun, and `results-rerun.txt` is their log (`rerun9.sh`).
- `new/` holds the new programs, their earlier tries, `one-line/`, `bugs/`, and `mistakes/`. `reductions/` holds the reductions. `results-new.txt` is their log (`runnew.sh`).
- `probes/` holds this session's probes, and `results-probes.txt` is their log (`runprobes.sh`).
- `runall.sh` runs one program on every tool. Its `OUT` line names the earlier session's scratchpad, so a rerun needs a new `OUT`.
- `counts.py` gives the counts of section 1.1 from `git show` on both commits. `compare_rerun.py` compares the reruns with round 9's logs. `summarize.py` counts the front-end results of a log.

Statements that rest on general knowledge say so.

## 1. The state from a programmer's view

Front-end round 23 fixed every round 9 finding but one. All 17 round 9 reductions run to result 0. The two wrong verdicts on string literals are gone. All five round 9 programs run in their last version, three first tries now run, and the two first tries that held a mistake are now rejected with the rule of the mistake (INST-6, NEW-37). Every program test runs, which is the second condition of decision 0060.

The new programs are again library code: an open-addressing hash map, a doubly linked list with const iterators, owning pointers with a shared control block, a tuple with indexed leaves and the structured binding protocol, and a variant over a recursive union.
- None runs on the front end on the first try, as in round 9. Three of the first tries build and exit 0 on GCC and Clang (the tuple's with two Clang warnings). The other two hold my own mistakes.
- After workarounds, all five run. Round 9 ran one of five.
- The workarounds changed the design of three programs. The owning pointers lost their friend declarations and made their members public. The tuple left indexed leaves for recursive storage, and left class template argument deduction for `make_tuple`. The hash map left `new[]` for raw storage.

The 13 reductions show front-end faults that no earlier report names: 3 wrong verdicts on valid programs, 7 false rejects, and 2 stops. In the 13th, the front end agrees with GCC and Clang and the text says nothing of the form. Round 23 fixed none of the 13, and neither W20 nor I5 names one. A probe adds one missed reject.

Wrong verdicts are back. Round 23 removed the PRIM-9 wrong verdicts, and two new faults give three new ones. One drops the initializers of a pack of bases, and the other reports NEW-5 for a `delete[]` after a placement `new` into an element. `conformance.tsv` shows 0 wrong run results because no test has these forms.

The front end still runs every short program and catches planted errors well. It runs 28 of 28 short programs, stops 7 of 8 planted bugs with `undefined` and a rule, and rejects 9 of 9 planted mistakes with the rule of the cause.

### 1.1 Counts

| Measure | Round 9 (`a15a877d`) | Now (`ebc109b9`) | Source |
|---|---|---|---|
| Rule headers | 1697 (1682 in force, 15 withdrawn) | 1698 (1683 in force, 15 withdrawn) | `counts.py` |
| Example blocks | 220: 65 unmarked, 28 `fragment`, 112 `reject`, 15 `diverges` | 222: 67 unmarked, the rest the same | `counts.py` |
| Tests | 6711 | 6790 | `counts.py`, files with `tidal-rules`, `.tu.cpp` left out |
| Program tests | 103 | 115 (5 `r9-`, 3 P1, 4 W20 added) | `counts.py` |
| Lines under `tests/programs/` | 12187 | 13615 | `counts.py` |
| Rules cited by a program test | 540 of 1682 | 551 of 1683 | `counts.py` |
| Tests that open `namespace std` | 241, 48 of them program tests | 255, 56 of them program tests | `counts.py` |
| Front end | 6478 of 6711 pass, 6520 their check | 6586 of 6790 pass, 6626 their check; every program test runs | close commits `a15a877d`, `ebc109b9` |
| `conformance.tsv` | 6681 rows | 6786 rows: 6284 pass, 274 EDG-ONLY, 14 pass-partial, 10 EDG-ALLOWED, 100 MISSED-REJECT, 58 unsupported, 40 run-unsupported, 4 FALSE-REJECT, 2 EDG-AGREES | the file |
| Changes under `frontend/` | none | round 23: 14 files, 900 lines added, 352 removed | `git diff --stat` |
| Port suites done | 1 of 8 (P1) | 1 of 8 | `exercises.md` section 8 |
| Write suites in the phase | none | W20 | `exercises.md` section 8 |
| Questions | 418: 174 open | 422: 176 open, 233 settled, 13 compiler defects | `status.md` |
| Front-end ledger, open rows | 0 | 5 (G153 to G157) of 157 | `rust-frontend/ledger.md` |
| Exercise ledger | 867 rows: 762 commit, 68 open, 26 planned | 925 rows: 878 commit, 17 open, 0 planned, 17 text, 10 question, 3 out of scope | `ledger.md` |

### 1.2 Round 9 items that phase 12 took and left

| Round 9 item | Phase 12 | Result |
|---|---|---|
| 1. Front-end round 23 with library-code targets | item FE | Done for the two wrong verdicts, the false rejects, INST-6, and I4-4 to I4-7. Not done: a range `for` by value over move-only elements still stops as "unsupported", and a stop can still name line 0. |
| 2. Tests for every front-end target that has none | item T | Done: 25 tests and 8 program tests. The range `for` over move-only elements got no test. |
| 3. Answer the 21 planned rows whose item ended | item F2, with RD | Done. Every row names a commit. P1-3, P1-5, P1-7, and P1-8 gave the examples and the flags I asked for. |
| 4. Repair the ledger | items L2 and R6 | Done, with a check on `planned` rows. |
| 5. Write suite W20, then Port P2 | item E | W20 done. P2 deferred. |
| 6. Interpreter output when every order agrees | none | Deferred by decision 0059. |
| 7. Diagnose batches | none | Deferred by decision 0059. |
| 8. A count of grammar forms by test | none | Deferred by decision 0059. |
| 9. Guides | none | Deferred by decision 0059. |

Round 9 process changes 1 (a front-end finding gets a test in the commit that records it) and 3 (Port programs become program tests) are choices of decision 0059. Change 2 (the ledger check) is in `13c03237`. Change 4 (a planted mistake that the front end runs becomes a reject test) was done for the data member of function type. Change 5 (grammar forms) is deferred.

Change 1 did not reach the two exercise commits of phase 12. W20 and I5 record four front-end faults with reductions (W20-2, W20-3, I5-9, I5-10), and no test has their forms (`grep` for W20 and I5 in `tests/` finds only the four W20 program tests). The front end at `ebc109b9` gives the same result on each of the four reductions as the reports give.

### 1.3 The round 8 and round 9 sketches, rerun

`compare_rerun.py` reads the 165 files of `results-rerun.txt`: 120 give the same front-end result as in round 9, and 45 changed. Every change moves toward the compilers' answer:
- All 17 round 9 reductions run to result 0 (one prints "unspecified: result 0"). Sixteen of them changed. The 17th, `cond_class_prvalue_vs_convertible_class.cpp`, ran in round 9.
- The last versions of `function.cpp`, `map.cpp`, `optional.cpp`, and `text.cpp` run. `vec.cpp` ran in round 9.
- `vec_first_try.cpp`, `text_first_try.cpp`, and `map_first_try.cpp` run. `function_first_try.cpp` is rejected with INST-6 and `optional_first_try.cpp` with NEW-37, which are the mistakes the round 9 report named.
- The round 8 pack forms run: the three parenthesized pack variables, the member and base initializers with a pack, `tasks.cpp`, and `tasks_type_pack_bases.cpp`.
- The planted mistake with a data member of function type is rejected with INST-6.

The compilers and EDG give the same result on every file as in round 9, apart from the exit codes of planted bugs with undefined behavior.

Three files still stop or reject where round 9 asked for a change:
- `round9/new/mistakes/copy_in_range_for_move_only.cpp` still gives "unsupported: range-based for". Every compiler rejects it with a deleted copy constructor.
- `round8/pipeline_first_try.cpp` gives TR-12. GCC and EDG reject it, and Clang accepts it, as in round 8.
- The three `consteval` rejects of round 8 (CEV-1) are correct rejects, as before.

### 1.4 New programs

| Program | GCC, Clang (three flag sets) | EDG 7.0 | `tidal-fe run`, first try | `tidal-fe run`, last version |
|---|---|---|---|---|
| `hash_map.cpp` (165 lines): open addressing with linear probing and tombstones, a `Hash<T>` primary with specializations, a lookup by string literal, `try_emplace` that forwards a pack into a slot in a union, rehash, erase, iteration, a move-only value | first try fails on every compiler (section 1.6); second and last exit 0, clean | first try rejects; second and last accept | undefined [NEW-5] on `delete[]`, also on the second try, which builds clean. **Wrong.** | unspecified: result 0; the bound of 64 runs stopped the exploration |
| `list.cpp` (177 lines): a sentinel node, `Iter<Const>` with a conversion from the mutable iterator, insert and erase, splice, reverse, a merge sort that relinks nodes, a variadic constructor with a fold, generic algorithms | exit 0, clean | accepts | rejected (OVL-35) on the conversion to the const iterator through a constrained constructor template. **False reject.** | unspecified: result 0 |
| `smart_ptr.cpp` (189 lines): `Unique<T, D>` with an empty deleter under `[[no_unique_address]]`, a converting move from a derived owner, `Shared<T>` with an in-place control block, `Weak<T>`, an aliasing constructor, `make_shared` that forwards a pack | exit 0, clean | accepts | unsupported: "overloaded operator", after a friend class declaration | unspecified: result 0, with every member public and no friend declaration |
| `tuple.cpp` (164 lines): `Tuple<Ts...>` from indexed leaves, `get<I>` for lvalues and rvalues, the structured binding protocol, an index sequence, `apply`, `make_tuple` with decay, `tuple_cat`, `==` and `<=>` with folds, all in constant evaluation | exit 0; Clang warns twice on the first try | accepts | rejected (DED-35) on `make_tuple`. The cause is a partial specialization that the front end calls ambiguous (TSP-16). **False reject.** | unspecified: result 0, after five changes |
| `variant.cpp` (160 lines): a recursive union, a converting constructor by exact type, `emplace`, copy and move through a table of function pointers, `visit` with an overload set of lambdas, `get_if`, an alternative with a destructor | first try: Clang rejects a comparison fold, as Q-0029 records; second to last exit 0, clean | accepts | rejected (DEP-24) on `v.as<T>()` with `v` of the class's own type. Every compiler accepts the call. | result: 0 |

The tuple needed five changes:
1. A fold in parentheses (the second try), with the same DED-35 result.
2. One array specialization in place of `T[N]` and `const T[N]`. The front end then gave "static_assert condition is not a constant expression: read of an object with no value" (OBJ-2, INIT-1), because it drops the initializers of the pack of leaves. **Wrong.**
3. Recursive storage with a base named `Store<I + 1, Rest...>` in the mem-initializer. The front end gave "unsupported: constructor with an initializer the front end does not model".
4. The base named through an alias. The front end then stopped on `Tuple(1, 2)` (class template argument deduction).
5. `make_tuple(1, 2)` in place of `Tuple(1, 2)`. The program runs.

The variant's third try built each alternative in a member of a union member that was not active. GCC, Clang, and EDG build it, and it exits 0 with the sanitizers. The front end stops it with "undefined [UNI-6]: access to a member of a union member that is not active". In constant evaluation GCC and Clang reject the same form (reduction `construct_in_member_of_inactive_union_member.cpp`). The last version makes each union on the path active before it builds the alternative. The front end found my mistake, as W20's front end found W20-1. UNI-14 does not name the form (section 6).

Each first try and each intermediate version is kept beside the last version (`*_first_try.cpp` to `*_fifth_try.cpp`).

### 1.5 Reductions

Each reduction builds and exits 0 on GCC and Clang under the three flag sets, and EDG accepts it, except where the table says otherwise. A grep of `tests/` for each form finds no test of it. The Status column compares with round 9's reductions and their reruns (section 1.3), and with the W20 and I5 reports.

| Reduction | Lines | `tidal-fe` | Status | What the specification says |
|---|---|---|---|---|
| `base_pack_mem_init_under_derived.cpp`: `Impl(int v) : Leaf<I>(v + I)... {}`, where `Leaf` has a default constructor and `Leaf(int)`, and `Impl<0, 1>` is the base of `Top` | 19 | undefined [OBJ-2, INIT-1]: read of an object with no value. **Wrong.** | New. Round 23 fixed the braced form `leaf<I>{v + I}...` with aggregate leaves, and the probe `base_pack_braced_under_derived.cpp` runs. | PACK-13 and PACK-14 give one initializer for each base. |
| `base_pack_paren_mem_init_with_constructor.cpp`: the same pack in a complete object | 15 | rejected, CTOR-15 ("'Leaf' is initialized two times"). **False reject.** | New. The probe `base_pack_paren_no_default.cpp` (no default constructor) gives the same. A pack of base types, `Bs(v)...`, runs. | PACK-14. CTOR-15 forbids naming one base two times, and `Leaf<0>` and `Leaf<1>` are two bases. |
| `two_specializations_of_one_base_template_mem_init.cpp`: `Pair(int a, int b) : Leaf<0, int>(a), Leaf<1, int>(b) {}` | 13 | rejected, CTOR-15. **False reject.** | New. The braced form gives the same (probe). Bases named through two aliases run. | CTOR-15 as above. The front end seems to compare the template name and drop the argument list. |
| `specialization_base_of_same_template_mem_init.cpp`: `Box<T*> : Box<int>` with `Box(int t) : Box<int>(t) {}` | 11 | rejected, CTOR-8 ("a constructor delegates to itself"). **False reject.** | New | DEP-15: `Box<int>` names another specialization. TARG-11 gives the injected class name only for the template's own arguments. |
| `placement_new_into_new_array_element_then_delete.cpp`: `int* a = new int[2];`, a placement `new` of an `int` at `&a[0]`, then `delete[] a` | 10 | undefined [NEW-5]: "delete[] of an object from new". **Wrong.** | New | STG-9: the new `int` replaces the element, so `a` still points to the array from `new[]`. |
| `placement_new_into_union_member_of_new_array_element.cpp`: the same with a `KV` built in the anonymous union member of a `Slot` element | 14 | undefined [NEW-5]. **Wrong.** | New | UNI-14: the new object is the member, inside the element. |
| `iterator_to_const_iterator_constructor_template.cpp`: `template <bool C = Const> requires C Iter(const Iter<!Const>&)` and a call `take(m)` with `take(Iter<true>)` | 15 | rejected, OVL-35. **False reject.** | New. Round 23 fixed P1-2 for a converting constructor that is not a template. | The constructor template takes `C` from its default (TPL-13), and its constraint holds for `Iter<true>`. |
| `member_of_other_specialization_by_constant_argument.cpp`: inside `Box<N>`, `int read(const Box<0>& o) const { return o.v; }` | 11 | rejected, NAM-8 ("no member named 'v'"). **False reject.** | New | DEP-15: another argument list names another specialization, which is dependent. DEP-39 then waits for the instantiation. |
| `member_template_through_current_instantiation_object.cpp`: `static int read(const Box& b) { return b.as<int>(); }` inside `Box<T>` | 12 | rejected, DEP-24. GCC, Clang, and EDG accept. | New | DEP-24's bullet for `.` and `->` asks for `template` with a type-dependent object, and DEP-2 makes `b` type-dependent. The `::` bullet and DEP-39 leave out the current instantiation, and the `.` bullet does not. The front end follows the text, and the text differs from every compiler. |
| `partial_spec_array_vs_const_array.cpp`: partial specializations for `T[N]` and `const T[N]`, used with `const char[4]` | 9 | rejected, TSP-16 (ambiguous). **False reject.** | New | TSP-15 and ORD-14: `const T[N]` is more specialized. In `tuple_first_try.cpp` the same fault surfaces as DED-35 on a call to `make_tuple`. |
| `ctad_guide_beside_constrained_constructor_template.cpp`: `Tuple(1, 2)` with a user guide, beside a constructor template whose constraint fails for the implicit guide | 12 | unsupported: "object of unknown type", at line 0 | New. Report 23 left the class template argument deduction tests untargeted. The line 0 is the form round 9 asked to fix. | CTAD-11 and CTAD-29: the guide from the constructor keeps its constraint and is not viable, and the user guide gives `Tuple<int, int>`. |
| `friend_class_then_overloaded_operator.cpp`: a class with `friend class Other;` and `operator*`, `operator+`, and `explicit operator bool` | 19 | unsupported: "overloaded operator" | New. A `friend struct` gives the same (probe). A friend function runs. `r8-owner`, a class template with a friend class template, runs. | ACC-15 and the operator rules; the friend declaration changes nothing in the calls. |
| `construct_in_member_of_inactive_union_member.cpp`: `std::construct_at(&o.in.d, 2.5)` in constant evaluation, where `o.in` is a union member that is not active | 17 | rejected, UNI-6 | New, beside W20-2. GCC and Clang reject the constant; EDG accepts it. The front end agrees with GCC and Clang. | UNI-14 covers an object of the member's own type at the member's address. It says nothing of an object built inside a member that is not active. W20-2 asks the same for an element of an array member. |

By status: 0 fixed by round 23, 0 named by W20 or I5, and 13 new. One of the 13 is a text gap beside W20-2.

The probes add one missed reject. `probes/anonymous_struct_member_in_anonymous_union.cpp` declares `union { struct { int key; int value; } kv; };` in a class. UNI-9 forbids a nested type in an anonymous union. Clang and EDG reject the program, GCC accepts it under `-pedantic-errors`, and the front end runs it. The test `anonymous-union-nested-type.cpp` names its nested type, and the front end passes it. The hash map's first try had this form.

### 1.6 Short programs, planted bugs, and planted mistakes

**Short programs.** The 28 short programs in `new/one-line/` build and run clean on GCC and Clang with every flag set, and EDG accepts all 28. The front end runs all 28 (`summarize.py`: 26 "result: 0" and 2 "unspecified: result 0"). The forms are:
- the swap idiom through argument-dependent lookup, `auto(x)`, and the `uz` suffix,
- unsigned bit-fields, a class type as a template parameter, and a variable template constant,
- `const` and non-`const` subscripts, `operator[]` with two arguments, and a `mutable` cache,
- `dynamic_cast` to a pointer, a virtual base in a diamond, a pure virtual destructor with a definition, and a default argument on a virtual function,
- enumeration flag operators, the bytes of a `float` through `unsigned char`, and small-integer promotion,
- a function-local static counter, a table of function pointers, and `goto` out of nested loops,
- `if consteval`, a lower-bound search, a range `for` with a sentinel, and a rethrow in a handler,
- a builder with `&&`-qualified setters, a scope guard deduced by class template argument deduction, `operator<<` chains, a hash `switch` over strings, user-defined literals for units, and `nullptr` in overload resolution.

Two of the short programs use forms near the 20 rules that changed after RV5: the `&&`-qualified setters (CLS-26) and small-integer promotion (CONV-1). Both run on every tool. None of this report's findings rests on the 20 rules.

**Planted run-time bugs.** The front end stops 7 of the 8 bugs in `new/bugs/` with `undefined` and a rule:

| Rule the front end names | Bug | Sanitizers |
|---|---|---|
| PFX-18 | a string copy that writes the terminator past a `char[5]` member | silent |
| NEW-5 | `delete[]` of an array of `Derived` through `Base*` | silent with these flags |
| UN-7 | the null result of a failed `dynamic_cast` used | UBSan |
| MUL-10 | `INT_MIN / -1` | UBSan |
| OBJ-2 | a list loop that reads `next` from a deleted node | ASan |
| STG-23 | an `int` read through a pointer into a `char` array | silent |
| OBJ-2 | a member read after an explicit destructor call | silent |

The eighth bug, a pure virtual call from a base constructor through a helper, gives "unsupported: function 'run' has no definition in the program". VIRT-12 makes the call undefined. The test `pure-call-in-ctor-sanitizer.cpp` has the same form and is `run-unsupported` in `conformance.tsv`, so the stop is known. The runtime library of both compilers aborts with "pure virtual method called".

For the array deleted through a base, the front end names NEW-5. VIRT-12 is the specific rule, and NEW-5 points to it. Four bugs pass every sanitizer, and the front end stops all four.

**Planted compile-time mistakes.** GCC, Clang, EDG, and the front end reject all 9 mistakes in `new/mistakes/`. In each case the front end's first error names the rule of the cause: VIRT-8 (an abstract class object), CPY-23 (assignment to a class with a `const` member), DEL-1, JMP-6, CLS-1 (a member of incomplete type), NAM-8 (a missing member in an instantiated template), VIRT-3, RET-7, and SA-1.

**My own mistakes from memory.** Three first tries held one each:
- `hash_map_first_try.cpp` used an unnamed struct as the member of an anonymous union (section 1.5, the UNI-9 probe), and `kv.~decltype(kv)()`. GCC rejects the destructor name, and Clang and EDG accept it. Q-0089 holds that split under DTY-3, and DTOR-6 does not point to DTY-3.
- `variant_first_try.cpp` wrote a comparison fold, which Clang rejects by default. Q-0029 and `chained-fold-not-chain.cpp` record that.
- `variant_third_try.cpp` built in a member of a member union that was not active (section 1.4). Only the front end reported it at run time.

### 1.7 The phase 12 reports, from a programmer's view

**Front-end report 23.** The round did what round 9 asked. The literal fix is the one a programmer needs: a pointer into one literal keeps its identity, so view types over literals work. The pack initializers, assignment through a converting constructor, arrays of a class with a constructor template, and the class conditional all run in my round 9 programs. The report's section 6 states the distance to decision 0060's test: 204 failing tests and every program test running.

The completion test reads only tests. My five new programs found 13 faults in one session, and the last versions of programs avoid the faults by construction. So a program test of a last version cannot show the faults that its first try met. G154 shows the same gap from inside the round. The front end accepted a copy through a private copy constructor until a conditional-operator test showed it.

**W20.** W20-1 and W20-6 are what a programmer meets when a container keeps its storage in the object: the specification's pattern gives no pointer and length. My hash map and variant met the neighbours of W20-2: an element of `new[]` storage and a member of a member union. W20-3 is a false reject (CNS-35) with a reduction and no test. W20-5 asks for one sentence in CAST-1, which every type-erased container needs.

**I5.** I5-10 matters most to a programmer. The front end binds a reference to a private or ambiguous base and runs the program to a result. Both compilers reject it. A missed reject tells a programmer that a wrong program is right. I4-6 and I4-7 found the same gap for the conditional operator, and round 23 fixed only that path. I5-4 is the text side of the same case. I5-3 and I5-6 are what a programmer writes every day: `const T&` bound to a function, and forwarding of a `const int` result.

### 1.8 Ledger rows

The status page names 17 open rows: W20-1 to W20-6 and I5-1 to I5-11. The front-end ledger has 5 more open rows, G153 to G157, which the status page does not count. No row is `planned`.

Four of the 22 rows are front-end faults with a reduction and no test: W20-2, W20-3, I5-9, and I5-10. G154 asks for tests of a private copy or move constructor selected by a copy-initialization, a by-value argument, and a return.

## 2. Ranked work items

| # | Item | Evidence | Size | Inputs |
|---|---|---|---|---|
| 1 | Tests for every front-end finding that has none | 13 reductions here, 1 probe, W20-2, W20-3, I5-9, I5-10, the round 9 range `for`, G154 | small | `reductions/`, `probes/`, the W20 and I5 reductions |
| 2 | Front-end round 24, with library-code targets first | 3 wrong verdicts, 8 false rejects, 4 missed rejects (I5-10's two bases, UNI-9, the range `for`), and 5 kinds of stop; 0 of 5 new programs run on the first try | large | item 1; item 4 for DEP-24 |
| 3 | Answer the 22 open ledger rows, starting with the storage rows | W20-1 to W20-6, I5-1 to I5-11, G153 to G157 | medium | the W20 and I5 reports, report 23 |
| 4 | Text from this round's reductions | DEP-24 and every compiler differ; UNI-9 with a GCC split; UNI-14 for a member of a member union; TSP-15 with no array example; DTOR-6 without DTY-3 | small, with probes | `reductions/`, `probes/` |
| 5 | Port suite P2 on generic collections | every round of library programs finds new faults: round 9, W20, and this round | one session | item 2 |
| 6 | Interpreter output when every order agrees | 4 of 5 new programs and all 5 `r9-` programs print "unspecified: result 0" | small | none |
| 7 | Diagnose batches with the front end's message | a partial specialization ambiguity reported as DED-35; a stop at line 0 | one session per 4 pages | none |
| 8 | A gap review of the 20 rules and principles changed after RV5 | close commit `ebc109b9` | one review session, then repairs | none |
| 9 | Guides on containers, owners, and type erasure | deferred since phase 5; the storage rows of W20 and this report's hash map and variant | one decision, then one session per guide | maintainer |

### Item 1. Tests for every front-end finding that has none

**What.** A clean-room writer adds a test for each of these, probes it on the matrix, and writes the expectation that the compilers and the text give:
- run tests that expect result 0: the two NEW-5 reductions (STG-9, UNI-14), the pack of bases under a derived class and in a complete object (PACK-14), the two template-id mem-initializers (CTOR-15) and the specialization base (DEP-15, CTOR-8), the iterator conversion (TPL-13, OVL-35), the member of another specialization (DEP-15, DEP-39), the array partial specializations (TSP-15), the deduction guide beside a constrained constructor template (CTAD-29), and the friend class with operators (ACC-15),
- the member template through an object of the current instantiation, after item 4 decides DEP-24,
- reject tests: the unnamed struct in an anonymous union (UNI-9, with a GCC override), I5-10's two reductions (REF-1, ACC-12, CONV-17), and the range `for` by value over move-only elements (DEL-1),
- W20-3's reduction (CNS-35, CPY-15) and I5-9's reduction (DED-24), with the compilers' answer,
- a constant test for the construction inside a member of a member union, after item 4 or a question records the split (EDG accepts),
- G154's three copy forms (ACC-3).

The integrator copies the five last versions into `tests/programs/roadmap/` as `r10-` program tests. The first tries of `list` and `smart_ptr` and the tuple's second try build clean on GCC and Clang, and they go in as program tests too (section 4, change 3).

**Why.** Section 1.5 and 1.8. The front-end harness sees only tests, so round 24 has no check for any of these faults without them. Decision 0059 chose that a front-end finding gets a test in the commit that records it, and the W20 and I5 commits added none.

**Size.** Small. Each reduction has at most 19 lines.

**Inputs.** `programmer-sketches/reductions/`, `probes/`, `new/`, `reports/clean-room/W20/reductions/`, `reports/clean-room/I5/reductions/`.

### Item 2. Front-end round 24, with library-code targets first

**What.** Round 24 takes the targets in this order:
1. The wrong verdicts. A pack of bases named by a template-id in parentheses keeps its initializers when the class is a base of another class. A placement `new` into an element of an array from `new[]`, or into a member of such an element, leaves the array deletable with `delete[]` (STG-9, UNI-14).
2. The false rejects:
   - CTOR-15 and CTOR-8 on a mem-initializer that names a base by a template-id, when two bases are specializations of one template or the base is another specialization of the class's own template,
   - OVL-35 on a conversion through a constrained constructor template with a default template argument,
   - NAM-8 on a member of another specialization of the enclosing template,
   - TSP-16 on `T[N]` against `const T[N]`,
   - CNS-35 on a constructor template that takes the class by value (W20-3),
   - DEP-24 on a member template through an object of the current instantiation, once item 4 changes the rule.
3. The missed rejects: a reference bound to a private or ambiguous base (I5-10), an unnamed class type in an anonymous union (UNI-9), and a range `for` that copies move-only elements (DEL-1).
4. The stops: an overloaded operator on a class with a friend class declaration, class template argument deduction beside a constrained constructor template, a subscript of an array member of a union (W20-2), `T::type` with an incomplete `T` (I5-9), and a virtual call of a pure virtual function in a run (VIRT-12).
5. The 204 tests of report 23's section 6, by page, toward decision 0060's first condition. The missed rejects come first among them.

A stop at line 0 gives the line of the declaration.

**Why.** Sections 1.4 and 1.5. A programmer trusts "undefined" only when the front end is never wrong on a correct program. The two new faults break that for any tuple built from indexed leaves and for any hash map that builds in place in `new[]` storage (general knowledge: both are common library forms). The false rejects and stops come in forms that iterators, tuples, and owners use. The workarounds cost my programs their friend declarations, their indexed leaves, and their deduction guides.

**Size.** Large.

**Inputs.** Item 1's tests. Item 4 for the DEP-24 target.

### Item 3. Answer the 22 open ledger rows, starting with the storage rows

**What.** For a programmer, these come first:
- W20-1 and W20-6: a short table beside the `FixedVec` and `FixedStack` examples. It gives each storage form and says whether it runs in constant evaluation, gives a contiguous `T*`, and needs a default constructor.
- W20-2 with this report's member of a member union: one rule for objects built inside a union member that is not active, for an element of an array member and for a member of a member union. Where the compilers split, a question.
- W20-5: one sentence in CAST-1 on the `void*` round trip, with a link to STG-16.
- I5-4 and I5-10: DED-15 points to OVL-37 for an ambiguous or private base, and REF-1's reject case gets a test (item 1).
- I5-3 and I5-6: `const` on a function type, and the cv-qualifiers of a non-class prvalue in deduction.
- G154 and G155: tests of a private copy constructor (item 1), and a reject rule for a deleted or inaccessible allocation function.

Then W20-3, W20-4, I5-1, I5-2, I5-5, I5-7, I5-8, I5-11, G153, G156, and G157, each with the probe that its report names.

**Why.** Section 1.8. W20 and this report met the same storage questions from two personas.

**Size.** Medium. The storage rows need probes.

**Inputs.** The W20 and I5 reports, front-end report 23, `probes/`.

### Item 4. Text from this round's reductions

**What.**
- DEP-24: the bullet for `.` and `->` leaves out an object whose type is the current instantiation, when lookup finds the member template in that class. The `::` bullet and DEP-39 already leave it out. GCC, Clang, and EDG accept the reduction.
- DEP-15 and DEP-39: whether an argument list with no template parameter in it, such as `Box<0>` inside `Box<N>`, names a dependent specialization (section 6).
- UNI-9: whether an unnamed class type declared in a member declaration of an anonymous union counts as a nested type. Clang and EDG reject the form, and GCC accepts it under `-pedantic-errors`. A question holds the split.
- UNI-14: the construction inside a member of a member union that is not active (item 3).
- TSP-15: an example with `T[N]` and `const T[N]`, the form of every decay trait (general knowledge).
- DTOR-6: a link to DTY-3 and Q-0089 for `~decltype(x)()`.

**Why.** Section 1.5. In the DEP-24 case the front end follows the text, and the text gives a reject that no compiler gives. The tests of item 1 need these answers for their expectations.

**Size.** Small. Each needs a probe on the matrix.

**Inputs.** `reductions/member_template_through_current_instantiation_object.cpp`, `member_of_other_specialization_by_constant_argument.cpp`, `construct_in_member_of_inactive_union_member.cpp`, `partial_spec_array_vs_const_array.cpp`, `probes/anonymous_struct_member_in_anonymous_union.cpp`, `probes/destructor_name_decltype*.cpp`.

### Item 5. Port suite P2 on generic collections

**What.** P2, with a Java or C# persona, ports four small programs that use generic collections: a hash map with a custom hasher, a linked list with iterators, a tuple or record with structural equality, and an owner with shared references.

**Why.** Each round of library-shaped programs finds front-end faults that the tests miss: 14 in round 9, 2 in W20 (W20-2, W20-3), and 13 reductions in this round. Decisions 0058 and 0059 deferred P2. A port from a managed language writes iterators, `friend` access, and deduction guides as a newcomer writes them (general knowledge), and those forms stopped my programs.

**Size.** One session.

**Inputs.** Item 2, so that P2 measures the new front end.

### Item 6. Interpreter output when every order agrees

**What.** Round 9 item 6, unchanged. When every explored order gives one result, `tidal-fe run` prints the result first, such as "result: 0 (every order of 4 gives 0)". When the bound stops the exploration, the line says so first.

**Why.** Four of the five new programs, all five `r9-` programs, and two short programs print "unspecified: result 0" with no other outcome. A programmer reads "unspecified" as a fault in the program.

**Size.** Small.

**Inputs.** None.

### Item 7. Diagnose batches with the front end's message

**What.** Round 9 item 7, unchanged: batches for templates and dependent names, each scoring whether the `tidal-fe check` message leads to the rule.

**Why.** `tuple_first_try.cpp` reports DED-35 on `make_tuple`, and the cause is a TSP-16 ambiguity two templates away. The class template argument deduction stop names line 0.

**Size.** One session per 4 pages.

**Inputs.** None.

### Item 8. A gap review of the 20 rules and principles changed after RV5

**What.** A source-reader review of CLS-26, BF-6, ALN-4, AUTO-P3, RET-6, REF-1, SINIT-6, UNW-3, LAM-5 to LAM-7, CONV-1, NAM-21, STG-5, STG-P3, TR-P1, ICS-11, JMP-10, SW-P1, and ORD-13, then repairs.

**Why.** The close commit names these 20 for the next review. A programmer ranks the review lower, because no finding of this report rests on them, and the short programs near CLS-26 and CONV-1 run on every tool.

**Size.** One review session, then repairs.

**Inputs.** None.

### Item 9. Guides on containers, owners, and type erasure

**What.** A maintainer decision on publishing guides, then how-to guides on storage inside an object and in `new[]` storage, a variant over a recursive union, and owners with a control block.

**Why.** W20-1, W20-6, and this report's hash map and variant show that the storage rules are spread over the storage, unions, and new-delete pages. My variant's mistake (building inside a member of a member union) is what such a guide would prevent.

**Size.** One decision, then one session per guide.

**Inputs.** The maintainer.

## 3. Out of scope

| Topic | Proposal | For | Against |
|---|---|---|---|
| The general library, including `forward`, `move`, `decay`, `tuple`, `variant`, and the smart pointers | Keep out. Programs define their own, as the five new programs do. | Each program needs them (general knowledge). | The charter leaves out the library. Each took a few lines. |
| The front-end rewrite (FR) before decision 0060's first condition | Keep waiting for the completion test. | Every program test runs. | 204 tests fail, and this round's 13 forms have no test. Item 1 lets the completion test see them. |
| A sanitizer for uninitialized reads, and the type sanitizer in my checks | Keep out of the matrix runs here. | The front end stops 4 bugs that no sanitizer in these flags reports. | Tidal's matrix already runs the type sanitizer on Clang 22. |
| `typeid` | Keep deferred. | | Each test needs the library's `type_info` (Q-0322). |
| Threads, the memory model, coroutines, modules, extended floating-point types | Keep out. | | as the charter says |
| Language versions after C++23 | Keep out until `pending.md` row 7 is decided. | | Strict mode rejects them. |

## 4. Process changes

1. **The commit that saves an exercise report adds a test for each front-end finding.** Decision 0059 chose this, and the W20 and I5 commits added none. `check_specification.py` could fail on a ledger row of kind "front end" whose Record names no test.
2. **The status page counts the open rows of both ledgers.** It names the 17 open W20 and I5 rows and leaves out G153 to G157.
3. **A roadmap program's first try becomes a program test when it builds clean on GCC and Clang.** A last version avoids each fault by construction. The first tries carry the forms that a programmer writes first.
4. **A probe that the front end accepts against two compilers becomes a reject test,** as round 9 change 4 asked for planted mistakes. The UNI-9 probe is the case here.

## 5. Suggested order

1. Item 4 comes first, because it is small and the tests of item 1 need its DEP-24 and UNI-9 answers.
2. Item 1's tests are the first commit of item 2. Round 24 then takes the wrong verdicts first.
3. Item 3 runs beside item 2. Its storage rows and item 4's UNI-14 point share probes.
4. Item 5 follows item 2.
5. Items 6, 7, and 8 run beside the others at any point. Item 9 waits on the maintainer.

## 6. Doubts for the clean-room writers

- Whether DEP-24's bullet for `.` and `->` should leave out an object of the current instantiation, as its `::` bullet and DEP-39 do. Every compiler accepts the reduction without `template`.
- Whether `Box<0>` inside `Box<N>` names a dependent specialization. DEP-15 says any other argument list is dependent, and the list has no template parameter in it.
- Whether a construction inside a member of a union member that is not active makes the enclosing member active (UNI-14). GCC and Clang reject the form in constant evaluation, and EDG accepts it. The front end calls the run undefined.
- Whether a placement `new` into an element of an array from `new[]`, or into a member of such an element, leaves `delete[]` valid. STG-9 and UNI-14 read that way to me, and the front end reports NEW-5.
- Whether UNI-9's "nested types" covers an unnamed class type in a member declaration. GCC accepts the form under `-pedantic-errors`.
- Whether a range `for` by value over move-only elements belongs in a RFOR rule or only under DEL-1 (round 9, still open). The front end still stops on it.
- Whether "unspecified: result 0" should stay the first word when every explored order gives the same result (rounds 7 to 9, still open).
