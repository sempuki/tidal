# Tidal

Tidal is an open specification of a programming language intended to stay
source-compatible with ISO/IEC 14882 (C++). The specification text is written
independently. It is derived from the observable behavior of open source C++
implementations and their test suites, never from the ISO text.

**Status:** proof of concept. The scope is limited to expressions and statements.
See [docs/charter.md](docs/charter.md).

## Layout

| Path | Contents |
|---|---|
| `docs/charter.md` | Goals, scope, and non-goals |
| `docs/clean-room.md` | Who may read what, and how contamination is checked |
| `docs/spec-format.md` | How rules and tests are written |
| `docs/pipeline.md` | The step-by-step process for producing a section |
| `docs/decisions/` | One file per design decision, numbered in order |
| `docs/journal.md` | Dated log of what was done and what was learned |
| `spec/` | The Tidal specification text |
| `tests/` | Conformance tests, each tied to one or more rules |
| `tools/` | Test runner, input fetcher, similarity checker |
| `inputs/` | Downloaded implementation sources (ignored by git, see `inputs/MANIFEST.md`) |

## History as a record

This repository's git history is part of the project's provenance record.
Each design decision and each pipeline run is committed separately with a
message that says what was done and why. Don't squash or rewrite history.
