# Q-0412: Which effects can a standard attribute have, and can a new one add a required rejection?

**Status:** provisional
**Affects:** ATTR-P1, ATTR-P2, ATTR-P4
**Source:** finding X22-6 of `reports/clean-room/X22/report.md`, and the designer view of `reports/clean-room/roadmap-8/designer.md`, section 1.3.
**Waits for:** the maintainer (pending.md row 9)

## Context

ATTR-P2 says that a standard attribute does not change what a valid
program does. ATTR-P4 says that the other attributes turn warnings on
or off, and that a program that ignores them stays valid. Neither says
whether a standard attribute can make a program not valid outside its
own placement rules. Propose reports X12, X14, X18, and X22 each
reasoned from these principles to choose or rule out an attribute as
the marker of a design. X18 ruled out an attribute for a feature that
only adds a rejection, and X22 chose one for a feature of the same
kind.

The question is a design question about Tidal's model of attributes.
No program and no compiler result decides it.

## Evidence

Propose reports X12, X14, X18, and X22, and the finding X22-6.

## Options

1. A standard attribute has these effects only: a placement rule, a
   warning, a promise that the implementation may rely on, and a
   layout change. A new attribute does not add a required rejection
   outside its placement rules.
2. A new standard attribute may add a required rejection, as a check
   that the program asks for.

## Provisional answer

Placement rules, warnings, promises, and layout changes only (option
1). This keeps ATTR-P2 as it is: ignoring an attribute does not change
the meaning of a valid program. The maintainer decides it, because it
fixes a design choice that Propose reports rely on.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
