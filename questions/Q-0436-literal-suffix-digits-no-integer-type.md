# Q-0436: Does PRIM-2 apply to an integer literal whose suffix calls a raw literal operator or a literal operator template?

**Status:** provisional
**Affects:** PRIM-2, OPR-8, `tests/classes/operators/literal-raw-long-digits.cpp`, `tests/classes/operators/literal-operator-template-long-digits.cpp`, `tests/programs/roadmap/r12-bigint-second-try.cpp`
**Source:** the round 12 programmer report (`reports/clean-room/roadmap-12/programmer.md`, sections 1.5 and 6), reduction `raw_literal_operator_long_literal.cpp`, and the programmer's sketches `bigint_first_try.cpp` and `bigint_second_try.cpp`.
**Waits for:** a first review

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
