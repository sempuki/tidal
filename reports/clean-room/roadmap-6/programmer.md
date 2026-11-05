# Roadmap, round 6: what to do after phase 8, from a programmer's view

Date: 2026-11-04. Decision 0029. This report comes from a clean-room reader in the role of a C++ programmer who uses the specification to write and check programs. The reader read AGENTS.md, CLAUDE.md, the round 5 programmer report (`reports/clean-room/roadmap-5/programmer.md`) and its kept sketches, the charter, roadmap, status, style guide, specification format, exercises catalog, decisions 0033, 0039, 0044, 0050, 0051, 0054, and 0055, and `documentation/decisions/pending.md`. The reader also read parts of `language-support.md`, `unions.md`, `instantiation.md`, and `member-pointers.md`, the headers of `tests/programs/` and of the tests named below, `questions/README.md` with Q-0377 to Q-0385, front-end reports 18 and 19, the W16, W17, and I1 reports, the R8 commit, the exercise status rows of X17 to X20, and `git log --oneline dc90663~1..HEAD` (18 commits). The reader did not read the other round 6 reports. All counts come from read-only `grep` and Python scripts on the tree at commit `1b16115`.

The reader built the front end from `1b16115` into the session scratchpad. The reader ran `tidal-fe run` on the four new program tests and on every test that `conformance.tsv` marks as run-unsupported, a wrong run result, or a false reject, and reran the round 4 and round 5 sketches. The reader wrote three new programs (583 lines) and reduced each stop to a short program. Each program and reduction was built with host GCC 16.2.1 and Clang 22.1.8 (`-std=c++23 -pedantic-errors -Wall -Wextra`, then `-fsanitize=undefined,address`), checked on the EDG 7.0 container (`cpfe --c++23 --strict`), and run through `tidal-fe run`. The reader also extracted the 197 `cpp` example blocks of `specification/` and ran `tidal-fe check` on the 163 blocks that carry no marker or the `reject` marker. The sketches are in `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/rm6/programmer/sketches/`. No file in the repository was written. Statements that rest on general knowledge say so.

## 1. The state from a programmer's view

Phase 8 took every item that the round 5 programmer view ranked first to third. The five reductions became tests, and the interpreter now runs each one. The Write wave ran, and its four programs are program tests. A tool compiles the page examples. The interpreter knows a closed list of C functions, so a program can print.

Two gaps remain. First, the Write prompt does not ask the reader to run the interpreter, and the second Write wave ran after the last front-end round. The W16 and W17 reports say that no build or run stopped, but the interpreter stops on 2 of their 4 programs. Second, the example tool runs only the compilers. The front end accepts 6 page examples that the page marks as rejected.

My three new programs found one more false `undefined` verdict, two more run stops, and two missed rejects. Each fault sits where two pages meet: a destructor that is virtual through a base, a union member written inside a member function, an aggregate member built by a converting constructor. Each page's own tests pass.

### 1.1 Counts

