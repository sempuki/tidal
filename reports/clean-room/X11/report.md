# X11: P14, scope-exit blocks

Date: 2026-11-03. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone.

Inputs read:
- AGENTS.md, and section 1 of `documentation/style-guide.md`.
- Section 4 of `documentation/exercises.md`.
- `reports/clean-room/X8/report.md`, as the model for this report's form.
- P14 in `reports/clean-room/roadmap/designer.md`. Section 1, the P14 row of section 2, and item 2 of `reports/clean-room/roadmap-3/designer.md`. The notes on D16 and D20 in the round-1 and round-2 designer reports.
- All of `statements/blocks.md`, `statements/jumps.md`, `statements/loops.md`, `exceptions/unwinding.md` and `foundations/objects.md`. The principles of `statements/switch.md`, `statements/range-for.md`, `exceptions/handlers.md` and `foundations/keywords.md`. The Syntax segment of `statements/index.md`.
- These rules on other pages: STO-1, STO-2, DTOR-4, EXS-4, TRY-6 to TRY-8, TRY-14, TRY-15, TRY-17, SW-5, IF-8 to IF-10, IF-17, CE-P1, CE-8, CE-31, CX-6, VC-9, FN-3, FN-8, TR-2, TR-P3, STG-21.
- The tests under `tests/statements/blocks/` (names), and the tests that JMP-4, JMP-6, JMP-9, JMP-16, OBJ-3, LOOP-1, LOOP-3, UNW-5, UNW-7, TRY-15 and SW-5 cite, as far as this report names them. A search of `tests/` for the identifier `defer`, which finds none.
- Q-0041 and Q-0044 (titles).

Compiler runs: 21 small programs built with g++ and clang++ (`-std=c++23 -pedantic-errors`) under the exercise's scratch directory (probes p1 to p21). Most use a guard object whose destructor calls a lambda, which is today's form of a scope-exit block. The report gives the results where they support a claim.

Totals: 67 rule changes (7 new, 22 amended, 38 unchanged but must state their effect). No existing test changes its expected outcome. 9 findings, X11-1 to X11-9.

## 1. Design

### 1.1 The feature

A *scope-exit statement* is `defer` followed by a block. The block is the *scope-exit block*. When control reaches the statement, nothing runs. The statement *registers* its block. When control later leaves the enclosing block, by any route, the scope-exit block runs.

```cpp
void copy_file(const char* from, const char* to) {
  File in = open(from);
  defer { close(in); }          // registered here, runs at every exit
  File out = create(to);
  defer { close(out); }
  if (!transfer(in, out)) return;   // runs close(out), then close(in)
  log("done");
}                                   // the same two blocks, in the same order
```

The model is a guard object (the Guard example in `exceptions/unwinding.md`). A scope-exit statement behaves as the declaration of an unnamed automatic object with a `noexcept` destructor whose body is the block. Every rule that ends automatic objects then gives the time and order of a scope-exit block. The differences from a guard object are these:
- The block names the enclosing variables directly. It needs no lambda, no capture, no class and no deleted copy.
- The block reads each variable when the block runs, not when it registers (X11:new-5).
- A jump must not leave or enter the block (X11:new-6, X11:new-7).
- A jump must not skip the statement, whatever the type of anything else (amended JMP-6). Q-0041 does not apply.

Registered blocks and automatic objects of one block end in one sequence, in the reverse of the order in which they registered or were constructed (amended OBJ-3):

```text
{
  T a;              1. a is constructed
  defer { A }       2. A registers
  T b;              3. b is constructed
  defer { B }       4. B registers
}                   exit: B runs, b ends, A runs, a ends
```

### 1.2 Syntax and the word `defer` (X11:new-1, X11:new-2, X11:new-3)

The Syntax segment of blocks.md gains one production, and `plain_stmt` in statements/index.md gains one alternative:

```text
scope_exit_stmt: "defer" compound_stmt
plain_stmt: ... | scope_exit_stmt
```

`defer` stays an identifier (amended KW-3). KW-P1 says that a late word has a special meaning only where a keyword could not appear, so programs that use the word as a name stay valid. Today `defer{...}` at the start of a statement is valid in exactly one case. `defer` names a type, a class template or an alias template, and the braces hold an initializer. Both compilers accept `defer{1};` for a class, for a class template through deduction, and for an alias of `int` (probes p1, p2, p10). A variable or a function named `defer` cannot be followed by `{` (probe p4: both compilers reject `defer {1};` for an `int` variable).

