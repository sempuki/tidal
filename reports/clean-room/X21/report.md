# X21: P15, declared initialization order across translation units

Date: 2026-11-04. Commit read: `926831c`. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone. P15 has a short description in `reports/clean-room/roadmap/designer.md` and a row in section 1.7 of `reports/clean-room/roadmap-6/designer.md`. The marker, the rules and the choice of class for a cycle are this report's own.

Inputs read:
- AGENTS.md, `documentation/style-guide.md` section 1, and `documentation/exercises.md` section 4.
- The output of `tools/propose_check.py`, run before any earlier Propose report was read. `reports/clean-room/X12/report.md`, as the model for this report's form.
- `reports/clean-room/roadmap-6/designer.md`: sections 1.4 to 1.7, and item 4. The P15 entry in `reports/clean-room/roadmap/designer.md`. The D30 rows of `reports/clean-room/roadmap-3/designer.md` and `roadmap-5/designer.md`. Item TE in `documentation/decisions/0055-phase-8.md`.
- In full: `specification/declarations/initialization/static.md`, `specification/declarations/storage-linkage.md`, `specification/declarations/inline.md`, `specification/foundations/odr.md`, and `specification/foundations/translation.md`.
- The cited rules of these pages: objects (OBJ-P2, OBJ-2, OBJ-4), unwinding (UNW-4, UNW-5, UNW-7, UNW-10), constexpr (CINIT-1), constant-evaluation (CE-24), keywords (KW-3), declarators (the Syntax segment), and the `simple_decl` and `init_declarator` productions of `declarations/index.md`. The glossary entry for "constant initializer".
- Q-0014 and Q-0175.
- Every test of SINIT-1 to SINIT-7, the STO-9 and STO-10 tests, `foundations/objects/static-constant-initialized-destroyed-in-place.cpp`, and the `tidal-rules` line of each of the 45 tests that expect `link-fail` on some compiler.
- The program tests under `tests/programs/`, through a scanner (section 4) and by hand for the hits.

Compiler runs: small programs built with g++ 16.2.1 and clang++ 22.1.8 (`-std=c++23 -pedantic-errors`, at `-O0` and, where stated, `-O2`), and `nm -C` on two object files. The probes are in `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X21/p`. The scanner is `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X21/scan_programs.py`, and its output is `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X21/scan_programs.out`. Each probe flushes its output before it can abort. The report gives the results where they support a claim. The harness did not let this subagent save `report.md`, so the integrator saves the text below.

Totals: 32 rule changes (5 new, 8 amended of which 1 is a Syntax segment, 19 unchanged but must state their effect). No existing test changes its expected outcome. 11 findings, X21-1 to X21-11.

## 1. Design

### 1.1 The feature

Today the order of dynamic initialization between translation units is unspecified (SINIT-3). An initializer that reads a variable of another unit gets its final value or zero, and on the matrix the answer follows the link order. Programs work around this with a block-scope `static` behind a function (SINIT-P2, STO-2) or with `constinit`.

P15 lets the definition of a variable list the variables that must be initialized before it. The list is an `after` clause after the declarator:

```cpp
// config.cpp
volatile int seven = 7;
int first = seven;                         // dynamic
int config = first * 2;                    // dynamic: 14

// table.cpp
extern int config;
int slots after(config) = config * 2;      // 28 in every link order
```

Today `int slots = config * 2;` gives 28 or 0. With unit `table.cpp` linked first, both compilers give 0 (probe `n_b.cpp` with `e_a.cpp`). With the list, `slots` is 28 in both link orders.

The W14 program `p4_init_order` keeps its counter as a block-scope `static` for this reason. Under P15 the counter can be an ordinary namespace-scope object:

```cpp
// counter.cpp
Counter the_counter;                       // constructor is not constexpr

// a.cpp
extern Counter the_counter;
int a_ticket after(the_counter) = the_counter.take();
```

The list names variables. The implementation does not check that the initializer reads only those variables, because reads through calls are not visible. The list is the programmer's statement of order.

### 1.2 The clause (new rule X21:new-2; amended Syntax, KW-3)

