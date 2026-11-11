# Q-0426: Does a copy of a class check the constraint of a constructor template with the class as the template's argument?

**Status:** provisional
**Affects:** CPY-15, CNS-35, CNS-11, `tests/templates/constraints/by-value-constructor-template-constraint-copies-class.cpp`
**Source:** W20-3 (`reports/clean-room/W20/report.md`) with the writer probe `reports/clean-room/roadmap-10/writer-probes/w20-3-self-by-value-constraint.json`.
**Waits for:** a first review; a rule

## Context

A class `Holder` has a default constructor and a constructor template
`Holder(T)` whose constraint requires that a `T` can be built from a
`const T&`. A concept checks `Holder(t)` with `t` a `const T&`, and a
`static_assert` checks the concept for `Holder` itself. For
`T = Holder`, the constraint of the template asks again whether a
`Holder` can be built from a `const Holder&`. CPY-15 says that a
constructor template never gives a constructor whose first parameter is
the class by value. The text does not say whether that specialization
leaves the candidates before or after its constraint is checked. With
the form `Holder(const T&)`, no rule removes the specialization, and the
copy constructor wins only after the constraint is checked. If the
constraint is checked, the check needs its own result, which CNS-35
rejects.

| Form | Clang 18 | Clang 22 | GCC 14 and 16 | EDG 7.0 | Tidal front end |
|---|---|---|---|---|---|
| `Holder(T)`, the concept for `Holder` | rejects (the constraint depends on itself) | accepts | accepts | accepts | rejects (CNS-35) |
| `Holder(const T&)`, the same | rejects | accepts | accepts | accepts | rejects (CNS-35) |

## Evidence

The test in **Affects**. It expects a run and records Clang 18 as an
override. The writer probe covers both parameter forms, a copy of a
`const Holder`, and a copy-initialization.

## Options

1. The copy does not check the constraint of the template for
   `T = Holder`, so the check never needs its own result, and the
   program runs (Clang 22, GCC, EDG).
2. The copy checks the constraint, the check needs its own result, and
   CNS-35 rejects the program (Clang 18).

## Provisional answer

No; the copy does not check that constraint, and the program runs
(option 1).
Every newest compiler accepts, and Clang 18 alone rejects.

## Opinions