Under X11:new-2, a statement that starts with the identifier `defer` followed by `{` is a scope-exit statement, unless ordinary lookup of `defer` at that point finds a type, a class template or an alias template. If lookup finds one of these, the statement is read as it is today (STMT-6). In a template, unqualified lookup at the definition decides, so a type `defer` in a dependent base does not change the reading.

X11:new-3 limits the place of the statement. A scope-exit statement must be a statement of a compound statement, with or without labels before it. It must not be a branch of `if`, the body of a loop, or the body of a `switch`, written without braces. IF-8 and LOOP-4 make such a branch or body a scope of its own, so its block would run at once. A `defer` in that place is always a mistake.

### 1.3 How the block runs on each exit (X11:new-4, amended rules)

The enclosing block is the innermost compound statement that holds the scope-exit statement. The table gives each way control leaves it.

| Exit | When the registered blocks run | Rules |
|---|---|---|
| Control reaches the closing brace | At the brace, in reverse order with the block's automatic objects | OBJ-3, STO-1 |
| `break` | Before the loop or `switch` ends. A `for` initializer variable ends after them (LOOP-3) | JMP-1, JMP-9, LOOP-3 |
| `continue` | At the end of the body, before the iteration expression and the condition. A `for` or `while` condition variable ends after them (LOOP-3, X11-6) | JMP-2, OBJ-3 (X11-7) |
| `return` | After `return` has computed its value and its own temporaries have ended (VC-9). Before the parameters end (VC-16) | JMP-4 |
| `goto` out of the block | Before control reaches the label | JMP-9 |
| `goto` back to a point before the statement in the same block | The block runs, as JMP-9 destroys a variable. The statement registers again when control reaches it | JMP-9 |
| `goto`, `case` or `default` forward past the statement into its scope | Rejected | JMP-6, SW-5 |
| An exception | During unwinding, after the temporaries of the throwing full expression (UNW-9), before the handler runs | UNW-1 |
| End of a handler, by any route | Before the handled exception ends. The block can still rethrow inside its own try block (probe p15) | TRY-15 (X11-4, X11-8) |
| End of `main`, or `return` from `main` | Before the objects with static storage duration end | FN-8, OBJ-4, TR-2 |
| An exception with no handler | Implementation-defined, as for the automatic objects | UNW-7 |
| An exception that reaches a `noexcept` boundary | The same as for the automatic objects between the throw and the boundary. Tidal does not say what that is (X11-3) | UNW-5 |
| Program end through the library | Out of scope *(library referenced)* | |
| Constant evaluation | The same as a run | OBJ-3, CE-P1 |

A loop body registers again in each iteration, so a block in a loop body runs once per pass. Probe p8 uses a guard in a `for` body with `continue` in the first pass. Both compilers give the order guard, step, body, guard, step. Probe p12 runs a guard with a `constexpr` destructor under `static_assert`, with an early `break`, on both compilers.

Static locals are not affected. A scope-exit block is not an object and has no storage duration (STO-2, UNW-4).

**The returned value.** `return` computes its value first. A scope-exit block that changes the returned local changes what the caller sees only if the copy is elided (JMP-11). Probes p5 and p11 show both cases. With one return path, both compilers elide the copy, and the caller sees the change. With two return paths, neither compiler elides it, and the caller sees the old value. For a scalar the value is always taken before the block runs (probe p5, function `g`). The proposal keeps this behavior. Today a guard destructor has the same effect, and Tidal states it nowhere (X11-5).

### 1.4 Names in the block (X11:new-5)

A scope-exit block is a nested block at the place of the statement (STMT-2). The block sees the names declared before the statement, the parameters and `this`. Names declared after the statement are not in scope. The block uses each object as it is when the block runs. Nothing is copied at registration.

Every object that the block can name by its scope is still alive when the block runs, because those objects were constructed before the registration and end after the block (X11:new-4). A pointer to an object declared after the statement is a different case. That object has already ended, and an access through the pointer is undefined (OBJ-2).

### 1.5 A scope-exit block that jumps (X11:new-6, X11:new-7)

A jump must not leave a scope-exit block. The forms are:
- `return`,
- `break` or `continue` whose loop or `switch` is outside the block,
- `goto` to a label outside the block.

