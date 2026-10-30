# Status

Where the project stands and what comes next. This page describes the
current state only. The history of how it got here is in `git log`.

## Specification

| Section | Rules | Classes | Tests | Questions | Similarity |
|---|---|---|---|---|---|
| [The if statement](../spec/statements/if.md) | 20 | 14 required, 6 reject | 56 | Q-0007 | 0 flagged |
| [Multiplicative operators](../spec/expressions/multiplicative.md) | 15 | 6 required, 2 reject, 5 undefined, 1 unspecified, 1 impl-defined | 27 | Q-0001 to Q-0005 | 0 flagged |
| [Placeholder types in variable declarations](../spec/declarations/auto.md) | 13 | 11 required, 2 reject | 40 | Q-0010 | 0 flagged |
| [Storage duration and linkage](../spec/declarations/storage-linkage.md) | 16 | 11 required, 3 reject, 2 no-diagnostic | 40 | Q-0011 | 0 flagged |
| [Declarators and cv-qualifiers](../spec/declarations/declarators.md) | 18 | 13 required, 3 reject, 1 undefined, 1 impl-defined | 61 | Q-0012 | 0 flagged |
| [Initialization](../spec/declarations/initialization/) (4 files) | 22 | 17 required, 1 reject, 2 undefined, 2 unspecified | 74 | Q-0013, Q-0014 | 0 flagged |
| [Structured bindings](../spec/declarations/structured-bindings.md) | 8 | 6 required, 2 reject | 36 | Q-0015, Q-0016 | 0 flagged |
| [Enumerations](../spec/declarations/enumerations.md) | 10 | 8 required, 1 impl-defined, 1 undefined | 41 | Q-0017, Q-0018 | 0 flagged |
| [Namespaces and using](../spec/declarations/namespaces.md) | 8 | 8 required | 32 | Q-0019 | 0 flagged |
| [Standard attributes](../spec/declarations/attributes.md) | 15 | 5 required, 2 reject, 2 undefined, 1 impl-defined, 5 recommended | 29 | Q-0020, Q-0021 | 0 flagged |
| [Functions](../spec/declarations/functions/) (4 pages) | 21 | 8 required, 10 reject, 1 undefined, 1 impl-defined, 1 recommended | 67 | Q-0022, Q-0023, Q-0024 | 0 flagged |
| [constexpr, consteval, and constinit](../spec/declarations/constexpr.md) | 10 | 2 required, 8 reject | 41 | Q-0020, Q-0025 | 0 flagged |
| [Glossary](../spec/glossary.md) | 15 terms | | | | 0 flagged |

`tools/build_site.py` builds the specification as a searchable HTML site
(decision 0016). CI builds it on every change and publishes it to
https://sempuki.github.io/tidal/ after each push to `main`. Pages use
the structure and writing rules in `docs/style-guide.md`. All pages are
converted. The latest similarity report, covering every page, is
`reports/similarity/2026-10-30-format-conversion.txt`, and the constexpr page's
is `reports/similarity/2026-10-30-constexpr.txt`.

## Test matrix

All compilers are baseline. 544/544 tests pass on each. Tests may span several translation units (decision 0014). CI runs the
containerized compilers on every push and pull request (decision 0012).

| Compiler | Where | Notes |
|---|---|---|
| Clang 22.1.8 | host, and container (apt.llvm.org) | |
| GCC 16.2.1 | host | |
| GCC 16.2.0 | container (official image) | skips `-m32` tests |
| Clang 18.1.3 | container | |
| GCC 14.2.0 | container | |
| EDG 7.0 | container | front end only; skips `-m32` and warning-flag tests; per-compiler expectations on 4 tests (Q-0002, Q-0003) |

Full results are in `tests/RESULTS.md`.

## Open questions

Twenty-four questions are open for opinions, each with a provisional answer the
spec follows. Q-0009 (license) is settled. See `questions/README.md`.

## Known limits of the specification text

- Some rules mix a legality fact with a run-time fact, and sit in the
  segment of the larger part: STO-4, DCL-3, SB-1, INIT-4, INIT-5, LIST-3,
  REF-2, and REF-3. Each should be split into two rules with new IDs, as
  ATTR-3, ATTR-10, FN-4, and FN-3 were.
- The glossary covers the terms used so far. Terms marked *(referenced)*
  belong to topics that Tidal does not specify yet.

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

1. Write a contributor guide: roles, how to claim a section, and the steps
   a section passes before it counts as drafted.
2. Report the EDG behavior in Q-0002 and Q-0003 to the EDG project.
3. Add a `tidal-error` directive that matches the expected diagnostic, so
   `reject` tests check their cause.
4. Add sanitizer-based tests that run in the container images.
5. Add EDG's 32-bit configuration as a second EDG matrix entry.
6. Read GCC's and EDG's sanitized source for the `if` section.
7. Split the rules listed under "Known limits of the specification text"
   so each fits one segment.
8. Draft the remaining statements (`switch`, loops, jumps, labels,
   expression and compound statements) and expression families in order
   of dependency.

## Size estimate

The `if` section is typical of a statement. The multiplicative section is
typical of a simple operator family.

- Statements: about 8 sections of the `if` section's size.
- Declarations: 10 sections (decision 0013). Placeholder types are typical
  of the smaller ones. Initialization and function definitions are about
  four times that size each.
- Expressions: about 20 operator families like the multiplicative section,
  plus large topics (value categories and sequencing, constant expressions,
  lambdas, casts, `new` and `delete`, function calls) at roughly four times
  that size each.

That totals about 66 section-units. Drafting one unit takes one working
session for the writer. Review and adjudication are not included and are
expected to dominate once reviewers are involved.
