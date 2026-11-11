# X24: P22, labeled `break` and `continue`

Date: 2026-11-11. Commit read: `5f0ffdd9`. Version: C++23 only. Kind: Propose (decision 0064, item E). Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone. P22 is in `reports/clean-room/roadmap-7/portfolio.md`. The rules below are this report's own.

The reader returned this report as a message, and the integrator saved it here with the reader's files.

P22 lets a `break` or `continue` name a label. `break rows;` ends the loop or `switch` that the label `rows` marks. `continue rows;` ends the current pass of the loop that `rows` marks, and the loop goes on as after a normal pass. The design settles the portfolio's three open points:
- A label names the loop or `switch` that it marks directly, after any further labels and attribute lists. A label on a block or on another statement names nothing that a jump can act on (1.3).
- A labeled `continue` must name a loop. It cannot name a `switch` (1.4).
- A `break` or `continue` inside a lambda body sees only the labels of the lambda body. A label of the enclosing function is never found, so the jump cannot leave the lambda body. The same holds inside the member functions of a class defined inside a function (1.5).

P22 adds no keyword and no token. JMP-17 rejects every program that has the new form today, because the grammar has no place for it (TR-12). The proposal changes the outcome of two tests, from `reject` to `accept`. No valid program changes meaning or becomes invalid, and no program under `tests/programs/` changes. Sections 4.3 and 4.4 give the reasons in full.

Open questions that bear on P22: none that the jump pages cite as open for this form. Q-0041 and Q-0121 (JMP-6, JMP-16) concern jumps that skip a declaration. A labeled jump only leaves scopes, so neither question arises for it (1.6, X24-2).

Inputs read:
- AGENTS.md, CLAUDE.md, `documentation/exercises.md` sections 4 and 9, decision 0064 (item E), and row 7 of `documentation/decisions/pending.md`.
- The output of `python3 -I tools/propose_check.py` and `python3 -I tools/propose_check.py --corpus`, run before any earlier Propose report was read. Also the docstring and the conflict check of `tools/propose_check.py`, to learn which change kinds the tool reads. That file is outside the brief's list of readable files. It holds no specification text.
- `reports/clean-room/X25/report.md` as the model for this report's form. X24 relies on no claim of X25. The P22 entry of `reports/clean-room/roadmap-7/portfolio.md`. `frontend/DESIGN-rewrite.md` section 7.
- In full: `specification/index.md`, `specification/statements/index.md`, `statements/jumps.md`, `statements/loops.md`, `statements/blocks.md`, and `statements/switch.md`.
- The start of `specification/foundations/translation.md` (TR-P1 and TR-12), and the glossary entry "discarded statement".
- In `statements/range-for.md`: RFOR-P1, RFOR-1, RFOR-3, and RFOR-4. In `statements/if.md`: IF-P2, the principle on `if constexpr`, and IF-17.
- In `foundations/constant-evaluation.md`: CE-P1 and CE-17. In `foundations/objects.md`: the table of OBJ-P1 and OBJ-3. In `foundations/names.md`: NAM-P4.
- In `exceptions/handlers.md`: TRY-7 and TRY-15. In `declarations/attributes.md`: the place table of ATTR-P1, ATTR-7, and ATTR-8. In `declarations/functions/definitions.md`: FN-7 and its Evidence notes. In `templates/dependent-names.md`: DEP-28, DEP-29, DEP-30, and DEP-31.
- The rule tests that section 4 names, and the program `tests/programs/W3/nested.cpp`, which the scan names.

Compiler runs: small programs built with g++ 16.2.1 and clang++ 22.1.8 (`-std=c++23 -pedantic-errors`). The probes are in `reports/clean-room/X24/p/`. The scan is `scan_jumps.py` in `reports/clean-room/X24/`, with its output in `scan_jumps.out`. No compiler implements P22 in C++ mode. Both reject the two JMP-17 tests, and Clang says that the named form is "only supported in C2y". The design does not use that C mode. EDG was not run. The harness did not let this subagent save `report.md`, so the integrator saves the text. The change table is also in `/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/X24-check/table-for-check.md`, with shortened descriptions. `python3 -I tools/propose_check.py --strict` reads 33 rows there with 0 conflicts and 0 warnings.

Totals: 33 rule changes (3 new, 12 amended, 1 withdrawn, 17 unchanged but must state their effect). One amended row is the grammar of `jump_stmt` and one is a glossary entry. 2 existing tests change their expected outcome, from `reject` to `accept`. No program under `tests/programs/` changes, out of 152. 8 findings, X24-1 to X24-8.

## 1. Design

### 1.1 The feature

JMP-P1 lists three ways to leave two loops at once: a `goto` to a label after the outer loop, a `return` from a function that holds the loops, and a flag that every loop condition tests. The `goto` form fails in constant evaluation (CE-17). The flag form adds a test to each condition. The `return` form needs a function of its own. Under P22 the search reads as follows:

