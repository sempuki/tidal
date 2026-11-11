# Status

Where the project stands and what comes next. This page describes the
current state only. The history of how the project got here is in
`git log`.

## Specification

| Section | Rules | Classes | Tests | Single-test rules | Overrides | Report mentions | Questions | Evidence words | Similarity |
|---|---|---|---|---|---|---|---|---|---|
| [Translation, execution, and behavior classes](../specification/foundations/translation.md) | 17 | 4 required, 5 reject, 1 undefined, 1 unspecified, 5 impl-defined, 1 recommended | 89 | 0 | 24 | 151 | Q-0012, Q-0024, Q-0029, Q-0312, Q-0350, Q-0351, Q-0366 | 2492 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Fundamental types](../specification/foundations/types.md) | 14 | 7 required, 1 reject, 6 impl-defined | 63 | 0 | 2 | 76 | Q-0120 | 470 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Language-support types](../specification/foundations/language-support.md) | 13 | 8 required, 4 reject, 1 no-diagnostic | 74 | 0 | 22 | 188 | Q-0125, Q-0299, Q-0301, Q-0302, Q-0303, Q-0321, Q-0323, Q-0334, Q-0386, Q-0402, Q-0421, Q-0431 | 1745 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Keywords and reserved names](../specification/foundations/keywords.md) | 4 | 2 required, 1 reject, 1 no-diagnostic | 101 | 0 | 1 | 55 | Q-0382 | 210 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Objects, storage, and lifetime](../specification/foundations/objects.md) | 12 | 8 required, 3 undefined, 1 impl-defined | 63 | 0 | 5 | 150 | Q-0115, Q-0368, Q-0374, Q-0416 | 744 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Raw storage and object representation](../specification/foundations/storage.md) | 29 | 17 required, 1 reject, 7 undefined, 2 unspecified, 2 impl-defined | 110 | 0 | 6 | 290 | Q-0304, Q-0305, Q-0306, Q-0307, Q-0308, Q-0323, Q-0324, Q-0325, Q-0327, Q-0328, Q-0350, Q-0357, Q-0359, Q-0425, Q-0430 | 1786 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Value categories and temporaries](../specification/foundations/values.md) | 16 | 9 required, 4 reject, 2 unspecified, 1 impl-defined | 68 | 0 | 2 | 81 | Q-0034, Q-0054 | 291 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Standard conversions](../specification/foundations/conversions.md) | 21 | 12 required, 6 reject, 1 undefined, 2 impl-defined | 98 | 0 | 6 | 168 | Q-0028, Q-0042, Q-0100, Q-0331 | 457 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Names, scopes, and lookup](../specification/foundations/names.md) | 42 | 22 required, 20 reject | 246 | 0 | 5 | 186 | Q-0311, Q-0316, Q-0362, Q-0364, Q-0377, Q-0410, Q-0415 | 1043 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Use, headers, and the one-definition rule](../specification/foundations/odr.md) | 20 | 10 required, 3 reject, 2 impl-defined, 4 no-diagnostic, 1 recommended | 98 | 0 | 18 | 123 | Q-0127, Q-0128, Q-0129, Q-0130, Q-0297, Q-0315, Q-0358, Q-0367, Q-0369 | 1406 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Sequencing and evaluation order](../specification/foundations/evaluation-order.md) | 11 | 5 required, 1 undefined, 2 unspecified, 1 recommended | 40 | 0 | 0 | 153 | Q-0319 | 500 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Constant expressions](../specification/foundations/constant-evaluation.md) | 32 | 4 required, 20 reject, 1 unspecified, 1 impl-defined, 1 no-diagnostic | 163 | 0 | 30 | 423 | Q-0002, Q-0004, Q-0020, Q-0033, Q-0039, Q-0078, Q-0090, Q-0115, Q-0117, Q-0192, Q-0193, Q-0318, Q-0335, Q-0349, Q-0421, Q-0425 | 1982 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Expression statements and blocks](../specification/statements/blocks.md) | 10 | 7 required, 2 reject, 1 recommended | 29 | 0 | 2 | 28 | Q-0096 | 198 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [The if statement](../specification/statements/if.md) | 25 | 14 required, 11 reject | 81 | 0 | 1 | 100 | Q-0329 | 202 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [The switch statement](../specification/statements/switch.md) | 8 | 2 required, 4 reject, 2 recommended | 44 | 0 | 1 | 80 | Q-0041 | 351 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Iteration statements](../specification/statements/loops.md) | 9 | 5 required, 4 reject | 29 | 0 | 1 | 19 | Q-0044 | 199 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [The range-based for statement](../specification/statements/range-for.md) | 8 | 5 required, 3 reject | 36 | 0 | 8 | 85 | Q-0020, Q-0026, Q-0098, Q-0122 | 409 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Jump statements and labels](../specification/statements/jumps.md) | 17 | 9 required, 7 reject, 1 unspecified | 69 | 0 | 4 | 114 | Q-0041, Q-0121, Q-0422 | 373 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Primary expressions](../specification/expressions/primary.md) | 35 | 14 required, 10 reject, 1 undefined, 1 unspecified, 4 impl-defined, 4 conditionally-supported, 1 recommended | 147 | 0 | 13 | 110 | Q-0029, Q-0033, Q-0063, Q-0064, Q-0194, Q-0330, Q-0380, Q-0436, Q-0440 | 1026 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [decltype, unevaluated operands, and discarded values](../specification/expressions/decltype.md) | 11 | 5 required, 4 reject, 2 recommended | 54 | 0 | 5 | 47 | Q-0065, Q-0089 | 231 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Multiplicative operators](../specification/expressions/multiplicative.md) | 20 | 7 required, 3 reject, 4 undefined, 2 unspecified, 3 impl-defined, 1 recommended | 54 | 0 | 5 | 82 | Q-0001, Q-0002, Q-0003, Q-0004, Q-0005 | 858 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Additive operators](../specification/expressions/additive.md) | 17 | 7 required, 1 reject, 6 undefined, 1 unspecified, 1 impl-defined, 1 recommended | 67 | 0 | 9 | 75 | Q-0001, Q-0005, Q-0031, Q-0049, Q-0298, Q-0304, Q-0430 | 1212 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Shift operators](../specification/expressions/shift.md) | 9 | 6 required, 1 reject, 1 undefined, 1 recommended | 30 | 0 | 2 | 32 | Q-0332 | 189 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Comparison operators](../specification/expressions/comparison.md) | 27 | 15 required, 6 reject, 2 unspecified, 4 recommended | 103 | 0 | 19 | 128 | Q-0029, Q-0030, Q-0050, Q-0051, Q-0103, Q-0104, Q-0387 | 1223 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Bitwise operators](../specification/expressions/bitwise.md) | 8 | 4 required, 1 reject, 3 recommended | 24 | 0 | 0 | 12 |  | 104 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Logical operators](../specification/expressions/logical.md) | 14 | 10 required, 2 reject, 2 recommended | 34 | 0 | 1 | 35 | Q-0101 | 173 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [The conditional operator](../specification/expressions/conditional.md) | 16 | 9 required, 7 reject | 94 | 0 | 8 | 125 | Q-0102, Q-0373, Q-0378, Q-0401, Q-0418 | 712 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Assignment operators](../specification/expressions/assignment.md) | 13 | 7 required, 5 reject, 1 unspecified | 37 | 0 | 1 | 37 | Q-0032 | 152 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [The comma operator](../specification/expressions/comma.md) | 9 | 5 required, 3 reject, 1 recommended | 25 | 0 | 0 | 11 |  | 52 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Unary operators](../specification/expressions/unary.md) | 21 | 10 required, 6 reject, 3 undefined, 1 impl-defined, 1 recommended | 75 | 0 | 4 | 101 | Q-0031, Q-0124, Q-0318 | 335 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Pointers to members](../specification/expressions/member-pointers.md) | 12 | 5 required, 5 reject, 1 undefined, 1 unspecified | 81 | 0 | 2 | 60 | Q-0333, Q-0353 | 293 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Postfix expressions](../specification/expressions/postfix.md) | 22 | 8 required, 8 reject, 5 undefined, 1 conditionally-supported | 83 | 0 | 8 | 118 | Q-0031, Q-0034, Q-0124, Q-0178, Q-0316, Q-0318 | 888 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Casts](../specification/expressions/casts.md) | 19 | 8 required, 7 reject, 2 undefined, 2 impl-defined | 64 | 0 | 1 | 71 | Q-0017, Q-0325 | 363 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [New and delete](../specification/expressions/new-delete.md) | 42 | 22 required, 11 reject, 4 undefined, 3 unspecified, 2 impl-defined | 156 | 0 | 26 | 259 | Q-0005, Q-0035, Q-0036, Q-0091, Q-0125, Q-0220, Q-0221, Q-0222, Q-0300, Q-0301, Q-0302, Q-0303, Q-0313, Q-0417 | 2329 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Lambda expressions](../specification/expressions/lambdas.md) | 29 | 16 required, 9 reject, 1 undefined, 2 unspecified, 1 recommended | 99 | 0 | 4 | 151 | Q-0037, Q-0092 | 793 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Placeholder types in variable declarations](../specification/declarations/auto.md) | 22 | 12 required, 10 reject | 65 | 0 | 0 | 46 | Q-0010 | 83 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Type aliases and class names](../specification/declarations/type-names.md) | 10 | 4 required, 5 reject, 1 no-diagnostic | 70 | 0 | 9 | 38 | Q-0066, Q-0067, Q-0093 | 486 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Storage duration and linkage](../specification/declarations/storage-linkage.md) | 23 | 14 required, 6 reject, 1 undefined, 2 no-diagnostic | 85 | 0 | 2 | 95 | Q-0011, Q-0045, Q-0105, Q-0297 | 540 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [The inline specifier](../specification/declarations/inline.md) | 8 | 4 required, 2 reject, 2 no-diagnostic | 37 | 0 | 3 | 26 | Q-0068 | 189 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Declarators and cv-qualifiers](../specification/declarations/declarators.md) | 34 | 17 required, 14 reject, 1 undefined, 1 recommended | 104 | 0 | 3 | 107 | Q-0106, Q-0191, Q-0348 | 458 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Alignment](../specification/declarations/alignment.md) | 8 | 2 required, 4 reject, 1 impl-defined, 1 no-diagnostic | 54 | 0 | 25 | 78 | Q-0069, Q-0070, Q-0071, Q-0072, Q-0073, Q-0120, Q-0313, Q-0314, Q-0360 | 1175 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Volatile](../specification/declarations/volatile.md) | 13 | 5 required, 5 reject, 1 undefined, 1 impl-defined, 1 recommended | 42 | 0 | 1 | 44 | Q-0080, Q-0176, Q-0320, Q-0326, Q-0359 | 558 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Forms of initialization](../specification/declarations/initialization/forms.md) | 9 | 6 required, 2 reject, 1 undefined | 41 | 0 | 1 | 64 | Q-0013, Q-0039 | 296 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [List initialization and aggregates](../specification/declarations/initialization/list.md) | 16 | 8 required, 8 reject | 73 | 0 | 4 | 83 | Q-0042, Q-0043, Q-0166, Q-0167, Q-0331 | 540 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Reference binding](../specification/declarations/initialization/references.md) | 14 | 7 required, 4 reject, 1 undefined, 1 recommended | 74 | 0 | 15 | 143 | Q-0040, Q-0109, Q-0110, Q-0149, Q-0150, Q-0151, Q-0157, Q-0164, Q-0165, Q-0166, Q-0180, Q-0294 | 1222 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Initialization of static and thread storage](../specification/declarations/initialization/static.md) | 7 | 3 required, 2 unspecified, 2 impl-defined | 26 | 0 | 0 | 96 | Q-0014, Q-0175 | 554 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Structured bindings](../specification/declarations/structured-bindings.md) | 17 | 10 required, 7 reject | 71 | 0 | 7 | 80 | Q-0007, Q-0015, Q-0016, Q-0048, Q-0107, Q-0376, Q-0395 | 568 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Enumerations](../specification/declarations/enumerations.md) | 22 | 11 required, 8 reject, 1 undefined, 1 unspecified, 1 impl-defined | 81 | 0 | 3 | 77 | Q-0017, Q-0018, Q-0047, Q-0114, Q-0336, Q-0407 | 687 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Namespaces and using](../specification/declarations/namespaces.md) | 17 | 8 required, 9 reject | 57 | 0 | 3 | 58 | Q-0019, Q-0371, Q-0375 | 413 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Linkage specifications](../specification/declarations/linkage-specifications.md) | 8 | 3 required, 2 reject, 2 impl-defined, 1 no-diagnostic | 45 | 0 | 13 | 42 | Q-0074, Q-0075, Q-0076, Q-0369 | 616 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Standard attributes](../specification/declarations/attributes.md) | 20 | 7 required, 4 reject, 2 undefined, 2 impl-defined, 5 recommended | 90 | 0 | 18 | 71 | Q-0020, Q-0021, Q-0046, Q-0108, Q-0123, Q-0177, Q-0347, Q-0390 | 1100 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Function definitions](../specification/declarations/functions/definitions.md) | 16 | 5 required, 6 reject, 1 undefined, 3 impl-defined, 1 recommended | 63 | 0 | 13 | 83 | Q-0022, Q-0024, Q-0076, Q-0369 | 923 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Default arguments](../specification/declarations/functions/default-arguments.md) | 13 | 3 required, 9 reject, 1 no-diagnostic | 51 | 0 | 10 | 68 | Q-0038, Q-0111, Q-0140, Q-0370, Q-0379, Q-0389 | 670 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Deleted and defaulted functions](../specification/declarations/functions/deleted.md) | 5 | 1 required, 4 reject | 21 | 0 | 1 | 28 | Q-0007, Q-0023 | 101 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Return type deduction](../specification/declarations/functions/return-deduction.md) | 7 | 2 required, 4 reject, 1 undefined | 28 | 0 | 0 | 68 |  | 55 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Exception specifications](../specification/declarations/exception-specifications.md) | 10 | 4 required, 4 reject, 1 impl-defined, 1 no-diagnostic | 46 | 0 | 11 | 89 | Q-0077, Q-0078, Q-0079, Q-0094, Q-0095, Q-0393 | 624 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [The constexpr, consteval, and constinit specifiers](../specification/declarations/constexpr.md) | 13 | 2 required, 7 reject, 1 no-diagnostic | 74 | 0 | 10 | 136 | Q-0020, Q-0025, Q-0192, Q-0193, Q-0413 | 561 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Static assertions](../specification/declarations/static-assert.md) | 8 | 4 required, 4 reject | 30 | 0 | 0 | 31 |  | 30 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Class definitions and members](../specification/classes/members.md) | 34 | 18 required, 11 reject, 2 unspecified, 2 impl-defined, 1 no-diagnostic | 174 | 0 | 16 | 186 | Q-0027, Q-0080, Q-0181, Q-0317, Q-0338, Q-0341, Q-0342, Q-0361, Q-0381, Q-0384, Q-0406 | 1423 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Access control and friends](../specification/classes/access.md) | 20 | 9 required, 11 reject | 95 | 0 | 6 | 82 | Q-0052, Q-0131, Q-0391, Q-0404, Q-0414 | 423 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Constructors and destructors](../specification/classes/constructors.md) | 34 | 13 required, 17 reject, 2 undefined, 1 no-diagnostic, 1 recommended | 206 | 0 | 21 | 242 | Q-0041, Q-0053, Q-0054, Q-0082, Q-0083, Q-0084, Q-0126, Q-0374 | 1304 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [List initialization of classes](../specification/classes/initializer-lists.md) | 10 | 5 required, 5 reject | 36 | 0 | 2 | 61 | Q-0081, Q-0132 | 175 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Copy and move](../specification/classes/copy-move.md) | 28 | 15 required, 10 reject, 2 unspecified, 1 recommended | 137 | 0 | 9 | 183 | Q-0055, Q-0115, Q-0339, Q-0343, Q-0344, Q-0426 | 1193 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Derived classes](../specification/classes/derived.md) | 22 | 9 required, 12 reject, 1 undefined | 91 | 0 | 1 | 99 | Q-0085 | 86 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Virtual functions and abstract classes](../specification/classes/virtual.md) | 21 | 7 required, 10 reject, 1 undefined, 2 no-diagnostic, 1 recommended | 147 | 0 | 7 | 176 | Q-0056, Q-0086, Q-0087, Q-0212, Q-0355, Q-0394 | 1172 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Unions and bit-fields](../specification/classes/unions.md) | 28 | 16 required, 8 reject, 1 undefined, 2 impl-defined, 1 recommended | 179 | 0 | 20 | 206 | Q-0045, Q-0058, Q-0088, Q-0120, Q-0213, Q-0309, Q-0313, Q-0314, Q-0339, Q-0340, Q-0345, Q-0372, Q-0424, Q-0425 | 1956 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Conversion functions](../specification/classes/conversion-functions.md) | 12 | 5 required, 6 reject, 1 recommended | 70 | 0 | 3 | 45 | Q-0059, Q-0388 | 250 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Operator overloading](../specification/classes/operators.md) | 16 | 8 required, 6 reject, 1 unspecified, 1 no-diagnostic | 122 | 0 | 4 | 121 | Q-0060, Q-0383, Q-0435, Q-0436, Q-0440 | 540 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Defaulted comparisons](../specification/classes/comparisons.md) | 13 | 6 required, 6 reject, 1 recommended | 82 | 0 | 7 | 90 | Q-0061, Q-0062, Q-0158, Q-0190, Q-0403 | 494 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Implicit conversion sequences](../specification/overloading/conversion-sequences.md) | 47 | 46 required, 1 reject | 145 | 0 | 31 | 253 | Q-0081, Q-0132, Q-0133, Q-0134, Q-0135, Q-0136, Q-0137, Q-0138, Q-0152, Q-0153, Q-0154, Q-0155, Q-0156, Q-0157, Q-0167, Q-0188, Q-0294, Q-0321, Q-0337, Q-0419, Q-0423 | 2218 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Overload resolution](../specification/overloading/overload-resolution.md) | 44 | 35 required, 9 reject | 207 | 0 | 30 | 382 | Q-0138, Q-0139, Q-0140, Q-0141, Q-0142, Q-0143, Q-0144, Q-0145, Q-0146, Q-0147, Q-0148, Q-0159, Q-0161, Q-0163, Q-0187, Q-0189, Q-0211, Q-0411 | 2069 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Template declarations](../specification/templates/declarations.md) | 49 | 26 required, 23 reject | 141 | 0 | 12 | 127 | Q-0195, Q-0196, Q-0197, Q-0198, Q-0199, Q-0200, Q-0296, Q-0385 | 930 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Template arguments](../specification/templates/arguments.md) | 27 | 17 required, 10 reject | 99 | 0 | 6 | 106 | Q-0117, Q-0168, Q-0169, Q-0170, Q-0201, Q-0409 | 697 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Explicit and partial specialization](../specification/templates/specialization.md) | 32 | 16 required, 15 reject, 1 no-diagnostic | 97 | 0 | 15 | 62 | Q-0171, Q-0172, Q-0173, Q-0174, Q-0203, Q-0204, Q-0205, Q-0206, Q-0207, Q-0292, Q-0295 | 990 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Template argument deduction](../specification/templates/deduction.md) | 39 | 36 required, 3 reject | 163 | 0 | 13 | 318 | Q-0208, Q-0209, Q-0215, Q-0219, Q-0220, Q-0221, Q-0222, Q-0295, Q-0399, Q-0420, Q-0429 | 2045 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Partial ordering of templates](../specification/templates/ordering.md) | 17 | 16 required, 1 reject | 65 | 0 | 19 | 175 | Q-0142, Q-0146, Q-0211, Q-0214, Q-0216, Q-0217, Q-0218, Q-0219, Q-0293, Q-0396, Q-0397, Q-0398, Q-0405 | 1783 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Dependent names](../specification/templates/dependent-names.md) | 40 | 21 required, 13 reject, 6 no-diagnostic | 147 | 0 | 27 | 324 | Q-0223, Q-0224, Q-0225, Q-0226, Q-0227, Q-0228, Q-0229, Q-0230, Q-0245, Q-0246, Q-0247, Q-0248, Q-0249, Q-0250, Q-0354, Q-0427, Q-0432, Q-0434 | 2581 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Instantiation](../specification/templates/instantiation.md) | 36 | 16 required, 14 reject, 2 unspecified, 3 no-diagnostic | 168 | 0 | 32 | 224 | Q-0231, Q-0232, Q-0233, Q-0234, Q-0235, Q-0236, Q-0237, Q-0251, Q-0252, Q-0253, Q-0254, Q-0255, Q-0256, Q-0257, Q-0310, Q-0346, Q-0392, Q-0428, Q-0433, Q-0441 | 2675 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Concepts and requires expressions](../specification/templates/concepts.md) | 37 | 21 required, 16 reject | 126 | 0 | 18 | 124 | Q-0258, Q-0259, Q-0260, Q-0270, Q-0281, Q-0285, Q-0286, Q-0287, Q-0288, Q-0289, Q-0290, Q-0400, Q-0411 | 1621 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Constraints](../specification/templates/constraints.md) | 40 | 24 required, 12 reject, 4 no-diagnostic | 147 | 0 | 34 | 173 | Q-0260, Q-0261, Q-0262, Q-0263, Q-0264, Q-0265, Q-0266, Q-0267, Q-0268, Q-0269, Q-0281, Q-0282, Q-0283, Q-0284, Q-0285, Q-0289, Q-0291, Q-0292, Q-0385, Q-0392, Q-0411, Q-0426 | 2494 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Packs](../specification/templates/packs.md) | 31 | 19 required, 11 reject, 1 no-diagnostic | 82 | 0 | 4 | 96 | Q-0238, Q-0271, Q-0272, Q-0273, Q-0274 | 1213 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Class template argument deduction](../specification/templates/class-template-deduction.md) | 29 | 21 required, 8 reject | 133 | 0 | 26 | 215 | Q-0020, Q-0081, Q-0239, Q-0240, Q-0241, Q-0242, Q-0243, Q-0244, Q-0275, Q-0276, Q-0277, Q-0278, Q-0279, Q-0280, Q-0352, Q-0408, Q-0437, Q-0438, Q-0439 | 2332 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Throw expressions and the exception object](../specification/exceptions/throw.md) | 13 | 11 required, 1 reject | 50 | 0 | 0 | 77 |  | 418 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Try blocks and handlers](../specification/exceptions/handlers.md) | 18 | 8 required, 6 reject, 1 undefined, 3 recommended | 105 | 0 | 7 | 119 | Q-0363 | 981 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Unwinding and termination](../specification/exceptions/unwinding.md) | 11 | 6 required, 1 reject, 2 impl-defined, 1 recommended | 60 | 0 | 10 | 165 | Q-0179, Q-0365 | 1018 | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |
| [Glossary](../specification/glossary.md) | 157 terms | | | | | | | | none flagged ([2026-11-11-phase-15-f5](../reports/similarity/2026-11-11-phase-15-f5.txt)) |

