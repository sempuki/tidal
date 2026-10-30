# Clean-room protocol

Tidal must describe the same language as ISO C++ without copying the ISO text.
This document says who may read which material and how the output is checked.

## Roles

**Writer.** Drafts rules and tests. In the target setup this is an LLM pipeline
plus human reviewers who have not studied the ISO text.

**Checker.** Has read the ISO text. Runs the similarity check against it and
reports results to the writer. The checker never supplies wording.

**Adjudicator.** Decides what Tidal says when implementations disagree. Works
only from evidence the writer can also see (test programs and compiler
output), and records the evidence with the decision.

One person may hold the checker and adjudicator roles. Nobody may be both
writer and checker on the same section.

## Inputs

Every input belongs to one of three classes. The class of each downloaded
input is recorded in `inputs/MANIFEST.md`.

### Clean

The writer may use these directly.

- Behavior of an implementation: whether a program compiles, its diagnostics,
  its output, and the result of constant evaluation.
- Tidal's own specification and tests.
- General knowledge of programming that does not come from the ISO text.

### Sanitized

The writer may use these only after they pass through `tools/sanitize.py`,
which removes every comment and every string that cites the standard.

- Implementation source code (Clang `lib/Parse`, `lib/Sema`, `lib/AST`; GCC
  `gcc/cp`).
- Implementation test files outside the directories listed under Forbidden.

Comments are removed because implementations quote the standard in comments,
often a full paragraph at a time. Code identifiers are kept even when they
echo standard terms (`isLValue`, `CheckConditionalOperands`).

### Forbidden

The writer must not read these.

- The ISO standard, its working drafts, and any rendering of them
  (eel.is/c++draft, timsong-cpp.github.io, the `cplusplus/draft` sources).
- WG21 papers. Their wording sections are diffs against the ISO text.
- cppreference.com. It paraphrases the standard closely and is licensed
  CC-BY-SA, which would force that license on Tidal.
- Clang's `clang/test/CXX/` and `clang/test/CXX/drs/` directories, and libc++'s
  `libcxx/test/std/`. They are organized by ISO section and quote it.
- Language-lawyer answers on Q&A sites, which usually quote the standard.

## Similarity check

The checker runs `tools/similarity.py` with the Tidal spec and a local copy of
the ISO draft. The draft copy lives outside this repository and is never
committed.

The tool reports, for each Tidal rule, the longest run of words it shares with
the draft and an overall overlap score. It prints only Tidal text and Tidal
rule IDs. It never prints draft text, so the report can be handed to the
writer without contaminating them.

A rule is flagged when it shares a run of 8 or more words with the draft
(after normalization) or its overlap score exceeds the threshold in the tool.
The writer rewrites flagged rules without help from the checker, and the
checker runs the tool again. Both runs are committed.

## Records

For each section the repository holds:

- the input manifest entries the writer used,
- the rules and tests, committed before the similarity check,
- the similarity report,
- any rewrites, committed separately with the report that triggered them,
- adjudication notes for every compiler disagreement.

## Known limitations of the proof of concept

This proof of concept cannot meet the protocol, and the results must be read
with that in mind.

1. **The writer is an LLM trained on the ISO text.** Claude has almost
   certainly seen the working drafts during training. It can follow the input
   rules above, but it cannot unlearn what it has read. The similarity check is
   the only control on this.
2. **The author is a WG21 participant.** The author has read the ISO text
   closely and acts as checker and adjudicator. They can't also serve as an
   untainted reviewer of the writer's output.
3. **Shared disk.** The repository sits next to WG21 paper drafts. The writer
   follows a rule not to read outside `tidal/` and `inputs/`.

A real run needs writers and reviewers who have not studied the ISO text, and
ideally a model whose training data excluded it.
