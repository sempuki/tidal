# Roadmap, round 11: what to do after phase 13, from a programmer's view

Date: 2026-11-11. Decision 0029. This report comes from a clean-room reader in the role of a C++ programmer who uses the specification to write and check programs.

**What the reader read.**
- AGENTS.md, CLAUDE.md, and the round 10 programmer report (`reports/clean-room/roadmap-10/programmer.md`) with its sketches and logs.
- The status page, the roadmap's phase 13 table, decisions 0060 and 0061, and `documentation/decisions/pending.md`.
- `git log c67b349b..f940ea53` with every commit body (18 commits: the 17 of phase 13 and the corrections commit `f940ea53`).
- The I6 report, the P2 report, front-end report 24, the two RV6 gap reviews (`reports/review/2026-11-10-gap-review-phase-13-*.md`), `reports/clean-room/ledger.md`, `rust-frontend/ledger.md`, and `conformance.tsv`.
- The rules that the findings name: ADD-6, ADD-10, ADD-11, ADD-14, STG-P2, STG-P3, STG-17, STG-28, STG-29, NEW-20, NEW-33, NEW-34, LSUP-1, LSUP-2, LSUP-10, LSUP-12, CTAD-1, DED-34, LAM-15, TPL-11, TARG-12, CLS-13, CLS-14, and the tests `member-template-through-current-instantiation-object.cpp`, `constant-parameter-types.cpp`, and `explicit-private-member-argument.cpp`.

The reader did not read the other round 11 reports or their scratch directories, `frontend/` sources, `inputs/`, or `build/exercise-keys/`. Commit read: `f940ea53`. `git diff 3bd790ae f940ea53` touches only `pending.md`, `status.md`, and the front-end ledger, so the tests, the text, and the front end are those of the phase 13 close.

**What the reader did.** The reader built the front end from `f940ea53` into its scratchpad with cargo, with no build failure. The reader reran the 87 round 10 files and wrote five new programs (947 lines) with 19 earlier tries, 24 short programs, 8 planted bugs, 9 planted mistakes, 11 reductions, and 22 probes. Each file went through these checks:
- host GCC 16.2.1 and Clang 22.1.8 with `-std=c++23 -pedantic-errors -Wall -Wextra`,
- the same with `-std=c++23 -pedantic-errors` alone,
- the same with `-fsanitize=undefined,address`,
- the EDG 7.0 container (`cpfe --c++23 --strict --no_code_gen --instantiate=used`),
- `tidal-fe run`.

Everything is in `programmer-sketches/` beside this report:
- `round10/` holds the copies that were rerun, and `results-rerun.txt` is their log (`rerun10.sh`). `compare_rerun.py` compares the log with round 10's logs.
- `new/` holds the new programs, their earlier tries, `one-line/`, `bugs/`, and `mistakes/`. `reductions/` and `probes/` hold the reductions and probes. `results-new.txt` is their log (`runnew.sh`). `results-late.txt` holds the template lambda reduction and its two probes, which came after that run.
- `runall.sh` runs one program on every tool. `reduce.py` is the line-based reducer that found the first cut of two reductions. `work/` holds the intermediate cuts.
- `counts.py` gives the counts of section 1.1 from `git show`. `ledger_counts.py` counts both ledgers by state. `scan_tests.py` scans `tests/` for the forms of the reductions. `summarize.py` counts the front-end results of a log.

Statements that rest on general knowledge say so. Section 7 lists the claims that rest on the host compilers alone.

## 1. The state from a programmer's view

Front-end round 24 fixed 10 of the 12 front-end faults in the round 10 reductions. All five round 10 programs still run in their last version. Four more earlier tries run, and the hash map's first try is now rejected with UNI-9, the rule of my mistake.

The new programs are again library code: lazy range adaptors with sentinels, a sorted flat map with a transparent comparator, an `Expected<T, E>` with explicit object parameters, a signal and slot list with move-only connections, and a monotonic arena with a vector inside it.
- Two first tries build clean and exit 0 on GCC and Clang. One builds and exits 4, and two fail to build. Each failure is my mistake.
- After workarounds, all five run on the front end. Round 10 also ran all five after workarounds.
- `flat_map` needed no front-end workaround. `expected` needed one, and `views`, `signal`, and `arena` needed two each. The workarounds cost `signal` its connection by member pointer and its generic fold, and cost the arena vector its index arithmetic.

The 11 reductions show faults that no earlier report names, except that the first explains a known stop:
- 3 false rejects, one of which is the cause of the TR-13 stop of `r10-list-first-try` that report 24 could not reduce,
- 5 stops,
- 3 results on storage where the front end differs from every compiler. One of the 3 is a wrong verdict on a valid program. The other two rest on a doubt in ADD-14 (section 6).

The front end found two of my storage mistakes that no compiler and no sanitizer reports. It named the rule of the cause for one of the two.

