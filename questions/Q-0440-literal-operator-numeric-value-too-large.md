# Q-0440: Must an integer literal that calls the numeric form of a literal operator have a value that `unsigned long long` holds?

**Status:** provisional
**Affects:** PRIM-2, OPR-8, `tests/classes/operators/literal-numeric-form-value-too-large.cpp`
**Source:** the neighbor case of Q-0436, rerun on the matrix by the item F5 writer with `tools/probe_grid.py`.
**Waits for:** a principle (PRIM-P1)

## Context

The only literal operator for the suffix `_n` takes `unsigned long
long`. The program writes a literal one greater than the largest
`unsigned long long` value, or a literal of 30 digits, with that suffix.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| `18446744073709551615_n`, the largest value | accepts | accepts | accepts |
| `18446744073709551616_n` | rejects: too large | accepts | rejects: too large |
| a 30-digit literal | rejects | accepts | rejects |
| a 30-digit literal, with the `const char*` form also declared | rejects | accepts | rejects |
| a 30-digit literal, with only the `const char*` form or only a template | accepts | accepts | accepts |

## Evidence

The test in **Affects**. Q-0436 holds the raw form and the template.

## Options

1. The literal must have a value that `unsigned long long` holds when
   it calls the numeric form (Clang, EDG).
2. The program is valid, and the implementation can warn (GCC, which warns that the literal is too large for its type).

## Provisional answer

The literal must have a value that `unsigned long long` holds (option
1). PRIM-2 says so. Clang and EDG agree, and GCC alone differs.

Kept open on 2026-11-11 under decision 0054 (decision 0063, item RV8). The review supports the answer with no doubt, and GCC alone differs. No principle gives the answer. PRIM-P1 gives the type list of a plain literal and does not say that a literal that calls the numeric form passes its digits as such a literal, so PRIM-P1 fails step 2 of the principle test.

## Opinions

### Gap review, 2026-11-11 (phase 15 text, expressions, overloading, and templates)

The source-reader gap review of the phase 15 text on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`) reads the question: supports with no doubt (RV8-B-10). A literal that calls the numeric form becomes a call whose argument is the same digits as a plain `unsigned long long` literal. That literal must have a type in its list, so a value that the type cannot hold is not valid (option 1). The list allows an extended unsigned integer type that holds the value, and Tidal's targets have none. The review adds that GCC's warning is a diagnostic, and that a diagnostic is all that the reference asks for a program that is not valid. So GCC conforms too, and the split lies in how Tidal's tests count a warning.

### Gap review, 2026-11-11 (phase 15 text, classes, declarations, foundations, and statements)

The source-reader gap review of the phase 15 text on classes, declarations, foundations, statements, and the glossary (`reports/review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md`) names the question in RV8-A-5 and agrees that the plain literal of the call must fit its type. The review adds a small doubt for an implementation whose extended integer types hold the value. There the argument has that type and converts to `unsigned long long` in the call.
