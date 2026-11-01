# 0053. An expectation that only the interpreter checks

Date: 2026-11-01
Status: Accepted. Builds on 0047 and 0048.

## Context

A test whose run has undefined behavior must not expect `run`. If no
sanitizer stops the behavior, the test expects `accept`, and no tool
checks its run. The front end's interpreter stops some of these runs
and names the rule, for example the STG-19 and STG-20 tests. Nothing
records or checks these stops. Decision 0048 wants the harness to
expect a stop that names one of the test's rules.

## Decision

- A test can carry the line `// tidal-interpreter: run-fail`. The test
  expects `accept` on every matrix compiler.
- The matrix compilers only build such a test. `tools/run_tests.py`
  checks the form of the line and ignores it otherwise.
- `tidal-fe conform` runs such a test in the interpreter. The run must
  stop at undefined behavior that names one of the test's rules. A run
  that ends with a result, or a stop that names another rule, fails.
- The line has its own directive name. A `tidal-expect-<x>` line names
  a matrix compiler, and the agreement counts read such lines as
  divergences. The interpreter is part of the front end, and the matrix
  compilers agree on these tests.
- An interpreter stop shows only that the program reaches the named
  operation. The interpreter is written from the rules that its stop
  would confirm. A rule's class still rests on compiler evidence:
  diagnostics, constant evaluation, sanitizers, and variation across
  implementations (decision 0047).

## Consequences

- `documentation/specification-format.md` lists the directive.
- `tools/run_tests.py`, `tools/frontend_check.py`, and
  `frontend/src/conform.rs` read the line.
- The STG-19 and STG-20 tests that moved to `accept` under front-end
  finding G78 carry the line.
- Other `accept` tests of `undefined` rules gain the line when the
  interpreter stops them.