The front end still runs most short programs and catches planted errors well. It runs 23 of 24 short programs, stops 8 of 8 planted bugs with `undefined` and a rule, and rejects 9 of 9 planted mistakes. Its first error names the rule of the cause in 5 of the 9.

### 1.1 Counts

| Measure | Round 10 (`ebc109b9`) | Now (`f940ea53`) | Source |
|---|---|---|---|
| Rule headers | 1698 (1683 in force, 15 withdrawn) | 1698 (1683 in force, 15 withdrawn) | `counts.py` |
| Example blocks | 222: 67 unmarked, 28 `fragment`, 112 `reject`, 15 `diverges` | 224: 69 unmarked, the rest the same | `counts.py` |
| Tests | 6790 | 6881 | `counts.py`, files with `tidal-rules`, `.tu.cpp` left out |
| Program tests | 115 | 126 (8 `r10-`, 3 P2 added) | `counts.py` |
| Lines under `tests/programs/` | 13615 | 15442 | `counts.py` |
| Rules cited by a program test | 551 of 1683 | 559 of 1683 | `counts.py` |
| Tests that open `namespace std` | 255, 56 of them program tests | 269, 65 of them program tests | `counts.py` |
| Front end | 6586 of 6790 pass, 6626 their check | 6721 of 6881 pass, 6745 their check | close commits `ebc109b9`, `3bd790ae` |
| Completion line of decision 0060 | none (report 23 counted 204) | 127 tests fail their check, 2 of 126 program tests do not run, 9 out of scope | `frontend_check.py` in `3bd790ae` |
| `conformance.tsv` | 6786 rows | 6878 rows: 6402 pass, 282 EDG-ONLY, 18 pass-partial, 10 EDG-ALLOWED, 6 NO-DIAGNOSTIC-ALLOWED, 69 MISSED-REJECT, 57 unsupported, 24 run-unsupported, 8 FALSE-REJECT, 2 EDG-AGREES | the file |
| Changes under `frontend/` | round 23 | round 24: 12 files, 775 lines added, 60 removed | `git diff --stat ebc109b9 3bd790ae -- frontend/` |
| Port suites done | 1 of 8 (P1) | 2 of 8 (P1, P2) | `exercises.md` section 8 |
| Questions | 422: 176 open | 428: 180 open, 235 settled, 13 compiler defects | `status.md` |
| Front-end ledger | 157 rows, 5 open | 163 rows: 152 commit, 4 text, 4 out of scope, 1 question, 2 open (G161, G162) | `ledger_counts.py` |
| Exercise ledger | 925 rows, 17 open | 988 rows: 920 commit, 27 text, 12 question, 3 out of scope, 26 open | `ledger_counts.py` |

### 1.2 Round 10 items that phase 13 took and left

| Round 10 item | Phase 13 | Result |
|---|---|---|
| 1. Tests for every front-end finding that has none | item T2 | Done: every reduction and probe, the round 9 range `for`, I5-9, I5-10, W20-2, W20-3, G154, and 8 `r10-` program tests. |
| 2. Front-end round 24 with library-code targets | item FE | Done for the wrong verdicts, the false rejects outside the current instantiation, the friend class stop, and three missed rejects. Left: DEP-24 through an object of the current instantiation, class template argument deduction beside a constrained constructor template, and the TR-13 stop of `r10-list-first-try`. A stop can still name line 0. |
| 3. Answer the 22 open ledger rows | item F3 | Done. G153 to G157 name commits. |
| 4. Text from the round 10 reductions | item F3 | Done: DEP-24 with Q-0427, DEP-15 with DEP-39, UNI-9 with Q-0424, UNI-14 with Q-0425, a TSP-15 array example, and DTOR-6's link to DTY-3. |
| 5. Port suite P2 | item E | Done with a Java persona. All three ports ran on the first try. |
| 6. Interpreter output when every order agrees | none | Deferred by decision 0061. P2-9 now holds it in the ledger. |
| 7. Diagnose batches | none | Deferred by decision 0061. |
| 8. A gap review of the 20 rules changed after RV5 | item RV6 | Done, with repairs. The corrections commit `f940ea53` says that no source reader read the repair diff. |
| 9. Guides | none | Deferred by decision 0061. |

Round 10 process changes 1, 2, 3, and 4 are in place. `check_specification.py` fails on a front-end ledger row that names no test (`45d5b9bd`). The status page names G161 and G162. A clean first try is a program test, and a probe that the front end accepts against two compilers is a reject test (decision 0061).

### 1.3 The round 10 sketches, rerun