| Measure | Round 5 (`7d8c982`) | Now (`1b16115`) | Source |
|---|---|---|---|
| Rule headers | 1691 (1676 in force, 15 withdrawn) | 1696 (1681 in force, 15 withdrawn) | `:::{rule}` headers and `:kind:` lines in `specification/` |
| Content pages | 85 | 85 | pages with a rule |
| Tests | 6337 | 6500 | files with `tidal-rules` under `tests/`, `.tu.cpp` units left out |
| Rules with one test | 8 | 0 by `:tests:` lines; 2 by test headers (CPY-28, CPT-23) | the status table, and this report's script |
| Program tests | 67 | 71 (W16 and W17 add 4) | `tests/programs/` |
| Rules cited by a program test | 477 of 1676 | 498 of 1681 | `tidal-rules` in `tests/programs/` |
| Pages reached by a program test | 80 of 85 | 84 of 85 (`keywords.md` is the one left) | the pages of those rules |
| Program tests that run to a result in `tidal-fe` | 67 of 67 | 69 of 71 | `conformance.tsv`, and this report's run |
| Write sessions in the phase | 0 | 2 (W16, W17): 4 programs, 1456 lines, 14 findings | `exercises.md` section 8 |
| Port suites | 0 of 8 | 0 of 8 | `exercises.md` section 3 |
| Pages with a Diagnose batch | 4 | 4 | `exercises.md` section 8 |
| Guides | 5 (last change 2026-10-31) | 5 (last change 2026-10-31) | `guides/` |
| Questions | 376: 209 open, 154 settled, 13 compiler defects | 385: 221 open, 151 settled, 13 compiler defects | `questions/README.md`, `status.md` |
| *(referenced)* markers | 32 | 33: library 21, extended floating-point 5, coroutines 3, preprocessor 2, memory model 1, threads 1 | `status.md` |
| Front end, full pass | 6068 of 6337 (judged by EDG overrides first) | 6183 of 6500 (judged by the base expectation since item HN) | `conformance.tsv` |
| Front end, run-unsupported | 65 | 57 | `conformance.tsv` |
| Front end, false rejects and wrong run results | 4 and 1 | 12 and 7 | `conformance.tsv`; item HN counts more cases as failures |
| Example blocks | 181, none checked | 197: 54 with no marker, 27 `fragment`, 109 `reject`, 7 `diverges`; `tools/check_examples.py` checks each on the compilers | this report's script |
| Example blocks where `tidal-fe check` disagrees with the marker | not measured | 6 of 109 `reject` blocks accepted, 1 unsupported; 0 of 54 unmarked blocks rejected | this report's script |
| Tests that open `namespace std` | 200, 25 of them program tests | 212, 28 of them program tests | `grep` |

The largest run-unsupported reasons in the front end are a `new` expression (8), a constructor call (6), an overload set (3), a jump into a nested statement (3), a variable of an enclosing function in a local class (3), and range-based `for` (3). The 7 wrong run results are these:

- the two UNW-1 tests and the OBJ-11 test that `pending.md` row 4 holds,
- two STG-29 tests with a member byte array (G139),
- two linkage tests (`c-variable-and-global-variable-two-units.cpp`, `class-unnamed-namespace-internal.cpp`).

### 1.2 Round 5 items that phase 8 took and left

| Round 5 item | Phase 8 | Result |
|---|---|---|
| 1. Tests from the reductions, and a front-end round | items RT and FE (round 19) | Done. The five reductions and the tuple protocol example are tests, and all pass in the front end. The round 5 sketches themselves did not become tests (section 1.4). |
| 2. A second Write wave of larger programs | item E (W16, W17) | Done, with the four tasks that round 5 named. The prompt has no interpreter step, so the readers did not see the two stops. |
| 3. A tool that checks the page examples | item RT (`tools/check_examples.py`) | Done for the compilers. The tool does not run the front end (`specification-format.md`, section Examples). |
| 4. Port suites | not taken | Decision 0055 defers Port again. |
| 5. Guides | not taken | Deferred. The W16 and W17 repairs put two of the guide's shapes on the pages as examples: the move-only owner on `copy-move.md` and the fixed-capacity container with kept pointers on `storage.md`. The `reinterpret_cast` part of CAST-6 now links STG-29. |
| 6. Diagnose batches | not taken | Deferred. |
| 7. The program's boundary | item FE | Done. `_Exit`, `exit`, `abort`, `malloc`, `free`, `puts`, and `putchar` run. The round 4 `hello.cpp` now runs to result 0. |
| 8. Q-0322 and `typeid` | item R2 for Q-0322 | Q-0322 has the EDG 7.0 results. `typeid` has its own Deferred entry and still no rule (13 mentions on 5 pages, 1 test). |
| 9. Records that drift | item R2 | Done for the roadmap's Deferred list, `exercises.md` sections 6 and 7, and the `typeid` sentence. Smaller drifts are new (section 1.6). |

Round 5 process changes 1, 2, and 4 are in decision 0055 or the tools. Change 3 holds in the Write prompt for compiler stops only. Change 5 did not happen. Report 19 does not rerun the roadmap sketches, and the round 5 `evaluator.cpp` still stops.

### 1.3 Program tests

The 71 program tests carry 6 EDG overrides (W9 three, W12 one, W2 and W6 one each), and the new programs carry none. Two new programs stop in the interpreter. Front-end round 19 (`1f81776`) ran before W16 and W17 (`f049e89`, `26f05fc`), so no round has seen them.

