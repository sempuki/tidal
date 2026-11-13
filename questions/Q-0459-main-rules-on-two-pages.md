# Q-0459: Which rule names each constraint on `main`, when TR-3 and FN-4 both state it?

**Status:** provisional
**Affects:** TR-3, FN-4, the tests of both rules
**Waits for:** a first review

## Context

TR-3 (`foundations/translation`) and FN-4
(`declarations/functions/definitions`) both constrain the function
`main` of the global namespace. The two lists overlap:

| Constraint | TR-3 | FN-4 |
|---|---|---|
| return type `int`, not deduced | yes | yes |
| not `inline`, `constexpr`, `consteval`, or a template | yes | yes |
| not called, address not taken | yes | yes |
| not `static` or deleted | yes | no |
| a parameter list that the implementation accepts (FN-6) | yes | no |
| no C language linkage for a function or variable named `main` | in any namespace | for a function in another namespace |
| no global variable named `main` | yes | yes |

Neither rule's text names the other, so a test that cites one rule
does not reach the other as a linked rule. Both pages have tests of
the same forms. `main-inline.cpp`, `main-constexpr.cpp`, and
`main-consteval.cpp` exist under `foundations/translation/`, where they
cite TR-3, and under `declarations/functions/definitions/`, where they
cite FN-4.

## Evidence

The rewrite's front end (`frontend/next/`) names TR-3 for the
declaration checks and FN-4 for a call and an address. Each report
gives the other rule as its cause, so the first error names the rule
that either page's tests cite.

## Options

1. Keep both rules. Each diagnostic about `main` names both, and the
   tests may cite either.
2. Keep one rule. The other page states the constraint by reference to
   it, and the tests of the other page cite the kept rule.
3. Split by subject. TR-3 keeps what makes `main` the program's start
   (linkage, the namespace, `static`, deleted), and FN-4 keeps what
   the function's declaration and uses may be. Each rule links to the
   other.

## Provisional answer

Keep both rules (option 1). The specification text does not change,
and a front end names both rules on each diagnostic about `main`.

Opened on 2026-11-13 (decision 0066, item RW3). The rewrite's first
errors named one rule where the test cited the other, which kept these
tests out of the takeover list. Option 2 or 3 is an edit of the text
that a review can weigh.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
