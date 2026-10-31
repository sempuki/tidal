# X1: P3, left-to-right evaluation everywhere

Role: clean-room reader (language designer persona, decision 0037).
Exercise: Propose X1. Inputs: Tidal's `spec/`, `docs/`, `tests/`,
`questions/`, and `reports/clean-room/roadmap/designer.md`, plus runs of
the host `clang++` and `g++` with `-std=c++23 -pedantic-errors` at `-O0`
and `-O2`.

## Summary

- 30 rule changes: 19 amended, 4 reclassified, 6 withdrawn, 1 new.
- 18 tests affected. 6 change their expected outcome, 9 must be
  tightened from "either order" to one order, and 3 need only new rule
  tags. Five of the tightened tests fail on GCC today.
- 17 findings. The biggest: SEQ-8 read literally contradicts SHF-8,
  LOG-5, and COMMA-1 (X1-1); six constructs have no stated order at all
  (X1-3 to X1-8); constant evaluation does not behave as CE-P3 and CE-9
  say for order (X1-9, X1-10).

## 1. Design

### 1.1 The rule

Within a full expression, every subexpression is evaluated in the order
it is written, from left to right. Each one completes, with all its side
effects, before the next one starts. This covers:

- the two operands of every binary operator,
- the arguments of a call, after the expression that names the function
  (SEQ-2 stays),
- the values of a braced or parenthesized initializer,
- the captures of a lambda,
- the parts of a `new` expression.

Two kinds of operator keep their own order.

- Assignment and compound assignment evaluate the right operand first,
  as ASN-4 says today.
- `&&`, `||`, and `?:` keep their conditional evaluation (LOG-5 and the
  conditional page). They already go left to right.

With this rule, no two evaluations in one thread are unsequenced. SEQ-4
has nothing left to apply to, so the undefined behavior it gives goes
away.

```cpp
f(a(), b());        // a() then b()
x * y();            // x is read, then y() runs
int i = 1;
int r = i++ + i++;  // r == 3, i == 3 (undefined today)
left() = right();   // unchanged: right() first
```

### 1.2 Order follows the written expression

The order depends only on how the expression is written. It does not
depend on which function overload resolution picks.

- A member operator and a non-member operator give the same order: the
  left operand first. For a member operator, the left operand is the
  object.
- A rewritten or reversed comparison (DCMP-6) evaluates `x` before `y`
  in `x == y`, even when the chosen function takes `y` first.
- An overloaded operator in the assignment family keeps right-first
  order (SEQ-6, OPR-7).

```cpp
struct A {}; struct B {};
bool operator==(B, A);
mkA() == mkB();     // mkA() first, then the reversed call
```

### 1.3 Calls

- Explicit arguments go from left to right. Default arguments follow,
  in parameter order (DEFARG-3).
- A pack expansion in an argument list follows the order of its
  elements. Tidal does not cover packs (X1-13).
- `a[x, y]` for an `operator[]` with several parameters evaluates `a`,
  then `x`, then `y` (OPR-3).
- Parameter objects are created in argument order. They end in the
  reverse order. The point at which they end stays the
  implementation's choice (FN-10, VC-10).

```cpp
int g(int, int = d());
g(h());             // h() then d()
```

### 1.4 `new`

The parts run in this order: placement arguments, the array bound, the
allocation function, then the initializer arguments (NEW-10, NEW-12).
Both host compilers today evaluate the bound before the placement
arguments, so this choice changes both. Section 4.1 gives the
alternative of keeping the bound first.

```cpp
new (p()) T[n()];   // p(), n(), allocation, then the elements
```

### 1.5 Constant evaluation

Constant evaluation uses the same order as a run. A constant that
depends on order has one value on every implementation. This makes
CE-P3's promise of one answer true for order.

### 1.6 What stays

- The as-if rule (TR-P5). An implementation can still reorder work
  that the observable behavior does not depend on.
- The order of dynamic initialization across translation units
  (SINIT-3).
