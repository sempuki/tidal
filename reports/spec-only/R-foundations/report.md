# Predict batches: Foundations

Date: 2026-10-31. Decisions 0029 and 0030. Two readers, each a
programmer new to C++, predicted the outcome of every test cited by the
seven Foundations pages. They read only `spec/`, `docs/`, and their
batch directories, and compiled nothing. Their answers are in this
directory (`answers-R-<page>.json`).

## Scores

| Batch | Programs | Matching |
|---|---|---|
| R-translation | 26 | 26 |
| R-objects | 27 | 27 |
| R-values | 41 | 41 |
| R-names | 26 | 26 |
| R-conversions | 47 | 47 |
| R-evaluation-order | 18 | 18 |
| R-constant-evaluation | 46 | 46 |

One answer (R-translation p21, a program with no `main`) first scored as
wrong. The batch could not tell the reader whether such a program is
linked, so the scorer now accepts both "compiles" and "link-fail" for
it.

Every prediction matched, so the findings below come from the readers'
notes. Each one marks a place where the reader had to infer, search, or
guess.

## Findings

### Translation

**T1. No rule rejects a pointer converted to an integer.** R-translation
p25, p26. Rules: TR-1, CONV-P3, CONV-8. Each program initializes an
`int` from a string literal. CONV-P3 and CONV-8 cover pointer-to-pointer
and integer-to-pointer conversions only. Suggestion: add a `reject` rule
to conversions.md that a pointer, and so an array that decays to one,
never converts implicitly to an integer type.

**T2. Warnings that `-Werror` turns into rejection have no rule.**
R-translation p24 (unused local), R-evaluation-order p11 (unsequenced
modifications). Rules: TR-6, ATTR-6, SEQ-4. The warnings appear only in
Evidence. Suggestion: add `recommended` rules that an implementation
should warn about a local variable that is never used and about
unsequenced modifications it can see.

**T3. A name with no declaration in its unit is not a stated reject.**
R-translation p05. Rules: TR-4, NAM-1. TR-4 says each unit sees only its
own declarations, as a `required` fact. Suggestion: add a `reject` rule
that a name must find a declaration in its own translation unit, linked
from TR-P2.

**T4. The no-`main` case sits in a `reject` rule but fails at link
time.** R-translation p18, p21. Rule: TR-3. Suggestion: move it to a
separate rule whose class matches a link failure.

**T5. `argc` may or may not be at least 1.** R-translation p17. Rule:
TR-2. Suggestion: state whether `argc` can be 0 and what `argv[0]`
holds, as `impl-defined` if it varies.

**T6. When a `constexpr` function body is checked.** R-translation p08,
p22. Rules: TR-5, TR-8. Suggestion: say that a `constexpr` function that
can never produce a constant is not an error until a constant
evaluation reaches it.

### Objects and names

**O1. Member destruction order is only in a principle.** R-objects p05,
p25. Rules: OBJ-1, OBJ-P2, OBJ-6. Suggestion: add to OBJ-1 that members
are built in declaration order, and that members and bases are
destroyed in the reverse of their construction order.

**O2. Exercises have no outcome for undefined behavior at run time.**
R-names p12. Rules: NAM-1, OBJ-2, TR-8. `int a = a + 1;` is valid by
NAM-1 and undefined by OBJ-2. Suggestion: link NAM-1's own-initializer
example to OBJ-2, and give the Predict format an outcome for undefined
behavior.

**O3. Rule ID prefixes are hard to guess.** All batches. A reader
guessed `NAME-` for `NAM-`. Suggestion: list every page's prefix on
rules.md.

**O4. Names that clash with library names.** R-evaluation-order p02,
p05, p14. These programs declare a global named `log` with no header.
Suggestion: say which names a program may use when it includes no
header.

### Values

**V1. Lifetime extension through a member access is hard to find.**
R-values p22. Rules: REF-4, VC-6, OBJ-8. VC-6 and OBJ-8 say only "a
reference" binds a temporary. Suggestion: say "a temporary or a member
of one" there and link REF-4.

**V2. A braced list as the right operand of `=`.** R-values p09. Rule:
VC-2. Suggestion: cover it on the assignment page and link it from
VC-2.

**V3. Life of an array prvalue passed as a pointer.** R-values p33.
Rules: VC-8, VC-9. Suggestion: add an example.

**V4. A `const` non-class return type.** R-values p14. Rules: DTY-2,
VC-2. Suggestion: one sentence saying the declaration is valid and the
call's prvalue has no `const`.

### Conversions

**C1. No way to get the NaN that CONV-3 converts.** R-conversions p15.
Rules: CONV-3, MUL-11. The program divides zero by zero, which MUL-11
makes undefined. Suggestion: settle Q-0004, or give CONV-3 an example
whose NaN comes from somewhere else.

**C2. Sanitizer behavior is not in Evidence.** R-conversions p13. Rules:
CONV-5, TR-8. Suggestion: say in Evidence that the sanitizer stops the
program with a nonzero status.

**C3. An impl-defined result is only in Evidence.** R-conversions p39.
Rule: CONV-7. Suggestion: an example such as a `float` initialized from
16777219 that becomes 16777220 on every matrix target.

**C4. Out-of-range floating narrowing in constant evaluation.**
R-conversions p04. Rules: CONV-11, CE-9, CONV-5, MUL-12. Suggestion:
state that this conversion is not undefined and is allowed in constant
evaluation, with EDG noted as the exception.

**C5. Function operand of unary `+`; enum promotion.** R-conversions
p05, p17. Rules: UN-1, CONV-1, CONV-13. Suggestion: say in UN-1 that a
function operand becomes a function pointer, and give the promotion of
an enumeration without a fixed type.

**C6. Prohibitions inside `required` rules.** R-conversions p14, p33.
Rules: CONV-14, CONV-15, CONV-17. The ambiguous or inaccessible base
case and the reverse member-pointer direction are stated in `required`
rules. Suggestion: move them into a `reject` rule.

### Constant evaluation

**E1. Template arguments are missing from the converted-constant list.**
R-constant-evaluation p34. Rule: CE-11. Suggestion: add non-type
template arguments to CE-11, or mark them *(referenced)*.

**E2. The answer depends on the compiler.** R-constant-evaluation p41
(CE-13, Q-0033) and p01 (CE-18, Q-0090). Suggestion: when a rule has an
open question, have the Predict README name the compilers that differ.

**E3. A `constexpr` local starts a new evaluation.** R-constant-evaluation
p20. Rules: CE-3, TR-1. Suggestion: add the case of a `constexpr` local
initialized from a parameter as an example.

**E4. Limits are only in Evidence.** R-constant-evaluation p04, p06,
p16. Rule: CE-10. Suggestion: state a minimum depth and step count that
every matrix compiler meets.

**E5. Dangling reads and extended temporaries in constant evaluation.**
R-constant-evaluation p27, p46. Rules: CE-9, CE-14. Suggestion: add a
read through a pointer to an ended local to CE-9's list, and an example
of a `constexpr` reference bound to a temporary.
