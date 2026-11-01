# Q-0118: Should instantiation (N2) come before deduction (O) in phase 4?

**Status:** provisional
**Affects:** `documentation/roadmap.md`, phase 4

## Context

The roadmap orders phase 4 as template declarations (N1), deduction and
function templates (O), dependent names and instantiation (N2), concepts
(P), and packs (R). The designer's roadmap would put N2 before O. Some
of the designer's proposals are instantiations with no deduction, such
as an expansion over the elements of an object or a constant parameter.

## Evidence

`reports/clean-room/roadmap/designer.md`, ranked topic 4.
`reports/clean-room/roadmap/writer.md`, the prerequisites of O and N2.

## Options

1. Keep O before N2. N2 needs outcome sets in the test runner, and O
   needs overload resolution (M), which comes first anyway.
2. Put N2 before O, and add runner outcome sets before phase 4.

## Provisional answer

Option 1. The order follows the prerequisites. The question comes back
when phase 4 starts.

## Opinions