- When parameter objects end (FN-10, VC-10).
- The unspecified results of CMP-8 and the closure layout of LAM-20.
- Threads, which Tidal does not cover.

## 2. Rule changes

"Tighten" means the test accepts either order today and must require
the left-to-right order. "Retag" means the test keeps its assertion and
cites the new rule.

| ID | Change | Old class | New class | Description | Tests whose expectation changes |
|---|---|---|---|---|---|
| SEQ-P1 | amended | principle | principle | The list of fixed orders becomes one general order, written order, with assignment as the exception. | none |
| SEQ-P2 | withdrawn | principle | none | The implementation no longer chooses an order. The promise that two evaluations do not interleave moves into SEQ-P1. | none |
| SEQ-1 | reclassified | unspecified | required | Call arguments are evaluated from left to right, then any default arguments. | tighten `expressions/sequencing/function-arguments-order.cpp`, `expressions/sequencing/function-arguments-not-interleaved.cpp` |
| SEQ-3 | amended | required | required | Becomes the general order rule. Keeps assignment right-first. Adds parenthesized initializers and captures. | retag `expressions/sequencing/compound-assignment-order.cpp` |
| SEQ-8 | reclassified | unspecified | required | The two operands of a built-in binary operator are evaluated left first. | tighten `expressions/sequencing/comparison-operand-order.cpp`, `expressions/sequencing/bitwise-operand-order.cpp`, `expressions/sequencing/multiplicative-operand-order.cpp`; retag `expressions/sequencing/calls-not-interleaved.cpp` |
| SEQ-4 | withdrawn | undefined | none | No unsequenced pair remains, so no access is undefined for that reason. | `expressions/sequencing/unsequenced-increments.cpp` (reject to run); `expressions/sequencing/adjacent-bitfields-unsequenced.cpp` and `expressions/sequencing/volatile-reads-unsequenced.cpp` (undefined to defined, can become run tests); retag `expressions/sequencing/unsequenced-constant-evaluation.cpp` |
| SEQ-7 | withdrawn | recommended | none | Nothing is left to warn about. | `expressions/sequencing/unsequenced-increments.cpp` |
| SEQ-6 | amended | required | required | Every overloaded operator follows the general order. The list shrinks to the assignment family as the exception. | none |
| ADD-P4 | amended | principle | principle | The operands are ordered, left first. | none |
| ADD-13 | reclassified | unspecified | required | Left operand first. | tighten `expressions/additive/operand-order.cpp` |
| MUL-P4 | amended | principle | principle | Left first. The warning about writing a scalar in one operand goes. | none |
| MUL-13 | reclassified | unspecified | required | Left operand first, built-in and overloaded. | tighten `expressions/multiplicative/operand-order.cpp`, `expressions/multiplicative/operand-order-overloaded.cpp` |
| MUL-14 | withdrawn | undefined | none | Modifying a scalar in one operand and reading it in the other is defined. | `expressions/multiplicative/unsequenced-modification.cpp` (reject to run) |
| CMP-P1 | amended | principle | principle | The operands are ordered, left first. | none |
| BIT-P1 | amended | principle | principle | The operands of a binary bitwise operator are ordered, left first. | none |
| SHF-P4 | amended | principle | principle | Left-first order is no longer an exception. | none |
| ASN-P3 | amended | principle | principle | Right-first order is now the one exception to the general order. | none |
| ASN-11 | withdrawn | unspecified | none | A call that today is unordered with a compound assignment is now ordered by its position. | `expressions/assignment/compound-indeterminate-with-call.cpp` (1 or 10 becomes 10), `expressions/assignment/compound-indeterminate-run.cpp` (same) |
| OPR-P1 | amended | principle | principle | Operands are evaluated in the general order for every operator. | none |
| OPR-7 | amended | required | required | Operator syntax evaluates in written order for every operator, member or not. The assignment family keeps right-first order. | none |
| DCMP-6 | amended | required | required | Rewritten and reversed candidates evaluate the operands in written order. | none (needs a new test) |
| DEFARG-3 | amended | required | required | Default arguments are evaluated after the explicit arguments, in parameter order. | none (needs a new test) |
| NEW-12 | amended | required | required | Placement arguments, then the bound, then allocation, then the initializer. | none (needs a new test) |
| LIST-3 | amended | required | required | The values of a parenthesized aggregate initializer are evaluated left to right, like LIST-7. | none (needs a new test) |
| LAM-7 | amended | required | required | Captures are initialized in the order written. | none (needs a new test) |
| VC-6 | amended | required | required | Text stays. Because creation order is fixed, the destruction order of a full expression's temporaries is now fixed too. The example and Evidence change. | tighten `expressions/value-categories/temporaries-reverse-order.cpp` |
| new VC rule | new | none | required | The parameter objects of one call end in the reverse of their creation order, at the point VC-10 lets the implementation choose. | none (needs a new test) |
| CE-P3 | amended | principle | principle | Evaluation order is no longer a source of disagreement. Constant evaluation uses the run-time order. | none |
| TR-P5 | amended | principle | principle | Drops SEQ-P2 as a source of freedom. The as-if freedom stays. | none |
| glossary: unsequenced | withdrawn | term | none | No evaluation is unsequenced. The term can go, or stay for threads if Tidal adds them. | none |