A jump must not enter a scope-exit block. A `goto`, `case` label or `default` label outside the block must not transfer control to a point inside it. The rule follows TRY-7.

A jump inside the block is valid, and so is a loop, a `switch` or a lambda inside the block. A lambda body is a different function (JMP-P1), so a `return` in a lambda inside a scope-exit block is valid. Labels belong to the whole function (JMP-5), so the reject rule must name labels inside the block. JMP-7 and JMP-14 already say the same for a lambda body, and the two rules gain the scope-exit block as a second boundary.

A destructor has no way to jump out of itself, and a scope-exit block has the same limit. A jump out has no meaning when the block runs during unwinding, and a `return` from a scope-exit block would replace a value that `return` has already computed.

### 1.6 A scope-exit block that throws (amended UNW-5, UNW-8)

An exception that leaves a scope-exit block ends the program (UNW-5). The rule holds on every exit, also when no other exception is in flight. The block behaves as a destructor that is `noexcept` by DTOR-4. Probe p13 shows the same for a guard today. A guard destructor that throws on a normal exit ends the program on both compilers.

A scope-exit block can throw and catch inside itself (UNW-6). It can call a function that does so. During unwinding, `throw;` inside the block finds no active handler and ends the program (TRY-8), unless the enclosing block is a handler (probe p15).

UNW-8 gains a case. An implementation should warn about a `throw` in a scope-exit block when no try block inside the scope-exit block can catch it.

The exception specification of the enclosing function does not change (EXS-4). No exception leaves a scope-exit block.

During constant evaluation a `throw` is rejected when the evaluation reaches it (CE-8), so the question does not arise there.

### 1.7 Alternatives considered

