# Specification format

## Pages and rules

Each topic is one MyST Markdown page under `spec/`. `docs/style-guide.md`
gives the page structure, the `rule` directive, the writing rules, and the
grammar notation (Sphinx `productionlist`). Rule IDs are an uppercase
prefix and a number, such as `IF-3`. IDs are never reused. A withdrawn
rule keeps its directive with the class `withdrawn` and a sentence that
says why.

## Principles

A page leads with a `Principles` segment (decision 0024). Each principle
is a `principle` directive with an ID and a `:rules:` list naming the
rules that are its evidence. `tools/check_spec.py` checks that each
principle cites at least one rule on its page and that every rule on
such a page is cited. The rules keep their IDs, classes, and tests.

## Rule classes

| Class | Meaning | Typical evidence |
|---|---|---|
| `required` | Every implementation behaves this way. | Both compilers agree, at compile time and run time. |
| `reject` | The program is invalid and the implementation must reject it with a diagnostic. | Both compilers give an error under `-pedantic-errors`. |
| `no-diagnostic` | The program is invalid, but an implementation need not detect it. | Compilers accept, but behavior is inconsistent or depends on information unavailable at that point. |
| `impl-defined` | The implementation chooses the behavior and documents it. | Compilers differ, or differ across targets, and each is internally consistent. |
| `unspecified` | The implementation picks from a listed set of outcomes, and need not document or be consistent. | Results vary with optimization level or context. |
| `undefined` | Tidal places no requirements on the program. | Constant evaluation rejects it, UBSan reports it, or the optimizer assumes it doesn't happen. |
| `recommended` | Implementations should behave this way, usually by warning, but a program that triggers it is still valid. | Compilers warn by default or under a common flag; the test makes the warning an error. |
| `conditionally-supported` | An implementation can reject the construct. If it accepts it, it documents the behavior. | Some compilers in the matrix reject under `-pedantic-errors` and others accept, each consistently. |

Classifying behavior is the hardest step. Each rule in the last four classes
needs a sentence in the section's "Evidence" notes explaining why it was put
there.

## Tests

Tests are single C++ files under `tests/`, mirroring the `spec/` layout. A
header of `//` directives at the top of the file controls the runner:

```cpp
// tidal-rules: IF-1 IF-2
// tidal-expect: run
```

| Directive | Values | Meaning |
|---|---|---|
| `tidal-rules` | rule IDs | Rules this test supports. Required. A test of a failed constant cites the rule that the evaluation broke, and can also cite the rule that needs the constant (decision 0038). |
| `tidal-expect` | `accept`, `reject`, `link-fail`, `run`, `run-fail` | `accept`: compiles. `reject`: fails to compile. `link-fail`: every translation unit compiles, but linking them fails. `run`: compiles, links, runs, and exits 0. `run-fail`: compiles and links, but the run exits nonzero, for example because a sanitizer stops it (decision 0028). |
| `tidal-expect-<x>` | same | Override for a compiler family (`clang`, `gcc`, `edg`) or a compiler id from `tools/compilers.json`. Use only with a `tidal-note` naming the question in `questions/`. |
| `tidal-error` | regular expression | For a `reject`, `link-fail`, or `run-fail` test: one of the compiler's error or note messages, the linker's message, or a line of the program's output must match it, case-insensitive. A failure for another reason fails the test. |
| `tidal-error-<x>` | regular expression | The same, for a compiler family (`clang`, `gcc`, `edg`) or a compiler id. Overrides `tidal-error` for that compiler. |
| `tidal-flags` | compiler flags | Extra flags, for example `-fsanitize=undefined -fno-sanitize-recover=undefined`. |
| `tidal-sources` | file names | Extra translation units for this test, named `*.tu.cpp` and kept next to it. Each unit is compiled on its own, then all are linked together. |
| `tidal-note` | text | Free text, such as the question ID (`Q-0001`) behind an override. |

A `tidal-error` pattern names the reason for the rejection in words that
the compiler's message uses, without its quote characters: the host GCC
quotes with `‘’` and the container GCCs with `''`. Write one pattern per
family, because the three families word their messages differently.
`tools/run_tests.py --errors-out FILE` writes every rejecting compiler's
messages to a JSON file, which helps when writing patterns.

Every compile uses the compiler's strict C++23 mode (`-std=c++23 -pedantic-errors`
for GCC and Clang, `--c++23 --strict` for EDG). Run tests should check their
own results and return nonzero on failure. Prefer `static_assert` where the
behavior can be observed at compile time.

Test code is written fresh. Don't adapt examples from any source.

Tests use only standard C++. Compiler built-ins such as `__is_same` or
`__INT_MAX__` are not allowed, because a test must mean the same thing to
every implementation in the matrix. Avoid standard library headers too.
Front-end-only implementations such as EDG have no library, and a small
helper written in the test (a `same<A, B>` variable template, for example)
is enough for most checks.
