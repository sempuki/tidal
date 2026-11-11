# Q-0451: Is a `goto` to a missing label rejected in a templated function that no specialization uses or in a discarded statement?

**Status:** provisional
**Affects:** JMP-14, `tests/statements/jumps/goto-missing-label-template-never-used.cpp`, `tests/statements/jumps/goto-missing-label-class-template-member.cpp`, `tests/statements/jumps/goto-missing-label-generic-lambda.cpp`, `tests/statements/jumps/goto-missing-label-discarded-branch.cpp`
**Source:** X24-3 (`reports/clean-room/X24/report.md`, probe `t1.cpp`), rerun on the matrix in `reports/clean-room/roadmap-14/writer-probes/x24-compile.json`.
**Waits for:** a first review

## Context

A function body names a label in a `goto`, and the function has no
label of that name. The body is in a templated context:

```cpp
template <class T> void step() { goto nowhere; }
```

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| a function template that no specialization uses | rejects | accepts | rejects |
| a member function of a class template that no specialization uses | rejects | accepts | rejects |
| a generic lambda that the program never calls | rejects | accepts | rejects |
| a discarded branch of `if constexpr` in a function template, used or not | rejects | accepts | rejects |
| a function template with an explicit instantiation | rejects | rejects | rejects |
| a discarded branch of `if constexpr` outside a template | rejects | rejects | rejects |

Every compiler also rejects, in a function template that no
specialization uses, a label name that occurs twice and a `continue`
outside a loop. Every compiler rejects a `break` outside a loop in a
discarded branch of a used function template.

## Evidence

The tests in **Affects** and the probe in **Source**.
`tests/statements/jumps/duplicate-label-template-never-used.cpp` and
`continue-outside-loop-template-never-used.cpp` hold the forms that
every compiler rejects.

## Options

1. The implementation checks the labels of a function at its
   definition, so it rejects each form (Clang, EDG).
2. The check waits for a specialization, as for the constructs of
   DEP-30, so an implementation need not diagnose a template that no
   specialization uses (GCC).

## Provisional answer

The implementation rejects each form (option 1). JMP-14 says that its
conditions are checked at the definition, also in a template that no
specialization uses and in a discarded statement. Clang and EDG agree.
GCC alone accepts, and GCC checks the other jump conditions of JMP-7
and JMP-14 at the definition. A label of a function does not depend on
a template parameter, so DEP-30's list does not take the case.

Opened on 2026-11-11 (decision 0065, item F7). No principle gives the answer. JMP-P1 says that the place a jump names must exist in the same function, and it does not say when an implementation checks the place in a template.
