# Charter

## Goal

Produce a specification for expressions, statements, and declarations that

1. describes the same language as ISO C++23, so that a program's meaning under
   Tidal matches its meaning under ISO C++, and
2. contains no text, examples, or organization copied from the ISO standard.

Tidal is a proof of concept. The project tests whether an LLM-assisted
pipeline can produce usable specification text from implementations,
and the project measures how close that text ends up to the ISO
wording.

## Baseline

- Language level: C++23, as implemented by `-std=c++23`.
- Reference implementations: Clang, GCC, and EDG. Tidal reads the sanitized
  source of Clang and EDG. Tidal uses GCC through GCC's behavior (decision 0021).
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
- Exceptions: `throw` expressions and the exception object, try blocks
  and handlers, unwinding and termination (decision 0025).
- Programs of several translation units: the use of a variable or
  function, the one-definition rule, and `#include "file"` with include
  guards (roadmap phase 3, items I and J).
- Name lookup, implicit conversion sequences, and overload resolution
  (roadmap phase 3, items H, L, and M).
- Templates: declarations and parameters, template-ids and template
  arguments, explicit and partial specialization, variable templates,
  and alias templates (roadmap phase 4, item N1).

Referenced but not specified:

The in-scope topics depend on the rest of templates and on concepts:
deduction and partial ordering, dependent names and instantiation,
constraints, packs, forwarding references, and class template argument
deduction. Tidal sections refer to them by name, mark each use
*(templates referenced)* or *(concepts referenced)*, and list them in a
"Depends on" block. Phase 4 of the roadmap brings them into scope
(decision 0033, Q-0113).

Out of scope:

- The preprocessor other than `#include "file"` and include guards,
  lexical structure, modules.
- The parts of templates that phase 4 has not brought in yet.
- The standard library, except the language-support types that the core
  rules name (decision 0041).
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
3. A size estimate for the full expressions and statements scope, kept in
   [status.md](status.md).
