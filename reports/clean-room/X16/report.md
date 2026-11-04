# X16: P4, chained comparisons

Date: 2026-11-03. Commit read: `573d849`. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. P4 is described under "### P4" in `reports/clean-room/roadmap/designer.md`. Its state is in `reports/clean-room/roadmap-4/designer.md`, which rates it ready and complete, with CMP-14 and SEQ-11 added for D10 and D11. The detailed design and the rules below are this report's own.

Inputs read:
- AGENTS.md, `documentation/style-guide.md` section 1, and `documentation/exercises.md` section 4.
- The output of `tools/propose_check.py` (12 reports, 29 conflicts, of which 1 is in X12: the withdrawn CE-31, merged into CE-8). Then `reports/clean-room/X12/report.md`, as the model for this report's form.
- `reports/clean-room/roadmap/designer.md` (P4), `reports/clean-room/roadmap-3/designer.md` (the D10 and D11 rows and the P4 row), and `reports/clean-room/roadmap-4/designer.md` (item 2 and the P4 row).
- In full: `specification/expressions/comparison.md`, `specification/foundations/evaluation-order.md`, and `specification/expressions/logical.md`.
- In `specification/overloading/overload-resolution.md`: OVL-P1 to OVL-P4, OVL-7 to OVL-13, OVL-43, OVL-32 and OVL-33.
- The cited rules of other pages:
  - DCMP-P1, DCMP-P2, DCMP-3, DCMP-6 and DCMP-12 (classes/comparisons),
  - OPR-6, OPR-7 and OPR-11 (classes/operators),
  - VC-P3 and VC-6 (values), and ASN-9,
  - PRIM-P6 and PRIM-27 to PRIM-29 (primary),
  - DEP-2, DEP-3, CPT-24 to CPT-27, UN-18 and DTY-2,
  - the warning-flag table and the list of errors with no flag in `specification/foundations/translation.md`.
- Q-0029.
- These tests:
  - for CMP-1, `grouping.cpp` and `chained-comparison.cpp`,
  - all four CMP-14 tests and the two SEQ-11 tests,
  - for SEQ-8, `expressions/sequencing/comparison-operand-order.cpp`,
  - for OPR-11, `classes/operators/associativity-kept.cpp`,
  - for PRIM-27, `expressions/primary/fold-expansion.cpp`.

Compiler runs:
- Small programs built with g++ 16.2.1 and clang++ 22.1.8 under `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X16/p`. Each run used `-std=c++23 -pedantic-errors`, with no further flag, with `-Wall -Wextra`, or with `-Wno-error=parentheses` where stated. Each run program flushes its output.
- Three scans of the test suite (`scan.sh`, `scang.sh` and `tokscan.py` in the X16 directory):
  - all 6335 test files compiled with `-fsyntax-only -Wparentheses` on GCC, collecting the chained-comparison warning,
  - the same on Clang,
  - a token scan for two relational operators at one level of parentheses.
- Section 4.5 gives the results. The harness did not let this subagent save `report.md`, so the integrator saves the text below.

Totals: 41 rule changes (3 new, 12 amended of which 3 are Examples or Evidence segments, 26 unchanged but must state their effect). 3 existing tests change their expected outcome. 1 more keeps its outcome under a different rule. 7 findings, X16-1 to X16-7.

## 1. Design

### 1.1 The feature

Today `a < b < c` groups as `(a < b) < c` (CMP-1). The `bool` result of `a < b` converts to `0` or `1` and is compared with `c`. P4 gives a chain of relational operators its mathematical meaning.

```cpp
int lo = 1, x = 5, hi = 3;
bool in   = lo < x < hi;    // today: (1 < 5) < 3, so 1 < 3, true
                            // P4: 1 < 5 && 5 < 3, false
bool down = 3 > 2 > 1;      // today: 1 > 1, false. P4: true
bool mix  = lo < x > hi;    // today: valid. P4: rejected (X16:new-2)
bool same = lo == x == hi;  // unchanged: (lo == x) == hi
bool par  = (lo < x) < hi;  // unchanged: parentheses keep today's grouping
```

The grammar does not change. The `relational_expr` production already reads a flat list of `three_way_expr` operands joined by relational operators. P4 changes the meaning that CMP-1 gives that list.

### 1.2 What a chain is (new rule X16:new-1)

A *comparison chain* is a `relational_expr` with two or more relational operators (`<`, `>`, `<=`, `>=`) and no parentheses between them. Each adjacent pair of operands with its operator is a *link*. `a < b <= c < d` has three links: `a < b`, `b <= c` and `c < d`.

The chain rule applies when the first link has type `bool`, ignoring cv-qualifiers. The first link is resolved first, as today, by the built-in operator or by overload resolution (OVL-7). The type of the first link then decides the meaning of the whole expression.

