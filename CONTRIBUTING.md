# Contributing to Tidal

Tidal is written by many hands, human and AI, under one process. This
guide says which role you take, how to claim work, and what a change
needs before it is merged. The detailed rules live in the files it links
to.

## 1. Pick your role

Read `AGENTS.md` first. The role you can take depends on what you have
read.

| Role | Has read | Can do |
|---|---|---|
| **Writer** | No ISO text, WG21 papers, cppreference, or unsanitized implementation sources | Write and change specification text, tests, tools, and docs |
| **Source reader** | Any of the above | Give writers summaries in prose: gaps, doubts, behaviors to probe. No code, no quotes, no stable names. Never change a file |
| **Contributor** | Anything | Argue for an answer to a question in `questions/` from evidence anyone can check |

A source reader stays a source reader for the rest of that session or
context. If you are not sure which role you have, you are a source
reader.

## 2. Ways to help

- **Weigh in on a question.** Each file in `questions/` has a provisional
  answer that the specification follows. Add an opinion with evidence:
  a test program, compiler output, or permissively licensed source. See
  `questions/README.md`.
- **Report a gap.** As a source reader, write a summary under the rules
  in `AGENTS.md`. A writer checks it with `tools/similarity.py` and
  triages it into probes (decision 0019).
- **Use the spec on its own.** Write programs, or extend the front end
  in `frontend/`, from the spec pages alone, and report where the text
  was not enough (decision 0029). This works for any role, provided you
  read only Tidal's files while you do it.
- **Draft or extend a page.** As a writer, follow the pipeline below.
- **Improve the tooling.** The runner, the checks, and the site build are
  in `tools/` and `site/`.

## 3. Claim work

Open an issue that names the page or the topic and the steps you will
take. One person or agent owns a page at a time. When several agents
work at once, each owns its own pages and test directories and does not
commit; one integrator numbers questions and commits (decision 0018).

## 4. The pipeline for a page

`docs/pipeline.md` gives every step. In short:

1. **Topic card.** The page file with a title, a summary, and a
   "Depends on" line.
2. **Inputs.** Fetch any sanitized, permissively licensed implementation
   sources the topic needs with `tools/fetch_inputs.py`.
3. **Probes.** Small test programs, each with a predicted result in
   `tidal-expect`. Run them on the matrix. The compilers decide.
4. **Source reading.** Read the sanitized diagnostics and code for
   conditions the probes missed, and probe those.
5. **Classify** each behavior (`docs/spec-format.md`).
6. **Draft.** Principles first, then syntax, then rules in their
   segments (`docs/style-guide.md`). Every rule names its tests, and
   every rule is cited by a principle.
7. **Questions.** Each divergence between compilers gets a file in
   `questions/` with a provisional answer.
8. **Similarity.** Run `tools/similarity.py` against the reference
   corpus. Nothing may be flagged.
9. **Rewrite** any flagged unit and check again.
10. **Status.** Update `docs/status.md` and `tests/RESULTS.md`.

Each step ends in its own commit, so the history shows how the page was
built.

## 5. Before you push

Run `tools/check_all.sh`. It runs the spec cross-reference check, the
full test matrix, and the site build. The similarity check needs a local
copy of the reference corpus outside the repository; see
`docs/clean-room.md`. CI runs all of these on every push and pull
request.

A change is ready to merge when:

- `tools/check_spec.py` reports no problems,
- every test passes on the baseline compilers, or carries a per-compiler
  expectation with a note that names its question,
- the similarity check flags nothing,
- `tools/build_site.py --strict` succeeds,
- each commit message says what changed and why.

## 6. Writing

Write in the plain style of `docs/style-guide.md`: one statement per
sentence, the active voice, the word list for requirements, and no
cleft sentences, colon reveals, or dashes for asides. Write each
principle and rule from the tests and the compilers' behavior. Do not
copy or paraphrase any other specification.

## 7. License

Contributions are under Apache-2.0 WITH LLVM-exception (`LICENSE.txt`).