```cpp
constexpr int find(const int (&grid)[3][4], int target) {
  int at = -1;
rows:
  for (int r = 0; r < 3; ++r)
    for (int c = 0; c < 4; ++c)
      if (grid[r][c] == target) {
        at = r * 4 + c;
        break rows;                 // ends the outer for (X24:new-3)
      }
  return at;
}

int sum_rows(const int (&grid)[3][4]) {
  int sum = 0;
rows:
  for (int r = 0; r < 3; ++r) {
    for (int c = 0; c < 4; ++c) {
      if (grid[r][c] < 0) continue rows;   // skip the rest of row r
      sum += grid[r][c];
    }
  }
  return sum;
}
```

`find` is valid in constant evaluation. A labeled jump is not a `goto`, so CE-17 does not reject it (1.6).

### 1.2 Syntax (amended `jump_stmt`, withdrawn JMP-17)

The production `jump_stmt` of `statements/jumps.md` gains an optional label name after each of the two keywords:

```text
jump_stmt: "break" [identifier] ";"
         | "continue" [identifier] ";"
         | "return" [expression | braced_init] ";"
         | "goto" identifier ";"
```

The identifier is a label name. The search for it looks only at labels, which have their own name space (NAM-P4). A variable or a type with the same name plays no part. The form needs no new token. `break` or `continue` followed by an identifier derives from no production today, so the extension makes no text ambiguous.

JMP-17 has nothing left to state, so it is withdrawn. Its two tests change from `reject` to `accept` (4.1). X24:new-1 and X24:new-2 take over the limits on the label.

The rest of this report calls `break L;` and `continue L;` *labeled jumps*, and calls `L` the jump's label.

### 1.3 What the label names (new rule X24:new-1; amended JMP-5, JMP-7)

In `labeled_stmt: label statement`, the label *marks* the statement after it. When that statement is itself a labeled statement or starts with attribute lists, the label marks the statement at the end of the chain. So in the text below, `first` and `second` both mark the `for`:

```cpp
first:
second:
  [[likely]] for (int i = 0; i < 3; ++i) { /* ... */ }
```

A `case` or `default` label in the chain is skipped in the same way. A label at the end of a block (STMT-5) marks no statement.

X24:new-1 says that a labeled jump is valid only if all of these hold:
- The label is a label of the jump's own function (JMP-14, 1.5).
- The statement that the label marks is a loop (`while`, `do`, `for`, or a range-based `for`) or a `switch`.
- The jump is inside the body of that statement.

A label on a block, on an `if`, or on an expression statement names nothing that a labeled jump can act on. A label on a loop that does not contain the jump, such as an earlier loop or a sibling loop, is not valid either. A label that names the innermost loop is valid, and the jump then means the same as the jump without a label.

JMP-14 already makes each label name occur once in a function, so a label name finds one statement. JMP-5 already makes a label visible in all of its function. A label that marks an enclosing statement always comes before the jump in the text.

The label still serves `goto`. `goto L;` with `L` on a loop goes to the start of the loop statement, which runs the initializer of a `for` again (JMP-8). Probe `s1.cpp` shows that both compilers accept a chain of two labels and an attribute list before a loop, with a `goto` to each label.

The design takes the statement that the label marks for three reasons:
- Each label gives one target. If a label could name any statement that holds a loop, `L: { for (...) for (...) break L; }` would need a rule to pick one of the two loops.
- `break` keeps its meaning from JMP-P1: it ends a loop or a `switch`. FN-7 and ATTR-7 reason about `break` in that shape.
- A forward exit from a block is a `goto` today. 1.8 lists a labeled block as an alternative that P22 leaves for later.

JMP-7 changes. Its first two sentences cover a jump without a label. For a labeled jump, X24:new-1 takes their place.

### 1.4 A labeled `continue` names a loop (new rule X24:new-2)

X24:new-2 says that the label of a labeled `continue` must mark a loop. A `continue` whose label marks a `switch` is not valid. A labeled `break` can name a `switch`. Such a `break` leaves every loop and `switch` between the jump and the named `switch`:

```cpp
dispatch:
switch (mode) {
  case 1:
    for (int i = 0; i < n; ++i)
      if (items[i] < 0) break dispatch;     // leaves the for and the switch
    break;
  case 2:
    for (int i = 0; i < n; ++i)
      if (items[i] < 0) continue dispatch;  // rejected: dispatch marks a switch
    break;
}
```

A `switch` has no next pass. A `continue` that named a `switch` could mean "dispatch again on a new value" or "go to the end of the switch body". The second meaning is a `break`. The first is a new kind of loop. JMP-2 already says that a `switch` between a `continue` and its loop does not change the target. Rejecting the form keeps one meaning for `continue`, and a later proposal can still give the form a meaning.

### 1.5 Lambda bodies and local classes (amended JMP-14)

JMP-7, JMP-14, and SW-3 treat a lambda body as a separate function. SW-3 says the same of the member functions of a class defined inside a function. P22 applies that model to labels. The search for a jump's label looks only at the labels of the function that holds the jump. JMP-14 changes so that its second condition covers a labeled jump as it covers a `goto`.

