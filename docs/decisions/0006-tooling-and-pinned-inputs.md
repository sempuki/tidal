# 0006. Tooling and pinned inputs

Date: 2026-10-30
Status: Accepted

## Context

The pipeline needs repeatable inputs and mechanical checks.

## Decision

Four tools, all standard-library Python:

- `tools/fetch_inputs.py` downloads files listed in `tools/inputs.json` at a
  pinned tag, sanitizes them, and writes `inputs/MANIFEST.md` with a SHA-256
  of each raw file. Only the manifest is tracked in git.
- `tools/sanitize.py` removes comments (keeping string and raw-string
  literals) and redacts bracketed section citations left in code.
- `tools/run_tests.py` compiles each test with `clang++` and `g++` at
  `-std=c++23 -pedantic-errors`, runs `run` tests, and can write
  `tests/RESULTS.md`.
- `tools/check_spec.py` cross-checks rule IDs and test references.
- `tools/similarity.py` is the checker's tool. It refuses a reference corpus
  inside the repository and prints only Tidal text.

Pinned versions match the compilers installed on the author's machine:
LLVM `llvmorg-22.1.8` and GCC `releases/gcc-16.2.0` (the installed GCC is
Fedora's 16.2.1 prerelease build).

## Consequences

- Anyone can reproduce the writer's inputs from the manifest.
- Committed source would be large and is easy to refetch, so it stays out
  of git.
- The Fedora GCC build is slightly ahead of the pinned source tag. Behavior
  differences from that gap are unlikely to matter for this scope.
