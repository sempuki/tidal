# Status

Where the project stands and what comes next. This page describes the
current state only. The history of how it got here is in `git log`.

## Specification

| Section | Rules | Classes | Tests | Questions | Similarity |
|---|---|---|---|---|---|
| [The if statement](../spec/statements/if.md) | 20 | 14 required, 6 reject | 56 | Q-0007 | 0 flagged |
| [Multiplicative operators](../spec/expressions/multiplicative.md) | 15 | 6 required, 2 reject, 5 undefined, 1 unspecified, 1 impl-defined | 27 | Q-0001 to Q-0005 | 0 flagged |
| [Glossary](../spec/glossary.md) | 8 terms | | | | 0 flagged |

`tools/build_pdf.py` builds the whole specification as one PDF
(decision 0009). The latest similarity report is
`reports/similarity/2026-10-30-pilots-2.txt`.

## Test matrix

All compilers are baseline. 83/83 tests pass on each.

| Compiler | Where | Notes |
|---|---|---|
| Clang 22.1.8 | host | |
| GCC 16.2.1 | host | |
| Clang 18.1.3 | container | |
| GCC 14.2.0 | container | |
| EDG 7.0 | container | front end only; skips `-m32` and warning-flag tests; per-compiler expectations on 4 tests (Q-0002, Q-0003) |

Full results are in `tests/RESULTS.md`.

## Open questions

Eight questions are open for opinions, each with a provisional answer the
spec follows. See `questions/README.md`.

## Known limits of the tooling

- A `reject` test passes whenever compilation fails, even for an unrelated
  reason. The report lists each compiler's errors so a reviewer can check
  the cause, but nothing checks it automatically.
- Undefined-behavior evidence comes from constant evaluation and warnings.
  Cases that constant evaluation can't express (some pointer and lifetime
  errors) will need sanitizers.
- EDG runs with its x86-64 Linux configuration only.
- MSVC is not in the matrix.
- The `if` section draws on Clang's source only. GCC's and EDG's parsers
  are unread for it.

## Next steps

1. Publish the repository on GitHub, with continuous integration that runs
   the matrix, the spec checks, and the similarity check, and builds the
   PDF on every change.
2. Write a contributor guide: roles, how to claim a section, and the steps
   a section passes before it counts as drafted.
3. Report the EDG behavior in Q-0002 and Q-0003 to the EDG project.
4. Add a `tidal-error` directive that matches the expected diagnostic, so
   `reject` tests check their cause.
5. Add sanitizer-based tests that run in the container images.
6. Add EDG's 32-bit configuration as a second EDG matrix entry.
7. Read GCC's and EDG's sanitized source for the `if` section.
8. Draft the remaining statements (`switch`, loops, jumps, labels,
   expression and compound statements), then expression families in order
   of dependency.

## Size estimate

The `if` section is typical of a statement. The multiplicative section is
typical of a simple operator family.

- Statements: about 8 sections of the `if` section's size.
- Expressions: about 20 operator families like the multiplicative section,
  plus large topics (value categories and sequencing, constant expressions,
  lambdas, casts, `new` and `delete`, function calls) at roughly four times
  that size each.

That totals about 50 section-units. Drafting one unit takes one working
session for the writer. Review and adjudication are not included and are
expected to dominate once reviewers are involved.
