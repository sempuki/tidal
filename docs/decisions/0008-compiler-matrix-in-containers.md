# 0008. Compiler matrix in containers, with EDG as a third implementation

Date: 2026-10-30
Status: Accepted

## Context

Two host compilers are a thin oracle. They can share a behavior by
accident, and contributors can't reproduce results without the same
machine. The project needs many compiler versions and targets.

The EDG C++ front end is open source (`github.com/edgcpp/compiler`,
Apache-2.0 with LLVM exception, first public release 2026-09-30). It is an
independent implementation that shares no code with Clang or GCC. MSVC is
not available on Linux and is left out for now.

Bazel is not used. The workload is thousands of independent
single-file compiles with no dependency graph. Bazel's main
benefits are caching and remote execution, and configuring Bazel C++
toolchains for many compilers costs more than it saves at this size.

## Decision

- `tools/compilers.json` lists the matrix. Each entry gives a family
  (clang, gcc, edg), a role, command templates, and an optional container
  image.
- **Roles.** `baseline` compilers decide whether a test passes. `report`
  compilers are shown in results but don't fail the run. A compiler is
  promoted to `baseline` after its results have been reviewed once.
- **Images.** `docker/apt.Dockerfile` installs a distribution compiler
  package (versioned GCC and Clang). `docker/edg.Dockerfile` builds EDG's
  front end from a release tag. `tools/containers.py` builds them with
  Docker or Podman, whichever is installed.
- **Execution.** The runner starts one container per compiler for the whole
  run and executes each compile inside it. The tests directory is mounted
  read-only.
- **Caching.** Results are cached by a hash of the test text, the
  compiler's version output and image ID, and its command templates. Only
  changed tests or compilers are rerun.
- **Front-end-only compilers.** A compiler with no `build` command checks
  programs but can't run them. A test expecting `run` is checked as
  `accept` on such a compiler.
- **Unsupported flags.** A compiler may list test flags it can't honor
  (EDG doesn't take `-m32` or GCC warning flags). Tests that use them are
  skipped on that compiler.
- **Expectations.** `tidal-expect-<x>` may name a family or a compiler id.
  An id takes precedence over a family.

## Consequences

- Running the full matrix needs a container engine. Running baseline
  compilers that are installed on the host does not.
- Tests must use only standard C++, because compiler built-ins such as
  `__is_same` mean nothing to other implementations (see
  `docs/spec-format.md`).
- EDG is configured with its x86-64 Linux target settings. Its 32-bit
  settings could later support the `-m32` tests.
