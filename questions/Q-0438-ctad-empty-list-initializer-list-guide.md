# Q-0438: Does an empty braced list skip the initializer-list guides when a guide with no parameters exists?

**Status:** provisional
**Affects:** CTAD-18, CTAD-17, `tests/templates/class-template-deduction/initializer-list-guide-empty-list.cpp`
**Source:** the I7 report (`reports/clean-room/I7/report.md`, I7-6), the round 12 writer probe `reports/clean-room/roadmap-12/writer-probes/i7-ctad.json`.
**Waits for:** a first review; newest families that agree

## Context

A class template has a default constructor and a constructor that
takes `std::initializer_list<int>`. User guides give a different result
for each: `C(std::initializer_list<int>) -> C<long>` and
`C() -> C<char>`. The program writes `C c{};`.

ILC-3 runs two phases for a braced list: the initializer-list
constructors first, then all constructors. For constructors, an empty
list with a default constructor gives value initialization. CTAD-18
uses the two phases for the guides, and CTAD-17 sends the reader to
ILC-3.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| `C c{};` | `C<char>`, the guide with no parameters | `C<long>`, the initializer-list guide | `C<long>` |
| `C c{1, 2};` with a guide `(std::initializer_list<int>, int = 0)` and a guide `(int, int)` | the initializer-list guide | the initializer-list guide | the initializer-list guide |
| `C c(1, 2);`, same guides | `(int, int)` | `(int, int)` | `(int, int)` |

## Evidence

The test in **Affects**, `initializer-list-guide-with-default.cpp` for
the list of two, and the probe in **Source**.

## Options

1. An empty list skips the first phase when a guide with no parameters
   exists, as ILC-3 does for a default constructor (Clang).
2. An empty list tries the initializer-list guides first (GCC, EDG).

## Provisional answer

An empty list skips the initializer-list guides when a guide with no
parameters exists (option 1). The guides act as the constructors of the
class (CTAD-17), and ILC-3 gives this order for constructors. GCC and
EDG differ, so two newest families stand against the answer.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
