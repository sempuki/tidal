# Q-0118: Should instantiation (N2) come before deduction (O) in phase 4?

**Status:** settled
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

## Answer

No, O first; phase 4 drafted O before N2 (decision 0050) (option 1). Phase 4 drafted deduction and function templates (O) before
dependent names and instantiation (N2), and both pages exist
(`specification/templates/deduction.md`,
`specification/templates/instantiation.md`). Decision 0050 records
phase 4 as drafted, so the order no longer has an effect.

## Opinions
