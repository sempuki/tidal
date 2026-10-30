# Rules for agents

These rules apply to every agent, human or AI, that works on Tidal. They
keep text from protected sources out of Tidal. Decision 0020 records why.

## Two kinds of agent

A **source reader** is an agent that has read material that can contain
protected specification text. This material includes:

- the ISO C++ standard and its drafts, in any form or rendering,
- WG21 papers,
- cppreference,
- implementation sources and test suites that have not been sanitized
  (`docs/clean-room.md` lists them),
- implementation sources that are not under a permissive license, such
  as GCC, sanitized or not,
- posts and answers that quote any of the above.

A **writer** is an agent that has not read such material.

An agent that reads such material becomes a source reader for the rest of
its session or context. It cannot become a writer again in that session.

## Rules

1. A source reader must not write or change any file in this repository.
   This includes specification text, tests, examples, the glossary,
   questions, tools, and documentation.
2. A source reader gives information to a writer only as a summary.
3. A summary contains only descriptions of language behavior, gaps, and
   doubts, written in the source reader's own words.
4. A summary must not contain:
   - code examples of any size, including expressions from a source,
   - quoted or paraphrased text from a source,
   - proposed wording for Tidal rules,
   - section numbers or stable names from a source.
5. A writer makes all wording and all code itself. It uses only these
   inputs:
   - summaries from source readers,
   - the behavior and diagnostics of implementations,
   - sanitized, permissively licensed implementation sources in
     `inputs/clean/`,
   - Tidal's own text and tests.
6. Before a writer reads a summary, `tools/similarity.py` checks the
   summary against the reference corpus. A summary with a flagged passage
   goes back to the source reader.
7. Summaries that a writer uses are committed under `reports/review/`.
   Each one names the role of the agent that wrote it.

## What a writer can do

A writer can compile and run programs on any implementation, read
compiler diagnostics, read the sanitized sources, and read every file in
this repository. `docs/clean-room.md` gives the full input rules, and
`docs/style-guide.md` gives the rules for writing specification text.
