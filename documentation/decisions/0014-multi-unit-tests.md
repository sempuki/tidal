# 0014. Tests may span several translation units

Date: 2026-10-30
Status: Accepted

## Context

Linkage decides whether a name in one translation unit refers to the same
entity as a name in another. Single-file tests can't observe linkage. Some
errors also only show up when units are linked, and implementations are
not required to catch all of them.

## Decision

- A test lists extra translation units with `tidal-sources`. They are
  named `*.tu.cpp`, live next to the test, and are not tests themselves.
- The runner compiles each unit on its own with the compiler's `check`
  command, then links all of them with its `build` command.
- A new outcome, `link-fail`, means every unit compiled and linking failed.
  `reject` still means a unit failed to compile.
- Front-end-only compilers (EDG) compile each unit and treat `link-fail`
  and `run` expectations as `accept`.
- Linker messages ("multiple definition of", "undefined reference to")
  appear in the report next to compiler errors.
- Run timeouts are not cached, because they are a symptom of load, and the
  limit is 60 seconds.

## Consequences

- Linkage rules get direct tests. Two units that define the same name
  show whether the names refer to one entity or two.
- `no-diagnostic` rules can show their evidence with units that each
  compile, where linking may or may not catch the error.
