# Q-0436: Does PRIM-2 apply to an integer literal whose suffix calls a raw literal operator or a literal operator template?

**Status:** provisional
**Affects:** PRIM-2, OPR-8, `tests/classes/operators/literal-raw-long-digits.cpp`, `tests/classes/operators/literal-operator-template-long-digits.cpp`, `tests/programs/roadmap/r12-bigint-second-try.cpp`
**Source:** the round 12 programmer report (`reports/clean-room/roadmap-12/programmer.md`, sections 1.5 and 6), reduction `raw_literal_operator_long_literal.cpp`, and the programmer's sketches `bigint_first_try.cpp` and `bigint_second_try.cpp`.
**Waits for:** a principle (PRIM-P1)

## Context

PRIM-2 says that an integer literal must have a type in its list. A
program writes a literal of 25 or more digits with a suffix of its own:

```cpp
constexpr int operator""_digits(const char* text) { /* counts */ }
int n = 1234567890123456789012345678_digits;
```

No integer type holds the value. The suffix names a raw literal
operator, which gets the digits as a string (OPR-8), so the program
never needs the value as an integer. A literal operator template with a
pack of `char` gets the digits as template arguments in the same way
({question}`Q-0435`).

PRIM-2 does not say whether the rule covers a literal with such a
suffix. The text read alone can reject the program.

| Form | GCC 14, 16 | Clang 18, 22 | EDG 7.0 | Tidal front end |
|---|---|---|---|---|
| a raw literal operator and a 25-digit or 28-digit literal | runs | runs | accepts | rejects, PRIM-2 |
| a literal operator template and a 30-digit literal | runs | runs | accepts | unsupported |

The front-end results come from the round 12 programmer report.

## Evidence

The tests in **Affects**, which the matrix runs on every baseline
compiler.

## Options

1. PRIM-2 applies only to a literal with no suffix or with a suffix of
   the built-in forms. A literal whose suffix calls a raw literal
   operator or a literal operator template needs no integer type, so
   any number of digits is valid.
2. PRIM-2 applies to every integer literal, so a long literal is not
   valid with any suffix. Every compiler accepts the programs.

## Provisional answer

PRIM-2 does not apply to such a literal (option 1). Every compiler
accepts the programs, and the raw form exists to pass digits that no
type holds. Item F5 of decision 0063 writes the text.

A neighbor case stays outside this answer. When the suffix calls the
`unsigned long long` form, the writer's host probe gives a split on a
30-digit literal. GCC 16 accepts the program with a warning that the
literal exceeds the range of the type, and Clang 22 rejects it. No test
holds the case yet.

Rule text on 2026-11-11 (decision 0063, item F5): PRIM-2 says that a literal whose suffix calls a literal operator has no type list, and that its value must fit in `unsigned long long` only when it calls the numeric form. The neighbor case of the numeric form now has its own question, Q-0440, and the test `literal-numeric-form-value-too-large.cpp`. The question now waits for a first review.

Kept open on 2026-11-11 under decision 0054 (decision 0063, item RV8). The review supports the answer with no doubt, and every compiler agrees. No principle gives the answer. PRIM-P1 says that an integer literal that no type in its list can hold is rejected, and it does not set apart a literal whose suffix calls a literal operator. A reader of PRIM-P1 alone can reach option 2, so PRIM-P1 fails step 3 of the principle test. OPR-P3 says nothing on the value.

## Opinions

### Gap review, 2026-11-11 (phase 15 text, classes, declarations, foundations, and statements)

The source-reader gap review of the phase 15 text on classes, declarations, foundations, statements, and the glossary (`reports/review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md`) reads the question: supports with no doubt (RV8-A-5). A literal with a suffix of its own is a separate kind of token, built from the digits and the suffix. The rule that an integer literal must fit a type in its list covers only the plain kind. The raw form receives the digits as a string and the template receives them as characters, so neither needs an integer type to hold the value (option 1). The doubt that the review adds belongs to Q-0440.

### Gap review, 2026-11-11 (phase 15 text, expressions, overloading, and templates)

The source-reader gap review of the phase 15 text on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`) reads PRIM-2 with the question's case: agrees (RV8-B-10). The raw form and the template receive the characters, and the reference sets no limit on the digits for them.

### Read of the RV8 repair diff, 2026-11-11

The source-reader read of the RV8 repair diff (`reports/review/2026-11-11-read-of-rv8-repair-diff.md`) names the question among the records that state a review's reading correctly. The read gives no new reading of the question's answer.
