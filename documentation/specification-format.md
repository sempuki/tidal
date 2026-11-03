# Specification format

## Pages and rules

Each topic is one MyST Markdown page under `specification/`. `documentation/style-guide.md`
gives the page structure, the `rule` directive, the writing rules, and the
grammar notation (Sphinx `productionlist`). Rule IDs are an uppercase
prefix and a number, such as `IF-3`. IDs are never reused. A withdrawn
rule keeps its directive with the class `withdrawn` and a sentence that
says why. When a rule merges into another, the sentence names the
target with a `{rule}` reference, so that tools can follow it. The
commit message of a change that withdraws a rule lists the Propose
reports that cite the rule. `tools/propose_check.py`, run after the
change, prints each such row (`documentation/exercises.md`, section 4).

## Principles

A page leads with a `Principles` segment (decision 0024). Each principle
is a `principle` directive with an ID and a `:rules:` list naming the
rules that are its evidence. `tools/check_specification.py` checks that each
principle cites at least one rule on its page and that every rule on
such a page is cited. The rules keep their IDs, classes, and tests.

## Rule classes

| Class | Meaning | Typical evidence |
|---|---|---|
| `required` | Every implementation behaves this way. | Both compilers agree, at compile time and run time. |
| `reject` | The program is invalid and the implementation must reject it with a diagnostic. | Both compilers give an error under `-pedantic-errors`. |
| `no-diagnostic` | The program is invalid, but an implementation need not detect it. | Compilers accept, but behavior is inconsistent or depends on information unavailable at that point. |
| `impl-defined` | The implementation chooses the behavior and documents it. | Compilers differ, or differ across targets, and each is internally consistent. |
| `unspecified` | The implementation picks from a listed set of outcomes, and need not document or be consistent. | Compilers, or configurations of one compiler, give different results from the listed set. |
| `undefined` | Tidal places no requirements on the program. | Constant evaluation rejects it, or UBSan reports it. A configuration whose result only undefined behavior allows supports the class (decision 0047). |
| `recommended` | Implementations should behave this way, usually by warning, but a program that triggers it is still valid. | Compilers warn by default or under a common flag; the test makes the warning an error. |
| `conditionally-supported` | An implementation can reject the construct. If the implementation accepts the construct, the implementation documents the behavior. | Some compilers in the matrix reject under `-pedantic-errors` and others accept, each consistently. |

A `no-diagnostic` rule allows an implementation to reject a program
that breaks the rule or to accept it. A test whose program breaks such
a rule accepts every outcome that the rule allows. The test's
`tidal-expect` gives one outcome, and a `tidal-expect-<x>` override
records each compiler that gives the other. The override's `tidal-note`
names the rule or the question that allows the outcome. A failing test
then means that a compiler changed its behavior, and the failure says
nothing about conformance. The test checks only translation and
linking, because the rule allows any result at run time (decision
0047).

Classifying behavior is the hardest step. Each rule in the last four classes
needs a sentence in the section's "Evidence" notes explaining why the
rule was put in that class.

## Evidence

The `Evidence` segment ends a page and is informative
(`documentation/style-guide.md`, section 7). The segment has these parts:

- the agreement sentence, which `tools/status_table.py` keeps current,
- one bullet for each divergence or classification note. A divergence
  bullet names the compilers on each side and links the question that
  holds the provisional answer, for example ``({question}`Q-0053`)``.
- an `Inputs:` line, last.

The `Inputs:` line lists the sanitized implementation sources that the
clean-room writer read for the page. Each entry is an input ID from
`tools/inputs.json`, such as `clang-sema-declcxx`, and
`inputs/MANIFEST.md` gives the file and revision for each ID. A page
written without such sources has no `Inputs:` line.

## Tests

Tests are single C++ files under `tests/`, mirroring the `specification/` layout. A
header of `//` directives at the top of the file controls the runner:

```cpp
// tidal-rules: IF-1 IF-2
// tidal-expect: run
```

