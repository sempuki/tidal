# Status

Where the project stands and what comes next. This page describes the
current state only. The history of how it got here is in `git log`.

## Specification

| Section | Rules | Classes | Tests | Questions | Similarity |
|---|---|---|---|---|---|
| [Expression statements and blocks](../spec/statements/blocks.md) | 6 | 5 required, 1 recommended | 11 |  | 0 flagged |
| [The if statement](../spec/statements/if.md) | 20 | 14 required, 6 reject | 61 |  | 0 flagged |
| [The switch statement](../spec/statements/switch.md) | 5 | 2 required, 3 reject | 26 |  | 0 flagged |
| [Iteration statements](../spec/statements/loops.md) | 6 | 4 required, 2 reject | 14 | Q-0044 | 0 flagged |
| [The range-based for statement](../spec/statements/range-for.md) | 6 | 4 required, 2 reject | 22 | Q-0020, Q-0026 | 0 flagged |
| [Jump statements and labels](../spec/statements/jumps.md) | 11 | 8 required, 2 reject, 1 unspecified | 30 | Q-0041 | 0 flagged |
| [Value categories and temporaries](../spec/expressions/value-categories.md) | 7 | 6 required, 1 unspecified | 19 |  | 0 flagged |
| [Standard conversions](../spec/expressions/conversions.md) | 16 | 9 required, 4 reject, 1 undefined, 2 impl-defined | 34 | Q-0028 | 0 flagged |
| [Sequencing and evaluation order](../spec/expressions/sequencing.md) | 6 | 4 required, 1 undefined, 1 unspecified | 16 |  | 0 flagged |
| [Multiplicative operators](../spec/expressions/multiplicative.md) | 16 | 6 required, 2 reject, 5 undefined, 1 unspecified, 1 impl-defined, 1 recommended | 28 | Q-0001, Q-0002, Q-0003, Q-0004, Q-0005 | 0 flagged |
| [Additive operators](../spec/expressions/additive.md) | 15 | 7 required, 1 reject, 4 undefined, 1 unspecified, 1 impl-defined, 1 recommended | 33 | Q-0005, Q-0031, Q-0049 | 0 flagged |
| [Shift operators](../spec/expressions/shift.md) | 9 | 6 required, 1 reject, 1 undefined, 1 recommended | 20 |  | 0 flagged |
| [Comparison operators](../spec/expressions/comparison.md) | 21 | 12 required, 3 reject, 2 unspecified, 4 recommended | 46 | Q-0029, Q-0030, Q-0050, Q-0051 | 0 flagged |
| [Bitwise operators](../spec/expressions/bitwise.md) | 8 | 4 required, 1 reject, 3 recommended | 18 |  | 0 flagged |
| [Logical operators](../spec/expressions/logical.md) | 11 | 8 required, 1 reject, 2 recommended | 18 |  | 0 flagged |
| [The conditional operator](../spec/expressions/conditional.md) | 10 | 6 required, 4 reject | 30 |  | 0 flagged |
| [Assignment operators](../spec/expressions/assignment.md) | 8 | 7 required, 1 reject | 24 | Q-0032 | 0 flagged |
| [The comma operator](../spec/expressions/comma.md) | 7 | 5 required, 1 reject, 1 recommended | 13 |  | 0 flagged |
| [Unary operators](../spec/expressions/unary.md) | 20 | 10 required, 6 reject, 2 undefined, 1 impl-defined, 1 recommended | 39 |  | 0 flagged |
| [Postfix expressions](../spec/expressions/postfix.md) | 13 | 7 required, 4 reject, 2 undefined | 28 | Q-0034 | 0 flagged |
| [Casts](../spec/expressions/casts.md) | 12 | 4 required, 6 reject, 1 undefined, 1 impl-defined | 25 |  | 0 flagged |
| [Constant expressions](../spec/expressions/constant-expressions.md) | 13 | 2 required, 10 reject, 1 impl-defined | 32 | Q-0020, Q-0033 | 0 flagged |
| [New and delete](../spec/expressions/new-delete.md) | 9 | 4 required, 3 reject, 2 undefined | 31 | Q-0035, Q-0036 | 0 flagged |
| [Lambda expressions](../spec/expressions/lambdas.md) | 16 | 14 required, 2 reject | 36 | Q-0037 | 0 flagged |
| [Placeholder types in variable declarations](../spec/declarations/auto.md) | 14 | 12 required, 2 reject | 43 | Q-0010 | 0 flagged |
| [Storage duration and linkage](../spec/declarations/storage-linkage.md) | 20 | 14 required, 3 reject, 1 undefined, 2 no-diagnostic | 47 | Q-0011, Q-0045 | 0 flagged |
| [Declarators and cv-qualifiers](../spec/declarations/declarators.md) | 23 | 16 required, 4 reject, 1 undefined, 1 impl-defined, 1 recommended | 75 | Q-0012 | 0 flagged |
| [Forms of initialization](../spec/declarations/initialization/forms.md) | 5 | 4 required, 1 undefined | 18 | Q-0013, Q-0039 | 0 flagged |
| [List initialization and aggregates](../spec/declarations/initialization/list.md) | 10 | 8 required, 2 reject | 48 | Q-0042, Q-0043 | 0 flagged |
| [Reference binding](../spec/declarations/initialization/references.md) | 7 | 6 required, 1 undefined | 24 | Q-0040 | 0 flagged |
| [Initialization of static and thread storage](../spec/declarations/initialization/static.md) | 5 | 2 required, 2 unspecified, 1 impl-defined | 9 | Q-0014 | 0 flagged |
| [Structured bindings](../spec/declarations/structured-bindings.md) | 10 | 8 required, 2 reject | 43 | Q-0007, Q-0015, Q-0016, Q-0048 | 0 flagged |
| [Enumerations](../spec/declarations/enumerations.md) | 10 | 8 required, 1 undefined, 1 impl-defined | 46 | Q-0017, Q-0018, Q-0047 | 0 flagged |
| [Namespaces and using](../spec/declarations/namespaces.md) | 8 | 8 required | 36 | Q-0019 | 0 flagged |
| [Standard attributes](../spec/declarations/attributes.md) | 16 | 6 required, 2 reject, 2 undefined, 1 impl-defined, 5 recommended | 36 | Q-0020, Q-0021, Q-0046 | 0 flagged |
| [Function definitions](../spec/declarations/functions/definitions.md) | 10 | 3 required, 3 reject, 1 undefined, 2 impl-defined, 1 recommended | 32 | Q-0022, Q-0024 | 0 flagged |
| [Default arguments](../spec/declarations/functions/default-arguments.md) | 6 | 3 required, 3 reject | 18 | Q-0038 | 0 flagged |
| [Deleted and defaulted functions](../spec/declarations/functions/deleted.md) | 5 | 1 required, 4 reject | 13 | Q-0007, Q-0023 | 0 flagged |
| [Return type deduction](../spec/declarations/functions/return-deduction.md) | 6 | 2 required, 3 reject, 1 undefined | 16 |  | 0 flagged |
| [The constexpr, consteval, and constinit specifiers](../spec/declarations/constexpr.md) | 11 | 3 required, 8 reject | 46 | Q-0020, Q-0025 | 0 flagged |
| [Class definitions and members](../spec/classes/members.md) | 15 | 11 required, 3 reject, 1 impl-defined | 59 | Q-0027 | 0 flagged |
| [Access control and friends](../spec/classes/access.md) | 13 | 7 required, 6 reject | 38 | Q-0052 | 0 flagged |
| [Glossary](../spec/glossary.md) | 15 terms | | | | 0 flagged |