| Program | `tidal-fe run` | Reduction | Cause |
|---|---|---|---|
| W16 `evaluator.cpp` | unsupported at line 290: class assignment | `assign_converted_owner.cpp` (15 lines) | An assignment from a prvalue of `Owner<Derived>` to an `Owner<Base>`. The converting constructor template makes the temporary, then the move assignment runs. With the conversion in its own declaration first, the program runs (`assign_converted_owner_twostep.cpp`, `evaluator_w16_twostep.cpp`). |
| W17 `units/main.cpp` | unsupported at line 7: overloaded operator | `template_member_operator_outside.cpp` (9 lines) | A member operator of a class template, defined after the class body, used with operator syntax. The call `a.operator<(b)` runs, and so does the same class without a template. With the six operators defined in the class, the whole program runs (`units_inline/`). |

No test under `tests/` defines a member operator of a class template outside its class. `programs/W17/units/fixed.h` is the only file with that form.

The round 5 `evaluator.cpp` stops on the same assignment. The interpreter now reports it as "class assignment", and round 5 saw "conversion function".

### 1.4 Sketches

The round 4 and round 5 sketches, rerun at `1b16115`:

| Sketch | Round 5 | Now |
|---|---|---|
| round 4 `hello.cpp` | unsupported (`puts`) | result 0 |
| round 4 `condtriv.cpp` | unsupported: "static_assert: concept" | The same. It uses a compiler built-in. RT added the built-in-free form as `destructor-trivial-by-concept.cpp`. |
| round 4 `typeid.cpp` | unsupported | unsupported |
| round 5 `evaluator.cpp` | unsupported: "conversion function" | unsupported: "class assignment" (section 1.3) |
| round 5 `hashmap.cpp` | false reject (TR-12) | undefined (STG-23, at the `reinterpret_cast` read), which is correct |
| round 5 `hashmap_kept_pointer.cpp`, `prelude.cpp`, `two_byte_rows.cpp`, `template_member_alias.cpp`, `move_assign_only.cpp`, `owner_convert.cpp` | false undefined, false reject, or unsupported | result 0 |

Every other round 4 and round 5 sketch gives the result it gave before. None of the whole sketches became a test, though decision 0055 says that correct sketches become tests. RT added the reductions only.

The new sketches:

| Sketch | GCC, Clang, sanitizers | EDG 7.0 | `tidal-fe run` |
|---|---|---|---|
| `inventory_first_try.cpp` (196 lines): a sorted ledger with `<=>`, a growable array from an `initializer_list`, a user literal, a fold expression, `if constexpr`, a `consteval` function that allocates, and a failed `dynamic_cast` to a reference | both reject | rejects | result 0 (missed reject, MPTR-8 with INST-6) |
| `inventory_strong_only.cpp`: the same after the fix, with only `std::strong_ordering` declared | exit 0, clean | rejects | result 0 (missed reject, LSUP-2) |
| `inventory.cpp` (211 lines): with all three ordering classes | exit 0, clean | accepts | result 0 |
| `scheduler_first_try.cpp` (210 lines): a priority heap over raw storage, a virtual base, a covariant `clone`, a function-try-block, a rethrow, and a member function pointer | both reject | rejects | rejected (DER-13), which is correct |
| `scheduler.cpp`: the same with the virtual base initialized by the most derived class | exit 0, clean | accepts | false undefined (VIRT-12) |
| `scheduler_cast_delete.cpp`: the same, deleting through the derived pointer | exit 0 | not run | result 0 |
| `text_first_try.cpp` (161 lines): a string with a short buffer in an anonymous union, words through an iterator and an end marker, and a pair with the tuple protocol and class template argument deduction | both reject | rejects | rejected (NEW-20), which is correct |
| `text.cpp`: the same with the placement form declared before the class | exit 0, clean | accepts | unsupported: "pointer arithmetic" |
| `text_local_init.cpp`: the same with `local{}` in the move constructor and `Str("")` in one aggregate | exit 0 | not run | result 0 |

A copy of each finished program with one wrong expected value exits 1 on GCC and in the interpreter.

I reduced each stop to a short program. Each run reduction builds and exits 0 on GCC and Clang with both sanitizers, and EDG accepts it. GCC, Clang, and EDG reject `member_template_signature_invalid.cpp`. EDG rejects the two LSUP-2 reductions, and GCC and Clang accept them.