Counts: 30 changes. 18 tests are affected:

- Outcome changes (6): `sequencing/unsequenced-increments.cpp`,
  `multiplicative/unsequenced-modification.cpp`,
  `sequencing/adjacent-bitfields-unsequenced.cpp`,
  `sequencing/volatile-reads-unsequenced.cpp`,
  `assignment/compound-indeterminate-with-call.cpp`,
  `assignment/compound-indeterminate-run.cpp`.
- Tighten (9): `sequencing/function-arguments-order.cpp`,
  `sequencing/function-arguments-not-interleaved.cpp`,
  `sequencing/comparison-operand-order.cpp`,
  `sequencing/bitwise-operand-order.cpp`,
  `sequencing/multiplicative-operand-order.cpp`,
  `additive/operand-order.cpp`, `multiplicative/operand-order.cpp`,
  `multiplicative/operand-order-overloaded.cpp`,
  `value-categories/temporaries-reverse-order.cpp`.
- Retag only (3): `sequencing/compound-assignment-order.cpp`,
  `sequencing/calls-not-interleaved.cpp`,
  `sequencing/unsequenced-constant-evaluation.cpp`.

(All paths are under `tests/expressions/`.)

On the host GCC, five of these fail after the change:
`function-arguments-order.cpp`, `function-arguments-not-interleaved.cpp`,
`operand-order-overloaded.cpp`, `temporaries-reverse-order.cpp`, and
`compound-indeterminate-run.cpp`. Clang passes all of them.

Tests that stay as they are: the `*-order-kept.cpp` tests under
`classes/operators/`, `logical-and-order.cpp`, `no-short-circuit.cpp`,
`shift/operand-order.cpp`, `comma/left-before-right.cpp`,
`assignment/right-before-left.cpp`, `allocation-before-initializer.cpp`,
`braced-order.cpp`, `tuple-get-order.cpp`, and the parameter-lifetime
tests of FN-10 and VC-10.

Q-0005 chose "unspecified" for MUL-13 to stay compatible with
implementations. P3 reverses that choice, so Q-0005 closes with the
proposal.

## 3. Interactions

