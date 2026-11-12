# Q-0452: In constant evaluation, when does a `return` or a `break` end the condition and initializer variables of a statement?

**Status:** provisional
**Affects:** OBJ-3, LOOP-3, `tests/foundations/objects/statement-scope-ends-after-return-constant.cpp`
**Source:** X24-1 (`reports/clean-room/X24/report.md`), with the item F7 writer's probe grid `ce-order` under `tools/probe_grid.py`. Each row records the order of destructor calls in a `constexpr` function and shows it in the first error.
**Waits for:** newest families that agree

## Context

A `for` loop declares a variable in its initializer and one in its
condition, and its body declares a third. A `return` in the body
leaves the loop during constant evaluation:

```cpp
constexpr void leave(Log* log) {
  for (Mark init(log, 'a'); Mark cond = Mark(log, 'b');) {
    Mark body(log, 'c');
    return;
  }
}
```

Each destructor adds its letter to the log. The table gives the order
of the letters.

| Form | Clang 18 and 22, GCC 14 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| `return` out of a `for` with an initializer and a condition variable | c b a | c a | b c a |
| `return` out of a `while` with a condition variable | c b | c | b c |
| `break` out of a `for` with an initializer and a condition variable | c b a | c b a | b c a |
| a `while` whose condition variable ends after each iteration | c b c b | c b c b | b c b c |
| `return` out of an `if` with an initializer and a condition variable | c b a | c b a | b a c |
| `return` out of a `switch` with an initializer | c a | c a | a c |
| `return` out of a `for` with an initializer and no condition variable | c a | c a | c a |
| `return` out of a range-based `for` (r is the range temporary) | c r | c r | c r |

At run time, every compiler that builds gives the order of the first
column for each exit (`reports/clean-room/roadmap-14/writer-probes/run-probes.out`).
EDG does not link, so no run shows its order.

Q-0044 already records that EDG ends a `for` condition variable before
the body in constant evaluation. The grid shows the same order for a
`while` and an `if` condition variable, and for the variable of an
`if` or `switch` initializer. GCC 16 ends each variable in its place
after a `break` and at the end of an iteration, and skips the
condition variable only after a `return`.

## Evidence

The test in **Affects** and the probe grid in **Source**.
`tests/foundations/objects/statement-scope-ends-after-return.cpp` and
`statement-scope-ends-after-goto.cpp` hold the run-time order, and
every compiler that builds agrees.

## Options

1. Constant evaluation ends the objects as a run does: the body's
   objects first, then the condition variable, then the initializer
   variable (Clang, GCC 14).
2. Another order or a skipped destructor is allowed in constant
   evaluation (GCC 16 skips the condition variable after a `return`,
   and EDG ends the statement's variables before the body's objects).

## Provisional answer

Constant evaluation ends the objects as a run does (option 1). OBJ-3
says that its order applies to constant evaluation, and LOOP-3 ends
the initializer's variable after any exit. Clang and GCC 14 agree.
GCC 16 and EDG 7.0 differ, so two newest families stand against the
answer.

Opened on 2026-11-11 (decision 0065, item F7). OBJ-P2 says that objects end in the reverse of the order in which they began, by any route, and it does not set constant evaluation apart. A reader of OBJ-P2 alone reaches option 1, so OBJ-P2 passes step 3 of the principle test. No reader or reviewer has used OBJ-P2 on this program.

Kept open on 2026-11-11 under decision 0054 (decision 0065, item RV10). The gap review of the phase 17 text supports the answer with no doubt (RV10-13). GCC 16 and EDG 7.0 still differ, so two newest families stand against the answer. OBJ-P2 still passes step 3 of the principle test, and the review does not name it.

## Opinions

### Gap review, 2026-11-11 (phase 17 text)

The source-reader gap review of the phase 17 text (`reports/review/2026-11-11-gap-review-phase-17.md`) gives the first reading of the question: supports with no doubt (option 1, RV10-13). Constant evaluation has the same semantics as a run. A transfer of control destroys the variables that are active at the source and not at the target, in the reverse of their construction, and the variables of a statement's initializer and condition have a scope that holds the body. So a `return` ends the body's objects, then the condition variable, then the initializer variable, during constant evaluation as at run time. A skipped destructor or another order gives a different log and a different result. GCC 16 and EDG 7.0 read against.

### Read of the RV10 repair diff, 2026-11-11

The source-reader read of the RV10 repair diff (`reports/review/2026-11-11-read-of-rv10-repair-diff.md`) reads the dated note and the gap review's entry of decision 0065, item RV10: agrees as a record. Both state RV10-13 correctly. The read gives no new reading of the question's answer.