| Reduction | Lines | `tidal-fe` | What the specification says |
|---|---|---|---|
| `delete_through_middle_base.cpp`: `B` has a virtual destructor, `M : B`, `D : M`, and the program deletes a `D` through an `M*` | 8 | undefined, VIRT-12 | VIRT-7: the implicit destructor of `M` overrides the virtual destructor of `B`, so it is virtual. Deleting through a `B*` runs (`delete_through_root_base.cpp`). The same fault happens when `M` declares `~M() {}` (`delete_through_middle_base_declared_dtor.cpp`). |
| `union_array_write_in_member_function.cpp`: a member function writes `local[1]` of an anonymous union whose array member is not active | 10 | unsupported: "pointer arithmetic" | UNI-13: an assignment through member accesses and built-in subscripts makes each inactive union member active. The same write from `main` runs (`union_array_write_from_main.cpp`). A named union fails the same way. |
| `aggregate_member_converting_constructor.cpp`: `Pair p{"x"};` where the member type has a constructor from `const char*` | 11 | unsupported: "aggregate with nested aggregates" | An aggregate element is copy-initialized from its initializer, here through a converting constructor. `Pair p{Name("x")}` runs. |
| `assign_converted_owner.cpp` | 15 | unsupported: "class assignment" | section 1.3 |
| `template_member_operator_outside.cpp` | 9 | unsupported: "overloaded operator" | section 1.3 |
| `member_template_signature_invalid.cpp`: `Box<int>` where `Box<T>` declares `template <class K> void by(K T::* key);` | 5 | accepted | INST-5 and INST-6: the instantiation of the class makes each member declaration, and the declaration must be valid. MPTR-8: the class of a pointer to member must be a class. The front end rejects the same declaration as a plain member function (`member_signature_invalid.cpp`). |
| `ordering_strong_only_defaulted.cpp`: a defaulted `<=>` with only `std::strong_ordering` declared | 15 | result 0 | LSUP-2: a program that uses a defaulted `<=>` declares all three ordering classes first. |
| `ordering_strong_only_builtin.cpp`: `1 <=> 2` with only `std::strong_ordering` | 12 | result 0 | the same rule and the same split |

My three first tries each had one mistake from memory, and Tidal has the rule for each. The interpreter named two of the rules in its message (DER-13 and NEW-20). It missed the third (MPTR-8 inside a member template). For LSUP-2, only EDG rejected my program, and only the rule's own text told me why. No test checks LSUP-2's demand for three classes. `builtin-three-way.cpp` declares two classes and expects acceptance, because its operands appear only in `decltype`.

The VIRT-12 verdict matters most to a programmer. A class hierarchy with a virtual destructor only at the root, deleted through a middle class, is ordinary code (general knowledge). The interpreter calls it undefined. `virtual-dtor-inherited.cpp` deletes through the root, so no test covers the middle class.

### 1.5 Page examples

The specification has 197 `cpp` example blocks. `tools/check_examples.py` compiles each one on the eight baseline compilers and checks its marker. The front end is not part of the check. I ran `tidal-fe check` on each unmarked block and each `reject` block:

| Marker | Blocks | `tidal-fe check` |
|---|---|---|
| none | 54 | 54 accepted |
| `reject` | 109 | 102 rejected, 6 accepted, 1 unsupported |

The 6 accepted `reject` blocks:

- `classes/derived.md` line 471: a mixin base whose constructor is protected, made as a variable of its own,
- `classes/virtual.md` line 503: a class whose implicit destructor overrides a `final` destructor,
- `declarations/constexpr.md` line 283: a `consteval` constructor that reaches a `throw` in an initialization,
- `templates/arguments.md` line 530: a private nested class used as a template argument outside the class,
- `templates/dependent-names.md` line 743: a name of a dependent base used without `this->`,
- `templates/specialization.md` line 542: an explicit specialization of a member template inside a template that is not specialized.

I did not check that each of the 102 rejections names the rule that the block shows.

### 1.6 Records that drift

- `exercises.md` section 7 says that 18 pages have only a Predict batch that showed test names. R8 then ran without test names on six of those pages: constant evaluation, evaluation order, names, constraints, instantiation, and dependent names.
- 17 `:tests:` entries name a test whose `tidal-rules` header does not cite the rule. Two rules (CPY-28, CPT-23) have two tests by their `:tests:` line and one by the headers.
- `exercises.md` section 8 says that the W16 and W17 programs "build and run on GCC and Clang". The interpreter stops on 2 of the 4 programs, and no record says so.

