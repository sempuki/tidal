# 0048. An interpreter for the abstract machine

Date: 2026-10-31
Status: Accepted. Builds on 0035 and 0047.

## Context

The front end checks programs: it accepts or rejects them and names the
rule. Nothing checks the run-time rules in the same way. The matrix
compilers run the 901 `run` tests, and their results show only that one
implementation agrees. The front end has a constant evaluator for
`static_assert` and constant expressions, which runs a subset of the
language with the extra limits of constant evaluation.

## Decision

- The front end gets an interpreter, a component of its own that
  executes a checked program on the abstract machine of the
  specification. The interpreter is not part of the constant evaluator.
  Constant evaluation and any other feature use the interpreter as a
  client and add their own limits.
- The interpreter ends a run in one of three ways (decision 0047):

  | Outcome | Meaning |
  |---|---|
  | result | the program ends with an exit status, which the rules define |
  | unspecified | the rules allow several results; the interpreter can try each allowed choice and report the set |
  | undefined | the run reaches undefined behavior; the interpreter stops and names the rule, and gives no result |

- Constant evaluation runs the interpreter with its own limits. An
  operation that constant evaluation does not allow stops the run with
  the rule that forbids the operation. Undefined behavior stops the run
  with the rule that names it, as at run time.
- `tidal-fe run FILE` runs a program. The conformance harness runs each
  `run` and `run-fail` test, compares the exit status with the test's
  expectation, and for a test of undefined behavior expects a stop that
  names one of the test's rules.
- A defined result must match every configuration of every matrix
  compiler. A mismatch is a finding against the rule, or a question
  when a compiler may be at fault.

## Consequences

- The interpreter is written from the specification as a clean-room
  reader task, like the rest of the front end (decisions 0029, 0035),
  and reports findings where the rules do not say what a run does.
- The constant evaluator moves onto the interpreter. Its behavior on
  the existing tests must not get worse (`tools/frontend_check.py`).
- The roadmap lists the interpreter as a front-end item beside phase 4.