| First link | Meaning | Example |
|---|---|---|
| a built-in comparison | chain (X16:new-1) | `1 < x < 10` |
| an operator function or rewritten candidate that returns `bool` | chain (X16:new-1) | `s1 < s2 < s3` for a string class |
| an operator function that returns another type | today's grouping, unchanged | an expression-template or test-macro class whose `<` returns a class |

The type test settles D5 without a new rule in overload resolution:
- A rewritten candidate `(x <=> y) < 0` has type `bool` (OVL-13), so a class with only `<=>` chains.
- A class that converts to `int` reaches the built-in candidate (OVL-9), which gives `bool`, so that class chains too.
- A library whose `<` returns a class keeps today's meaning.

Section 4.5 shows that GCC's warning already uses this boundary.

If the first link is type-dependent (DEP-2), the decision waits for the specialization. One template can then chain for `int` and keep today's grouping for a class whose `<` returns a class.

### 1.3 Meaning of a chain (X16:new-1)

A chain `e0 op1 e1 op2 e2 ... opn en` has the value of `e0 op1 e1 && e1 op2 e2 && ... && e(n-1) opn en`. These details apply:
- The `&&` is the built-in operator. No overloaded `&&` is a candidate.
- Each link after the first is an operand of that `&&`. So the link converts to `bool` by contextual conversion (LOG-2). A link that cannot convert makes the program not valid (LOG-3).
- The links are evaluated from left to right. A link is evaluated completely before the next one starts (LOG-5).
- If a link is `false`, the later links and their operands are not evaluated (LOG-4). In constant evaluation, an operation in a link that is not evaluated has no effect (LOG-7).
- Each operand is evaluated at most once. X16:new-3 says how two links share a middle operand.
- The result is a `bool` prvalue.

### 1.4 One direction (new rule X16:new-2)

In a chain under X16:new-1, every operator points one way. `<` and `<=` can be mixed, and `>` and `>=` can be mixed. A chain with an operator from each group is not valid.

`a < b > c` says nothing about how `a` and `c` relate, and a reader takes it for a range test. Today the expression is valid with a different meaning. If P4 kept it valid with that meaning, a silent trap would sit beside the new meaning. The existing test `chained-mixed-relational-warns.cpp` already asks for a diagnostic under `-Werror`. X16:new-2 makes that diagnostic a rejection.

X16:new-2 does not apply when the first link is not `bool`. A test-macro class that writes `Capture() <= a > b` keeps today's grouping, as in 1.2.

### 1.5 The middle operand (new rule X16:new-3; amended SEQ-11)

Each operand other than the first and the last is used by two links. D11 asked the proposal to define this use. SEQ-11 now states the general rule ("each use refers to that one result"). X16:new-3 adds a row to SEQ-11 and gives the details that a chain needs:
- The operand is evaluated once, as part of the first link that uses it.
- The first link uses the operand with its own type and value category, as today's `a < b` does.
- The second link uses the same result. A glvalue result gives the second link an lvalue for the same object. A prvalue of non-class type gives the same value.
- A prvalue of class type is materialized once, before the first link. The temporary lives to the end of the full expression (VC-6). The first link sees the temporary as an xvalue, and the second link sees it as an lvalue.
- Each link applies its own conversions to the shared result. A conversion function of the middle operand can run once for each link (SEQ-9).
- If the result is a glvalue of scalar type, each link reads the object. A `volatile` middle operand is read twice.

The second link sees an lvalue, so an xvalue middle operand such as `std::move(s)` is not moved from twice. A range-based `for` uses its range through a hidden reference in the same way (RFOR-3, SEQ-11).

### 1.6 What does not change

- **Equality operators.** `a == b == c` and `a < b == c` keep today's grouping (D10). Comparing two `bool` results for equality is a real idiom. `a < b == c` is an equality expression whose left operand is a comparison.
- **`<=>`.** `a <=> b <=> c` is not a chain. In `(x <=> y) < 0`, the left operand of `<` is a three-way comparison, so the expression has one relational operator.
- **Parentheses.** `(a < b) < c` has one relational operator at its level and keeps today's meaning. Under P4, a program writes today's meaning this way.
- **Fold expressions.** A fold over `<` keeps the grouping that PRIM-P6 shows, with each nested part in parentheses (amended PRIM-27).
- **Other first links.** A chain whose first link has a type other than `bool` keeps today's grouping (1.2).
- **Preprocessing.** Tidal does not specify `#if` expressions, and P4 makes no change there.

### 1.7 Alternatives considered

