# tidal-fe

A C++ front end written in Rust from the Tidal specification, to exercise
the specification. The front end checks programs and does not generate
code. The front end can use any crate that does not encode knowledge of
C++ (decision 0035).

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
warning-flag table in the Evidence of `spec/foundations/translation.md`:
ATTR-6 with `-Wall`, `-Wunused`, `-Wunused-variable`, or
`-Wunused-but-set-variable`, SEQ-7 with `-Wall`, ATTR-7 with
`-Wimplicit-fallthrough` or `-Wextra`, CTOR-13 with `-Wall`, the
deprecated-copy part of CPY-13 with `-Wextra`, and the pessimizing-move
part of CPY-13 with `-Wall`.

## Run the conformance tests

```sh
target/release/tidal-fe conform ../tests [-v] [--out FILE]
```

The harness reads the `// tidal-` directives at the top of each test. The
harness uses the `tidal-expect-edg` override when a test has one, and treats
`run`, `run-fail`, and `link-fail` as accept. `-Werror` in `tidal-flags`
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

The output then lists the unsupported constructs, with the number of tests each
one affects. `-v` also lists every test that did not pass. `--out FILE`
writes the table and one line per test to FILE.

## Layout

| File | Contents |
|---|---|
| `src/lexer.rs` | tokens and literal checks (PRIM rules) |
| `src/parser.rs` | parser state, entities, scopes, name lookup |
| `src/decl.rs`, `src/decl2.rs` | decl-specifiers, declarators, declarations, functions, templates |
| `src/class.rs` | class and enumeration specifiers |
| `src/expr.rs`, `src/expr2.rs` | expressions, conversions, calls, casts, initialization |
| `src/stmt.rs` | statements and jump checks |
| `src/consteval.rs` | constant evaluation |
| `src/conform.rs` | the conformance harness |

The findings about the specification are in
`reports/clean-room/rust-frontend/report.md` and `report-2.md` to
`report-4.md`.
