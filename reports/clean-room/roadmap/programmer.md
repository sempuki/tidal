# Roadmap: what to specify next, from a working programmer's view

Date: 2026-10-31. Decision 0029. A clean-room reader in the role of a
programmer who wants to write ordinary programs from Tidal alone read
AGENTS.md, docs/, spec/, reports/clean-room/, and questions/. Where it
relied on its general sense of what everyday programs use, it says so.
The front-end report was not yet written when it read.

## What blocks programs today

The pages cover nearly everything inside a single non-generic function
or class. Three gaps stop ordinary programs:

- **Overload resolution** is out of scope but marked *(referenced)* on
  about 15 pages: operators, copy-move, initializer-lists,
  conversion-functions, deleted, conditional, assignment, access, and
  the arithmetic operator pages.
- **Templates** are out of scope. Generic code and user-written
  containers cannot be written from the spec. No Write suite asks for a
  template, which shows the gap.
- **The library** is out of scope, and tasks may not use `#include`. A
  few facilities the core language relies on then have no source: the
  move cast, `std::initializer_list`, the `<=>` result types, size
  types, placement storage, and integer limits. The Java reader used
  outside knowledge for three of them (F4, F5, F6).

## 1. Ranked topics

**1. Overload resolution, everyday slice (one or two new pages).** Any
class with two constructors depends on it, as do a copy constructor
beside a move constructor and a `const` and non-`const` `operator[]`.
About 15 pages point at it. The common cases look easy and often go
wrong: `f(0)` against `f(int*)` and `f(long)`, a pointer that becomes
`bool`, `int` to `long` against `int` to `double`, and ambiguity. Scope:
candidate and viable functions, ranks of conversion sequences, reference
binding tie-breaks, the implicit object parameter, and ambiguity; leave
template partial ordering to item 7. Depends on conversions, reference
binding, conversion functions, list initialization of classes, and
operator candidates (OPR-6). About three times multiplicative.md.

**2. Templates I: function and class templates (new chapter; charter
change).** Containers of your own and generic code are impossible
without it, and the syntax is hard to guess: out-of-class member
definitions, the `typename` and `template` disambiguators, dependent
base members that need `this->`. Scope: declaring and defining
templates, explicit and default arguments, plain deduction, when a body
is checked and instantiated, dependent names, member templates. Depends
on item 1, names.md, members.md, and constant evaluation (R-foundations
E1). The largest item: about three pages the size of constructors.md.

**3. A language-support slice of the library (new page; small charter
change).** Core rules already rely on these names: the move cast,
`std::initializer_list`, the ordering types of `<=>`, the exception
`new` throws, and raw storage with construction in place for a growable
container (general knowledge; the spec does not discuss it). Scope:
`std::move` and `std::forward` or a plain statement that
`static_cast<T&&>` is the move, `std::initializer_list`, the size and
null pointer types, `std::byte`, the ordering types, placement `new`,
the exceptions the core language throws, `std::terminate`, the tuple
protocol of structured bindings, and which names a program may declare
without a header (R-foundations O4). Allow Write tasks to include only
those headers. About the size of if.md.

**4. Fundamental types and their ranges (new Foundations page).**
Cheap and needed constantly. Overflow checks need the range of `int`
(F4); the glossary marks conversion rank *(referenced)*; example
comments assume 32-bit `int` without a table; the signedness of `char`
and the exit-status truncation (F9) are the same kind of fact. Scope:
the integer, character, and floating types, ranks, widths and ranges
on each matrix target as `impl-defined` rules, two's complement (link
conversions.md), IEEE formats, sizes of pointers and `bool`. About the
size of shift.md.

**5. Name lookup in classes, and argument-dependent lookup (deepen
names.md, new page).** A derived `f(double)` hides the base's `f(int)`;
a free `operator<<` or `swap` beside a class is found only through
argument-dependent lookup, which range-for, access, and operators mark
*(referenced)*. Templates need it for dependent names. About twice
names.md.

**6. Programs in several files: a minimal `#include` slice and shared
definitions (new page).** A class used in two source files needs the
same definition in both, which everyday code gets from a header. STO-P4
states the same-definition rule for `inline` entities only. Scope:
textual `#include "file"`, include guards or `#pragma once`, and the
one-definition rule for classes, enumerations, inline functions, and
templates, with its no-diagnostic class. About the size of inline.md.

**7. Templates II: forwarding, packs, aliases, class argument
deduction.** Needed for `emplace`-style functions and `make`-style
helpers (general experience), alias templates, `Pair p{1, 2.0}`, and
partial ordering of function templates. About twice lambdas.md.

**8. Concepts and constraints, slice (new page).** auto.md already uses
a concept and marks the topic *(referenced)*. Scope: defining a concept,
`requires` clauses and expressions, satisfaction, constrained `auto`,
plain subsumption. About the size of constexpr.md.

**9. Template specialization and variable templates.** For simple type
traits a program writes itself. About one and a half times switch.md.

**10. Fixes to existing pages from the clean-room findings.** The W0
findings F1, F2, F3, F5, F9, F10, F12 and the R-foundations items T1,
O1, V1, E4. About one multiplicative.md in total.

**11. The complete standard-layout and trivially-copyable lists.**
CLS-19 and CPY-19 both mark the rest *(referenced)*. A few rules.

**12. `typeid` and run-time type names (addition to virtual.md).** A
few rules.

## 2. Task-level guides that help more than new rules

Each guide is informative, short, links rule IDs, and builds one
complete example. One guide per Write suite theme fits the scheme.

1. Writing a class that owns a resource (CPY-4, CPY-5, CPY-9, DTOR-5,
   NEW-4, NEW-5, and the double-free trap of F1).
2. Writing your own container (after items 1 to 3).
3. Error handling: what to throw and how (by value, never `new`, from
   F8), a cleanup guard in place of `defer`, `noexcept`, and THR-10.
4. Integer arithmetic without undefined behavior (MUL, ADD, SHF, CONV
   with item 4).
5. Value types: constructors, `explicit`, defaulted `==` and `<=>`,
   member and friend operators.
6. Enumerations in switches (F3).
7. Organizing a program across files (Q-0014).
8. Reading the spec: rule classes, ID prefixes, where topics live.

## 3. Out of scope for now

Coroutines (rare, and need a library protocol); threads, atomics, and
the memory model; modules (support differs across the matrix); the
preprocessor beyond `#include` and guards; the general library, which
the Write suites build; advanced template techniques; lexical structure
as a topic, since primary.md gives the literals programs use.

## Suggested order

10 (fixes, now), then 4, 1, 3, 5, 2, 6, 7, 8, 9, 11, 12. Guides 1, 3,
4, 6, and 8 can be written now; guides 2, 5, and 7 wait for items 1 to
3 and 6. Items 1 and 3 change the charter, item 2 moves templates from
referenced into scope, and item 6 adds a sliver of the preprocessor.
