# 0031. Role names: specification writer and clean-room reader

Date: 2026-10-31
Status: Accepted. Amends 0020, 0029, and 0030. Amended by 0032.

## Context

`AGENTS.md` named two kinds of agent, a source reader and a writer.
Decisions 0029 and 0030 added readers who use only Tidal's files, under
the name "spec-only reader". "Writer" alone does not say what the agent
writes, and "spec-only" does not say what the reader has not read.

## Decision

- The *specification writer* writes the specification text, tests, tools, and
  documentation. The rules for it are the rules for the former writer.
- A *clean-room reader* has read no protected material and reads only
  Tidal's own files. It uses the specification as its audience would and
  reports findings. It writes only in its exercise's directories.
- The *source reader* keeps its name and rules.
- The spec-only exercises are the clean-room exercises, and their
  reports move from `reports/spec-only/` to `reports/clean-room/`.

## Consequences

- `AGENTS.md` defines three kinds of agent.
- Living documents use the new names. Earlier decisions and reports keep
  their wording as records.
