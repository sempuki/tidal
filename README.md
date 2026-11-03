# Tidal

Tidal is an open specification of a programming language intended to stay
source-compatible with ISO/IEC 14882 (C++). The specification text is written
independently. It is derived from the observable behavior of open source C++
implementations and their test suites, never from the ISO text.

The [Ferrocene Language Specification](https://rust-lang.github.io/fls/)
for Rust is the inspiration for Tidal's presentation: its fixed page
segments, short identified rules, linked glossary terms, and informative
examples. See Acknowledgements below.

## Purpose

Tidal explores new ways to present and extend the specification of
C++. It tests forms that the ISO standard does not use:

- principles first, with each rule tied to the principles it supports;
- short rules, each with an ID, a class, and the tests that support it;
- the observed behavior of several compilers, recorded beside each rule;
- open questions, recorded where the compilers disagree;
- an interpreter that runs programs against the rules;
- design exercises that propose extensions to the language and trace
  each change through the rules and tests.

Tidal does not replace the ISO standard, and it is not a way to avoid
buying it. The ISO standard remains the normative definition of C++.
Readers who need that text should obtain ISO/IEC 14882 from ISO or
from their national standards body.

Repository: https://github.com/sempuki/tidal
Specification site: https://sempuki.github.io/tidal/

**Status:** statements, expressions, declarations, classes, exceptions,
name lookup, overload resolution, and the core of templates are specified.
The rest of templates is in progress. [documentation/charter.md](documentation/charter.md)
gives the scope, and [documentation/status.md](documentation/status.md) gives the current
counts and next steps.

## Layout

| Path | Contents |
|---|---|
| `CONTRIBUTING.md` | Roles, how to claim work, the pipeline in short, and what a change needs before merge |
| `AGENTS.md` | Rules for all agents: source readers give clean-room writers summaries only |
| `documentation/charter.md` | Goals, scope, and non-goals |
| `documentation/setup.md` | What a machine needs to run every check |
| `documentation/clean-room.md` | Who may read what, and how contamination is checked |
| `documentation/style-guide.md` | How to write specification text: writing rules, page structure, rule format |
| `documentation/specification-format.md` | Rule classes and test directives |
| `documentation/pipeline.md` | The step-by-step process for producing a section |
| `documentation/exercises.md` | Clean-room exercises that check the specification by using it |
| `documentation/decisions/` | One file per design decision, numbered in order |
| `documentation/status.md` | Where the project stands and what comes next |
| `documentation/roadmap.md` | The order of the work ahead, from four clean-room views |
| `specification/` | The Tidal specification text; `specification/index.md` is the site front page and table of contents |
| `docker/` | Container images for the compiler matrix |
| `tests/` | Conformance tests, each tied to one or more rules |
| `questions/` | Open questions where the compilers disagree or a rule is unclear, each with the provisional answer the specification follows |
| `frontend/` | `tidal-fe`, a C++ compiler front end in Rust, written only from the specification to test it. It checks programs, names the rule each error breaks, and runs the conformance tests (`frontend/README.md`) |
| `guides/` | A tutorial and how-to guides written from the specification, kept apart from it and unpublished (decisions 0045, 0046) |
| `tools/` | Test runner and compiler matrix, input fetcher, similarity checker, site build |
| `site/` | Sphinx configuration, the `rule` directive extension, and the stylesheet |
| `reports/` | Similarity reports from the checker, see `reports/README.md` |
| `inputs/` | Downloaded implementation sources (ignored by git, see `inputs/MANIFEST.md`) |

## Common commands

    tools/containers.py build               # build matrix images (docker or podman)
    tools/run_tests.py --report tests/RESULTS.md   # full matrix
    tools/run_tests.py --compilers clang-host,gcc-host   # host only, no containers
    tools/check_specification.py                     # rules and tests refer to each other
    tools/build_site.py                     # build/site (HTML, searchable)
    tools/frontend_check.py                 # the compiler front end against its baseline
    tools/check_all.sh                      # all of the above; run before pushing

## Acknowledgements

Tidal's page structure follows the
[Ferrocene Language Specification](https://rust-lang.github.io/fls/) for
Rust: fixed segments, short identified rules, linked glossary terms, and
informative examples (decision 0016). Tidal does not copy FLS text or
tooling. The writing rules adapt ASD-STE100 Simplified Technical English.

## License

Tidal is licensed under the Apache License v2.0 with LLVM Exceptions
(SPDX: `Apache-2.0 WITH LLVM-exception`). See [LICENSE.txt](LICENSE.txt).
Contributions are accepted under the same license.

Copyright the Tidal contributors.

## History as a record

Documents in this repository describe the current state and next steps.
The history of how the project got there is the git log. Each design
decision and each pipeline step is committed separately with a message
that says what changed and why. Don't squash or rewrite history.