`compare_rerun.py` reads the 87 files of `results-rerun.txt`: 62 give the same front-end result as in round 10, and 25 changed. Every change but one moves toward the compilers' answer:
- 10 reductions now run to result 0. The 4 probes that changed run or reject as the compilers do. The UNI-9 probe is rejected, as Clang and EDG reject it.
- `smart_ptr_first_try.cpp` and `smart_ptr_second_try.cpp` run. The friend class stop is gone.
- `hash_map_second_try.cpp` runs. The NEW-5 wrong verdict is gone. `hash_map_first_try.cpp` is rejected with UNI-9, as every compiler rejects it.
- `variant_third_try.cpp` runs, as GCC and Clang run it. RV6 made the construction inside a member of an inactive member union valid at run time (G163).
- The tuple's first four tries now stop on "deduction of class template arguments" where they stopped on TSP-16, a missing constructor model, or OBJ-2. They get further and stop on `Tuple(1, 2)`.

The change that does not move toward the compilers is `list_first_try.cpp` and `list_second_try.cpp`. They gave OVL-35 and now give TR-13: "instantiation of 'cond<0, const Node*, Node*>' nests deeper than the limit of 256". Section 1.5 reduces the cause.

Three files still differ from GCC and Clang as in report 24's section 6:
- `reductions/member_template_through_current_instantiation_object.cpp`, `variant_first_try.cpp`, and `variant_second_try.cpp` give DEP-24. Q-0427 and the test follow the compilers.
- `reductions/ctad_guide_beside_constrained_constructor_template.cpp` still stops with "object of unknown type" at line 0.

The compilers and EDG give the same result on every file as in round 10, apart from the exit codes of planted bugs.

### 1.4 New programs

| Program | GCC, Clang (three flag sets) | EDG 7.0 | `tidal-fe run`, first try | `tidal-fe run`, last version |
|---|---|---|---|---|
| `views.cpp` (166 lines): `Iota`, a `Span` with a deduction guide, `Filter`, `Transform`, and `Take` over a range whose end is a sentinel of another type, adaptor closures joined with `|`, a fold over an iterator and a sentinel, three `static_assert` pipelines | first try builds, exits 4 (my mistake); second exits 1 (my mistake); third to last exit 0, clean | accepts all | rejected, TR-12, on `return Adaptor{...}` in a function template. **False reject.** | unspecified: result 0, after two workarounds |
| `flat_map.cpp` (224 lines): sorted storage from `::operator new`, binary search with a transparent comparator, `insert_or_assign` that shifts by moves, `operator[]`, `erase`, `erase_if`, `at` that throws, structured bindings over entries, a key class with `<=>` | first try fails (two mistakes of mine); second exits 1 (my mistake); third exits 0, clean | first try rejects; second and third accept | rejected, OVL-43, on my first mistake | result: 0, with no front-end workaround |
| `expected.cpp` (178 lines): a union of value and error built with `std::construct_at`, `Unexpected<E>`, `value()` and `error()` through an explicit object parameter for each value category, `and_then`, `transform`, `or_else`, a partial specialization for `void`, nine `static_assert` checks | first try exits 0, clean | accepts | rejected, TR-12, on `R(Unexpected{...})` in a member template. **False reject.** | result: 0, after one workaround |
| `signal.cpp` (204 lines): `Signal<R(Args...)>` as a partial specialization, slots in an intrusive list, `connect` for a function, for a member function given as an `auto` template argument, and for a callable by reference, a move-only `Connection` through a virtual base, emission that survives a slot that disconnects itself or another slot | first try exits 0, clean; third fails (my mistake) | accepts all but the third | unsupported, on `connect<&Counter::on>(c)` | result: 0, after two workarounds |
| `arena.cpp` (175 lines): a bump allocator with alignment, `make<T>` that records a destructor in the arena, a mark that rewinds and destroys in reverse order, a constructor that throws inside `make`, an over-aligned type, and `ArenaVec<T>` that grows inside the arena | first try fails (my mistake); second to last exit 0, clean | first rejects; the rest accept | rejected, NEW-20, on my mistake | result: 0, after three text-driven changes and one workaround |

The workarounds:
- `views`: `Adaptor(...)` in parentheses in place of `Adaptor{...}`, then `template <class P> filter(P)` in place of `filter(auto)`.
- `expected`: `Unexpected<E>{...}` in place of `Unexpected{...}` inside the member templates.
- `signal`: a lambda in place of `connect<&Counter::on>(c)`, then `sum(Args...)` with no template parameters in place of `collect(T init, Op op, Args... args)`.
- `arena`: `ArenaVec` steps through bytes and casts each slot, in place of `&d[i]` on a `T*`.

The arena's changes before the workaround came from the text. Its second try kept the storage in a member byte array, so `static_cast<T*>(allocate(...))` points to a byte (STG-29), and `&d[i]` for `i` of 2 or more is undefined (ADD-14). The front end stopped it with "undefined [STG-17]: placement new at an address that is not aligned", which names the wrong rule (section 1.5). The third try took its storage from `::operator new`. The front end then stopped it with ADD-11 on my `offset()`, which subtracted a byte pointer into an object from the storage pointer. That stop is right by ADD-11, and no compiler or sanitizer reports it. The fourth try used the aligned allocation form, because an offset that is a multiple of 32 is an aligned address only when the storage starts on 32. No tool reported that mistake. The front end then gave STG-17 for `&d[i]` in storage from the allocation function, which section 1.5 calls a wrong verdict.