| Area | Rules | What the proposal must say |
|---|---|---|
| Built-in binary operators | SEQ-3, SEQ-8, ADD-13, MUL-13, CMP-P1, BIT-P1, SHF-8 | Left operand first for all. Shifts are unchanged. |
| Assignment | ASN-4, ASN-P3, ASN-11, SEQ-3 | Right first stays. `a[i] = i++` and `i = i++` are already defined and do not change. ASN-11 goes. |
| Logical, comma, conditional | LOG-5, LOG-11, COMMA-1, conditional page | No change. They are already left first. |
| Function calls | SEQ-1, SEQ-2, SEQ-5, PFX call rules | Callee, then arguments left to right, then defaults. A reference parameter binds at its position, so a later argument that changes the object is visible in the body. |
| Default arguments | DEFARG-3, SEQ-5, VC-9 | Evaluated after the explicit arguments, at the call, in parameter order. Their temporaries belong to the call's full expression, as now. |
| Overloaded operators | OPR-P1, OPR-1, OPR-3, OPR-6, OPR-7, SEQ-6 | Order comes from the written form, so it is the same for member, non-member, and built-in candidates. Multi-argument `operator[]` evaluates its arguments left to right (X1-16). |
| Rewritten comparisons | DCMP-6, DCMP-P2 | Written order of `x` and `y`, whatever the candidate's parameter order. Clang changes here (section 4.1). |
| Defaulted comparisons | DCMP-1, DCMP-P1 | No change. They already compare in declaration order. |
| Templates | OPR-1 (operator templates), primary page note on folds | The order is fixed before instantiation, so a dependent operator has the same order whatever it resolves to. Fold expressions and pack expansions follow the expanded order (X1-13). |
| Initialization | LIST-7, LIST-3, LIST-9, initialization forms (declarators in order) | Braced lists are unchanged. Parenthesized aggregate initialization becomes left to right (X1-6). A constructor call in parentheses is a call, so SEQ-1 covers it. |
| Lambdas | LAM-7, LAM-20 | Captures initialize in written order (X1-7). The closure's layout stays unspecified. |
| Structured bindings | SB rule on `get` order | No change. |
| `new` | NEW-10, NEW-12, UNW-3 | Order of placement arguments, bound, allocation, initializer (X1-5). UNW-3 must say which of these can throw before storage exists. |
| Object lifetime | VC-P3, VC-6, VC-9, VC-10, FN-10, OBJ-P2 | Temporaries end in the reverse of a now fixed creation order. Parameters end in reverse order (X1-8). The point at which parameters end stays impl-defined. |
| Exceptions | UNW-P1, UNW-1, UNW-2, UNW-3 | When the third argument throws, the first two have run and their temporaries end. Today which ones ran is unspecified. Tidal has no rule for temporaries during unwinding (X1-11). |
| Constant evaluation | CE-P3, CE-9, SEQ-4 table | Constants use the run-time order. Programs that are undefined today, such as `i++ + i++`, become valid constants. Both host compilers already accept them with the left-to-right value (X1-10). |
| Volatile and observable behavior | TR-P5, SEQ-4, volatile page | `v + v` reads `v` twice in a fixed order, so the volatile access order is part of the observable behavior with no exception. |
| Optimization | TR-P5 | The as-if rule still lets an implementation reorder when nothing observable depends on it. The cost falls on calls whose arguments have side effects (X1-14). |
| Bit-fields | SEQ-4 | Writes to adjacent bit-fields in two operands become ordered and defined. |
| Glossary | unsequenced | The term goes or changes meaning (X1-15). |

## 4. Impact on existing programs

### 4.1 Valid programs that change meaning

All of these programs are valid today. For most of them the two host
compilers already give different results. The `new` case gives the same
result on both, and P3 changes it on both. Results below are from runs
of the host compilers. The same results hold at `-O0` and `-O2`.

| Construct | Clang today | GCC today | P3 | Tidal support |
|---|---|---|---|---|
| `sum(a(), b())` | left first | right first | left first | SEQ-1 Evidence |
| non-member overloaded `a() + b()` | left first | right first | left first | Q-0005, `operand-order-overloaded.cpp` note |
| member overloaded `a() - b()` | left first | right first | left first | none (X1-2) |
| reversed `==` chosen for `x() == y()` | `y()` first | `x()` first | `x()` first | none (X1-3) |
| `g(h())` with a default for the second parameter | `h()` first | default first | `h()` first | none (X1-4) |
| `new (p()) int[n()]` | `n()` first | `n()` first | `p()` first | none (X1-5) |
| two by-value parameters, destruction | second ends first | first ends first | second ends first | none (X1-8) |
| temporaries `use(Note{1}, Note{2})`, destruction | `2` then `1` | `1` then `2` | `2` then `1` | `temporaries-reverse-order.cpp` accepts both |
| `take(g += 1, bump())` at run time | `g == 10` | `g == 1` | `g == 10` | ASN-11 tests accept both |
| built-in `a() < b()`, `a() & b()`, `a() * b()` | left first | left first | left first | Q-0005 for `*` |
| parenthesized aggregate `T(a(), b())` | left first | left first | left first | none (X1-6) |
| init captures `[x = a(), y = b()]` | left first | left first | left first | none (X1-7) |

