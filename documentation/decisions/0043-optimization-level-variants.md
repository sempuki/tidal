# 0043. Optimization-level variants in the matrix

Date: 2026-10-31
Status: Accepted. Extends 0008 and 0028. Amended by 0047.

## Context

Roadmap items I and J (use of an entity and the one-definition rule)
have many `no-diagnostic` and `unspecified` rules. Whether a program
with such a fault links or runs can differ between configurations of
one compiler.
The baseline compilers build every test at the default level, so the
matrix shows one outcome.

## Decision

- `tools/compilers.json` has two `report` compilers, `gcc-16-o2` and
  `clang-22-o2`. Each one is its base compiler with `-O2` before the
  test's flags. A report compiler does not fail a run, and the
  agreement sentences count only baseline compilers.
- A compiler entry can name `variant_of`, the compiler whose
  per-compiler expectations and error patterns it uses when a test has
  none for the variant itself.
- A test gives the variant's outcome with `tidal-expect-<variant id>`
  when the outcome differs at `-O2`. The test's note names the rule
  that allows both outcomes.
- A sanitizer test passes `-O0` first in `tidal-flags`. The later flag
  wins, so the variants build the test without optimization.

## Consequences

- These tests have a different outcome at `-O2`:

  | Test | Rule | At `-O2` |
  |---|---|---|
  | `classes/virtual/virtual-not-defined.cpp` | VIRT-20 | links |
  | `declarations/inline/inline-in-one-unit-only.cpp` | INL-7 | does not link |
  | `foundations/evaluation-order/implicit-calls.cpp` | NEW-14 | Clang omits a `new` and `delete` pair |
  | three sanitizer tests | NEW-5, OBJ-2, LAM-19 | the sanitizer reports nothing |

  The first two tests have overrides for the variants. The third test
  accepts the omitted pair, as NEW-14 requires. The sanitizer tests
  pass `-O0`.
- A page can cite a variant's outcome in Evidence as one more
  implementation result that its rules must allow (decision 0047).