Each first try and each intermediate version is kept beside the last version (`*_first_try.cpp` to `*_fifth_try.cpp`).

### 1.5 Reductions

Each reduction builds and exits 0 on GCC and Clang under the three flag sets, and EDG accepts it. The Status column compares with report 24 and round 10. `scan_tests.py` scans `tests/` for four of the forms (deduction with braces in a template, a pointer to member as a function template argument, a generic lambda in an abbreviated template, and a member template over the class pack) and finds none. Its few hits are other forms, which I read.

| Reduction | Lines | `tidal-fe` | Status | What the specification says |
|---|---|---|---|---|
| `converting_constructor_template_after_derived_pointer_construction.cpp`: `Iter<false> m(&d)` with `d` of a derived class and `explicit Iter(Base*)`, then `take(m)` with `take(Iter<true>)`, where `template <bool C = Const> requires C Iter(const Iter<false>&)` converts | 17 | rejected, OVL-35. **False reject.** | New. The cause of `r10-list-first-try`'s TR-13: one `static_cast<NodeBase*>(n)` in its `emplace` makes it run (probe `r10_list_first_try_with_base_cast.cpp`). The probes with a `Base` object, with a cast to `Base*`, and without the constraint show that the derived-to-base conversion in the earlier construction is the trigger. | TPL-13 and the conversion rules: the earlier construction changes nothing in the later conversion. |
| `ctad_braced_in_function_template_return.cpp`: `return Box{k};` inside `template <class K> auto times(K k)` | 7 | rejected, TR-12. **False reject.** | New. `Box(k)` and `Box b{k};` run (probes). In a member of a class template the same form stops (probe). | CTAD-1 lists a functional cast with braces. |
| `member_template_class_pack_before_deduced_parameter.cpp`: `template <class T> T collect(Args... args, T init)` in `Sig<Args...>` | 8 | rejected, DED-35. **False reject.** | New | The class's pack is known in the member, so only `T` is deduced. |
| `member_template_class_pack_after_deduced_parameter.cpp`: the same with `(T init, Args... args)` | 9 | unsupported: "call needing template deduction or unknown arguments" | New. A plain class type parameter after the deduced one runs (probe). | as above |
| `member_pointer_as_auto_template_argument.cpp`: `call<&Counter::add>(c)` with `template <auto Mem>` | 8 | unsupported: "call needing template deduction or unknown arguments" | New. A typed parameter `int (Counter::*)(int)`, a pointer to data member, and a class template stop too (probes). A function pointer runs. No test calls a function template with a pointer to member as its argument. | TPL-11 and TARG-12 allow the argument. |
| `generic_lambda_returned_from_abbreviated_template.cpp`: `auto scale(auto k) { return [k](auto r) { return r * k; }; }`, then `scale(2)(3)` | 6 | unsupported: "call of unknown callee" | New. With `template <class K> scale(K k)` the program runs (probe). | DED-34 and LAM-15: the two templates are the same as with named parameters. |
| `generic_lambda_inside_abbreviated_template.cpp`: the generic lambda inside `int apply(auto k)` | 7 | unsupported: "object of unknown type" | New | as above |
| `template_lambda_with_requires_clause.cpp`: `[]<class T>(T x) requires (sizeof(T) > 1) { return x; }` | 6 | unsupported: "object of unknown type", at line 0 | New. The requires clause after the template head stops too, and the lambda without a requires clause runs (probes). Line 0 is the form that rounds 9 and 10 asked to fix. | LAM-15 |
| `placement_at_typed_step_after_byte_offset_in_allocated_storage.cpp`: `int* d` cast from `base + 8`, where `base` comes from `::operator new(64)`, then `::new (d + 1) int(2)` | 11 | undefined [STG-17]: not aligned. **Wrong.** | New. From `d` at the start of the storage, the same runs (probe). | STG-P3 and STG-28: the allocation creates the `int` objects, and ADD-14 does not apply to a `T*` into them. |
| `placement_at_typed_step_into_byte_array.cpp`: `int* d` cast from a byte array, then `::new (d + 1) int(2)` | 9 | undefined [STG-17]: not aligned | New | ADD-14 counts `*d` as an array of one, so `d + 1` is one past it. The text does not say where that position is, or whether placement `new` can use it (section 6). |
| `typed_step_over_byte_array_distance.cpp`: the byte distance from the array to `d + 1` | 7 | result: 1. GCC and Clang give 0 (a distance of `sizeof(int)`). | New. With `d + 2` the front end runs and reports nothing, where ADD-14 reads as undefined (probe). | as above |

By status: 1 explains a known stop, and 10 are new. The front end steps a `T*` that points to a byte by one byte. The last three rows, and the arena's STG-17 stops, come from that one fault.

