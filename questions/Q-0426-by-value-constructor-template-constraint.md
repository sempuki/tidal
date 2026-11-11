# Q-0426: Does a copy of a class check the constraint of a constructor template with the class as the template's argument?

**Status:** provisional
**Affects:** CPY-15, CNS-35, CNS-11, `tests/templates/constraints/by-value-constructor-template-constraint-copies-class.cpp`, `tests/templates/constraints/exact-non-template-skips-constraint-check.cpp`, `tests/templates/constraints/added-const-non-template-checks-constraint.cpp`, `tests/classes/copy-move/copy-constructor-exact-skips-template-constraint.cpp`, `tests/classes/copy-move/by-value-constructor-template-constraint-checked.cpp`
**Source:** W20-3 (`reports/clean-room/W20/report.md`) with the writer probe `reports/clean-room/roadmap-10/writer-probes/w20-3-self-by-value-constraint.json`.
**Waits for:** the maintainer (pending.md row 4)

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

A recursive constraint does not show on every compiler whether the
check runs. The writer probes
`reports/clean-room/roadmap-10/writer-probes/f3b-w20-3-constraint-checked.json`
and `f3b-w20-3-function-exact-match.json` use a constraint that
instantiates a class whose static assertion fails for the class type.
The program is then rejected exactly when the constraint is checked:

| Call | Clang 18 | Clang 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|---|
| a copy of a `const` lvalue or a move of an xvalue, beside a template `Box(T)` or `Box(const T&)` | checks | does not check | does not check | checks |
| a copy of an lvalue that is not `const`, beside `Box(const T&)` or `Box(T&)` | checks | checks | checks | checks |
| a copy of an lvalue that is not `const`, beside `Box(T)` | checks | checks | does not check | checks |
| `f(x)` with a function `f(const Box&)` that is not a template, `x` a `const` lvalue, beside a template `f(const T&)` | checks | does not check | does not check | checks |
| the same with `x` an lvalue that is not `const`, or a prvalue | checks | checks | checks | checks |
| `f(x)` with `f(Box&&)` and an xvalue or prvalue, or with `f(Box)` and any argument | checks | does not check | does not check | checks |

Clang 22 and GCC skip the check when a candidate that is not a template
takes every argument with no conversion. The check is skipped for
functions and for constructors alike. EDG and Clang 18 always check.
For the recursive form, EDG checks and still accepts.

## Evidence

The tests in **Affects**:

- `by-value-constructor-template-constraint-copies-class.cpp`, the
  recursive form, expects a run and records Clang 18 as an override.
- `exact-non-template-skips-constraint-check.cpp` and
  `copy-constructor-exact-skips-template-constraint.cpp` expect the
  check to be skipped, and record EDG and Clang 18 as overrides.
- `added-const-non-template-checks-constraint.cpp` expects the check,
  with no override.
- `by-value-constructor-template-constraint-checked.cpp` expects the
  check for a by-value template and an lvalue that is not `const`, and
  records GCC as an override.

The writer probe `w20-3-self-by-value-constraint.json` covers both
parameter forms of the recursive constraint, a copy of a `const
Holder`, and a copy-initialization.

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

Rule text on 2026-11-10 (decision 0061, item F3): CNS-11 says that the
constraints of the function template candidates are not checked when a
candidate that is not a template takes every argument with no
conversion. The copy constructor takes a `const Holder` lvalue that
way, so the W20-3 program runs. CPY-15 says that the constraints of a
by-value constructor template are checked before its specialization is
dropped, unless CNS-11 skips the check. GCC differs on the second
point, and EDG on the first, so in each case one newest family
differs. The question waits for a first review.

Kept open on 2026-11-10 under decision 0054. The review reads against the answer, and Clang 22 and GCC 16 keep the matrix's answer, so the answer stays and the question joins `pending.md` row 4 (item RV6). EDG follows the review in the CNS-11 forms, and checks and still accepts in the W20-3 form.

## Opinions

### Gap review, 2026-11-10 (phase 13 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 13 wrote on expressions, overloading, and templates (`reports/review/2026-11-10-gap-review-phase-13-expressions-overloading-templates.md`) reads the question: reads against the answer (RV6-B-1). Every function template candidate goes through deduction, and deduction checks the constraints once the template arguments are known. The reference has no step that skips that check because a candidate that is not a template matches every argument exactly, and it gives an example where an error in that check makes a call not valid beside such a candidate. So the W20-3 copy checks the constraint for `Holder` itself (option 2), as Clang 18 and EDG do. CPY-15's order, a check before the by-value specialization is dropped, is a fair reading. The reference has no rule for a check that needs its own result, so it does not decide Clang 18's rejection beyond the check itself. The review of the first part (RV6-A-18) reads CPY-15's link to CNS-11 the same way.

### Gap review, 2026-11-10 (phase 13 text, classes, declarations, foundations, and statements)

The source-reader gap review of the text phase 13 wrote on classes, declarations, foundations, and statements (`reports/review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md`) names the question in its reading of CPY-15 (RV6-A-18). The main part of CPY-15 agrees. The last sentence, which lets CNS-11 skip the constraint check, reads against the reference: every template candidate has its constraints checked during deduction, also for the class type itself in every copy. The rule that a template never gives a by-value constructor of the class applies after that check.
