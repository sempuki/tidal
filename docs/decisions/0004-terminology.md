# 0004. Terminology: common industry terms allowed, definitions written fresh

Date: 2026-10-30
Status: Accepted

## Context

C++ programmers, compiler diagnostics, and compiler source code all use terms
that originated in the standard: lvalue, prvalue, xvalue, undefined behavior,
ill-formed, sequenced before. Inventing replacements would make Tidal harder to
read and harder to map onto implementations. Short terms and phrases are
generally not protected by copyright, but definitions of them are prose and
can be.

## Decision

Tidal may use a term when it appears in implementation source identifiers or
diagnostics of Clang or GCC, or is in common industry use. Tidal writes its own
definition of every term in `spec/glossary.md`.

Where a term is ambiguous or its standard meaning is hard to state from
implementation behavior, Tidal may use a different term and record the
mapping in the glossary.

## Consequences

- Readers familiar with C++ can read Tidal without a translation table for
  common terms.
- The similarity check will see some shared short phrases. Its 8-word
  threshold is meant to sit above the length of a term of art.