- **`defer` as a full keyword.** The grammar would be simpler. A program with a type named `defer` that writes `defer{...};` would become invalid, against KW-P1. Rejected.
- **A function-level list** of deferred blocks that run when the function returns. Registration in a loop would grow the list without bound and needs storage at run time. The block-level design reuses OBJ-3 unchanged. Rejected.
- **A skippable registration.** A forward jump past the statement would be valid, and the block would not run. Each statement would need a flag at run time. Q-0041 shows the compilers already disagree on the analogous case for objects. Rejected.
- **A scope-exit block that can throw when no other exception is in flight**, like a `noexcept(false)` destructor (UNW-6). On a `return`, the returned value already exists, and the rule would have to end it. The block would also make the function's exception analysis depend on its cleanup. Rejected. A program that must report a cleanup failure sets a variable in the block.
- **`return` in a scope-exit block replaces the returned value.** This hides the original value and has no analogue in a destructor. Rejected.
- **Copy the named values at registration.** The common use is cleanup of the current state, and the guard example in unwinding.md captures by reference. Rejected.
- **A `try { } finally { }` form.** It puts the cleanup far from the acquisition and needs a new level of nesting for each resource. Rejected.
- **A scope-exit statement as a branch without braces.** The block would run at once. Rejected (X11:new-3).
- **A library guard only.** Today's form works, with a class, a lambda, a deleted copy and a prvalue return (the unwinding.md example). The language form removes that code and settles the jump question, and keeps the same timing.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| X11:new-1 | new | none | syntax | `scope_exit_stmt: "defer" compound_stmt` in the Syntax segment of blocks.md (1.2). | none |
| X11:new-2 | new | none | required | `defer {` at the start of a statement is a scope-exit statement, unless ordinary lookup of `defer` finds a type, a class template or an alias template (1.2). | none |
| X11:new-3 | new | none | reject | A scope-exit statement must be a statement of a compound statement. It must not be a branch or a loop or `switch` body without braces (1.2). | none |
| X11:new-4 | new | none | required | Reaching the statement registers the block. Leaving the enclosing block by any route runs each registered block, in one reverse sequence with the block's automatic objects. Each pass registers again (1.1, 1.3). | none |
| X11:new-5 | new | none | required | The block sees the names declared before the statement and uses each object as it is when the block runs (1.4). | none |
| X11:new-6 | new | none | reject | No `return`, `break`, `continue` or `goto` leaves a scope-exit block (1.5). | none |
| X11:new-7 | new | none | reject | No `goto`, `case` or `default` enters a scope-exit block from outside (1.5). | none |
| `plain_stmt` syntax (statements/index.md) | amended | syntax | syntax | Gains `scope_exit_stmt`. | none |
| STMT-P1 | amended | principle | principle | A block can register cleanup that runs when the block ends. Says that the block's objects end at its exit (X11-1). | none |
| STMT-2 | amended | required | required | Points to OBJ-3 for the end of a block's objects and to X11:new-4 for registered blocks (X11-1). | none |
| OBJ-P2 | amended | principle | principle | The table gains a row: registered scope-exit blocks, run by their block, when control leaves the block by any route. | none |
| OBJ-3 | amended | required | required | Registered blocks run in the same reverse sequence as the automatic objects. | none |
| JMP-P1 | amended | principle | principle | A scope-exit block is a boundary that a jump cannot cross, like a lambda body. | none |
| JMP-P2 | amended | principle | principle | The table says that each case applies to registered blocks: a jump out runs them, a backward jump runs and re-registers, a forward jump past is rejected. | none |
| JMP-4 | amended | required | required | After the value and its temporaries (VC-9), the registered blocks run with the destruction of the locals, in one reverse order. | none |
| JMP-6 | amended | reject | reject | A jump must not skip a scope-exit statement whose block holds the target. No exception for this case. | none |
| JMP-7 | amended | reject | reject | The loop or `switch` of a `break` or `continue` in a scope-exit block must be inside that block. | none |
| JMP-9 | amended | required | required | A jump out of a block, or back before a scope-exit statement, runs the registered block. | none |
| JMP-14 | amended | reject | reject | A `goto` in a scope-exit block must name a label in that block, and a `goto` outside must not name a label inside. | none |
| SW-5 | amended | reject | reject | A transfer to a `case` or `default` label must not skip a scope-exit statement that registers for the label's block. | none |
| UNW-P1 | amended | principle | principle | Unwinding runs the registered blocks of each block it leaves. | none |
| UNW-1 | amended | required | required | The registered blocks run with the destruction, in the same reverse order, before the handler runs. | none |
| UNW-P3 | amended | principle | principle | A scope-exit block cannot let an exception out, as a destructor that is `noexcept` cannot. | none |
| UNW-5 | amended | required | required | Adds an item: an exception that leaves a scope-exit block. | none |
| UNW-7 | amended | impl-defined | impl-defined | The choice covers registered blocks too. | none |
| UNW-8 | amended | recommended | recommended | Warn about a `throw` in a scope-exit block that no try block inside the block can catch. | none |
| TRY-15 | amended | required | required | The registered blocks of the handler run before the handled exception ends (X11-4). Adds `continue` to the list (X11-8). | none |
| KW-P1 | amended | principle | principle | Names `defer` among the late words. | none |
| KW-3 | amended | required | required | `defer` is an identifier, with a special meaning only under X11:new-2. | none |
| STO-1 | unchanged, state effect | required | required | The model for registration: each pass through the statement registers again. | none |
| STO-2 | unchanged, state effect | required | required | A static local is not ended by a block exit, so no scope-exit block affects it. | none |
| OBJ-2 | unchanged, state effect | undefined | undefined | A scope-exit block that reaches an object declared after its statement, through a pointer, reads an ended object. | none |
| OBJ-4 | unchanged, state effect | required | required | The registered blocks of `main` run before the static objects end. | none |
| JMP-1 | unchanged, state effect | required | required | `break` runs the registered blocks of the blocks it leaves. | none |
| JMP-2 | unchanged, state effect | required | required | The registered blocks of the body run before the iteration expression (X11-7). | none |
| JMP-8 | unchanged, state effect | required | required | `goto` runs the registered blocks of the blocks it leaves (JMP-9). | none |
| JMP-11 | unchanged, state effect | unspecified | unspecified | Whether a scope-exit block's change to a returned local reaches the caller depends on this choice (X11-5). | none |
| JMP-12 | unchanged, state effect | reject | reject | A model for a jump limit. X11:new-6 is the general one for scope-exit blocks. | none |
| JMP-16 | unchanged, state effect | required | required | No change. A skipped variable cannot hide a skipped registration, because JMP-6 rejects the second. | none |
| LOOP-1 | unchanged, state effect | required | required | A `while` body's registered blocks run before the condition variable ends (X11-6). | none |
| LOOP-3 | unchanged, state effect | required | required | A `for` body's registered blocks run before the iteration expression and before the initializer variable ends. | none |
| RFOR-P2 | unchanged, state effect | principle | principle | A registered block in the body runs before the loop variable of that iteration ends. | none |
| IF-8 | unchanged, state effect | reject | reject | The reason for X11:new-3: a branch without braces is its own scope. | none |
| IF-17 | unchanged, state effect | reject | reject | A scope-exit statement in a discarded branch never registers. | none |
| TRY-7 | unchanged, state effect | reject | reject | The model for X11:new-7. | none |
| TRY-8 | unchanged, state effect | required | required | `throw;` in a scope-exit block that runs during unwinding finds no active handler. | none |
| TRY-17 | unchanged, state effect | reject | reject | A model for a jump limit. | none |
| UNW-2 | unchanged, state effect | required | required | A scope-exit block in a constructor body runs before the completed members end. | none |
| UNW-4 | unchanged, state effect | required | required | Unwinding does not touch static objects, and a scope-exit block is not one. | none |
| UNW-6 | unchanged, state effect | required | required | A scope-exit block can throw and catch inside itself. | none |
| UNW-9 | unchanged, state effect | required | required | The temporaries of the throwing full expression end before the registered blocks run. | none |
| UNW-10 | unchanged, state effect | impl-defined | impl-defined | Applies to the termination that the new UNW-5 item causes. | none |
| VC-9 | unchanged, state effect | required | required | The temporaries of a `return` expression end before the registered blocks run (probe p14). | none |
| VC-16 | unchanged, state effect | required | required | Parameters end after the registered blocks of the function body, so a block can use them. | none |
| DTOR-4 | unchanged, state effect | required | required | The model for the `noexcept` behavior of a scope-exit block. | none |
| EXS-4 | unchanged, state effect | required | required | A scope-exit block does not change the exception specification of its function. | none |
| FN-3 | unchanged, state effect | undefined | undefined | A function that returns a value and reaches its end is still undefined. The registered blocks do not make it defined. | none |
| FN-8 | unchanged, state effect | required | required | The end of `main` runs the registered blocks, as `return 0;` does. | none |
| TR-2 | unchanged, state effect | required | required | A scope-exit block in `main` cannot change the status, because it cannot `return`. | none |
| CE-P1 | unchanged, state effect | principle | principle | Constant evaluation runs the registered blocks as a run does. | none |
| CE-8 | unchanged, state effect | reject | reject | A `throw` in a scope-exit block that the evaluation reaches is rejected. | none |
| CE-31 | unchanged, state effect | reject | reject | Termination from a scope-exit block cannot occur in constant evaluation. | none |
| CX-6 | unchanged, state effect | reject | reject | A `goto` inside a scope-exit block is rejected when constant evaluation reaches it. | none |
| STG-21 | unchanged, state effect | undefined | undefined | A scope-exit block that ends a variable explicitly makes the later implicit destruction undefined, as any code does. | none |
| STMT-5 | unchanged, state effect | required | required | A label can stand before a scope-exit statement. | none |
| STMT-6 | unchanged, state effect | required | required | Gives the reading of `defer{...};` when X11:new-2 does not apply. | none |
| STMT-8 | unchanged, state effect | reject | reject | A scope-exit statement declares no name, so it cannot clash. | none |

