# 0027. A class for conditionally supported constructs

Date: 2026-10-31
Status: Accepted. Amends 0005 and 0016.

## Context

Four rules described a construct that one implementation rejects and
another accepts and documents: the extended floating-point suffixes, an
out-of-range floating literal, a multi-unit character in a plain
character literal, and a non-trivial class passed to an ellipsis
parameter. They were classed `impl-defined` with a sentence saying that
an implementation can reject the construct. "May refuse" and "must
choose and document" are different promises, and a reader who knows
the ISO vocabulary looks for the difference.

## Decision

- The class `conditionally-supported` means: an implementation can
  reject the construct, and an implementation that accepts it documents
  the behavior. Its rules live in `Implementation Requirements`.
- The translation page lists the class with the others, and gives an
  informative table from the ISO vocabulary to Tidal's words and
  classes.
- The specification keeps its own words in rule text. It does not use
  *shall*, *may*, or *ill-formed*.

## Consequences

- PRIM-15, PRIM-22, PRIM-23, and PFX-15 change class. Their tests do
  not change.
- `check_spec`, the site, and the status table know the class.