### 1.6 Short programs, planted bugs, and planted mistakes

**Short programs.** The 24 short programs in `new/one-line/` build and run clean on GCC and Clang with every flag set, and EDG accepts all 24. Clang warns twice on `aggregate_with_base.cpp` for elided braces. The front end runs 23 (`summarize.py`: 16 "result: 0" and 7 "unspecified: result 0"). The forms are:
- a recursive lambda through `this auto`, a `static` call operator, and a lambda with a template head,
- designated initializers, `using enum` in a `switch`, `if` with an init statement, and `[[fallthrough]]`,
- a `consteval` table, a `constinit` global, the `noexcept` operator, and a rotate on `unsigned`,
- the curiously recurring template pattern, inheriting constructors, aggregate initialization with a base, and a delegating constructor,
- the type of a UTF-8 literal, a nested namespace definition with an inline namespace, `decltype(auto)` on a member in parentheses, and an `explicit operator bool`,
- a pointer to data member as a sort key, capture-free lambdas in a table of function pointers, structured bindings to an array and to a struct, a member function with a requires clause, and a pack element by recursion and by a comma fold.

The front end stops on `template_lambda.cpp`, a lambda with a template head and a requires clause, at line 0 (section 1.5). The forms near the 22 rules changed after RV6 read the tree are few: `decltype_auto_member.cpp` touches CV-6's `decltype` and runs on every tool. None of this report's findings rests on the 22 rules.

**Planted run-time bugs.** The front end stops all 8 bugs in `new/bugs/` with `undefined` and a rule:

| Rule the front end names | Bug | Sanitizers |
|---|---|---|
| MUL-10 | a hash in `int` that overflows | UBSan |
| PFX-4 | a loop with `<=` that reads past the array | ASan |
| OBJ-2 | a reference returned to a local | ASan (Clang), UBSan (GCC) |
| SHF-7 | `1u << 32` | UBSan |
| SEQ-4 | `i++ + i++` | silent; both compilers warn |
| OBJ-2 | a shallow copy of an owner, then a read after the copy's `delete` | ASan (GCC); both runtimes abort on the double free |
| DER-17 | a `static_cast` to the wrong derived class | UBSan (GCC) |
| OBJ-2 | a view returned over a local array | ASan |

The unsequenced increments pass every sanitizer, and the front end stops them. For the shallow copy, the front end stops at the read before the second `delete`, which is the first undefined operation.

**Planted compile-time mistakes.** GCC, Clang, EDG, and the front end reject all 9 mistakes in `new/mistakes/`. The front end's first error names the rule of the cause in 5: CLS-25 (a non-`const` member on a `const` object), LIST-2, DEL-1, ILC-8, and OVL-36. In the other 4 it names a general rule:
- TR-12 for `C::value_type v` with no `typename`, where DEP-19 is the cause,
- TR-12 for `t.get<1>()` with no `template`, where DEP-24 is the cause,
- DED-35 for a call whose only candidate has a failed constraint, where a CNS rule is the cause,
- OVL-35 for `int&` bound to a temporary, where REF-1 is the cause.

**My own mistakes from memory.** Four programs held seven:
- `views_first_try.cpp`: `Span(data)` on an aggregate with no constructor is aggregate initialization in parentheses, so the length was 0. Every compiler builds it, and only GCC warns (`-Wmissing-field-initializers`). The front end stopped before the run.
- `views_second_try.cpp` and `flat_map_second_try.cpp`: wrong expected sums. The front end gave result 1 on the second, as the compilers did.
- `flat_map_first_try.cpp`: `std::strong_ordering` without the reversed operators, which `"key" < Str` needs (LSUP-12), and a range `for` over a braced list with no `std::initializer_list` (LSUP-1). The front end named OVL-43 for the first. LSUP-12 is the rule of the cause.
- `signal_third_try.cpp`: `R sum(R init, ...)` in a class that `Signal<void(int)>` instantiates. The front end named DCL-22.
- `arena_first_try.cpp`: an array placement `new` with no array placement form. The front end named NEW-20.
- `arena_second_try.cpp` and `arena_third_try.cpp`: the storage mistakes of section 1.4. Only the front end reported them.

### 1.7 The phase 13 reports, from a programmer's view

**Front-end report 24.** The round did what round 10 asked, and its count follows decision 0061. The two fixes that matter most to a programmer are the base pack in parentheses and `delete[]` after placement `new` into an element, because each gave `undefined` on a valid program. The report states its distance to decision 0060 with one line: 127 tests and 2 program tests. The two program tests are `r10-list-first-try` and `r10-tuple-second-try`. Section 1.5's first reduction gives the cause of the first. The second needs class template argument deduction beside a constrained constructor template, which round 10 reduced.

