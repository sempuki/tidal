# 0029. Spec-only readers and a spec-only front end

Date: 2026-10-31
Status: Accepted. Extends 0019. Amended by 0035.

## Context

The pipeline recovers the language from implementations (probes, source
reading, sanitizers) and finds missing behavior through gap reviews.
None of these techniques shows whether the text is enough on its own: whether a
reader can write correct programs from it, or an implementer can build
a checker from it. A specification that every reviewer reads alongside
other material can lean on knowledge it never states.

## Decision

Two more techniques check the specification by using it.

- **Spec-only programmer.** An agent takes the role of an experienced
  programmer in another statically typed language who has never written
  C++. It writes small programs from the spec pages alone, builds them
  with the baseline compilers and the sanitizers, and reports each place
  where the text was missing, unclear, wrong, or hard to find. It logs
  every point where it needed knowledge the spec did not give.
- **Spec-only front end.** An agent builds a small C++ front end in
  Rust (`frontend/`, standard library only) from the spec pages, using
  the Tidal test suite as its conformance suite. It reports the grammar
  gaps, ambiguities, and untestable rules it meets, and the cases where
  a test's expectation disagrees with its reading of the text.

Both readers read only Tidal's own files (`spec/`, `docs/`, and for the
front end `tests/` and `questions/`). They use no web access, no ISO
text or drafts, no WG21 papers, no cppreference, and no compiler source.
Their feedback cites Tidal rule and principle IDs, never ISO wording or
section names. Topics the charter leaves out (lexical structure, name
lookup, overload resolution, templates) are logged as "outside the
spec" findings, and those findings help decide what to specify next.

Reports go under `reports/spec-only/`. They pass the similarity check
before a writer uses them, as review summaries do (decision 0019). A
writer treats each finding as a question for the pipeline, and a probe
decides any claimed behavior.

## Consequences

- The first spec-only programmer has a Java background.
- `frontend/` is a new part of the repository. Its conformance numbers
  show how much of the test suite a reader of the text can decide.
- The readers' agents write only in their own directories and do not
  commit (decision 0018).
