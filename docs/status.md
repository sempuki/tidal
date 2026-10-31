# Status

Where the project stands and what comes next. This page describes the
current state only. The history of how it got here is in `git log`.

## Specification

| Section | Rules | Classes | Tests | Questions | Similarity |
|---|---|---|---|---|---|
| [Translation, execution, and behavior classes](../spec/foundations/translation.md) | 8 | 4 required, 2 reject, 1 undefined, 1 recommended | 26 | Q-0112 | 0 flagged |
| [Objects, storage, and lifetime](../spec/foundations/objects.md) | 11 | 8 required, 2 undefined, 1 unspecified | 27 |  | 0 flagged |
| [Value categories and temporaries](../spec/foundations/values.md) | 11 | 8 required, 1 reject, 1 unspecified, 1 impl-defined | 41 | Q-0099 | 0 flagged |
| [Standard conversions](../spec/foundations/conversions.md) | 17 | 9 required, 5 reject, 1 undefined, 2 impl-defined | 47 | Q-0028, Q-0100 | 0 flagged |
| [Names, scopes, and lookup](../spec/foundations/names.md) | 7 | 6 required, 1 reject | 26 |  | 0 flagged |
| [Sequencing and evaluation order](../spec/foundations/evaluation-order.md) | 6 | 4 required, 1 undefined, 1 unspecified | 18 |  | 0 flagged |
| [Constant expressions](../spec/foundations/constant-evaluation.md) | 18 | 4 required, 13 reject, 1 impl-defined | 46 | Q-0020, Q-0033, Q-0078, Q-0090 | 0 flagged |
| [Expression statements and blocks](../spec/statements/blocks.md) | 7 | 6 required, 1 recommended | 15 | Q-0096 | 0 flagged |
| [The if statement](../spec/statements/if.md) | 21 | 14 required, 7 reject | 65 |  | 0 flagged |
| [The switch statement](../spec/statements/switch.md) | 5 | 2 required, 3 reject | 32 | Q-0097 | 0 flagged |
| [Iteration statements](../spec/statements/loops.md) | 7 | 4 required, 3 reject | 19 | Q-0044 | 0 flagged |
| [The range-based for statement](../spec/statements/range-for.md) | 6 | 4 required, 2 reject | 26 | Q-0020, Q-0026, Q-0098 | 0 flagged |
| [Jump statements and labels](../spec/statements/jumps.md) | 12 | 8 required, 3 reject, 1 unspecified | 40 | Q-0041, Q-0097 | 0 flagged |
| [Primary expressions](../spec/expressions/primary.md) | 23 | 9 required, 6 reject, 1 undefined, 1 unspecified, 2 impl-defined, 3 conditionally-supported, 1 recommended | 94 | Q-0063, Q-0064 | 0 flagged |
| [decltype, unevaluated operands, and discarded values](../spec/expressions/decltype.md) | 9 | 5 required, 2 reject, 2 recommended | 49 | Q-0065, Q-0089 | 0 flagged |
| [Multiplicative operators](../spec/expressions/multiplicative.md) | 18 | 7 required, 2 reject, 5 undefined, 2 unspecified, 1 impl-defined, 1 recommended | 34 | Q-0001, Q-0002, Q-0003, Q-0004, Q-0005 | 0 flagged |
| [Additive operators](../spec/expressions/additive.md) | 16 | 7 required, 1 reject, 5 undefined, 1 unspecified, 1 impl-defined, 1 recommended | 38 | Q-0001, Q-0005, Q-0031, Q-0049 | 0 flagged |
| [Shift operators](../spec/expressions/shift.md) | 9 | 6 required, 1 reject, 1 undefined, 1 recommended | 23 |  | 0 flagged |
| [Comparison operators](../spec/expressions/comparison.md) | 22 | 13 required, 3 reject, 2 unspecified, 4 recommended | 67 | Q-0029, Q-0030, Q-0050, Q-0051, Q-0103, Q-0104 | 0 flagged |
| [Bitwise operators](../spec/expressions/bitwise.md) | 8 | 4 required, 1 reject, 3 recommended | 21 |  | 0 flagged |
| [Logical operators](../spec/expressions/logical.md) | 13 | 10 required, 1 reject, 2 recommended | 21 | Q-0101 | 0 flagged |
| [The conditional operator](../spec/expressions/conditional.md) | 12 | 8 required, 4 reject | 37 | Q-0102 | 0 flagged |
| [Assignment operators](../spec/expressions/assignment.md) | 10 | 7 required, 2 reject, 1 unspecified | 29 | Q-0032 | 0 flagged |
| [The comma operator](../spec/expressions/comma.md) | 8 | 5 required, 2 reject, 1 recommended | 17 |  | 0 flagged |
| [Unary operators](../spec/expressions/unary.md) | 21 | 10 required, 6 reject, 3 undefined, 1 impl-defined, 1 recommended | 54 | Q-0031 | 0 flagged |
| [Pointers to members](../spec/expressions/member-pointers.md) | 11 | 5 required, 4 reject, 1 undefined, 1 unspecified | 66 |  | 0 flagged |
| [Postfix expressions](../spec/expressions/postfix.md) | 15 | 7 required, 5 reject, 2 undefined, 1 conditionally-supported | 39 | Q-0034 | 0 flagged |
| [Casts](../spec/expressions/casts.md) | 18 | 8 required, 6 reject, 2 undefined, 2 impl-defined | 45 | Q-0017 | 0 flagged |
| [New and delete](../spec/expressions/new-delete.md) | 15 | 8 required, 4 reject, 2 undefined, 1 unspecified | 46 | Q-0035, Q-0036, Q-0091 | 0 flagged |
| [Lambda expressions](../spec/expressions/lambdas.md) | 21 | 15 required, 3 reject, 1 undefined, 1 unspecified, 1 recommended | 55 | Q-0037, Q-0092 | 0 flagged |
| [Placeholder types in variable declarations](../spec/declarations/auto.md) | 15 | 12 required, 3 reject | 49 | Q-0010 | 0 flagged |
| [Type aliases and class names](../spec/declarations/type-names.md) | 7 | 4 required, 2 reject, 1 no-diagnostic | 60 | Q-0066, Q-0067, Q-0093 | 0 flagged |
| [Storage duration and linkage](../spec/declarations/storage-linkage.md) | 20 | 14 required, 3 reject, 1 undefined, 2 no-diagnostic | 54 | Q-0011, Q-0045, Q-0105 | 0 flagged |
| [The inline specifier](../spec/declarations/inline.md) | 7 | 4 required, 1 reject, 2 no-diagnostic | 34 | Q-0068 | 0 flagged |
| [Declarators and cv-qualifiers](../spec/declarations/declarators.md) | 25 | 17 required, 5 reject, 1 undefined, 1 impl-defined, 1 recommended | 80 | Q-0012, Q-0106 | 0 flagged |
| [Alignment](../spec/declarations/alignment.md) | 6 | 2 required, 2 reject, 1 impl-defined, 1 no-diagnostic | 31 | Q-0069, Q-0070, Q-0071, Q-0072, Q-0073 | 0 flagged |
| [Volatile](../spec/declarations/volatile.md) | 7 | 4 required, 1 reject, 1 undefined, 1 recommended | 33 | Q-0080 | 0 flagged |
| [Forms of initialization](../spec/declarations/initialization/forms.md) | 6 | 5 required, 1 undefined | 22 | Q-0013, Q-0039 | 0 flagged |
| [List initialization and aggregates](../spec/declarations/initialization/list.md) | 10 | 8 required, 2 reject | 54 | Q-0042, Q-0043 | 0 flagged |
| [Reference binding](../spec/declarations/initialization/references.md) | 7 | 6 required, 1 undefined | 31 | Q-0040, Q-0109, Q-0110 | 0 flagged |
| [Initialization of static and thread storage](../spec/declarations/initialization/static.md) | 6 | 2 required, 2 unspecified, 2 impl-defined | 11 | Q-0014 | 0 flagged |
| [Structured bindings](../spec/declarations/structured-bindings.md) | 12 | 9 required, 3 reject | 49 | Q-0007, Q-0015, Q-0016, Q-0048, Q-0107 | 0 flagged |
| [Enumerations](../spec/declarations/enumerations.md) | 12 | 9 required, 1 reject, 1 undefined, 1 impl-defined | 52 | Q-0017, Q-0018, Q-0047 | 0 flagged |
| [Namespaces and using](../spec/declarations/namespaces.md) | 9 | 8 required, 1 reject | 41 | Q-0019 | 0 flagged |
| [Linkage specifications](../spec/declarations/linkage-specifications.md) | 6 | 3 required, 1 reject, 2 impl-defined | 29 | Q-0074, Q-0075, Q-0076 | 0 flagged |
| [Standard attributes](../spec/declarations/attributes.md) | 17 | 6 required, 3 reject, 2 undefined, 1 impl-defined, 5 recommended | 46 | Q-0020, Q-0021, Q-0046, Q-0108 | 0 flagged |
| [Function definitions](../spec/declarations/functions/definitions.md) | 12 | 3 required, 4 reject, 1 undefined, 3 impl-defined, 1 recommended | 38 | Q-0022, Q-0024, Q-0076 | 0 flagged |
| [Default arguments](../spec/declarations/functions/default-arguments.md) | 7 | 3 required, 3 reject, 1 no-diagnostic | 21 | Q-0038, Q-0111 | 0 flagged |
| [Deleted and defaulted functions](../spec/declarations/functions/deleted.md) | 5 | 1 required, 4 reject | 13 | Q-0007, Q-0023 | 0 flagged |
| [Return type deduction](../spec/declarations/functions/return-deduction.md) | 6 | 2 required, 3 reject, 1 undefined | 20 |  | 0 flagged |
| [Exception specifications](../spec/declarations/exception-specifications.md) | 6 | 3 required, 1 reject, 1 impl-defined, 1 no-diagnostic | 36 | Q-0077, Q-0078, Q-0079, Q-0094, Q-0095 | 0 flagged |
| [The constexpr, consteval, and constinit specifiers](../spec/declarations/constexpr.md) | 11 | 3 required, 8 reject | 53 | Q-0020, Q-0025 | 0 flagged |
| [Static assertions](../spec/declarations/static-assert.md) | 5 | 4 required, 1 reject | 28 |  | 0 flagged |
| [Class definitions and members](../spec/classes/members.md) | 20 | 14 required, 4 reject, 1 impl-defined, 1 no-diagnostic | 84 | Q-0027 | 0 flagged |
| [Access control and friends](../spec/classes/access.md) | 16 | 8 required, 8 reject | 46 | Q-0052 | 0 flagged |
| [Constructors and destructors](../spec/classes/constructors.md) | 26 | 11 required, 10 reject, 2 undefined, 2 no-diagnostic, 1 recommended | 120 | Q-0053, Q-0054, Q-0082, Q-0083, Q-0084 | 0 flagged |
| [List initialization of classes](../spec/classes/initializer-lists.md) | 6 | 5 required, 1 reject | 30 | Q-0081 | 0 flagged |
| [Copy and move](../spec/classes/copy-move.md) | 19 | 13 required, 3 reject, 2 unspecified, 1 recommended | 87 | Q-0055 | 0 flagged |
| [Derived classes](../spec/classes/derived.md) | 18 | 9 required, 8 reject, 1 undefined | 73 | Q-0085 | 0 flagged |
| [Virtual functions and abstract classes](../spec/classes/virtual.md) | 20 | 7 required, 9 reject, 1 undefined, 2 no-diagnostic, 1 recommended | 107 | Q-0056, Q-0086, Q-0087 | 0 flagged |
| [Unions and bit-fields](../spec/classes/unions.md) | 20 | 11 required, 5 reject, 1 undefined, 2 impl-defined, 1 recommended | 87 | Q-0057, Q-0058, Q-0088 | 0 flagged |
| [Conversion functions](../spec/classes/conversion-functions.md) | 8 | 5 required, 2 reject, 1 recommended | 60 | Q-0059 | 0 flagged |
| [Operator overloading](../spec/classes/operators.md) | 12 | 8 required, 2 reject, 1 unspecified, 1 recommended | 89 | Q-0060 | 0 flagged |
| [Defaulted comparisons](../spec/classes/comparisons.md) | 8 | 5 required, 1 reject, 1 no-diagnostic, 1 recommended | 63 | Q-0061, Q-0062 | 0 flagged |
| [Throw expressions and the exception object](../spec/exceptions/throw.md) | 11 | 8 required, 2 reject, 1 recommended | 30 |  | 0 flagged |
| [Try blocks and handlers](../spec/exceptions/handlers.md) | 17 | 7 required, 6 reject, 1 undefined, 3 recommended | 74 |  | 0 flagged |
| [Unwinding and termination](../spec/exceptions/unwinding.md) | 8 | 6 required, 1 impl-defined, 1 recommended | 29 |  | 0 flagged |
| [Glossary](../spec/glossary.md) | 16 terms | | | | 0 flagged |

