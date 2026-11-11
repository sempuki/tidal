# Q-0437: Does copy initialization leave out a guide that is explicit for its own deduced arguments before the choice?

**Status:** provisional
**Affects:** CTAD-5, CTAD-17, `tests/templates/class-template-deduction/conditional-explicit-dropped-before-choice.cpp`
**Source:** the I7 report (`reports/clean-room/I7/report.md`, I7-14), the round 12 writer probe `reports/clean-room/roadmap-12/writer-probes/i7-ctad.json`.
**Waits for:** a first review

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