```text
init_declarator: declarator [order_list] [init]
               | declarator requires_clause
order_list:      "after" "(" id_expression {"," id_expression} ")"
```

`after` is an identifier, as `final` is (KW-3). The place after the declarator of a variable and before its initializer holds no identifier today. Both compilers reject `int x after(y) = 1;`, `int x after(y);` and `static inline int m after(y) = 1;` in a class (probes `s1.cpp` to `s3.cpp`). `after` stays usable as a name. `tests/programs/W7/date.cpp` defines a function `after`, and three tests use `after` as a variable or member name. All keep their meaning (probe `s5.cpp`).

X21:new-2 limits the clause. The clause can appear only on a definition of one of these:
- a variable at namespace scope,
- a static data member, defined outside its class or `inline` in its class.

These are rejected:
- a clause on a declaration that is not a definition (ODR-15), such as `extern int v after(x);`,
- a clause on a `thread_local`, `constexpr` or `constinit` variable,
- a clause in a block, on a non-static data member, or on a function,
- a name in the list that is not a variable with static storage duration declared before the clause, such as a function, a block-scope `static`, or a `thread_local` variable.

`constexpr` and `constinit` variables are constant-initialized (SINIT-1, CINIT-1), so no other initialization can come before them. A `thread_local` variable is initialized per thread (SINIT-6) and has no place in the program's start.

Only the definition carries the list. Other units declare the variable as today. An `inline` variable has a definition in each unit that uses it, and its list is part of the tokens that ODR-10 compares.

### 1.3 What the list promises (new rule X21:new-1; amended SINIT-P1, SINIT-P2, SINIT-3, SINIT-5, OBJ-4, ODR-3)

X21:new-1 states the order. Before the initialization of a variable `v` with a list, each variable `x` in the list has completed its initialization. So have the variables that SINIT-2 orders before `x`. If `x` is not initialized yet, the implementation initializes those variables and then `x` at that point, in the order of SINIT-2. They are not initialized again later.

The rule pulls the variables before `x` along with `x` so that SINIT-2 still holds in the unit of `x`. The initializer of `x` can read an earlier variable of its own unit and get its final value, as today. The author of `x`'s unit does not have to know that another unit names `x`.

The rule applies to each kind of `x` in this way:

| `x` is | What is initialized first |
|---|---|
| an ordinary variable or a static data member defined outside its class | the variables of `x`'s unit up to `x`, in definition order (SINIT-2, SINIT-7) |
| an `inline` variable | the `inline` variables that SINIT-2 orders before `x`, then `x` |
| a specialization of a template, such as `Holder<int>::v` | `x` alone, because SINIT-3 gives it no place |
| constant-initialized (SINIT-1) | nothing at run time. `x` has its value before any code runs |

Naming `x` in a list is a use of `x` (amended ODR-3). So `x` needs a definition (LNK-7, INL-6). A unit that names an `inline` variable defines it, so the unit sees the `inline` variables before it.

The other rules follow:
- **Destruction.** `x` completes its initialization before `v`. STO-10 then destroys `v` first. Probes `d1.cpp` and `d2.cpp` show that destruction across units already reverses the completed order, in both link orders, on both compilers, at `-O0` and `-O2`. A constant-initialized `x` takes the place before `v` for OBJ-4 (amended OBJ-4).
- **SINIT-3.** The cross-unit order stays unspecified, except where a list orders two variables (amended SINIT-3).
- **SINIT-4.** An early initialization before the program starts satisfies a list.
- **SINIT-5.** An implementation that defers a unit's initialization initializes the unit up to `x` when a list pulls `x` (amended SINIT-5).
- **Exceptions.** If the initializer of a pulled variable throws, UNW-5 ends the program, as for any namespace-scope initializer.
- **In one unit.** A list that names an earlier variable of the same unit changes nothing, unless one of the two is `inline` or a specialization (SINIT-3). A list that names a later variable is a cycle (1.4).

SINIT-P2 gains a third safe form in its table, a definition with a list. SINIT-P1's diagram gains one line in step 2.

