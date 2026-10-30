# Q-0009: Which license covers the specification, tests, and tools?

**Status:** open
**Affects:** the whole repository

## Context

The repository has no license file, so no one has permission to copy,
modify, or redistribute the work, and contributions have no stated terms.
The repository owner adds the license.

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

## Provisional answer

None. A license only takes effect when the repository owner adds it.

## Opinions
