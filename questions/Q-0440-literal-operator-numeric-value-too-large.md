# Q-0440: Must an integer literal that calls the numeric form of a literal operator have a value that `unsigned long long` holds?

**Status:** provisional
**Affects:** PRIM-2, OPR-8, `tests/classes/operators/literal-numeric-form-value-too-large.cpp`
**Source:** the neighbor case of Q-0436, rerun on the matrix by the item F5 writer with `tools/probe_grid.py`.
**Waits for:** a first review

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
