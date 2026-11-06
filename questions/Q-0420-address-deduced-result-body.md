# Q-0420: Does taking the address of an overload set instantiate a template whose result type comes from its body, when a plain function matches the target?

**Status:** provisional
**Affects:** DED-29, `tests/templates/deduction/address-deduced-result-body-instantiated.cpp`, `tests/templates/deduction/address-deduced-result-body-static-cast.cpp`
**Source:** the RV4 gap review of the text phase 10 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-10-expressions-templates.md`, the DED-29 section), from the probe grid `reports/clean-room/roadmap-9/writer-probes/ded29-address-target.json` that a clean-room writer ran for it.
**Waits for:** a first review

## Context

An overload set `g` has a plain function `int g(const Circle&)` and a
function template whose result type is `auto` and whose body names a
member that `Circle` lacks. The program takes the address of `g` with
the target type `int (*)(const Circle&)`, by initialization or by a
`static_cast`. Deduction from the target gives the template argument
`Circle`. The result type of that specialization comes from its body.

| Implementation | Result |
|---|---|
| Clang 18 and 22 | rejects: the body of the specialization is not valid |
| GCC 14 and 16 | accepts, and picks the plain function |
| EDG 7.0 | accepts, and picks the plain function |

When the template declares its result type, or gives it with a trailing
`decltype` that fails for `Circle`, every compiler accepts. A call
`g(Circle{})` is accepted everywhere, and DED-29 already says that the
body of a losing candidate of a call is not instantiated.

## Evidence

The two tests in **Affects** expect the rejection and record GCC and EDG
as overrides. `address-declared-result-body-not-instantiated.cpp` is
the control.

## Options

1. The specialization is instantiated to find its result type, because
   the result type takes part in the match with the target. The error
   in its body makes the program not valid (Clang, and the review's
   reading).
2. A plain function that matches the target is preferred before the
   template's type is needed, so the body is not instantiated (GCC,
   EDG).

## Provisional answer

The specialization is instantiated and the program is not valid
(option 1). The match with a target type compares the whole function
type, result included, for each candidate, and the gap review reads the
reference the same way.

## Opinions