`tools/build_site.py` builds the specification as a searchable HTML site
(decision 0016). CI builds the site on every change and publishes it to
https://sempuki.github.io/tidal/ after each push to `main`. Pages use
the structure and writing rules in `documentation/style-guide.md`. Every page
opens with its principles (decision 0024), and the Foundations chapter
holds the models that the other pages cite.
The latest similarity report that checked every page is `reports/similarity/2026-11-11-phase-15-f5.txt`.
The table above comes from `tools/status_table.py`. Its Similarity
column names the latest similarity report that checked each page, its
Report mentions column leaves out the roadmap reports, and its Evidence
words column counts the words of each page's Evidence segment.
`tools/status_table.py` also writes the counts in the prose of this
page, and `tools/check_specification.py` fails when one is stale.

## Test matrix

The matrix has 8 baseline compilers, and 6947 of 6947 tests pass on each. The tree has 6989 test files, and 42 of them have no row in `tests/RESULTS.md`. Two
report compilers build at `-O2` (decision 0043). Tests may
span several translation units (decision 0014). CI runs the
containerized compilers on every push and pull request (decision 0012).

| Compiler | Where | Notes |
|---|---|---|
| Clang 22.1.8 | host, and container (apt.llvm.org) | |
| GCC 16.2.1 | host | skips type sanitizer tests |
| GCC 16.2.0 | container (official image) | skips `-m32` and type sanitizer tests |
| Clang 18.1.3 | container | skips type sanitizer tests |
| GCC 14.2.0 | container | skips type sanitizer tests |
| EDG 7.0 | container | front end only; skips `-m32`, warning-flag, and sanitizer tests; per-compiler expectations on 307 tests, each with a note that names its question or the implementation choice |
| EDG 7.0, ILP32 | container, built with EDG's i686 configuration | front end only; runs only the `-m32` tests; differs on the alignment of `long long` (Q-0120) |
| GCC 16.2.0, `-O2` | container, report | runs the tests at `-O2`; skips `-m32` and type sanitizer tests; does not fail a run |
| Clang 22.1.8, `-O2` | container, report | runs the tests at `-O2`; does not fail a run |

