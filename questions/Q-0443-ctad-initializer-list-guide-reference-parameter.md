# Q-0443: Is a guide whose first parameter is a reference to `std::initializer_list<E>` an initializer-list guide?

**Status:** provisional
**Affects:** CTAD-18, `tests/templates/class-template-deduction/initializer-list-guide-reference-parameter.cpp`
**Source:** point RV8-B-8 of the gap review of the phase 15 text on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`), with the writer probe `reports/clean-room/roadmap-12/writer-probes/rv8-points.json`.
**Waits for:** a first review

## Context

CTAD-18 calls a guide an initializer-list guide when its first
parameter is `std::initializer_list<E>`. A class template has a
constructor that takes `const std::initializer_list<int>&` and a
constructor that takes `(int, int)`. User guides give a different
result for each, and the program writes `Bag pair{1, 2};`.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| guide `Bag(const std::initializer_list<int>&) -> Bag<long>` and guide `Bag(int, int) -> Bag<char>`, with `Bag pair{1, 2};` | `Bag<long>` | `Bag<long>` | `Bag<char>` |

## Evidence

The test in **Affects**, and the probe in **Source**.

## Options

1. A first parameter that is a reference to `std::initializer_list<E>`,
   with or without `const`, also makes an initializer-list guide, so the
   list chooses it in the first phase (Clang, GCC).
2. Only a first parameter of type `std::initializer_list<E>` makes an
   initializer-list guide, and the list chooses among all guides (EDG).

## Provisional answer

A reference to `std::initializer_list<E>` also makes an initializer-list
guide (option 1). Clang and GCC agree, and an initializer-list
constructor often takes its list by reference to `const`. EDG alone
differs.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
