# Q-0402: Must the ordering class be declared before a defaulted `<=>` is declared, or only before it is used?

**Status:** provisional
**Affects:** LSUP-2, `tests/foundations/language-support/ordering-declared-after-class.cpp`
**Source:** section 1.5 and the doubts of `reports/clean-room/roadmap-7/programmer.md`, from the reduction `ordering_declared_after_class.cpp`.
**Waits for:** a first review

## Context

LSUP-2 says that a program must first declare the ordering class that a
use of `<=>` yields. The rule did not say before what. In the test, a
class declares `auto operator<=>(const P&) const = default;`, the
program then declares `std::strong_ordering`, and `main` uses `<` on
two objects of the class.

| Implementation | Result |
|---|---|
| Clang 22 | rejects the defaulted declaration |
| GCC 16 | accepts and runs |
| EDG 7.0 | rejects the defaulted declaration |

## Evidence

The test in **Affects**, with an override for GCC.

## Options

1. The class is declared before the defaulted declaration, because the
   declaration deduces its return type there (Clang, EDG).
2. The class is declared before the first use of the defaulted `<=>`
   (GCC).

## Provisional answer

Before the defaulted declaration (option 1). Two of three families
agree, and the return type `auto` is deduced where the function is
declared as defaulted.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