So:

- Programs whose output depends on argument order or overloaded
  operand order keep their meaning on Clang and change on GCC. The
  SEQ-1 Evidence already records that these programs differ between
  two matrix compilers. P3 picks one of the two existing behaviors.
- Reversed comparisons with side effects change on Clang. A design that
  used the candidate's parameter order would instead change nothing on
  Clang and change GCC. The written order is simpler to state and keeps
  the rule of section 1.2.
- The `new` choice changes both compilers. The alternative is to keep
  the bound first and state that as a second exception. That costs one
  exception and breaks no program.
- Built-in operands, parenthesized aggregates, and init captures already
  go left to right on both compilers, so these programs keep their
  meaning everywhere observed.

Constant evaluation is a special case. For the `take(g += 1, bump())`
function, the host GCC gives 10 when it evaluates the function as a
constant and 1 when it runs the same function. Clang gives 10 both
times. Under P3, GCC's run-time value changes to match its own constant
value. No constant value changes on either compiler (X1-9).

### 4.2 Programs that become valid

These programs are undefined today under SEQ-4 or MUL-14, and P3 gives
each a meaning.

- `i++ + i++` (from 1: value 3), `i + i++` (value 2), `i++ * i`
  (value 2). At run time, the host GCC gives 3 for `i + i++`, and Clang
  gives 2.
- `(x.a = 1) + (x.b = 2)` on adjacent bit-fields
  (`adjacent-bitfields-unsequenced.cpp`).
- `v + v` on a volatile `v` (`volatile-reads-unsequenced.cpp`).
- Any constant whose evaluation contains such a pair. By CE-9 these are
  rejected today. Both host compilers accept all three expressions above
  in a `static_assert` and give the P3 values, so for constants the
  change matches what the compilers already do (X1-10).

### 4.3 Programs that become invalid

None. P3 adds no constraint and no reject rule. Two tests change from
reject to run (`unsequenced-increments.cpp`,
`multiplicative/unsequenced-modification.cpp`). They reject today only
because SEQ-7's recommended warning is made an error with `-Werror`.
Under P3 the warning has no basis. A build that keeps such warnings as
errors would still reject these programs on today's compilers until the
compilers drop the warning.

### 4.4 Implementation cost

- GCC changes its argument order, its overloaded-operand order, and its
  default-argument order. Clang changes reversed comparisons. Both
  change `new` with placement arguments and a run-time bound.
- Both constant evaluators already use left-to-right order in every
  case tried.
- Optimizers lose the freedom that TR-P5 credits to SEQ-P2 only where a
  reordering would be observable.

## 5. Findings

**X1-1.** Page: foundations/evaluation-order.md. Rules: SEQ-3, SEQ-8,
SHF-8, LOG-5, COMMA-1. Kind: contradictory. The page intro lists shifts,
`&&`, `||`, comma, and `?:` as fixed. SEQ-3 lists only subscripts,
assignment, member pointers, and braced lists. SEQ-8 says every built-in
operator that SEQ-3 does not list has unsequenced operands. Read
literally, that makes shift and comma operands unsequenced. I needed one
list of every fixed order to know what P3 leaves unchanged. Suggestion:
SEQ-8 should name the rules that fix an order (SHF-8, LOG-5, COMMA-1,
the conditional rule, SEQ-3), or SEQ-3 should list them all.

