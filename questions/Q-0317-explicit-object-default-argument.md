# Q-0317: Can an explicit object parameter have a default argument?

**Status:** provisional
**Affects:** CLS-14, `tests/classes/members/explicit-object-default-argument.cpp`

## Context

A member function declares its explicit object parameter as
`this Gauge self = Gauge{}`.

| Compiler | Result |
|---|---|
| Clang 22, 18 | rejects |
| GCC 16, 14 | rejects |
| EDG 7.0 | accepts |

The object of a call always initializes the parameter (CLS-13), so a
default argument for it can never be used.

## Evidence

The test above. Clang says that the explicit object parameter cannot
have a default argument, and GCC says the same in other words.

## Options

1. The default argument is not valid.
2. The default argument is valid and has no effect.

## Provisional answer

No (CLS-14); EDG accepts (option 1). Clang and GCC reject the declaration, and the default
argument could never take effect. The test expects a rejection and
records EDG's result with an override.

Kept open on 2026-11-03 under decision 0054, because no principle on the page states the case. CLS-P3 lists what a function with an explicit object parameter cannot be, and the list does not mention a default argument. EDG 7.0 is the one newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CLS-P3 failed step 3 of the test, because its list of what the function cannot be left out the default argument, so option 2 also fitted the model. Item PQ widened CLS-P3. The object of each call initializes the explicit object parameter, so a default argument could never take effect, and the parameter cannot have one. CLS-14 and `explicit-object-default-argument.cpp` check the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CLS-P3.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. The grammar of a parameter has no form with both the explicit object marker and a default argument. EDG's acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
