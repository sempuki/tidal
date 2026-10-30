# Journal

Dated entries describing what was done and what was learned. Decisions get
their own records in `decisions/`. This file holds the narrative.

## 2026-10-30: Project start

Started Tidal as a proof of concept for producing a C++-compatible
specification from open source implementations with LLM assistance.

The author (a WG21 participant) and an LLM assistant (Claude) are the only
participants. Both have prior exposure to the ISO text, so this run cannot be
a true clean room. The protocol in `clean-room.md` is written for a future
team, and this proof of concept follows it as closely as two tainted
participants can. The point is to test the mechanics and measure how close
the output lands to ISO wording.

Created the repository (decision 0001) and fixed the scope (decision 0002).

Wrote the clean-room protocol (`clean-room.md`, decision 0003) and the
terminology policy (decision 0004). The protocol bans the two best-organized
test suites (Clang `test/CXX`, libc++ `test/std`) because they are arranged by
ISO section and quote it. Tidal will have to write its own tests, which is
probably the largest cost of the whole approach.

Wrote `spec-format.md` and `pipeline.md` (decision 0005). The pipeline uses
compiler diagnostics as the main checklist for `reject` rules, since a
diagnostic's message is compiler output and is a clean input. Constant
evaluation is the main evidence for `undefined`, because compilers must
reject undefined behavior during constant evaluation.