- **A chain for every operand type, decided by the syntax alone.** This is simpler to state. But for a class whose `<` returns a class, the chain would then need `b < c` and a conversion to `bool`. Libraries use such classes for expression templates and for test macros that capture both operands (X16-6). Rejected.
- **Built-in operands only.** A string class or a duration class with an operator that returns `bool` would keep the old meaning. `s1 < s2 < s3` would then differ from `1 < 2 < 3`. Rejected.
- **"Converts to `bool`" in place of "has type `bool`".** A capture class with an `explicit operator bool` would then chain, and the test-macro pattern would break. Rejected.
- **Mixed directions with a meaning,** with `a < b > c` meaning `a < b && b > c`. That meaning is well defined, but readers misread it, and the form is a likely typo for a range test. Rejected in favor of X16:new-2.
- **Mixed directions keep today's meaning.** Two expressions that look alike would follow different rules. Rejected.
- **Equality in chains** (D10). `x == y == flag` compares two truth values today, and P4 would change that. Rejected.
- **Reject every chain,** making CMP-14 a rejection. This is the smallest change. Clang 22 already comes close for built-in operands (Q-0029). But the change gives chains no new meaning. Kept as a possible first step if P4 is staged.
- **Read a scalar middle operand once** and give the second link the value. `f() < x < g()` would then not depend on whether `g` writes `x`. But the second link would need a new kind of result, a value taken from a glvalue, which no construct in SEQ-11 has. Class operands could not follow it. Rejected.
- **The second link keeps the original value category.** An xvalue middle operand would then be moved from twice. Rejected.
- **Folds over relational operators as chains,** so that `(... < p)` tests whether a pack is in ascending order. The feature is useful. But PRIM-P6 shows the parentheses, and a right fold nests the other way. Left for later.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome or cited rule changes. New rules use this exercise's ID space.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| X16:new-1 | new | none | required | Defines a comparison chain and its links. If the first link has type `bool`, the chain means the built-in `&&` of its links, from left to right, with short-circuit evaluation (1.2, 1.3). | `expressions/comparison/chained-comparison.cpp`: accept becomes reject, because `3 > 2 > 1` is now `true` |
| X16:new-2 | new | none | reject | In a chain under X16:new-1, all operators point one way (1.4). | `expressions/comparison/chained-mixed-relational-warns.cpp`: stays reject, now under X16:new-2 with no warning flags |
| X16:new-3 | new | none | required | A middle operand is evaluated once and shared by its two links. The second link sees an lvalue, or the same value for a non-class prvalue. A class prvalue is materialized once (1.5). | none |
| CMP-P1 | amended | principle | principle | "So `a < b < c` compares a `bool` with `c`" becomes the chain meaning. Parentheses give today's meaning. | none |
| CMP-1 | amended | required | required | Precedence and left association stay. The last sentence points to X16:new-1 for a chain. | `chained-comparison.cpp` (above) |
| CMP-P6 | amended | principle | principle | The list item "a chained comparison" becomes "a relational comparison as the left operand of an equality operator". | none |
| CMP-14 | amended | recommended | recommended | An implementation should warn if the left operand of `==` or `!=` is a relational comparison or a chain without parentheses, as in `a < b == c`. A same-direction chain no longer gets a warning. | `chained-comparison-warns.cpp` and `chained-less-equal-warns.cpp`: reject becomes accept |
| Examples (comparison.md) | amended | segment | segment | `3 > 2 > 1` becomes `true`. A mixed chain and a parenthesized chain are added. | none |
| Evidence (comparison.md) | amended | segment | segment | The CMP-1 and CMP-14 items describe today's compilers against the new rules (X16-1, X16-3). | none |
| SEQ-P1 | amended | principle | principle | The list of constructs with a fixed order gains "a comparison chain, link by link". | none |
| SEQ-11 | amended | required | required | A row for a chain's middle operand, which two links use. A sentence on the value category of the second use (X16:new-3). | none |
| SEQ-8 | amended | unspecified | unspecified | The list of rules that fix an order gains X16:new-1. The operands within one link stay unordered. | none |
| OPR-11 | amended | required | required | An overloaded operator keeps precedence and associativity, except that X16:new-1 regroups a chain whose first link is `bool`. | none |
| PRIM-27 | amended | required | required | Each nested part of a fold's expansion acts as if it were in parentheses. So a fold over a relational operator is not a chain (X16-2). | none |
| Evidence (translation.md) | amended | segment | segment | The CMP-14 row of the warning-flag table and the list of errors with no flag follow the new CMP-14 (X16-1, X16-2). | none |
| CMP-2 | unchanged, state effect | required | required | Each built-in link gives `bool`, so a chain of built-in links always chains. | none |
| CMP-9 | unchanged, state effect | reject | reject | Rejects a second link of a pointer and an integer, though today's `bool < 1` was valid (4.4). | none |
| CMP-23 | unchanged, state effect | reject | reject | Rejects a second link of a scoped enumeration and an integer (4.4). | none |
| LOG-P2 | unchanged, state effect | principle | principle | A chain follows the diagram for the built-in `&&`. | none |
| LOG-2 | unchanged, state effect | required | required | Each later link converts to `bool` by contextual conversion. | none |
| LOG-3 | unchanged, state effect | reject | reject | A later link that cannot convert to `bool` makes the chain not valid. | none |
| LOG-4 | unchanged, state effect | required | required | A `false` link skips the later links. | none |
| LOG-5 | unchanged, state effect | required | required | A link finishes before the next one starts. The temporaries of a link live to the end of the full expression. | none |
| LOG-7 | unchanged, state effect | required | required | A skipped link does not affect constant evaluation (4.1). | none |
| LOG-14 | unchanged, state effect | reject | reject | An evaluated link with an operation that constant evaluation does not permit makes the expression not constant. | none |
| SEQ-4 | unchanged, state effect | undefined | undefined | Some unsequenced pairs across links become sequenced (4.2). Within one link the rule applies as today. | none |
| SEQ-9 | unchanged, state effect | required | required | A conversion function of a middle operand can run once for each link. | none |
| VC-6 | unchanged, state effect | required | required | A materialized middle operand lives to the end of the full expression. | none |
| OVL-7 | unchanged, state effect | required | required | Each link has its own candidate set. | none |
| OVL-9 | unchanged, state effect | required | required | A class that converts to an arithmetic type reaches a built-in candidate, whose result is `bool`. | none |
| OVL-11 | unchanged, state effect | reject | reject | The rules of the built-in operator apply to each link separately. | none |
| OVL-13 | unchanged, state effect | required | required | A rewritten link gives `bool`, so a class with only `<=>` chains. The inner `(x <=> y) < 0` is not a chain. | none |
| DCMP-6 | unchanged, state effect | required | required | Rewritten and reversed candidates apply to each link. | none |
| DCMP-12 | unchanged, state effect | reject | reject | Not affected. X16:new-1 follows its `bool` test and also ignores cv-qualifiers. | none |
| OPR-6 | unchanged, state effect | required | required | Each link selects its own operator function. | none |
| OPR-7 | unchanged, state effect | required | required | A chain fixes the order between links. No overloaded relational operator fixes an order today. | none |
| CPT-26 | unchanged, state effect | required | required | A dependent requirement that holds today can stop holding (4.3). | none |
| CPT-27 | unchanged, state effect | reject | reject | A non-dependent requirement that becomes invalid makes the program not valid (4.4). | none |
| UN-18 | unchanged, state effect | required | required | `noexcept` of a chain looks at every link, also a link that the short-circuit would skip. | none |
| DTY-2 | unchanged, state effect | required | required | `decltype` of a chain is `bool` (4.3). | none |
| DEP-2 | unchanged, state effect | required | required | The chain decision for a type-dependent first link waits for the specialization. | none |

