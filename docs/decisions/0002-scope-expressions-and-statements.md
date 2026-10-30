# 0002. Proof of concept covers expressions and statements only

Date: 2026-10-30
Status: Accepted

## Context

A full specification of C++ is thousands of pages. The proof of concept needs
a scope small enough to finish without expert reviewers, but large enough to
exercise the hard parts of the pipeline: undefined behavior, evaluation
order, implementation-defined results, and disagreements between compilers.

## Decision

Cover expressions and statements, as listed in `docs/charter.md`. Concepts
they depend on (types, lookup, overload resolution, classes, templates) are
referenced by name and not specified.

The baseline is C++23 as implemented by Clang and GCC with `-std=c++23`.

## Consequences

- Sections will contain "Depends on" lists that point at concepts Tidal does
  not yet define. That is expected for now.
- C++26 features (contracts, reflection, `#embed`) are excluded, partly
  because implementations are incomplete and partly because the inputs that
  describe them are WG21 papers, which quote ISO text.
