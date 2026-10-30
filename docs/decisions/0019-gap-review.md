# 0019. Gap review by a separate reviewer

Date: 2026-10-30
Status: Accepted. Amends 0003. Amended by 0020.

## Context

The pipeline finds behavior from implementations, so it can miss whole
behaviors that no probe asks about. A reviewer who compares Tidal with the
reference material can point out those gaps. The writer must not read that
material (`docs/clean-room.md`).

## Decision

- A *reviewer* is a separate role. The reviewer reads the reference
  material and Tidal, and reports suggestions to the writer.
- A review report contains only facts about the language in the reviewer's
  own words, test programs that the reviewer writes, and the IDs of Tidal
  rules that seem incomplete or wrong. It contains no quoted or
  paraphrased reference text, no reference examples, no section numbers or
  stable names, and no proposed wording.
- Before the writer uses a report, `tools/similarity.py` checks it against
  the reference corpus. A report with a flagged passage goes back to the
  reviewer.
- The writer treats each suggestion as a question for the pipeline. A
  suggested behavior becomes a probe, and the compilers decide it as for
  any other probe.
- Reports are committed under `reports/review/`.

## Consequences

- Gaps that probes miss can be found without the writer reading the
  reference.
- The reviewer and the writer must be different agents or people.