A variable with a list is constant-initialized in one case, such as `int v after(x) = 3;`. The list then orders only the destruction of `v` (OBJ-4).

### 1.4 Cycles (new rules X21:new-3, X21:new-4, X21:new-5)

A variable `v` *waits for* `x` if the list of `v` names `x`, or if SINIT-2 orders `x` before `v`. A cycle is a chain of such steps that comes back to its start. A cycle can pass through the definition order of units that the list's author never sees:

```cpp
// one.cpp
extern int d;
int a after(d) = d + 1;    // a waits for d
int b = source();          // b waits for a (SINIT-2)

// two.cpp
extern int b;
int c after(b) = b + 1;    // c waits for b
int d = source();          // d waits for c (SINIT-2)
```

No list names a variable that names it back, yet `a`, `d`, `c` and `b` form a cycle.

The roadmap asks the design to choose between a `no-diagnostic` cycle and a new class for a diagnostic at link time. TR-4 states that Tidal has no class of rule that requires a diagnostic when the units are linked. P15 keeps that policy and makes the cross-unit cycle `no-diagnostic`. The rules are these:

- **X21:new-3 (`reject`).** A translation unit must not hold a cycle that uses only the lists of its own definitions and the SINIT-2 order of its variables that are neither `inline` nor specializations. A list that names its own variable, or a later variable of the same unit, is such a cycle. The compiler sees the whole cycle, as it sees a second definition under ODR-1.
- **X21:new-4 (`no-diagnostic`).** The program must not hold a cycle. An implementation need not diagnose a break. This is the split of ODR-1 and ODR-8, applied to order.
- **X21:new-5 (`recommended`).** When the initialization reaches a variable whose initialization is in progress through a list, an implementation should end the program with a message. An implementation can instead report the cycle when it links the units. TR-4 allows that report.

```cpp
int a after(b) = 1;        // rejected (X21:new-3): b comes later in this unit
int b = source();

inline int p after(q) = 1; // rejected: p and q name each other
inline int q after(p) = 2;
```

The run-time check costs one state for each unit: how far its initialization has gone, and which initializer is running. Block-scope statics keep the same kind of state today. Both compilers end a cross-unit recursion through two block-scope statics with `recursive_init_error` (probes `r1.cpp`, `r2.cpp`). Probes `c1.cpp` and `c2.cpp` write the cycle above by hand, with a progress state for each unit. The cycle is caught in both link orders on both compilers. The message names a different unit in each link order.

The reasons for `no-diagnostic` over a new class are in 1.6.

### 1.5 How an implementation can do it

A unit gives each of its namespace-scope variables with external linkage and dynamic initialization an entry. The entry runs the unit's initializers up to that variable, each one once. A unit with a list calls the entry of each named variable before the variable's own initializer. The unit's normal start-up calls the same entries, so nothing runs twice. Probes `e_a.cpp` and `e_b.cpp` write this by hand. `slots` is 28 in both link orders on both compilers.

The parts exist today. Probe `g1.cpp` and `nm -C` show these symbols on both compilers:
- a guard variable for an `inline` variable and for a template's static data member, the two cases SINIT-3 leaves unordered,
- for an `extern thread_local` variable, a weak reference to its "TLS init function" in the unit that uses it.

The weak reference handles a named variable that is constant-initialized in its own unit. That unit has no entry, the reference stays null, and the caller skips the call. An ordinary variable gets no guard and no entry today. Only the unit's start-up function `_GLOBAL__sub_I_...` exists.

### 1.6 Alternatives considered

- **A class for a diagnostic at link time.** Rejected, for these reasons:
  - TR-4 states the policy. Item TE of decision 0055 asked for a class or a stated policy, and TR-4 gives the policy.
  - A linker would need each unit's lists and definition order in the object file. No linker reads such data today.
  - EDG checks each unit and does not link. Tests could not check one of the three families.
  - The `link-fail` outcome of the harness already means one allowed result of a `no-diagnostic` rule (TR-4 Notes, 45 tests). A new class would give the same outcome a second meaning.
  - X21:new-5 lets an implementation report the cycle when it links, with no new class.
