# 0020. Source readers give writers summaries only

Date: 2026-10-30
Status: Accepted. Amends 0019.

## Context

Decision 0019 let a reviewer who reads the reference material give the
writer facts and self-written test programs. A test program from a
source reader can still carry the form of a reference example, and the
project expects many agents to work on Tidal, so the rule needs to be
public and simple.

## Decision

`AGENTS.md` states the rules for all agents:

- An agent that has read material that can contain protected
  specification text is a *source reader*. It must not write or change
  any file in the repository.
- A source reader gives a writer only a summary in its own words: no code
  of any size, no quoted or paraphrased text, no proposed wording, and no
  section numbers or stable names.
- The writer makes all wording and code itself, from summaries,
  implementation behavior, sanitized sources, and Tidal's own files.
- Summaries pass the similarity check before a writer reads them, and the
  ones a writer uses are committed under `reports/review/`.

## Consequences

- Review reports describe behaviors to probe in prose. The writer writes
  each probe from that description.
- The reviewer in decision 0019 follows these rules. Test programs are no
  longer part of a review report.
