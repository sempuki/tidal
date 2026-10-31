# Pipeline

The steps for producing one section. Each numbered step ends in a commit, so
the history shows the section being built in order.

```
 1. topic card ──► 2. inputs ──► 3. probe ──► 4. read source ──► 5. classify
                                    ▲                                  │
                                    └────────── new questions ─────────┤
                                                                       ▼
 10. status ◄── 9. rewrite ◄── 8. similarity ◄── 7. questions ◄── 6. draft
```

## 1. Topic card

Add the section file with a title, a one-paragraph description of the
constructs covered, and the "Depends on" list. No rules yet.

## 2. Inputs

Fetch the permissively licensed implementation sources (Clang and EDG)
that the section needs with `tools/fetch_inputs.py`. It downloads pinned files into `inputs/raw/`, writes
sanitized copies to `inputs/clean/`, and updates `inputs/MANIFEST.md` with the
upstream commit, path, license, class, and SHA-256 of each file. The spec writer
reads only `inputs/clean/`.

## 3. Probe

Write small test programs that ask one question each ("does `if` accept a
declaration as its condition?", "is the right operand of `&&` evaluated when
the left is false?"). Run them with `tools/run_tests.py`. Probes that pass
become conformance tests.

Each probe records the spec writer's predicted outcome in `tidal-expect`. The
compilers overrule the prediction when they disagree with it, so background
knowledge never reaches the spec without a test behind it.

## 4. Read source

Read the sanitized parser and semantic-analysis code for the construct in
Clang and EDG, even when the probes agree. GCC is used through its
behavior only. A source reader can summarize GCC's code under `AGENTS.md`.
The goal is to find conditions the probes missed. The most useful thing to
collect is the list of diagnostics the code can emit, because each error
diagnostic usually corresponds to one `reject` rule. Diagnostic message text
is compiler output, so it counts as a clean input.

Source also answers questions that behavior can't. When compilers show no
variation, source can show whether a behavior is fixed by design or only by
current implementation choice. For example, source can show that an option
for strict evaluation order applies to some operators and not others.

Every condition found in source becomes a new probe (back to step 3).

## 5. Classify

Assign each observed behavior a rule class (see `spec-format.md`). Evidence
for `undefined`:

- **Constant evaluation.** Compilers must reject undefined behavior during
  constant evaluation. Running the construct inside a `constexpr` function
  and getting a compile error is strong evidence. The compiler's note
  usually names the cause.
- **Warnings.** Some compilers warn about undefined behavior they can see
  (for example, unsequenced modification). A test can turn the warning into
  an error with `tidal-flags`.
- **Sanitizers.** `-fsanitize=undefined` with recovery off, or
  `-fsanitize=address`, stops the program and names the error. The test expects `run-fail` and matches
  the report with `tidal-error` (decision 0028).
- **Optimizer assumptions.** Code after the construct is removed or folded
  at `-O2`.

Evidence for `unspecified` and `impl-defined` is variation: across compilers,
optimization levels, or targets. A second target such as `-m32` is a cheap
way to show behavior that depends on type sizes.

## 6. Draft

Write the rules and attach tests. Run `tools/run_tests.py --compilers all`
and `tools/check_spec.py`. For every `reject` test, read the errors listed
in `tests/RESULTS.md` and confirm the rejection has the intended cause. A
test that fails to compile for an unrelated reason still passes. The check script fails if a rule has no test, a test
names a missing rule, or a rule ID is duplicated.

Writing guidance:
- One requirement per rule.
- Describe the behavior a program can observe, in plain sentences.
- Use the glossary's terms and add missing ones to it.
- Prefer a short example to a long explanation.

## 7. Questions

For every compiler disagreement or unclear classification, the spec writer adds
a file to `questions/` from `questions/TEMPLATE.md`: the facts, the
evidence, the options, and a provisional answer. The spec follows the
provisional answer, and the section's "Divergences" heading links to the
question. Work continues without waiting for anyone. Contributors change
answers through pull requests (`questions/README.md`).

## 8. Similarity check

Run `tools/similarity.py` against the reference corpus and commit the report
to `reports/similarity/` (`reports/README.md`). Anyone may run it, including
the spec writer, because it prints only Tidal text.

## 9. Rewrite

Rewrite flagged rules, rerun tests and the similarity check, and commit the
new report.

## 10. Status

Update `docs/status.md`: the section's rule and test counts, open
divergences, open similarity flags, and next steps. If the pipeline itself
needs to change, change this document and record the reason in a decision
record.

## Checks on finished pages

These run across pages, outside the per-section steps.

- **Gap review.** A source reader compares Tidal with the reference
  material and reports gaps in prose (decision 0019). Each gap becomes a
  probe.
- **Clean-room exercises.** Readers who use only Tidal's files write
  programs, predict test outcomes, or build the front end in
  `frontend/`, and report where the text was not enough (decisions 0029
  and 0030, `docs/exercises.md`). Each finding becomes a probe or an
  edit, and the similarity check runs on the report first.
