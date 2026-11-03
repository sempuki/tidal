# Status

Where the project stands and what comes next. This page describes the
current state only. The history of how the project got here is in
`git log`.

## Specification

| Section | Rules | Classes | Tests | Single-test rules | Overrides | Report mentions | Questions | Similarity |
|---|---|---|---|---|---|---|---|---|
| [Translation, execution, and behavior classes](../specification/foundations/translation.md) | 12 | 4 required, 4 reject, 1 undefined, 2 impl-defined, 1 recommended | 37 | 1 | 1 | 61 | Q-0024, Q-0029, Q-0312 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Fundamental types](../specification/foundations/types.md) | 14 | 7 required, 1 reject, 6 impl-defined | 56 | 3 | 2 | 29 | Q-0120 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Language-support types](../specification/foundations/language-support.md) | 11 | 8 required, 3 reject | 46 | 2 | 12 | 53 | Q-0299, Q-0301, Q-0321, Q-0323 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Keywords and reserved names](../specification/foundations/keywords.md) | 4 | 2 required, 1 reject, 1 no-diagnostic | 96 | 2 | 0 | 4 |  | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Objects, storage, and lifetime](../specification/foundations/objects.md) | 11 | 8 required, 2 undefined, 1 unspecified | 43 | 4 | 2 | 46 | Q-0115 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Raw storage and object representation](../specification/foundations/storage.md) | 29 | 17 required, 1 reject, 7 undefined, 1 unspecified, 3 impl-defined | 70 | 5 | 3 | 104 | Q-0304, Q-0305, Q-0306, Q-0307, Q-0308, Q-0323, Q-0324, Q-0325, Q-0327, Q-0328 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Value categories and temporaries](../specification/foundations/values.md) | 16 | 9 required, 4 reject, 2 unspecified, 1 impl-defined | 56 | 8 | 1 | 44 | Q-0034 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Standard conversions](../specification/foundations/conversions.md) | 20 | 11 required, 6 reject, 1 undefined, 2 impl-defined | 64 | 5 | 2 | 56 | Q-0028, Q-0100 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Implicit conversion sequences](../specification/foundations/conversion-sequences.md) | 47 | 46 required, 1 reject | 114 | 12 | 23 | 84 | Q-0081, Q-0132, Q-0133, Q-0134, Q-0135, Q-0136, Q-0137, Q-0138, Q-0152, Q-0153, Q-0154, Q-0155, Q-0156, Q-0157, Q-0188, Q-0294 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Overload resolution](../specification/foundations/overload-resolution.md) | 44 | 35 required, 9 reject | 164 | 5 | 26 | 137 | Q-0138, Q-0139, Q-0140, Q-0141, Q-0142, Q-0143, Q-0144, Q-0145, Q-0146, Q-0147, Q-0148, Q-0159, Q-0161, Q-0163, Q-0187, Q-0189, Q-0211 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Names, scopes, and lookup](../specification/foundations/names.md) | 38 | 20 required, 18 reject | 191 | 4 | 2 | 84 | Q-0311 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Use, headers, and the one-definition rule](../specification/foundations/odr.md) | 19 | 9 required, 3 reject, 2 impl-defined, 4 no-diagnostic, 1 recommended | 68 | 2 | 10 | 42 | Q-0127, Q-0128, Q-0129, Q-0130, Q-0297, Q-0315 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Sequencing and evaluation order](../specification/foundations/evaluation-order.md) | 11 | 7 required, 1 undefined, 2 unspecified, 1 recommended | 35 | 4 | 0 | 93 | Q-0319 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Constant expressions](../specification/foundations/constant-evaluation.md) | 32 | 6 required, 23 reject, 1 unspecified, 1 impl-defined, 1 no-diagnostic | 111 | 9 | 14 | 192 | Q-0002, Q-0004, Q-0020, Q-0033, Q-0039, Q-0078, Q-0090, Q-0115, Q-0117, Q-0192, Q-0193 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)); 1 IDs not checked |
| [Expression statements and blocks](../specification/statements/blocks.md) | 10 | 7 required, 2 reject, 1 recommended | 26 | 3 | 2 | 10 | Q-0096 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [The if statement](../specification/statements/if.md) | 25 | 14 required, 11 reject | 69 | 8 | 1 | 47 | Q-0329 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [The switch statement](../specification/statements/switch.md) | 7 | 2 required, 4 reject, 1 recommended | 36 | 0 | 1 | 33 | Q-0041 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Iteration statements](../specification/statements/loops.md) | 8 | 4 required, 4 reject | 19 | 3 | 1 | 5 | Q-0044 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [The range-based for statement](../specification/statements/range-for.md) | 7 | 4 required, 3 reject | 29 | 1 | 6 | 54 | Q-0020, Q-0026, Q-0098 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Jump statements and labels](../specification/statements/jumps.md) | 17 | 9 required, 7 reject, 1 unspecified | 57 | 1 | 3 | 56 | Q-0041, Q-0121 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Primary expressions](../specification/expressions/primary.md) | 33 | 14 required, 10 reject, 1 undefined, 1 unspecified, 3 impl-defined, 3 conditionally-supported, 1 recommended | 114 | 9 | 8 | 41 | Q-0033, Q-0063, Q-0064, Q-0194, Q-0330 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [decltype, unevaluated operands, and discarded values](../specification/expressions/decltype.md) | 11 | 5 required, 4 reject, 2 recommended | 51 | 3 | 3 | 28 | Q-0065, Q-0089 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Multiplicative operators](../specification/expressions/multiplicative.md) | 20 | 7 required, 3 reject, 4 undefined, 2 unspecified, 3 impl-defined, 1 recommended | 52 | 0 | 5 | 56 | Q-0001, Q-0002, Q-0003, Q-0004, Q-0005 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Additive operators](../specification/expressions/additive.md) | 17 | 7 required, 1 reject, 6 undefined, 1 unspecified, 1 impl-defined, 1 recommended | 57 | 0 | 9 | 41 | Q-0001, Q-0005, Q-0031, Q-0049, Q-0298, Q-0304 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Shift operators](../specification/expressions/shift.md) | 9 | 6 required, 1 reject, 1 undefined, 1 recommended | 25 | 3 | 0 | 21 |  | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Comparison operators](../specification/expressions/comparison.md) | 27 | 15 required, 6 reject, 2 unspecified, 4 recommended | 89 | 1 | 11 | 48 | Q-0029, Q-0030, Q-0050, Q-0051, Q-0103, Q-0104 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Bitwise operators](../specification/expressions/bitwise.md) | 8 | 4 required, 1 reject, 3 recommended | 21 | 2 | 0 | 8 |  | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Logical operators](../specification/expressions/logical.md) | 14 | 10 required, 2 reject, 2 recommended | 34 | 0 | 1 | 11 | Q-0101 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [The conditional operator](../specification/expressions/conditional.md) | 15 | 8 required, 7 reject | 39 | 9 | 1 | 9 | Q-0102 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Assignment operators](../specification/expressions/assignment.md) | 13 | 7 required, 5 reject, 1 unspecified | 32 | 4 | 1 | 22 | Q-0032 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [The comma operator](../specification/expressions/comma.md) | 9 | 5 required, 3 reject, 1 recommended | 24 | 1 | 0 | 7 |  | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Unary operators](../specification/expressions/unary.md) | 21 | 10 required, 6 reject, 3 undefined, 1 impl-defined, 1 recommended | 69 | 3 | 4 | 67 | Q-0031, Q-0124 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Pointers to members](../specification/expressions/member-pointers.md) | 12 | 5 required, 5 reject, 1 undefined, 1 unspecified | 71 | 1 | 0 | 20 |  | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Postfix expressions](../specification/expressions/postfix.md) | 21 | 7 required, 8 reject, 5 undefined, 1 conditionally-supported | 69 | 5 | 7 | 81 | Q-0031, Q-0034, Q-0124, Q-0178, Q-0316, Q-0318 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Casts](../specification/expressions/casts.md) | 19 | 8 required, 7 reject, 2 undefined, 2 impl-defined | 56 | 4 | 1 | 39 | Q-0017, Q-0325 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [New and delete](../specification/expressions/new-delete.md) | 42 | 22 required, 11 reject, 4 undefined, 3 unspecified, 2 impl-defined | 128 | 11 | 20 | 106 | Q-0005, Q-0035, Q-0036, Q-0091, Q-0125, Q-0220, Q-0221, Q-0222, Q-0300, Q-0301, Q-0302, Q-0303, Q-0313 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Lambda expressions](../specification/expressions/lambdas.md) | 28 | 16 required, 9 reject, 1 undefined, 1 unspecified, 1 recommended | 63 | 13 | 2 | 53 | Q-0037, Q-0092 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Placeholder types in variable declarations](../specification/declarations/auto.md) | 22 | 12 required, 10 reject | 55 | 8 | 0 | 28 | Q-0010 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Type aliases and class names](../specification/declarations/type-names.md) | 10 | 4 required, 5 reject, 1 no-diagnostic | 64 | 2 | 4 | 7 | Q-0066, Q-0067, Q-0093 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Storage duration and linkage](../specification/declarations/storage-linkage.md) | 23 | 14 required, 6 reject, 1 undefined, 2 no-diagnostic | 57 | 11 | 2 | 42 | Q-0011, Q-0045, Q-0105, Q-0297 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [The inline specifier](../specification/declarations/inline.md) | 8 | 4 required, 2 reject, 2 no-diagnostic | 35 | 2 | 3 | 6 | Q-0068 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Declarators and cv-qualifiers](../specification/declarations/declarators.md) | 34 | 17 required, 14 reject, 1 undefined, 1 impl-defined, 1 recommended | 94 | 6 | 3 | 56 | Q-0012, Q-0106, Q-0191 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Alignment](../specification/declarations/alignment.md) | 8 | 2 required, 4 reject, 1 impl-defined, 1 no-diagnostic | 35 | 2 | 15 | 12 | Q-0069, Q-0070, Q-0071, Q-0072, Q-0073, Q-0313, Q-0314 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Volatile](../specification/declarations/volatile.md) | 13 | 5 required, 5 reject, 1 undefined, 1 impl-defined, 1 recommended | 37 | 3 | 2 | 19 | Q-0080, Q-0176, Q-0320, Q-0326 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Forms of initialization](../specification/declarations/initialization/forms.md) | 9 | 6 required, 2 reject, 1 undefined | 34 | 3 | 1 | 39 | Q-0013, Q-0039 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [List initialization and aggregates](../specification/declarations/initialization/list.md) | 16 | 8 required, 8 reject | 62 | 4 | 4 | 20 | Q-0042, Q-0043, Q-0166, Q-0167, Q-0331 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Reference binding](../specification/declarations/initialization/references.md) | 14 | 8 required, 4 reject, 1 undefined, 1 recommended | 64 | 1 | 15 | 66 | Q-0040, Q-0109, Q-0110, Q-0149, Q-0150, Q-0151, Q-0157, Q-0164, Q-0165, Q-0166, Q-0180, Q-0294 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Initialization of static and thread storage](../specification/declarations/initialization/static.md) | 7 | 3 required, 2 unspecified, 2 impl-defined | 20 | 2 | 0 | 31 | Q-0014, Q-0175 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Structured bindings](../specification/declarations/structured-bindings.md) | 17 | 10 required, 7 reject | 55 | 5 | 4 | 43 | Q-0007, Q-0015, Q-0016, Q-0048, Q-0107 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Enumerations](../specification/declarations/enumerations.md) | 21 | 10 required, 8 reject, 1 undefined, 1 unspecified, 1 impl-defined | 59 | 5 | 3 | 24 | Q-0017, Q-0018, Q-0047, Q-0114 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Namespaces and using](../specification/declarations/namespaces.md) | 17 | 8 required, 9 reject | 43 | 4 | 1 | 13 | Q-0019 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Linkage specifications](../specification/declarations/linkage-specifications.md) | 7 | 3 required, 2 reject, 2 impl-defined | 32 | 1 | 6 | 17 | Q-0074, Q-0075, Q-0076 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Standard attributes](../specification/declarations/attributes.md) | 19 | 7 required, 4 reject, 2 undefined, 1 impl-defined, 5 recommended | 72 | 6 | 13 | 46 | Q-0020, Q-0021, Q-0046, Q-0108, Q-0123, Q-0177 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Function definitions](../specification/declarations/functions/definitions.md) | 16 | 5 required, 6 reject, 1 undefined, 3 impl-defined, 1 recommended | 45 | 4 | 6 | 29 | Q-0022, Q-0024, Q-0076 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Default arguments](../specification/declarations/functions/default-arguments.md) | 9 | 3 required, 5 reject, 1 no-diagnostic | 22 | 2 | 2 | 19 | Q-0038, Q-0111 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Deleted and defaulted functions](../specification/declarations/functions/deleted.md) | 5 | 1 required, 4 reject | 18 | 2 | 1 | 9 | Q-0007, Q-0023 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Return type deduction](../specification/declarations/functions/return-deduction.md) | 7 | 2 required, 4 reject, 1 undefined | 23 | 1 | 0 | 29 |  | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Exception specifications](../specification/declarations/exception-specifications.md) | 10 | 4 required, 4 reject, 1 impl-defined, 1 no-diagnostic | 41 | 2 | 6 | 55 | Q-0077, Q-0078, Q-0079, Q-0094, Q-0095 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [The constexpr, consteval, and constinit specifiers](../specification/declarations/constexpr.md) | 12 | 3 required, 9 reject | 69 | 2 | 5 | 84 | Q-0020, Q-0025, Q-0193 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Static assertions](../specification/declarations/static-assert.md) | 8 | 4 required, 4 reject | 29 | 0 | 0 | 8 |  | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Class definitions and members](../specification/classes/members.md) | 27 | 14 required, 11 reject, 1 impl-defined, 1 no-diagnostic | 91 | 10 | 3 | 41 | Q-0027, Q-0181, Q-0317 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Access control and friends](../specification/classes/access.md) | 19 | 8 required, 11 reject | 64 | 6 | 2 | 28 | Q-0052, Q-0131 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Constructors and destructors](../specification/classes/constructors.md) | 34 | 13 required, 16 reject, 2 undefined, 2 no-diagnostic, 1 recommended | 168 | 7 | 10 | 122 | Q-0041, Q-0053, Q-0054, Q-0082, Q-0083, Q-0084, Q-0126 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [List initialization of classes](../specification/classes/initializer-lists.md) | 10 | 5 required, 5 reject | 31 | 1 | 2 | 19 | Q-0081, Q-0132 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Copy and move](../specification/classes/copy-move.md) | 27 | 14 required, 10 reject, 2 unspecified, 1 recommended | 103 | 3 | 2 | 72 | Q-0055 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Derived classes](../specification/classes/derived.md) | 22 | 9 required, 12 reject, 1 undefined | 78 | 4 | 1 | 38 | Q-0085 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Virtual functions and abstract classes](../specification/classes/virtual.md) | 21 | 7 required, 10 reject, 1 undefined, 2 no-diagnostic, 1 recommended | 119 | 4 | 5 | 56 | Q-0056, Q-0086, Q-0087, Q-0212 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Unions and bit-fields](../specification/classes/unions.md) | 26 | 14 required, 8 reject, 1 undefined, 2 impl-defined, 1 recommended | 124 | 5 | 8 | 85 | Q-0045, Q-0058, Q-0088, Q-0120, Q-0213, Q-0309, Q-0313, Q-0314 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Conversion functions](../specification/classes/conversion-functions.md) | 12 | 5 required, 6 reject, 1 recommended | 64 | 1 | 1 | 27 | Q-0059 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Operator overloading](../specification/classes/operators.md) | 16 | 8 required, 6 reject, 1 unspecified, 1 recommended | 99 | 3 | 1 | 39 | Q-0060 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Defaulted comparisons](../specification/classes/comparisons.md) | 13 | 6 required, 5 reject, 1 no-diagnostic, 1 recommended | 77 | 1 | 5 | 33 | Q-0061, Q-0062, Q-0158, Q-0190 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Template declarations](../specification/templates/declarations.md) | 49 | 26 required, 21 reject, 2 no-diagnostic | 121 | 1 | 8 | 70 | Q-0195, Q-0196, Q-0197, Q-0198, Q-0199, Q-0200, Q-0296 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Template arguments](../specification/templates/arguments.md) | 27 | 16 required, 10 reject, 1 unspecified | 88 | 0 | 6 | 54 | Q-0117, Q-0168, Q-0169, Q-0170, Q-0201 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Explicit and partial specialization](../specification/templates/specialization.md) | 32 | 16 required, 15 reject, 1 no-diagnostic | 93 | 0 | 13 | 18 | Q-0171, Q-0172, Q-0173, Q-0174, Q-0203, Q-0204, Q-0205, Q-0206, Q-0207, Q-0295 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Template argument deduction](../specification/templates/deduction.md) | 39 | 36 required, 3 reject | 119 | 0 | 8 | 74 | Q-0208, Q-0209, Q-0215, Q-0219, Q-0220, Q-0295 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Partial ordering of templates](../specification/templates/ordering.md) | 16 | 15 required, 1 reject | 41 | 0 | 11 | 25 | Q-0146, Q-0211, Q-0214, Q-0216, Q-0217, Q-0218, Q-0293 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Dependent names](../specification/templates/dependent-names.md) | 39 | 20 required, 13 reject, 6 no-diagnostic | 117 | 0 | 20 | 76 | Q-0223, Q-0224, Q-0225, Q-0226, Q-0227, Q-0228, Q-0229, Q-0230, Q-0245, Q-0246, Q-0247, Q-0248, Q-0249, Q-0250 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Instantiation](../specification/templates/instantiation.md) | 36 | 16 required, 13 reject, 1 undefined, 2 unspecified, 1 impl-defined, 3 no-diagnostic | 130 | 1 | 18 | 55 | Q-0231, Q-0232, Q-0233, Q-0234, Q-0235, Q-0236, Q-0237, Q-0251, Q-0252, Q-0253, Q-0254, Q-0255, Q-0256, Q-0257, Q-0310 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Concepts and requires expressions](../specification/templates/concepts.md) | 37 | 21 required, 16 reject | 118 | 0 | 17 | 50 | Q-0258, Q-0259, Q-0260, Q-0270, Q-0285, Q-0286, Q-0287, Q-0288, Q-0289, Q-0290 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Constraints](../specification/templates/constraints.md) | 39 | 24 required, 13 reject, 2 no-diagnostic | 122 | 0 | 26 | 43 | Q-0260, Q-0261, Q-0262, Q-0263, Q-0264, Q-0265, Q-0266, Q-0267, Q-0268, Q-0269, Q-0281, Q-0282, Q-0283, Q-0284, Q-0291, Q-0292 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Packs](../specification/templates/packs.md) | 31 | 19 required, 11 reject, 1 no-diagnostic | 69 | 0 | 4 | 45 | Q-0238, Q-0271, Q-0272, Q-0273, Q-0274 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Class template argument deduction](../specification/templates/class-template-deduction.md) | 29 | 21 required, 8 reject | 113 | 0 | 19 | 32 | Q-0239, Q-0240, Q-0241, Q-0242, Q-0243, Q-0244, Q-0275, Q-0276, Q-0277, Q-0278, Q-0279, Q-0280 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Throw expressions and the exception object](../specification/exceptions/throw.md) | 13 | 11 required, 1 reject | 29 | 6 | 0 | 28 |  | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Try blocks and handlers](../specification/exceptions/handlers.md) | 18 | 8 required, 6 reject, 1 undefined, 3 recommended | 78 | 6 | 2 | 52 |  | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Unwinding and termination](../specification/exceptions/unwinding.md) | 9 | 7 required, 1 impl-defined, 1 recommended | 35 | 2 | 1 | 42 | Q-0179 | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |
| [Glossary](../specification/glossary.md) | 119 terms | | | | | | | none flagged ([2026-11-03-repairs-rs-rec-red](../reports/similarity/2026-11-03-repairs-rs-rec-red.txt)) |

