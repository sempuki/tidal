# Q-0116: Should a designer persona that knows WG21 papers also run, as a source reader?

**Status:** provisional
**Affects:** decision 0037, `documentation/exercises.md`

## Context

Decision 0037 adds a language-designer persona that takes all of its
C++ knowledge from Tidal. A designer who has read many WG21 papers would
know which design questions proposals usually raise. AGENTS.md makes
such a reader a source reader.

## Evidence

AGENTS.md rules 1 to 4. Decision 0029, which keeps clean-room readers to
Tidal's own files.

## Options

1. Run only the clean-room designer. Every gap it meets is a finding.
2. Also run a source-reader designer. The source-reader designer can say
   which kinds of design questions proposals usually raise that Tidal
   cannot answer. Its report is a summary under rules 2 to 4, checked
   for similarity, with no code, no wording, and no paper names.

## Provisional answer

No, the clean-room designer only for now (option 1). The clean-room designer finds gaps without a protected input.
Option 2 can follow if the roadmap of the clean-room designer misses
whole areas.

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`, row 9).

## Opinions