## 2. Ranked work items

| # | Item | Evidence | Cost | Risk | Unblocks |
|---|---|---|---|---|---|
| 1 | Tests from the round 6 reductions and the round 4 and 5 sketches | 8 reductions of faults that no test covers; 0 whole sketches in `tests/` | small | low | a test that tracks each fault, as decision 0055 asks |
| 2 | A front-end round on the program faults, false verdicts first | 1 false `undefined`, 4 run stops (2 of them stop W16 and W17), and 2 missed rejects in 583 lines; 6 page examples accepted; 7 wrong run results | medium | low | trust in "result 0" and "undefined"; all 71 program tests run |
| 3 | The interpreter in the Write and Port prompts, and in the example check | W16 and W17 reported no stop, and the interpreter stopped on 2 of 4; `check_examples.py` runs no front end | small | low | exercises that find interpreter faults when they happen |
| 4 | Port suites P1 to P8 | 0 of 8 since phase 5; deferred by decisions 0054 and 0055 | one session per suite | low | the idioms a programmer brings from another language |
| 5 | Run tests where two pages meet | every fault in section 1.4 joins two pages, and each page's tests pass | one session per chapter pair | low | coverage of ordinary code between the unit tests and whole programs |
| 6 | Diagnose batches | 4 pages since phase 5; 2981 tests carry `tidal-error` patterns; 3 of my 4 mistakes reached me as a message | one session per 4 pages | low | a check that a message leads to the rule |
| 7 | LSUP-2 tests for the three-class demand | GCC and Clang accept one class, EDG and LSUP-2 do not, and no test shows the split | tiny | low | a programmer who copies one class learns the rule from a test |
| 8 | Guides, published | deferred since phase 5; two of the guide shapes are now page examples | one decision, then one session per guide | low | a way into the specification by task |
| 9 | Records that drift | section 1.6 | tiny | none | trust in the catalog |

### Item 1. Tests from the round 6 reductions and the earlier sketches

**What it is.** A clean-room writer adds tests for the reductions of section 1.4, under the rules they exercise:

- `delete_through_middle_base.cpp` and the form with a declared destructor, under VIRT-7 and VIRT-12, as run tests,
- `union_array_write_in_member_function.cpp` and the copy constructor form, under UNI-13, as run tests and in constant evaluation,
- `aggregate_member_converting_constructor.cpp`, under the aggregate initialization rules, as a run test,
- `assign_converted_owner.cpp`, under CPY-3 and CTOR-10, as a run test,
- `template_member_operator_outside.cpp`, under TPL-23 and the operator rules, as a run test,
- `member_template_signature_invalid.cpp`, under INST-6 and MPTR-8, as a reject test,
- the two LSUP-2 reductions (item 7).

The integrator also adds the correct whole sketches as program tests, in a directory such as `tests/programs/roadmap/`. These are the round 4 sketches that run, the round 5 `evaluator.cpp`, `hashmap_kept_pointer.cpp`, and `prelude.cpp`, and the round 6 `inventory.cpp`, `scheduler.cpp`, and `text.cpp`.

**Evidence.** Section 1.4. Decision 0055 says that the commit that acts on a roadmap round adds each correct sketch and reduction. RT added the reductions and none of the sketches. The round 5 `evaluator.cpp` still stops, and only the W16 program, written later, tracks the same fault.

**What it unblocks.** Each fault has a test before the front-end round starts, so `tools/frontend_check.py` shows each fix.

**Cost.** Small. Each reduction has at most 15 lines and builds on GCC, Clang, and EDG.

**Risk.** Low. The LSUP-2 tests need GCC and Clang overrides with a question.

### Item 2. A front-end round on the program faults

**What it is.** Front-end round 20 takes these targets, in this order:

1. the false VIRT-12 verdict on a delete through a middle class,
2. the missed rejects: MPTR-8 in a member template's declaration, LSUP-2 with fewer than three classes, and the 6 page examples of section 1.5,
3. the run stops: the union array written in a member function, the aggregate member built by a converting constructor, the assignment from a converted prvalue, and the class template's member operator defined outside the class,
4. the 7 wrong run results, starting with the two STG-29 tests with a member byte array (G139), since a container in raw storage keeps its bytes in a member array.