- **A `required` end at run time.** The program would always end when it reaches a cycle. Rejected. SINIT-4 lets an implementation initialize a variable of the cycle before the program starts, and then no initializer re-enters it. The required rule would have to forbid SINIT-4 for every variable that a list can reach, and a unit cannot know which those are. X21:new-5 keeps the check as advice.
- **An attribute, `[[after(x)]]`.** An implementation can ignore an attribute that it does not recognize (ATTR-1), and a standard attribute does not change what a valid program does (ATTR-P2). The list changes values that the program reads. Rejected.
- **Order inferred from the initializer.** The compiler sees the names in `config * 2`. The compiler does not see the read in `read_y()` (SINIT-2 Examples). Inference would also reorder programs that are valid today, with no edit (4.3). Rejected.
- **A priority number on each variable.** Numbers chosen by separate libraries have no common scale. Rejected.
- **Pull `x` alone.** Fewer cycles, because only lists make edges. But `x`'s initializer could then read zero from an earlier variable of its own unit, and SINIT-2 would break in a unit that nobody edited. Rejected.
- **The list on every declaration, with an agreement rule like STO-12.** The roadmap asked that every unit see the same list. Only the unit that defines `v` reads `v`'s list, and the entries of 1.5 carry the rest. ODR-10 covers `inline` definitions. Rejected as cost with no use.
- **Names of functions in the list.** The block-scope accessor of SINIT-P2 already gives that. Left out.
- **Structured bindings at namespace scope.** A structured binding is not an `init_declarator`, so it cannot carry a list. Left for later.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes. New rules use this exercise's ID space.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| X21:new-1 | new | none | required | Before a variable with a list is initialized, each named variable and the variables that SINIT-2 orders before it have completed initialization. A variable pulled early is not initialized again (1.3). | none |
| X21:new-2 | new | none | reject | Where a list can appear and what it can name (1.2). | none |
| X21:new-3 | new | none | reject | A cycle that one unit holds entirely, through its lists and the SINIT-2 order of its ordinary variables (1.4). | none |
| X21:new-4 | new | none | no-diagnostic | The program holds no cycle of lists and SINIT-2 order (1.4). | none |
| X21:new-5 | new | none | recommended | The implementation ends the program with a message when initialization reaches a cycle, or reports the cycle at link time (1.4). | none |
| `init_declarator` syntax (declarations/index.md) | amended | syntax | syntax | An optional `order_list` between the declarator and the initializer. | none |
| KW-3 | amended | required | required | `after` has its special meaning after the declarator of a variable, before the initializer. | none |
| SINIT-P1 | amended | principle | principle | Step 2 notes that a list can pull variables of other units first. | none |
| SINIT-P2 | amended | principle | principle | A definition with a list joins the table of safe forms. | none |
| SINIT-3 | amended | unspecified | unspecified | The order is unspecified unless a list orders the two variables. | none |
| SINIT-5 | amended | impl-defined | impl-defined | A list pulls a deferred unit's initialization up to the named variable. | none |
| OBJ-4 | amended | required | required | A constant-initialized object named in a list takes a place before the variable whose list names it. | none |
| ODR-3 | amended | required | required | A name in a list is a use. | none |
| SINIT-1 | unchanged, state effect | required | required | A constant-initialized named variable needs nothing at run time. | none |
| SINIT-2 | unchanged, state effect | required | required | The order in each unit holds, also when another unit pulls a variable early. | none |
| SINIT-4 | unchanged, state effect | unspecified | unspecified | An early initialization satisfies a list. | none |
| SINIT-6 | unchanged, state effect | impl-defined | impl-defined | `thread_local` variables take no list and cannot be named in one. | none |
| SINIT-7 | unchanged, state effect | required | required | A static data member defined outside its class is pulled with the variables before its definition. | none |
| STO-2 | unchanged, state effect | required | required | The block-scope accessor keeps working, and a list is an alternative. | none |
| STO-3 | unchanged, state effect | required | required | A variable not yet pulled still holds zero. | none |
| STO-9 | unchanged, state effect | undefined | undefined | A cycle through lists is X21:new-4, separate from a re-entered block-scope `static`. | none |
| STO-10 | unchanged, state effect | required | required | A named variable completes first, so it is destroyed after the variable that names it. | none |
| OBJ-P2 | unchanged, state effect | principle | principle | Static objects end in the reverse of a construction order that lists now constrain. | none |
| LNK-4 | unchanged, state effect | required | required | An `inline` variable is one entity, initialized once, whichever unit pulls it. | none |
| LNK-7 | unchanged, state effect | no-diagnostic | no-diagnostic | A named variable with no definition breaks LNK-7. | none |
| INL-6 | unchanged, state effect | no-diagnostic | no-diagnostic | A unit that names an `inline` variable defines it. | none |
| ODR-8 | unchanged, state effect | no-diagnostic | no-diagnostic | The list's use counts toward "uses the entity". | none |
| ODR-10 | unchanged, state effect | no-diagnostic | no-diagnostic | The list is part of the tokens of an `inline` definition. | none |
| TR-P1 | unchanged, state effect | principle | principle | No new class. | none |
| TR-4 | unchanged, state effect | required | required | X21:new-4 follows TR-4's policy for a break that one unit cannot see. | none |
| UNW-5 | unchanged, state effect | required | required | A pulled initializer that throws ends the program. | none |
| CINIT-1 | unchanged, state effect | reject | reject | X21:new-2 rejects a list on a `constinit` variable. | none |

