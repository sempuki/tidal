# 0032. The clean room is shared

Date: 2026-10-31
Status: Accepted. Amends 0031.

## Context

Decision 0031 named the writer the *spec writer* and the readers who use
only Tidal's files the *clean-room readers*. Both roles have read no
protected material, so both work in the same clean room. Only one of
the two names said so.

## Decision

- The *clean room* is the work space shared by the agents that have not
  read protected material.
- The *clean-room writer* (0031's spec writer) writes the specification
  text, tests, tools, and documentation.
- The *clean-room reader* reads only Tidal's own files and reports
  findings to the clean-room writers.
- A *source reader* is outside the clean room. An agent that becomes a
  source reader cannot enter it again in that session.

## Consequences

- `AGENTS.md` describes the clean room and its two roles. Living
  documents use the new names.