**Evidence.** Sections 1.3 to 1.5. A false `undefined` on correct code is the worst answer the interpreter can give, as round 5 said of OBJ-2. Phase 8 fixed that one, and VIRT-12 is the next. The two stops in W16 and W17 leave 2 of 71 program tests unrun.

**What it unblocks.** All 71 program tests run. A programmer can trust both "result 0" and "undefined".

**Cost.** Medium. Virtual dispatch of destructors, union activation, and aggregate initialization are central to the interpreter.

**Risk.** Low. Every run reduction builds clean on GCC and Clang with sanitizers, and EDG accepts each one.

### Item 3. The interpreter in the Write and Port prompts, and in the example check

**What it is.** Two changes:

- The Write and Port prompt of `exercises.md` section 9 adds a step after the compilers. The reader runs the program with `tidal-fe run`, checks it on EDG, and reduces each stop to a short program. A finding of kind "interpreter" goes to the next front-end round.
- `tools/check_examples.py` also runs `tidal-fe check` on each unmarked block and each `reject` block, and records a disagreement as a front-end failure in the way `tools/frontend_check.py` does.

**Evidence.** W16 and W17 each report an empty reductions directory, because no build or run stopped on the compilers. The interpreter stops on 2 of their 4 programs. Section 1.5 shows 6 page examples that the front end accepts, which `check_examples.py` cannot see.

**What it unblocks.** Item 2's targets arrive with each exercise, without a roadmap reader.

**Cost.** Small: a prompt paragraph and about 30 lines of script.

**Risk.** Low. A Write reader then reads interpreter messages that name rules. The messages come from Tidal's own front end, so they are a clean input.

### Item 4. Port suites P1 to P8

**What it is.** The eight Port suites of `exercises.md` section 3, one per persona, with the interpreter step of item 3.

**Evidence.** Port has waited since phase 5, and decisions 0054 and 0055 each deferred it. My `scheduler.cpp` and `text.cpp` are ports in practice. A string with a short buffer and a task heap are shapes that a programmer from another language rebuilds first (general knowledge), and each one found an interpreter fault. W16 and W17 filed 6 findings whose kind includes "needs example" (W16-2, W16-5, W16-6, W17-1, W17-4, W17-5), each about such an idiom.

**What it unblocks.** The exercise by use that decision 0055 asks of each phase, on a kind not yet run.

**Cost.** One session per suite.

**Risk.** Low. Port prompts must not copy known example programs (general knowledge).

### Item 5. Run tests where two pages meet

**What it is.** A clean-room writer writes short run tests for the joins between pages that ordinary programs use: virtual and derived classes, unions and member functions, aggregates and constructors, templates and operators, copy and move with conversions, and raw storage and classes. Each test cites a rule of each page. A first list comes from the chapter pairs that the program tests use most and that few tests cover with rules of both pages.

**Evidence.** Every fault of section 1.4 is a join. Each page's own tests pass, and the 71 programs reach 84 of 85 pages. The joins sit between those two levels. Decision 0055 deferred a construct count over `tests/programs/`. This item asks for a count of rule pairs over all tests, which is smaller.

**What it unblocks.** Tests find the faults that today only programs find.

**Cost.** One session per chapter pair, and a script that counts rule pairs per test.

**Risk.** Low. The tests are short and run on the matrix.

### Item 6. Diagnose batches

**What it is.** Diagnose batches for the chapters with the most error patterns, starting with templates, classes, and overloading. Each batch gives the compiler message and the `tidal-fe check` message.

**Evidence.** Diagnose has reached 4 pages. 2981 tests carry `tidal-error` patterns. Three of my four mistakes reached me as a compiler message, and the interpreter named the rule in two of them. A programmer meets the specification through a message more often than through a test (general knowledge).

**What it unblocks.** A measure of whether each message leads to the rule, for the compilers and for the front end.

**Cost.** One session per 4 pages.

**Risk.** Low.

### Item 7. LSUP-2 tests for the three-class demand

**What it is.** A clean-room writer adds reject tests for a defaulted `<=>` and for a built-in `<=>` with only `std::strong_ordering` declared. GCC and Clang get overrides that name a new question. The writer also checks `builtin-three-way.cpp` and `builtin-three-way-mixed.cpp` against LSUP-2, since they declare two of the three classes and expect acceptance.

