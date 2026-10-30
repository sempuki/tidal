# 0018. Order of work for expressions

Date: 2026-10-30
Status: Accepted. Extends 0002.

## Context

The charter puts expressions in scope, but only the multiplicative
operators have a page. Statements, declarations, and classes all depend on
expressions. The operator families are independent of each other and have
the same shape as the multiplicative page.

## Decision

Expressions come before the remaining class sections. The pages are:

1. Foundations, written first: value categories and temporary
   materialization, standard conversions, and sequencing and evaluation
   order.
2. Operator families, written in parallel: additive (with pointer
   arithmetic), shifts, comparisons, bitwise, logical, conditional,
   assignment and compound assignment, and comma.
3. Then: unary operators, postfix operators (subscript, call, member
   access, increment), casts, constant expressions, `new` and `delete`,
   and lambdas.

Several forks may draft operator families at the same time. Each works in
its own test directory and spec page, and does not commit. Questions and
status are integrated by the main writer, who commits each page's probes
and draft as separate pipeline steps.

## Consequences

- Classes resume after the operator families, at access control.
- Operator pages refer to the foundations pages by rule ID once they
  exist.