`tools/build_site.py` builds the specification as a searchable HTML site
(decision 0016). CI builds the site on every change and publishes it to
https://sempuki.github.io/tidal/ after each push to `main`. Pages use
the structure and writing rules in `documentation/style-guide.md`. Every page
opens with its principles (decision 0024), and the Foundations chapter
holds the models that the other pages cite.
The latest similarity report that checked every page is `reports/similarity/2026-11-03-repairs-rs-rec-red.txt`.
The table above comes from `tools/status_table.py`. Its Similarity
column names the latest similarity report that checked each page, and
its Report mentions column leaves out the roadmap reports.
`tools/status_table.py` also writes the counts in the prose of this
page, and `tools/check_specification.py` fails when one is stale.

## Test matrix

The matrix has 8 baseline compilers, and 5408 of 5408 tests pass on each. Two
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
| EDG 7.0 | container | front end only; skips `-m32`, warning-flag, and sanitizer tests; per-compiler expectations on 175 tests, each with a note that names its question or the implementation choice |
| EDG 7.0, ILP32 | container, built with EDG's i686 configuration | front end only; runs only the `-m32` tests; differs on the alignment of `long long` (Q-0120) |
| GCC 16.2.0, `-O2` | container, report | runs the tests at `-O2`; does not fail a run |
| Clang 22.1.8, `-O2` | container, report | runs the tests at `-O2`; does not fail a run |