So a labeled jump in a lambda body has these outcomes:
- If the lambda body declares the label, the jump uses it, and X24:new-1 applies inside the lambda body.
- If only the enclosing function declares the label, the jump is not valid. This holds also when the label marks a loop around the lambda expression.

```cpp
outer:
for (int i = 0; i < n; ++i) {
  auto stop = [&] {
    for (int j = 0; j < m; ++j)
      if (bad(i, j)) break outer;     // rejected: outer is not a label of the lambda body
  };
  auto scan = [&] {
  outer:                              // valid: a label of the lambda body (JMP-14)
    for (int j = 0; j < m; ++j)
      if (bad(i, j)) break outer;     // ends the lambda's own loop
  };
  scan();
}
```

Probe `l1.cpp` shows that both compilers accept one label name in a lambda body and in its enclosing function today.

A lambda can run after the loop around it has ended, or from another function that received it. A jump from its body to the enclosing loop would be a non-local exit, which only a throw gives today. A program that needs the effect returns a value from the lambda and tests the value in the loop.

### 1.6 Run time (new rule X24:new-3; amended JMP-1, JMP-2, LOOP-3, OBJ-3)

X24:new-3 gives the run-time meaning:
- `break L;` ends the statement that `L` marks, as JMP-1 ends the innermost loop or `switch`. Control goes on after that statement.
- `continue L;` ends the current pass of the body of the loop that `L` marks, as JMP-2 does for the innermost loop. The loop then evaluates its iteration expression, if any, and its condition. A range-based `for` advances to the next element, and a `do` evaluates its condition (LOOP-2).

The jump leaves each block and each statement between the jump and the target. The objects end from the innermost outward, each group in the reverse of its construction order (OBJ-3):
- the objects of each block that the jump leaves,
- for each loop, `switch`, `if`, and range-based `for` inside the target that the jump leaves, the variables of its own scope, after its body's objects: the initializer's variables, the condition's variable, and the hidden variables of a range-based `for` with the temporaries of its range expression (RFOR-4),
- for `break L`, the target's own initializer and condition variables, as for a `break` without a label (LOOP-3, LOOP-1).

For `continue L`, the target's initializer variables stay. A condition variable of a `while` ends after the body's objects, as in each pass (LOOP-1). A condition variable of a `for` ends after the iteration expression (LOOP-3).

Probe `d1.cpp` shows this order on both compilers for a `goto` to the same places. The default build models `break L` with a `goto` past the outer loop. With `-DCONT` it models `continue L` with a `goto` to the end of the outer body. Both compilers end the inner body's object, the inner `for`'s condition and initializer variables, the range body's object, and the range temporary, in that order, then the outer body's object.

OBJ-3 and JMP-9 speak of blocks. LOOP-3 names only `break` for the end of a `for` initializer's variables. A labeled jump leaves whole statements whose scope variables are in no block, so P22 needs OBJ-3 to cover a statement's own scope (X24-1). LOOP-3 changes so that "also after a `break`" reads "also after any jump out of the statement".

A labeled jump only leaves scopes. It enters none, so JMP-6 and SW-5 do not apply. `continue L` leaves the body of the target loop. A declaration later in that body is not skipped in the sense of JMP-6, as for an unlabeled `continue` (probe `k1.cpp`, X24-2).

A labeled jump out of a handler ends the handler's objects and then its handled exception (TRY-15). A jump out of several handlers ends each handled exception in turn, innermost first. A labeled jump can leave a try block or a branch of `if constexpr` or `if consteval`. TRY-7 and IF-17 limit only entry.

Constant evaluation runs a labeled jump. CE-17 rejects a `goto` that an evaluation reaches and says nothing of `break` or `continue`. CE-P1 lists loops among what an evaluation can do. X24:new-3 states that labeled jumps run there too, so `find` in 1.1 is a constant.

### 1.7 Templates (unchanged DEP-30)

The checks of X24:new-1 and X24:new-2 depend on no template parameter. They depend only on the function's statements and labels. P22 makes both rules `reject` rules that an implementation checks at the definition of a template, as both host compilers check a `break` outside a loop there. Probe `t1.cpp -DTEMPLATE_BREAK` shows both compilers reject a `break` outside a loop in a function template that is never instantiated.

A `goto` to a missing label differs on the host today. Clang rejects it in a template that is never instantiated and in a discarded branch of an instantiated template, and GCC accepts both (`t1.cpp`). Tidal does not say which answer holds (X24-3). P22 does not follow the `goto` case. If the writer settles `goto` under DEP-30, the alternative of 1.8 puts labeled jumps under DEP-30 too.

Outside a template, a discarded branch of `if constexpr` is still checked (the principle on `if constexpr` in `statements/if.md`). A labeled jump there must meet X24:new-1 and X24:new-2.

### 1.8 Alternatives considered