The completion line counts tests only. In this round, five library programs in one session found 11 faults that the 6881 tests do not hold. Round 10 found 13 the same way. A front end that meets decision 0060's test can still stop on half of the first tries of a library programmer.

**P2.** The Java persona ran all three ports on the first try. My programs met more faults because they use more template forms: class template argument deduction inside a template, generic lambdas in abbreviated templates, pointers to members as template arguments, and member templates over a class pack. P2-1 and P2-2 are the questions I answer from general knowledge in every round: how to write an owning array and a shared handle with no library. My `flat_map` and `arena` are the "write" branch of NEW-P2 that P2-1 asks for an example of. P2-9 is the "unspecified" first word that rounds 7 to 10 raised. It now has a ledger row.

**I6.** I6-5, I6-9, and I6-13 matter most to a programmer. A programmer writes `f(this->n)`, `ns::f(t)` with a dependent `t`, and a call by value of a class template specialization every day (general knowledge). The text does not say when the first two bind, or whether the third instantiates the class. I6-11 and I6-12 were front-end faults with tests, and round 24 fixed both.

**The RV6 reviews.** The repairs changed the answer for my round 10 variant: a construction inside a member of an inactive member union is valid at run time, and the front end now runs `variant_third_try.cpp`. CNS-11 and CPY-15 (RV6-B-1, RV6-A-18) read against the C++23 text and keep the compilers' answer under Q-0426. For a programmer the compilers' answer is the useful one, because the W20-3 holder runs on GCC and Clang.

### 1.8 Ledger rows

The status page names 26 open exercise rows (P2-1 to P2-4, P2-6 to P2-9, I6-1 to I6-10, I6-13 to I6-20) and the open front-end rows G161 and G162. `ledger_counts.py` gives the same 26 and 2. No row is `planned`. No open row names a front-end fault without a test, because P2-9 names `programs/P2/ledger.cpp` and G161 and G162 name their tests.

## 2. Ranked work items

| # | Item | Evidence | Size | Inputs |
|---|---|---|---|---|
| 1 | Tests for this round's reductions, probes, and clean first tries | 11 reductions, 8 probes that stop or reject, 2 first tries that build clean | small | `reductions/`, `probes/`, `new/` |
| 2 | Front-end round 25, with library-code targets first | 1 wrong verdict and a wrong step for a `T*` over bytes, 3 false rejects, 5 kinds of stop; the 2 program tests of the completion line | large | item 1; item 3 for the ADD-14 expectations |
| 3 | Text for typed pointers over bytes, and G161 and G162 | the ADD-14 position, placement `new` at it, an arena on a member byte array; G161, G162 | small, with probes | `reductions/`, the front-end ledger |
| 4 | Answer the 26 open rows, starting with the rows a programmer meets | P2-1 to P2-4, P2-7, I6-5, I6-9, I6-13, then the rest | medium | the P2 and I6 reports |
| 5 | Interpreter output when every order agrees | P2-9; 1 of 5 last versions and 7 of 24 short programs print "unspecified: result 0" | small | none |
| 6 | First errors that name the rule of the cause | 4 of 9 planted mistakes and 2 of my mistakes; a stop at line 0 | one session | none |
| 7 | A gap review of the 22 rules changed after RV6 read the tree, with a read of the RV6 repair diff | close commit `3bd790ae`; `f940ea53` | one review session, then repairs | none |
| 8 | Port suite P3 on pipelines and events | 9 of this round's 11 reductions come from views, expected, and signal | one session | item 2 |
| 9 | Guides on storage, owners, and adaptors | the arena's three text-driven changes; P2-1, P2-2 | one decision, then one session per guide | maintainer |

### Item 1. Tests for this round's reductions, probes, and clean first tries

**What.** A clean-room writer adds a test for each of these, probes it on the matrix, and writes the expectation that the compilers and the text give:
- run tests that expect result 0: the converting constructor template after a derived pointer (TPL-13, OVL-35), the braced deduction in a function template and in a member of a class template (CTAD-1), both member templates over a class pack (DED-2), a pointer to member function and to data member as template arguments of a function template and of a class template (TPL-11, TARG-12), the two generic lambdas in abbreviated templates (DED-34, LAM-15), the template lambda with a requires clause in both places (LAM-15), and the placement at a step in storage from the allocation function (STG-28, ADD-14),
- the two byte-array reductions and the `d + 2` probe, after item 3 decides ADD-14's position,
- the first tries `expected_first_try.cpp` and `signal_first_try.cpp` as program tests, since both build clean on GCC and Clang (decision 0061). The last versions of all five go in as `r11-` program tests.

**Why.** Sections 1.4 and 1.5. The front-end harness sees only tests. The completion line of decision 0060 cannot count a fault that no test holds.

**Size.** Small. Each reduction has at most 17 lines.

**Inputs.** `programmer-sketches/reductions/`, `probes/`, `new/`.

### Item 2. Front-end round 25, with library-code targets first

