# Roadmap: what to specify next, from an implementer's view

Date: 2026-10-31. Decision 0029. A clean-room reader in the role of an
implementer building a front end from Tidal read AGENTS.md, docs/,
spec/, tests/, questions/, reports/clean-room/, and frontend/, and ran
the front end's conformance harness (3059 tests: 2291 pass, 0 false
rejects, 564 missed rejects, 172 unsupported). Test counts come from
grep heuristics and are approximate. The list is in recommended writing
order: overload resolution (item 4) has the highest value, and items 1
to 3 come first because they are small and it needs them.

## 1. Ranked topics

**1. Fundamental types and integer ranks (new Foundations page).** No
page lists the fundamental types, the valid type-specifier combinations,
what "same type" means through aliases, or complete and incomplete
types. The glossary marks conversion rank *(referenced)* twice, though
promotion and the usual arithmetic conversions rely on it. The front end
invents its type list; W0 F4 needed the range of `int`. Unblocks the
glossary entries, CONV-1, CONV-13, UN-15, MUL-15, and the rank tables of
item 4. Depends on nothing. 8 to 12 rules, with `impl-defined` size and
range tables for the matrix targets.

**2. Name lookup, completed (deepen names.md).** Missing or scattered:
qualified lookup into classes, namespaces, and enumerations; member
lookup across bases (now on derived.md); using-directives (now on
namespaces.md); the full associated set for argument-dependent lookup
(bases, enumerations, hidden friends); lookup of operator names; the
injected class name; lookup in out-of-line member definitions.
Unblocks the lookup markers in range-for.md, access.md, operators.md,
and primary.md; every candidate set of item 4 starts from lookup. About
18 rules.

**3. Implicit conversion sequences (new page).** The measure overload
resolution compares: standard sequences and their categories,
qualification adjustment, reference binding as a sequence, user-defined
sequences (the one-conversion limit), list-initialization sequences,
ellipsis. conversions.md describes the conversions but not how they
rank. Depends on item 1, conversions.md, references.md,
conversion-functions.md, list.md. 15 to 18 rules.

**4. Overload resolution (new page; highest value).** Candidate sets
per context (call by name, call through an object, operator expressions
with built-in candidates, construction, copy-initialization of a class,
conversion functions, reference initialization); viability; the best
viable function and tie-breakers (non-template over template,
constraints, rewritten candidates for `==` and `<=>`); ambiguity,
deleted selections, access after selection; the address of an overload
set. Over ten pages lean on it. About 75 tests are named for a choice or
an ambiguity, about 127 declare one name for several functions, 314
declare operator functions, and 66 have rvalue-reference parameters.
The front end's largest missed-reject groups are here (CTOR 43, CPY 35,
OPR 30, DCMP 28, CVF 23, ILC 10). W0 F6 needed it. Depends on items 1
to 3 and deleted.md; the template and constraint tie-breakers wait for
items 8 and 13. 25 to 30 rules, possibly two pages.

**5. Templates, core (new chapter).** Declarations and parameters,
template-ids and the `<` disambiguation, `typename` and `template`,
explicit and partial specialization of class and variable templates,
implicit instantiation as far as completeness. 184 tests use the `same`
helper with a partial specialization; 27 declare
`std::initializer_list`; about 8 use the tuple protocol. CE-11 needs
non-type template arguments (E1). Depends on items 1 and 2; can run in
parallel with items 3 and 4. About the size of constructors.md.

**6. Language-support types (small page; narrow charter carve-out).**
Only what the language requires of the library types it names:
`std::initializer_list` (26 tests), the comparison categories (29), the
tuple protocol (about 14), `std::nullptr_t` (13), `std::size_t`,
`std::ptrdiff_t`. 72 test files declare these in `namespace std`
themselves. Covers R-foundations O4 and W0 F5. Depends on item 5. About
the size of static-assert.md.

**7. Use of an entity and the one-definition rule (new Foundations
page).** translation.md maps odr-use to "use *(referenced)*", and no
page defines it, though lambda capture, inline.md, static members,
STO-P4, and the link-fail tests rely on it. Depends on item 2. 6 to 10
rules.