Full results are in `tests/RESULTS.md`.

## Open questions

The question index has 331 questions: 292 open for opinions (291
provisional and 1 open), 10 compiler defects, and 29 settled. Each open
question has a provisional answer that the specification follows. A
compiler-defect answer says that one compiler or an older version is
wrong (Q-0002, Q-0003, Q-0013, Q-0016, Q-0044, Q-0115, Q-0212, Q-0213,
Q-0238, Q-0309). Q-0009 (license) is among the settled questions. See
`questions/README.md`.

## Known limits of the specification text

- The glossary covers the terms used so far. Terms marked *(referenced)*
  belong to topics that Tidal does not specify yet.
- 29 *(referenced)* markers remain. Each names a topic with no page: the
  library (15), extended floating-point types (5), coroutines (3), class
  properties (2), the preprocessor (2), the memory model (1), or threads
  (1).

## Known limits of the tooling

- The clean-room front end passes 4946 of 5520 tests, and 5083 pass their check.
  The front end reads `#include "file"` and no standard header.
  `tools/frontend_check.py` fails when a test that it passed no longer
  passes (`reports/clean-room/rust-frontend/conformance.tsv`).

- 2575 tests carry `tidal-error` patterns. A pattern checks that a
  compiler rejects the test, fails to link the test, or stops the test
  at run time, for the reason the test's rule gives.
  In 45 tests, some compiler family has no pattern, because that
  family reports only a generic parse or constant-evaluation error.
