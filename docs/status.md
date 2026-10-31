# Status

Where the project stands and what comes next. This page describes the
current state only. The history of how it got here is in `git log`.

## Specification

| Section | Rules | Classes | Tests | Single-test rules | Overrides | Report mentions | Questions | Similarity |
|---|---|---|---|---|---|---|---|---|
| [Translation, execution, and behavior classes](../spec/foundations/translation.md) | 10 | 4 required, 3 reject, 1 undefined, 1 impl-defined, 1 recommended | 27 | 1 | 1 | 23 | Q-0112 | 0 flagged |
| [Fundamental types](../spec/foundations/types.md) | 13 | 7 required, 1 reject, 5 impl-defined | 51 | 3 | 0 | 4 |  | 0 flagged |
| [Objects, storage, and lifetime](../spec/foundations/objects.md) | 11 | 8 required, 2 undefined, 1 unspecified | 31 | 4 | 2 | 18 |  | 0 flagged |
| [Value categories and temporaries](../spec/foundations/values.md) | 14 | 8 required, 4 reject, 1 unspecified, 1 impl-defined | 51 | 6 | 1 | 13 | Q-0099 | 0 flagged |
| [Standard conversions](../spec/foundations/conversions.md) | 20 | 11 required, 6 reject, 1 undefined, 2 impl-defined | 60 | 6 | 2 | 23 | Q-0028, Q-0100 | 0 flagged |
| [Names, scopes, and lookup](../spec/foundations/names.md) | 12 | 6 required, 6 reject | 32 | 4 | 0 | 13 |  | 0 flagged |
| [Sequencing and evaluation order](../spec/foundations/evaluation-order.md) | 8 | 4 required, 1 undefined, 2 unspecified, 1 recommended | 21 | 1 | 0 | 10 |  | 0 flagged |
| [Constant expressions](../spec/foundations/constant-evaluation.md) | 25 | 6 required, 18 reject, 1 impl-defined | 74 | 7 | 6 | 72 | Q-0020, Q-0033, Q-0078, Q-0090, Q-0115, Q-0117 | 0 flagged |
| [Expression statements and blocks](../spec/statements/blocks.md) | 8 | 6 required, 1 reject, 1 recommended | 15 | 3 | 2 | 2 | Q-0096 | 0 flagged |
| [The if statement](../spec/statements/if.md) | 24 | 13 required, 11 reject | 65 | 9 | 0 | 12 |  | 0 flagged |
| [The switch statement](../spec/statements/switch.md) | 6 | 2 required, 4 reject | 32 | 0 | 1 | 9 | Q-0097 | 0 flagged |
| [Iteration statements](../spec/statements/loops.md) | 8 | 4 required, 4 reject | 19 | 3 | 1 | 2 | Q-0044 | 0 flagged |
| [The range-based for statement](../spec/statements/range-for.md) | 7 | 4 required, 3 reject | 26 | 1 | 5 | 5 | Q-0020, Q-0026, Q-0098 | 0 flagged |
| [Jump statements and labels](../spec/statements/jumps.md) | 15 | 8 required, 6 reject, 1 unspecified | 40 | 3 | 2 | 15 | Q-0041, Q-0097 | 0 flagged |
| [Primary expressions](../spec/expressions/primary.md) | 25 | 9 required, 8 reject, 1 undefined, 1 unspecified, 2 impl-defined, 3 conditionally-supported, 1 recommended | 94 | 6 | 4 | 9 | Q-0033, Q-0063, Q-0064 | 0 flagged |
| [decltype, unevaluated operands, and discarded values](../spec/expressions/decltype.md) | 11 | 5 required, 4 reject, 2 recommended | 49 | 4 | 3 | 5 | Q-0065, Q-0089 | 0 flagged |
| [Multiplicative operators](../spec/expressions/multiplicative.md) | 19 | 7 required, 3 reject, 5 undefined, 2 unspecified, 1 impl-defined, 1 recommended | 38 | 9 | 5 | 17 | Q-0001, Q-0002, Q-0003, Q-0004, Q-0005 | 0 flagged |
| [Additive operators](../spec/expressions/additive.md) | 16 | 7 required, 1 reject, 5 undefined, 1 unspecified, 1 impl-defined, 1 recommended | 41 | 9 | 6 | 7 | Q-0001, Q-0005, Q-0031, Q-0049 | 0 flagged |
| [Shift operators](../spec/expressions/shift.md) | 9 | 6 required, 1 reject, 1 undefined, 1 recommended | 25 | 3 | 0 | 4 |  | 0 flagged |
| [Comparison operators](../spec/expressions/comparison.md) | 25 | 13 required, 6 reject, 2 unspecified, 4 recommended | 67 | 9 | 9 | 10 | Q-0029, Q-0030, Q-0050, Q-0051, Q-0103, Q-0104 | 0 flagged |
| [Bitwise operators](../spec/expressions/bitwise.md) | 8 | 4 required, 1 reject, 3 recommended | 21 | 2 | 0 | 2 |  | 0 flagged |
| [Logical operators](../spec/expressions/logical.md) | 14 | 10 required, 2 reject, 2 recommended | 21 | 8 | 1 | 5 | Q-0101 | 0 flagged |
| [The conditional operator](../spec/expressions/conditional.md) | 15 | 8 required, 7 reject | 37 | 9 | 1 | 2 | Q-0102 | 0 flagged |
| [Assignment operators](../spec/expressions/assignment.md) | 13 | 7 required, 5 reject, 1 unspecified | 32 | 4 | 1 | 7 | Q-0032 | 0 flagged |
| [The comma operator](../spec/expressions/comma.md) | 8 | 5 required, 2 reject, 1 recommended | 17 | 1 | 0 | 1 |  | 0 flagged |
| [Unary operators](../spec/expressions/unary.md) | 21 | 10 required, 6 reject, 3 undefined, 1 impl-defined, 1 recommended | 60 | 4 | 1 | 10 | Q-0031 | 0 flagged |
| [Pointers to members](../spec/expressions/member-pointers.md) | 12 | 5 required, 5 reject, 1 undefined, 1 unspecified | 67 | 1 | 0 | 4 |  | 0 flagged |
| [Postfix expressions](../spec/expressions/postfix.md) | 17 | 7 required, 7 reject, 2 undefined, 1 conditionally-supported | 47 | 7 | 2 | 10 | Q-0034 | 0 flagged |
| [Casts](../spec/expressions/casts.md) | 19 | 8 required, 7 reject, 2 undefined, 2 impl-defined | 49 | 6 | 1 | 5 | Q-0017 | 0 flagged |
| [New and delete](../spec/expressions/new-delete.md) | 17 | 8 required, 6 reject, 2 undefined, 1 unspecified | 49 | 5 | 3 | 10 | Q-0035, Q-0036, Q-0091 | 0 flagged |
| [Lambda expressions](../spec/expressions/lambdas.md) | 27 | 15 required, 9 reject, 1 undefined, 1 unspecified, 1 recommended | 56 | 15 | 2 | 8 | Q-0037, Q-0092 | 0 flagged |
| [Placeholder types in variable declarations](../spec/declarations/auto.md) | 22 | 12 required, 10 reject | 49 | 9 | 0 | 5 | Q-0010 | 0 flagged |
| [Type aliases and class names](../spec/declarations/type-names.md) | 10 | 4 required, 5 reject, 1 no-diagnostic | 60 | 3 | 3 | 2 | Q-0066, Q-0067, Q-0093 | 0 flagged |
| [Storage duration and linkage](../spec/declarations/storage-linkage.md) | 23 | 14 required, 6 reject, 1 undefined, 2 no-diagnostic | 54 | 11 | 2 | 9 | Q-0011, Q-0045, Q-0105 | 0 flagged |
| [The inline specifier](../spec/declarations/inline.md) | 8 | 4 required, 2 reject, 2 no-diagnostic | 34 | 2 | 2 | 0 | Q-0068 | 0 flagged |
| [Declarators and cv-qualifiers](../spec/declarations/declarators.md) | 34 | 17 required, 14 reject, 1 undefined, 1 impl-defined, 1 recommended | 82 | 8 | 2 | 14 | Q-0012, Q-0106 | 0 flagged |
| [Alignment](../spec/declarations/alignment.md) | 7 | 2 required, 3 reject, 1 impl-defined, 1 no-diagnostic | 31 | 2 | 13 | 1 | Q-0069, Q-0070, Q-0071, Q-0072, Q-0073 | 0 flagged |
| [Volatile](../spec/declarations/volatile.md) | 11 | 4 required, 5 reject, 1 undefined, 1 recommended | 33 | 3 | 1 | 2 | Q-0080 | 0 flagged |
| [Forms of initialization](../spec/declarations/initialization/forms.md) | 8 | 5 required, 2 reject, 1 undefined | 22 | 4 | 1 | 8 | Q-0013, Q-0039 | 0 flagged |
| [List initialization and aggregates](../spec/declarations/initialization/list.md) | 16 | 8 required, 8 reject | 54 | 5 | 2 | 9 | Q-0042, Q-0043 | 0 flagged |
| [Reference binding](../spec/declarations/initialization/references.md) | 11 | 6 required, 4 reject, 1 undefined | 31 | 2 | 3 | 9 | Q-0040, Q-0109, Q-0110 | 0 flagged |
| [Initialization of static and thread storage](../spec/declarations/initialization/static.md) | 6 | 2 required, 2 unspecified, 2 impl-defined | 11 | 2 | 0 | 3 | Q-0014 | 0 flagged |
| [Structured bindings](../spec/declarations/structured-bindings.md) | 16 | 9 required, 7 reject | 49 | 6 | 4 | 4 | Q-0007, Q-0015, Q-0016, Q-0048, Q-0107 | 0 flagged |
| [Enumerations](../spec/declarations/enumerations.md) | 20 | 9 required, 8 reject, 1 undefined, 1 unspecified, 1 impl-defined | 58 | 4 | 3 | 17 | Q-0017, Q-0018, Q-0047, Q-0114 | 0 flagged |
| [Namespaces and using](../spec/declarations/namespaces.md) | 17 | 8 required, 9 reject | 41 | 5 | 1 | 1 | Q-0019 | 0 flagged |
| [Linkage specifications](../spec/declarations/linkage-specifications.md) | 7 | 3 required, 2 reject, 2 impl-defined | 29 | 1 | 4 | 0 | Q-0074, Q-0075, Q-0076 | 0 flagged |
| [Standard attributes](../spec/declarations/attributes.md) | 17 | 6 required, 3 reject, 2 undefined, 1 impl-defined, 5 recommended | 55 | 6 | 10 | 11 | Q-0020, Q-0021, Q-0046, Q-0108 | 0 flagged |
| [Function definitions](../spec/declarations/functions/definitions.md) | 14 | 3 required, 6 reject, 1 undefined, 3 impl-defined, 1 recommended | 40 | 5 | 6 | 13 | Q-0022, Q-0024, Q-0076 | 0 flagged |
| [Default arguments](../spec/declarations/functions/default-arguments.md) | 9 | 3 required, 5 reject, 1 no-diagnostic | 21 | 3 | 2 | 10 | Q-0038, Q-0111 | 0 flagged |
| [Deleted and defaulted functions](../spec/declarations/functions/deleted.md) | 5 | 1 required, 4 reject | 13 | 2 | 1 | 6 | Q-0007, Q-0023 | 0 flagged |
| [Return type deduction](../spec/declarations/functions/return-deduction.md) | 7 | 2 required, 4 reject, 1 undefined | 22 | 1 | 0 | 11 |  | 0 flagged |
| [Exception specifications](../spec/declarations/exception-specifications.md) | 8 | 3 required, 3 reject, 1 impl-defined, 1 no-diagnostic | 36 | 2 | 6 | 3 | Q-0077, Q-0078, Q-0079, Q-0094, Q-0095 | 0 flagged |
| [The constexpr, consteval, and constinit specifiers](../spec/declarations/constexpr.md) | 12 | 3 required, 9 reject | 63 | 2 | 5 | 27 | Q-0020, Q-0025 | 0 flagged |
| [Static assertions](../spec/declarations/static-assert.md) | 8 | 4 required, 4 reject | 28 | 0 | 0 | 4 |  | 0 flagged |
| [Class definitions and members](../spec/classes/members.md) | 27 | 14 required, 11 reject, 1 impl-defined, 1 no-diagnostic | 84 | 12 | 1 | 16 | Q-0027 | 0 flagged |
| [Access control and friends](../spec/classes/access.md) | 19 | 8 required, 11 reject | 48 | 9 | 1 | 4 | Q-0052 | 0 flagged |
| [Constructors and destructors](../spec/classes/constructors.md) | 30 | 11 required, 14 reject, 2 undefined, 2 no-diagnostic, 1 recommended | 122 | 7 | 6 | 18 | Q-0053, Q-0054, Q-0082, Q-0083, Q-0084 | 0 flagged |
| [List initialization of classes](../spec/classes/initializer-lists.md) | 10 | 5 required, 5 reject | 30 | 1 | 1 | 7 | Q-0081 | 0 flagged |
| [Copy and move](../spec/classes/copy-move.md) | 26 | 13 required, 10 reject, 2 unspecified, 1 recommended | 88 | 3 | 2 | 21 | Q-0055 | 0 flagged |
| [Derived classes](../spec/classes/derived.md) | 22 | 9 required, 12 reject, 1 undefined | 73 | 4 | 1 | 4 | Q-0085 | 0 flagged |
| [Virtual functions and abstract classes](../spec/classes/virtual.md) | 20 | 7 required, 9 reject, 1 undefined, 2 no-diagnostic, 1 recommended | 107 | 3 | 3 | 5 | Q-0056, Q-0086, Q-0087 | 0 flagged |
| [Unions and bit-fields](../spec/classes/unions.md) | 23 | 11 required, 8 reject, 1 undefined, 2 impl-defined, 1 recommended | 86 | 5 | 3 | 9 | Q-0057, Q-0058, Q-0088 | 0 flagged |
| [Conversion functions](../spec/classes/conversion-functions.md) | 12 | 5 required, 6 reject, 1 recommended | 60 | 1 | 1 | 2 | Q-0059 | 0 flagged |
| [Operator overloading](../spec/classes/operators.md) | 16 | 8 required, 6 reject, 1 unspecified, 1 recommended | 89 | 3 | 1 | 19 | Q-0060 | 0 flagged |
| [Defaulted comparisons](../spec/classes/comparisons.md) | 12 | 5 required, 5 reject, 1 no-diagnostic, 1 recommended | 63 | 2 | 2 | 5 | Q-0061, Q-0062 | 0 flagged |
| [Throw expressions and the exception object](../spec/exceptions/throw.md) | 11 | 9 required, 1 reject | 27 | 4 | 0 | 12 |  | 0 flagged |
| [Try blocks and handlers](../spec/exceptions/handlers.md) | 17 | 7 required, 6 reject, 1 undefined, 3 recommended | 74 | 5 | 2 | 8 |  | 0 flagged |
| [Unwinding and termination](../spec/exceptions/unwinding.md) | 8 | 6 required, 1 impl-defined, 1 recommended | 30 | 1 | 0 | 9 |  | 0 flagged |
| [Glossary](../spec/glossary.md) | 19 terms | | | | | | | 0 flagged |