Counts: 41 changes. 3 new, 12 amended (3 of them Examples or Evidence segments), 26 unchanged but must state their effect. 0 withdrawn, 0 split.

These existing tests change their outcome:
- `expressions/comparison/chained-comparison.cpp` (CMP-1). `static_assert(3 > 2 > 1 == false)` now fails, so accept becomes reject. Suggested rewrite: `static_assert(3 > 2 > 1);`, accept, cited by X16:new-1. The `clang-22` and `clang-host` lines then go, because they record Clang 22's rejection through its warning.
- `expressions/comparison/chained-comparison-warns.cpp` and `chained-less-equal-warns.cpp` (CMP-14). Reject becomes accept. Each is a same-direction chain of `int`, which gets no warning under the amended CMP-14. Today's compilers still warn, so a P4 implementation passes these tests and today's compilers fail them.

One test keeps its outcome under a different rule:
- `expressions/comparison/chained-mixed-relational-warns.cpp` stays reject, cited by X16:new-2. The `-Werror -Wparentheses` flags and the `clang-18` line go.

These tests keep both their outcome and their rule:
- `expressions/comparison/grouping.cpp`. `1 < 2 == 3 > 4` and `1 == 2 != 0` have no relational chain.
- `expressions/comparison/chained-parenthesized-no-warning.cpp`, with parentheses at each level.
- `expressions/sequencing/comparison-operand-order.cpp`, `classes/operators/associativity-kept.cpp`, `expressions/primary/fold-expansion.cpp`, and the two SEQ-11 tests.

The scans in 4.5 find no other relational chain in the test suite.

The proposal needs these new tests.

