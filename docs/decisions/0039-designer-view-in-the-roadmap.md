# 0039. The designer's view in the roadmap

Date: 2026-10-31
Status: Accepted. Extends 0033.

## Context

The language-designer roadmap (`reports/clean-room/roadmap/designer.md`)
is a fifth view beside the four that decision 0033 used. It agrees with
the repair phase, with fundamental types early, and with overload
resolution as the topic of highest value. It differs in four places:

- It adds cross-page inventories on the Foundations pages. Proposals
  need to cite each step of the usual arithmetic conversions, the
  contexts of implicit conversion, and the full expression. These exist
  only as a glossary entry or across many pages.
- It wants "trivial" defined now. JMP-6, SW-5, and UNI-5 use the word,
  and a definition needs no library type.
- It wants a narrow lexical slice now: keywords, reserved names, and the
  literal suffix grammar. Every proposal that adds a word needs it.
- It would specify instantiation (N2) before deduction (O).

## Decision

- Phase 2 gains item Y (the inventories), item S1 (the definitions of
  trivial special member functions), and item G1 (the narrow lexical
  slice, written as tables to keep word runs short).
- The rest of items G and S stays deferred.
- Phase 4 keeps O before N2. N2 needs outcome sets in the runner, which
  O does not. Q-0118 records the designer's order.
- Propose exercises X1 to X4 enter the catalog and run after item Y.
  Their reports list each rule change in a table.

## Consequences

- `docs/roadmap.md` and `docs/exercises.md` list the new items and
  exercises.
- `docs/status.md` orders the next steps by this roadmap.
