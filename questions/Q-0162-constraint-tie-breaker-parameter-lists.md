# Q-0162: Does the constraint tie-breaker apply when the parameter lists differ beyond the parameters that the call uses?

**Status:** settled
**Affects:** OVL-30, `tests/foundations/overload-resolution/tie-constraints-default-parameter.cpp`, `tests/foundations/overload-resolution/tie-constraints-ellipsis.cpp`

Duplicate of Q-0146.

## Answer

Duplicate of Q-0146. ORD-13 states every condition under which constraints decide, and
OVL-30 and OVL-41 use those conditions. Q-0146 asks the question for
every condition and holds this question's cases in its table, with the
context, the evidence, and the provisional answer.

## Opinions

### Inventory, 2026-10-31

The source-reader inventory of concepts and constraints (`reports/review/2026-10-31-inventory-concepts-and-constraints.md`) names the question in its list of compiler differences that Tidal already records, and gives no reading.