Counts: 67 changes. 7 new (one of them a Syntax production), 22 amended (one of them a Syntax segment), 38 unchanged but must state their effect. 0 withdrawn, 0 split. No existing test changes its outcome.

No test in `tests/` uses the identifier `defer`. The tests nearest the boundary keep their outcome:
- `statements/jumps/return-destroys-locals.cpp`, `goto-out-destroys.cpp`, `goto-back-recreates.cpp`: no scope-exit statement.
- `statements/jumps/goto-skips-nontrivial-destructor.cpp`: Q-0041 is unchanged for objects.
- `statements/switch/jump-past-block.cpp`, `case-skips-nontrivial-destructor.cpp`: SW-5 is unchanged for objects.
- `exceptions/unwinding/leaves-implicit-noexcept-destructor-terminates.cpp`, `no-handler-unwinding-unobservable.cpp`: no scope-exit statement.
- `statements/blocks/ambiguous-declaration.cpp`: STMT-6 is unchanged.

The proposal needs these new tests under `tests/statements/blocks/`:
- `scope-exit-end-of-block.cpp` (run; order with two objects and two blocks),
- `scope-exit-break.cpp`, `scope-exit-continue.cpp` (run; the order of p8),
- `scope-exit-return.cpp` (run; after the value and its temporaries, as p14),
- `scope-exit-returned-local.cpp` (run; accepts either value, as p5 and p11 show),
- `scope-exit-goto-out.cpp`, `scope-exit-goto-back.cpp` (run; the order of p18),
- `scope-exit-goto-forward-past.cpp`, `scope-exit-case-past.cpp` (reject; JMP-6, SW-5),
- `scope-exit-exception.cpp` (run; runs before the handler),
- `scope-exit-in-handler-rethrow.cpp` (run; the case of p15),
- `scope-exit-throws-terminates.cpp` (run-fail; UNW-5),
- `scope-exit-throw-caught-inside.cpp` (run; UNW-6),
- `scope-exit-return-inside.cpp`, `scope-exit-break-out.cpp`, `scope-exit-goto-out-of-block.cpp`, `scope-exit-goto-into-block.cpp` (reject; X11:new-6, X11:new-7),
- `scope-exit-lambda-return.cpp` (run; a `return` in a lambda inside the block),
- `scope-exit-branch-without-braces.cpp` (reject; X11:new-3),
- `scope-exit-type-named-defer.cpp` (run; `defer{1};` with a class `defer` keeps today's meaning),
- `scope-exit-variable-named-defer.cpp` (run; an `int` named `defer` in scope does not block the statement),
- `scope-exit-main-before-statics.cpp` (run; OBJ-4),
- `scope-exit-constant.cpp` (accept; under `static_assert`, as p12),
- `scope-exit-throw-warns.cpp` (warning; UNW-8).

## 3. Interactions

**Blocks (STMT-P1, STMT-2, STMT-3, STMT-5, STMT-6, STMT-8).** The scope-exit statement is the first statement whose effect comes at the end of its block. blocks.md says nothing about the end of a block today. STMT-2 points to STO-1 and STO-2 only for the creation of variables (X11-1). STMT-6 gives the old reading of `defer{...};` when a type is in scope.

**Objects (OBJ-P2, OBJ-3, OBJ-4, OBJ-2, STO-1).** The guard model lets OBJ-3 give every timing. OBJ-3 and STO-1 speak of "their block". IF-8 and LOOP-4 make a branch or body without braces a scope, and an object declared there ends at the end of that branch (probe p20). The rules do not say so (X11-2). X11:new-3 keeps scope-exit statements out of such places, so the proposal does not depend on the answer.

**Jumps (JMP-P1, JMP-P2, JMP-1, JMP-2, JMP-4, JMP-6, JMP-7, JMP-9, JMP-11, JMP-14).** The table of JMP-P2 carries over row by row. JMP-6 needs a separate item, because its exception for trivial types does not fit a statement with no type. `continue` is not stated as an exit from the body's blocks (X11-7). JMP-4 and JMP-11 together decide what the caller sees when a scope-exit block changes a returned local (X11-5). Tidal has no general notion of a region that a jump must not cross. The limits are spread over JMP-7, JMP-14, TRY-7, TRY-17, JMP-12 and IF-17 (X11-9).

**Loops (LOOP-1, LOOP-3, RFOR-P2).** LOOP-3 fixes the order for a `for` condition variable. LOOP-1 does not fix it for `while` (X11-6). Q-0044 records that GCC 14 differs for `for`. A scope-exit block in the body runs before the condition variable on both orders, because the body ends first in both.

**Switch (SW-P2, SW-5).** A scope-exit statement at the top level of a `switch` body is valid only if no `case` or `default` label follows it. A block inside a `case` keeps the statement out of the jump's way, as `jump-past-block.cpp` does for a variable.

**Unwinding and handlers (UNW-P1, UNW-P3, UNW-1, UNW-5 to UNW-9, TRY-8, TRY-15).** The registered blocks run during unwinding, as destructors do. In a handler they run while the handled exception is still current (probe p15), and TRY-15 does not state that order (X11-4). TRY-15 also omits `continue` (X11-8). An exception that reaches a `noexcept` boundary ends the program, and Tidal does not say whether the objects between the throw and the boundary end first (X11-3). Neither compiler ends them (probes p6, p17). The registered blocks follow the objects.

**Constant evaluation (CE-P1, CE-8, CE-31, CX-6, OBJ-3).** OBJ-3 already covers constant evaluation. A scope-exit block runs there unchanged.

**Keywords (KW-P1, KW-3).** D20 is closed. KW-1 lists the keywords, and `defer` is not one. KW-3 lists the late identifiers. X11:new-2 is the first late word whose special position can also hold a type name. The lookup test keeps KW-P1 true.

**Coroutines.** OBJ-3 and UNW-P1 now say that the rules assume a body runs without suspension. The proposal inherits that assumption. A scope-exit block across a suspension is out of scope.

## 4. Impact on existing programs

Tidal has no corpus of real programs, so the claims below come from reasoning about the rules and from compiler runs.

### 4.1 Rejected becomes valid

- `defer { ... }` where lookup of `defer` finds nothing. Both compilers reject it today with "undeclared identifier" (probe p3).
- `defer { ... }` where `defer` names a variable, a function or a namespace. Both compilers reject `defer {1};` for an `int` variable today (probe p4).

### 4.2 Undefined becomes defined

None. The proposal touches no `undefined` rule.

### 4.3 Valid programs that change meaning

None. `defer{...};` with a type, a class template or an alias template named `defer` keeps its meaning (probes p1, p2, p10).

One later change to a program can change meaning. If a program declares a type named `defer` in a scope that encloses an existing scope-exit statement, the statement is read as an expression statement. In almost every case the block's content is not a valid initializer, or no `;` follows, so the program becomes invalid and the change is visible. The silent case is `defer {};`, an empty block followed by a null statement. That form does nothing in either reading.

### 4.4 Valid programs that become invalid

None. Each new reject rule (X11:new-3, X11:new-6, X11:new-7, and the items in JMP-6, JMP-7, JMP-14 and SW-5) covers only programs that contain a scope-exit statement.

### 4.5 Evidence

- Probes p1, p2, p10: both compilers accept `defer{1};` for a class, a class template and an alias of `int` named `defer`. Clang warns that the result is unused.
- Probes p3, p4: both compilers reject `defer { ... }` with no declaration and with an `int` variable named `defer`.
- Probe p8: a guard in a `for` body with `continue` gives guard, step, body, guard, step on both compilers.
- Probe p12: a guard with a `constexpr` destructor runs under `static_assert`, with and without `break`, on both compilers.
- Probe p13: a guard destructor that throws on a normal exit ends the program on both compilers, with a handler around the block.
- Probe p14: the temporary of a `return` expression ends before the locals and the guard (trace 7251 on both).
- Probes p5 and p11: the caller sees a guard's change to the returned local with one return path (copy elided) and does not see it with two return paths (copy made). Both compilers agree.
- Probe p15: in a handler, a guard's `throw;` inside its own try block rethrows the handled exception on both compilers.
- Probe p16: a `while` condition variable ends after the body's guard in each iteration on both compilers.
- Probes p6 and p17: an exception that reaches a `noexcept` function ends the program without destroying the objects between the throw and the boundary, on both compilers, with and without an outer handler.
- Probe p18: a backward `goto` past a guard runs the guard before the guard is constructed again, on both compilers.
- Probe p19: a `case` label past a guard is rejected by both compilers.
- Probe p20: an object declared in a branch without braces, and in a loop body without braces, ends at the end of the branch or body.
- Probe p21: `continue` from a handler in a loop ends the handler's object, on both compilers.

## 5. Findings

**X11-1** (hard to find; statements/blocks.md; STMT-P1, STMT-2, STMT-3, OBJ-3). blocks.md says when a block's variables are created (STMT-2 points to STO-1 and STO-2) and that statements run in order (STMT-3). blocks.md says nothing about what happens when control leaves a block. A reader of the blocks page cannot find that the block's objects end at the closing brace, in reverse order, on every exit. STMT-P1 does not cite OBJ-P2. Suggestion: one sentence in STMT-2, "The objects of the block end when control leaves the block ({rule}`OBJ-3`)", and a citation of OBJ-P2 in STMT-P1.

**X11-2** (unclear; foundations/objects.md, declarations/storage-linkage.md, statements/if.md, statements/loops.md; OBJ-3, STO-1, IF-8, LOOP-4). OBJ-3 and STO-1 end an automatic object when control leaves "their block". IF-8 and LOOP-4 make a branch or loop body without braces a scope, and such a scope is not a compound statement. `if (c) T t;` declares `t` in a scope with no block. Probe p20 shows that both compilers end `t` at the end of the branch, and end an object in a loop body without braces at the end of each pass. Suggestion: OBJ-3 and STO-1 say "their block, or the branch or body that IF-8 and LOOP-4 make a scope", or the glossary defines "block" to cover these scopes. Add a run test with a branch without braces.

**X11-3** (missing; exceptions/unwinding.md; UNW-5, UNW-7, UNW-P3). UNW-7 makes it implementation-defined whether the automatic objects are destroyed when an exception has no handler. UNW-5 also ends the program when an exception reaches a `noexcept` function. No rule says whether the objects between the throw and that boundary are destroyed first. Probes p6 and p17 put an object in the throwing function and one in the `noexcept` function. Neither compiler destroys either object, with or without a handler outside the `noexcept` function. Suggestion: extend UNW-7 to every case of UNW-5 in which an exception is still in flight, and add a test like `no-handler-unwinding-unobservable.cpp` for the `noexcept` boundary.

**X11-4** (unclear; exceptions/handlers.md; TRY-15, TRY-P3, TRY-P4, TRY-8). TRY-15 says that "the local objects of the handler then end ... and the exception that the handler was handling is over". TRY-P4 says that leaving the handler by any route ends the handled exception. Neither text says that the objects end first, while the exception is still current. Probe p15 shows that a destructor of a handler's local can rethrow the handled exception inside its own try block on both compilers. Suggestion: TRY-15 states the order, and a run test rethrows from a destructor of a handler's local.

**X11-5** (unclear, needs example; statements/jumps.md; JMP-4, JMP-11, JMP-P2). JMP-4 says that `return` "first evaluates the returned value" and then destroys the locals. JMP-P2 says the variables are destroyed "after `return` has taken its value". When the copy of a named local is elided (JMP-11), the local is the result object. A destructor of another local that writes the returned local then changes what the caller sees. Probes p5 and p11 show the caller seeing the change with one return path and not seeing it with two, on both compilers. For a scalar the value is always taken first. The text of JMP-4 reads as if the value were fixed in every case. Suggestion: a note in JMP-11 that the choice is observable through a destructor that runs after the value is taken, an Examples entry, and a run test that accepts either value.

**X11-6** (unclear; statements/loops.md; LOOP-1, LOOP-3, LOOP-P2). LOOP-3 says that a `for` condition variable is destroyed after the iteration expression. LOOP-1 says only that a `while` condition variable "is created and destroyed in each iteration". LOOP-1 does not say whether it ends before or after the objects of the body. Probe p16 shows that the body's objects end first on both compilers, and then the condition variable. `while-condition-declaration.cpp` counts creations and destructions and cannot see the order. Suggestion: one sentence in LOOP-1 and an order test like `for-condition-destroyed-after-step.cpp`.

**X11-7** (unclear; statements/jumps.md; JMP-2, JMP-9, JMP-P2). JMP-2 says that `continue` "goes to the end of the body". JMP-9 and the JMP-P2 table speak of "a jump out of a block". `continue` from a nested block leaves that block, and it ends the body's block for this iteration. No rule says that the objects of those blocks end before the iteration expression. OBJ-3 covers "each iteration of a loop" but not the `continue` path. Probe p8 shows the order on both compilers. No test of JMP-2 uses an object with a destructor. Suggestion: JMP-2 says that the objects of each block that `continue` leaves, and of the body, end before the iteration expression. Add a run test.

**X11-8** (missing; exceptions/handlers.md; TRY-15). TRY-15 lists `return`, `break` and `goto` as the ways a handler ends early. The list omits `continue`. A handler in a loop body can `continue`, and probe p21 shows that the handler's object then ends on both compilers. Suggestion: add `continue` to the list and a run test like `goto-out-of-handler.cpp`.

**X11-9** (hard to find; statements/jumps.md, exceptions/handlers.md, statements/if.md; JMP-P1, JMP-7, JMP-12, JMP-14, TRY-7, TRY-17, IF-17). Tidal has no single statement of the regions that a jump cannot cross. The limits are spread over seven rules on three pages:
- a lambda body, in both directions (JMP-7, JMP-14),
- a try block or handler, entry only (TRY-7),
- a branch of `if constexpr` or `if consteval`, entry only (IF-17),
- a constructor's handler, `return` only (TRY-17),
- a constructor or destructor, `return` with a value (JMP-12).

The round-1 designer note on P14 named only TRY-17 and JMP-12, and missed TRY-7, IF-17 and the lambda rules. A designer who adds a new boundary must find all of them by search. Suggestion: a table in JMP-P1 with one row per region and columns for entry, exit, the jumps covered, and the rule.
