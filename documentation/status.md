# Status

Where the project stands and what comes next. This page describes the
current state only. The history of how the project got here is in
`git log`.

## Specification

| Section | Rules | Classes | Tests | Single-test rules | Overrides | Report mentions | Questions | Similarity |
|---|---|---|---|---|---|---|---|---|
| [Translation, execution, and behavior classes](../specification/foundations/translation.md) | 11 | 4 required, 3 reject, 1 undefined, 2 impl-defined, 1 recommended | 31 | 1 | 1 | 44 | Q-0024, Q-0029 | 0 flagged |
| [Fundamental types](../specification/foundations/types.md) | 13 | 7 required, 1 reject, 5 impl-defined | 51 | 3 | 2 | 7 | Q-0120 | 0 flagged |
| [Language-support types](../specification/foundations/language-support.md) | 10 | 7 required, 3 reject | 42 | 2 | 12 | 24 | Q-0299, Q-0301, Q-0321 | 0 flagged |
| [Keywords and reserved names](../specification/foundations/keywords.md) | 4 | 2 required, 1 reject, 1 no-diagnostic | 96 | 2 | 0 | 9 |  | 0 flagged |
| [Objects, storage, and lifetime](../specification/foundations/objects.md) | 11 | 8 required, 2 undefined, 1 unspecified | 40 | 4 | 2 | 54 | Q-0115 | 0 flagged |
| [Raw storage and object representation](../specification/foundations/storage.md) | 27 | 15 required, 1 reject, 7 undefined, 1 unspecified, 3 impl-defined | 62 | 8 | 3 | 39 | Q-0304, Q-0305, Q-0306, Q-0307, Q-0308 | 0 flagged |
| [Value categories and temporaries](../specification/foundations/values.md) | 16 | 9 required, 4 reject, 2 unspecified, 1 impl-defined | 56 | 8 | 1 | 50 | Q-0034 | 0 flagged |
| [Standard conversions](../specification/foundations/conversions.md) | 20 | 11 required, 6 reject, 1 undefined, 2 impl-defined | 63 | 6 | 2 | 45 | Q-0028, Q-0100 | 0 flagged |
| [Implicit conversion sequences](../specification/foundations/conversion-sequences.md) | 47 | 46 required, 1 reject | 114 | 12 | 23 | 79 | Q-0081, Q-0132, Q-0133, Q-0134, Q-0135, Q-0136, Q-0137, Q-0138, Q-0152, Q-0153, Q-0154, Q-0155, Q-0156, Q-0157, Q-0188, Q-0294 | 0 flagged |
| [Overload resolution](../specification/foundations/overload-resolution.md) | 44 | 35 required, 9 reject | 164 | 5 | 26 | 136 | Q-0138, Q-0139, Q-0140, Q-0141, Q-0142, Q-0143, Q-0144, Q-0145, Q-0146, Q-0147, Q-0148, Q-0159, Q-0161, Q-0163, Q-0187, Q-0189, Q-0211 | 0 flagged |
| [Names, scopes, and lookup](../specification/foundations/names.md) | 38 | 20 required, 18 reject | 190 | 4 | 2 | 76 | Q-0311 | 0 flagged |
| [Use and the one-definition rule](../specification/foundations/odr.md) | 19 | 9 required, 3 reject, 2 impl-defined, 4 no-diagnostic, 1 recommended | 67 | 2 | 10 | 35 | Q-0127, Q-0128, Q-0129, Q-0130, Q-0297, Q-0315 | 0 flagged |
| [Sequencing and evaluation order](../specification/foundations/evaluation-order.md) | 10 | 6 required, 1 undefined, 2 unspecified, 1 recommended | 33 | 3 | 0 | 90 | Q-0319 | 0 flagged |
| [Constant expressions](../specification/foundations/constant-evaluation.md) | 30 | 6 required, 21 reject, 1 unspecified, 1 impl-defined, 1 no-diagnostic | 103 | 9 | 14 | 192 | Q-0002, Q-0004, Q-0020, Q-0033, Q-0078, Q-0090, Q-0115, Q-0117, Q-0192, Q-0193 | 0 flagged |
| [Expression statements and blocks](../specification/statements/blocks.md) | 10 | 7 required, 2 reject, 1 recommended | 26 | 3 | 2 | 3 | Q-0096 | 0 flagged |
| [The if statement](../specification/statements/if.md) | 24 | 13 required, 11 reject | 67 | 8 | 0 | 50 |  | 0 flagged |
| [The switch statement](../specification/statements/switch.md) | 7 | 2 required, 4 reject, 1 recommended | 34 | 0 | 1 | 20 | Q-0041 | 0 flagged |
| [Iteration statements](../specification/statements/loops.md) | 8 | 4 required, 4 reject | 19 | 3 | 1 | 2 | Q-0044 | 0 flagged |
| [The range-based for statement](../specification/statements/range-for.md) | 7 | 4 required, 3 reject | 29 | 1 | 6 | 49 | Q-0020, Q-0026, Q-0098 | 0 flagged |
| [Jump statements and labels](../specification/statements/jumps.md) | 16 | 9 required, 6 reject, 1 unspecified | 54 | 2 | 3 | 45 | Q-0041 | 0 flagged |
| [Primary expressions](../specification/expressions/primary.md) | 29 | 11 required, 10 reject, 1 undefined, 1 unspecified, 2 impl-defined, 3 conditionally-supported, 1 recommended | 106 | 6 | 5 | 22 | Q-0033, Q-0063, Q-0064, Q-0194 | 0 flagged |
| [decltype, unevaluated operands, and discarded values](../specification/expressions/decltype.md) | 11 | 5 required, 4 reject, 2 recommended | 51 | 3 | 3 | 25 | Q-0065, Q-0089 | 0 flagged |
| [Multiplicative operators](../specification/expressions/multiplicative.md) | 20 | 7 required, 3 reject, 4 undefined, 2 unspecified, 3 impl-defined, 1 recommended | 52 | 0 | 5 | 47 | Q-0001, Q-0002, Q-0003, Q-0004, Q-0005 | 0 flagged |
| [Additive operators](../specification/expressions/additive.md) | 17 | 7 required, 1 reject, 6 undefined, 1 unspecified, 1 impl-defined, 1 recommended | 56 | 0 | 8 | 25 | Q-0001, Q-0005, Q-0031, Q-0049, Q-0298, Q-0304 | 0 flagged |
| [Shift operators](../specification/expressions/shift.md) | 9 | 6 required, 1 reject, 1 undefined, 1 recommended | 25 | 3 | 0 | 11 |  | 0 flagged |
| [Comparison operators](../specification/expressions/comparison.md) | 27 | 15 required, 6 reject, 2 unspecified, 4 recommended | 85 | 1 | 10 | 30 | Q-0029, Q-0030, Q-0050, Q-0051, Q-0103, Q-0104 | 0 flagged |
| [Bitwise operators](../specification/expressions/bitwise.md) | 8 | 4 required, 1 reject, 3 recommended | 21 | 2 | 0 | 2 |  | 0 flagged |
| [Logical operators](../specification/expressions/logical.md) | 14 | 10 required, 2 reject, 2 recommended | 34 | 0 | 1 | 11 | Q-0101 | 0 flagged |
| [The conditional operator](../specification/expressions/conditional.md) | 15 | 8 required, 7 reject | 37 | 9 | 1 | 7 | Q-0102 | 0 flagged |
| [Assignment operators](../specification/expressions/assignment.md) | 13 | 7 required, 5 reject, 1 unspecified | 32 | 4 | 1 | 21 | Q-0032 | 0 flagged |
| [The comma operator](../specification/expressions/comma.md) | 9 | 5 required, 3 reject, 1 recommended | 24 | 1 | 0 | 7 |  | 0 flagged |
| [Unary operators](../specification/expressions/unary.md) | 21 | 10 required, 6 reject, 3 undefined, 1 impl-defined, 1 recommended | 68 | 3 | 4 | 45 | Q-0031, Q-0124 | 0 flagged |
| [Pointers to members](../specification/expressions/member-pointers.md) | 12 | 5 required, 5 reject, 1 undefined, 1 unspecified | 70 | 1 | 0 | 11 |  | 0 flagged |
| [Postfix expressions](../specification/expressions/postfix.md) | 21 | 7 required, 8 reject, 5 undefined, 1 conditionally-supported | 68 | 6 | 7 | 61 | Q-0031, Q-0034, Q-0124, Q-0178, Q-0316, Q-0318 | 0 flagged |
| [Casts](../specification/expressions/casts.md) | 19 | 8 required, 7 reject, 2 undefined, 2 impl-defined | 51 | 5 | 1 | 23 | Q-0017 | 0 flagged |
| [New and delete](../specification/expressions/new-delete.md) | 39 | 21 required, 11 reject, 3 undefined, 3 unspecified, 1 impl-defined | 116 | 11 | 18 | 60 | Q-0005, Q-0035, Q-0036, Q-0091, Q-0125, Q-0220, Q-0221, Q-0222, Q-0300, Q-0301, Q-0302, Q-0303, Q-0313 | 0 flagged |
| [Lambda expressions](../specification/expressions/lambdas.md) | 28 | 16 required, 9 reject, 1 undefined, 1 unspecified, 1 recommended | 62 | 14 | 2 | 53 | Q-0037, Q-0092 | 0 flagged |
| [Placeholder types in variable declarations](../specification/declarations/auto.md) | 22 | 12 required, 10 reject | 55 | 8 | 0 | 25 | Q-0010 | 0 flagged |
| [Type aliases and class names](../specification/declarations/type-names.md) | 10 | 4 required, 5 reject, 1 no-diagnostic | 61 | 2 | 4 | 3 | Q-0066, Q-0067, Q-0093 | 0 flagged |
| [Storage duration and linkage](../specification/declarations/storage-linkage.md) | 23 | 14 required, 6 reject, 1 undefined, 2 no-diagnostic | 54 | 11 | 2 | 27 | Q-0011, Q-0045, Q-0105, Q-0297 | 0 flagged |
| [The inline specifier](../specification/declarations/inline.md) | 8 | 4 required, 2 reject, 2 no-diagnostic | 34 | 2 | 3 | 0 | Q-0068 | 0 flagged |
| [Declarators and cv-qualifiers](../specification/declarations/declarators.md) | 34 | 17 required, 14 reject, 1 undefined, 1 impl-defined, 1 recommended | 89 | 7 | 3 | 36 | Q-0012, Q-0106, Q-0191 | 0 flagged |
| [Alignment](../specification/declarations/alignment.md) | 8 | 2 required, 4 reject, 1 impl-defined, 1 no-diagnostic | 35 | 2 | 15 | 2 | Q-0069, Q-0070, Q-0071, Q-0072, Q-0073, Q-0313, Q-0314 | 0 flagged |
| [Volatile](../specification/declarations/volatile.md) | 12 | 5 required, 5 reject, 1 undefined, 1 recommended | 35 | 2 | 2 | 8 | Q-0080, Q-0176, Q-0320 | 0 flagged |
| [Forms of initialization](../specification/declarations/initialization/forms.md) | 9 | 6 required, 2 reject, 1 undefined | 31 | 3 | 1 | 32 | Q-0013, Q-0039 | 0 flagged |
| [List initialization and aggregates](../specification/declarations/initialization/list.md) | 16 | 8 required, 8 reject | 60 | 4 | 3 | 20 | Q-0042, Q-0043, Q-0166, Q-0167 | 0 flagged |
| [Reference binding](../specification/declarations/initialization/references.md) | 14 | 8 required, 4 reject, 1 undefined, 1 recommended | 62 | 2 | 14 | 68 | Q-0040, Q-0109, Q-0110, Q-0149, Q-0150, Q-0151, Q-0157, Q-0164, Q-0165, Q-0166, Q-0180, Q-0294 | 0 flagged |
| [Initialization of static and thread storage](../specification/declarations/initialization/static.md) | 7 | 3 required, 2 unspecified, 2 impl-defined | 18 | 2 | 0 | 16 | Q-0014, Q-0175 | 0 flagged |
| [Structured bindings](../specification/declarations/structured-bindings.md) | 17 | 10 required, 7 reject | 55 | 5 | 4 | 45 | Q-0007, Q-0015, Q-0016, Q-0048, Q-0107 | 0 flagged |
| [Enumerations](../specification/declarations/enumerations.md) | 21 | 10 required, 8 reject, 1 undefined, 1 unspecified, 1 impl-defined | 59 | 5 | 3 | 22 | Q-0017, Q-0018, Q-0047, Q-0114 | 0 flagged |
| [Namespaces and using](../specification/declarations/namespaces.md) | 17 | 8 required, 9 reject | 41 | 5 | 1 | 3 | Q-0019 | 0 flagged |
| [Linkage specifications](../specification/declarations/linkage-specifications.md) | 7 | 3 required, 2 reject, 2 impl-defined | 29 | 1 | 4 | 4 | Q-0074, Q-0075, Q-0076 | 0 flagged |
| [Standard attributes](../specification/declarations/attributes.md) | 19 | 7 required, 4 reject, 2 undefined, 1 impl-defined, 5 recommended | 72 | 6 | 13 | 39 | Q-0020, Q-0021, Q-0046, Q-0108, Q-0123, Q-0177 | 0 flagged |
| [Function definitions](../specification/declarations/functions/definitions.md) | 15 | 4 required, 6 reject, 1 undefined, 3 impl-defined, 1 recommended | 43 | 4 | 6 | 20 | Q-0022, Q-0024, Q-0076 | 0 flagged |
| [Default arguments](../specification/declarations/functions/default-arguments.md) | 9 | 3 required, 5 reject, 1 no-diagnostic | 21 | 3 | 2 | 17 | Q-0038, Q-0111 | 0 flagged |
| [Deleted and defaulted functions](../specification/declarations/functions/deleted.md) | 5 | 1 required, 4 reject | 17 | 2 | 1 | 9 | Q-0007, Q-0023 | 0 flagged |
| [Return type deduction](../specification/declarations/functions/return-deduction.md) | 7 | 2 required, 4 reject, 1 undefined | 23 | 1 | 0 | 26 |  | 0 flagged |
| [Exception specifications](../specification/declarations/exception-specifications.md) | 10 | 4 required, 4 reject, 1 impl-defined, 1 no-diagnostic | 40 | 2 | 6 | 48 | Q-0077, Q-0078, Q-0079, Q-0094, Q-0095 | 0 flagged |
| [The constexpr, consteval, and constinit specifiers](../specification/declarations/constexpr.md) | 12 | 3 required, 9 reject | 68 | 2 | 5 | 70 | Q-0020, Q-0025 | 0 flagged |
| [Static assertions](../specification/declarations/static-assert.md) | 8 | 4 required, 4 reject | 29 | 0 | 0 | 10 |  | 0 flagged |
| [Class definitions and members](../specification/classes/members.md) | 27 | 14 required, 11 reject, 1 impl-defined, 1 no-diagnostic | 90 | 10 | 3 | 44 | Q-0027, Q-0181, Q-0317 | 0 flagged |
| [Access control and friends](../specification/classes/access.md) | 19 | 8 required, 11 reject | 63 | 6 | 2 | 24 | Q-0052, Q-0131 | 0 flagged |
| [Constructors and destructors](../specification/classes/constructors.md) | 34 | 13 required, 16 reject, 2 undefined, 2 no-diagnostic, 1 recommended | 166 | 7 | 10 | 106 | Q-0041, Q-0053, Q-0054, Q-0082, Q-0083, Q-0084, Q-0126 | 0 flagged |
| [List initialization of classes](../specification/classes/initializer-lists.md) | 10 | 5 required, 5 reject | 31 | 1 | 2 | 17 | Q-0081, Q-0132 | 0 flagged |
| [Copy and move](../specification/classes/copy-move.md) | 27 | 14 required, 10 reject, 2 unspecified, 1 recommended | 103 | 3 | 2 | 53 | Q-0055 | 0 flagged |
| [Derived classes](../specification/classes/derived.md) | 22 | 9 required, 12 reject, 1 undefined | 77 | 4 | 1 | 34 | Q-0085 | 0 flagged |
| [Virtual functions and abstract classes](../specification/classes/virtual.md) | 21 | 7 required, 10 reject, 1 undefined, 2 no-diagnostic, 1 recommended | 117 | 4 | 5 | 43 | Q-0056, Q-0086, Q-0087, Q-0212 | 0 flagged |
| [Unions and bit-fields](../specification/classes/unions.md) | 25 | 13 required, 8 reject, 1 undefined, 2 impl-defined, 1 recommended | 117 | 5 | 7 | 61 | Q-0045, Q-0058, Q-0088, Q-0213, Q-0309, Q-0313, Q-0314 | 0 flagged |
| [Conversion functions](../specification/classes/conversion-functions.md) | 12 | 5 required, 6 reject, 1 recommended | 64 | 1 | 1 | 22 | Q-0059 | 0 flagged |
| [Operator overloading](../specification/classes/operators.md) | 16 | 8 required, 6 reject, 1 unspecified, 1 recommended | 98 | 3 | 1 | 43 | Q-0060 | 0 flagged |
| [Defaulted comparisons](../specification/classes/comparisons.md) | 13 | 6 required, 5 reject, 1 no-diagnostic, 1 recommended | 74 | 2 | 5 | 24 | Q-0061, Q-0062, Q-0158, Q-0190 | 0 flagged |
| [Template declarations](../specification/templates/declarations.md) | 49 | 26 required, 21 reject, 2 no-diagnostic | 121 | 1 | 8 | 71 | Q-0195, Q-0196, Q-0197, Q-0198, Q-0199, Q-0200, Q-0296 | 0 flagged |
| [Template arguments](../specification/templates/arguments.md) | 27 | 16 required, 10 reject, 1 unspecified | 88 | 0 | 6 | 57 | Q-0117, Q-0168, Q-0169, Q-0170, Q-0201 | 0 flagged |
| [Explicit and partial specialization](../specification/templates/specialization.md) | 32 | 16 required, 15 reject, 1 no-diagnostic | 93 | 0 | 13 | 15 | Q-0171, Q-0172, Q-0173, Q-0174, Q-0203, Q-0204, Q-0205, Q-0206, Q-0207, Q-0295 | 0 flagged |
| [Template argument deduction](../specification/templates/deduction.md) | 39 | 36 required, 3 reject | 119 | 0 | 8 | 77 | Q-0208, Q-0209, Q-0215, Q-0219, Q-0220, Q-0295 | 0 flagged |
| [Partial ordering of templates](../specification/templates/ordering.md) | 16 | 15 required, 1 reject | 41 | 0 | 11 | 28 | Q-0146, Q-0211, Q-0214, Q-0216, Q-0217, Q-0218, Q-0293 | 0 flagged |
| [Dependent names](../specification/templates/dependent-names.md) | 39 | 20 required, 13 reject, 6 no-diagnostic | 117 | 0 | 20 | 72 | Q-0223, Q-0224, Q-0225, Q-0226, Q-0227, Q-0228, Q-0229, Q-0230, Q-0245, Q-0246, Q-0247, Q-0248, Q-0249, Q-0250 | 0 flagged |
| [Instantiation](../specification/templates/instantiation.md) | 36 | 16 required, 13 reject, 1 undefined, 2 unspecified, 1 impl-defined, 3 no-diagnostic | 130 | 1 | 18 | 60 | Q-0231, Q-0232, Q-0233, Q-0234, Q-0235, Q-0236, Q-0237, Q-0251, Q-0252, Q-0253, Q-0254, Q-0255, Q-0256, Q-0257, Q-0310 | 0 flagged |
| [Concepts and requires expressions](../specification/templates/concepts.md) | 37 | 21 required, 16 reject | 118 | 0 | 17 | 52 | Q-0258, Q-0259, Q-0260, Q-0270, Q-0285, Q-0286, Q-0287, Q-0288, Q-0289, Q-0290 | 0 flagged |
| [Constraints](../specification/templates/constraints.md) | 39 | 24 required, 13 reject, 2 no-diagnostic | 122 | 0 | 26 | 46 | Q-0260, Q-0261, Q-0262, Q-0263, Q-0264, Q-0265, Q-0266, Q-0267, Q-0268, Q-0269, Q-0281, Q-0282, Q-0283, Q-0284, Q-0291, Q-0292 | 0 flagged |
| [Packs](../specification/templates/packs.md) | 31 | 19 required, 11 reject, 1 no-diagnostic | 69 | 0 | 4 | 41 | Q-0238, Q-0271, Q-0272, Q-0273, Q-0274 | 0 flagged |
| [Class template argument deduction](../specification/templates/class-template-deduction.md) | 29 | 21 required, 8 reject | 113 | 0 | 19 | 32 | Q-0239, Q-0240, Q-0241, Q-0242, Q-0243, Q-0244, Q-0275, Q-0276, Q-0277, Q-0278, Q-0279, Q-0280 | 0 flagged |
| [Throw expressions and the exception object](../specification/exceptions/throw.md) | 12 | 10 required, 1 reject | 28 | 5 | 0 | 37 |  | 0 flagged |
| [Try blocks and handlers](../specification/exceptions/handlers.md) | 18 | 8 required, 6 reject, 1 undefined, 3 recommended | 80 | 6 | 2 | 47 |  | 0 flagged |
| [Unwinding and termination](../specification/exceptions/unwinding.md) | 9 | 7 required, 1 impl-defined, 1 recommended | 34 | 2 | 1 | 46 | Q-0179 | 0 flagged |
| [Glossary](../specification/glossary.md) | 117 terms | | | | | | | 0 flagged |

