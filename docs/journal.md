# Journal

Dated entries describing what was done and what was learned. Decisions get
their own records in `decisions/`. This file holds the narrative.

## 2026-10-30: Project start

Started Tidal as a proof of concept for producing a C++-compatible
specification from open source implementations with LLM assistance.

The author and an LLM assistant (Claude) are the participants. The author
serves as checker and adjudicator, and Claude serves as writer, following the
protocol in `clean-room.md`. The goal of this phase is to test the mechanics
of the pipeline and to measure the output with the similarity check.

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

Built the tools (decision 0006) and fetched nine source files for the pilots
(about 134,000 lines after sanitizing). Before sanitizing, 291 lines across
the Clang and GCC files contained section citations. After comment removal,
none remained in code, so the citations all lived in comments. The
diagnostic definition files had none.

## 2026-10-30: Pilot 1, the if statement

Probes (step 3): wrote 41 probe programs covering the ordinary form, the
initializer, condition declarations, scopes, object lifetimes, jumps into
branches, `if constexpr`, and `if consteval`. Each probe carried the
writer's prediction.

The compilers agreed with each other on every probe. They disagreed with
the writer twice. The writer predicted that `if constexpr (2)` and
`if constexpr (&g)` are rejected as narrowing conversions. Clang and GCC
both accept them in C++23 mode. The writer's prediction matched an older
language version. Probes exist to catch this kind of error, and the miss
shows that the writer's background knowledge must not be trusted without a
test.

A structured binding as a condition is rejected under `-pedantic-errors`.
Both compilers describe it as a newer-standard extension and accept it
with a warning otherwise.

Source reading (step 4): read Clang's sanitized `ParseIfStatement` and the
condition-checking path in Sema. Questions it raised:

- The parser accepts `!` only before `consteval`. Probe added: `if !constexpr`
  is rejected.
- The else branch of `if !consteval` is parsed as an immediate function
  context. Probe added.
- A discarded branch is parsed in a "discarded statement" evaluation context
  in all functions, not only templates. Probe added: return type deduction
  ignores discarded returns in a non-template function.
- The parser checks that both `if consteval` branches are compound statements
  after parsing them. Already covered.

Probing condition declarations further found that `static`, `extern`,
`thread_local`, and class definitions are rejected in the condition but a
`static` variable and a class definition are fine in the initializer. 14
probes added. All 56 pass on both compilers with no disagreements.

Reading GCC's parser was skipped for this section because Clang and GCC
agreed on every probe. A full run should read both.

Classify and draft (steps 5 and 6): wrote 20 rules (13 `required`, 7
`reject`) and four glossary entries. Every test is cited by the rules it
names, and `check_spec.py` reports no problems. Adjudication (step 7) had
nothing to do because the compilers never disagreed.

This section has no `undefined`, `unspecified`, or `impl-defined` behavior,
so it did not test the hardest part of the pipeline. The second pilot should
be an expression family that does.

## 2026-10-30: Pilot 2, multiplicative operators

Probes (steps 3 and 4): 27 probes. Undefined behavior was probed through
constant evaluation, which both compilers must reject. Implementation-defined
behavior was probed by compiling the same assertion for the default target
(LP64) and for `-m32` (ILP32). Unsequenced modification was probed with
`-Wall -Werror`, relying on each compiler's own warning.

Before writing the tests, two quick probes found:

- The first divergence. During constant evaluation, Clang accepts a floating
  multiply that overflows to infinity and GCC rejects it. Both reject
  floating division by zero.
- No variation in operand order. Clang and GCC both evaluated `f() * g()`
  left to right at `-O0` and `-O2`. GCC's sanitized source shows that its
  strict-order option (`flag_strong_eval_order`) applies to subscripts and
  shifts, and not to `*`, `/`, or `%`. That is the evidence used for
  classifying operand order as `unspecified`.

All 27 probes matched the writer's predictions. The writer then checked the
first error of every `reject` test, because a test can be rejected for an
unrelated mistake and still pass. All were rejected for the intended reason.
The report now records these errors so reviewers can do the same check.