- **A label names any statement that holds the jump.** `L: { ... break L; ... }` would leave the block. This gives a forward exit from a block that constant evaluation accepts, which `goto` does not give. The rule must then pick a target among nested loops inside the labeled statement, and FN-7 and ATTR-7 must learn that `break` can end a block. Deferred. A later proposal can add it without changing a program that is valid under P22.
- **A labeled `continue` to a `switch` dispatches again.** This makes a `switch` a loop. Rejected (1.4).
- **A jump in a lambda body can name a label of the enclosing function.** This is a non-local exit (1.5). Rejected.
- **A count instead of a label (`break 2;`).** The count changes when an edit adds or removes a loop, and the reader must count braces. Labels exist, have their own name space (NAM-P4), and are unique in a function (JMP-14). Rejected.
- **A name in the loop header, such as a name after `for`.** This is new syntax for a thing that the `label` production already does. Rejected.
- **Labeled jumps under DEP-30.** An implementation could wait for an instantiation to check a labeled jump in a template, as GCC does for a `goto` today (1.7). The proposal changes no count in section 4 either way. The choice should follow the answer to X24-3.
- **Labeled jumps fail in constant evaluation, as `goto` does.** A labeled jump has a structured target, and the objects it ends are the ones that a `break` or `continue` ends. Rejected.

### 1.9 Migration

- A `goto` past the outer loop, as in `find_goto` of the JMP Examples and of `tests/programs/W3/nested.cpp`, becomes a label on the outer loop and a `break` with that label. The function can then be `constexpr`.
- A flag that each condition tests, as in `find_flag` of the same two places, becomes a `break` with a label. The flag and its tests go.
- A helper function that returns from inside two loops stays as it is.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests affected" lists the existing tests whose expected outcome changes. New rules use this exercise's ID space.

| Rule ID | Kind of change | Old class | New class | Description | Tests affected |
|---|---|---|---|---|---|
| X24:new-1 | new | none | reject | A labeled jump must name a label of its own function, the label must mark a loop or a `switch` (after further labels and attribute lists), and the jump must be inside that statement's body (1.3). | none |
| X24:new-2 | new | none | reject | The label of a labeled `continue` must mark a loop, and not a `switch` (1.4). | none |
| X24:new-3 | new | none | required | `break L` ends the statement that `L` marks. `continue L` ends the pass of the body of the loop that `L` marks, and the loop goes on as after a normal pass. Each block and statement left ends its objects, innermost first. Constant evaluation runs both forms (1.6). | `statements/jumps/break-with-label.cpp`, `statements/jumps/continue-with-label.cpp` (reject to accept) |
| JMP-17 | withdrawn | reject | none | The grammar has a place for a label after `break` and `continue`. X24:new-1 and X24:new-2 take over the limits. | `statements/jumps/break-with-label.cpp`, `statements/jumps/continue-with-label.cpp` (reject to accept) |
| jump_stmt (Syntax) | amended | grammar | grammar | `"break" [identifier] ";"` and `"continue" [identifier] ";"` (1.2). | `statements/jumps/break-with-label.cpp`, `statements/jumps/continue-with-label.cpp` |
| JMP-P1 | amended | principle | principle | The table gives the labeled forms. The paragraph "`break` and `continue` take no label" gives the labeled jump as a fourth way to leave two loops, the one that constant evaluation accepts. The lambda row of the region table adds the labeled forms. | none |
| JMP-1 | amended | required | required | `break` ends the innermost loop or `switch`, or the statement that its label marks (X24:new-3). | none |
| JMP-2 | amended | required | required | `continue` goes to the end of the body of the innermost loop, or of the loop that its label marks. Each statement left ends its scope variables too (X24:new-3). | none |
| JMP-5 | amended | required | required | A `goto` can name a label before or after it. A labeled jump names the label of an enclosing loop or `switch` (X24:new-1). | none |
| JMP-7 | amended | reject | reject | The first two sentences cover a jump without a label. A labeled jump falls under X24:new-1 and X24:new-2. | none |
| JMP-14 | amended | reject | reject | The second condition covers a `goto` and a labeled jump. A lambda body and each member function of a class defined inside a function are separate functions (1.5). | none |
| LOOP-3 | amended | required | required | The initializer's variables end when the statement ends, also after any jump out of the statement, a labeled jump from a nested loop included. | none |
| OBJ-3 | amended | required | required | The variables of a statement's own scope (initializer, condition, the hidden variables of a range-based `for`) end when control leaves the statement, after the objects of its body (X24-1). | none |
| FN-7 | amended | recommended | recommended | A loop whose condition is constant `true` ends a path if no `break` without a label in its own body, and no labeled `break` that names it, leaves it (X24-4). | none |
| ATTR-P1 | amended | principle | principle | The place table says "a name that a `goto`, `break`, or `continue` uses" for an identifier label. | none |
| glossary: label | amended | glossary | glossary | A new entry: a label marks a statement. An identifier label is a target of `goto` and names a loop or `switch` for a labeled jump. A `case` or `default` label belongs to the innermost `switch` (X24-5). | none |
| JMP-P2 | unchanged, state effect | principle | principle | A labeled jump only leaves scopes, so the row "Out of a block" applies to each block that it leaves. | none |
| JMP-6 | unchanged, state effect | reject | reject | A labeled jump enters no scope, so JMP-6 does not apply to it (X24-2). | none |
| JMP-8 | unchanged, state effect | required | required | A `goto` to a label that marks a loop goes to the start of the loop statement. | none |
| JMP-9 | unchanged, state effect | required | required | Covers each block that a labeled jump leaves. | none |
| LOOP-1 | unchanged, state effect | required | required | A `while` condition variable ends after the body's objects, also for `continue L` and `break L`. | none |
| LOOP-2 | unchanged, state effect | required | required | A `continue L` that names a `do` evaluates its condition. | none |
| LOOP-9 | unchanged, state effect | required | required | The model of "goes on as after a normal pass" that `continue L` uses. | none |
| RFOR-4 | unchanged, state effect | required | required | The temporaries of an inner range-based `for` end when a labeled jump leaves that loop. | none |
| SW-1 | unchanged, state effect | required | required | A labeled `break` that names this `switch` or an enclosing statement also ends the run through the labels. | none |
| SW-3 | unchanged, state effect | reject | reject | A `case` or `default` label belongs to the innermost `switch`. An identifier label in a chain before a statement does not change that. | none |
| STMT-5 | unchanged, state effect | required | required | A label at the end of a block marks no statement, so a labeled jump cannot name it. | none |
| CE-17 | unchanged, state effect | reject | reject | A labeled jump is not a `goto`, and constant evaluation runs it. | none |
| TR-12 | unchanged, state effect | reject | reject | The amended `jump_stmt` is part of the grammar. A label after `return` stays a grammar error. | none |
| TRY-15 | unchanged, state effect | required | required | A labeled `break` or `continue` is one of the jumps that end a handler. | none |
| IF-17 | unchanged, state effect | reject | reject | A labeled jump can leave a branch of `if constexpr` or `if consteval` and never enters one. | none |
| ATTR-7 | unchanged, state effect | recommended | recommended | Statements that end with a labeled `break` or `continue` do not fall through. | none |
| DEP-30 | unchanged, state effect | no-diagnostic | no-diagnostic | X24:new-1 and X24:new-2 are checked at a template's definition and are not in DEP-30's list (1.7). | none |

