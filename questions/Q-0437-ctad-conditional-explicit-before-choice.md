# Q-0437: Does copy initialization leave out a guide that is explicit for its own deduced arguments before the choice?

**Status:** provisional
**Affects:** CTAD-5, CTAD-17, `tests/templates/class-template-deduction/conditional-explicit-dropped-before-choice.cpp`
**Source:** the I7 report (`reports/clean-room/I7/report.md`, I7-14), the round 12 writer probe `reports/clean-room/roadmap-12/writer-probes/i7-ctad.json`.
**Waits for:** a principle (CTAD-P1)

## Context

`template <class T> struct N { constexpr explicit(sizeof(T) == 1) N(T) {} };`
has a guide from its constructor that is explicit for a deduced `T` of
size 1. A user guide `N(int) -> N<int>` is not explicit. The program
writes `N b = 'c';`. The guide from the constructor deduces `T = char`,
is explicit for `char`, and is the better match. The user guide needs a
promotion.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| `N b = 'c';` | `N<int>`, the user guide | chooses `N<char>`, then rejects the conversion from `char` | `N<int>` |
| `N b = {'c'};` | rejects (CTAD-23) | rejects | rejects |
| `N b('c');` | `N<char>` | `N<char>` | `N<char>` |

## Evidence

The test in **Affects**, `conditional-explicit.cpp`, and the probe in
**Source**.

## Options

1. Each guide's condition is evaluated for its own deduced arguments
   before the choice, and copy initialization leaves out each guide that
   is then explicit (Clang, EDG).
2. The condition is evaluated after the choice, and an explicit result
   makes the copy initialization fail (GCC).

## Provisional answer

Copy initialization leaves out a guide that is explicit for its own
deduced arguments before the choice (option 1). CTAD-17 says so. Clang
and EDG agree, and GCC alone differs.

Kept open on 2026-11-11 under decision 0054 (decision 0063, item RV8). The review supports the answer with no doubt, and GCC alone differs. No principle gives the answer. CTAD-P1 checks an explicit guide in step 4, after the choice, and names only copy-list initialization, so a reader of CTAD-P1 alone can reach option 2. CTAD-P1 fails step 3 of the principle test.

## Opinions

### Gap review, 2026-11-11 (phase 15 text, expressions, overloading, and templates)

The source-reader gap review of the phase 15 text on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`) reads the question: supports with no doubt. For a candidate whose explicit condition depends on template parameters, the reference deduces first. A context that takes only candidates that are not explicit then drops the specialization when the condition is true. A guide keeps the explicit condition of its constructor. So copy initialization drops the guide that is explicit for its own deduced arguments before the choice, as Clang and EDG do (option 1).

### Read of the RV8 repair diff, 2026-11-11

The source-reader read of the RV8 repair diff (`reports/review/2026-11-11-read-of-rv8-repair-diff.md`) names the question among the records that state a review's reading correctly. The read gives no new reading of the question's answer.