Under `tests/expressions/comparison/`:
- `chain-values.cpp` (accept). `1 < 2 < 3`, `3 > 2 > 1`, `1 <= 1 < 2` and `-5 < -3 < -1` are `true`, and `1 < 5 < 3` is `false`.
- `chain-short-circuit.cpp` (run). A `false` first link skips a call in the last operand.
- `chain-middle-once.cpp` (run). A call in the middle operand runs once. The conversion function of a class middle operand runs once for each link.
- `chain-middle-xvalue.cpp` (run). `a < std::move(s) < c` moves from `s` at most once.
- `chain-constant-skips.cpp` (accept). `0 < d < 10 / d` is a constant for `d == 0`.
- `chain-mixed-direction.cpp` (reject; X16:new-2).
- `chain-second-link-invalid.cpp` (reject). `p < q < 1` for pointers `p` and `q`.
- `chain-link-not-bool.cpp` (reject). A later link returns a class with no conversion to `bool`.
- `chain-class-bool.cpp` (run). A class whose `operator<` returns `bool`.
- `chain-class-rewritten.cpp` (accept). A class with only `<=>`.
- `chain-class-result-kept.cpp` (run). A capture class whose `<=` returns a class keeps today's grouping, with mixed directions too.
- `chain-template.cpp` (accept). One function template chains for `int` and keeps today's grouping for a capture class.
- `chain-requires.cpp` (accept). A concept that tests `x < y < 1` is `false` for a scoped enumeration.
- `chain-type-noexcept.cpp` (accept). `decltype` of a chain is `bool`, and `noexcept` looks at every link.
- `chain-equality-unchanged.cpp` (accept). `a == b == c` and `a < b == c` keep today's grouping.
- `chain-fold-unchanged.cpp` (accept). `(... < p)` keeps today's grouping.
- `equality-of-relational-warns.cpp` (reject with `-Werror -Wparentheses`; the amended CMP-14).

Under `tests/expressions/sequencing/`:
- `chain-links-sequenced.cpp` (run). `i < 0 < i++` is defined and gives `false` for `i == 1`.

## 3. Interactions

**Comparison (CMP-P1, CMP-1, CMP-2, CMP-P6, CMP-9, CMP-14, CMP-23).** CMP-1's precedence and left association stay for everything except a chain whose first link is `bool`. CMP-14 loses its old target and takes `a < b == c`, which looks like a chain under P4 and is not one. Today GCC warns about that form under `-Wall`, and Clang 22 does not (probe `e1.cpp`). Q-0029 asks whether `3 > 2 > 1` is valid. Under P4 the expression is valid with the new meaning, and the answer to Q-0029 no longer decides a test.

**Logical operators (LOG-P2, LOG-2 to LOG-5, LOG-7, LOG-14).** X16:new-1 uses the built-in `&&`. So short-circuit evaluation, sequencing, temporaries and constant evaluation all come from the logical page with no new text.

**Evaluation order (SEQ-P1, SEQ-4, SEQ-8, SEQ-9, SEQ-11).** Today the three operands of `a < b < c` are unsequenced with each other (SEQ-8). Under P4 the first link is sequenced before the second. Within the second link, the read of a glvalue middle operand and the evaluation of the last operand stay unsequenced. So `a < x < x++` stays undefined. SEQ-11 already has the "one result, used twice" wording that D11 asked for, and P4 adds a row. The class prvalue case is new to SEQ-11. Each construct in its table today uses a glvalue, a scalar, or a hidden variable.

**Overload resolution (OVL-7, OVL-9, OVL-11, OVL-13, DCMP-6, DCMP-12).** P4 adds no step to overload resolution. Each link is resolved as a separate expression. The only new input is the type of the first link's result, which is read after resolution. Built-in and rewritten candidates both give `bool`, so both chain.

**Operators (OPR-6, OPR-7, OPR-11).** OPR-11 gains its first exception. OPR-7's list of operators with a fixed order does not change, because the order comes from the built-in `&&`.

**Templates and constraints (DEP-2, CPT-26, CPT-27).** A chain in a template is decided for each specialization. A requires expression sees the new validity (4.3, 4.4).

**Unevaluated operands (UN-18, DTY-2).** `decltype` and `noexcept` see the new expression (4.3).

**Fold expressions (PRIM-P6, PRIM-27).** PRIM-P6 shows a fold's expansion with parentheses. PRIM-27 does not say so (X16-2). P4 needs those parentheses, so that folds keep their meaning.

**Values (VC-P3, VC-6).** A materialized middle operand has the usual end of life. A skipped link creates no temporaries, so the set of destructor calls can change (4.3).

**Translation (the warning-flag table).** The CMP-14 row and the list of errors that Clang 22 gives with no flag change with CMP-14.

**Not touched.** The grammar, `<=>`, the equality operators, parenthesized comparisons, folds, and chains whose first link is not `bool`.

## 4. Impact on existing programs

Tidal has no corpus of real programs. The claims below come from the rules, from compiler runs, and from scans of the test suite.

### 4.1 Rejected becomes valid

- **Valid links, invalid outer comparison.** In these same-direction chains, today's outer comparison is not valid but each link is. Both compilers reject each one today (probes `j1.cpp` to `j3.cpp`):
  - pointers, `x < x + 1 < x + 2`, which compares a `bool` with a pointer (CMP-9),
  - a scoped enumeration, `E::a < E::b < E::c` (CMP-23),
  - a class with only `operator<(S, S)`, `a < b < c`.
