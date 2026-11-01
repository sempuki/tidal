# 0012. Continuous integration on GitHub Actions

Date: 2026-10-30
Status: Accepted

## Context

Contributors open pull requests against the specification, tests, and questions.
Each change needs the same checks: specification cross-references, the compiler
matrix, the similarity check, and the PDF build. GitHub Actions is free for
public repositories on standard runners.

## Decision

`.github/workflows/check.yml` runs on every push to `main` and every pull
request:

1. `tools/check_specification.py`.
2. Build the compiler images, with Docker layers cached between runs.
3. `tools/run_tests.py --compilers containers`. The runner has no host
   compilers, so the matrix includes containerized GCC 16 and Clang 22,
   matching the host baseline versions.
4. `tools/similarity.py --fail-on-flag` against N4950, fetched into the
   runner's temporary directory.
5. `tools/build_pdf.py`, with pinned pandoc and typst versions.

The PDF, test results, and similarity report are uploaded as workflow
artifacts.

`tools/check_all.sh` runs the same checks locally.

## Consequences

- A pull request that breaks a test or adds text that matches the
  reference corpus fails visibly.
- The first run builds EDG from source. Later runs reuse cached layers.
- Tests that need `-m32` are skipped on the official GCC 16 image, which
  has no 32-bit support. They still run on GCC 14, Clang 18, and Clang 22.
