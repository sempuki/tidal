# Charter

## Goal

Produce a specification for expressions, statements, and declarations that

1. describes the same language as ISO C++23, so that a program's meaning under
   Tidal matches its meaning under ISO C++, and
2. contains no text, examples, or organization copied from the ISO standard.

This is a proof of concept. It tests whether an LLM-assisted pipeline can
produce usable specification text from implementations, and it measures how
close that text ends up to the ISO wording.

## Baseline

- Language level: C++23, as implemented by `-std=c++23`.
- Reference implementations: Clang, GCC, and EDG. Tidal reads the sanitized
  source of Clang and EDG. It uses GCC through its behavior (decision 0021).
  MSVC may be added later as a black-box oracle.
- Versions used for this proof of concept are recorded in `inputs/MANIFEST.md`.

## Scope

In scope:

- Statements: expression statements, compound statements, selection (`if`,
  `switch`), iteration (`while`, `do`, `for`, range-based `for`), jumps
  (`break`, `continue`, `return`, `goto`), labels, declaration statements as
  statements.
- Expressions: primary expressions and literals, value categories and
  temporaries, the built-in operators, pointers to members, `decltype` and
  unevaluated operands, evaluation order and sequencing, conversions
  applied to operands, constant evaluation as far as it decides whether an
  expression is a constant expression.
- Declarations: placeholder types (`auto`, `decltype(auto)`), storage
  classes and linkage, declarators (pointers, references, arrays,
  functions), cv-qualifiers, initialization, structured bindings,
  enumerations, namespaces and `using`, standard attributes, function
  definitions, the `constexpr`, `consteval`, and `constinit` specifiers,
  type aliases and elaborated type specifiers, static assertions, the
  `inline` specifier, alignment, linkage specifications, exception
  specifications, and `volatile`.
- Classes: class definitions and members, access control, constructors
  and destructors, copy and move, derived classes, virtual functions,
  unions and bit-fields, conversion functions, operator overloading,
  defaulted comparisons (decision 0017), and list initialization of
  classes.

Referenced but not specified:

The in-scope topics depend on name lookup, overload resolution, and
templates. Tidal sections refer to those concepts by name and list them
in a "Depends on" block. They are not specified here.

Out of scope:

- The preprocessor, lexical structure, modules.
- Templates and overload resolution as topics of their own.
- The standard library.
- Coroutines (`co_await`, `co_yield`, `co_return`).
- Exception handling (`throw`, try blocks, handlers). It is a later
  chapter of its own (decision 0023).
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
3. A size estimate for the full expressions and statements scope, kept in
   [status.md](status.md).
