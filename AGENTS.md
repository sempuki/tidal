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

## Working practices

These practices apply to every agent that changes the repository.

- **Keep going.** Work through `docs/status.md` next steps in order.
  When a step ends, start the next one.
- **Do not wait for decisions.** When the compilers disagree or a
  classification is unclear, adopt the writer's answer as provisional,
  record it as a question in `questions/`, and continue. Ask the
  maintainer only before an action that is new in kind and reaches
  outside the repository, such as contacting another project.
- **Commit and push each step.** One commit per pipeline step or
  decision, pushed when it is made. The history is the record of the
  process. Do not squash or rewrite it.
- **Documents state the present.** Status pages, decisions, and Evidence
  segments say where the project is and what comes next. The story of
  how a change came about goes in its commit message. Name roles
  (maintainer, writer, source reader, contributor), not people.
- **Describe the process plainly.** Documents and commit messages record
  inputs, tools, roles, and steps as facts. They do not evaluate them.
- **One integrator.** When several agents work at once, each writes only
  in its own files and does not commit. One integrator numbers questions
  and decisions and makes the commits (decision 0018). A subagent that
  cannot write its report returns it as text, and the integrator saves
  it.
- **Check reports before use.** A review summary or a spec-only report
  passes `tools/similarity.py` before a writer acts on it (decisions
  0019, 0029).

### Shell habits

Each of these habits prevents a commit that claims more than it did.

- Chain a check and its commit with `&&`. A `;` anywhere between them
  lets a failed step through.
- Do not pipe a check into `tail` or `grep` before `&&` unless
  `set -o pipefail` is on, because a pipe hides the check's exit code.
- Write commit messages to a file and use `git commit -F FILE`. Two
  heredocs on one command line feed their bodies to the wrong commands.
- Never run two `tools/run_tests.py` processes at once. They share
  binaries under `build/`, and one fails with "Text file busy".
- When a commit describes an edit made by a script, check
  `git show --stat` before pushing. If a commit message turns out to be
  wrong, make a follow-up commit that says what the first one missed.
