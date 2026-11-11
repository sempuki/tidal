# Q-0431: Must an implementation reject a declared `std::initializer_list` whose count comes before the pointer?

**Status:** provisional
**Affects:** LSUP-1, `tests/foundations/language-support/initializer-list-count-before-pointer.cpp`
**Source:** the P2 report (`reports/clean-room/P2/report.md`, P2-6), the round 11 writer probe `reports/clean-room/roadmap-11/writer-probes/p2-points.json`.
**Waits for:** a review (silent)

## Context

A program declares `std::initializer_list` itself (LSUP-1) and then
initializes a parameter of that type from a braced list.

| Shape | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| a pointer, then a count of type `decltype(sizeof 0)`, private constructor | accepts | accepts | accepts |
| a count, then a pointer | rejects | rejects | accepts |
| a pointer, then a count of type `unsigned int` | rejects | rejects | accepts |

## Evidence

The test in **Affects**, and the probe in **Source**.

## Options

1. The shape of LSUP-1 is fixed: the pointer comes first, and the count
   has the type of `sizeof`. Another shape is rejected (Clang, GCC).
2. An implementation can accept other orders and count types (EDG).

## Provisional answer

The shape of LSUP-1 is fixed, and another shape is rejected (option 1).
Clang and GCC reject the other shapes, and EDG alone accepts them.

Kept open on 2026-11-11 under decision 0054 (item RV7). The review finds the reference silent on the shape. The answer is Tidal's own choice, and the question waits with `pending.md` row 10.

Kept open on 2026-11-11 (decision 0063, item RV8). The read of the RV7 repair diff agrees with the Q-0431 line and gives no new reading, so the question still waits with `pending.md` row 10.

## Opinions

### Gap review, 2026-11-11 (phase 14 text, classes, declarations, foundations, and statements)

The source-reader gap review of the text phase 14 wrote on classes, declarations, foundations, and statements (`reports/review/2026-11-11-gap-review-phase-14-classes-declarations-foundations.md`) reads the question: the reference is silent (RV7-A-9). The reference gives no layout for the template, and its library text names two possible representations only as a note. The reference also requires the library's own declaration of the template to be reachable at each use, and a program must not add declarations to the standard namespace. So the reference does not decide which shapes of a program's own declaration work. The rejection by Clang and GCC and the acceptance by EDG are both outside what the reference decides, and LSUP-1 records Tidal's own choice.

### Read of the RV7 repair diff, 2026-11-11

The source-reader read of the RV7 repair diff (`reports/review/2026-11-11-read-of-rv7-repair-diff.md`) reads the RV7 repair: the question's line now says the reference is silent on the shape, which the read finds correct.

### Read of the RV8 repair diff, 2026-11-11

The source-reader read of the RV8 repair diff (`reports/review/2026-11-11-read-of-rv8-repair-diff.md`) names the question among the records that state a review's reading correctly. The read gives no new reading of the question's answer.