`tools/build_site.py` builds the specification as a searchable HTML site
(decision 0016). CI builds the site on every change and publishes it to
https://sempuki.github.io/tidal/ after each push to `main`. Pages use
the structure and writing rules in `documentation/style-guide.md`. Every page
opens with its principles (decision 0024), and the Foundations chapter
holds the models that the other pages cite. The latest similarity
report, covering every page, is
`reports/similarity/2026-10-31-types.txt`. The table above comes from
`tools/status_table.py`.

## Test matrix

Eight compilers are baseline, and 5408/5408 tests pass on each. Two
report compilers build at `-O2` (decision 0043). Tests may
span several translation units (decision 0014). CI runs the
containerized compilers on every push and pull request (decision 0012).

| Compiler | Where | Notes |
|---|---|---|
| Clang 22.1.8 | host, and container (apt.llvm.org) | |
| GCC 16.2.1 | host | |
| GCC 16.2.0 | container (official image) | skips `-m32` tests |
| Clang 18.1.3 | container | |
| GCC 14.2.0 | container | |
| EDG 7.0 | container | front end only; skips `-m32`, warning-flag, and sanitizer tests; per-compiler expectations on 61 tests, each with a note that names its question or the implementation choice |
| EDG 7.0, ILP32 | container, built with EDG's i686 configuration | front end only; runs only the `-m32` tests; differs on the alignment of `long long` (Q-0120) |
| GCC 16.2.0, `-O2` | container, report | runs the tests at `-O2`; does not fail a run |
| Clang 22.1.8, `-O2` | container, report | runs the tests at `-O2`; does not fail a run |

