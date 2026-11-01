# 0033. A roadmap from four clean-room views

Date: 2026-10-31
Status: Accepted. Provisional on Q-0113 for the scope of phases 3 and 4.

## Context

Every topic in the charter's scope has a page. The clean-room exercises
found that the existing pages have contradictions, rule classes that do
not say what to diagnose, and a grammar that cannot drive a parser, and
that most test outcomes depend on topics the charter lists as
referenced. Four clean-room agents planned the next work from Tidal's own
files: a programmer, an implementer, an editor, and a clean-room writer
(`reports/clean-room/roadmap/`).

## Decision

- `documentation/roadmap.md` orders the work in four phases: repair what exists;
  foundations that later pages need (types and targets, grammar,
  language-support types); lookup and overload resolution; templates.
  Parallel lanes and deferred items are listed there.
- Phases 3 and 4 move name lookup, overload resolution, templates, the
  language-support types, and a minimal `#include` slice into scope,
  provisionally. The charter changes when each item starts, with its
  own decision.
- Items L, M, O, and N2 follow the method controls on the roadmap page.
- `documentation/status.md` points its next steps at the roadmap.

## Consequences

- Q-0113 records the open choice between normative and informative
  treatment of overload resolution and templates.
- The writer's prerequisites (probe grid, runner outcome sets, an
  optimization-level variant, new inputs, check_specification checks) become
  work items in the phases that need them.