**8. Template argument deduction and function templates.** Deduction
from a call and for placeholders, `initializer_list` element deduction,
abbreviated function templates and generic lambdas (29 tests with
`auto` parameters), overloading of function templates and partial
ordering, packs briefly. Depends on items 4 and 5. About lambdas.md.

**9. Dependent names, checking in two phases, and when instantiation
happens.** if.md, static-assert.md, decltype.md, and constexpr.md lean
on it; about 30 tests outside the `same` helper. Depends on items 5 and
8. About decltype.md.

**10. A consolidated grammar.** Many nonterminals are used and never
defined; there is no decl-specifier, type-id, abstract-declarator, or
template-id grammar; the statement and declaration disambiguation rule
is only in blocks.md. About 20 front-end parse-unsupported tests.
Depends on items 5 and 11. Mostly reorganization.

**11. Lexical structure, without the preprocessor.** Tokens,
identifiers and keywords, comments, maximal munch, alternative tokens
and digraphs, string-literal concatenation with suffixes (OPR-8,
OPR-10), universal character names, splitting `>>`. Few tests turn on
it alone, but every program depends on it. Needs a charter change. 8 to
12 rules.

**12. Constant evaluation of class objects (deepen
constant-evaluation.md).** W0 F2 (every scalar subobject initialized,
and Clang and GCC differ), E3, E5, and `new` and `delete` in constant
evaluation in one place. The front end's unsupported list is dominated
by constant evaluation of classes. Depends on constructors.md,
new-delete.md, item 4. About 6 rules.

**13. Concepts and constraints.** Definitions, requires-clauses and
expressions, satisfaction, subsumption, constrained placeholders
(auto.md, Q-0016). Only 3 test files use `concept`. Depends on items 4,
5, 8, 9.

## 2. Dependency sketch

```
1 Types ─┬─> 3 Conversion sequences ─> 4 Overload resolution ─┬─> 8 Deduction ─> 9 Dependent names ─> 13 Concepts
         │                               ^                     │
2 Lookup ┼───────────────────────────────┘                     │
         └─> 5 Templates core ─────────────────────────────────┘
                 ├─> 6 Language-support types
                 └─> 10 Grammar <── 11 Lexical
2 Lookup ─> 7 Use and one definition
4 + constructors + new-delete ─> 12 Constant evaluation of classes
```

Items 1 and 2 can be written in parallel. Item 5 needs only items 1 and
2, so it can run beside items 3 and 4.

## 3. In-scope text still too thin for an implementer

- CPY-19 (trivially copyable) and the standard-layout rule on
  members.md both end "and others that this page does not list yet
  *(referenced)*". "Trivial" in jumps.md has no definition.
- "Use" (odr-use), conversion rank, the associated set of NAM-5, and
  qualified lookup into a class or enumeration (NAM-2, NAM-4) are
  undefined. declarators.md has no decl-specifier grammar.
- The prohibition, constant-evaluation, and ordering gaps also found by
  R-foundations and W0 (T1, C6, T4, T5, F9, T2, E5, E4, E1, F2, O1, F1,
  F6).
- The tuple protocol in the SB rules (Q-0048) and `begin`/`end` lookup
  in RFOR are *(referenced)*.
- Stale markers that now have pages: *(classes referenced)* in
  unary.md, jumps.md, declarators.md, deleted.md, values.md, casts.md,
  list.md; abstract class in new-delete.md and lambdas.md; access in
  structured-bindings.md; operator overloading in members.md and
  default-arguments.md; bit-field in conditional.md; virtual base in
  constructors.md; standard-layout in unions.md; throw-expression in
  unary.md.
- Missed rejects on pages that look covered (attributes 27, virtual
  49, destructor declarations) are mostly front-end work. A Diagnose
  batch on attributes.md and virtual.md would separate the two.

## 4. Out of scope for now

The preprocessor (one test contains a directive); modules; coroutines
(no tests; they come after items 4 to 6 in any case); the memory model
and threads; the library beyond item 6; packs and fold expressions in
depth (one test); full concepts (three tests); deduction guides and
class template argument deduction (after item 8); named escapes that
need a Unicode name table, and extended floating-point types.

Items 2, 4, 5, 8, and 9 move topics the charter lists as referenced
into scope, and item 11 moves lexical structure, which the charter
excludes. Each needs a decision before writing starts.