Counts: 32 changes. 5 new, 8 amended (1 of them a Syntax segment), 19 unchanged but must state their effect. 0 withdrawn, 0 split. No existing test changes its outcome.

Every program that P15 affects has an `after` clause in a declarator, and no valid program has one today. The tests nearest the boundary keep their outcome:
- `declarations/initialization/static/across-units-order.cpp`, `inline-order-unspecified.cpp`, `template-member-unordered.cpp`: no list, so SINIT-3 still accepts both results.
- `declarations/initialization/static/accessor-across-units.cpp`: the accessor and `constinit` forms keep working.
- `declarations/storage-linkage/recursive-static-init.cpp` and `recursive-static-init-constructor.cpp`: STO-9 is unchanged.
- `classes/unions/anonymous-union-member-order.cpp`, `classes/unions/anonymous-union-mem-init-constructor.cpp`, `foundations/language-support/byte-self-declared.cpp`: `after` stays a name.

The proposal needs these new tests.

Under `tests/declarations/initialization/static/`:
- `after-across-units.cpp` (run; two units, `slots` of 1.1 is 28),
- `after-class-object.cpp` (run; a namespace-scope `Counter` in one unit, named by a list in another),
- `after-predecessors.cpp` (run; the pulled variable reads an earlier variable of its own unit and gets its final value),
- `after-inline.cpp` (run; `inline int derived after(base) = base + 1;` gives 8 in both link orders),
- `after-template-member.cpp` (run; a list names `Holder<int>::v`, and the trace has one order),
- `after-constant-named.cpp` (run; a list names a `constinit` variable),
- `after-destruction-order.cpp` (run; the named variable is destroyed last, in both link orders),
- `after-later-same-unit.cpp`, `after-self.cpp`, `after-inline-pair.cpp` (reject; X21:new-3),
- `after-not-definition.cpp`, `after-thread-local.cpp`, `after-constinit.cpp`, `after-block.cpp`, `after-names-function.cpp` (reject; X21:new-2),
- `after-cycle-across-units.cpp` (run-fail; the program of 1.4, citing X21:new-4 and X21:new-5).

Elsewhere:
- `foundations/keywords/after-identifier.cpp` (accept; `after` as a variable, a function and a member),
- `foundations/odr/after-list-is-use.cpp` (link-fail; a list names a variable that no unit defines, citing LNK-7).

## 3. Interactions

**Static initialization (SINIT-P1, SINIT-P2, SINIT-1 to SINIT-7).** P15 narrows SINIT-3 only where a list asks. SINIT-2 stays true in every unit, which is why X21:new-1 pulls the variables before `x`. Three gaps on the page touch the design. SINIT-3 claims "final value or zero" for a class member read from another unit (X21-1). SINIT-7 does not place an in-class `inline` static data member (X21-3). SINIT-5's "first use" has no definition (X21-6).

