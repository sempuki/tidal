# Clean-room protocol

Tidal must describe the same language as ISO C++ without copying the ISO text.
This document says who may read which material and how the output is checked.

## Roles

**Writer.** Drafts rules and tests. In the target setup this is an LLM pipeline
plus human reviewers who have not studied the ISO text.

**Checker.** Compares Tidal text with the reference corpus. This role is
filled by `tools/similarity.py` (decision 0011). The tool reads the corpus
and reports only Tidal text, so anyone may run it, including the writer.
Nobody needs to read the corpus.

**Contributors.** Decide what Tidal says when implementations disagree or a
classification is unclear. Each such question is a file in `questions/`
with a provisional answer that the spec follows. Contributors change answers
through pull requests that argue from evidence anyone can check (test
programs, compiler output, implementation source), as described in
`questions/README.md` (decision 0010).

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

`tools/similarity.py` runs with the Tidal spec and a local copy of the ISO
working draft (WG21 document N4950, the C++23 draft, from the
`cplusplus/draft` repository). The copy lives outside this repository and is
never committed or opened.

The tool reports, for each Tidal rule, the longest run of words it shares with
the draft and an overall overlap score. It prints only Tidal text and Tidal
rule IDs, never draft text.

A rule is flagged when it shares a run of 8 or more words with the draft
(after normalization) or its overlap score exceeds the threshold in the tool.
The writer rewrites flagged rules and runs the tool again. Both reports are
committed.

## Records

For each section the repository holds:

- the input manifest entries the writer used,
- the rules and tests, committed before the similarity check,
- the similarity report,
- any rewrites, committed separately with the report that triggered them,
- a question in `questions/` for every compiler disagreement or unclear
  classification.

## Staffing

| Role | Filled by |
|---|---|
| Writer | Claude (an LLM from Anthropic), working only inside this repository and `inputs/clean/` |
| Checker | `tools/similarity.py` |
| Contributors | Anyone, through pull requests to `questions/` and the spec |