Counts: 33 changes. 3 new, 12 amended, 1 withdrawn, 17 unchanged but must state their effect, 0 split. One amended row is the grammar of `jump_stmt` and one is a glossary entry. 2 existing tests change their outcome, from `reject` to `accept`.

The two changed tests keep their programs. Their rules become X24:new-3 and JMP-1 (`break-with-label.cpp`) and X24:new-3 and JMP-2 (`continue-with-label.cpp`), with `tidal-expect: accept`, and the error patterns go. A `static_assert` could fix the results: `f()` gives 5 in the first test and 6 in the second. The proposal needs these new tests under `tests/statements/jumps/`:
- `break-label-outer-loop-constant.cpp` (accept; `find` of 1.1 in a `static_assert`),
- `continue-label-outer-loop-run.cpp` (run; `sum_rows` of 1.1),
- `break-label-ends-inner-statements-run.cpp` and `continue-label-ends-inner-statements-run.cpp` (run; the order of probe `d1.cpp`, with the labeled forms),
- `break-label-switch-from-loop.cpp` (run; `dispatch` of 1.4, case 1),
- `continue-label-names-switch.cpp` (reject; X24:new-2),
- `break-label-not-enclosing.cpp` (reject; a label on a sibling loop),
- `break-label-on-block.cpp` and `break-label-at-block-end.cpp` (reject; X24:new-1),
- `break-label-from-lambda.cpp` (reject; JMP-14 and X24:new-1, `stop` of 1.5),
- `break-label-same-name-in-lambda.cpp` (accept; `scan` of 1.5),
- `break-label-chain.cpp` (accept; the chain of 1.3),
- `continue-label-do-checks-condition.cpp` (run; LOOP-2),
- `continue-label-past-later-declaration.cpp` (accept; JMP-6 does not apply, probe `k1.cpp` with the labeled form),
- `break-label-out-of-handler.cpp` (run; TRY-15),
- `break-label-template-never-instantiated.cpp` (reject; 1.7),
- under `declarations/functions/definitions/`: `flow-off-end-labeled-break-warns.cpp` (FN-7).

## 3. Interactions

**Constant evaluation (CE-17, CE-P1).** Labeled jumps run in constant evaluation. The `goto` form of JMP-P1 does not. P22 gives `constexpr` code a way to leave two loops without a flag.

**Templates (DEP-30, JMP-7, JMP-14).** P22 checks labeled jumps at the definition (1.7). The host compilers differ on a `goto` to a missing label in a template (X24-3). The labeled form does not inherit that split.

**Scopes that a jump leaves (OBJ-3, JMP-9, LOOP-3, RFOR-4).** A labeled jump leaves whole statements, so more programs depend on when their scope variables end. The compilers already end them when a `goto` leaves (`d1.cpp`). Tidal's text covers blocks only (X24-1).

**Jumps that skip a declaration (JMP-6, JMP-16, Q-0041, Q-0121).** A labeled jump enters no scope. These rules and questions do not arise for it (X24-2).

**Handlers (TRY-15, TRY-7).** A labeled jump is one of the exits of a handler. No new rule is needed.