- **A skipped forbidden operation.** Today a constant evaluation can evaluate a forbidden operation in a link that P4 skips. For `d == 0`, both compilers reject `0 < d < 10 / d` today, because `10 / 0` is evaluated (probe `c1.cpp`). Under P4 the first link is `false`, and the expression is the constant `false`.
- **Clang 22 diagnostics.** Today Clang 22 rejects, with no flag, every same-direction chain of built-in operands outside a dependent context (Q-0029). The rejection is a diagnostic choice, and under P4 the warning has no reason to exist.

### 4.2 Undefined becomes defined

- **An unsequenced pair across links.** `i < 0 < i++` reads `i` in the first link and writes `i` in the last operand. Today the read and the write are unsequenced, which is undefined (SEQ-4). Both compilers warn (probe `u1.cpp`). Under P4 the first link is sequenced before the second link, so the expression is defined.
- **Undefined behavior in a skipped link.** `a < b < 10 / d` with `a >= b` and `d == 0` divides by zero today. Under P4 the division is not evaluated.

### 4.3 Valid programs that change meaning

Every valid chain whose first link has type `bool` changes meaning. The table lists the ways a valid program can see the change.

| Channel | Today | Under P4 | Probe |
|---|---|---|---|
| Run-time value | `(a < b) < c`, a `0` or `1` compared with `c` | `a < b && b < c` | `r1.cpp`: `3 < 2 < 5`, `3 > 2 > 1` and `-5 < -3 < -1` each flip |
| Side effects of the last operand | always evaluated | not evaluated after a `false` link | `r1.cpp`: `0 < -1 < f()` calls `f` today |
| Temporaries and destructors of the last operand | always created | not created after a `false` link | by LOG-5 |
| Which comparison runs second | the `bool` compared with `c` | `b` compared with `c` | `c2.cpp`, `w1.cpp` `f2` |
| Rewritten candidates | `R{3} < R{2} < R{1}` is `false < R{1}`, so `R(0) < R(1)` through the converting constructor, `true` | `false` | `k1.cpp` |
| Conversion functions and `volatile` reads of the middle operand | once | once for each link | by X16:new-3 |
| A class prvalue in the middle | initializes the first link's by-value parameter directly | materialized first, then an xvalue, so a by-value parameter can add a move | by X16:new-3 |
| `static_assert` and other constants | `3 > 2 > 1` is `false` | `true` | `chained-comparison.cpp` |
| Constant to not constant | `static_assert(B{1} < B{2} < 2)` holds, since `true < 2` is built-in | the second link calls the non-`constexpr` `operator<(B, int)`, so the program is rejected | `c2.cpp` |
| Requires expressions and concepts | `requires(E x, E y) { x < y < 1; }` holds for a scoped enumeration and for `int*` | does not hold, since `y < 1` is not valid | `s1.cpp`, `s2.cpp` |
| Branches chosen by such tests | `if constexpr (requires ...)` and constrained overloads take the "valid" branch | take the other branch, with no diagnostic | `s2.cpp` `pick` |
| `decltype` and `auto` | `decltype(1 < 2 < C{})` is `X`, the result of `operator<(int, C)` | `bool`, or not valid if `X` does not convert to `bool` | `s1.cpp` |
| `noexcept` | `noexcept(b1 < b2 < b3)` is `false`, through `operator<(bool, B)` | `true` if `operator<(B, B)` is `noexcept` | `s1.cpp` |
| Order of evaluation | the three operands are unsequenced | the first link before the second | by LOG-5 |

These forms keep their meaning:
- equality chains,
- `<=>`,
- parenthesized comparisons,
- folds (`f1.cpp`),
- chains whose first link returns a class (`w1.cpp` `f3` to `f5`).

The detection channels need the most care. A requires expression in a dependent context changes its answer with no diagnostic on either compiler. Neither compiler warns about a chain in a dependent requirement today (probe `s2.cpp`, the concept `Chain` and the function `pick`). So no warning flag finds the code whose meaning P4 changes there. Only a search of the source for chains can find it.

### 4.4 Valid programs that become invalid

- A mixed-direction chain whose first link is `bool`, such as `left < middle > right` (X16:new-2).
- A chain whose second link is not valid although today's comparison of the `bool` with `c` is valid. `p < q < 1` for pointers (CMP-9) and `e1 < e2 < 1` for a scoped enumeration (CMP-23) are examples.
- A later link whose result does not convert to `bool` (LOG-3). Today `1 < 2 < C{}` is valid with type `X` (probe `s1.cpp`).
- A constant expression whose new link is not constant (probe `c2.cpp`).
- A non-dependent requirement that now forms an invalid expression (CPT-27). Outside a template, `requires(E x, E y) { x < y < 1; }` is valid today, and GCC accepts it. Clang 22 rejects it today through its warning (probe `s2.cpp`, `q1`).

