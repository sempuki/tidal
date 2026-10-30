# 0010. Open questions are files with provisional answers

Date: 2026-10-30
Status: Accepted

## Context

The pipeline raises questions that evidence alone doesn't settle:
compiler divergences, borderline classifications, and process choices.
Tidal has no standing body to decide them, and work must not stop while
they are open. The project expects many contributors, who need a way to
disagree that leaves a record.

## Decision

- Each question is a Markdown file in `questions/`, numbered `Q-NNNN`, with
  context, evidence, options, a provisional answer, and an "Opinions"
  section.
- The writer sets the provisional answer when raising the question. The spec
  follows it immediately.
- Contributors add opinions and propose new answers through pull requests.
  A pull request that changes an answer also updates the affected rules and
  tests, and the baseline tests must pass.
- Opinions argue from evidence anyone can check. Quoting or paraphrasing the
  ISO text, WG21 papers, or cppreference is not allowed.
- `tools/check_spec.py` fails when a spec, test, or document refers to a
  question with no file.

## Consequences

- The adjudicator role in decision 0003 is replaced by provisional answers
  plus pull requests.
- Every provisional answer can change. Rules that depend on one link to it.
- Git history and pull request discussion record why each answer changed.