**Storage and objects (STO-2, STO-3, STO-9, STO-10, OBJ-P2, OBJ-4).** Destruction follows from STO-10 with no new rule, and the probes confirm it across units. OBJ-4 needs a sentence for a constant-initialized named variable. STO-10 and OBJ-4 already disagree about constant-initialized objects (X21-2), and P15 inherits that gap.

**Linkage and inline (LNK-4, LNK-7, INL-P1, INL-6).** A list can name a variable of another unit only through a declaration with external linkage. A name with internal linkage can be named in its own unit. Under INL-6 a unit that names an `inline` variable defines it, so the unit knows that variable's SINIT-2 predecessors.

**One-definition rule (ODR-3, ODR-8, ODR-10, ODR-15).** The list adds a use that is not an expression, so ODR-3 needs one item. ODR-15 decides what "a definition" is for X21:new-2.

**Translation (TR-P1, TR-P2, TR-4).** P15 adds no class. X21:new-3 and X21:new-4 split one fault between what one unit sees and what only the whole program shows, as ODR-1 and ODR-8 do. The policy that sends the second part to `no-diagnostic` is stated inside a `required` rule and not in TR-P1 (X21-4).

**Keywords and attributes (KW-3, ATTR-1, ATTR-P2).** `after` follows the model of `final`. The attribute rules rule out an attribute (1.6). GCC and Clang accept a GNU `asm` label in the same place, also under `-pedantic-errors` (probe `s4.cpp`). Tidal does not describe that extension. An implementation that has both would read the label first.

**Unwinding (UNW-5, UNW-10).** A pulled initializer that throws ends the program, as any namespace-scope initializer does. X21:new-5 is advice, so UNW-5's list does not grow.

**Not touched.** Block-scope statics, `thread_local` variables, constant evaluation, and the order inside one unit keep today's rules.

## 4. Impact on existing programs

The program tests under `tests/programs/` are Tidal's corpus of ordinary code, written by clean-room readers. The scanner finds 85 program files (71 from the Write suites and 14 programmer sketches under `roadmap/`), 10361 lines with headers and extra units. Five programs have more than one unit. The scanner classes each namespace-scope variable definition. A hand review of its "maybe dynamic" list leaves these dynamic initializations:

| Program | Variables | Reads another unit? |
|---|---|---|
| `W14/p4_init_order` | `a_ticket`, `b_ticket` | yes, through the block-scope accessor `counter()` |
| `W6/config_table.cpp` | `early` | no; one unit, reads a later `constinit` variable |
| `W6/bindings.cpp` | two structured bindings that copy | no |
| `W12/counter.cpp` | `c1`, `c2`, from a `constexpr` function | no |

So one program in the corpus initializes across units, and its reader wrote it to show the order problem. Two programs use a block-scope accessor (`W14/p4_init_order` and `W6/config_table.cpp`). No program has an `after` clause, which every compiler rejects today. One program names a function `after` (`W7/date.cpp`).

### 4.1 Rejected becomes valid

- `int v after(x) = ...;`, `int v after(x);`, `T v after(x, y){...};` at namespace scope.
- `static inline int m after(x) = ...;` in a class, and `int S::m after(x) = ...;` outside it.

Both compilers reject each form today (probes `s1.cpp` to `s3.cpp`).

### 4.2 Undefined becomes defined

None for an existing program. A program that adds a list gets these changes:
- An unspecified value becomes one value. `slots` of 1.1 is 28, where today it is 28 or 0.
- A read of a member of a class-type variable of another unit before its constructor runs is undefined today (OBJ-2, Q-0175, X21-1). With a list naming that variable, the constructor has completed, and the read is defined.

### 4.3 Valid programs that change meaning

None. Only definitions with a list change, and no valid program has one. A unit that a list reaches keeps its own order (SINIT-2), so the other units of an existing program keep their meaning when one unit adds a list.