**X1-2.** Pages: foundations/evaluation-order.md, classes/operators.md.
Rules: SEQ-2, OPR-P1, OPR-7. Kind: ambiguous. SEQ-2 says the object of a
member function call is evaluated before the arguments. It does not say
if this holds for a member operator in operator syntax. The host GCC
evaluates the right operand of `a() - b()` first when `operator-` is a
member. OPR-P1 suggests no order. I needed to know if member operators
are ordered today. Suggestion: state that SEQ-2 covers only the call
syntax, and add a test with a member operator.

**X1-3.** Page: classes/comparisons.md. Rules: DCMP-6. Kind: missing.
DCMP-6 does not say in which order the operands of a reversed or
rewritten candidate are evaluated. The host Clang evaluates the
operand that becomes the first argument first (`y` in `x == y`). The
host GCC evaluates `x` first. I needed the current order to state the
impact. Suggestion: record the order as unspecified with a test, and
add the observed behavior to Evidence.

**X1-4.** Page: declarations/functions/default-arguments.md. Rules:
DEFARG-3, SEQ-1. Kind: missing. No rule says when a default argument is
evaluated relative to the explicit arguments. The host GCC evaluates
the default first. I needed this to place defaults in the order.
Suggestion: state that the order is unspecified as part of SEQ-1, with
a test.

**X1-5.** Pages: expressions/new-delete.md,
foundations/evaluation-order.md. Rules: NEW-10, NEW-12. Kind: missing
(roadmap D9, still open). The order of placement arguments, the array
bound, and the allocation call is not given. Both host compilers
evaluate the bound before the placement arguments. Suggestion: a rule
that gives the full order of a `new` expression, with a test.

**X1-6.** Page: declarations/initialization/list.md. Rules: LIST-3,
LIST-7, LIST-9. Kind: missing. LIST-7 gives the order of a braced list.
No rule gives the order of a parenthesized aggregate initializer. Both
host compilers go left to right. Suggestion: extend LIST-7 to
parentheses, or say the order is unspecified.

**X1-7.** Page: expressions/lambdas.md. Rules: LAM-7. Kind: missing. No
rule gives the order in which captures are initialized. Both host
compilers go left to right for init captures. Suggestion: a rule and a
test.

**X1-8.** Pages: declarations/functions/definitions.md,
foundations/values.md. Rules: FN-10, VC-10. Kind: missing. The rules say
when parameter objects end, and not in which order relative to each
other. Both host compilers end them in the reverse of the order they
created them in. So the order differs between them today. Suggestion:
state the order, or state that it follows the argument order, and add a
test.

**X1-9.** Pages: foundations/constant-evaluation.md,
expressions/assignment.md. Rules: CE-P3, ASN-11, SEQ-1. Kind:
contradictory. CE-P3 says a constant evaluation gives one answer that
every implementation can agree on, and rejects results that depend on
an order the language does not fix. The ASN-11 constant test accepts
either 1 or 10, so two implementations may disagree. The host GCC gives
10 as a constant and 1 at run time for the same function. Tidal does
not say that an implementation's constant order can differ from its
run-time order. Suggestion: narrow CE-P3 to undefined operations, and
record the GCC difference in the SEQ-1 Evidence.

**X1-10.** Pages: foundations/constant-evaluation.md,
foundations/evaluation-order.md. Rules: CE-9, SEQ-4, MUL-14. Kind:
contradictory. CE-9 says constant evaluation rejects every undefined
operation. Both host compilers accept `i++ + i++`, `i + i++`, and
`i++ * i` in a `static_assert` and give 3, 2, and 2. The SEQ-4 test
`unsequenced-constant-evaluation.cpp` contains no unsequenced access, so
it does not test the rule. Suggestion: a constant test with a real
unsequenced pair, and an entry for SEQ-4 and MUL-14 in the undefined
table that says the compilers accept it.