**What.** Round 25 takes the targets in this order:
1. The wrong verdicts and the wrong step. A `T*` that points into storage steps by `sizeof(T)`. Placement `new` at such a step in storage from an allocation function runs. Item 3 gives the answer for a byte array.
2. The two program tests of the completion line. The converting constructor template after a derived-to-base pointer conversion (section 1.5) fixes `r10-list-first-try`. Class template argument deduction beside a constrained constructor template fixes `r10-tuple-second-try` and the tuple's earlier tries.
3. The false rejects: `Name{...}` with deduction inside a template, and a member template whose parameters hold the class's pack before a deduced parameter.
4. The stops: the pack after a deduced parameter, a pointer to member as a template argument, a generic lambda in an abbreviated template, and a template lambda with a requires clause. A stop gives the line of the construct, never line 0.
5. DEP-24 through an object of the current instantiation (Q-0427), which report 24 calls rewrite work.
6. The 127 tests of the completion line, by page.

**Why.** Sections 1.4 and 1.5. The first two targets make the program half of decision 0060's test true. The others are what library code writes first: `Unexpected{e}`, `connect<&C::f>(obj)`, `filter(auto pred)`, and a fold over the class's pack. My workarounds cost `signal` its member connections and its fold, and cost `ArenaVec` its index arithmetic.

**Size.** Large.

**Inputs.** Item 1's tests. Item 3 for the byte-array expectations.

### Item 3. Text for typed pointers over bytes, and G161 and G162

**What.**
- ADD-14: say where `d + 1` is when `d` is a `T*` that points to a byte, and whether placement `new` at that position is valid. Every compiler places it `sizeof(T)` bytes on.
- STG-P2 says that a class with a member byte array cannot use index arithmetic over its objects, and that such a class takes its storage from an allocation function. An arena example beside the FixedVec example would show the consequence, with the aligned allocation form for an over-aligned type.
- G161: a jump into a loop body goes on as after a normal pass.
- G162: the library defines `std::nothrow`.

**Why.** Section 1.5. The arena is a common library form (general knowledge), and the text leaves the expectation of two reductions open. G161 and G162 are the two open front-end rows.

**Size.** Small. Each needs a probe on the matrix.

**Inputs.** `reductions/placement_at_typed_step_*.cpp`, `typed_step_over_byte_array_distance.cpp`, `probes/typed_step_two_past_byte.cpp`, `new/arena_second_try.cpp` to `arena.cpp`, report 24 section 8.

### Item 4. Answer the 26 open rows, starting with the rows a programmer meets

**What.** For a programmer, these come first:
- P2-1 and P2-2: one example of an owning array with its copy functions deleted, and one of a counted shared handle (NEW-P2, CPY-P2). This round's `flat_map` and `arena` are inputs for the first.
- P2-3 and P2-4: an example line for `==` on two `const char*`, and a wrapping hash in `unsigned` that ties ADD-9, MUL-10, MUL-8, and CONV-2.
- P2-7 and P2-8: a pointer or `nullptr` for a missing value, and a link to CE-3 from the array bound rule.
- I6-5, I6-9, and I6-13: `f(this->n)`, a qualified call with a dependent argument, and the uses that instantiate a class.

Then P2-6 and the other I6 rows. P2-9 goes with item 5.

**Why.** Section 1.7. P2 and this round met the same gaps from two personas.

**Size.** Medium.

**Inputs.** The P2 and I6 reports and their programs.

### Item 5. Interpreter output when every order agrees

**What.** Round 10 item 6, unchanged, now ledger row P2-9. When every explored order gives one result, `tidal-fe run` prints the result first, such as "result: 0 (every order of 4 gives 0)". When the bound stops the exploration, the line says so first.

**Why.** `views.cpp`, 7 of 24 short programs, and all five round 10 last versions but the variant print "unspecified: result 0" with no other outcome. Two personas now read it as a fault in the program.

**Size.** Small.

**Inputs.** None.

### Item 6. First errors that name the rule of the cause

**What.** A Diagnose batch, or a front-end target, for the first error of these forms: a missing `typename` (DEP-19) and a missing `template` (DEP-24) reported as TR-12, a failed constraint reported as DED-35, a reference binding reported as OVL-35, a missing reversed ordering operator reported as OVL-43 (LSUP-12), and a step over bytes reported as STG-17 (ADD-14).

**Why.** Section 1.6. The planted mistakes in round 10 all named the cause. This round's planted mistakes are template and binding mistakes, and 4 of 9 name a general rule. Report 24 gives 2087 of 2709 passing rejects (77%) whose first error names a cited or linked rule.

**Size.** One session.

**Inputs.** `new/mistakes/`, `new/flat_map_first_try.cpp`, `new/arena_second_try.cpp`.

### Item 7. A gap review of the 22 rules changed after RV6 read the tree, with a read of the RV6 repair diff