A test of validity cannot observe a list. The clause is part of a namespace-scope declaration. A requires expression or a `decltype` operand holds no such declaration. So no detection code changes its answer.

Adding a list to an existing program is a change to the program. The change can create a cycle through the definition order of units the author did not edit (1.4). That program is then not valid under X21:new-4, with no diagnostic required.

### 4.4 Valid programs that become invalid

None. X21:new-2 and X21:new-3 apply only to definitions with a list. `after` stays an identifier everywhere else, and the one function and three test names that use it keep their meaning.

### 4.5 Evidence

- `s1.cpp` to `s3.cpp`: both compilers reject `after(y)` after the declarator of a namespace-scope variable, with and without an initializer, and in a class. `s5.cpp`: a function `after`, a variable `after_v` and a member `after` compile on both.
- `s4.cpp`: both compilers accept `int z asm("renamed_z") = 1;` under `-pedantic-errors`.
- `e_a.cpp`, `e_b.cpp`, `n_b.cpp`: without a list, `slots` is 0 when its unit is linked first and 28 otherwise, on both compilers. With the hand-written entry of 1.5, `slots` is 28 in both orders.
- `c1.cpp`, `c2.cpp`: the hand-written cycle of 1.4 ends with status 134 on both compilers and in both link orders. The reported position follows the link order.
- `r1.cpp`, `r2.cpp`: two block-scope statics in two units whose initializers call each other end with `recursive_init_error` and status 134 on both compilers.
- `g1.cpp` with `nm -C`: both compilers emit a guard variable for an `inline` variable and for `Holder<int>::v`, a weak "TLS init function" reference for an `extern thread_local` variable, and no guard for an ordinary variable.
- `d1.cpp`, `d2.cpp`: destruction across two units is the reverse of construction in both link orders, on both compilers, at `-O0` and `-O2`.
- `m1.cpp`: `inline int before`, a class with `static inline int m`, then `inline int after_v`, give the trace 123 on both compilers at `-O0` and `-O2`. `m2.cpp`: an `inline` variable that reads an earlier in-class `inline` member gets its final value.
- `scan_programs.out` holds the scanner's output.

## 5. Findings

**X21-1** (wrong; declarations/initialization/static.md; SINIT-3, SINIT-P2, SINIT-2, OBJ-2, Q-0175). SINIT-3 says that an initializer that reads a variable from another unit "gets its final value or zero". SINIT-P2 says the same. SINIT-2 says that a member of a class object has no value until the object's initialization gives it one, and the page's Example marks the same read in one unit as undefined. Across units, SINIT-3 claims zero for that read and OBJ-2 calls it undefined. No SINIT-3 test uses a class type. Suggestion: SINIT-3 and SINIT-P2 limit "final value or zero" to scalar variables and cite SINIT-2 and Q-0175 for class members. Add a cross-unit test with a class-type variable, handled as Q-0175's tests are.

**X21-2** (wrong; declarations/storage-linkage.md, foundations/objects.md; STO-10, STO-P1, OBJ-4). STO-10 says "The object whose initialization completed last is destroyed first." OBJ-4 says a constant-initialized object takes the place that a dynamic initialization at its definition would give it. In `foundations/objects/static-constant-initialized-destroyed-in-place.cpp`, `b` completes before `a`, and `b` is destroyed first. STO-10 read alone predicts the other order. Suggestion: STO-10 adds the OBJ-4 exception, or cites OBJ-4 for constant-initialized objects. STO-P1 has the same sentence.

**X21-3** (missing; declarations/initialization/static.md, declarations/inline.md; SINIT-2, SINIT-7, INL-4). SINIT-7 places a static data member defined outside its class at its definition. SINIT-2 orders `inline` variables by where they "come first" in each unit. Neither rule says where an `inline` static data member defined in its class sits. Both compilers place it at the class definition. Probe `m1.cpp` gives 123 for `inline` variable, class, `inline` variable, at `-O0` and `-O2`. Suggestion: a second sentence in SINIT-7, "An `inline` static data member defined in its class takes its place at the class definition", with a run test like `m1.cpp`.

