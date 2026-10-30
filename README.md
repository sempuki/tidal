# Tidal

Tidal is an open specification of a programming language intended to stay
source-compatible with ISO/IEC 14882 (C++). The specification text is written
independently. It is derived from the observable behavior of open source C++
implementations and their test suites, never from the ISO text.

Repository: https://github.com/sempuki/tidal

**Status:** proof of concept. The scope is limited to expressions and statements.
Pilot sections: `spec/statements/if.md` and `spec/expressions/multiplicative.md`.
See [docs/charter.md](docs/charter.md).

## Layout

| Path | Contents |
|---|---|
| `docs/charter.md` | Goals, scope, and non-goals |
| `docs/clean-room.md` | Who may read what, and how contamination is checked |
| `docs/spec-format.md` | How rules and tests are written |
| `docs/pipeline.md` | The step-by-step process for producing a section |
| `docs/decisions/` | One file per design decision, numbered in order |
| `docs/status.md` | Where the project stands and what comes next |
| `spec/` | The Tidal specification text; `spec/README.md` sets section order |
| `docker/` | Container images for the compiler matrix |
| `tests/` | Conformance tests, each tied to one or more rules |
| `tools/` | Test runner and compiler matrix, input fetcher, similarity checker, PDF build |
| `reports/` | Similarity reports from the checker, see `reports/README.md` |
| `inputs/` | Downloaded implementation sources (ignored by git, see `inputs/MANIFEST.md`) |

## Common commands

    tools/containers.py build               # build matrix images (docker or podman)
    tools/run_tests.py --report tests/RESULTS.md   # full matrix
    tools/run_tests.py --compilers clang-host,gcc-host   # host only, no containers
    tools/check_spec.py                     # rules and tests refer to each other
    tools/build_pdf.py                      # build/tidal.pdf

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
