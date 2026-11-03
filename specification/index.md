# Tidal

Tidal is an open specification of a language that is compatible with
ISO C++23. The specification is reference material. Each page states
what the language does. The {doc}`glossary <glossary>` defines the
terms that the pages use.

The Foundations pages state the models that the other pages build on.
Each page opens with its principles, then its grammar, then its rules in
fixed segments. A principle names the rules that are its evidence. Each
rule has an ID, a class, and links to the tests that support it.
`documentation/style-guide.md` in the repository describes the format.

(baseline-compilers)=
## Baseline compilers

The Evidence segment of each page reports the results of eight
baseline compilers. Each test runs on each compiler that supports its
flags:

| Compiler | Family | Notes |
|---|---|---|
| Clang 18 | Clang | |
| Clang 22 | Clang | |
| host Clang | Clang | Clang 22 |
| GCC 14 | GCC | |
| GCC 16 | GCC | |
| host GCC | GCC | GCC 16 |
| EDG 7.0 | EDG | a front end only, so EDG translates each program and runs none |
| EDG 7.0, ILP32 | EDG | the same front end for a target with 32-bit `long` and pointers; runs only the tests for such a target |

A compiler skips each test that uses a flag the compiler does not
accept. A skipped test has no result on that compiler, so the
outcome of the test is the outcome on the compilers that run it.

| Flags | Compilers that skip the test |
|---|---|
| `-Werror`, `-Wall`, `-O2`, `-fsanitize=...` | EDG 7.0, EDG 7.0 ILP32 |
| `-m32` | EDG 7.0, GCC 16 |
| no `-m32` | EDG 7.0 ILP32 |

Each test of a `recommended` rule that expects a rejection uses
`-Werror`, so the outcome of such a test comes from Clang and GCC.

"Clang" in an Evidence segment means every Clang compiler in the
table, and the same holds for "GCC" and "EDG". A version number names
one compiler. A result on any baseline compiler counts, also on an
older version such as Clang 18 or GCC 14. `tests/RESULTS.md` gives the
exact version of each compiler.

## Acknowledgements

The page structure of Tidal follows the
[Ferrocene Language Specification](https://rust-lang.github.io/fls/)
(FLS) for Rust. Its segments, short identified paragraphs, linked glossary
terms, and informative examples are the model for Tidal pages. Tidal does
not copy FLS text or tooling.

The writing rules adapt ASD-STE100 Simplified Technical English, which the
ASD Simplified Technical English Maintenance Group maintains.

:::{toctree}
:numbered:
:maxdepth: 2

foundations/index
statements/index
expressions/index
declarations/index
classes/index
templates/index
exceptions/index
glossary
rules
:::
