# Q-0160: Do constraints reduce an overload set whose address has no target type?

**Status:** settled
**Affects:** OVL-20, `tests/foundations/overload-resolution/address-no-target-constraints.cpp`, `tests/foundations/overload-resolution/address-no-target-more-constrained.cpp`

Duplicate of Q-0142.

## Answer

Duplicate of Q-0142. Q-0142 asks the same question with a target type. OVL-20 uses the steps
of OVL-19, so one answer covers both. Q-0142 holds the context, the
evidence, and the provisional answer.

## Opinions

### Inventory, 2026-10-31

The source-reader inventory of concepts and constraints (`reports/review/2026-10-31-inventory-concepts-and-constraints.md`) names the question in its list of compiler differences that Tidal already records, and gives no reading.
