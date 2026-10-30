# Pipeline

The steps for producing one section. Each numbered step ends in a commit, so
the history shows the section being built in order.

```
 1. topic card ──► 2. inputs ──► 3. probe ──► 4. read source ──► 5. classify
                                    ▲                                  │
                                    └────────── new questions ─────────┤
                                                                       ▼
 10. journal ◄── 9. rewrite ◄── 8. similarity ◄── 7. adjudicate ◄── 6. draft
```

## 1. Topic card

Add the section file with a title, a one-paragraph description of the
constructs covered, and the "Depends on" list. No rules yet.

## 2. Inputs

Fetch the implementation sources the section needs with
`tools/fetch_inputs.py`. It downloads pinned files into `inputs/raw/`, writes
sanitized copies to `inputs/clean/`, and updates `inputs/MANIFEST.md` with the
upstream commit, path, license, class, and SHA-256 of each file. The writer
reads only `inputs/clean/`.

## 3. Probe

Write small test programs that ask one question each ("does `if` accept a
declaration as its condition?", "is the right operand of `&&` evaluated when
the left is false?"). Run them with `tools/run_tests.py`. Probes that pass
become conformance tests.

## 4. Read source

Read the sanitized parser and semantic-analysis code for the construct. The
goal is to find conditions the probes missed. The most useful thing to
collect is the list of diagnostics the code can emit, because each error
diagnostic usually corresponds to one `reject` rule. Diagnostic message text
is compiler output, so it counts as a clean input.

Every condition found in source becomes a new probe (back to step 3).

## 5. Classify

Assign each observed behavior a rule class (see `spec-format.md`). Evidence
for `undefined`:

- **Constant evaluation.** Compilers must reject undefined behavior during
  constant evaluation. Running the construct inside a `constexpr` function
  and getting a compile error is strong evidence.
- **Sanitizers.** `-fsanitize=undefined` reports at run time.
- **Optimizer assumptions.** Code after the construct is removed or folded
  at `-O2`.

Evidence for `unspecified` and `impl-defined` is variation: across compilers,
optimization levels, or targets.

## 6. Draft

Write the rules and attach tests. Run `tools/run_tests.py` and
`tools/check_spec.py`. The check script fails if a rule has no test, a test
names a missing rule, or a rule ID is duplicated.

Writing guidance:
- One requirement per rule.
- Describe the behavior a program can observe, in plain sentences.
- Use the glossary's terms and add missing ones to it.
- Prefer a short example to a long explanation.

## 7. Adjudicate

For each test where Clang and GCC disagree, the adjudicator adds a note under
the section's "Divergences" heading: the test, what each compiler does,
the decision, and the reason. The decision may be that the behavior is
`impl-defined`, that one compiler has a bug (with a bug tracker link if one
exists), or that the question stays open.

## 8. Similarity check

Run by the checker, per `clean-room.md`. The report is committed to
`reports/similarity/<section>-<n>.txt`.

## 9. Rewrite

Rewrite flagged rules, rerun tests and the similarity check, and commit the
new report.

## 10. Journal

Add an entry to `docs/journal.md`: time spent, rules and tests produced, how
many disagreements, how many flags, and anything the pipeline got wrong.
