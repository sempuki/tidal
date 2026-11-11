# Q-0439: Can a static data member with no initializer have a class template name with no argument list as its type?

**Status:** provisional
**Affects:** CTAD-1, CTAD-2, `tests/templates/class-template-deduction/site-static-member-no-initializer.cpp`
**Source:** the I7 report (`reports/clean-room/I7/report.md`, I7-13), the round 12 writer probe `reports/clean-room/roadmap-12/writer-probes/i7-ctad.json`.
**Waits for:** the maintainer (pending.md row 4)

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

Kept open on 2026-11-11 under decision 0054 (decision 0063, item RV8). The review supports the answer for a member that is not inline and reads against it for an inline member. For the inline form, Clang 22 and EDG 7.0 keep the matrix's answer and GCC 16 follows the review, so the answer stays and the question joins `pending.md` row 4 with that form.

## Opinions

### Gap review, 2026-11-11 (phase 15 text, expressions, overloading, and templates)

The source-reader gap review of the phase 15 text on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`) reads the question: supports for a static member that is not inline, and reads against for an inline member (RV8-B-4). A deduced class type is allowed in the declaration that sets up a variable's initialization, and default initialization counts. The in-class declaration of a member that is not inline is no definition, so with no initializer it sets up no initialization, and CTAD-2 agrees for that form. An inline member is defined in its class, and that definition default-initializes it. So deduction runs with no arguments, as for a variable with no initializer, which is GCC's answer. The test declares both members in one class, so its rejection holds through the member that is not inline alone, and no test checks the inline form.