| Directive | Values | Meaning |
|---|---|---|
| `tidal-rules` | rule IDs | Rules this test supports. Required. A test of a failed constant cites the rule that the evaluation broke, and can also cite the rule that needs the constant (decision 0038). A test whose program breaks a general `reject` rule and a specific one cites the specific rule, and can also cite the general one. The general rule names the specific rules (decision 0042). The same holds for `undefined` rules, and a run stop that names either rule matches (decision 0052). |
| `tidal-expect` | `accept`, `reject`, `link-fail`, `run`, `run-fail` | `accept`: compiles. `reject`: fails to compile. `link-fail`: every translation unit compiles, but linking them fails. `run`: compiles, links, runs, and exits 0. `run-fail`: compiles and links, but the run exits nonzero, for example because a sanitizer stops it (decision 0028). |
| `tidal-expect-<x>` | same | Override for a compiler family (`clang`, `gcc`, `edg`) or a compiler id from `tools/compilers.json`. Use only with a `tidal-note` naming the question in `questions/`, or, for a report compiler such as `gcc-16-o2`, the rule that allows both outcomes (decisions 0043 and 0047). A variant compiler uses the overrides of the compiler it is a variant of, unless the test gives its own. |
| `tidal-error` | regular expression | For a `reject`, `link-fail`, or `run-fail` test: one of the compiler's error or note messages, the linker's message, or a line of the program's output must match it, case-insensitive. A failure for another reason fails the test. |
| `tidal-error-<x>` | regular expression | The same, for a compiler family (`clang`, `gcc`, `edg`) or a compiler id. Overrides `tidal-error` for that compiler. |
| `tidal-interpreter` | `run-fail` | For an `accept` test whose run has undefined behavior that no sanitizer stops. The matrix compilers only build the program, and `tools/run_tests.py` ignores the line. The front end's interpreter runs the program and must stop at undefined behavior that names one of the test's rules (decision 0053). |
| `tidal-flags` | compiler flags | Extra flags, for example `-O0 -fsanitize=undefined -fno-sanitize-recover=undefined`. A sanitizer test starts with `-O0`, because at `-O2` the optimizer can remove the operation before the sanitizer sees it (decision 0043). A compiler skips a test whose flags match its `unsupported_flags` in `tools/compilers.json`. |
| `tidal-sources` | file names | Extra translation units for this test, named `*.tu.cpp` and kept next to it. Each unit is compiled on its own, then all are linked together. |
| `tidal-note` | text | Free text, such as the question ID (`Q-0001`) behind an override. |

A test whose program has undefined behavior at run time must not
expect `run`. A `run` test asserts an outcome, and undefined behavior
promises none (decision 0047). The test expects one of these:

| Case | Expectation |
|---|---|
| A sanitizer configuration stops the behavior. | `run-fail`, with the flags of decisions 0028 and 0043. |
| No sanitizer configuration stops the behavior. | `accept`. The matrix compilers only build the program. If the front end's interpreter stops the run with one of the test's rules, the test adds `tidal-interpreter: run-fail` (decision 0053). |

Clang's type sanitizer checks the type of each read and write against
the object in the storage (`-O0 -fsanitize=type`). The sanitizer prints
a report and lets the program go on, and Clang 22 has no option that
makes it exit nonzero. So `tools/run_tests.py` counts a run that prints
a `TypeSanitizer` error report as `run-fail`, whatever its exit status.
Only Clang 22 has the sanitizer. GCC, Clang 18, and EDG list
`-fsanitize=type` in `unsupported_flags` and skip the test, so a type
sanitizer test names `-fsanitize=type` as a flag of its own, without
other sanitizers. Its `tidal-error` pattern matches the report line that
names the two types, such as `with type float accesses an existing
object of type int`.

If constant evaluation can reach the behavior, a separate test expects
`reject`. The notes and the rule's Evidence describe which
configurations stop the behavior, and give no run result.

A `tidal-error` pattern names the reason for the rejection in words that
the compiler's message uses, without its quote characters. The host GCC
quotes with `‘’` and the container GCCs with `''`. Write one pattern per
family, because the three families word their messages differently.
`tools/run_tests.py --errors-out FILE` writes every rejecting compiler's
messages to a JSON file. The file helps when writing patterns.

Every compile uses the compiler's strict C++23 mode (`-std=c++23
-pedantic-errors` for GCC and Clang, `--c++23 --strict` for EDG). Run
tests should check their own results and return nonzero on failure. Prefer `static_assert` where the
behavior can be observed at compile time.

Test code is written fresh. Don't adapt examples from any source.

Tests use only standard C++. Compiler built-ins such as `__is_same` or
`__INT_MAX__` are not allowed, because a test must mean the same thing to
every implementation in the matrix. Avoid standard library headers too.
Front-end-only implementations such as EDG have no library, and a small
helper written in the test (a `same<A, B>` variable template, for example)
is enough for most checks.
