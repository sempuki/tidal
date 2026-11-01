# Q-0113: Should Tidal specify overload resolution and templates in full?

**Status:** settled
**Affects:** `documentation/roadmap.md` phases 3 and 4, `documentation/charter.md`

## Context

The charter lists name lookup, overload resolution, and templates as
referenced but not specified. The clean-room front end and the
programmer, implementer, and writer roadmaps found that these topics
decide most test outcomes and stop ordinary programs. The editor's
roadmap argues for deepening the existing pages first, and for only
informative subsets of overload resolution and templates.

## Options

1. Specify name lookup, overload resolution, and templates as normative
   chapters, in the order of the roadmap.
2. Write informative notes that cover only what the in-scope rules and
   the tests rely on, and keep the topics referenced.
3. Keep the topics referenced, and state for each affected rule that the
   rule cannot be decided without these topics.

## Answer

Option 1. Decision 0033 adopted the order, and phases 3 and 4 drafted
name lookup, overload resolution, and templates as normative chapters
(`specification/foundations/` and `specification/templates/`). Decision
0050 starts phase 5 from that state.

## Opinions