### 4.5 Evidence

**Test suite.**
- Both compilers' chain warnings were collected over all 6335 test files (`scan.out`, `scang.out`). Each compiler flags the same four files: `chained-comparison.cpp`, `chained-comparison-warns.cpp`, `chained-less-equal-warns.cpp` and `chained-mixed-relational-warns.cpp`.
- The token scan (`tok.out`) finds 2281 segments with two relational tokens at one level. 251 segments have unbalanced `<` and `>`, or contain `<=` or `>=` (`tok2.out`), and I checked each one by eye. I checked the balanced segments with a pattern for `x < y > z`. Outside the four files, each segment is a template argument list, a cast, or an operator declaration. An example is `Num<3 > 2>` in `templates/arguments/greater-ends-list.cpp`, which tests TARG-4.
- No test folds over a relational operator. The only fold over a comparison is `(V <=> ...)` in `fold-three-way-operator.cpp`.

**Today's diagnostics for a chain.** The table shows which changed programs compile silently today. With no flag, GCC gives nothing for every row. `-Wno-error=parentheses` turns each Clang error into a warning (Q-0029). For the rows marked "nothing", Clang 22 also gives nothing under `-Wall` (probes `w1.cpp`, `k1.cpp`, `c2.cpp`, `d1.cpp`, `s2.cpp`).

| Form | Clang 22, no flag | GCC 16, `-Wall` | Probe |
|---|---|---|---|
| Built-in operands, not dependent | error | warning | `r1.cpp`, `d2.cpp` |
| Through a macro, `LESS(3, 2) < 1` | error | warning | `d2.cpp` |
| A template or a generic lambda, instantiated with `int` | error | warning | `d2.cpp`, `e1.cpp` |
| A template that is never instantiated | nothing | nothing | `d1.cpp` |
| Class operands, first link returns `bool` (an operator or a rewritten `<=>`) | nothing | warning | `w1.cpp` `f2`, `k1.cpp`, `c2.cpp` |
| Class operands, first link returns a class | nothing | nothing | `w1.cpp` `f3` to `f5` |
| A requires expression, not dependent | error | nothing | `s2.cpp` `q1` |
| A requires expression in a concept or in `if constexpr` | nothing | nothing | `s2.cpp` |
| `decltype` and `noexcept` operands, built-in | error | warning | `s2.cpp` |
| A fold over `<` (P4 does not change it) | error | nothing | `d2.cpp`, `f1.cpp` |

The roadmap gives Clang 22's rejection as the impact evidence ("on one matrix compiler the changed programs do not compile today"). The table limits that claim. Clang 22 rejects the built-in forms. It accepts every chain with class operands and every chain in a dependent context. GCC's warning under `-Wall` matches the boundary of X16:new-1. It fires when the first link is `bool`, and not when the first link returns a class (`w1.cpp`).

**Other probes.**
- `r1.cpp`: both compilers print `1 0 1 calls=1 0 0`. `3 < 2 < 5` is `1`, `3 > 2 > 1` is `0`, and `0 < -1 < f()` is `1` and calls `f`. `1 < 2 < 0.5` is `0` today and under P4. `-5 < -3 < -1` is `0`.
- `c1.cpp`: both compilers reject `constexpr bool v = f(0);`, where `f` returns `0 < d < 10 / d`.
- `c2.cpp`: both compilers accept `static_assert(B{1} < B{2} < 2)`, with a non-`constexpr` `operator<(B, int)` declared. Clang gives no warning.
- `s1.cpp`: both compilers accept every assertion. These include `chain_ok<E, int>`, `chain_ok<int*, int>`, `decltype(1 < 2 < C{})` as `X`, and `!noexcept(b1 < b2 < b3)`.
- `k1.cpp`: both compilers accept `static_assert(R{3} < R{2} < R{1})` for a class with a defaulted `<=>` and a converting constructor from `int`. The run prints `1` for `R{1} < R{2} < R{3}`.
- `f1.cpp`: on both compilers, `(... < t)` for `3, 2, 1` is `true`, and `(t < ...)` for `1, 2, 3` is `false`.
- `j1.cpp` to `j3.cpp`: both compilers reject the three forms of 4.1.
- `e1.cpp`: under `-Wall`, GCC warns about `a < b == c` and `a == b == c` ("suggest parentheses around comparison in operand of '=='"). Clang 22 does not.
- `u1.cpp`: GCC warns "operation on 'i' may be undefined" for `i < 0 < i++`, and Clang warns "unsequenced modification and access to 'i'".

## 5. Findings