Full results are in `tests/RESULTS.md`.

## Open questions

Two hundred and eighty-nine questions are open for opinions, each
with a provisional answer the specification follows. Twenty more have the status
"compiler defect". Their answer is that one compiler or an older
version is wrong (Q-0002, Q-0003, Q-0013, Q-0015, Q-0016, Q-0023,
Q-0026, Q-0044, Q-0072, Q-0101, Q-0115, Q-0125, Q-0177, Q-0181, Q-0206,
Q-0212, Q-0213, Q-0238, Q-0308, Q-0309). Twelve questions are settled,
Q-0009 (license) among them. See `questions/README.md`.

## Known limits of the specification text

- The glossary covers the terms used so far. Terms marked *(referenced)*
  belong to topics that Tidal does not specify yet.
- 21 *(referenced)* markers remain. Each names a topic with no page:
  the library (11), extended floating-point types (4), class properties
  (2), the preprocessor (2), coroutines (1), or the memory model (1).

## Known limits of the tooling

- The clean-room front end passes 4439 of 5408 tests. The front end
  has no preprocessor, so it rejects the valid tests that use
  `#include`.
  `tools/frontend_check.py` fails when a test that it passed no longer
  passes (`reports/clean-room/rust-frontend/conformance.tsv`).

- 1451 tests carry `tidal-error` patterns. A pattern checks that a
  compiler rejects the test, fails to link the test, or stops the test
  at run time, for the reason the test's rule gives.
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
  for that section.
