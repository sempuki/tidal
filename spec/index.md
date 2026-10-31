# The specification

Tidal is an open specification of a language that is compatible with
ISO C++23. The specification is reference material. Each page states
what the language does. The guides that teach tasks are a separate part
of the site. The {doc}`glossary <glossary>` defines the terms that the
pages use.

The Foundations pages state the models that the other pages build on.
Each page opens with its principles, then its grammar, then its rules in
fixed segments. A principle names the rules that are its evidence. Each
rule has an ID, a class, and links to the tests that support it.
`docs/style-guide.md` in the repository describes the format.

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
exceptions/index
glossary
rules
:::