**X16-1** (unclear; expressions/comparison.md, foundations/translation.md; CMP-14, CMP-1 Evidence, the warning-flag table). CMP-14 defines a chained comparison as one whose left operand is "a relational comparison without parentheses". The rule does not say whether a comparison through an operator function counts, and the compilers differ:
- Under `-Wall`, GCC warns when the inner comparison is an operator function or rewritten candidate that returns `bool`. GCC does not warn when the inner comparison returns a class.
- Clang 22 gives no warning for any class operand, even under `-Wall` (probes `w1.cpp`, `k1.cpp`, `c2.cpp`).

The CMP-14 tests use `int` only. The translation page lists "the chained comparison of CMP-14" among the errors that Clang 22 gives with no flag. That claim holds only for built-in operands. Suggestion:
- CMP-14 says which inner comparisons count.
- Add a test with a class whose `<` returns `bool`, where GCC rejects under `-Werror -Wparentheses` and Clang accepts.
- Add a test with a class whose `<` returns a class, which both compilers accept.
- The translation list then says "with built-in operands".

**X16-2** (missing; expressions/primary.md, expressions/comparison.md; PRIM-27, PRIM-P6, CMP-14). PRIM-P6 shows a fold's expansion with parentheses, as in `(p1 - p2) - p3`. PRIM-27 says only that a fold "nests to the left" or "to the right". So a reader cannot tell whether `(... < p)` is a chained comparison under CMP-14. Clang 22 treats the fold as a chain and rejects it with no flag ("comparison in fold expression would evaluate to '(X < Y) < Z'"). GCC gives no warning, even under `-Wall` (probes `d2.cpp`, `f1.cpp`). No test folds over a relational operator. Neither Q-0029 nor the translation page lists this rejection. Suggestion: PRIM-27 says that each nested part acts as if it were in parentheses. CMP-14 says whether a fold counts. Add a fold test with a `clang-22` reject line, and a row in Q-0029.

**X16-3** (missing; expressions/comparison.md, Q-0029; CMP-1 and CMP-14 Evidence). The Evidence says that a chain of variables and a chain of constants "give the same result on each compiler". The Evidence does not cover other contexts, where the compilers differ (the table in 4.5):
- Clang 22 rejects a chain in a non-dependent requires expression. GCC gives no warning there, even under `-Wall`.
- Neither compiler diagnoses a chain in a dependent requirement or in a template that is never instantiated.

Suggestion: add these rows to the CMP-14 Evidence table. Add a test of a non-dependent requires expression (`clang-22` reject) and a test of a chain in a concept (accept, with no warning on either compiler).

**X16-4** (needs example; classes/operators.md, overloading/overload-resolution.md; OPR-11, OVL-13). The only associativity test for OPR-11 uses `-`. No test shows that a relational chain of class operands groups as `(a < b) < c`. No test shows that the outer comparison then reaches an operator function through the `bool`. Probe `k1.cpp` uses a class `R` with a defaulted `<=>` and a converting constructor from `int`. On both compilers, `R{3} < R{2} < R{1}` is `true`. The `false` from the first link converts to `R(0)`, and a rewritten candidate compares `R(0)` with `R(1)`. This case combines OPR-11, the conversion sequences and OVL-13, and no test pins the combination. P4 changes this case the most. Suggestion: a run test, `classes/operators/relational-chain-class.cpp`, with one class that has an `operator<` and one that has only a defaulted `<=>`.

**X16-5** (missing; expressions/comparison.md, Q-0029; CMP-14 Evidence). The CMP-14 Evidence table has a column "Equality chain, `-Werror -Wparentheses`" with results for four compilers. No test file stands behind the column. `grouping.cpp` has equality chains but no flags. GCC's warning for `a < b == c` is a different warning from its chain warning: "suggest parentheses around comparison in operand of '=='" (probe `e1.cpp`). The table does not say which of the two equality forms each result covers. Suggestion: two tests, `a == b == c` and `a < b == c`. Each test passes `-Werror -Wparentheses` and has a `gcc` reject line. Give each form its own row in the table.

**X16-6** (outside knowledge). These points come from general experience:
- Other languages give chains the meaning that P4 proposes, with the middle operand evaluated once.
- Test frameworks write an assertion macro as a capture object followed by `<=` and the user's expression. `Capture() <= a < b` is then a relational chain whose first link returns a class, and `Capture() <= a > b` is a mixed-direction chain. Both must keep today's meaning. This need sets the condition "the first link has type `bool`".
- Expression-template libraries return a class from `<`.
- C keeps today's meaning. A function body in a header that C shares would change meaning in C++ only.

**X16-7** (outside the spec; Q-0007, roadmap-4 item 5). Three needs fall outside the specification:
- **Version marking.** P4 changes the meaning of valid C++23 programs. Tidal has no field that marks a rule as changing what a valid program does, as distinct from adding new programs.
- **A corpus.** Tidal cannot measure the impact on real programs. The test suite holds four chains, and each one was written to test chains.
- **`#if` expressions.** Tidal does not specify the preprocessor. So P4 cannot say whether `#if A < B < C` changes.
