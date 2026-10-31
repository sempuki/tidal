# Roadmap: where editing effort should go next, from an editor's view

Date: 2026-10-31. Decision 0029. A clean-room reader in the role of the
editor responsible for the specification's quality read docs/,
AGENTS.md, spec/, tests/, questions/, and reports/clean-room/, and
counted the evidence below with read-only commands. Some findings it
cites were acted on in commits made while it read (ba0f7a8, 460916c,
631651f); the combined roadmap marks them.

Columns: **Q** open questions per page; **F** distinct clean-room
findings citing the page (R-foundations, W0, rust-frontend); **X** tests
with per-compiler overrides, mapped through `tidal-rules`; **1t** rules
backed by exactly one test; **Req+rej** `required` rules whose tests
include `reject` or `link-fail`; **Ref** `*(referenced)*` markers.

Repo-wide: 822 rules, 205 backed by a single test; 178 of 464 `required`
rules have reject or link-fail tests; 92 `*(referenced)*` markers; 111
open questions.

## 1. The ten weakest pages

| # | Page | Rules / tests | Q | F | X | 1t | Req+rej | Ref | Other evidence |
|---|---|---|---|---|---|---|---|---|---|
| 1 | foundations/constant-evaluation | 18 / 47 | 4 | 13 | 3 | 6 | 2 | 1 | CE-9 lists few of the undefined rules that apply (FE F19); no rule that a constant's subobjects are initialized, and Clang and GCC differ (W0 F2); "literal type" undefined (FE F14) |
| 2 | foundations/translation | 9 / 27 | 1 | 12 | 1 | 0 | 2 | 1 | TR-3 contradicts FN-6 on `main`'s parameters (FE F16); TR-4 and TR-5 sit in Runtime Behavior with reject tests |
| 3 | expressions/multiplicative | 18 / 38 | 5 | 11 | 5 | 8 | 1 | 2 | Largest Evidence segment; Q-0001 to Q-0005 are unsettled UB classifications |
| 4 | foundations/conversions | 18 / 52 | 2 | 11 | 2 | 5 | (C6) | 1 | Qualification conversion lives on another page (W0 F10) |
| 5 | declarations/declarators | 25 / 80 | 2 | 8 | 2 | 5 | 10 | 2 | Declarator grammar incomplete; no procedure for building a type from a nested declarator (FE F4) |
| 6 | expressions/comparison | 22 / 67 | 6 | 3 | 9 | 5 | 3 | 1 | Without rewritten candidates, 10 front-end false rejections (FE F11) |
| 7 | declarations/alignment | 6 / 31 | 5 | 1 | 13 | 1 | 1 | 0 | 42% of its tests have overrides |
| 8 | expressions/additive | 16 / 40 | 4 | 6 | 6 | 9 | 1 | 1 | Q-0031, Q-0049; no way to get the range of `int` (W0 F4) |
| 9 | declarations/enumerations | 12 / 53 | 3 | 6 | 3 | 2 | 7 | 1 | ENUM-10 with ENUM-3 rejects a program a test accepts (FE F18) |
| 10 | declarations/attributes | 17 / 48 | 4 | 1 | 10 | 6 | 0 | 1 | Warnings only in Evidence (T2) |

Close behind: functions/definitions (the other half of TR-3 against
FN-6), exception-specifications (5 questions, 6 overrides, 6 rules),
constructors (5 questions, 6 overrides), logical (1.6 tests per rule,
7 of 13 rules single-test), initialization/static (11 tests). Twelve
pages, mostly classes and exceptions, have no finding because no
exercise has reached them yet.

## 2. Ranked work items

1. **Fix the contradictions** (small, about one session): TR-3 against
   FN-6 (F16, Q-0024, Q-0112); CX-1 and CINIT-1 "must have an
   initializer" against tests that accept `constexpr S s;` (F17);
   ENUM-10 with ENUM-3 (F18); the no-`main` case (T4); RET-1 and `void`
   deduction against AUTO-8 (F23); the glossary's bit-field promotion
   marker (F24).
2. **Repair rule classes so the class says what to diagnose** (all
   pages, two to four sessions). 178 of 464 `required` rules have reject
   or link-fail tests; the front end counts 420 reject tests citing only
   `required` rules (268 rules). Eight `required` rules in Runtime
   Behavior have reject tests: ILC-5, UNI-10, LIST-8, VOL-1, UNEV-2,
   OBJ-8, TR-4, TR-5. Needs a decision first: a rule with a reject test
   becomes `reject`, and its positive half stays `required`. Start with
   declarators (10), namespaces (8), and copy-move, members, auto,
   enumerations (7 each).