**Evidence.** Section 1.4. The LSUP-2 Evidence states the EDG split twice. No test shows the split, and the interpreter accepts both reductions.

**What it unblocks.** A test that teaches the rule to a programmer who copies only the class that a use needs.

**Cost.** Tiny.

**Risk.** Low. The question may end with the maintainer, since GCC and Clang follow the other answer.

### Item 8. Guides, published

**What it is.** A maintainer decision on decision 0046 for the guides, then three how-to guides: a container in raw storage, a move-only owner, and a class hierarchy with a virtual destructor, a virtual base, and a clone.

**Evidence.** Rounds 3, 4, and 5 asked for guides. Phase 8 put the owner and the fixed container on the pages as examples, so a guide can now link them. The VIRT-12 case and the DER-13 mistake in section 1.4 are what a hierarchy guide would show.

**What it unblocks.** A way into the specification by task.

**Cost.** One decision, then one session per guide.

**Risk.** Low. A guide adds no facts (`guides/index.md`).

### Item 9. Records that drift

**What it is.** Fix the records of section 1.6:

- recount section 7 of `exercises.md` from the Predict rows, or have a tool write it,
- make each `:tests:` entry match its test's header, and add a check to `tools/check_specification.py`,
- add the interpreter result to the W16 and W17 rows of `exercises.md` section 8.

**Cost.** Tiny. **Risk.** None.

## 3. Out of scope

| Topic | Proposal | For | Against |
|---|---|---|---|
| I/O and the general library | Keep out. The closed C list of round 19 covers printing. | Programs print (general knowledge). | The library is outside the charter. |
| `typeid` | Keep deferred until the maintainer decides how to observe library-dependent properties. | `typeid` is core grammar, and a hierarchy program would use it (general knowledge). | Each test needs the library's `type_info`. EDG rejects a self-declared one (Q-0322). |
| The launder operation | Keep out until `pending.md` row 2 is decided. | A raw-storage container written the common way uses it (general knowledge). | The storage page now shows the kept-pointer form, which needs no library. |
| The preprocessor beyond `#include "file"` | Keep out. | `assert` is common (general knowledge). | High similarity risk. No program here needed it. |
| Threads, the memory model, coroutines, modules, extended floating-point types | Keep out. | | as the charter says |
| Language versions after C++23 | Keep out until `pending.md` row 7 is decided. | | |

## 4. Process changes

1. **Write and Port readers run the interpreter** (item 3). A program that stops in `tidal-fe` gets a reduction in the report, as a compiler stop does.
2. **A front-end round follows the last exercise that adds program tests.** In phase 8, round 19 ran before W16 and W17, so their two stops wait a whole phase.
3. **Whole sketches become tests,** under `tests/programs/`, beside their reductions. Decision 0055 asks for this, and RT added only the reductions.
4. **The example check runs the front end** (item 3).
5. **Each front-end round reruns the roadmap sketches,** as round 5 asked. With change 3, the program tests do this.
6. **Each exercise's status row records the interpreter result** beside the compilers.

## 5. Suggested order

Items 1, 7, and 9 come first, because each is small and item 2 needs their tests. Item 3 comes next, so that the exercises after it report interpreter stops. Item 2 then runs. Item 4 is the phase's exercise by use and runs beside item 2, and its reductions feed the round after. Item 5 can start once item 1 shows which joins fail. Items 6 and 8 run beside the others. Item 8 waits on the maintainer.

## 6. Doubts for the clean-room writers

- Whether the VIRT-12 verdict comes from the interpreter looking for the `virtual` keyword on the static class's own destructor. A delete through the root class runs, and a delete through any middle class fails, with or without a declared destructor there.
- Why a union array write inside a member function stops and the same write from `main` runs. UNI-13 names a chain of member accesses and subscripts, and I read an implicit `this->` as the start of such a chain.
- Whether LSUP-2 means three classes for every use of `<=>`, or only for the uses that EDG checks. `builtin-three-way.cpp` declares two classes and every compiler accepts it, because its `<=>` appears only in `decltype`.
- Whether a check of the page examples should also compare the rule that `tidal-fe` names with a rule that the block's text cites. I checked only accept and reject.
