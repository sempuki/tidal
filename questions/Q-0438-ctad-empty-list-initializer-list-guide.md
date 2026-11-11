# Q-0438: Does an empty braced list skip the initializer-list guides when a guide with no parameters exists?

**Status:** provisional
**Affects:** CTAD-18, CTAD-17, `tests/templates/class-template-deduction/initializer-list-guide-empty-list.cpp`, `tests/templates/class-template-deduction/initializer-list-guide-empty-list-defaults.cpp`
**Source:** the I7 report (`reports/clean-room/I7/report.md`, I7-6), the round 12 writer probe `reports/clean-room/roadmap-12/writer-probes/i7-ctad.json`.
**Waits for:** a review (doubt); newest families that agree

## Context

A class template has a default constructor and a constructor that
takes `std::initializer_list<int>`. User guides give a different result
for each: `C(std::initializer_list<int>) -> C<long>` and
`C() -> C<char>`. The program writes `C c{};`.

ILC-3 runs two phases for a braced list: the initializer-list
constructors first, then all constructors. For constructors, an empty
list with a default constructor gives value initialization. CTAD-18
uses the two phases for the guides, and CTAD-17 sends the reader to
ILC-3.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| `C c{};` | `C<char>`, the guide with no parameters | `C<long>`, the initializer-list guide | `C<long>` |
| `C c{1, 2};` with a guide `(std::initializer_list<int>, int = 0)` and a guide `(int, int)` | the initializer-list guide | the initializer-list guide | the initializer-list guide |
| `C c(1, 2);`, same guides | `(int, int)` | `(int, int)` | `(int, int)` |

## Evidence

The test in **Affects**, `initializer-list-guide-with-default.cpp` for
the list of two, and the probe in **Source**.

## Options

1. An empty list skips the first phase when a guide with no parameters
   exists, as ILC-3 does for a default constructor (Clang).
2. An empty list tries the initializer-list guides first (GCC, EDG).

## Provisional answer

An empty list skips the initializer-list guides when a guide with no
parameters exists (option 1). The guides act as the constructors of the
class (CTAD-17), and ILC-3 gives this order for constructors. GCC and
EDG differ, so two newest families stand against the answer.

Kept open on 2026-11-11 under decision 0054 (decision 0063, item RV8). The review supports the answer with a doubt, and GCC 16 and EDG 7.0 differ.

Rule text on 2026-11-11 (decision 0063, item RV8): CTAD-18 says that an empty list skips the first phase when a guide that takes no arguments exists, also a guide whose parameters all have defaults (RV8-B-8). The test `initializer-list-guide-empty-list-defaults.cpp` holds that form, and GCC and EDG differ as for the guide with no parameters. The case of a first parameter that is a reference to the list has its own question, Q-0443.

## Opinions

### Gap review, 2026-11-11 (phase 15 text, expressions, overloading, and templates)

The source-reader gap review of the phase 15 text on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`) reads the question: supports with a doubt (RV8-B-8). For an empty list and a class with a default constructor, the reference skips the phase of initializer-list constructors and value-initializes. The guides act as the constructors of a notional class, and a guide that takes no arguments is the natural default constructor of that class. That is Clang's reading and option 1. The reference does not say what a default constructor of the notional class is. The review adds two doubts on CTAD-18. A guide whose parameters all have defaults also takes no arguments, and CTAD-18 speaks only of a guide with no parameters. CTAD-18's definition of an initializer-list guide leaves out a first parameter that is a reference to an initializer list type.

### Read of the RV8 repair diff, 2026-11-11

The source-reader read of the RV8 repair diff (`reports/review/2026-11-11-read-of-rv8-repair-diff.md`) reads the Q-0438 line on a guide whose parameters all have defaults: agrees, with the doubt that RV8-B-8 gave the question. The CTAD-18 repairs close both doubts of RV8-B-8 on CTAD-18. The doubt on what a default constructor of the notional class is stays with this question. The read gives no new reading of the question's answer.
