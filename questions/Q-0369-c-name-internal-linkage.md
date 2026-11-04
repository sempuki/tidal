# Q-0369: Are C names with internal linkage in two namespaces one entity?

**Status:** provisional
**Affects:** LSP-4, LNK-5, `tests/declarations/linkage-specifications/c-static-function-two-namespaces.cpp`, `tests/declarations/linkage-specifications/c-static-variable-two-namespaces.cpp`, `tests/declarations/linkage-specifications/c-static-not-joined.cpp`, `tests/declarations/linkage-specifications/c-name-unnamed-namespace.cpp`, `tests/declarations/linkage-specifications/c-name-unnamed-namespace-other-unit.cpp`, `tests/declarations/linkage-specifications/c-name-unnamed-namespace-redefined.cpp`

## Context

LSP-4 makes the C names of one spelling in different namespaces one
entity. A `static` function or variable in an `extern "C"` block has
internal linkage (LSP-1). The question is whether two such names with
one spelling, in two namespaces of one unit, are one entity defined
twice or two entities.

| Program | Clang 18 and 22 | GCC 16 | GCC 14 | EDG 7.0 |
|---|---|---|---|---|
| Two `static` C functions `cb` in two namespaces | runs, two functions | the assembler fails ("symbol `cb' is already defined") | as GCC 16 | accepts |
| Two `static` C variables in two namespaces | runs, two variables | the assembler fails | runs | accepts |
| A `static` C function in `a`, and a `static` C declaration of the same name in `b` that the program calls | rejects: `b::cb` is not defined | rejects: `b::cb` is used but never defined | as GCC 16 | rejects |

The third row shows that every compiler treats the two `static` names
as two entities in lookup. GCC accepts the first two programs as source
and fails only when its assembler sees one symbol name twice. At `-O2`,
GCC 16 emits no symbol and the programs run.

A C function in an unnamed namespace behaves as a C name with external
linkage on Clang and GCC. A C declaration in another namespace names the
same function, and a declaration in another unit links to it.

## Evidence

The tests in Affects, and probes `c1.cpp`, `c2.cpp`, and `c5.cpp` of
the Propose exercise X15.

## Options

1. A C name with internal linkage belongs to its namespace only, so the
   two names are two entities (Clang, and the lookup of every compiler).
2. The two names are one entity, so the program defines it twice and is
   not valid.

## Provisional answer

Two entities (option 1). Every compiler's lookup treats them as two
entities, and GCC's failure comes from its symbol names, after the
source is accepted. A C name in an unnamed namespace stays one entity
with the other C names of its spelling, as Clang and GCC link it.

Kept open on 2026-11-04 under decision 0054, because the gap review of the rules that phase 7 rewrote reads the reference against the last sentence of the answer. EDG 7.0 follows the review: it accepts a definition of a C function in an unnamed namespace beside a definition in another namespace, which Clang 22 rejects and GCC 16 fails at assembly. Tidal keeps the answer of Clang and GCC until the maintainer decides (`documentation/decisions/pending.md`, row 4). GCC 16 also fails at assembly for the first two programs.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that the phase 7 front-end and Propose repairs rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`) agrees with the main answer and contradicts its last sentence. A name with internal linkage has no language linkage in the reference, so the rule that joins C names across namespaces does not apply to two such names, and the reference's own example says the same. An unnamed namespace has internal linkage, and so do the variables and functions declared in it at any depth. A linkage specification gives its language linkage only to names with external linkage. So a C function or variable in an unnamed namespace has internal linkage and no language linkage. The name is not one entity with C names of its spelling elsewhere, and another unit cannot reach it. Clang and GCC conflict with the reference there, and the review knows of no later resolution, though implementations have long differed on the point.
