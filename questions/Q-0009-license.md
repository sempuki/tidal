# Q-0009: Which license covers the specification, tests, and tools?

**Status:** settled
**Affects:** the whole repository

## Context

A repository needs a license before anyone may copy, modify, or
redistribute it, and before contributions have stated terms. The
repository owner chooses it.

The project mixes three kinds of material:

- specification prose (`spec/`, `docs/`, `questions/`),
- test programs (`tests/`),
- tools (`tools/`, `docker/`).

Implementation sources that the tools download keep their own licenses
(Apache-2.0 with LLVM exception for LLVM and EDG, GPL-3.0-or-later for GCC)
and are not committed.

## Options

1. Apache-2.0 with LLVM exception for everything. Matches LLVM and EDG.
2. CC-BY-4.0 for prose, and Apache-2.0 (or MIT) for tests and tools.
3. CC0 or another public-domain dedication for everything, so the
   specification can be adopted anywhere without conditions.

## Answer

Option 1, added by the repository owner in `LICENSE.txt`. Contributions are
accepted under the same license. The specification, tests, and tools share
one license with LLVM and EDG, the main implementations Tidal draws on.

## Opinions