**What.** A source-reader review of DTOR-6, DTOR-8, CPY-15, UNI-P1, UNI-P3, UNI-6, UNI-9, UNI-14, CV-6, RET-6, REF-1, REF-2, NEW-22, CONV-1, NAM-21, VC-2, JMP-P3, JMP-10, DED-8, DED-24, DEP-P5, and DEP-24, and of the repair diff `9c009448`, then repairs.

**Why.** The close commit names the 22 rules for the next review, and `f940ea53` says that no source reader read the repair diff. A programmer ranks the review lower, because no finding of this report rests on these rules.

**Size.** One review session, then repairs.

**Inputs.** None.

### Item 8. Port suite P3 on pipelines and events

**What.** A port suite with a C# persona: a LINQ-style pipeline of `Where`, `Select`, and `Take` over a sequence, an event with handlers that remove themselves, and a result type with chained steps.

**Why.** Nine of this round's 11 reductions come from `views`, `expected`, and `signal`. A C# programmer writes generic lambdas, deduction from braces, and handlers bound to members as a newcomer writes them (general knowledge). P2's Java ports used plain classes and ran on the first try.

**Size.** One session.

**Inputs.** Item 2, so that P3 measures the new front end.

### Item 9. Guides on storage, owners, and adaptors

**What.** A maintainer decision on publishing guides, then how-to guides on an arena and a vector in raw storage, an owning array and a shared handle (P2-1, P2-2), and range adaptors with sentinels.

**Why.** The arena needed three changes that came from STG-29, ADD-14, ADD-11, and the alignment of the allocation function. The rules are right, and they are spread over the storage, additive, and new-delete pages.

**Size.** One decision, then one session per guide.

**Inputs.** The maintainer.

## 3. Out of scope

| Topic | Proposal | For | Against |
|---|---|---|---|
| The general library, including `forward`, `move`, `construct_at`, `strong_ordering`, and `initializer_list` | Keep out. Programs declare their own, as the five new programs do. | Each program needs them (general knowledge). | The charter leaves out the library. LSUP-1, LSUP-2, and LSUP-10 give the shapes, and each took a few lines. |
| The launder operation | Keep referenced. | An arena on a member byte array needs it (STG-29). | It is a library function. Storage from the allocation function avoids it. |
| The front-end rewrite (FR) before decision 0060's first condition | Keep waiting for the completion test. | The completion line has one count, and 2 program tests are left. | 127 tests fail their check, and this round's 11 forms have no test. |
| `typeid` | Keep deferred. | | Each test needs the library's `type_info` (Q-0322). |
| Threads, the memory model, coroutines, modules, extended floating-point types | Keep out. | | as the charter says |
| Language versions after C++23 | Keep out until `pending.md` row 7 is decided. | | Strict mode rejects them. |

## 4. Process changes

1. **A roadmap round adds its own reductions as tests in the phase that follows it**, as T2 did for round 10. Item 1 keeps that practice.
2. **The completion line has a companion count of library programs.** A phase close could also give how many roadmap and Write first tries that build clean on GCC and Clang run on the front end. Decision 0060's test reads tests only, and this round's faults came from first tries.
3. **A planted mistake whose first error names a general rule gets a ledger row**, as a front-end finding with its test. Round 10's mistakes all named the cause, and this round's did not.

## 5. Suggested order

1. Item 3 comes first, because it is small and two tests of item 1 need its ADD-14 answer.
2. Item 1's tests are the first commit of item 2. Round 25 then takes the wrong step and the two program tests first.
3. Item 4 runs beside item 2.
4. Item 8 follows item 2.
5. Items 5, 6, and 7 run beside the others at any point. Item 9 waits on the maintainer.

## 6. Doubts for the clean-room writers

- Where `d + 1` is when `d` is a `T*` that points to a byte (ADD-14), and whether placement `new` at that position is valid. Every compiler places it `sizeof(T)` bytes on. The front end places it one byte on and reports STG-17.
- Whether a `T*` made from a byte offset into storage from an allocation function points into an array of `T` that the allocation created (STG-28, STG-P3). My reading is yes, and the front end treats it as a pointer to a byte.
- Whether an `unsigned char*` that points into an object can be subtracted from the pointer to the storage that hosts the object (ADD-11). The front end says no, and the compilers run it.
- Whether "unspecified" should stay the first word when every explored order gives the same result (rounds 7 to 10 and P2-9, still open).

## 7. Claims that rest on the host compilers alone

Decision 0061 asks a reader report to list these, so that a writer probes them on the matrix first:
- Every reduction and probe result for GCC and Clang comes from host GCC 16.2.1 and Clang 22.1.8. EDG 7.0 ran in its container. GCC 14, GCC 16.2.0, and Clang 18 did not run.
- The byte distance of `d + 1` over a byte array (`typed_step_over_byte_array_distance.cpp`) is `sizeof(int)` on the host GCC and Clang.
- The sanitizer results of section 1.6 come from the host compilers.