3. **Act on the Foundations findings** (two to three sessions), plus W0
   F2 (a constant needs every subobject initialized), FE F14 (define
   "literal type" and "constant initializer"), and FE F19 (a generated
   index of undefined rules on constant-evaluation.md, marked by what
   each compiler rejects).
4. **Sweep stale `*(referenced)*` markers** (about one session). About 40
   of 92 point to topics that now have pages: "classes referenced" in
   casts, unary, jumps, list, deleted, values; abstract classes in
   lambdas and new-delete; bit-fields in conditional and
   structured-bindings; operator functions in shift, bitwise,
   assignment, comma, logical; a throw-expression in unary; constant
   evaluation in the glossary; virtual bases in constructors; explicit
   constructors in forms. What remains then measures real gaps: about
   16 for overload resolution and lookup, 9 for templates and concepts,
   11 for the library, 3 for threads.
5. **Consolidate the grammar** (about three sessions): FE F1 to F8. Put
   the top-level productions and the expression chain on the chapter
   index pages.
6. **Add a matrix-targets page** (one to two sessions): sizes,
   alignments, signedness, `size_t` and `ptrdiff_t`, the range of
   `int`, exit-status bits, `argc`, layout of empty classes and
   bit-fields, under LP64 and `-m32` (W0 F4, F9; R-foundations T5, C3;
   FE F13).
7. **Deepen multiplicative, additive, comparison** (two to three
   sessions): settle Q-0001 to Q-0005, Q-0031, Q-0049, and the
   comparison questions; no rule on one test; move Evidence-only results
   into rules. Depends on items 2 and 6.
8. **Predict batches for the expressions pages, weakest first**, after
   the format gains outcomes that can catch errors (section 3).
9. **Thicken thin pages**: logical, initialization/static, comma,
   blocks, evaluation-order. At least one positive and one negative or
   edge test per rule.
10. **Ownership guidance, then W8** (W0 F1, F5, F6, F12).
11. **Divergence-heavy declaration pages**: alignment, attributes,
    exception-specifications (29 overrides, 14 questions). Check whether
    some rules fit `conditionally-supported` or `impl-defined` better.
12. **Name lookup beyond blocks on names.md** (FE F10). Depends on
    item 4.
13. **A minimal informative overload-resolution note** (needs a charter
    decision): the candidate sets and ranking that built-in operators
    and comparisons need (FE F11, W0 F6, 16 markers).
14. **The template subset the tests rely on** (needs the same
    decision): 182 tests use the `same<A, B>` idiom (FE F12, F22).

## 3. Process changes

- New checks in `tools/check_spec.py`: a `reject` test cites a `reject`,
  `undefined`, or (with `-W` flags) `recommended` rule; a `required`
  rule outside Constraints has no reject tests; an `undefined` rule has
  a constant-evaluation or run-fail test (flags VOL-6 and TRY-11); every
  grammar nonterminal is defined once; a `*(referenced)*` marker names
  no topic that has a page.
- New columns in the status table: single-test rules, overrides per
  page, findings per page (the coverage table that exercises.md promises).
- Move `tests/expressions/{constant-expressions, conversions,
  sequencing, value-categories}` under `tests/foundations/`, so tests
  mirror `spec/`. Generate the status page's EDG note: it says 4
  tests, and there are 61 `tidal-expect-edg` lines.
- Flag Evidence sentences that state a value or a warning with no rule
  behind it.
- Predict needs outcomes for "undefined at run time" and "depends on
  compiler"; without them the score saturates. Run Diagnose batches
  soon after item 2. Run front-end conformance on every spec change as
  a cheap regression signal, accepting either answer on override tests.
- Give questions that are, by their own provisional answers, defects in
  EDG or older compilers (Q-0002, Q-0003, Q-0013, Q-0015, Q-0016,
  Q-0023, Q-0026, Q-0044) a "compiler defect" status.

## 4. Out of scope for now

Lexical structure and the preprocessor (at most a note listing
alternative tokens and digraphs); the standard library (keep its types
referenced, state the language-side requirement where used, and use
`static_cast<T&&>` in the glossary); templates and overload resolution
in full (only the informative subsets of items 13 and 14); coroutines,
modules, threads, and the memory model; new topic pages in general;
MSVC and EDG 32-bit entries until the existing overrides are digested.
