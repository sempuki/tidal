# Charter

## Goal

Produce a specification for expressions and statements that

1. describes the same language as ISO C++23, so that a program's meaning under
   Tidal matches its meaning under ISO C++, and
2. contains no text, examples, or organization copied from the ISO standard.

This is a proof of concept. It tests whether an LLM-assisted pipeline can
produce usable specification text from implementations, and it measures how
close that text ends up to the ISO wording.

## Baseline

- Language level: C++23, as implemented by `-std=c++23`.
- Reference implementations: Clang and GCC (open source). MSVC and EDG may be
  added later as black-box oracles.
- Versions used for this proof of concept are recorded in `inputs/MANIFEST.md`.

## Scope

In scope:

- Statements: expression statements, compound statements, selection (`if`,
  `switch`), iteration (`while`, `do`, `for`, range-based `for`), jumps
  (`break`, `continue`, `return`, `goto`), labels, declaration statements as
  statements.
- Expressions: value categories, the built-in operators, evaluation order and
  sequencing, conversions applied to operands, constant evaluation as far as
  it decides whether an expression is a constant expression.

Referenced but not specified:

Expressions and statements depend on types, declarations, name lookup,
overload resolution, classes, and templates. Tidal sections in this proof of
concept refer to those concepts by name and list them in a "Depends on"
block. They are not specified here.

Out of scope:

- The preprocessor, lexical structure, modules.
- Declarations, classes, templates, and overload resolution as topics of their
  own.
- The standard library.
- Coroutines (`co_await`, `co_yield`, `co_return`).
- The memory model and threads.

## Non-goals

- Changing the language. Tidal describes ISO C++ behavior. Where Clang and GCC
  differ, Tidal records the difference and does not pick a winner by fiat.
- Mirroring ISO structure. Tidal organizes topics its own way.
- Being normative yet. Nothing here should be cited as a definition of C++.

## Success criteria for the proof of concept

1. A pilot section for one statement and one expression family, each with
   rules and passing conformance tests on Clang and GCC.
2. A similarity report against the ISO draft for the pilot sections, produced
   under the protocol in [clean-room.md](clean-room.md).
3. A retrospective in [journal.md](journal.md) estimating the cost of covering
   the full expressions and statements scope.
