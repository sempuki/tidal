# Tidal

Tidal is an open specification of a programming language intended to stay
source-compatible with ISO/IEC 14882 (C++). The specification text is written
independently. It is derived from the observable behavior of open source C++
implementations and their test suites, never from the ISO text.

Repository: https://github.com/sempuki/tidal
Specification site: https://sempuki.github.io/tidal/

**Status:** proof of concept. The scope is limited to expressions and statements.
Pilot sections: `spec/statements/if.md` and `spec/expressions/multiplicative.md`.
See [docs/charter.md](docs/charter.md).

## Layout

| Path | Contents |
|---|---|
| `docs/charter.md` | Goals, scope, and non-goals |
| `docs/clean-room.md` | Who may read what, and how contamination is checked |
| `docs/style-guide.md` | How to write specification text: writing rules, page structure, rule format |
| `docs/spec-format.md` | Rule classes and test directives |
| `docs/pipeline.md` | The step-by-step process for producing a section |
| `docs/decisions/` | One file per design decision, numbered in order |
| `docs/status.md` | Where the project stands and what comes next |
| `spec/` | The Tidal specification text; `spec/README.md` sets section order |
| `docker/` | Container images for the compiler matrix |
| `tests/` | Conformance tests, each tied to one or more rules |
| `tools/` | Test runner and compiler matrix, input fetcher, similarity checker, site build |
| `site/` | Sphinx configuration, the `rule` directive extension, and the stylesheet |
| `reports/` | Similarity reports from the checker, see `reports/README.md` |
| `inputs/` | Downloaded implementation sources (ignored by git, see `inputs/MANIFEST.md`) |

## Common commands

    tools/containers.py build               # build matrix images (docker or podman)
    tools/run_tests.py --report tests/RESULTS.md   # full matrix
    tools/run_tests.py --compilers clang-host,gcc-host   # host only, no containers
    tools/check_spec.py                     # rules and tests refer to each other
    tools/build_site.py                     # build/site (HTML, searchable)
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