`tools/build_site.py` builds the specification as a searchable HTML site
(decision 0016). CI builds it on every change and publishes it to
https://sempuki.github.io/tidal/ after each push to `main`. Pages use
the structure and writing rules in `docs/style-guide.md`. Every page
opens with its principles (decision 0024), and the Foundations chapter
holds the models that the other pages cite. The latest similarity report, covering every page, is
`reports/similarity/2026-10-31-types.txt`. The table above comes from
`tools/status_table.py`.

## Test matrix

All compilers are baseline. 3176/3176 tests pass on each. Tests may span several translation units (decision 0014). CI runs the
containerized compilers on every push and pull request (decision 0012).

| Compiler | Where | Notes |
|---|---|---|
| Clang 22.1.8 | host, and container (apt.llvm.org) | |
| GCC 16.2.1 | host | |
| GCC 16.2.0 | container (official image) | skips `-m32` tests |
| Clang 18.1.3 | container | |
| GCC 14.2.0 | container | |
| EDG 7.0 | container | front end only; skips `-m32`, warning-flag, and sanitizer tests; per-compiler expectations on 61 tests, each with a note that names its question or the implementation choice |

Full results are in `tests/RESULTS.md`.

## Open questions

One hundred and seventeen questions are open for opinions, each with a
provisional answer the spec follows. Nine of them have the status
"compiler defect": their answer is that one compiler or an older version
is wrong (Q-0002, Q-0003, Q-0013, Q-0015, Q-0016, Q-0023, Q-0026,
Q-0044, Q-0115). Q-0009 (license) is settled. See `questions/README.md`.