- CI caches test results between runs (`actions/cache`). A full rebuild
  of the cache happens when the cache key format changes
  (`OUTCOME_VERSION` in `tools/run_tests.py`) or a compiler image changes.

## Topics in scope with no page yet

None. Each topic from the gap reviews (`reports/review/`) has a page.
Exception handling is a later chapter (decision 0023). The
forward-progress rule belongs with threads, which are out of scope.

## Next steps

`documentation/roadmap.md` orders the work (decisions 0033, 0039, and
0050). Phases 1 to 5 are done. Next:

1. Roadmap round 3. Clean-room views (programmer, implementer, editor,
   clean-room writer, language designer) rank what comes after phase 5,
   in `reports/clean-room/roadmap-3/`. A provisional decision then sets
   phase 6.
2. Follow-ups that no phase item holds:
   - a Clang `-fsanitize=type` configuration for STG-18 and STG-23
     (Clang 22 reports these accesses but does not stop the run),
   - the copy-move triviality split, where Clang copies a union member
     whose class has two copy constructors,
   - a namespace-scope rule that a class and a variable can share a
     name (`names.md`).

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

These counts total about 66 section-units. Drafting one unit takes one working
session for the clean-room writer. The estimate excludes review and
adjudication, which are expected to dominate once reviewers are
involved.
