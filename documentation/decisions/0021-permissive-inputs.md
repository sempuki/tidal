# 0021. Writers read only permissively licensed sources

Date: 2026-10-30
Status: Accepted. Amends 0003, 0006, and 0020.

## Context

Tidal is under Apache-2.0 WITH LLVM-exception. The writer reads sanitized
implementation source to find conditions that probes miss. Clang and EDG
use the same license as Tidal. GCC uses GPL-3.0-or-later.

## Decision

- The writer reads sanitized source from permissively licensed
  implementations only. These implementations are Clang and
  EDG.
- GCC stays in the compiler matrix. Tidal uses its behavior, diagnostics,
  and output.
- GCC source, and other sources that are not under a permissive license,
  are source-reader material under `AGENTS.md`. A source reader can read
  them and give the writer a summary.
- `tools/inputs.json` and `inputs/MANIFEST.md` list no GCC files. The
  `Inputs:` lines on specification pages name Clang and EDG inputs only.
- `tools/sanitize.py` still runs on every input. Permissively licensed
  sources also quote the standard in comments.

## Consequences

- Evidence that came from GCC source now comes from behavior. Q-0005
  rests on a test that shows GCC and Clang use different operand orders
  for an overloaded `*`.
- A section that needs detail about GCC's design asks a source reader
  for a summary.
