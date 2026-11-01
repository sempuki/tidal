# tidal-fe

A C++ front end written in Rust from the Tidal specification, to exercise
the specification. The front end checks programs and runs them on an
interpreter of the abstract machine. It does not generate code. The
front end can use any crate that does not encode knowledge of C++
(decision 0035).

## Build

```sh
cd frontend
cargo build --release
```

The binary is `target/release/tidal-fe`.

## Check one file

```sh
target/release/tidal-fe check FILE [--m32] [--werror] [-Wname ...]
```

| Exit status | Meaning |
|---|---|
| 0 | accept |
| 1 | reject: at least one error, printed with a Tidal rule ID when one applies |
| 2 | unsupported: the front end met a construct it does not understand and found no error |

Diagnostics go to standard error, one per line:

```
FILE:LINE: error [SW-3]: duplicate case value 1
FILE:LINE: error [CX-1, CE-22]: initializer is not a constant expression (...)
FILE:LINE: unsupported: static_assert: new expression
FILE:LINE: warning [STMT-4]: expression result unused
```

An error names a rule of a class that allows rejection (decision 0034).
When a constant was needed and the evaluation failed, the error names
two rules: the rule that needs the constant, then the rule that the
evaluation broke. The conformance count of matching rules accepts either.

`--m32` switches the target from LP64 (x86-64 Linux) to ILP32.
`--werror` turns the warnings for `recommended` rules into errors.
Some warnings run only when a `-W` flag asks for them, following the
warning-flag table in the Evidence of `specification/foundations/translation.md`:
ATTR-6 with `-Wall`, `-Wunused`, `-Wunused-variable`, or
`-Wunused-but-set-variable`, SEQ-7 with `-Wall`, ATTR-7 with
`-Wimplicit-fallthrough` or `-Wextra`, CTOR-13 with `-Wall`, the
deprecated-copy part of CPY-13 with `-Wextra`, and the pessimizing-move
part of CPY-13 with `-Wall`. ODR-11 is on with no flag, and every test
passes `-pedantic-errors`, which makes it an error. The front end names
ODR-11 with ODR-9.

## Run one program

```sh
target/release/tidal-fe run FILE [FILE...] [--m32]
```

`run` checks each file as a translation unit of its own, links the units
together, and runs the program on the interpreter (decision 0048). It
prints one line:

```
result: 0
unspecified: result 0 with fixed choices: SEQ-1 at line 7 (arguments evaluated from left to right)
undefined [MUL-9] at line 8: division by zero
terminated [UNW-5] at line 4: an exception leaves main
unsupported at line 3: pointer to member constant
rejected [CTOR-25] at line 5: no constructor of 'S' accepts the arguments
```

| Exit status | Meaning |
|---|---|
| the program's status, reduced to 0 to 255 | `result` or `unspecified`: the program returned from `main` |
| 125 | the check rejected a translation unit |
| 126 | `undefined`: the run stopped at undefined behavior and gives no result |
| 127 | `unsupported`: the check or the interpreter met a construct it does not model |
| 134 | `terminated`: the program ended without returning from `main` (UNW-5, THR-7), as `std::abort` does on the matrix targets |

A program's own status can also be 125 to 127 or 134. The printed line
tells the cases apart.

An `unspecified` line lists the choice points that the run passed, each
with the rule that leaves the choice open. The interpreter takes one
fixed choice at each point and does not try the others.

## Run the conformance tests

```sh
target/release/tidal-fe conform ../tests [-v] [--out FILE]
```

`check` reads `#include "file"` relative to the directory of the file
that holds the directive (INC-2). The other preprocessing is a small
slice:

- the conditional groups of the ODR page's grammar: `#if`, `#ifdef`,
  `#ifndef`, `#elif`, `#else`, and `#endif`, with integer expressions
  and `defined`,
- `#define` of a name with or without a value, and `#undef`. A name with
  a value is replaced by its tokens.
- `#pragma once` anywhere in a file (INC-5).

A function-like `#define` and other directives are unsupported.

The harness reads the `// tidal-` directives at the top of each test. The
harness uses the `tidal-expect-edg` override when a test has one, and treats
`run`, `run-fail`, and `link-fail` as accept for the check. `-Werror` in `tidal-flags`
passes `--werror`, `-m32` passes `--m32`, and the other `-W` flags
pass through. A test with
`tidal-sources` checks each translation unit on its own.

The output is a table per directory and per subdirectory with these
columns:

- **Pass**: the answer matches the expectation.
- **False reject**: the front end rejected a valid program.
- **Missed reject**: the front end accepted an invalid program.
- **Unsupported**: the front end gave up on a construct and found no error.
- **Differs from EDG only**: the answer differs from the EDG override
  but matches the test's base `tidal-expect` line.

The harness then runs each test whose base expectation is `run` or
`run-fail`, if the check accepted every translation unit, and links the
units of a test with `tidal-sources`. A `run` test passes when the
result is exit status 0. A `run-fail` test passes when the run stops at
undefined behavior that names one of the test's rules, or the program
ends with a nonzero status or terminates. A separate table counts the run
tests:

- **Pass**.
- **Wrong result**: a `run` test ended with a nonzero status or
  terminated.
- **Missed undefined behavior**: a `run-fail` test ended with status 0.
- **Unexpected undefined behavior**: the run stopped at undefined
  behavior that the test does not expect.
- **Unsupported**: the interpreter met a construct it does not model.
- **Check did not accept**: the check rejected a unit or gave up, so the
  program did not run.

The output then lists the unsupported constructs, with the number of tests each
one affects, first for the interpreter and then for the check. `-v`
also lists every test that did not pass. The lines for run tests start
with `RUN`, so that `tools/frontend_check.py` reads only the check
lines. `--out FILE` writes the tables and one line per test to FILE.

## Layout

| File | Contents |
|---|---|
| `src/lexer.rs` | tokens, literal checks (PRIM rules), and `#include "file"` with include guards and `#pragma once` (INC rules) |
| `src/parser.rs` | parser state, entities, scopes, name lookup |
| `src/decl.rs`, `src/decl2.rs` | decl-specifiers, declarators, declarations, functions, templates |
| `src/class.rs` | class and enumeration specifiers |
| `src/expr.rs`, `src/expr2.rs` | expressions, conversions, calls, casts, initialization |
| `src/ics.rs` | implicit conversion sequences and their comparison (ICS rules), and overload resolution for calls (OVL-22 to OVL-25, OVL-34 to OVL-36) |
| `src/ovl.rs` | overload resolution for operators, constructors, calls on a class object, and the address of an overload set (OVL rules) |
| `src/stmt.rs` | statements and jump checks |
| `src/interp/mod.rs` | the interpreter's machine: values, objects, places, the objects that end with a block, a full expression, or the program, and the stops (decision 0048) |
| `src/interp/expr.rs` | expressions, conversions, initialization, constructors, copies, assignments, calls, `new`, `delete`, `throw`, and `dynamic_cast` |
| `src/interp/stmt.rs` | statements, blocks, `goto`, `switch`, try blocks and handlers, and static locals |
| `src/interp/run.rs` | a whole program: linking the units, static initialization, `main`, and the end of the static objects |
| `src/consteval.rs` | constant evaluation, as a client of the interpreter with the limits of `constant-evaluation.md` |
| `src/conform.rs` | the conformance harness |

The findings about the specification are in
`reports/clean-room/rust-frontend/report.md` and `report-2.md` to
`report-7.md`.