**X1-11.** Page: exceptions/unwinding.md. Rules: UNW-P1, UNW-1, UNW-3,
VC-6. Kind: missing. The unwinding rules cover block objects,
subobjects, and `new`. They do not say what happens to the temporaries
and argument objects of a full expression when a later operand throws.
I needed this to describe what a fixed order changes for exceptions.
Suggestion: a rule that temporaries already created end in reverse
order during unwinding, and that operands not yet reached are not
evaluated.

**X1-12.** Pages: foundations/evaluation-order.md, tests/RESULTS.md.
Rules: SEQ-1, SEQ-8, ADD-13, MUL-13. Kind: needs example (evidence).
The Evidence gives the observed order only for SEQ-1 on Clang 22 and
GCC 16, and Q-0005 gives it for `*`. RESULTS.md shows pass or fail
only. The impact section needs the order that each matrix compiler
(including Clang 18 and GCC 14) takes for each construct. My table in
section 4.1 comes from the host compilers alone. Suggestion: an
Evidence table of observed orders per compiler and construct.

**X1-13.** Page: expressions/primary.md. Rules: none (templates
referenced). Kind: outside the spec. Fold expressions and pack
expansions belong to templates, which Tidal does not cover. P3 must say
that an expanded argument list or fold follows the expanded order, and
I could not cite a rule for it. Suggestion: cover packs when templates
are written.

**X1-14.** Page: foundations/translation.md. Rules: TR-P5. Kind:
outside knowledge. TR-P5 says the freedom of SEQ-P2 is where
optimization lives, without saying what it buys. My view that GCC's
right-to-left order is tied to how it passes arguments, and so has a
cost to change, comes from outside Tidal. Suggestion: a short note in
the Evidence of SEQ-1 on why implementations differ, if Tidal has a
source for it.

**X1-15.** Pages: glossary.md, foundations/evaluation-order.md,
expressions/assignment.md. Rules: SEQ-P2, SEQ-1, SEQ-8, ASN-11. Kind:
ambiguous. Tidal has two relations: unsequenced (may overlap, undefined
if they conflict) and "either order, no overlap" (SEQ-1, ASN-11). Only
the first has a glossary term. The glossary says "most binary
operators", which no longer matches SEQ-8. P3 removes both relations,
and the proposal has to name each. Suggestion: a glossary term for the
second relation, used by SEQ-1, SEQ-8, and ASN-11.

**X1-16.** Page: classes/operators.md. Rules: OPR-3, OPR-7, SEQ-6. Kind:
missing. OPR-7 says `[]` keeps the built-in order, and the built-in
subscript has one index. A multi-argument `operator[]` has no stated
order among its arguments. Suggestion: say they are call arguments
under SEQ-1.

**X1-17.** Page: foundations/evaluation-order.md. Rules: SEQ-P2, SEQ-4.
Kind: needs example. The Examples segment shows `i++ + i++` as undefined
and `pair(i++, i++)` as fine, which is the best illustration of the two
relations. It does not show a read and a write, such as `i + i++`, or a
bit-field pair, which the rule text names. Suggestion: add one line for
each.

## Rule IDs cited

SEQ-P1, SEQ-P2, SEQ-1, SEQ-2, SEQ-3, SEQ-4, SEQ-5, SEQ-6, SEQ-7, SEQ-8,
ADD-P4, ADD-13, MUL-P4, MUL-13, MUL-14, CMP-P1, CMP-8, BIT-P1, SHF-P4,
SHF-8, ASN-P3, ASN-4, ASN-11, COMMA-1, LOG-5, LOG-11, OPR-P1, OPR-1,
OPR-3, OPR-6, OPR-7, DCMP-P1, DCMP-P2, DCMP-1, DCMP-6, DEFARG-3,
NEW-10, NEW-12, LIST-3, LIST-7, LIST-9, LAM-7, LAM-20, VC-P3, VC-6,
VC-9, VC-10, FN-10, OBJ-P2, UNW-P1, UNW-1, UNW-2, UNW-3, CE-P3, CE-9,
TR-P5, SINIT-3, glossary "unsequenced", Q-0005.