`tools/build_site.py` builds the specification as a searchable HTML site
(decision 0016). CI builds it on every change and publishes it to
https://sempuki.github.io/tidal/ after each push to `main`. Pages use
the structure and writing rules in `docs/style-guide.md`. All pages are
converted. The latest similarity report, covering every page, is
`reports/similarity/2026-10-30-access.txt`. The table above comes from
`tools/status_table.py`.

## Test matrix

All compilers are baseline. 1301/1301 tests pass on each. Tests may span several translation units (decision 0014). CI runs the
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

Fifty-one questions are open for opinions, each with a provisional answer the
spec follows. Q-0009 (license) is settled. See `questions/README.md`.

## Known limits of the specification text

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

## Topics in scope with no page yet

From the gap review (`reports/review/2026-10-30-gap-review.md`). Each
needs a page through the full pipeline.

- Expressions: primary expressions (names, `this`, literals, parentheses),
  the pointer-to-member operators, discarded-value expressions and
  unevaluated operands, `decltype`, temporaries in general, and `throw`
  expressions (or a recorded decision to exclude exceptions).
- Declarations: `typedef` and alias declarations, `static_assert`, the
  `inline` specifier, `alignas` and alignment, linkage specifications,
  elaborated type specifiers and forward class declarations, the
  `explicit` specifier, `noexcept` in function types, pointer-to-member
  declarators, and `volatile`.
- Classes: the sections of decision 0017, plus initialization by
  constructors and initializer-list constructors.
- Statements: the forward-progress rule belongs with threads, which are
  out of scope.

## Next steps

1. Write a contributor guide: roles, how to claim a section, and the steps
   a section passes before it counts as drafted.
2. Report the EDG behavior in Q-0002 and Q-0003 to the EDG project.
3. Add a `tidal-error` directive that matches the expected diagnostic, so
   `reject` tests check their cause.
4. Add sanitizer-based tests that run in the container images.
5. Add EDG's 32-bit configuration as a second EDG matrix entry.
6. Read GCC's and EDG's sanitized source for the `if` section.
7. Continue classes in the order set by decision 0017. Next: constructors
   and destructors. Statements, expressions, and declarations in the
   charter's scope all have pages.

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