Full results are in `tests/RESULTS.md`.

## Open questions

The question index has 441 questions: 192 open for opinions (192
provisional and 0 open), 13 compiler defects, and 236 settled. Each open
question has a provisional answer that the specification follows. A
compiler-defect answer says that one compiler or an older version is
wrong (Q-0002, Q-0003, Q-0013, Q-0016, Q-0044, Q-0115, Q-0212, Q-0213,
Q-0238, Q-0309, Q-0332, Q-0333, Q-0356). Q-0009 (license) is among the
settled questions. See `questions/README.md`.

## Known limits of the specification text

- The glossary covers the terms used so far. Terms marked *(referenced)*
  belong to topics that Tidal does not specify yet.
- 33 *(referenced)* markers remain. Each names a topic with no page: the
  library (21), extended floating-point types (5), coroutines (3), the
  preprocessor (2), the memory model (1), or threads (1).

## Known limits of the tooling

- The clean-room front end passed 6801 of 6938 tests at front-end round 25 (`conformance.tsv`), and 6825 passed their check.
  At that round, its interpreter ran 1825 tests: 1791 gave the expected result (20 of them with a partial exploration), 0 gave a wrong result, 24 stopped as unsupported, and 10 did not run because the check failed.
  The front end reads `#include "file"` and no standard header.
  `tools/frontend_check.py` fails when a test that it passed no longer
  passes (`reports/clean-room/rust-frontend/conformance.tsv`).
  A test added after the last front-end round has no row in
  `conformance.tsv`, and `tools/frontend_check.py` lists each such test
  that the front end does not pass ("new test ... not passing"). The
  next front-end round takes them.