- Undefined-behavior evidence comes from constant evaluation, warnings,
  and the UBSan and ASan sanitizers (80 tests, decision 0028). Errors
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

`documentation/roadmap.md` orders the work (decisions 0033, 0039, 0050,
and 0051). Phases 1 to 5 are done, and phase 6 is provisional. Next:

1. Phase 6, items R0, G, W (first wave), and D, which need nothing
   before them. Then Q, P, C, O, and FE (decision 0051).
2. A Clang `-fsanitize=type` configuration for STG-18 and STG-23
   (Clang 22 reports these accesses but does not stop the run). The
   copy-move triviality split goes with item O, and the rule that a
   class and a variable can share a name goes with item C.

Waiting on the maintainer:

- Ask the reviewer who prompted the principles-first change to read the
  Foundations chapter and two topic pages (decision 0024).
- Approve reporting the EDG behavior in Q-0002 and Q-0003 to the EDG
  project.
- Confirm or change the provisional choices of decision 0051: settling
  a question when the newest version of each family agrees, and
  keeping the implementation-only operations referenced (Q-0322).
- Decide Q-0300 and Q-0181, which the gap review of question cluster 1
  reopened. In both, the compilers of the matrix agree with each other,
  and the review reads the C++23 text the other way. Q-0181 also asks
  whether Tidal follows a later change that the newest compilers apply.
- Decide whether to probe which C++23 rejections later language
  versions accept, as an index with no rules (designer view, item 4;
  Q-0007).

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
