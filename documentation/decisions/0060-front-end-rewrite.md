# 0060. Rewrite the front end when it is feature complete

Date: 2026-11-05
Status: Accepted (the maintainer's direction). The completion test and
the form of the rewrite are provisional. Amends 0035.

## Context

The clean-room front end in `frontend/` grew over 22 rounds, each round
driven by the tests and exercise findings of its phase. It has 50160
lines of Rust in 28 files. The largest files are `expr.rs` (7358
lines), `decl2.rs` (7016), and `interp/expr.rs` (4723).
`frontend/README.md` gives its use and a table of files. No document
describes the data that passes between its stages. Files split where a
round ran out of room, as `decl.rs` and `decl2.rs` or `expr.rs` and
`expr2.rs` show, and one file holds several of the specification's
chapters.

The maintainer directs that, once the front end is reasonably feature
complete, it is rewritten as software that a person can read,
understand, and extend.

## Decision

- **When.** The front end is feature complete when a front-end round
  ends with these results:
  - every test passes its check, except tests that the front-end
    ledger marks `out of scope` with a reason, and
  - every program test under `tests/programs/` runs to its expected
    result.

  The phase decision after that round adds the rewrite as an item. A
  roadmap round can propose the rewrite earlier, if the remaining
  failures are few and stable over two rounds. Decision 0062 reads "few" as
  at most 50 tests that fail their check by the completion line of
  `tools/frontend_check.py`, and "stable" as no test moved from a
  passing status to a failing one in either round, with a named cause
  for each remaining test (provisional). Decision 0063 counts "few" without the tests that its marks file marks as needing the template model, and starts no rewrite code before the gate holds (provisional).
- **What the rewrite gives.**
  - A design document in `frontend/` that names each stage (lexing,
    parsing, semantic analysis, constant evaluation, interpretation)
    and the data that passes between stages.
  - Modules that follow the specification's chapters and pages, so
    that a reader can go from a rule to the code that implements it.
    Code that implements a rule names the rule ID in a comment.
  - A parser generated or written from the Syntax segments, as
    decision 0035 intends.
  - Unit tests for each stage, beside the conformance harness.
  - No file longer than about 2000 lines, and functions short enough to
    read on one screen.
- **How.**
  - The rewrite is clean-room work under the rules of decisions 0029
    and 0035. The writer reads Tidal's files, including the current
    front end, and no other implementation.
  - The rewrite lands in stages. Each stage keeps
    `tools/frontend_check.py` at 0 worse against the baseline, so the
    old and the new code never disagree on a test without a record.
  - The old code is removed only when the new code passes every test
    that the old code passed.
- The findings that the front end gives the specification stay the
  front end's purpose (decision 0035). The rewrite keeps the
  `unsupported` result for constructs that it does not implement.

## Consequences

- `documentation/roadmap.md` gains the rewrite in its front-end
  section, with the completion test as its prerequisite.
- Each front-end round report states how far the front end is from the
  completion test: the tests that fail their check, by page, and the
  program tests that do not run.
