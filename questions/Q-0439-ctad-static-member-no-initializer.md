# Q-0439: Can a static data member with no initializer have a class template name with no argument list as its type?

**Status:** provisional
**Affects:** CTAD-1, CTAD-2, `tests/templates/class-template-deduction/site-static-member-no-initializer.cpp`
**Source:** the I7 report (`reports/clean-room/I7/report.md`, I7-13), the round 12 writer probe `reports/clean-room/roadmap-12/writer-probes/i7-ctad.json`.
**Waits for:** a first review

## Context

`template <class T = int> struct Box { T v; };` has the guide with no
parameters (CTAD-6), so `Box b;` deduces `Box<int>`. CTAD-1 lists a
static data member with an initializer. The text did not say whether a
static data member with no initializer can use the name.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| `struct S { static Box b; };` | rejects: needs an initializer | accepts | rejects: argument list missing |
| `struct S { static inline Box b; };` | rejects | accepts | rejects |
| `struct S { static inline Box b{}; };` | `Box<int>` | `Box<int>` | `Box<int>` |
| `Box b;` at namespace scope | `Box<int>` | `Box<int>` | `Box<int>` |
| `new Box` | `Box<int>*` | `Box<int>*` | `Box<int>*` |

## Evidence

The test in **Affects**, `site-new-no-initializer.cpp`, and the probe in
**Source**.

## Options

1. A static data member needs an initializer to deduce, also when
   `inline` (Clang, EDG).
2. A static data member with no initializer deduces with no arguments,
   as a variable does (GCC).

## Provisional answer

A static data member with no initializer cannot deduce (option 1).
CTAD-2 lists the form. Clang and EDG agree, and GCC alone differs.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
