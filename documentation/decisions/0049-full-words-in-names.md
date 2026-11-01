# 0049. Full words in names and prose

Date: 2026-10-31
Status: Accepted.

## Context

The repository used the abbreviations "spec" and "docs" in directory
names, file names, a tool name, and prose. Tidal's writing rules ask
for plain words, and a reader new to the project meets these names
first.

## Decision

- The directories are `specification/` and `documentation/`.
- File and tool names use the full words:
  `documentation/specification-format.md`,
  `tools/check_specification.py`,
  `documentation/decisions/0029-specification-only-readers.md`, and
  `guides/reading-the-specification.md`.
- Prose writes "specification" and "documentation" in full.
- Command-line flags can stay short, for example the `--spec` flag of
  `tools/similarity.py`.
- Reports under `reports/` are dated records and keep the words and
  paths they were written with.

## Consequences

- Links, tools, the site build, CI, and the front end's comments use
  the new paths.
