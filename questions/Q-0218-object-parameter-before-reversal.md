# Q-0218: Is the object parameter of a swapped member candidate added before its parameters are reversed?

**Status:** provisional
**Affects:** ORD-7, `tests/templates/ordering/reversed-member-object.cpp`
**Waits for:** newest families that agree

## Context

`p == x`, with `p` an `int*` and `x` a `const X`, has two candidates
that are templates:

- the member `template <class T> bool X::operator==(T* const&) const`,
  as a swapped candidate,
- the non-member `template <class T> bool operator==(const T&, const X&)`.

The sequences tie. The member gets the object parameter `const X&`
(ORD-6), and its parameters are `(const X&, T* const&)`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | the member |
| GCC 14, 16, host | the non-member |
| EDG 7.0 | the non-member |

If the object parameter is added first and the two parameters are then
reversed, the member compares as `(T* const&, const X&)`, which is more
specialized than the non-member. Without the reversal, or with the
reversal before the object parameter, neither is more specialized, and
the written candidate wins (OVL-32). GCC does not reverse the
parameters of a swapped candidate at all (Q-0211). EDG reverses them
for a non-member swapped candidate.

## Evidence

`tests/templates/ordering/reversed-member-object.cpp`, with overrides
for GCC and EDG. Item O11 of the gap review of 2026-10-31
(`reports/review/2026-10-31-gap-review-deduction-and-ordering.md`)
describes option 1.

When both candidates are swapped, the reversal keeps their order, and every compiler chooses the more specialized one (`swapped-candidates-ordered.cpp`, `swapped-member-candidates-ordered.cpp`).

## Options

1. The object parameter is added, and then the two parameters are
   reversed (Clang).
2. The parameters of a swapped member are compared in the other order,
   or not reversed (GCC, EDG).

## Provisional answer

Yes; Clang differs from GCC and EDG (option 1). The gap review describes it, and Clang implements it. The
reversal of Q-0211 pairs the parameters that meet the same operand.
After the object parameter is added and the two are reversed, each
parameter meets the operand at its own position in `p == x`.

The question stays open under decision 0051 (2026-11-03). Clang 22 chooses the member, and GCC 16 and EDG 7.0 choose the non-member, so two families disagree. The provisional answer follows the one family. GCC 16 does not reverse a swapped candidate at all (Q-0211), so EDG 7.0 is the only family that reverses a non-member and orders a swapped member another way. ORD-P1 supports the provisional answer, because only option 1 pairs each parameter of a swapped member with the operand that it meets. Item O11 of the gap review describes option 1.

Kept on 2026-11-03. The source-reader gap review of question cluster 3 agrees with option 1.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

Kept open on 2026-11-04 under decision 0055, because ORD-P1 changed in this commit: the object parameter now has the member's `const` and `volatile`, and a member with no ref-qualifier takes an rvalue reference also against another member's object parameter. The question waits for the next use of ORD-P1.

Used on 2026-11-04 by the principle-use batch PU2 (`reports/clean-room/PU2/`), under decision 0056. A clean-room reader applied ORD-P1, without the rule text, to the programs of `swapped-member-candidates-ordered.cpp` and `reversed-member-object.cpp`. Each answer names ORD-P1 and gives an outcome that the tests allow, and the batch score counts the use as support. For `reversed-member-object.cpp` the answer says that the outcome depends on the compiler. The reader marked that answer doubtful (finding PU2-6), because the ORD-P1 example records GCC's lack of reversal only for two non-member templates. The answer's reason also says that EDG chooses the swapped member, and EDG 7.0 chooses the non-member.

Kept open on 2026-11-04 under decision 0054, because two newest families, GCC 16 and EDG 7.0, differ from the answer, and the reader answered `reversed-member-object.cpp` with doubt (finding PU2-6). The latest gap review (2026-11-03) also reads with a small doubt.

Kept open on 2026-11-04 under decision 0055, because ORD-P1 changed in this commit: its table gives the address of an overload set with no target type its own row, which cites OVL-20 (finding R9D-4). The ORD-7 example on the ordering page now shows a swapped member template and says that GCC and EDG choose the written non-member (finding PU2-6). The question waits for the next use of ORD-P1.

Kept open on 2026-11-05 under decision 0054, because two newest families differ from the answer in at least one form (gap review of the text phase 9 wrote, item RV3).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 3 (`reports/review/2026-11-03-gap-review-question-cluster-3.md`) reads the reference as supporting option 1, with a small doubt. The object parameter is inserted first, and then the parameter order is reversed, so the reversal covers the object parameter. The reference states the two steps one after the other and gives no example of a swapped member.

### Gap review, 2026-11-05 (expressions and templates)

The source-reader gap review of the text phase 9 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) used ORD-P1 on ``reversed-member-object.cpp``, without the rule text. The model's answer: The member gets its object parameter, then the swapped pair is reversed, and the member is more specialized. Matches the provisional answer: yes. Reference verdict: supports with no doubt.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
