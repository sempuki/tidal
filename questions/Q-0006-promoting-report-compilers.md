# Q-0006: When does a report compiler become a baseline compiler?

**Status:** provisional
**Affects:** `tools/compilers.json`, decision 0008

## Context

Baseline compilers decide whether tests pass. Report compilers (EDG 7.0,
GCC 14, Clang 18) only show results. Promoting a compiler means its
divergences block changes until expectations or questions account for them.

## Options

1. Promote after one review in which every difference is either fixed in
   the tests or recorded as a question.
2. Keep EDG as report until its divergences (Q-0002, Q-0003) are settled.
3. Promote only the newest version of each implementation.

## Provisional answer

Option 1. GCC 14 and Clang 18 already have no differences and can be
promoted. EDG can be promoted once its differing tests carry per-compiler
expectations that reference Q-0002 and Q-0003.

## Opinions