**Warnings (FN-7, ATTR-7).** FN-7 must count a labeled `break` that names a loop as leaving it, and must not count a `break` of an inner loop or `switch` (X24-4). ATTR-7 counts both labeled forms as ends of a statement list.

**Labels (NAM-P4, JMP-5, JMP-14, STMT-5, SW-3).** The label search, the name space, and the uniqueness of labels exist. P22 adds only the rule of which statement a label marks (1.3).

**The prototype hook (`frontend/DESIGN-rewrite.md` section 7).** The hook's three checks match X24:new-1, X24:new-2, and the same-function condition. In X24 the same-function condition is an amendment to JMP-14, and X24:new-3 is the run-time rule. The hook names its new rules `Xn:new-1` to `Xn:new-3`. For X24 these are the three new rows of section 2, and the third has other content than the hook's third check. The hook says that the run step needs no change. That holds only if the machine already ends the scope variables of each statement that a `goto` leaves (X24-1).

**Not touched.** P22 leaves these alone:
- `goto`, its targets, and JMP-6,
- `case` and `default` labels, and which `switch` they belong to,
- warnings about unused labels, which Tidal does not give,
- coroutines, which are outside Tidal's scope (KW-1),
- a label on a block (1.8).

## 4. Impact on existing programs

`python3 -I tools/propose_check.py --corpus` counts 152 programs under `tests/programs/` (99 from the Write suites and 53 programmer sketches under `roadmap/`), 8 with more than one unit, and 20156 lines in 175 files, with extra units and headers.

The scan `scan_jumps.py` reads all 7171 `.cpp`, `.h`, and `.hpp` files under `tests/`. It removes comments and literals and lists:
- each `break` or `continue` that an identifier follows,
- each identifier label that marks a loop or a `switch`, after further labels and attribute lists,
- each `goto`, and the programs under `tests/programs/` that have a label on a loop or a `goto`.

| Scan | Files | Hits |
|---|---|---|
| `break` or `continue` with an identifier | 7171 | 2, in `statements/jumps/break-with-label.cpp` and `continue-with-label.cpp` |
| identifier label that marks a loop or `switch` | 7171 | 2, the same two files |
| `goto` statements | 7171 | 46 |
| programs with a label on a loop, or a `goto` | 152 programs | 1, `W3/nested.cpp` (one `goto`, no label on a loop) |

By hand:
- The two JMP-17 tests are the only files with the P22 form. They change (4.1).
- `W3/nested.cpp` leaves two loops in the three ways of JMP-P1. Its header comment says that C++ has no labeled `break`. Its `goto` function is not `constexpr` because of the `goto`. Its meaning does not change. It shows the demand.
- A text search of `tests/programs/` for a loop condition that tests a negated flag finds only `W3/nested.cpp`.

No program under `tests/programs/` changes, out of 152.

### 4.1 Rejected becomes valid

- A `break` or `continue` with a label that meets X24:new-1 and X24:new-2. Two tests: `statements/jumps/break-with-label.cpp` and `statements/jumps/continue-with-label.cpp`, from `reject` to `accept`. Both host compilers reject them today (GCC: "expected ';' before 'outer'"; Clang: "named 'break' is only supported in C2y").

### 4.2 Undefined becomes defined

None. P22 adds no operation to a program that is valid today.

### 4.3 Valid programs that change meaning

None. The reasons:
- The new form derives from no production of the grammar today (TR-12, JMP-17). No valid program contains it, so no valid program gets a new meaning from X24:new-3.
- P22 adds no keyword, no token, and no name. A label name after `break` is searched only among labels (NAM-P4), so no declaration changes meaning.
- A test of validity cannot see the change. A statement occurs only in a function body. A function body inside a test of validity, such as a lambda body in a `requires` expression, is outside the immediate context. Both compilers reject a `break` outside a loop in such a lambda body as a hard error, with and without a dependent parameter (`r1.cpp`, `r1.cpp -DDEP`). So no `requires` expression or substitution can test whether a labeled jump is valid, giving `false` today and `true` under P22.
- The amendments to OBJ-3 and LOOP-3 state for statement scopes what both compilers already do for a `goto` (`d1.cpp`). They give no new order to any valid program.
- The FN-7 amendment changes which programs get a warning only where a labeled `break` occurs, and no valid program has one today. The part of the amendment on inner loops and `switch` states what both compilers already do (`f1.cpp`).

### 4.4 Valid programs that become invalid

None. The reasons:
- X24:new-1 and X24:new-2 constrain only labeled jumps, which no valid program has.
- The JMP-14 amendment covers a `goto` as before and adds labeled jumps.
- The checks at a template's definition (1.7) apply only to labeled jumps.
- No test changes from an accepted outcome to a rejection. No program under `tests/programs/` becomes invalid, out of 152.

The counts for the alternatives of 1.8:
- A label on any statement: the same counts. No file under `tests/` has a `break` or `continue` that names a label on a block.
- Labeled jumps under DEP-30: the same counts. No file under `tests/` has a labeled jump in a template.

### 4.5 Evidence