## Known limits of the specification text

- The glossary covers the terms used so far. Terms marked *(referenced)*
  belong to topics that Tidal does not specify yet.
- 42 *(referenced)* markers remain. Each names a topic with no page,
  such as overload resolution, argument-dependent lookup, templates and
  concepts, the library, threads, conversion ranks, or literal types.

## Known limits of the tooling

- The clean-room front end passes 2581 of 3176 tests, with no false
  rejects.
  `tools/frontend_check.py` fails when a test that it passed no longer
  passes (`reports/clean-room/rust-frontend/conformance.tsv`).

- 1451 tests carry `tidal-error` patterns, which check that a compiler
  rejects them, fails to link them, or stops them at run time, for the
  reason their rule gives.
  About 50 tests have no pattern for some compiler family, because that
  family reports only a generic parse or constant-evaluation error.
- Undefined-behavior evidence comes from constant evaluation, warnings,
  and the UBSan and ASan sanitizers (25 tests, decision 0028). Errors
  that neither sanitizer reports (a second destruction of a stack
  object, a modified `const` object) have constant-evaluation tests
  only.
- EDG runs with its x86-64 Linux configuration only.
- MSVC is not in the matrix.
- The `if` section draws on Clang's source only. EDG's parser is unread
  for it.
- CI caches test results between runs (`actions/cache`). A full rebuild
  of the cache happens when the cache key format changes
  (`OUTCOME_VERSION` in `tools/run_tests.py`) or a compiler image changes.

## Topics in scope with no page yet

None. Each topic from the gap reviews (`reports/review/`) has a page. Exception handling is a later chapter (decision 0023). The
forward-progress rule belongs with threads, which are out of scope.

## Next steps

`docs/roadmap.md` orders the work (decision 0033). Phase 1 is done. The
current phase is phase 2, foundations that later pages need:

1. Item S1: definitions of the trivial special member functions.
2. An EDG entry for the `-m32` target. EDG ships an i686 configuration,
   which builds EDG itself as a 32-bit program.
3. Item F: a grammar notation, then one grammar in which every
   nonterminal is defined. `tools/check_spec.py` then makes an undefined
   nonterminal an error.
4. Item G1: keywords, reserved names, and the literal suffix grammar.
5. Item K: the language-support types, then the task-level guides.
6. Propose exercises X1 to X4, after item Y.

Waiting on the maintainer:

- Ask the reviewer who prompted the principles-first change to read the
  Foundations chapter and two topic pages (decision 0024).
- Approve reporting the EDG behavior in Q-0002 and Q-0003 to the EDG
  project.

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
session for the clean-room writer. Review and adjudication are not included and are
expected to dominate once reviewers are involved.
