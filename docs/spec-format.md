# Specification format

## Sections

Each topic is one Markdown file under `spec/`, grouped by area:

```
spec/
  glossary.md
  statements/if.md
  expressions/logical.md
```

A section file has this shape:

```markdown
# The if statement

Depends on: [condition](../glossary.md#condition), [scope](../glossary.md#scope)
Inputs: clang-parse-stmt, clang-sema-stmt (see inputs/MANIFEST.md)

## Syntax

    if-stmt  = "if" [ "constexpr" ] "(" [ init ] cond ")" stmt [ "else" stmt ]

## Rules

### IF-1 (required)

Rule text.

Tests: statements/if/basic-true.cpp, statements/if/basic-false.cpp
```

## Syntax notation

Grammar uses plain EBNF with Tidal's own nonterminal names:
`[ x ]` is optional, `{ x }` is zero or more, `a | b` is a choice, and quoted
text is a literal token. Nonterminals are lowercase with hyphens.

## Rules

A rule heading is `### <ID> (<class>)`. IDs are an uppercase prefix for the
section plus a number, such as `IF-3` or `LOG-7`. IDs are never reused. A
withdrawn rule keeps its heading with the class `withdrawn` and a sentence
saying why.

Every rule ends with a `Tests:` line listing at least one test file, relative
to `tests/`.

### Rule classes

| Class | Meaning | Typical evidence |
|---|---|---|
| `required` | Every implementation behaves this way. | Both compilers agree, at compile time and run time. |
| `reject` | The program is invalid and the implementation must reject it with a diagnostic. | Both compilers give an error under `-pedantic-errors`. |
| `no-diagnostic` | The program is invalid, but an implementation need not detect it. | Compilers accept, but behavior is inconsistent or depends on information unavailable at that point. |
| `impl-defined` | The implementation chooses the behavior and documents it. | Compilers differ, or differ across targets, and each is internally consistent. |
| `unspecified` | The implementation picks from a listed set of outcomes, and need not document or be consistent. | Results vary with optimization level or context. |
| `undefined` | Tidal places no requirements on the program. | Constant evaluation rejects it, UBSan reports it, or the optimizer assumes it doesn't happen. |
| `recommended` | Implementations should behave this way, usually by warning, but a program that triggers it is still valid. | Compilers warn by default or under a common flag; the test makes the warning an error. |

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
| `tidal-rules` | rule IDs | Rules this test supports. Required. |
| `tidal-expect` | `accept`, `reject`, `link-fail`, `run` | `accept`: compiles. `reject`: fails to compile. `link-fail`: every translation unit compiles, but linking them fails. `run`: compiles, links, runs, and exits 0. |
| `tidal-expect-<x>` | same | Override for a compiler family (`clang`, `gcc`, `edg`) or a compiler id from `tools/compilers.json`. Use only with a `tidal-note` naming the question in `questions/`. |
| `tidal-flags` | compiler flags | Extra flags, for example `-fsanitize=undefined`. |
| `tidal-sources` | file names | Extra translation units for this test, named `*.tu.cpp` and kept next to it. Each unit is compiled on its own, then all are linked together. |
| `tidal-note` | text | Free text, such as the question ID (`Q-0001`) behind an override. |

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
