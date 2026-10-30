# Status

Where the project stands and what comes next. This page describes the
current state only. The history of how it got here is in `git log`.

## Specification

| Section | Rules | Classes | Tests | Open divergences | Similarity check |
|---|---|---|---|---|---|
| [The if statement](../spec/statements/if.md) | 20 | 14 required, 6 reject | 56 | none | not run |
| [Multiplicative operators](../spec/expressions/multiplicative.md) | 15 | 6 required, 2 reject, 5 undefined, 1 unspecified, 1 impl-defined | 27 | MUL-D1, MUL-D2, MUL-D3 | not run |
| [Glossary](../spec/glossary.md) | 8 terms | | | | not run |

`tools/build_pdf.py` builds the whole specification as one PDF
(decision 0009).

## Test matrix

| Compiler | Role | Result |
|---|---|---|
| Clang 22.1.8 (host) | baseline | 83/83 |
| GCC 16.2.1 (host) | baseline | 83/83 |
| Clang 18.1.3 (container) | report | 83/83 |
| GCC 14.2.0 (container) | report | 83/83 |
| EDG 7.0 (container, front end only) | report | 76 match, 4 differ, 2 skipped, 1 with no stated expectation |

Full results are in `tests/RESULTS.md`. EDG's four differences are
divergences MUL-D2 and MUL-D3.

## Open items

- **Adjudication.** MUL-D1 (floating overflow during constant evaluation),
  MUL-D2 and MUL-D3 (EDG accepts some undefined operations during constant
  evaluation) each have a recommendation and need a decision.
- **Similarity check.** Not yet run for any section. See
  `reports/README.md`.
- **Source reading.** The `if` section draws on Clang's source only. GCC's
  and EDG's parsers still need reading.

## Known limits of the tooling

- A `reject` test passes whenever compilation fails, even for an unrelated
  reason. The report lists each compiler's errors so a reviewer can check
  the cause, but nothing checks it automatically.
- Undefined-behavior evidence comes from constant evaluation and warnings.
  Cases that constant evaluation can't express (some pointer and lifetime
  errors) will need sanitizers.
- EDG runs with its x86-64 Linux configuration only, so it skips the
  `-m32` tests and tests that rely on GCC/Clang warning flags.
- MSVC is not in the matrix.

## Next steps

1. Run the similarity check on both sections and rewrite anything flagged.
2. Decide MUL-D1 to MUL-D3. Report MUL-D2 and MUL-D3 to the EDG project if
   they are confirmed as defects.
3. Review the report compilers' results and promote EDG, Clang 18, and
   GCC 14 to `baseline`.
4. Add a `tidal-error` directive that matches the expected diagnostic, so
   `reject` tests check their cause.
5. Add sanitizer-based tests that run in the container images, which
   include the UBSan runtime.
6. Add EDG's 32-bit configuration as a second EDG matrix entry.
7. Give Tidal a public remote and continuous integration that runs the
   matrix and builds the PDF on every change.
8. Write a contributor guide: roles, how to claim a section, and the review
   steps a section passes before it counts as drafted.
9. Draft the remaining statements (`switch`, loops, jumps, labels,
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