All probes are in `reports/clean-room/X24/p/`.
- `d1.cpp`: a `goto` out of an outer `for` with an initializer variable, a body object, an inner range-based `for` over a temporary range, and an inner `for` with initializer and condition variables. Both compilers print the same order: the inner body object, the inner condition variable, the inner initializer variable, the range body object, the range temporary, the outer body object, and the outer initializer variable. With `-DCONT` (a `goto` to the end of the outer body), both print the same order up to the range temporary, then the outer body object, then a second pass.
- `c1.cpp`: the flag form of a two-loop search is a constant on both compilers. With `-DGOTO`, both reject the `goto` form in a `static_assert` (GCC: "'goto' is not a constant expression").
- `l1.cpp`: one label name in a lambda body and in its enclosing function. Both accept and run it with status 0.
- `l2.cpp`: a loop holds a class defined inside the function, and a member function of that class has a `break`. Both reject it.
- `t1.cpp`: `-DTEMPLATE`, a `goto` to a missing label in a function template that is never instantiated: GCC accepts, Clang rejects. `-DDISCARDED`, the same `goto` in a discarded branch outside a template: both reject. `-DTEMPLATE_DISCARDED`, in a discarded branch of an instantiated template: GCC accepts, Clang rejects. `-DTEMPLATE_BREAK`, a `break` outside a loop in a template that is never instantiated: both reject (GCC under `-Wtemplate-body`, an error by default).
- `r1.cpp`: a `break` outside a loop in a lambda body inside a `requires` expression. Both reject the concept's definition, with and without `-DDEP`.
- `f1.cpp` with `-Wreturn-type` and code generation: both warn for `b`, whose `while (true)` a `goto` leaves. Neither warns for `a`, whose `break` leaves an inner `switch`, or for `c`, whose `break` leaves an inner `for`. GCC gives the warning only with code generation, and not with `-fsyntax-only`.
- `k1.cpp`: a `continue` and a `break` before an initialized declaration in the same loop body, in constant evaluation and at run time. Both accept and run it with status 0.
- `s1.cpp`: two labels and an attribute list before a `for`, with a `goto` to each label. Both accept and run it with status 0.
- The two JMP-17 tests, compiled as they are: GCC reports a missing `;`, and Clang reports that the named form is only supported in C2y. Both reject `break-with-label.cpp` also without `-pedantic-errors`.
- `scan_jumps.out` holds the scan's output.

## 5. Findings

### X24-1

Kind: unclear. Pages: `foundations/objects.md`, `statements/jumps.md`, `statements/loops.md`, `statements/range-for.md`. Rules: OBJ-3, JMP-9, LOOP-3, LOOP-1, RFOR-4.

What happened: OBJ-3 ends automatic objects "when control leaves their block", and counts an unbraced branch or loop body as a block. JMP-9 speaks of a jump out of a block. The variables of a `for` initializer, of a condition, and the hidden variables of a range-based `for` belong to the statement's own scope (IF-P2, LOOP-P2), which is not a block. LOOP-3 ends the initializer's variables "when the statement ends, also after a `break`". No rule says when these variables end after a `goto`, a `return`, or a throw out of the statement, or in which order relative to the body's objects. Both compilers end them after the body's objects, statement by statement from the inside out (`d1.cpp`). P22 makes the gap matter for each nested loop that a labeled jump leaves.

Suggestion: OBJ-3 says that the scope of a statement counts as a block for this rule, and that its variables end after the objects of the statement's body. LOOP-3 says "after any jump out of the statement". Add a run test with the order of `d1.cpp`.

### X24-2

Kind: unclear. Pages: `statements/jumps.md`. Rules: JMP-6, JMP-P2, JMP-2.

What happened: JMP-6 says that "a jump" must not skip the declaration of a variable that is in scope at the target. JMP-P1 calls `break`, `continue`, `return`, and `goto` jump statements. JMP-2 says that `continue` goes to the end of the loop body. A variable declared with an initializer later in the body is in scope at the end of the body, so a literal reading rejects `if (odd) continue; int y = i;`. Both compilers accept this form (`k1.cpp`). The JMP-P2 table row "Forward past an initialization: Rejected" has the same reach.

Suggestion: JMP-6 names the jumps it covers: a `goto`, and a transfer to a `case` or `default` label (SW-5). It says that `break`, `continue`, and `return` only leave scopes. Add `k1.cpp` as a test.

### X24-3

Kind: unclear. Pages: `statements/jumps.md`, `templates/dependent-names.md`, `statements/if.md`. Rules: JMP-14, DEP-30, DEP-28.

What happened: JMP-14 rejects a `goto` to a label that its function does not have. No rule says whether a template that is never instantiated, or a discarded branch of an instantiated template, must get the diagnostic. DEP-30 lists the independent constructs that need no diagnostic, and a jump target is not on the list. Clang rejects a `goto` to a missing label in both places, and GCC accepts both (`t1.cpp -DTEMPLATE`, `-DTEMPLATE_DISCARDED`). Both reject a `break` outside a loop in a template that is never instantiated (`-DTEMPLATE_BREAK`). P22 must choose a class for its own label checks, and the specification gives no model.