`tools/build_site.py` builds the specification as a searchable HTML site
(decision 0016). CI builds it on every change and publishes it to
https://sempuki.github.io/tidal/ after each push to `main`. Pages use
the structure and writing rules in `docs/style-guide.md`. Every page
opens with its principles (decision 0024), and the Foundations chapter
holds the models that the other pages cite. The latest similarity report, covering every page, is
`reports/similarity/2026-10-31-abstract-machine.txt`. The table above comes from
`tools/status_table.py`.

## Test matrix

All compilers are baseline. 2894/2894 tests pass on each. Tests may span several translation units (decision 0014). CI runs the
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

One hundred and eleven questions are open for opinions, each with a provisional answer the
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

None. Each topic from the gap reviews (`reports/review/`) has a page. Exception handling is a later chapter (decision 0023). The
forward-progress rule belongs with threads, which are out of scope.

## Next steps

0. Draft the Exceptions chapter (decision 0025): throw expressions,
   handlers, unwinding. Then ask the reviewer who prompted the
   principles-first change to read the Foundations chapter and two topic
   pages, and decide whether rules also want short annotations.
2. Report the EDG behavior in Q-0002 and Q-0003 to the EDG project.
3. Add a `tidal-error` directive that matches the expected diagnostic, so
   `reject` tests check their cause.
4. Add sanitizer-based tests that run in the container images.
5. Add EDG's 32-bit configuration as a second EDG matrix entry.
6. Read EDG's sanitized source for the `if` section.
7. Plan the exception handling chapter (decision 0023).
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
