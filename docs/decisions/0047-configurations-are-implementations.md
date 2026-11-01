# 0047. Compiler configurations are implementations

Date: 2026-10-31
Status: Accepted. Amends 0043.

## Context

The standard defines behavior through an abstract machine. The standard
says nothing about optimization. Decision 0043 added the `-O2` report
compilers, and some Evidence text then argued from optimization to a
rule's class, for example "the outcome depends on the optimization
level, so the rule needs no diagnostic". That argument runs the wrong
way. A rule's class comes from the abstract machine, not from what one
optimizer does.

Undefined behavior also stays undefined. A tool that runs programs, such
as a future interpreter, can stop at undefined behavior and name the
rule. A tool can never give such a program a result of its own, because
that result would define what the language leaves undefined.

## Decision

- Each configuration of a compiler, such as `gcc-16` or `gcc-16-o2`, is
  an implementation. The rules constrain every implementation in the
  same way.
- A rule's class never rests on a difference between optimization
  levels. Evidence for a class comes from the behavior of the abstract
  machine as the compilers show it: diagnostics, constant evaluation,
  sanitizers, and agreement or variation across implementations.
- A configuration's result must lie within what the rules allow:
  - a rule that defines a result must hold in every configuration,
  - an `unspecified` rule must contain each configuration's result in
    its set,
  - an `undefined` or `no-diagnostic` rule allows any result.
- When a configuration's result lies outside what the rules allow, the
  rule is wrong or the implementation has a defect. The writer records
  the case as a question and decides from the other evidence.
- Evidence text says that a rule allows a configuration's result. It
  does not say that the result makes the rule's class.

## Consequences

- The Evidence of VIRT-20, INL-7, and ODR-8, `docs/spec-format.md`,
  `docs/pipeline.md`, and decision 0043 follow this wording.
- A test's override for a report compiler names the rule that allows
  the result.
- A run tool, if one is built, has three outcomes: a defined result, a
  result from an unspecified set, or a stop at undefined behavior that
  names the rule. The tool's defined results must match every
  configuration of every compiler.