Suggestion: JMP-14 or DEP-30 states which class applies to a `goto` to a missing label in a templated context. Add the `t1.cpp` cases as tests, with a question for the host split.

### X24-4

Kind: unclear. Pages: `declarations/functions/definitions.md`. Rules: FN-7.

What happened: FN-7 counts "a loop whose condition is a constant `true`, if no `break` leaves it" as an end of a path. The rule does not say that a `break` of an inner loop or `switch` does not leave the outer loop. It does not mention a `goto` that leaves the loop. Both compilers warn when a `goto` leaves such a loop, and warn for neither inner `break` (`f1.cpp`). GCC gives the warning only with code generation.

Suggestion: FN-7 says "if no `break` in its own body, and no `goto`, leaves it", and the Evidence notes the `goto` case and GCC's need for code generation. Under P22, the rule adds a labeled `break` that names the loop.

### X24-5

Kind: hard to find. Pages: `glossary.md`, `statements/blocks.md`, `statements/jumps.md`, `declarations/attributes.md`. Rules: JMP-P1, STMT-5, ATTR-P1, SW-3.

What happened: the glossary has no entry for "label", "loop", or "jump statement". The `label` production, which covers identifier, `case`, and `default` labels, is in `blocks.md`. `jumps.md` opens with "A label names a statement that `goto` can transfer control to", which covers only identifier labels. The place table of ATTR-P1 describes an identifier label as "a name that a `goto` uses", which reads as if a label needs a `goto`. A designer collects what a label is from four pages.

Suggestion: add a glossary entry "label" that names the three kinds, says which statement a label marks, and links `blocks.md`, `jumps.md`, and `switch.md`. ATTR-P1 says "an identifier label".

### X24-6

Kind: unclear. Pages: `statements/jumps.md`, `statements/switch.md`. Rules: JMP-7, JMP-14, SW-3.

What happened: SW-3 says that a lambda body and the member functions of a class defined inside a function are separate functions. JMP-7 and JMP-14 name only the lambda body. A loop holds a class defined inside the function, and a member function of that class has a `break`. Both compilers reject the `break` (`l2.cpp`). JMP-7 gives that result only through the general reading that the member function is another function.

Suggestion: JMP-7 and JMP-14 name both bodies, as SW-3 does, and JMP-7 cites a test made from `l2.cpp`.

### X24-7

Kind: needs example. Pages: `statements/jumps.md`. Rules: JMP-5, JMP-14.

What happened: JMP-14 says that each label name occurs once in a function and that a lambda body is a separate function. No example or cited test shows one label name in a lambda body and in its enclosing function. Both compilers accept it (`l1.cpp`). The case matters for any feature that searches labels from inside a lambda body, such as P22.

Suggestion: add `l1.cpp` as a test of JMP-14, and a line to the Examples.

### X24-8

Kind: outside knowledge. Pages: none. Rules: none.

What happened: the reader knows from general experience that several other languages have labeled `break` and `continue`, and that later revisions of C, and work toward later versions of C++, consider a similar form. Clang's diagnostic on the JMP-17 tests also names a later C mode. The design takes its choices from Tidal alone and uses no text or rule from those sources. A writer who compares P22 with them does so outside the clean room, under the limits of `pending.md` row 7.

Suggestion: none for the specification.

## Host-only claims

Each claim below rests on g++ 16.2.1 and clang++ 22.1.8 on the host. A writer probes it on the matrix before acting.
- A `goto` out of nested loops ends the inner body's objects, then each inner statement's condition and initializer variables and range temporaries, statement by statement from the inside out, and then the outer body's objects (`d1.cpp`, `d1.cpp -DCONT`).
- Both compilers reject the `goto` form of a two-loop search in a `static_assert`, and accept the flag form (`c1.cpp`).
- Both accept one label name in a lambda body and in its enclosing function (`l1.cpp`).
- A loop holds a class defined inside the function, and a member function of that class has a `break`. Both compilers reject the `break` (`l2.cpp`).
- GCC accepts, and Clang rejects, a `goto` to a missing label in a function template that is never instantiated and in a discarded branch of an instantiated template. Both reject the `goto` in a discarded branch outside a template (`t1.cpp`).
- Both reject a `break` outside a loop in a function template that is never instantiated (`t1.cpp -DTEMPLATE_BREAK`).
- Both reject a `break` outside a loop in a lambda body inside a `requires` expression, with and without a dependent parameter (`r1.cpp`).
- Both warn under `-Wreturn-type` when a `goto` leaves a `while (true)` loop, and neither warns when a `break` leaves only an inner `switch` or `for`. GCC warns only with code generation (`f1.cpp`).
- Both accept a `continue` and a `break` before an initialized declaration in the same body, in constant evaluation and at run time (`k1.cpp`).
- Both accept two labels and an attribute list before a `for`, with a `goto` to each label (`s1.cpp`).
- Both reject the two JMP-17 tests. Clang names the form as supported only in C2y. Both reject `break-with-label.cpp` also without `-pedantic-errors`.