**X21-4** (hard to find; foundations/translation.md; TR-4, TR-P1). TR-4 is a `required` rule. Its second paragraph states a policy about classes: Tidal has no class that requires a diagnostic at link time, and a rule whose break shows only at link time is `no-diagnostic`. The class table of TR-P1 does not state this policy. A designer who reads TR-P1 to choose a class for a new rule does not find it. Suggestion: move the policy sentence to TR-P1 after the class table, and keep the facts about linking in TR-4.

**X21-5** (missing; glossary.md, declarations/initialization/static.md, foundations/objects.md; SINIT-1, SINIT-2). "Dynamically initialized" and "dynamic initialization" occur on 11 lines of four pages (static, objects, additive, templates/declarations). "Constant-initialized" occurs on 9 lines of two pages. Each term is defined only in italics inside a rule (SINIT-2, SINIT-1). The glossary has "constant initializer" and neither of these terms. Suggestion: glossary entries for both terms, citing SINIT-1 and SINIT-2, and `{term}` links on the other pages.

**X21-6** (unclear; declarations/initialization/static.md; SINIT-5, SINIT-P2, ODR-3). SINIT-5 lets an implementation defer a unit's initialization "until the first use of a function or variable of the same translation unit". SINIT-P2 says "the first use of something in that unit". Neither cites ODR-3 for "use". Some uses are settled during translation, such as `int* p = &x;` in another unit, which is constant-initialized. The rule does not say whether such a use counts at the program's start. The matrix compilers do not defer (SINIT-5 Evidence), so no test can show the difference. P15 needs the answer, because a list is a use that happens at a point in the start-up. Suggestion: SINIT-5 cites ODR-3 and says that a use in a constant initializer counts before any dynamic initialization.

**X21-7** (needs example; declarations/storage-linkage.md; STO-10). Both STO-10 tests use one unit. No test shows that destruction across units reverses the link-order-dependent construction. Probes `d1.cpp` and `d2.cpp` show it on both compilers in both link orders, at `-O0` and `-O2`. Suggestion: a two-unit run test that checks the reverse order for whichever construction order happened.

**X21-8** (needs example; declarations/initialization/static.md, declarations/storage-linkage.md; SINIT-P2, STO-9). SINIT-P2 says that the initializer of an accessor's block-scope variable "must not call its own function again (STO-9)". The words read as a direct call. Two accessors in two units that call each other also re-enter a declaration. Both compilers end that program with `recursive_init_error` (probes `r1.cpp`, `r2.cpp`). The STO-9 tests use one unit. Suggestion: SINIT-P2 says "must not reach its own declaration again, directly or through other functions", and a two-unit `run-fail` test joins STO-9.

**X21-9** (outside the spec; Q-0007, roadmap-6 section 1.6). Three needs fall outside the spec:
- Version marking. P15 takes the place after a variable's declarator. A designer cannot tell from Tidal whether a later version gives that place a meaning.
- Mixed implementations. The entries of 1.5 must exist in every unit that defines a named variable. A unit built by an implementation without P15 has no entry. A weak reference then stays null, and the list silently gives today's order. Tidal has no model of object files from different implementations.
- Cost. One entry per external variable with dynamic initialization, and one progress state per unit. Tidal has no scale for this cost.

**X21-10** (outside knowledge). These come from general experience:
- GCC has an `init_priority` attribute with numbers, and some languages order module start-up by their imports and detect a cycle at run time. These shaped the alternatives in 1.6.
- The name `__cxa_guard_acquire` in the probe's symbol list belongs to the guard protocol of the common C++ ABI on these targets. The report uses only what `nm` printed.

**X21-11** (documentation; `documentation/exercises.md` section 4, roadmap-6 section 1.1). The table of Propose exercises in section 4 stops at X16. X17 to X20 exist, and X21 is now done. The brief's figure for the corpus (67 programs, about 7600 lines) is older than the corpus. The scanner finds 85 program files and 10361 lines, and roadmap-6 counts 71 programs without `roadmap/`. Suggestion: add X17 to X21 to the table, and let a counting script print the corpus size so that briefs quote it.
