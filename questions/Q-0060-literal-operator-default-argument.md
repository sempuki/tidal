# Q-0060: Can a literal operator have a default argument?

**Status:** provisional
**Affects:** OPR-8, `tests/classes/operators/literal-default-arg.cpp`

## Context

A literal operator for strings takes a pointer and a length. A program
can declare the length parameter with a default argument. A literal never
uses the default, because the implementation always passes the length.
Clang and EDG reject the default argument. GCC accepts it.

## Options

1. A literal operator must not have a default argument.
2. A default argument is valid and has no effect on literals.

## Provisional answer

No; a literal operator must not have a default argument (option 1). Clang and EDG agree, and other operator functions have the
same limit.

Kept open on 2026-11-03 under decision 0054, because no principle on the page states the case. OPR-P2 bars default arguments for an operator function, and OPR-P3, which covers literal operators, does not mention default arguments. GCC 16 is the one newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). OPR-P3 failed step 3 of the test, because the model of literal operators did not mention default arguments, and OPR-P2 did not say that it covers literal operators. Item PQ widened OPR-P3. The literal supplies every argument, so a literal operator has no default arguments, as for the other operator functions. OPR-10 and `literal-default-arg.cpp` check the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses OPR-P3.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A default argument on any parameter of a literal operator makes the program not valid, and the C++23 text says so directly.
