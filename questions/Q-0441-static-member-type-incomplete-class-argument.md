# Q-0441: Does the instantiation of a class need the type of a static data member complete?

**Status:** provisional
**Affects:** INST-5, `tests/templates/instantiation/static-member-incomplete-argument-declared.cpp`, `tests/templates/instantiation/static-inline-member-incomplete-argument.cpp`
**Source:** the P3 report (`reports/clean-room/P3/report.md`, P3-6), the round 12 writer probe `reports/clean-room/roadmap-12/writer-probes/p3-points.json`.
**Waits for:** a review (doubt); newest families that agree

## Context

A base class template `Base<D>` takes the derived class as its argument,
so `Base<Derived>` is made while `Derived` is incomplete. A member of
`Base<D>` has the type `Cursor<D>`, and `Cursor<Derived>` needs
`Derived` complete.

| Member of `Base<D>` | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| a member function with a body in the class, returning `Cursor<D>` | accepts | accepts | accepts |
| the same, declared in the class and defined outside | accepts | accepts | accepts |
| a member function that takes `Cursor<D>` by value | accepts | accepts | accepts |
| a member function with a deduced return type | accepts | accepts | accepts |
| a data member of type `Cursor<D>`, not `static` | rejects | rejects | rejects |
| `static Cursor<D> first;`, declared only | accepts | rejects: incomplete type | accepts |
| `static inline Cursor<D> first{};` | accepts | rejects | rejects |

Each result is the same with and without a later use of a member
function after `Derived` is complete.

## Evidence

The tests in **Affects**, `member-parameter-type-waits-for-definition.cpp`
and `member-deduced-return-waits-for-definition.cpp`, and the probe in
**Source**.

## Options

1. The instantiation of the class makes only the declaration of a
   `static` data member, also an `inline` one, and does not need its
   type complete (Clang).
2. The declaration of a `static` data member that is not `inline` does
   not need its type complete, and an `inline` one does (EDG).
3. Both need the type complete (GCC).

## Provisional answer

The instantiation of the class does not need the type of a `static`
data member complete, also when the member is `inline` (option 1).
INST-5 and INST-P2 make only the declaration of a `static` data member
with the class, and the definition with its initializer waits for a
use (Q-0231). Clang agrees. GCC differs on both forms, and EDG on the
`inline` one, so two newest families stand against the `inline` case.

Kept open on 2026-11-11 under decision 0054 (decision 0063, item RV8). The review supports the answer with a doubt, and GCC 16 and EDG 7.0 differ on the inline form.

## Opinions

### Gap review, 2026-11-11 (phase 15 text, expressions, overloading, and templates)

The source-reader gap review of the phase 15 text on expressions, overloading, and templates (`reports/review/2026-11-11-gap-review-phase-15-expressions-overloading-templates.md`) reads the question: supports with a doubt (RV8-B-9). Making the class makes the declaration of a static data member and not its definition, and a member that is not inline may be declared in its class with an incomplete type. For an inline member, the declaration in the class is a definition outside templates, and a definition needs a complete type. The class instantiation still makes only the declaration, and the reference treats such a declaration as a definition only for the checks of a redeclaration, so the review leans toward option 1 for the inline form too. The doubt is whether making the declaration of an inline member must check its type, as GCC and EDG do. The review also finds the word "only" in INST-5 too narrow, because a call that passes or returns the class by value also needs the types complete (INST-1).
