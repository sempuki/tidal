# Q-0378: Which class operands of `?:` convert when the compilers split?

**Status:** provisional
**Affects:** COND-16, COND-3, COND-5, `tests/expressions/conditional/class-deleted-conversion-counts.cpp`, `tests/expressions/conditional/class-const-and-volatile-lvalues.cpp`, `tests/expressions/conditional/class-const-and-volatile-xvalues.cpp`
**Source:** reports/review/2026-11-04-gap-review-phase-7-additions.md and reports/review/2026-11-04-gap-review-phase-7-repairs.md, the COND-16 sections
**Waits for:** a review (doubt)

## Context

COND-16 tries each operand of `?:` against the other when the types
differ and one is a class. The two gap reviews of the text phase 7
wrote read the reference as asking only whether a conversion sequence
can be formed, with access and deleted functions left out of that
test. For two operands of one class type, the conversion goes only
toward the operand whose type has every cv-qualifier of the other.

Probes on the newest compilers split in two forms:

```cpp
struct Tag;
struct Wrap { Wrap(); Wrap(const Tag&); operator Tag() const = delete; };
struct Tag { };
Wrap w; Tag t;
auto r = c ? w : t;            // form 1

struct Box { };
const Box fixed{}; volatile Box shared{};
void f() { (void)(c ? fixed : shared); }   // form 2
```

| Form | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| 1: `Tag` converts to `Wrap`, and the conversion of `Wrap` to `Tag` is deleted | accepts | rejects | rejects |
| 2: a `const` and a `volatile` lvalue of one class type | rejects | rejects | accepts |

Every newest compiler counts a private conversion function in the same
place as the deleted one in form 1, and rejects the expression as
ambiguous.

## Evidence

The three tests in **Affects**. Each expects a rejection and records the
compiler that accepts as an override. Form 2 with two xvalues splits as
the lvalue form does.

## Options

1. Form 1: the deleted conversion counts, and the expression is
   ambiguous (GCC, EDG, and the reviews' reading). Form 2: neither
   operand converts, and the expression is not valid (Clang, GCC).
2. Form 1: a deleted conversion does not count, so only `Tag` converts
   and the result is a `Wrap` (Clang). Form 2: the result is a prvalue,
   as for two such lvalues of a non-class type (EDG).

## Provisional answer

The deleted conversion counts, and a `const` and a `volatile` class
lvalue do not convert (option 1). In each form two of the three newest
families agree. Form 1 treats a deleted function as access is treated,
which every compiler leaves out of the test. Form 2 follows the cv
condition of the COND-16 table.

Kept open on 2026-11-04 under decision 0054, because no review has read the provisional answer, and in each form one newest family differs from it. Item RV2 of decision 0056 reads the question.

Kept open on 2026-11-04 under decision 0055, because COND-16 changed in this commit. The gap review of the text phase 8 wrote on expressions, foundations, and templates supports option 1 in both forms. It also read the base-class row of the COND-16 table without a cv condition, and the row now follows that reading. The question waits for a reader or a reviewer to use the changed rule. In each form one newest family differs.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied COND-P2, without the rule text, to the programs of `class-const-and-volatile-lvalues.cpp` and `class-deleted-conversion-counts.cpp`. The answer for `class-deleted-conversion-counts.cpp` names COND-P2 and reaches the answer (the expression is rejected). The answer for `class-const-and-volatile-lvalues.cpp` names COND-P2 and says the program compiles, which is EDG's result and not the answer. The reader marked that answer doubtful (finding PU1-7), because COND-P2 does not say whether "one type" ignores cv-qualifiers. The batch score counts the answer as right, because the test lists EDG's outcome as an override. So the use does not support the answer in form 2.

Kept open on 2026-11-04 under decisions 0054 and 0056, because the use in batch PU1 does not support the answer in form 2 (finding PU1-7). The question waits for COND-P2 to say how the cv-qualifiers of two class lvalues count, and for a later use. In each form one newest family differs.

Kept open on 2026-11-04 under decision 0055, because COND-P2 changed in this commit after finding PU1-7. COND-P2 now says that two lvalues of one type apart from cv-qualifiers give an lvalue only when the cv-qualifiers of one include those of the other, and that a `const` and a `volatile` lvalue of one class type are rejected. Judged by the base expectation, the answer of batch PU1 for `class-const-and-volatile-lvalues.cpp` is wrong. The question waits for the next use of COND-P2. In each form one newest family differs.

Used on 2026-11-04 by the principle-use batch PU2 (`reports/clean-room/PU2/`), under decision 0056. A clean-room reader applied COND-P2, without the rule text, to the programs of `class-const-and-volatile-lvalues.cpp` and `class-deleted-conversion-counts.cpp`. Each answer names COND-P2 and reaches the answer (a rejection), and the batch score counts the use as support. The reader marked the answer for `class-deleted-conversion-counts.cpp` doubtful (finding PU2-3). COND-P2 does not say whether a conversion through a deleted function counts, and the reader took that from the order of the steps in OVL-P1.

Kept open on 2026-11-04 under decisions 0054 and 0056, because the reader answered form 1 with doubt (finding PU2-3). COND-P2 does not rule out the reading in which a deleted conversion function gives no conversion, so the use does not pass step 3 of the test of the style guide in that form. In each form one newest family differs.

Kept open on 2026-11-04 under decision 0055, because COND-P2 changed in this commit after finding PU2-3: the bullet on two class types that each convert to the other now says that a conversion counts when it can be formed, even if its function is deleted or not accessible. The question waits for the next use of COND-P2. In each form one newest family differs.

Kept open on 2026-11-05 under decision 0054, because the review reads the answer with a doubt (gap review of the text phase 9 wrote, item RV3).

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. The reference sets access, bit-fields, and deleted functions aside when it asks whether a conversion sequence can be formed, so both operands of form 1 convert, and the expression is not valid. In form 2 neither reference binding works, and the row for one class type needs the target to carry every cv-qualifier of the source, which fails in both directions. The overload step then finds no candidate.

### Gap review, 2026-11-05 (expressions and templates)

The source-reader gap review of the text phase 9 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) used COND-P2 on ``class-deleted-conversion-counts.cpp``, without the rule text. The model's answer: The deleted conversion counts, both operands convert, and the expression is rejected. Matches the provisional answer: yes. Reference verdict: supports with no doubt. used COND-P2 on ``class-const-and-volatile-lvalues.cpp``, without the rule text. The model's answer: Neither set of qualifiers includes the other, so two class lvalues are rejected. Matches the provisional answer: yes. Reference verdict: supports with a doubt.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