- 3163 tests carry `tidal-error` patterns. A pattern checks that a
  compiler rejects the test, fails to link the test, or stops the test
  at run time, for the reason the test's rule gives.
  In 62 tests, some compiler family has no pattern, because that
  family reports only a generic parse or constant-evaluation error.
- Undefined-behavior evidence comes from constant evaluation, warnings,
  and the UBSan, ASan, and type sanitizers (110 tests, decision 0028).
  Only Clang 22 has the type sanitizer, and the type sanitizer does not
  stop a run. The test runner counts its report as the run failure
  (`documentation/specification-format.md`). Errors that no sanitizer
  reports (a second destruction of a stack object, a modified `const`
  object) have constant-evaluation tests only.
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
0051, 0054, 0055, 0056, 0057, 0058, 0059, 0060, 0061, 0062, and 0063).
Phases 1 to 14 are done. Phase 15 (decision 0063) is next:

1. Items T4, F5, GC, R9, and the read of the RV7 repair diff, which can
   run together. F5 answers 26 open exercise rows (I7 and P3) and the
   open front-end rows G164 and G165.
2. Front-end round 26 (item FE) after T4 and GC, then Port suite P4.
   RD after GC, and Propose X25 after R9's portfolio step.
3. The RV8 gap review after F5.

The rewrite's code starts when a front-end round ends inside the gate
of decisions 0062 and 0063: at most 50 failing tests that the marks file
does not mark, no test made worse in two rounds, and a named cause for
each remaining test.

After the front end meets the completion test of decision 0060, a
phase item rewrites it (roadmap item FR).

The items that wait on the maintainer are in
`documentation/decisions/pending.md`, one row each, with the options,
the answer Tidal keeps until the maintainer decides, and the questions
the decision closes.

## Size estimate

| Chapter | Pages | Live rules | Words |
|---|---|---|---|
| Foundations | 12 | 224 | 48462 |
| Statements | 6 | 77 | 10136 |
| Expressions | 17 | 324 | 44497 |
| Declarations | 23 | 315 | 47532 |
| Classes | 11 | 238 | 39245 |
| Overloading | 2 | 91 | 14182 |
| Templates | 11 | 376 | 65512 |
| Exceptions | 3 | 40 | 9308 |
| Total | 85 | 1685 | 278874 |

Every topic in scope has a page (section "Topics in scope with no page
yet"), so these counts give the size of the scope. The expressions and
statements chapters, the scope that the charter's third success
criterion names, hold 23 pages, 401 live rules, and 54633 words.
`tools/status_table.py` writes this section.
